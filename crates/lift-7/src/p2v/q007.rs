//! Mechanically lifted functions (scripts/lift_run.py), module q007. Each one is checked against the original
//! in the world harness (tests/world_q007.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xe44a8 size=294
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e44a8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe44a8;
    loop {
        match pc {
        0xe44a8 => {
            // block e44a8
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e44a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e44a8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e44a9 mov rbp,rsp
            rbp = rsp;
            // e44ac push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e44ae push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e44b0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e44b2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e44b4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e44b5 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e44b9 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e44bb mov r14,rdi
            r14 = rdi;
            // e44be lea rdi,[r14+0x1038]
            rdi = r14.wrapping_add(0x1038);
            // e44c5 mov esi,0x1004
            rsi = (0x1004u64) as u32 as u64;
            // e44ca call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e44cf xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e44d2 lea r15,[rbp-0x2c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // e44d6 lea r12,[rbp-0x30]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0xe44da;
        }
        0xe44da => {
            // block e44da
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e44da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e44da mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            pc = 0xe44e1;
        }
        0xe44e1 => {
            // block e44e1
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e44e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e44e1 mov rdi,r14
            rdi = r14;
            // e44e4 mov rsi,r15
            rsi = r15;
            // e44e7 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // e44e9 call 0xe463c
            { let r = { let t = m.env.code_va(0xe463c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e44ee test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e44f0 je 0xe4550
            if fl.e() { pc = 0xe4550; continue; }
            pc = 0xe44f2;
        }
        0xe44f2 => {
            // block e44f2
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e44f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e44f2 cmp DWORD PTR [rbp-0x2c],0xffffffff
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64, 0xffffffffu64).1;
            // e44f6 jne 0xe44e1
            if fl.ne() { pc = 0xe44e1; continue; }
            pc = 0xe44f8;
        }
        0xe44f8 => {
            // block e44f8
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e44f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e44f8 mov edx,0x32
            rdx = (0x32u64) as u32 as u64;
            // e44fd mov rdi,r14
            rdi = r14;
            // e4500 mov rsi,r12
            rsi = r12;
            // e4503 call 0xe463c
            { let r = { let t = m.env.code_va(0xe463c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4508 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e450a je 0xe4569
            if fl.e() { pc = 0xe4569; continue; }
            pc = 0xe450c;
        }
        0xe450c => {
            // block e450c
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e450c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e450c mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // e450f cmp eax,DWORD PTR [r14+0x2048]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x2048)) as u64).1;
            // e4516 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e4518 je 0xe451d
            if fl.e() { pc = 0xe451d; continue; }
            pc = 0xe451a;
        }
        0xe451a => {
            // block e451a
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e451a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e451a mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            pc = 0xe451d;
        }
        0xe451d => {
            // block e451d
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e451d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e451d test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // e451f mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e4522 je 0xe44da
            if fl.e() { pc = 0xe44da; continue; }
            pc = 0xe4524;
        }
        0xe4524 => {
            // block e4524
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e4524 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4524 lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // e4528 mov edx,0x32
            rdx = (0x32u64) as u32 as u64;
            // e452d mov rdi,r14
            rdi = r14;
            // e4530 call 0xe463c
            { let r = { let t = m.env.code_va(0xe463c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4535 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e4537 je 0xe4575
            if fl.e() { pc = 0xe4575; continue; }
            pc = 0xe4539;
        }
        0xe4539 => {
            // block e4539
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e4539 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4539 mov ecx,DWORD PTR [rbp-0x34]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // e453c cmp ecx,0x401
            fl = sub::<32>((rcx as u32 as u64), 0x401u64).1;
            // e4542 jb 0xe4581
            if fl.b() { pc = 0xe4581; continue; }
            pc = 0xe4544;
        }
        0xe4544 => {
            // block e4544
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e4544 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4544 mov DWORD PTR [r14+0x8],0x14
            m.w32(r14.wrapping_add(0x8), (0x14u64) as u32);
            // e454c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e454e jmp 0xe455a
            pc = 0xe455a; continue;
        }
        0xe4550 => {
            // block e4550
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e4550 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4550 mov DWORD PTR [r14+0x8],0x11
            m.w32(r14.wrapping_add(0x8), (0x11u64) as u32);
            // e4558 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xe455a;
        }
        0xe455a => {
            // block e455a
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e455a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e455a add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e455e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e455f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e4561 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e4563 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e4565 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e4567 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4568 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe4569 => {
            // block e4569
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e4569 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4569 mov DWORD PTR [r14+0x8],0x12
            m.w32(r14.wrapping_add(0x8), (0x12u64) as u32);
            // e4571 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e4573 jmp 0xe455a
            pc = 0xe455a; continue;
        }
        0xe4575 => {
            // block e4575
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e4575 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4575 mov DWORD PTR [r14+0x8],0x13
            m.w32(r14.wrapping_add(0x8), (0x13u64) as u32);
            // e457d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e457f jmp 0xe455a
            pc = 0xe455a; continue;
        }
        0xe4581 => {
            // block e4581
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e4581 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4581 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e4583 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // e4585 je 0xe45c2
            if fl.e() { pc = 0xe45c2; continue; }
            pc = 0xe4587;
        }
        0xe4587 => {
            // block e4587
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e4587 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4587 lea r15,[rbp-0x38]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            pc = 0xe458b;
        }
        0xe458b => {
            // block e458b
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e458b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e458b mov rbx,rax
            rbx = rax;
            // e458e mov edx,0x32
            rdx = (0x32u64) as u32 as u64;
            // e4593 mov rdi,r14
            rdi = r14;
            // e4596 mov rsi,r15
            rsi = r15;
            // e4599 call 0xe463c
            { let r = { let t = m.env.code_va(0xe463c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e459e mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // e45a1 mov DWORD PTR [r14+rbx*4+0x1038],eax
            m.w32(r14.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0x1038), ((rax as u32 as u64)) as u32);
            // e45a9 lea rax,[rbx+0x1]
            rax = rbx.wrapping_add(0x1);
            // e45ad cmp eax,DWORD PTR [rbp-0x34]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64).1;
            // e45b0 jb 0xe458b
            if fl.b() { pc = 0xe458b; continue; }
            pc = 0xe45b2;
        }
        0xe45b2 => {
            // block e45b2
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e45b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e45b2 mov DWORD PTR [r14+rbx*4+0x103c],0x0
            m.w32(r14.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0x103c), (0x0u64) as u32);
            // e45be mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e45c0 jmp 0xe455a
            pc = 0xe455a; continue;
        }
        0xe45c2 => {
            // block e45c2
            #[cfg(feature = "trace")] eprintln!("f_000e44a8 pc e45c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e45c2 mov DWORD PTR [r14+0x8],0x15
            m.w32(r14.wrapping_add(0x8), (0x15u64) as u32);
            // e45ca xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e45cc jmp 0xe455a
            pc = 0xe455a; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x632dc size=291
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000632dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x632dc;
    loop {
        match pc {
        0x632dc => {
            // block 632dc
            #[cfg(feature = "trace")] eprintln!("f_000632dc pc 632dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 632dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 632dd mov rbp,rsp
            rbp = rsp;
            // 632e0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 632e1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 632e2 mov rbx,rsi
            rbx = rsi;
            // 632e5 lea rsi,[rip+0xce4d3] # 0x1317bf
            rsi = m.env.data_va(286604);
            // 632ec mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 632f1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 632f6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 632f9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 632fd mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 63301 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 63304 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 63307 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 6330b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 6330e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 63312 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 6331b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 6331e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 63322 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 6332d mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 6332f mov rdi,rax
            rdi = rax;
            // 63332 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63337 lea rsi,[rip+0xce488] # 0x1317c6
            rsi = m.env.data_va(286611);
            // 6333e mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 63343 mov rdi,rax
            rdi = rax;
            // 63346 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6334b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 6334e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 63352 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 63356 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 63359 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6335c mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 63360 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 63363 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 63367 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 63370 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 63373 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 63377 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 63382 mov esi,DWORD PTR [rbx+0x8]
            rsi = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 63385 mov rdi,rax
            rdi = rax;
            // 63388 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6338d lea rsi,[rip+0xce439] # 0x1317cd
            rsi = m.env.data_va(286618);
            // 63394 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 63399 mov rdi,rax
            rdi = rax;
            // 6339c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 633a1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 633a4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 633a8 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 633ac and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 633af or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 633b2 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 633b6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 633b9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 633bd mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 633c6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 633c9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 633cd mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 633d8 mov esi,DWORD PTR [rbx+0x4]
            rsi = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 633db mov rdi,rax
            rdi = rax;
            // 633de call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 633e3 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 633e6 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 633ea mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 633ee and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 633f1 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 633f4 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 633f8 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 633fc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 633fd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 633fe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x27191 size=285
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00027191<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x27191;
    loop {
        match pc {
        0x27191 => {
            // block 27191
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 27191 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27191 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 27192 mov rbp,rsp
            rbp = rsp;
            // 27195 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 27197 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 27199 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2719b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2719d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2719e sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 271a2 mov r14,rsi
            r14 = rsi;
            // 271a5 mov rbx,rdi
            rbx = rdi;
            // 271a8 mov rsi,QWORD PTR [r14+0x10]
            rsi = m.r64(r14.wrapping_add(0x10)) as u64;
            // 271ac call 0xec0a4
            { let r = { let t = m.env.stub_va(0x871); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 271b1 lea rsi,[rip+0x1032e2] # 0x12a49a
            rsi = m.env.data_va(257127);
            // 271b8 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 271bd mov rdi,rax
            rdi = rax;
            // 271c0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 271c5 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // 271c8 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 271cc cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 271cf je 0x27277
            if fl.e() { pc = 0x27277; continue; }
            pc = 0x271d5;
        }
        0x271d5 => {
            // block 271d5
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 271d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 271d5 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 271d9 jmp 0x271f2
            pc = 0x271f2; continue;
        }
        0x271db => {
            // block 271db
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 271db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 271db mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 271e0 mov rdi,rbx
            rdi = rbx;
            // 271e3 lea rsi,[rip+0x1032bb] # 0x12a4a5
            rsi = m.env.data_va(257138);
            // 271ea call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 271ef mov r13,r12
            r13 = r12;
            pc = 0x271f2;
        }
        0x271f2 => {
            // block 271f2
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 271f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 271f2 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 271f6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 271f8 mov rdi,r15
            rdi = r15;
            // 271fb call 0x60974
            { let r = { let t = m.env.code_va(0x60974); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27200 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 27204 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 27207 jne 0x27212
            if fl.ne() { pc = 0x27212; continue; }
            pc = 0x27209;
        }
        0x27209 => {
            // block 27209
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 27209 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27209 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 2720c lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 27210 jmp 0x2721a
            pc = 0x2721a; continue;
        }
        0x27212 => {
            // block 27212
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 27212 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27212 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 27216 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x2721a;
        }
        0x2721a => {
            // block 2721a
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 2721a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2721a mov rdi,rbx
            rdi = rbx;
            // 2721d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27222 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 27227 mov rdi,rax
            rdi = rax;
            // 2722a lea rsi,[rip+0x10326f] # 0x12a4a0
            rsi = m.env.data_va(257133);
            // 27231 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27236 lea rsi,[r13+0x20]
            rsi = r13.wrapping_add(0x20);
            // 2723a mov rdi,rax
            rdi = rax;
            // 2723d call 0x26f9d
            { let r = { let t = m.env.code_va(0x26f9d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27242 mov rdi,r15
            rdi = r15;
            // 27245 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2724a mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // 2724e test rax,rax
            fl = logic::<64>(rax & rax);
            // 27251 je 0x27261
            if fl.e() { pc = 0x27261; continue; }
            pc = 0x27253;
        }
        0x27253 => {
            // block 27253
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 27253 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27253 mov r12,rax
            r12 = rax;
            // 27256 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2725a test rax,rax
            fl = logic::<64>(rax & rax);
            // 2725d jne 0x27253
            if fl.ne() { pc = 0x27253; continue; }
            pc = 0x2725f;
        }
        0x2725f => {
            // block 2725f
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 2725f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2725f jmp 0x2726e
            pc = 0x2726e; continue;
        }
        0x27261 => {
            // block 27261
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 27261 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27261 mov r12,QWORD PTR [r13+0x10]
            r12 = m.r64(r13.wrapping_add(0x10)) as u64;
            // 27265 cmp QWORD PTR [r12],r13
            fl = sub::<64>(m.r64(r12) as u64, r13).1;
            // 27269 mov r13,r12
            r13 = r12;
            // 2726c jne 0x27261
            if fl.ne() { pc = 0x27261; continue; }
            pc = 0x2726e;
        }
        0x2726e => {
            // block 2726e
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 2726e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2726e cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 27271 jne 0x271db
            if fl.ne() { pc = 0x271db; continue; }
            pc = 0x27277;
        }
        0x27277 => {
            // block 27277
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 27277 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27277 lea rsi,[rip+0x1031d6] # 0x12a454
            rsi = m.env.data_va(257057);
            // 2727e mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 27283 mov rdi,rbx
            rdi = rbx;
            // 27286 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2728b add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2728f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 27290 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 27292 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 27294 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 27296 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 27298 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 27299 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2729a => {
            // block 2729a
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 2729a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2729a mov rbx,rax
            rbx = rax;
            // 2729d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 272a1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 272a6 mov rdi,rbx
            rdi = rbx;
            // 272a9 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x117fa size=280
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000117fa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x117fa;
    loop {
        match pc {
        0x117fa => {
            // block 117fa
            #[cfg(feature = "trace")] eprintln!("f_000117fa pc 117fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 117fa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 117fb mov rbp,rsp
            rbp = rsp;
            // 117fe push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 11800 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11802 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 11804 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 11806 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11807 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 11808 mov r13,rdi
            r13 = rdi;
            // 1180b mov rax,QWORD PTR [rip+0x12ea06] # 0x140218
            rax = m.r64(m.env.data_va(346597)) as u64;
            // 11812 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 11816 mov QWORD PTR [r13+0x0],rax
            m.w64(r13, (rax) as u64);
            // 1181a mov QWORD PTR [r13+0x8],rsi
            m.w64(r13.wrapping_add(0x8), (rsi) as u64);
            // 1181e mov DWORD PTR [r13+0x10],0x1
            m.w32(r13.wrapping_add(0x10), (0x1u64) as u32);
            // 11826 mov QWORD PTR [r13+0x20],0x0
            m.w64(r13.wrapping_add(0x20), (0x0u64) as u64);
            // 1182e mov QWORD PTR [r13+0x18],0x0
            m.w64(r13.wrapping_add(0x18), (0x0u64) as u64);
            // 11836 mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // 11839 mov rdi,rsi
            rdi = rsi;
            // 1183c call QWORD PTR [rax+0x50]
            { let r = { let t = m.r64(rax.wrapping_add(0x50)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1183f cmp eax,0xa
            fl = sub::<32>((rax as u32 as u64), 0xau64).1;
            // 11842 jne 0x11902
            if fl.ne() { pc = 0x11902; continue; }
            pc = 0x11848;
        }
        0x11848 => {
            // block 11848
            #[cfg(feature = "trace")] eprintln!("f_000117fa pc 11848 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11848 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 1184c mov rdi,r13
            rdi = r13;
            // 1184f call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11852 mov rcx,rax
            rcx = rax;
            // 11855 neg rcx
            { let (r, f) = neg::<64>(rcx); rcx = r; fl = f; }
            // 11858 cmovl rcx,rax
            if fl.l() { rcx = rax; }
            // 1185c mov QWORD PTR [r13+0x20],rcx
            m.w64(r13.wrapping_add(0x20), (rcx) as u64);
            // 11860 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 11864 mov rdi,r13
            rdi = r13;
            // 11867 call QWORD PTR [rax+0x20]
            { let r = { let t = m.r64(rax.wrapping_add(0x20)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1186a imul rax,QWORD PTR [r13+0x20]
            { let (r, f) = imul::<64>(rax, m.r64(r13.wrapping_add(0x20)) as u64); rax = r; fl = f; }
            // 1186f mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // 11874 mul rcx
            { let (lo, hi, f) = mul1::<64>(rax, rcx); rax = lo; rdx = hi; fl = f; }
            // 11877 mov rdi,0xffffffffffffffff
            rdi = 0xffff_ffff_ffff_ffffu64;
            // 1187e cmovno rdi,rax
            if fl.no() { rdi = rax; }
            // 11882 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11887 mov QWORD PTR [r13+0x18],rax
            m.w64(r13.wrapping_add(0x18), (rax) as u64);
            // 1188b mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 1188f mov rdi,r13
            rdi = r13;
            // 11892 call QWORD PTR [rax+0x68]
            { let r = { let t = m.r64(rax.wrapping_add(0x68)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11895 mov r14,rax
            r14 = rax;
            // 11898 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 1189c mov r15,QWORD PTR [r13+0x18]
            r15 = m.r64(r13.wrapping_add(0x18)) as u64;
            // 118a0 mov rdi,r13
            rdi = r13;
            // 118a3 call QWORD PTR [rax+0x20]
            { let r = { let t = m.r64(rax.wrapping_add(0x20)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 118a6 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 118a9 test rax,rax
            fl = logic::<64>(rax & rax);
            // 118ac jle 0x11902
            if fl.le() { pc = 0x11902; continue; }
            pc = 0x118ae;
        }
        0x118ae => {
            // block 118ae
            #[cfg(feature = "trace")] eprintln!("f_000117fa pc 118ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 118ae mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 118b2 mov rdi,r13
            rdi = r13;
            // 118b5 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 118b8 test rax,rax
            fl = logic::<64>(rax & rax);
            // 118bb mov ebx,0x0
            rbx = (0x0u64) as u32 as u64;
            // 118c0 jle 0x118df
            if fl.le() { pc = 0x118df; continue; }
            pc = 0x118c2;
        }
        0x118c2 => {
            // block 118c2
            #[cfg(feature = "trace")] eprintln!("f_000117fa pc 118c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 118c2 mov eax,DWORD PTR [r14+rbx*4]
            rax = (m.r32(r14.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 118c6 rol eax,0x8
            { let (r, f) = rol::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 118c9 mov DWORD PTR [r15+rbx*4],eax
            m.w32(r15.wrapping_add(rbx.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 118cd inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 118d0 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 118d4 mov rdi,r13
            rdi = r13;
            // 118d7 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 118da cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 118dd jl 0x118c2
            if fl.l() { pc = 0x118c2; continue; }
            pc = 0x118df;
        }
        0x118df => {
            // block 118df
            #[cfg(feature = "trace")] eprintln!("f_000117fa pc 118df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 118df mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 118e3 mov rdi,r13
            rdi = r13;
            // 118e6 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 118e9 add r14,rax
            { let (r, f) = add::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // 118ec add r15,QWORD PTR [r13+0x20]
            { let (r, f) = add::<64>(r15, m.r64(r13.wrapping_add(0x20)) as u64);
              r15 = r;
              fl = f; }
            // 118f0 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // 118f3 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 118f7 mov rdi,r13
            rdi = r13;
            // 118fa call QWORD PTR [rax+0x20]
            { let r = { let t = m.r64(rax.wrapping_add(0x20)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 118fd cmp r12,rax
            fl = sub::<64>(r12, rax).1;
            // 11900 jl 0x118ae
            if fl.l() { pc = 0x118ae; continue; }
            pc = 0x11902;
        }
        0x11902 => {
            // block 11902
            #[cfg(feature = "trace")] eprintln!("f_000117fa pc 11902 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11902 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 11906 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11907 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 11909 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1190b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1190d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1190f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11910 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1973b size=277
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001973b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1973b;
    loop {
        match pc {
        0x1973b => {
            // block 1973b
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 1973b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1973b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1973c mov rbp,rsp
            rbp = rsp;
            // 1973f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 19741 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19743 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 19745 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 19747 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19748 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 1974c mov rbx,rsi
            rbx = rsi;
            // 1974f mov r14d,edi
            r14 = ((rdi as u32 as u64)) as u32 as u64;
            // 19752 lea r12,[rbp-0x38]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 19756 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 1975e mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 19766 mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 1976a lea r13,[rbx+0x8]
            r13 = rbx.wrapping_add(0x8);
            // 1976e mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 19772 mov rdi,rbx
            rdi = rbx;
            // 19775 call 0xeb9a8
            { let r = { let t = m.env.stub_va(0x175); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1977a mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 19782 mov QWORD PTR [rbx],r13
            m.w64(rbx, (r13) as u64);
            // 19785 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 1978d lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 19791 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // 19794 call 0x19556
            { let r = { let t = m.env.code_va(0x19556); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19799 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1979b je 0x19808
            if fl.e() { pc = 0x19808; continue; }
            pc = 0x1979d;
        }
        0x1979d => {
            // block 1979d
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 1979d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1979d mov r14,QWORD PTR [rbp-0x40]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 197a1 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 197a4 cmp r14,r12
            fl = sub::<64>(r14, r12).1;
            // 197a7 je 0x1980b
            if fl.e() { pc = 0x1980b; continue; }
            pc = 0x197a9;
        }
        0x197a9 => {
            // block 197a9
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 197a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 197a9 lea r15,[rbp-0x44]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            pc = 0x197ad;
        }
        0x197ad => {
            // block 197ad
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 197ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 197ad mov edi,DWORD PTR [r14+0x1c]
            rdi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 197b1 call 0x5a914
            { let r = { let t = m.env.code_va(0x5a914); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 197b6 mov DWORD PTR [rbp-0x44],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), ((rax as u32 as u64)) as u32);
            // 197b9 cmp eax,0xb
            fl = sub::<32>((rax as u32 as u64), 0xbu64).1;
            // 197bc ja 0x197d9
            if fl.a() { pc = 0x197d9; continue; }
            pc = 0x197be;
        }
        0x197be => {
            // block 197be
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 197be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 197be mov rdi,rbx
            rdi = rbx;
            // 197c1 mov rsi,r15
            rsi = r15;
            // 197c4 call 0xeb9a2
            { let r = { let t = m.env.stub_va(0x16f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 197c9 cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // 197cc jne 0x197d9
            if fl.ne() { pc = 0x197d9; continue; }
            pc = 0x197ce;
        }
        0x197ce => {
            // block 197ce
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 197ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 197ce mov rdi,rbx
            rdi = rbx;
            // 197d1 mov rsi,r15
            rsi = r15;
            // 197d4 call 0xebace
            { let r = { let t = m.env.stub_va(0x29b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x197d9;
        }
        0x197d9 => {
            // block 197d9
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 197d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 197d9 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 197dd test rax,rax
            fl = logic::<64>(rax & rax);
            // 197e0 je 0x197ef
            if fl.e() { pc = 0x197ef; continue; }
            pc = 0x197e2;
        }
        0x197e2 => {
            // block 197e2
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 197e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 197e2 mov r14,rax
            r14 = rax;
            // 197e5 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 197e8 test rax,rax
            fl = logic::<64>(rax & rax);
            // 197eb jne 0x197e2
            if fl.ne() { pc = 0x197e2; continue; }
            pc = 0x197ed;
        }
        0x197ed => {
            // block 197ed
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 197ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 197ed jmp 0x197fe
            pc = 0x197fe; continue;
        }
        0x197ef => {
            // block 197ef
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 197ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 197ef mov rax,r14
            rax = r14;
            pc = 0x197f2;
        }
        0x197f2 => {
            // block 197f2
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 197f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 197f2 mov r14,QWORD PTR [rax+0x10]
            r14 = m.r64(rax.wrapping_add(0x10)) as u64;
            // 197f6 cmp QWORD PTR [r14],rax
            fl = sub::<64>(m.r64(r14) as u64, rax).1;
            // 197f9 mov rax,r14
            rax = r14;
            // 197fc jne 0x197f2
            if fl.ne() { pc = 0x197f2; continue; }
            pc = 0x197fe;
        }
        0x197fe => {
            // block 197fe
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 197fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 197fe cmp r14,r12
            fl = sub::<64>(r14, r12).1;
            // 19801 jne 0x197ad
            if fl.ne() { pc = 0x197ad; continue; }
            pc = 0x19803;
        }
        0x19803 => {
            // block 19803
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 19803 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19803 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 19806 jmp 0x1980b
            pc = 0x1980b; continue;
        }
        0x19808 => {
            // block 19808
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 19808 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19808 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x1980b;
        }
        0x1980b => {
            // block 1980b
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 1980b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1980b mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 1980f lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 19813 call 0xeb9c0
            { let r = { let t = m.env.stub_va(0x18d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19818 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 1981b add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 1981f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19820 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 19822 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 19824 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 19826 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 19828 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19829 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1982a => {
            // block 1982a
            #[cfg(feature = "trace")] eprintln!("f_0001973b pc 1982a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1982a mov rbx,rax
            rbx = rax;
            // 1982d mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 19831 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 19835 call 0xeb9c0
            { let r = { let t = m.env.stub_va(0x18d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1983a mov rdi,rbx
            rdi = rbx;
            // 1983d call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19842 jmp 0x1982a
            pc = 0x1982a; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8598 size=273
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8598<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc8598;
    loop {
        match pc {
        0xc8598 => {
            // block c8598
            #[cfg(feature = "trace")] eprintln!("f_000c8598 pc c8598 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8598 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8599 mov rbp,rsp
            rbp = rsp;
            // c859c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c859e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c85a0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c85a1 mov ax,WORD PTR [rdi]
            rax = (rax & !0xffff) | ((m.r16(rdi) as u64) as u16 as u64);
            // c85a4 mov WORD PTR [rdi-0x2],ax
            m.w16(rdi.wrapping_add(0xffff_ffff_ffff_fffe), ((rax as u16 as u64)) as u16);
            // c85a8 lea eax,[r8-0x1]
            rax = (r8.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // c85ac cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // c85ae mov ax,WORD PTR [rdi+rax*2]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // c85b2 lea ebx,[r8+0x1]
            rbx = (r8.wrapping_add(0x1)) as u32 as u64;
            // c85b6 movsxd rbx,ebx
            rbx = sx::<32>((rbx as u32 as u64)) as u64;
            // c85b9 mov WORD PTR [rdi+rbx*2],ax
            m.w16(rdi.wrapping_add(rbx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // c85bd movsxd rbx,r8d
            rbx = sx::<32>((r8 as u32 as u64)) as u64;
            // c85c0 mov WORD PTR [rdi+rbx*2],ax
            m.w16(rdi.wrapping_add(rbx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // c85c4 imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // c85c8 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // c85ca cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c85cb idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // c85cd mov r11d,eax
            r11 = ((rax as u32 as u64)) as u32 as u64;
            // c85d0 imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c85d4 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c85d6 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c85d7 idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // c85d9 mov r10d,eax
            r10 = ((rax as u32 as u64)) as u32 as u64;
            // c85dc cmp r11d,r10d
            fl = sub::<32>((r11 as u32 as u64), (r10 as u32 as u64)).1;
            // c85df jge 0xc86a2
            if fl.ge() { pc = 0xc86a2; continue; }
            pc = 0xc85e5;
        }
        0xc85e5 => {
            // block c85e5
            #[cfg(feature = "trace")] eprintln!("f_000c8598 pc c85e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c85e5 shl r8d,0x10
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x10u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // c85e9 mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // c85ec cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c85ed idiv r9d
            { let (q, r) = idiv::<32>(rdx, rax, (r9 as u32 as u64)); rax = q; rdx = r; }
            // c85f0 sub r10d,r11d
            { let (r, f) = sub::<32>((r10 as u32 as u64), (r11 as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // c85f3 movsxd rcx,r11d
            rcx = sx::<32>((r11 as u32 as u64)) as u64;
            // c85f6 lea r14,[rsi+rcx*2]
            r14 = rsi.wrapping_add(rcx.wrapping_mul(2));
            // c85fa imul r11d,eax
            { let (r, f) = imul::<32>((r11 as u32 as u64), (rax as u32 as u64)); r11 = (r) as u32 as u64; fl = f; }
            // c85fe lea r9,[rip+0x80dbb] # 0x1493c0
            r9 = m.env.data_va(383885);
            // c8605 mov r8w,0x4
            r8 = (r8 & !0xffff) | ((0x4u64) as u16 as u64);
            pc = 0xc860a;
        }
        0xc860a => {
            // block c860a
            #[cfg(feature = "trace")] eprintln!("f_000c8598 pc c860a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c860a mov edx,r11d
            rdx = ((r11 as u32 as u64)) as u32 as u64;
            // c860d sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // c8610 mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // c8613 shr ebx,0xb
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xbu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c8616 and ebx,0x1f
            { let r = (rbx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c8619 movsxd r15,edx
            r15 = sx::<32>((rdx as u32 as u64)) as u64;
            // c861c movsx esi,WORD PTR [rdi+r15*2-0x2]
            rsi = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // c8622 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // c8627 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c8629 imul esi,DWORD PTR [r9+rcx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r9.wrapping_add(rcx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // c862e movsx ecx,WORD PTR [rdi+r15*2]
            rcx = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // c8633 mov edx,0x40
            rdx = (0x40u64) as u32 as u64;
            // c8638 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c863a imul ecx,DWORD PTR [r9+rdx*4]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(r9.wrapping_add(rdx.wrapping_mul(4))) as u64); rcx = (r) as u32 as u64; fl = f; }
            // c863f add ecx,esi
            { let (r, f) = add::<32>((rcx as u32 as u64), (rsi as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c8641 movsx edx,WORD PTR [rdi+r15*2+0x2]
            rdx = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // c8647 mov esi,0x60
            rsi = (0x60u64) as u32 as u64;
            // c864c sub esi,ebx
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rbx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // c864e imul edx,DWORD PTR [r9+rsi*4]
            { let (r, f) = imul::<32>((rdx as u32 as u64), m.r32(r9.wrapping_add(rsi.wrapping_mul(4))) as u64); rdx = (r) as u32 as u64; fl = f; }
            // c8653 add edx,ecx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8655 movsx esi,WORD PTR [rdi+r15*2+0x4]
            rsi = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // c865b mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // c8660 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c8662 imul esi,DWORD PTR [r9+rcx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r9.wrapping_add(rcx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // c8667 add esi,edx
            { let (r, f) = add::<32>((rsi as u32 as u64), (rdx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // c8669 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // c866b and edx,0xffff0000
            { let r = (rdx as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8671 mov bx,0x3fb
            rbx = (rbx & !0xffff) | ((0x3fbu64) as u16 as u64);
            // c8675 cmp edx,0x3fb0000
            fl = sub::<32>((rdx as u32 as u64), 0x3fb0000u64).1;
            // c867b jg 0xc868e
            if fl.g() { pc = 0xc868e; continue; }
            pc = 0xc867d;
        }
        0xc867d => {
            // block c867d
            #[cfg(feature = "trace")] eprintln!("f_000c8598 pc c867d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c867d sar esi,0x10
            { let (r, f) = sar::<32>((rsi as u32 as u64), 0x10u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // c8680 cmp edx,0x40000
            fl = sub::<32>((rdx as u32 as u64), 0x40000u64).1;
            // c8686 cmovl si,r8w
            if fl.l() { rsi = (rsi & !0xffff) | (((r8 as u16 as u64)) as u16 as u64); }
            // c868b mov bx,si
            rbx = (rbx & !0xffff) | (((rsi as u16 as u64)) as u16 as u64);
            pc = 0xc868e;
        }
        0xc868e => {
            // block c868e
            #[cfg(feature = "trace")] eprintln!("f_000c8598 pc c868e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c868e mov WORD PTR [r14],bx
            m.w16(r14, ((rbx as u16 as u64)) as u16);
            // c8692 add r14,0x2
            { let (r, f) = add::<64>(r14, 0x2u64);
              r14 = r;
              fl = f; }
            // c8696 add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c8699 dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // c869c jne 0xc860a
            if fl.ne() { pc = 0xc860a; continue; }
            pc = 0xc86a2;
        }
        0xc86a2 => {
            // block c86a2
            #[cfg(feature = "trace")] eprintln!("f_000c8598 pc c86a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c86a2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c86a3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c86a5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c86a7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c86a8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1250d size=271
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001250d<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1250d;
    loop {
        match pc {
        0x1250d => {
            // block 1250d
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1250d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1250d push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1250e mov rbp,rsp
            rbp = rsp;
            // 12511 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12513 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12515 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 12517 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12519 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1251a sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 1251e mov rbx,r8
            rbx = r8;
            // 12521 mov r13,rcx
            r13 = rcx;
            // 12524 mov QWORD PTR [rbp-0x38],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r13) as u64);
            // 12528 mov r12,rdx
            r12 = rdx;
            // 1252b mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // 1252f mov rcx,rdi
            rcx = rdi;
            // 12532 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 12535 mov r14,rbx
            r14 = rbx;
            // 12538 cmp rcx,rsi
            fl = sub::<64>(rcx, rsi).1;
            // 1253b je 0x12589
            if fl.e() { pc = 0x12589; continue; }
            pc = 0x1253d;
        }
        0x1253d => {
            // block 1253d
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1253d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1253d cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 12540 je 0x125ab
            if fl.e() { pc = 0x125ab; continue; }
            pc = 0x12542;
        }
        0x12542 => {
            // block 12542
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 12542 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12542 mov eax,DWORD PTR [r12+0x18]
            rax = (m.r32(r12.wrapping_add(0x18)) as u64) as u32 as u64;
            // 12547 cmp eax,DWORD PTR [rcx+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(rcx.wrapping_add(0x18)) as u64).1;
            // 1254a jge 0x12563
            if fl.ge() { pc = 0x12563; continue; }
            pc = 0x1254c;
        }
        0x1254c => {
            // block 1254c
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1254c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1254c mov r13,rcx
            r13 = rcx;
            // 1254f mov rdi,r14
            rdi = r14;
            // 12552 mov rsi,r12
            rsi = r12;
            // 12555 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1255a add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 1255e mov rcx,r13
            rcx = r13;
            // 12561 jmp 0x12578
            pc = 0x12578; continue;
        }
        0x12563 => {
            // block 12563
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 12563 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12563 mov rdi,r14
            rdi = r14;
            // 12566 mov rsi,rcx
            rsi = rcx;
            // 12569 mov r13,rcx
            r13 = rcx;
            // 1256c call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12571 mov rcx,r13
            rcx = r13;
            // 12574 add rcx,0x38
            { let (r, f) = add::<64>(rcx, 0x38u64);
              rcx = r;
              fl = f; }
            pc = 0x12578;
        }
        0x12578 => {
            // block 12578
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 12578 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12578 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 1257b add r14,0x38
            { let (r, f) = add::<64>(r14, 0x38u64);
              r14 = r;
              fl = f; }
            // 1257f cmp rcx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 12583 mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 12587 jne 0x1253d
            if fl.ne() { pc = 0x1253d; continue; }
            pc = 0x12589;
        }
        0x12589 => {
            // block 12589
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 12589 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12589 cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 1258c je 0x125dd
            if fl.e() { pc = 0x125dd; continue; }
            pc = 0x1258e;
        }
        0x1258e => {
            // block 1258e
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1258e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1258e mov rdi,r14
            rdi = r14;
            // 12591 mov rsi,r12
            rsi = r12;
            // 12594 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12599 add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 1259d add r14,0x38
            { let (r, f) = add::<64>(r14, 0x38u64);
              r14 = r;
              fl = f; }
            // 125a1 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 125a4 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 125a7 jne 0x1258e
            if fl.ne() { pc = 0x1258e; continue; }
            pc = 0x125a9;
        }
        0x125a9 => {
            // block 125a9
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125a9 jmp 0x125dd
            pc = 0x125dd; continue;
        }
        0x125ab => {
            // block 125ab
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125ab cmp rcx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 125af je 0x125dd
            if fl.e() { pc = 0x125dd; continue; }
            pc = 0x125b1;
        }
        0x125b1 => {
            // block 125b1
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125b1 sub QWORD PTR [rbp-0x30],rcx
            { let (r, f) = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, rcx);
              m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r) as u64);
              fl = f; }
            // 125b5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 125b8 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x125bb;
        }
        0x125bb => {
            // block 125bb
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125bb mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 125bf lea rdi,[r14+r13*1]
            rdi = r14.wrapping_add(r13);
            // 125c3 lea rsi,[rcx+r13*1]
            rsi = rcx.wrapping_add(r13);
            // 125c7 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 125cc add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 125d0 dec r12
            { let (r, f) = dec::<64>(r12, fl); r12 = r; fl = f; }
            // 125d3 cmp QWORD PTR [rbp-0x30],r13
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, r13).1;
            // 125d7 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 125db jne 0x125bb
            if fl.ne() { pc = 0x125bb; continue; }
            pc = 0x125dd;
        }
        0x125dd => {
            // block 125dd
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125dd add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 125e1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 125e2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 125e4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 125e6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 125e8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 125ea pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 125eb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x125ec => {
            // block 125ec
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125ec jmp 0x125ee
            pc = 0x125ee; continue;
        }
        0x125ee => {
            // block 125ee
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125ee mov r14,rax
            r14 = rax;
            pc = 0x125f1;
        }
        0x125f1 => {
            // block 125f1
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125f1 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 125f4 je 0x1260c
            if fl.e() { pc = 0x1260c; continue; }
            pc = 0x125f6;
        }
        0x125f6 => {
            // block 125f6
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125f6 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 125f9 je 0x1260c
            if fl.e() { pc = 0x1260c; continue; }
            pc = 0x125fb;
        }
        0x125fb => {
            // block 125fb
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125fb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 125fe mov rdi,rbx
            rdi = rbx;
            // 12601 call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12603 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12607 dec r15
            { let (r, f) = dec::<64>(r15, fl); r15 = r; fl = f; }
            // 1260a jne 0x125fb
            if fl.ne() { pc = 0x125fb; continue; }
            pc = 0x1260c;
        }
        0x1260c => {
            // block 1260c
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1260c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1260c mov rdi,r14
            rdi = r14;
            // 1260f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12614 mov r14,rax
            r14 = rax;
            // 12617 sub r15,r12
            { let (r, f) = sub::<64>(r15, r12);
              r15 = r;
              fl = f; }
            // 1261a jmp 0x125f1
            pc = 0x125f1; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7c30 size=268
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007c30<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x7c30;
    loop {
        match pc {
        0x7c30 => {
            // block 7c30
            #[cfg(feature = "trace")] eprintln!("f_00007c30 pc 7c30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c30 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7c31 mov rbp,rsp
            rbp = rsp;
            // 7c34 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7c36 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7c38 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 7c3a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 7c3c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7c3d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 7c3e mov r14d,r9d
            r14 = ((r9 as u32 as u64)) as u32 as u64;
            // 7c41 mov r12,rcx
            r12 = rcx;
            // 7c44 mov r13,rdx
            r13 = rdx;
            // 7c47 mov rbx,rsi
            rbx = rsi;
            // 7c4a mov r15,rdi
            r15 = rdi;
            // 7c4d mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 7c51 sub rax,QWORD PTR [rbx]
            { let (r, f) = sub::<64>(rax, m.r64(rbx) as u64);
              rax = r;
              fl = f; }
            // 7c54 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 7c58 cmp rax,0x400
            fl = sub::<64>(rax, 0x400u64).1;
            // 7c5e jb 0x7c89
            if fl.b() { pc = 0x7c89; continue; }
            pc = 0x7c60;
        }
        0x7c60 => {
            // block 7c60
            #[cfg(feature = "trace")] eprintln!("f_00007c30 pc 7c60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c60 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // 7c64 sub rax,QWORD PTR [r13+0x0]
            { let (r, f) = sub::<64>(rax, m.r64(r13) as u64);
              rax = r;
              fl = f; }
            // 7c68 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 7c6c cmp rax,0x400
            fl = sub::<64>(rax, 0x400u64).1;
            // 7c72 jb 0x7c89
            if fl.b() { pc = 0x7c89; continue; }
            pc = 0x7c74;
        }
        0x7c74 => {
            // block 7c74
            #[cfg(feature = "trace")] eprintln!("f_00007c30 pc 7c74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c74 mov rax,QWORD PTR [r12+0x8]
            rax = m.r64(r12.wrapping_add(0x8)) as u64;
            // 7c79 sub rax,QWORD PTR [r12]
            { let (r, f) = sub::<64>(rax, m.r64(r12) as u64);
              rax = r;
              fl = f; }
            // 7c7d sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 7c81 cmp rax,0x400
            fl = sub::<64>(rax, 0x400u64).1;
            // 7c87 jae 0x7c9a
            if fl.ae() { pc = 0x7c9a; continue; }
            pc = 0x7c89;
        }
        0x7c89 => {
            // block 7c89
            #[cfg(feature = "trace")] eprintln!("f_00007c30 pc 7c89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c89 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x7c8b;
        }
        0x7c8b => {
            // block 7c8b
            #[cfg(feature = "trace")] eprintln!("f_00007c30 pc 7c8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c8b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 7c8f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7c90 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 7c92 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 7c94 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 7c96 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 7c98 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7c99 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x7c9a => {
            // block 7c9a
            #[cfg(feature = "trace")] eprintln!("f_00007c30 pc 7c9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c9a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 7c9d mov rdi,r15
            rdi = r15;
            // 7ca0 mov esi,r8d
            rsi = ((r8 as u32 as u64)) as u32 as u64;
            // 7ca3 mov DWORD PTR [rbp-0x2c],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((r8 as u32 as u64)) as u32);
            // 7ca7 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cad mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 7caf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7cb1 cmp r14d,0x1
            fl = sub::<32>((r14 as u32 as u64), 0x1u64).1;
            // 7cb5 ja 0x7c8b
            if fl.a() { pc = 0x7c8b; continue; }
            pc = 0x7cb7;
        }
        0x7cb7 => {
            // block 7cb7
            #[cfg(feature = "trace")] eprintln!("f_00007c30 pc 7cb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cb7 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 7cb9 jne 0x7c8b
            if fl.ne() { pc = 0x7c8b; continue; }
            pc = 0x7cbb;
        }
        0x7cbb => {
            // block 7cbb
            #[cfg(feature = "trace")] eprintln!("f_00007c30 pc 7cbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cbb mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 7cbf call 0xd0489
            { let r = { let t = m.env.code_va(0xd0489); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cc4 mov cx,ax
            rcx = (rcx & !0xffff) | (((rax as u16 as u64)) as u16 as u64);
            // 7cc7 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 7cc9 test cx,cx
            fl = logic::<16>((rcx as u16 as u64) & (rcx as u16 as u64));
            // 7ccc je 0x7c8b
            if fl.e() { pc = 0x7c8b; continue; }
            pc = 0x7cce;
        }
        0x7cce => {
            // block 7cce
            #[cfg(feature = "trace")] eprintln!("f_00007c30 pc 7cce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cce mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 7cd1 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 7cd6 mov rdi,r15
            rdi = r15;
            // 7cd9 mov edx,DWORD PTR [rbp-0x2c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 7cdc call QWORD PTR [rax+0xf08]
            { let r = { let t = m.r64(rax.wrapping_add(0xf08)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ce2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7ce4 je 0x7d31
            if fl.e() { pc = 0x7d31; continue; }
            pc = 0x7ce6;
        }
        0x7ce6 => {
            // block 7ce6
            #[cfg(feature = "trace")] eprintln!("f_00007c30 pc 7ce6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ce6 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 7ce9 mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 7cec lea rdx,[rip+0xf77bd] # 0xff4b0
            rdx = m.env.data_va(81021);
            // 7cf3 add r14d,DWORD PTR [rdx+rcx*4]
            { let (r, f) = add::<32>((r14 as u32 as u64), m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 7cf7 mov rdi,r15
            rdi = r15;
            // 7cfa mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 7cfd call QWORD PTR [rax+0x1000]
            { let r = { let t = m.r64(rax.wrapping_add(0x1000)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d03 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7d05 je 0x7d1c
            if fl.e() { pc = 0x7d1c; continue; }
            pc = 0x7d07;
        }
        0x7d07 => {
            // block 7d07
            #[cfg(feature = "trace")] eprintln!("f_00007c30 pc 7d07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d07 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 7d0a mov rdi,r15
            rdi = r15;
            // 7d0d mov rsi,rbx
            rsi = rbx;
            // 7d10 mov rdx,r13
            rdx = r13;
            // 7d13 mov rcx,r12
            rcx = r12;
            // 7d16 call QWORD PTR [rax+0xf18]
            { let r = { let t = m.r64(rax.wrapping_add(0xf18)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7d1c;
        }
        0x7d1c => {
            // block 7d1c
            #[cfg(feature = "trace")] eprintln!("f_00007c30 pc 7d1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d1c mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 7d1e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 7d21 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 7d23 mov rdi,r15
            rdi = r15;
            // 7d26 mov edx,DWORD PTR [rbp-0x2c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 7d29 call QWORD PTR [rax+0xf08]
            { let r = { let t = m.r64(rax.wrapping_add(0xf08)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d2f mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            pc = 0x7d31;
        }
        0x7d31 => {
            // block 7d31
            #[cfg(feature = "trace")] eprintln!("f_00007c30 pc 7d31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d31 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7d33 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 7d36 jmp 0x7c8b
            pc = 0x7c8b; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6051c size=263
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006051c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6051c;
    loop {
        match pc {
        0x6051c => {
            // block 6051c
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 6051c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6051c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6051d mov rbp,rsp
            rbp = rsp;
            // 60520 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 60522 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 60524 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 60526 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 60528 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 60529 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 6052d mov r14,rsi
            r14 = rsi;
            // 60530 mov rbx,rdi
            rbx = rdi;
            // 60533 lea rsi,[rip+0xc9f1c] # 0x12a456
            rsi = m.env.data_va(257059);
            // 6053a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 6053f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60544 mov rsi,QWORD PTR [r14+0x10]
            rsi = m.r64(r14.wrapping_add(0x10)) as u64;
            // 60548 mov rdi,rax
            rdi = rax;
            // 6054b call 0xec0a4
            { let r = { let t = m.env.stub_va(0x871); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60550 lea rsi,[rip+0xceb2f] # 0x12f086
            rsi = m.env.data_va(276563);
            // 60557 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 6055c mov rdi,rax
            rdi = rax;
            // 6055f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60564 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // 60567 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 6056b cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 6056e je 0x605ec
            if fl.e() { pc = 0x605ec; continue; }
            pc = 0x60570;
        }
        0x60570 => {
            // block 60570
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 60570 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 60570 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 60574 jmp 0x6058d
            pc = 0x6058d; continue;
        }
        0x60576 => {
            // block 60576
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 60576 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 60576 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 6057b mov rdi,rbx
            rdi = rbx;
            // 6057e lea rsi,[rip+0xc9f18] # 0x12a49d
            rsi = m.env.data_va(257130);
            // 60585 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6058a mov r13,r12
            r13 = r12;
            pc = 0x6058d;
        }
        0x6058d => {
            // block 6058d
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 6058d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6058d mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 60591 mov rdi,r15
            rdi = r15;
            // 60594 call 0x60623
            { let r = { let t = m.env.code_va(0x60623); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60599 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 6059d test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 605a0 jne 0x605ab
            if fl.ne() { pc = 0x605ab; continue; }
            pc = 0x605a2;
        }
        0x605a2 => {
            // block 605a2
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605a2 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 605a5 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 605a9 jmp 0x605b3
            pc = 0x605b3; continue;
        }
        0x605ab => {
            // block 605ab
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605ab mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 605af mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x605b3;
        }
        0x605b3 => {
            // block 605b3
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605b3 mov rdi,rbx
            rdi = rbx;
            // 605b6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 605bb mov rdi,r15
            rdi = r15;
            // 605be call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 605c3 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // 605c7 test rax,rax
            fl = logic::<64>(rax & rax);
            // 605ca je 0x605da
            if fl.e() { pc = 0x605da; continue; }
            pc = 0x605cc;
        }
        0x605cc => {
            // block 605cc
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605cc mov r12,rax
            r12 = rax;
            // 605cf mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 605d3 test rax,rax
            fl = logic::<64>(rax & rax);
            // 605d6 jne 0x605cc
            if fl.ne() { pc = 0x605cc; continue; }
            pc = 0x605d8;
        }
        0x605d8 => {
            // block 605d8
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605d8 jmp 0x605e7
            pc = 0x605e7; continue;
        }
        0x605da => {
            // block 605da
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605da mov r12,QWORD PTR [r13+0x10]
            r12 = m.r64(r13.wrapping_add(0x10)) as u64;
            // 605de cmp QWORD PTR [r12],r13
            fl = sub::<64>(m.r64(r12) as u64, r13).1;
            // 605e2 mov r13,r12
            r13 = r12;
            // 605e5 jne 0x605da
            if fl.ne() { pc = 0x605da; continue; }
            pc = 0x605e7;
        }
        0x605e7 => {
            // block 605e7
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605e7 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 605ea jne 0x60576
            if fl.ne() { pc = 0x60576; continue; }
            pc = 0x605ec;
        }
        0x605ec => {
            // block 605ec
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605ec lea rsi,[rip+0xc9e61] # 0x12a454
            rsi = m.env.data_va(257057);
            // 605f3 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 605f8 mov rdi,rbx
            rdi = rbx;
            // 605fb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60600 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 60604 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 60605 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 60607 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 60609 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6060b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6060d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6060e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x6060f => {
            // block 6060f
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 6060f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6060f mov rbx,rax
            rbx = rax;
            // 60612 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 60616 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6061b mov rdi,rbx
            rdi = rbx;
            // 6061e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18b50 size=253
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00018b50<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x18b50;
    loop {
        match pc {
        0x18b50 => {
            // block 18b50
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b50 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18b51 mov rbp,rsp
            rbp = rsp;
            // 18b54 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 18b56 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 18b58 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 18b5a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 18b5c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 18b5d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 18b5e mov rbx,rsi
            rbx = rsi;
            // 18b61 mov r14,rdi
            r14 = rdi;
            // 18b64 mov r13,QWORD PTR [rbx]
            r13 = m.r64(rbx) as u64;
            // 18b67 cmp r13,QWORD PTR [rbx+0x8]
            fl = sub::<64>(r13, m.r64(rbx.wrapping_add(0x8)) as u64).1;
            // 18b6b je 0x18c3b
            if fl.e() { pc = 0x18c3b; continue; }
            pc = 0x18b71;
        }
        0x18b71 => {
            // block 18b71
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b71 lea r12,[rip+0x111741] # 0x12a2b9
            r12 = m.env.data_va(256646);
            // 18b78 lea r15,[rip+0x11172f] # 0x12a2ae
            r15 = m.env.data_va(256635);
            pc = 0x18b7f;
        }
        0x18b7f => {
            // block 18b7f
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b7f mov eax,DWORD PTR [r13+0x0]
            rax = (m.r32(r13) as u64) as u32 as u64;
            // 18b83 cmp eax,0x6e6f696d
            fl = sub::<32>((rax as u32 as u64), 0x6e6f696du64).1;
            // 18b88 jg 0x18ba9
            if fl.g() { pc = 0x18ba9; continue; }
            pc = 0x18b8a;
        }
        0x18b8a => {
            // block 18b8a
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b8a cmp eax,0x69616c66
            fl = sub::<32>((rax as u32 as u64), 0x69616c66u64).1;
            // 18b8f jg 0x18bbd
            if fl.g() { pc = 0x18bbd; continue; }
            pc = 0x18b91;
        }
        0x18b91 => {
            // block 18b91
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b91 cmp eax,0x69616474
            fl = sub::<32>((rax as u32 as u64), 0x69616474u64).1;
            // 18b96 jne 0x18bed
            if fl.ne() { pc = 0x18bed; continue; }
            pc = 0x18b98;
        }
        0x18b98 => {
            // block 18b98
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b98 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 18b9d mov rdi,r14
            rdi = r14;
            // 18ba0 lea rsi,[rip+0x1116f9] # 0x12a2a0
            rsi = m.env.data_va(256621);
            // 18ba7 jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18ba9 => {
            // block 18ba9
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18ba9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18ba9 cmp eax,0x6e6f696e
            fl = sub::<32>((rax as u32 as u64), 0x6e6f696eu64).1;
            // 18bae jne 0x18bd5
            if fl.ne() { pc = 0x18bd5; continue; }
            pc = 0x18bb0;
        }
        0x18bb0 => {
            // block 18bb0
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bb0 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 18bb5 mov rdi,r14
            rdi = r14;
            // 18bb8 mov rsi,r15
            rsi = r15;
            // 18bbb jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18bbd => {
            // block 18bbd
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bbd cmp eax,0x69616c67
            fl = sub::<32>((rax as u32 as u64), 0x69616c67u64).1;
            // 18bc2 jne 0x18c05
            if fl.ne() { pc = 0x18c05; continue; }
            pc = 0x18bc4;
        }
        0x18bc4 => {
            // block 18bc4
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bc4 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 18bc9 mov rdi,r14
            rdi = r14;
            // 18bcc lea rsi,[rip+0x1116d3] # 0x12a2a6
            rsi = m.env.data_va(256627);
            // 18bd3 jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18bd5 => {
            // block 18bd5
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bd5 cmp eax,0x6f757420
            fl = sub::<32>((rax as u32 as u64), 0x6f757420u64).1;
            // 18bda jne 0x18c1d
            if fl.ne() { pc = 0x18c1d; continue; }
            pc = 0x18bdc;
        }
        0x18bdc => {
            // block 18bdc
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bdc mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18be1 mov rdi,r14
            rdi = r14;
            // 18be4 lea rsi,[rip+0x1116c9] # 0x12a2b4
            rsi = m.env.data_va(256641);
            // 18beb jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18bed => {
            // block 18bed
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bed cmp eax,0x69616573
            fl = sub::<32>((rax as u32 as u64), 0x69616573u64).1;
            // 18bf2 jne 0x18c1d
            if fl.ne() { pc = 0x18c1d; continue; }
            pc = 0x18bf4;
        }
        0x18bf4 => {
            // block 18bf4
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bf4 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18bf9 mov rdi,r14
            rdi = r14;
            // 18bfc lea rsi,[rip+0x111698] # 0x12a29b
            rsi = m.env.data_va(256616);
            // 18c03 jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18c05 => {
            // block 18c05
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c05 cmp eax,0x69736469
            fl = sub::<32>((rax as u32 as u64), 0x69736469u64).1;
            // 18c0a jne 0x18c1d
            if fl.ne() { pc = 0x18c1d; continue; }
            pc = 0x18c0c;
        }
        0x18c0c => {
            // block 18c0c
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c0c mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18c11 mov rdi,r14
            rdi = r14;
            // 18c14 lea rsi,[rip+0x11167b] # 0x12a296
            rsi = m.env.data_va(256611);
            // 18c1b jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18c1d => {
            // block 18c1d
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c1d mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18c22 mov rdi,r14
            rdi = r14;
            // 18c25 mov rsi,r12
            rsi = r12;
            pc = 0x18c28;
        }
        0x18c28 => {
            // block 18c28
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c28 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18c2d add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // 18c31 cmp r13,QWORD PTR [rbx+0x8]
            fl = sub::<64>(r13, m.r64(rbx.wrapping_add(0x8)) as u64).1;
            // 18c35 jne 0x18b7f
            if fl.ne() { pc = 0x18b7f; continue; }
            pc = 0x18c3b;
        }
        0x18c3b => {
            // block 18c3b
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c3b mov rax,r14
            rax = r14;
            // 18c3e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 18c42 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 18c43 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 18c45 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 18c47 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18c49 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18c4b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18c4c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9356 size=244
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9356<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb9356;
    loop {
        match pc {
        0xb9356 => {
            // block b9356
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9356 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9356 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9357 mov rbp,rsp
            rbp = rsp;
            // b935a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b935c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b935e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b9360 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b9362 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9363 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b9367 mov r13,rdx
            r13 = rdx;
            // b936a mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // b936d mov r14,rdi
            r14 = rdi;
            // b9370 lea rbx,[r13+0x8]
            rbx = r13.wrapping_add(0x8);
            // b9374 mov rsi,QWORD PTR [r13+0x8]
            rsi = m.r64(r13.wrapping_add(0x8)) as u64;
            // b9378 mov rdi,r13
            rdi = r13;
            // b937b call 0xebcf0
            { let r = { let t = m.env.stub_va(0x4bd); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9380 mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // b9388 mov QWORD PTR [r13+0x0],rbx
            m.w64(r13, (rbx) as u64);
            // b938c mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // b9394 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9398 call 0xcbc54
            { let r = { let t = m.env.code_va(0xcbc54); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b939d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b939f je 0xb9438
            if fl.e() { pc = 0xb9438; continue; }
            pc = 0xb93a5;
        }
        0xb93a5 => {
            // block b93a5
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93a5 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b93a9 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b93ae test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b93b0 je 0xb9438
            if fl.e() { pc = 0xb9438; continue; }
            pc = 0xb93b6;
        }
        0xb93b6 => {
            // block b93b6
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93b6 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b93ba call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b93bf xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // b93c2 movzx ecx,r12w
            rcx = ((r12 as u16 as u64)) as u32 as u64;
            // b93c6 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // b93c9 cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // b93cb mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // b93d0 jbe 0xb943a
            if fl.be() { pc = 0xb943a; continue; }
            pc = 0xb93d2;
        }
        0xb93d2 => {
            // block b93d2
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93d2 lea rax,[rip+0x495d7] # 0x1029b0
            rax = m.env.data_va(94589);
            // b93d9 mov eax,DWORD PTR [rax+rcx*4]
            rax = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // b93dc mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // b93e0 lea r12,[rbp-0x2d]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd3);
            pc = 0xb93e4;
        }
        0xb93e4 => {
            // block b93e4
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93e4 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // b93eb mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b93ee mov rcx,QWORD PTR [rbp-0x38]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b93f2 lea esi,[r15+rcx*1]
            rsi = (r15.wrapping_add(rcx)) as u32 as u64;
            // b93f6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // b93f8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b93fd xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b9400 mov rdi,r14
            rdi = r14;
            // b9403 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // b9407 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb940a;
        }
        0xb940a => {
            // block b940a
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b940a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b940a mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // b940d mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // b940f shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // b9411 mov BYTE PTR [rbp-0x2d],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3), ((rax as u8 as u64)) as u8);
            // b9414 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b9416 je 0xb9423
            if fl.e() { pc = 0xb9423; continue; }
            pc = 0xb9418;
        }
        0xb9418 => {
            // block b9418
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9418 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9418 mov rdi,r13
            rdi = r13;
            // b941b mov rsi,r12
            rsi = r12;
            // b941e call 0xebce4
            { let r = { let t = m.env.stub_va(0x4b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb9423;
        }
        0xb9423 => {
            // block b9423
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9423 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9423 add ebx,0x8
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x8u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // b9426 cmp ebx,0x20
            fl = sub::<32>((rbx as u32 as u64), 0x20u64).1;
            // b9429 jne 0xb940a
            if fl.ne() { pc = 0xb940a; continue; }
            pc = 0xb942b;
        }
        0xb942b => {
            // block b942b
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b942b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b942b inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // b942e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // b9430 cmp r15d,0x5
            fl = sub::<32>((r15 as u32 as u64), 0x5u64).1;
            // b9434 jne 0xb93e4
            if fl.ne() { pc = 0xb93e4; continue; }
            pc = 0xb9436;
        }
        0xb9436 => {
            // block b9436
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9436 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9436 jmp 0xb943a
            pc = 0xb943a; continue;
        }
        0xb9438 => {
            // block b9438
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9438 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb943a;
        }
        0xb943a => {
            // block b943a
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b943a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b943a add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b943e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b943f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b9441 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b9443 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9445 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9447 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9448 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5556 size=240
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5556<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb5556;
    loop {
        match pc {
        0xb5556 => {
            // block b5556
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5556 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5556 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5557 mov rbp,rsp
            rbp = rsp;
            // b555a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b555c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b555e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b5560 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5562 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5563 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b5567 mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b556b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b556e je 0xb5621
            if fl.e() { pc = 0xb5621; continue; }
            pc = 0xb5574;
        }
        0xb5574 => {
            // block b5574
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5574 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5574 mov QWORD PTR [rbp-0x48],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rsi) as u64);
            // b5578 movzx r15d,BYTE PTR [rdx]
            r15 = (m.r8(rdx) as u64) as u32 as u64;
            // b557c mov r12b,r15b
            r12 = (r12 & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // b557f and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5583 shr r15,1
            { let (r, f) = shr::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // b5586 mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // b558a mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // b558e mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // b5592 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // b5596 lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // b559a mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // b559e jmp 0xb55a3
            pc = 0xb55a3; continue;
        }
        0xb55a0 => {
            // block b55a0
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55a0 mov rbx,rax
            rbx = rax;
            pc = 0xb55a3;
        }
        0xb55a3 => {
            // block b55a3
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55a3 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // b55a6 mov r13,QWORD PTR [rbp-0x30]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b55aa cmove r13,r15
            if fl.e() { r13 = r15; }
            // b55ae movzx r14d,BYTE PTR [rbx+0x20]
            r14 = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // b55b3 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b55b6 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b55b8 jne 0xb55bf
            if fl.ne() { pc = 0xb55bf; continue; }
            pc = 0xb55ba;
        }
        0xb55ba => {
            // block b55ba
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55ba shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // b55bd jmp 0xb55c3
            pc = 0xb55c3; continue;
        }
        0xb55bf => {
            // block b55bf
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55bf mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xb55c3;
        }
        0xb55c3 => {
            // block b55c3
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55c3 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // b55c6 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b55ca cmove rdi,QWORD PTR [rbp-0x40]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64; }
            // b55cf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b55d1 jne 0xb55dc
            if fl.ne() { pc = 0xb55dc; continue; }
            pc = 0xb55d3;
        }
        0xb55d3 => {
            // block b55d3
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55d3 lea rsi,[rbx+0x20]
            rsi = rbx.wrapping_add(0x20);
            // b55d7 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // b55da jmp 0xb55e0
            pc = 0xb55e0; continue;
        }
        0xb55dc => {
            // block b55dc
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55dc mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xb55e0;
        }
        0xb55e0 => {
            // block b55e0
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55e0 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // b55e3 mov rdx,r13
            rdx = r13;
            // b55e6 cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // b55ea test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b55ed je 0xb55fc
            if fl.e() { pc = 0xb55fc; continue; }
            pc = 0xb55ef;
        }
        0xb55ef => {
            // block b55ef
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55ef call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b55f4 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b55f6 je 0xb55fc
            if fl.e() { pc = 0xb55fc; continue; }
            pc = 0xb55f8;
        }
        0xb55f8 => {
            // block b55f8
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55f8 jns 0xb560b
            if fl.ns() { pc = 0xb560b; continue; }
            pc = 0xb55fa;
        }
        0xb55fa => {
            // block b55fa
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55fa jmp 0xb5601
            pc = 0xb5601; continue;
        }
        0xb55fc => {
            // block b55fc
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55fc cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // b55ff jae 0xb560b
            if fl.ae() { pc = 0xb560b; continue; }
            pc = 0xb5601;
        }
        0xb5601 => {
            // block b5601
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5601 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5601 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b5604 test rax,rax
            fl = logic::<64>(rax & rax);
            // b5607 jne 0xb55a0
            if fl.ne() { pc = 0xb55a0; continue; }
            pc = 0xb5609;
        }
        0xb5609 => {
            // block b5609
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5609 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5609 jmp 0xb562d
            pc = 0xb562d; continue;
        }
        0xb560b => {
            // block b560b
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b560b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b560b mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b560f test rax,rax
            fl = logic::<64>(rax & rax);
            // b5612 jne 0xb55a0
            if fl.ne() { pc = 0xb55a0; continue; }
            pc = 0xb5614;
        }
        0xb5614 => {
            // block b5614
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5614 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5614 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b5618 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // b561b add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // b561f jmp 0xb5634
            pc = 0xb5634; continue;
        }
        0xb5621 => {
            // block b5621
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5621 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5621 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // b5625 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // b5628 mov rbx,rdi
            rbx = rdi;
            // b562b jmp 0xb5634
            pc = 0xb5634; continue;
        }
        0xb562d => {
            // block b562d
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b562d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b562d mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b5631 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            pc = 0xb5634;
        }
        0xb5634 => {
            // block b5634
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5634 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5634 mov rax,rbx
            rax = rbx;
            // b5637 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b563b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b563c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b563e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b5640 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5642 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b5644 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5645 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb799c size=236
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000b799c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb799c;
    loop {
        match pc {
        0xb799c => {
            // block b799c
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b799c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b799c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b799d mov rbp,rsp
            rbp = rsp;
            // b79a0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b79a2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b79a4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b79a5 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b79a9 mov rax,rsi
            rax = rsi;
            // b79ac mov r14,rdi
            r14 = rdi;
            // b79af mov r15,QWORD PTR [rip+0x88a22] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // b79b6 mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // b79b9 mov QWORD PTR [rbp-0x20],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (r15) as u64);
            // b79bd mov rcx,rax
            rcx = rax;
            // b79c0 shr rcx,0x20
            { let (r, f) = shr::<64>(rcx, 0x20u64, fl); rcx = r; fl = f; }
            // b79c4 mov BYTE PTR [rbp-0x29],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd7), ((rax as u8 as u64)) as u8);
            // b79c7 mov BYTE PTR [rbp-0x28],ah
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (((rax >> 8) as u8 as u64)) as u8);
            // b79ca mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // b79cc shr edx,0x10
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b79cf mov BYTE PTR [rbp-0x27],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd9), ((rdx as u8 as u64)) as u8);
            // b79d2 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // b79d4 shr edx,0x18
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x18u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b79d7 mov BYTE PTR [rbp-0x26],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffda), ((rdx as u8 as u64)) as u8);
            // b79da mov BYTE PTR [rbp-0x25],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb), ((rcx as u8 as u64)) as u8);
            // b79dd mov rcx,rax
            rcx = rax;
            // b79e0 shr rcx,0x28
            { let (r, f) = shr::<64>(rcx, 0x28u64, fl); rcx = r; fl = f; }
            // b79e4 mov BYTE PTR [rbp-0x24],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rcx as u8 as u64)) as u8);
            // b79e7 mov rcx,rax
            rcx = rax;
            // b79ea shr rcx,0x30
            { let (r, f) = shr::<64>(rcx, 0x30u64, fl); rcx = r; fl = f; }
            // b79ee mov BYTE PTR [rbp-0x23],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdd), ((rcx as u8 as u64)) as u8);
            // b79f1 mov rcx,rax
            rcx = rax;
            // b79f4 shr rcx,0x38
            { let (r, f) = shr::<64>(rcx, 0x38u64, fl); rcx = r; fl = f; }
            // b79f8 mov BYTE PTR [rbp-0x22],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffde), ((rcx as u8 as u64)) as u8);
            // b79fb mov BYTE PTR [rbp-0x21],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf), (0x0u64) as u8);
            // b79ff test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7a01 je 0xb7a5d
            if fl.e() { pc = 0xb7a5d; continue; }
            pc = 0xb7a03;
        }
        0xb7a03 => {
            // block b7a03
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a03 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xb7a05;
        }
        0xb7a05 => {
            // block b7a05
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a05 mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b7a07 and dl,0xdf
            { let r = (rdx as u8 as u64) & 0xdfu64; let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7a0a add dl,0xbf
            { let (r, f) = add::<8>((rdx as u8 as u64), 0xbfu64);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7a0d movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // b7a10 cmp edx,0x1a
            fl = sub::<32>((rdx as u32 as u64), 0x1au64).1;
            // b7a13 jb 0xb7a2f
            if fl.b() { pc = 0xb7a2f; continue; }
            pc = 0xb7a15;
        }
        0xb7a15 => {
            // block b7a15
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a15 mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b7a17 add dl,0xd0
            { let (r, f) = add::<8>((rdx as u8 as u64), 0xd0u64);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7a1a movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // b7a1d cmp edx,0xa
            fl = sub::<32>((rdx as u32 as u64), 0xau64).1;
            // b7a20 jb 0xb7a2f
            if fl.b() { pc = 0xb7a2f; continue; }
            pc = 0xb7a22;
        }
        0xb7a22 => {
            // block b7a22
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a22 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // b7a25 cmp eax,0x20
            fl = sub::<32>((rax as u32 as u64), 0x20u64).1;
            // b7a28 je 0xb7a2f
            if fl.e() { pc = 0xb7a2f; continue; }
            pc = 0xb7a2a;
        }
        0xb7a2a => {
            // block b7a2a
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a2a cmp eax,0x2d
            fl = sub::<32>((rax as u32 as u64), 0x2du64).1;
            // b7a2d jne 0xb7a5d
            if fl.ne() { pc = 0xb7a5d; continue; }
            pc = 0xb7a2f;
        }
        0xb7a2f => {
            // block b7a2f
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a2f lea eax,[rcx+0x1]
            rax = (rcx.wrapping_add(0x1)) as u32 as u64;
            // b7a32 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // b7a35 ja 0xb7a46
            if fl.a() { pc = 0xb7a46; continue; }
            pc = 0xb7a37;
        }
        0xb7a37 => {
            // block b7a37
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a37 mov al,BYTE PTR [rbp+rcx*1-0x28]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(rcx).wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u8 as u64);
            // b7a3b inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // b7a3e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7a40 jne 0xb7a05
            if fl.ne() { pc = 0xb7a05; continue; }
            pc = 0xb7a42;
        }
        0xb7a42 => {
            // block b7a42
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a42 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // b7a44 je 0xb7a5d
            if fl.e() { pc = 0xb7a5d; continue; }
            pc = 0xb7a46;
        }
        0xb7a46 => {
            // block b7a46
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a46 lea rbx,[rbp-0x29]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffd7);
            // b7a4a mov rdi,rbx
            rdi = rbx;
            // b7a4d call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7a52 mov rdi,r14
            rdi = r14;
            // b7a55 mov rsi,rbx
            rsi = rbx;
            // b7a58 mov rdx,rax
            rdx = rax;
            // b7a5b jmp 0xb7a69
            pc = 0xb7a69; continue;
        }
        0xb7a5d => {
            // block b7a5d
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a5d lea rsi,[rip+0x81264] # 0x138cc8
            rsi = m.env.data_va(316565);
            // b7a64 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b7a66 mov rdi,r14
            rdi = r14;
            pc = 0xb7a69;
        }
        0xb7a69 => {
            // block b7a69
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a69 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7a6e cmp r15,QWORD PTR [rbp-0x20]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // b7a72 jne 0xb7a82
            if fl.ne() { pc = 0xb7a82; continue; }
            pc = 0xb7a74;
        }
        0xb7a74 => {
            // block b7a74
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a74 mov rax,r14
            rax = r14;
            // b7a77 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b7a7b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7a7c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7a7e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b7a80 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7a81 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb7a82 => {
            // block b7a82
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a82 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x616c size=228
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000616c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x616c;
    loop {
        match pc {
        0x616c => {
            // block 616c
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 616c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 616c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 616d mov rbp,rsp
            rbp = rsp;
            // 6170 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6172 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6174 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6175 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6176 mov r14,rdx
            r14 = rdx;
            // 6179 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 617b mov r15,rdi
            r15 = rdi;
            // 617e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6181 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6187 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6189 jne 0x61b6
            if fl.ne() { pc = 0x61b6; continue; }
            pc = 0x618b;
        }
        0x618b => {
            // block 618b
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 618b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 618b mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 618f call 0xce76c
            { let r = { let t = m.env.code_va(0xce76c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6194 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 6197 jne 0x61af
            if fl.ne() { pc = 0x61af; continue; }
            pc = 0x6199;
        }
        0x6199 => {
            // block 6199
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 6199 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6199 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 619c mov rdi,r15
            rdi = r15;
            // 619f mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 61a1 mov rdx,r14
            rdx = r14;
            // 61a4 call QWORD PTR [rax+0xf48]
            { let r = { let t = m.r64(rax.wrapping_add(0xf48)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 61aa jmp 0x622f
            pc = 0x622f; continue;
        }
        0x61af => {
            // block 61af
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61af mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 61b1 cmp ebx,0x4
            fl = sub::<32>((rbx as u32 as u64), 0x4u64).1;
            // 61b4 jbe 0x61ba
            if fl.be() { pc = 0x61ba; continue; }
            pc = 0x61b6;
        }
        0x61b6 => {
            // block 61b6
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61b6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 61b8 jmp 0x622f
            pc = 0x622f; continue;
        }
        0x61ba => {
            // block 61ba
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61ba lea rcx,[rip+0x7b] # 0x623c
            rcx = m.env.code_va(0x623c);
            // 61c1 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 61c5 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 61c8 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x61ca => {
            // block 61ca
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61ca mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 61cd mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 61d2 jmp 0x61dc
            pc = 0x61dc; continue;
        }
        0x61d4 => {
            // block 61d4
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61d4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 61d7 mov esi,0x45
            rsi = (0x45u64) as u32 as u64;
            pc = 0x61dc;
        }
        0x61dc => {
            // block 61dc
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61dc mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 61e1 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 61e7 jmp 0x6226
            pc = 0x6226; continue;
        }
        0x61e9 => {
            // block 61e9
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61e9 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 61ec mov esi,0x45
            rsi = (0x45u64) as u32 as u64;
            // 61f1 mov ecx,0x40000000
            rcx = (0x40000000u64) as u32 as u64;
            // 61f6 mov r8d,0x1e
            r8 = (0x1eu64) as u32 as u64;
            // 61fc jmp 0x6226
            pc = 0x6226; continue;
        }
        0x61fe => {
            // block 61fe
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61fe mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6201 mov esi,0x45
            rsi = (0x45u64) as u32 as u64;
            // 6206 mov ecx,0x80000000
            rcx = (0x80000000u64) as u32 as u64;
            // 620b mov r8d,0x1f
            r8 = (0x1fu64) as u32 as u64;
            // 6211 jmp 0x6226
            pc = 0x6226; continue;
        }
        0x6213 => {
            // block 6213
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 6213 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6213 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6216 mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 621b mov ecx,0x200000
            rcx = (0x200000u64) as u32 as u64;
            // 6220 mov r8d,0x15
            r8 = (0x15u64) as u32 as u64;
            pc = 0x6226;
        }
        0x6226 => {
            // block 6226
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 6226 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6226 mov rdi,r15
            rdi = r15;
            // 6229 mov rdx,r14
            rdx = r14;
            // 622c call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x622f;
        }
        0x622f => {
            // block 622f
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 622f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 622f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6233 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6234 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6236 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6238 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6239 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x64a17 size=226
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00064a17<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x64a17;
    loop {
        match pc {
        0x64a17 => {
            // block 64a17
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64a17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a17 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 64a18 mov rbp,rsp
            rbp = rsp;
            // 64a1b push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 64a1d push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 64a1f push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 64a21 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 64a22 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 64a26 mov r14,rsi
            r14 = rsi;
            // 64a29 mov r12,rdi
            r12 = rdi;
            // 64a2c mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 64a35 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 64a3e mov QWORD PTR [r12],0x0
            m.w64(r12, (0x0u64) as u64);
            // 64a46 mov rbx,QWORD PTR [r14]
            rbx = m.r64(r14) as u64;
            // 64a49 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 64a4d cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 64a50 je 0x64ad6
            if fl.e() { pc = 0x64ad6; continue; }
            pc = 0x64a56;
        }
        0x64a56 => {
            // block 64a56
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64a56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a56 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 64a58 lea r15,[rbp-0x30]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 64a5c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 64a5e jmp 0x64a6d
            pc = 0x64a6d; continue;
        }
        0x64a60 => {
            // block 64a60
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64a60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a60 mov rax,QWORD PTR [r12+0x8]
            rax = m.r64(r12.wrapping_add(0x8)) as u64;
            // 64a65 mov rdx,QWORD PTR [r12+0x10]
            rdx = m.r64(r12.wrapping_add(0x10)) as u64;
            // 64a6a mov rbx,rcx
            rbx = rcx;
            pc = 0x64a6d;
        }
        0x64a6d => {
            // block 64a6d
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64a6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a6d mov ecx,DWORD PTR [rbx+0x1c]
            rcx = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 64a70 mov DWORD PTR [rbp-0x30],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rcx as u32 as u64)) as u32);
            // 64a73 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 64a7a mov DWORD PTR [rbp-0x28],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0xffffffffu64) as u32);
            // 64a81 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 64a88 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 64a8b jae 0x64aa4
            if fl.ae() { pc = 0x64aa4; continue; }
            pc = 0x64a8d;
        }
        0x64a8d => {
            // block 64a8d
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64a8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a8d mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 64a91 mov rdx,QWORD PTR [rbp-0x28]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 64a95 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 64a99 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 64a9c add QWORD PTR [r12+0x8],0x10
            { let (r, f) = add::<64>(m.r64(r12.wrapping_add(0x8)) as u64, 0x10u64);
              m.w64(r12.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 64aa2 jmp 0x64aaf
            pc = 0x64aaf; continue;
        }
        0x64aa4 => {
            // block 64aa4
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64aa4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64aa4 mov rdi,r12
            rdi = r12;
            // 64aa7 mov rsi,r15
            rsi = r15;
            // 64aaa call 0xebbbe
            { let r = { let t = m.env.stub_va(0x38b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x64aaf;
        }
        0x64aaf => {
            // block 64aaf
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64aaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64aaf mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 64ab3 test rax,rax
            fl = logic::<64>(rax & rax);
            // 64ab6 je 0x64ac5
            if fl.e() { pc = 0x64ac5; continue; }
            pc = 0x64ab8;
        }
        0x64ab8 => {
            // block 64ab8
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64ab8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ab8 mov rcx,rax
            rcx = rax;
            // 64abb mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 64abe test rax,rax
            fl = logic::<64>(rax & rax);
            // 64ac1 jne 0x64ab8
            if fl.ne() { pc = 0x64ab8; continue; }
            pc = 0x64ac3;
        }
        0x64ac3 => {
            // block 64ac3
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64ac3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ac3 jmp 0x64ad1
            pc = 0x64ad1; continue;
        }
        0x64ac5 => {
            // block 64ac5
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64ac5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ac5 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 64ac9 cmp QWORD PTR [rcx],rbx
            fl = sub::<64>(m.r64(rcx) as u64, rbx).1;
            // 64acc mov rbx,rcx
            rbx = rcx;
            // 64acf jne 0x64ac5
            if fl.ne() { pc = 0x64ac5; continue; }
            pc = 0x64ad1;
        }
        0x64ad1 => {
            // block 64ad1
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64ad1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ad1 cmp rcx,r14
            fl = sub::<64>(rcx, r14).1;
            // 64ad4 jne 0x64a60
            if fl.ne() { pc = 0x64a60; continue; }
            pc = 0x64ad6;
        }
        0x64ad6 => {
            // block 64ad6
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64ad6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ad6 mov rax,r12
            rax = r12;
            // 64ad9 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 64add pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 64ade pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 64ae0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 64ae2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 64ae4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 64ae5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x64ae6 => {
            // block 64ae6
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64ae6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ae6 mov r14,rax
            r14 = rax;
            // 64ae9 mov rdi,r12
            rdi = r12;
            // 64aec call 0xebb7c
            { let r = { let t = m.env.stub_va(0x349); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64af1 mov rdi,r14
            rdi = r14;
            // 64af4 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x103ce size=222
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000103ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x103ce;
    loop {
        match pc {
        0x103ce => {
            // block 103ce
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 103ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 103ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 103cf mov rbp,rsp
            rbp = rsp;
            // 103d2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 103d3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 103d4 mov rbx,rdi
            rbx = rdi;
            // 103d7 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 103db mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 103df mov rsi,QWORD PTR [rbx+0x20]
            rsi = m.r64(rbx.wrapping_add(0x20)) as u64;
            // 103e3 mov rdx,rsi
            rdx = rsi;
            // 103e6 shr rdx,0xa
            { let (r, f) = shr::<64>(rdx, 0xau64, fl); rdx = r; fl = f; }
            // 103ea lea r10,[rax+rdx*8]
            r10 = rax.wrapping_add(rdx.wrapping_mul(8));
            // 103ee sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 103f1 je 0x10424
            if fl.e() { pc = 0x10424; continue; }
            pc = 0x103f3;
        }
        0x103f3 => {
            // block 103f3
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 103f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 103f3 mov rdx,rsi
            rdx = rsi;
            // 103f6 and rdx,0x3ff
            { let r = rdx & 0x3ffu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 103fd shl rdx,0x2
            { let (r, f) = shl::<64>(rdx, 0x2u64, fl); rdx = r; fl = f; }
            // 10401 add rdx,QWORD PTR [r10]
            { let (r, f) = add::<64>(rdx, m.r64(r10) as u64);
              rdx = r;
              fl = f; }
            // 10404 lea r8,[rbx+0x28]
            r8 = rbx.wrapping_add(0x28);
            // 10408 add rsi,QWORD PTR [rbx+0x28]
            { let (r, f) = add::<64>(rsi, m.r64(rbx.wrapping_add(0x28)) as u64);
              rsi = r;
              fl = f; }
            // 1040c mov r9,rsi
            r9 = rsi;
            // 1040f shr r9,0xa
            { let (r, f) = shr::<64>(r9, 0xau64, fl); r9 = r; fl = f; }
            // 10413 and rsi,0x3ff
            { let r = rsi & 0x3ffu64; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // 1041a shl rsi,0x2
            { let (r, f) = shl::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 1041e add rsi,QWORD PTR [rax+r9*8]
            { let (r, f) = add::<64>(rsi, m.r64(rax.wrapping_add(r9.wrapping_mul(8))) as u64);
              rsi = r;
              fl = f; }
            // 10422 jmp 0x10436
            pc = 0x10436; continue;
        }
        0x10424 => {
            // block 10424
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 10424 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10424 lea r8,[rbx+0x28]
            r8 = rbx.wrapping_add(0x28);
            // 10428 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1042a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 1042c jmp 0x10436
            pc = 0x10436; continue;
        }
        0x1042e => {
            // block 1042e
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 1042e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1042e mov rdx,QWORD PTR [r10+0x8]
            rdx = m.r64(r10.wrapping_add(0x8)) as u64;
            // 10432 add r10,0x8
            { let (r, f) = add::<64>(r10, 0x8u64);
              r10 = r;
              fl = f; }
            pc = 0x10436;
        }
        0x10436 => {
            // block 10436
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 10436 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10436 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 10439 je 0x10450
            if fl.e() { pc = 0x10450; continue; }
            pc = 0x1043b;
        }
        0x1043b => {
            // block 1043b
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 1043b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1043b add rdx,0x4
            { let (r, f) = add::<64>(rdx, 0x4u64);
              rdx = r;
              fl = f; }
            // 1043f mov rdi,rdx
            rdi = rdx;
            // 10442 sub rdi,QWORD PTR [r10]
            { let (r, f) = sub::<64>(rdi, m.r64(r10) as u64);
              rdi = r;
              fl = f; }
            // 10445 cmp rdi,0x1000
            fl = sub::<64>(rdi, 0x1000u64).1;
            // 1044c jne 0x10436
            if fl.ne() { pc = 0x10436; continue; }
            pc = 0x1044e;
        }
        0x1044e => {
            // block 1044e
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 1044e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1044e jmp 0x1042e
            pc = 0x1042e; continue;
        }
        0x10450 => {
            // block 10450
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 10450 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10450 mov QWORD PTR [r8],0x0
            m.w64(r8, (0x0u64) as u64);
            // 10457 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 1045b cmp rcx,0x3
            fl = sub::<64>(rcx, 0x3u64).1;
            // 1045f jb 0x10486
            if fl.b() { pc = 0x10486; continue; }
            pc = 0x10461;
        }
        0x10461 => {
            // block 10461
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 10461 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10461 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // 10464 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10469 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1046d add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 10471 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 10475 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 10479 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 1047c sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 10480 cmp rcx,0x2
            fl = sub::<64>(rcx, 0x2u64).1;
            // 10484 ja 0x10461
            if fl.a() { pc = 0x10461; continue; }
            pc = 0x10486;
        }
        0x10486 => {
            // block 10486
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 10486 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10486 cmp rcx,0x2
            fl = sub::<64>(rcx, 0x2u64).1;
            // 1048a jne 0x10496
            if fl.ne() { pc = 0x10496; continue; }
            pc = 0x1048c;
        }
        0x1048c => {
            // block 1048c
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 1048c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1048c mov QWORD PTR [rbx+0x20],0x400
            m.w64(rbx.wrapping_add(0x20), (0x400u64) as u64);
            // 10494 jmp 0x104a4
            pc = 0x104a4; continue;
        }
        0x10496 => {
            // block 10496
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 10496 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10496 cmp rcx,0x1
            fl = sub::<64>(rcx, 0x1u64).1;
            // 1049a jne 0x104a4
            if fl.ne() { pc = 0x104a4; continue; }
            pc = 0x1049c;
        }
        0x1049c => {
            // block 1049c
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 1049c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1049c mov QWORD PTR [rbx+0x20],0x200
            m.w64(rbx.wrapping_add(0x20), (0x200u64) as u64);
            pc = 0x104a4;
        }
        0x104a4 => {
            // block 104a4
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 104a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 104a4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 104a8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 104a9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 104aa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
