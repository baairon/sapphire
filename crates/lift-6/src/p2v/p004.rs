//! Mechanically lifted functions (scripts/lift_run.py), module p004. Each one is checked against the original
//! in the world harness (tests/world_p004.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xea9ce size=462
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ea9ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xea9ce;
    loop {
        match pc {
        0xea9ce => {
            // block ea9ce
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc ea9ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea9ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea9cf mov rbp,rsp
            rbp = rsp;
            // ea9d2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ea9d4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ea9d6 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ea9d8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ea9da push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ea9db sub rsp,0x2d8
            { let (r, f) = sub::<64>(rsp, 0x2d8u64);
              rsp = r;
              fl = f; }
            // ea9e2 mov r14,rdi
            r14 = rdi;
            // ea9e5 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ea9e9 call 0x63dd3
            { let r = { let t = m.env.code_va(0x63dd3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // ea9ee lea r15,[rbp-0x50]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // ea9f2 mov QWORD PTR [rbp-0x48],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0x0u64) as u64);
            // ea9fa mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // eaa02 mov QWORD PTR [rbp-0x58],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r15) as u64);
            // eaa06 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // eaa0a lea r13,[rbp-0x38]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // eaa0e cmp rbx,r13
            fl = sub::<64>(rbx, r13).1;
            // eaa11 je 0xeab17
            if crate::cov::brk(0xeaa11, fl.e()) { pc = 0xeab17; continue; }
            pc = 0xeaa17;
        }
        0xeaa17 => {
            // block eaa17
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa17 lea r12,[rbp-0x58]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            pc = 0xeaa1b;
        }
        0xeaa1b => {
            // block eaa1b
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa1b mov edi,DWORD PTR [rbx+0x1c]
            rdi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // eaa1e call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eaa23 test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // eaa26 je 0xeaa34
            if crate::cov::brk(0xeaa26, fl.e()) { pc = 0xeaa34; continue; }
            pc = 0xeaa28;
        }
        0xeaa28 => {
            // block eaa28
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa28 lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // eaa2c mov rdi,r12
            rdi = r12;
            // eaa2f call 0xebbac
            { let r = { let t = m.env.stub_va(0x379); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xeaa34;
        }
        0xeaa34 => {
            // block eaa34
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa34 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // eaa38 test rax,rax
            fl = logic::<64>(rax & rax);
            // eaa3b je 0xeaa4a
            if crate::cov::brk(0xeaa3b, fl.e()) { pc = 0xeaa4a; continue; }
            pc = 0xeaa3d;
        }
        0xeaa3d => {
            // block eaa3d
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa3d mov rbx,rax
            rbx = rax;
            // eaa40 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // eaa43 test rax,rax
            fl = logic::<64>(rax & rax);
            // eaa46 jne 0xeaa3d
            if crate::cov::brk(0xeaa46, fl.ne()) { pc = 0xeaa3d; continue; }
            pc = 0xeaa48;
        }
        0xeaa48 => {
            // block eaa48
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa48 jmp 0xeaa59
            pc = 0xeaa59; continue;
        }
        0xeaa4a => {
            // block eaa4a
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa4a mov rax,rbx
            rax = rbx;
            pc = 0xeaa4d;
        }
        0xeaa4d => {
            // block eaa4d
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa4d mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // eaa51 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // eaa54 mov rax,rbx
            rax = rbx;
            // eaa57 jne 0xeaa4d
            if crate::cov::brk(0xeaa57, fl.ne()) { pc = 0xeaa4d; continue; }
            pc = 0xeaa59;
        }
        0xeaa59 => {
            // block eaa59
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa59 cmp rbx,r13
            fl = sub::<64>(rbx, r13).1;
            // eaa5c jne 0xeaa1b
            if crate::cov::brk(0xeaa5c, fl.ne()) { pc = 0xeaa1b; continue; }
            pc = 0xeaa5e;
        }
        0xeaa5e => {
            // block eaa5e
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa5e mov rbx,QWORD PTR [rbp-0x58]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // eaa62 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // eaa65 je 0xeab17
            if crate::cov::brk(0xeaa65, fl.e()) { pc = 0xeab17; continue; }
            pc = 0xeaa6b;
        }
        0xeaa6b => {
            // block eaa6b
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa6b lea r12,[rbp-0x2e0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            pc = 0xeaa72;
        }
        0xeaa72 => {
            // block eaa72
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa72 mov rdi,r12
            rdi = r12;
            // eaa75 call 0xb6722
            { let r = { let t = m.env.code_va(0xb6722); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eaa7a mov edi,DWORD PTR [rbx+0x1c]
            rdi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // eaa7d mov rsi,r12
            rsi = r12;
            // eaa80 call 0x1729c
            { let r = { let t = m.env.code_va(0x1729c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eaa85 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaa87 jne 0xeaabd
            if crate::cov::brk(0xeaa87, fl.ne()) { pc = 0xeaabd; continue; }
            pc = 0xeaa89;
        }
        0xeaa89 => {
            // block eaa89
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa89 mov rdi,r12
            rdi = r12;
            // eaa8c call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eaa91 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // eaa95 test rax,rax
            fl = logic::<64>(rax & rax);
            // eaa98 je 0xeaaa7
            if crate::cov::brk(0xeaa98, fl.e()) { pc = 0xeaaa7; continue; }
            pc = 0xeaa9a;
        }
        0xeaa9a => {
            // block eaa9a
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa9a mov rbx,rax
            rbx = rax;
            // eaa9d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // eaaa0 test rax,rax
            fl = logic::<64>(rax & rax);
            // eaaa3 jne 0xeaa9a
            if crate::cov::brk(0xeaaa3, fl.ne()) { pc = 0xeaa9a; continue; }
            pc = 0xeaaa5;
        }
        0xeaaa5 => {
            // block eaaa5
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaaa5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaaa5 jmp 0xeaab6
            pc = 0xeaab6; continue;
        }
        0xeaaa7 => {
            // block eaaa7
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaaa7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaaa7 mov rax,rbx
            rax = rbx;
            pc = 0xeaaaa;
        }
        0xeaaaa => {
            // block eaaaa
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaaaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaaaa mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // eaaae cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // eaab1 mov rax,rbx
            rax = rbx;
            // eaab4 jne 0xeaaaa
            if crate::cov::brk(0xeaab4, fl.ne()) { pc = 0xeaaaa; continue; }
            pc = 0xeaab6;
        }
        0xeaab6 => {
            // block eaab6
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaab6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaab6 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // eaab9 jne 0xeaa72
            if crate::cov::brk(0xeaab9, fl.ne()) { pc = 0xeaa72; continue; }
            pc = 0xeaabb;
        }
        0xeaabb => {
            // block eaabb
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaabb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaabb jmp 0xeab17
            pc = 0xeab17; continue;
        }
        0xeaabd => {
            // block eaabd
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaabd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaabd movzx eax,WORD PTR [rbp-0x2d8]
            rax = (m.r16(rbp.wrapping_add(0xffff_ffff_ffff_fd28)) as u64) as u32 as u64;
            // eaac4 mov DWORD PTR [r14+0x5c],eax
            m.w32(r14.wrapping_add(0x5c), ((rax as u32 as u64)) as u32);
            // eaac8 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // eaacb mov r15,QWORD PTR [rax+0x60]
            r15 = m.r64(rax.wrapping_add(0x60)) as u64;
            // eaacf mov esi,DWORD PTR [rbx+0x1c]
            rsi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // eaad2 lea rdi,[rbp-0x2f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd08);
            // eaad9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eaadb call 0x5c0a9
            { let r = { let t = m.env.code_va(0x5c0a9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eaae0 test BYTE PTR [rbp-0x2f8],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd08)) as u64 & 0x1u64);
            // eaae7 jne 0xeaaf2
            if crate::cov::brk(0xeaae7, fl.ne()) { pc = 0xeaaf2; continue; }
            pc = 0xeaae9;
        }
        0xeaae9 => {
            // block eaae9
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaae9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaae9 lea rsi,[rbp-0x2f7]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd09);
            // eaaf0 jmp 0xeaaf9
            pc = 0xeaaf9; continue;
        }
        0xeaaf2 => {
            // block eaaf2
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaaf2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaaf2 mov rsi,QWORD PTR [rbp-0x2e8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd18)) as u64;
            pc = 0xeaaf9;
        }
        0xeaaf9 => {
            // block eaaf9
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaaf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaaf9 mov rdi,r14
            rdi = r14;
            // eaafc call r15
            { let r = { let t = r15; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eaaff lea rdi,[rbp-0x2f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd08);
            // eab06 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eab0b lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // eab12 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xeab17;
        }
        0xeab17 => {
            // block eab17
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab17 mov rdi,r14
            rdi = r14;
            // eab1a call 0xe09b6
            { let r = { let t = m.env.code_va(0xe09b6); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eab1f mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // eab21 mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // eab25 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // eab29 call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eab2e mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // eab32 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // eab36 call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eab3b mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // eab3d add rsp,0x2d8
            { let (r, f) = add::<64>(rsp, 0x2d8u64);
              rsp = r;
              fl = f; }
            // eab44 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eab45 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eab47 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // eab49 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eab4b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eab4d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eab4e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xeab4f => {
            // block eab4f
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab4f mov rbx,rax
            rbx = rax;
            pc = 0xeab52;
        }
        0xeab52 => {
            // block eab52
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab52 mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // eab56 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // eab5a call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eab5f mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // eab63 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // eab67 call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eab6c mov rdi,rbx
            rdi = rbx;
            // eab6f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eab74 jmp 0xeab4f
            pc = 0xeab4f; continue;
        }
        0xeab76 => {
            // block eab76
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab76 mov rbx,rax
            rbx = rax;
            pc = 0xeab79;
        }
        0xeab79 => {
            // block eab79
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab79 lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // eab80 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eab85 jmp 0xeab52
            pc = 0xeab52; continue;
        }
        0xeab87 => {
            // block eab87
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab87 jmp 0xeab4f
            pc = 0xeab4f; continue;
        }
        0xeab89 => {
            // block eab89
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab89 jmp 0xeab76
            pc = 0xeab76; continue;
        }
        0xeab8b => {
            // block eab8b
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab8b mov rbx,rax
            rbx = rax;
            // eab8e lea rdi,[rbp-0x2f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd08);
            // eab95 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eab9a jmp 0xeab79
            pc = 0xeab79; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdc1f4 size=476
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dc1f4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdc1f4;
    loop {
        match pc {
        0xdc1f4 => {
            // block dc1f4
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc1f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc1f4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dc1f5 mov rbp,rsp
            rbp = rsp;
            // dc1f8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dc1fa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dc1fc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dc1fe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dc1ff sub rsp,0x30
            { let (r, f) = sub::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // dc203 mov r15,rdi
            r15 = rdi;
            // dc206 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dc209 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // dc20b xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dc20d call QWORD PTR [rax+0x18e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x18e8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // dc213 lea r14,[r15+0x258]
            r14 = r15.wrapping_add(0x258);
            // dc21a lea r12,[rip+0xe157f] # 0x1bd7a0
            r12 = m.env.data_va(860013);
            pc = 0xdc221;
        }
        0xdc221 => {
            // block dc221
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc221 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc221 mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            // dc228 cmp rax,QWORD PTR [r15+0x268]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x268)) as u64).1;
            // dc22f je 0xdc242
            if crate::cov::brk(0xdc22f, fl.e()) { pc = 0xdc242; continue; }
            pc = 0xdc231;
        }
        0xdc231 => {
            // block dc231
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc231 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc231 mov rcx,QWORD PTR [rbx+r12*1]
            rcx = m.r64(rbx.wrapping_add(r12)) as u64;
            // dc235 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dc238 add QWORD PTR [r15+0x260],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x260)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x260), (r) as u64);
              fl = f; }
            // dc240 jmp 0xdc24e
            pc = 0xdc24e; continue;
        }
        0xdc242 => {
            // block dc242
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc242 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc242 lea rsi,[r12+rbx*1]
            rsi = r12.wrapping_add(rbx);
            // dc246 mov rdi,r14
            rdi = r14;
            // dc249 call 0xebee2
            { let r = { let t = m.env.stub_va(0x6af); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xdc24e;
        }
        0xdc24e => {
            // block dc24e
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc24e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc24e add rbx,0x608
            { let (r, f) = add::<64>(rbx, 0x608u64);
              rbx = r;
              fl = f; }
            // dc255 cmp rbx,0x3040
            fl = sub::<64>(rbx, 0x3040u64).1;
            // dc25c jne 0xdc221
            if crate::cov::brk(0xdc25c, fl.ne()) { pc = 0xdc221; continue; }
            pc = 0xdc25e;
        }
        0xdc25e => {
            // block dc25e
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc25e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc25e lea rax,[rip+0x50ecf] # 0x12d134
            rax = m.env.data_va(268545);
            // dc265 mov QWORD PTR [rbp-0x28],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rax) as u64);
            // dc269 mov rcx,QWORD PTR [r15+0x260]
            rcx = m.r64(r15.wrapping_add(0x260)) as u64;
            // dc270 cmp rcx,QWORD PTR [r15+0x268]
            fl = sub::<64>(rcx, m.r64(r15.wrapping_add(0x268)) as u64).1;
            // dc277 jae 0xdc290
            if crate::cov::brk(0xdc277, fl.ae()) { pc = 0xdc290; continue; }
            pc = 0xdc279;
        }
        0xdc279 => {
            // block dc279
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc279 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc279 mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // dc27c mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            // dc283 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // dc287 mov QWORD PTR [r15+0x260],rax
            m.w64(r15.wrapping_add(0x260), (rax) as u64);
            // dc28e jmp 0xdc2a3
            pc = 0xdc2a3; continue;
        }
        0xdc290 => {
            // block dc290
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc290 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc290 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // dc294 mov rdi,r14
            rdi = r14;
            // dc297 call 0xebee8
            { let r = { let t = m.env.stub_va(0x6b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // dc29c mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            pc = 0xdc2a3;
        }
        0xdc2a3 => {
            // block dc2a3
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc2a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc2a3 lea rcx,[rip+0x5cf78] # 0x139222
            rcx = m.env.data_va(317935);
            // dc2aa mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // dc2ae cmp rax,QWORD PTR [r15+0x268]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x268)) as u64).1;
            // dc2b5 jae 0xdc2ce
            if crate::cov::brk(0xdc2b5, fl.ae()) { pc = 0xdc2ce; continue; }
            pc = 0xdc2b7;
        }
        0xdc2b7 => {
            // block dc2b7
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc2b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc2b7 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dc2ba mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            // dc2c1 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // dc2c5 mov QWORD PTR [r15+0x260],rax
            m.w64(r15.wrapping_add(0x260), (rax) as u64);
            // dc2cc jmp 0xdc2e1
            pc = 0xdc2e1; continue;
        }
        0xdc2ce => {
            // block dc2ce
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc2ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc2ce lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // dc2d2 mov rdi,r14
            rdi = r14;
            // dc2d5 call 0xebee8
            { let r = { let t = m.env.stub_va(0x6b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // dc2da mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            pc = 0xdc2e1;
        }
        0xdc2e1 => {
            // block dc2e1
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc2e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc2e1 lea rcx,[rip+0x5cf41] # 0x139229
            rcx = m.env.data_va(317942);
            // dc2e8 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // dc2ec cmp rax,QWORD PTR [r15+0x268]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x268)) as u64).1;
            // dc2f3 jae 0xdc30c
            if crate::cov::brk(0xdc2f3, fl.ae()) { pc = 0xdc30c; continue; }
            pc = 0xdc2f5;
        }
        0xdc2f5 => {
            // block dc2f5
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc2f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc2f5 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dc2f8 mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            // dc2ff add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // dc303 mov QWORD PTR [r15+0x260],rax
            m.w64(r15.wrapping_add(0x260), (rax) as u64);
            // dc30a jmp 0xdc31f
            pc = 0xdc31f; continue;
        }
        0xdc30c => {
            // block dc30c
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc30c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc30c lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // dc310 mov rdi,r14
            rdi = r14;
            // dc313 call 0xebee8
            { let r = { let t = m.env.stub_va(0x6b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // dc318 mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            pc = 0xdc31f;
        }
        0xdc31f => {
            // block dc31f
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc31f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc31f lea rcx,[rip+0x5cf0e] # 0x139234
            rcx = m.env.data_va(317953);
            // dc326 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // dc32a cmp rax,QWORD PTR [r15+0x268]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x268)) as u64).1;
            // dc331 jae 0xdc34a
            if crate::cov::brk(0xdc331, fl.ae()) { pc = 0xdc34a; continue; }
            pc = 0xdc333;
        }
        0xdc333 => {
            // block dc333
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc333 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc333 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dc336 mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            // dc33d add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // dc341 mov QWORD PTR [r15+0x260],rax
            m.w64(r15.wrapping_add(0x260), (rax) as u64);
            // dc348 jmp 0xdc35d
            pc = 0xdc35d; continue;
        }
        0xdc34a => {
            // block dc34a
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc34a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc34a lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // dc34e mov rdi,r14
            rdi = r14;
            // dc351 call 0xebee8
            { let r = { let t = m.env.stub_va(0x6b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // dc356 mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            pc = 0xdc35d;
        }
        0xdc35d => {
            // block dc35d
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc35d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc35d lea rcx,[rip+0x5cedf] # 0x139243
            rcx = m.env.data_va(317968);
            // dc364 mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // dc368 cmp rax,QWORD PTR [r15+0x268]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x268)) as u64).1;
            // dc36f jae 0xdc37e
            if crate::cov::brk(0xdc36f, fl.ae()) { pc = 0xdc37e; continue; }
            pc = 0xdc371;
        }
        0xdc371 => {
            // block dc371
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc371 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc371 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dc374 add QWORD PTR [r15+0x260],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x260)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x260), (r) as u64);
              fl = f; }
            // dc37c jmp 0xdc38a
            pc = 0xdc38a; continue;
        }
        0xdc37e => {
            // block dc37e
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc37e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc37e lea rsi,[rbp-0x48]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // dc382 mov rdi,r14
            rdi = r14;
            // dc385 call 0xebee8
            { let r = { let t = m.env.stub_va(0x6b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xdc38a;
        }
        0xdc38a => {
            // block dc38a
            #[cfg(feature = "trace")] eprintln!("f_000dc1f4 pc dc38a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc38a mov WORD PTR [r15+0x270],0x7
            m.w16(r15.wrapping_add(0x270), (0x7u64) as u16);
            // dc394 mov DWORD PTR [r15+0x274],0x0
            m.w32(r15.wrapping_add(0x274), (0x0u64) as u32);
            // dc39f mov BYTE PTR [r15+0x278],0x0
            m.w8(r15.wrapping_add(0x278), (0x0u64) as u8);
            // dc3a7 mov BYTE PTR [r15+0x279],0x1
            m.w8(r15.wrapping_add(0x279), (0x1u64) as u8);
            // dc3af mov BYTE PTR [r15+0x27b],0x1
            m.w8(r15.wrapping_add(0x27b), (0x1u64) as u8);
            // dc3b7 mov QWORD PTR [r15+0x280],0x0
            m.w64(r15.wrapping_add(0x280), (0x0u64) as u64);
            // dc3c2 add rsp,0x30
            { let (r, f) = add::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // dc3c6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dc3c7 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dc3c9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dc3cb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dc3cd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc3ce ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x92e4 size=484
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000092e4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x92e4;
    loop {
        match pc {
        0x92e4 => {
            // block 92e4
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 92e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 92e4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 92e5 mov rbp,rsp
            rbp = rsp;
            // 92e8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 92ea push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 92ec push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 92ee push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 92f0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 92f1 sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 92f5 mov QWORD PTR [rbp-0x88],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rdx) as u64);
            // 92fc mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 92ff mov r14,rdi
            r14 = rdi;
            // 9302 xorps xmm0,xmm0
            x0 = 0;
            // 9305 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 9309 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 930d movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 9311 mov rax,QWORD PTR [rdx]
            rax = m.r64(rdx) as u64;
            // 9314 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 9318 mov rdi,rdx
            rdi = rdx;
            // 931b call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 931d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 9320 mov rax,QWORD PTR [rax+0x1d28]
            rax = m.r64(rax.wrapping_add(0x1d28)) as u64;
            // 9327 mov rdi,r14
            rdi = r14;
            // 932a mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 932d call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 932f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 9331 je 0x933a
            if crate::cov::brk(0x9331, fl.e()) { pc = 0x933a; continue; }
            pc = 0x9333;
        }
        0x9333 => {
            // block 9333
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9333 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9333 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 9335 jmp 0x9485
            pc = 0x9485; continue;
        }
        0x933a => {
            // block 933a
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 933a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 933a mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 933d lea rcx,[rip+0xf636c] # 0xff6b0
            rcx = m.env.data_va(81533);
            // 9344 lea rsi,[rcx+rax*4]
            rsi = rcx.wrapping_add(rax.wrapping_mul(4));
            // 9348 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 934c call 0xeb882
            { let r = { let t = m.env.stub_va(0x4f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9351 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // 9353 cmp QWORD PTR [rbp-0x38],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, 0x0u64).1;
            // 9358 je 0x9485
            if crate::cov::brk(0x9358, fl.e()) { pc = 0x9485; continue; }
            pc = 0x935e;
        }
        0x935e => {
            // block 935e
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 935e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 935e lea r13,[rbp-0x78]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 9362 lea r12,[rbp-0x60]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            pc = 0x9366;
        }
        0x9366 => {
            // block 9366
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9366 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9366 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 936a mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 936e mov rdx,rcx
            rdx = rcx;
            // 9371 shr rdx,0xa
            { let (r, f) = shr::<64>(rdx, 0xau64, fl); rdx = r; fl = f; }
            // 9375 mov rax,QWORD PTR [rax+rdx*8]
            rax = m.r64(rax.wrapping_add(rdx.wrapping_mul(8))) as u64;
            // 9379 and rcx,0x3ff
            { let r = rcx & 0x3ffu64; let f = logic::<64>(r);
              rcx = r;
              fl = f; }
            // 9380 mov r15d,DWORD PTR [rax+rcx*4]
            r15 = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 9384 mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 938b mov DWORD PTR [rbp-0x68],0x6c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0x6cu64) as u32);
            // 9392 mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // 939a mov QWORD PTR [r13+0x0],0x0
            m.w64(r13, (0x0u64) as u64);
            // 93a2 mov QWORD PTR [rbp-0x80],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (r13) as u64);
            // 93a6 mov rdi,r12
            rdi = r12;
            // 93a9 call 0xeb87c
            { let r = { let t = m.env.stub_va(0x49); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 93ae cmp r15d,0xffffffff
            fl = sub::<32>((r15 as u32 as u64), 0xffffffffu64).1;
            // 93b2 je 0x946b
            if crate::cov::brk(0x93b2, fl.e()) { pc = 0x946b; continue; }
            pc = 0x93b8;
        }
        0x93b8 => {
            // block 93b8
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 93b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 93b8 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 93bb mov rax,QWORD PTR [rax+0x10c8]
            rax = m.r64(rax.wrapping_add(0x10c8)) as u64;
            // 93c2 mov rdi,r14
            rdi = r14;
            // 93c5 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 93c8 lea rdx,[rbp-0x64]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 93cc call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 93ce test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 93d0 je 0x946b
            if crate::cov::brk(0x93d0, fl.e()) { pc = 0x946b; continue; }
            pc = 0x93d6;
        }
        0x93d6 => {
            // block 93d6
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 93d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 93d6 mov edx,DWORD PTR [rbp-0x64]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // 93d9 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 93db je 0x946b
            if crate::cov::brk(0x93db, fl.e()) { pc = 0x946b; continue; }
            pc = 0x93e1;
        }
        0x93e1 => {
            // block 93e1
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 93e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 93e1 mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 93e8 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 93eb mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 93ef mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 93f2 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 93f4 mov edi,DWORD PTR [rbp-0x64]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // 93f7 lea rsi,[rbp-0x68]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 93fb call 0xbc0fc
            { let r = { let t = m.env.code_va(0xbc0fc); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9400 cmp DWORD PTR [rbp-0x68],0x6b
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, 0x6bu64).1;
            // 9404 ja 0x946b
            if crate::cov::brk(0x9404, fl.a()) { pc = 0x946b; continue; }
            pc = 0x9406;
        }
        0x9406 => {
            // block 9406
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9406 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9406 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 9409 mov rax,QWORD PTR [rax+0x198]
            rax = m.r64(rax.wrapping_add(0x198)) as u64;
            // 9410 mov rdi,r14
            rdi = r14;
            // 9413 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9415 mov esi,DWORD PTR [rbp-0x68]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // 9418 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 941a call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 941f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 9421 je 0x946b
            if crate::cov::brk(0x9421, fl.e()) { pc = 0x946b; continue; }
            pc = 0x9423;
        }
        0x9423 => {
            // block 9423
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9423 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9423 mov edi,DWORD PTR [rbp-0x68]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // 9426 lea rsi,[rbp-0x80]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 942a call 0xbc13c
            { let r = { let t = m.env.code_va(0xbc13c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 942f mov rbx,QWORD PTR [rbp-0x80]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 9433 jmp 0x9466
            pc = 0x9466; continue;
        }
        0x9435 => {
            // block 9435
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9435 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9435 lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 9439 mov rdi,r12
            rdi = r12;
            // 943c call 0xeb882
            { let r = { let t = m.env.stub_va(0x4f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9441 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 9445 test rax,rax
            fl = logic::<64>(rax & rax);
            // 9448 je 0x9457
            if crate::cov::brk(0x9448, fl.e()) { pc = 0x9457; continue; }
            pc = 0x944a;
        }
        0x944a => {
            // block 944a
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 944a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 944a mov rbx,rax
            rbx = rax;
            // 944d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 9450 test rax,rax
            fl = logic::<64>(rax & rax);
            // 9453 jne 0x944a
            if crate::cov::brk(0x9453, fl.ne()) { pc = 0x944a; continue; }
            pc = 0x9455;
        }
        0x9455 => {
            // block 9455
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9455 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9455 jmp 0x9466
            pc = 0x9466; continue;
        }
        0x9457 => {
            // block 9457
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9457 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9457 mov rax,rbx
            rax = rbx;
            pc = 0x945a;
        }
        0x945a => {
            // block 945a
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 945a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 945a mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 945e cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 9461 mov rax,rbx
            rax = rbx;
            // 9464 jne 0x945a
            if crate::cov::brk(0x9464, fl.ne()) { pc = 0x945a; continue; }
            pc = 0x9466;
        }
        0x9466 => {
            // block 9466
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9466 cmp rbx,r13
            fl = sub::<64>(rbx, r13).1;
            // 9469 jne 0x9435
            if crate::cov::brk(0x9469, fl.ne()) { pc = 0x9435; continue; }
            pc = 0x946b;
        }
        0x946b => {
            // block 946b
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 946b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 946b mov rsi,QWORD PTR [rbp-0x78]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 946f lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 9473 call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9478 cmp QWORD PTR [rbp-0x38],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, 0x0u64).1;
            // 947d jne 0x9366
            if crate::cov::brk(0x947d, fl.ne()) { pc = 0x9366; continue; }
            pc = 0x9483;
        }
        0x9483 => {
            // block 9483
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9483 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9483 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x9485;
        }
        0x9485 => {
            // block 9485
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9485 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9485 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 9489 call 0xeb83a
            { let r = { let t = m.env.stub_va(0x7); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 948e mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 9490 add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 9494 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 9495 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 9497 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 9499 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 949b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 949d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 949e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x949f => {
            // block 949f
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 949f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 949f jmp 0x94a1
            pc = 0x94a1; continue;
        }
        0x94a1 => {
            // block 94a1
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 94a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 94a1 mov rbx,rax
            rbx = rax;
            // 94a4 mov rsi,QWORD PTR [rbp-0x78]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 94a8 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 94ac call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x94b1;
        }
        0x94b1 => {
            // block 94b1
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 94b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 94b1 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 94b5 call 0xeb83a
            { let r = { let t = m.env.stub_va(0x7); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 94ba mov rdi,rbx
            rdi = rbx;
            // 94bd call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 94c2 mov rbx,rax
            rbx = rax;
            // 94c5 jmp 0x94b1
            pc = 0x94b1; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5a96d size=491
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005a96d<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5a96d;
    loop {
        match pc {
        0x5a96d => {
            // block 5a96d
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a96d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a96d push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5a96e mov rbp,rsp
            rbp = rsp;
            // 5a971 sub rsp,0x40
            { let (r, f) = sub::<64>(rsp, 0x40u64);
              rsp = r;
              fl = f; }
            // 5a975 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 5a977 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 5a979 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a97b test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // 5a97d je 0x5ab3f
            if crate::cov::brk(0x5a97d, fl.e()) { pc = 0x5ab3f; continue; }
            pc = 0x5a983;
        }
        0x5a983 => {
            // block 5a983
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a983 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a983 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 5a986 ja 0x5ab3f
            if crate::cov::brk(0x5a986, fl.a()) { pc = 0x5ab3f; continue; }
            pc = 0x5a98c;
        }
        0x5a98c => {
            // block 5a98c
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a98c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a98c lea r8d,[rdi-0x1]
            r8 = (rdi.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 5a990 cmp r8d,0x1e
            fl = sub::<32>((r8 as u32 as u64), 0x1eu64).1;
            // 5a994 jb 0x5a9c4
            if crate::cov::brk(0x5a994, fl.b()) { pc = 0x5a9c4; continue; }
            pc = 0x5a996;
        }
        0x5a996 => {
            // block 5a996
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a996 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a996 lea esi,[rdi-0x6e]
            rsi = (rdi.wrapping_add(0xffff_ffff_ffff_ff92)) as u32 as u64;
            // 5a999 cmp esi,0xc
            fl = sub::<32>((rsi as u32 as u64), 0xcu64).1;
            // 5a99c jb 0x5a9c4
            if crate::cov::brk(0x5a99c, fl.b()) { pc = 0x5a9c4; continue; }
            pc = 0x5a99e;
        }
        0x5a99e => {
            // block 5a99e
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a99e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a99e cmp edi,0x6d
            fl = sub::<32>((rdi as u32 as u64), 0x6du64).1;
            // 5a9a1 jg 0x5a9bf
            if crate::cov::brk(0x5a9a1, fl.g()) { pc = 0x5a9bf; continue; }
            pc = 0x5a9a3;
        }
        0x5a9a3 => {
            // block 5a9a3
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9a3 lea r9d,[rdi-0xd]
            r9 = (rdi.wrapping_add(0xffff_ffff_ffff_fff3)) as u32 as u64;
            // 5a9a7 cmp r9d,0x37
            fl = sub::<32>((r9 as u32 as u64), 0x37u64).1;
            // 5a9ab ja 0x5a9ce
            if crate::cov::brk(0x5a9ab, fl.a()) { pc = 0x5a9ce; continue; }
            pc = 0x5a9ad;
        }
        0x5a9ad => {
            // block 5a9ad
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9ad movabs rsi,0xf8000001f8600f
            rsi = 0xf8_0000_01f8_600fu64;
            // 5a9b7 bt rsi,r9
            fl.cf = bit::<64>(rsi, r9);
            // 5a9bb jb 0x5a9c4
            if crate::cov::brk(0x5a9bb, fl.b()) { pc = 0x5a9c4; continue; }
            pc = 0x5a9bd;
        }
        0x5a9bd => {
            // block 5a9bd
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9bd jmp 0x5a9ce
            pc = 0x5a9ce; continue;
        }
        0x5a9bf => {
            // block 5a9bf
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9bf cmp esi,0xc
            fl = sub::<32>((rsi as u32 as u64), 0xcu64).1;
            // 5a9c2 jae 0x5a9ce
            if crate::cov::brk(0x5a9c2, fl.ae()) { pc = 0x5a9ce; continue; }
            pc = 0x5a9c4;
        }
        0x5a9c4 => {
            // block 5a9c4
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9c4 cmp edx,0x1f
            fl = sub::<32>((rdx as u32 as u64), 0x1fu64).1;
            // 5a9c7 jbe 0x5a9e3
            if crate::cov::brk(0x5a9c7, fl.be()) { pc = 0x5a9e3; continue; }
            pc = 0x5a9c9;
        }
        0x5a9c9 => {
            // block 5a9c9
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9c9 jmp 0x5ab3f
            pc = 0x5ab3f; continue;
        }
        0x5a9ce => {
            // block 5a9ce
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9ce lea esi,[rdi-0x50]
            rsi = (rdi.wrapping_add(0xffff_ffff_ffff_ffb0)) as u32 as u64;
            // 5a9d1 cmp esi,0x1d
            fl = sub::<32>((rsi as u32 as u64), 0x1du64).1;
            // 5a9d4 ja 0x5ab3f
            if crate::cov::brk(0x5a9d4, fl.a()) { pc = 0x5ab3f; continue; }
            pc = 0x5a9da;
        }
        0x5a9da => {
            // block 5a9da
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9da cmp edx,0x20
            fl = sub::<32>((rdx as u32 as u64), 0x20u64).1;
            // 5a9dd jae 0x5ab3f
            if crate::cov::brk(0x5a9dd, fl.ae()) { pc = 0x5ab3f; continue; }
            pc = 0x5a9e3;
        }
        0x5a9e3 => {
            // block 5a9e3
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9e3 lea esi,[rdx-0x1c]
            rsi = (rdx.wrapping_add(0xffff_ffff_ffff_ffe4)) as u32 as u64;
            // 5a9e6 cmp esi,0x3
            fl = sub::<32>((rsi as u32 as u64), 0x3u64).1;
            // 5a9e9 jbe 0x5aa4e
            if crate::cov::brk(0x5a9e9, fl.be()) { pc = 0x5aa4e; continue; }
            pc = 0x5a9eb;
        }
        0x5a9eb => {
            // block 5a9eb
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9eb cmp r8d,0x78
            fl = sub::<32>((r8 as u32 as u64), 0x78u64).1;
            // 5a9ef ja 0x5ab3f
            if crate::cov::brk(0x5a9ef, fl.a()) { pc = 0x5ab3f; continue; }
            pc = 0x5a9f5;
        }
        0x5a9f5 => {
            // block 5a9f5
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9f5 movsxd rsi,r8d
            rsi = sx::<32>((r8 as u32 as u64)) as u64;
            // 5a9f8 lea rdi,[rip+0xa6991] # 0x101390
            rdi = m.env.data_va(88925);
            // 5a9ff mov esi,DWORD PTR [rdi+rsi*4]
            rsi = (m.r32(rdi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u32 as u64;
            // 5aa02 cmp esi,0xb
            fl = sub::<32>((rsi as u32 as u64), 0xbu64).1;
            // 5aa05 ja 0x5ab3f
            if crate::cov::brk(0x5aa05, fl.a()) { pc = 0x5ab3f; continue; }
            pc = 0x5aa0b;
        }
        0x5aa0b => {
            // block 5aa0b
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa0b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 5aa0f call 0x32788
            { let r = { let t = m.env.code_va(0x32788); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5aa14 mov ecx,DWORD PTR [rbp-0x40]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 5aa17 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5aa19 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 5aa1b je 0x5ab3f
            if crate::cov::brk(0x5aa1b, fl.e()) { pc = 0x5ab3f; continue; }
            pc = 0x5aa21;
        }
        0x5aa21 => {
            // block 5aa21
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa21 cmp DWORD PTR [rbp-0x3c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64, 0x0u64).1;
            // 5aa25 je 0x5ab3f
            if crate::cov::brk(0x5aa25, fl.e()) { pc = 0x5ab3f; continue; }
            pc = 0x5aa2b;
        }
        0x5aa2b => {
            // block 5aa2b
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa2b cmp WORD PTR [rbp-0xc],0x0
            fl = sub::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 5aa30 je 0x5ab3f
            if crate::cov::brk(0x5aa30, fl.e()) { pc = 0x5ab3f; continue; }
            pc = 0x5aa36;
        }
        0x5aa36 => {
            // block 5aa36
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa36 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // 5aa3a je 0x5ab3f
            if crate::cov::brk(0x5aa3a, fl.e()) { pc = 0x5ab3f; continue; }
            pc = 0x5aa40;
        }
        0x5aa40 => {
            // block 5aa40
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa40 imul ecx,DWORD PTR [rbp-0x38]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // 5aa44 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 5aa47 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 5aa49 jmp 0x5ab3f
            pc = 0x5ab3f; continue;
        }
        0x5aa4e => {
            // block 5aa4e
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa4e lea rax,[rip+0xf3] # 0x5ab48
            rax = m.env.code_va(0x5ab48);
            // 5aa55 movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // 5aa59 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 5aa5c jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x5aa5e => {
            // block 5aa5e
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa5e cmp r8d,0x78
            fl = sub::<32>((r8 as u32 as u64), 0x78u64).1;
            // 5aa62 ja 0x5aab2
            if crate::cov::brk(0x5aa62, fl.a()) { pc = 0x5aab2; continue; }
            pc = 0x5aa64;
        }
        0x5aa64 => {
            // block 5aa64
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa64 movsxd rax,r8d
            rax = sx::<32>((r8 as u32 as u64)) as u64;
            // 5aa67 lea rcx,[rip+0xa66c2] # 0x101130
            rcx = m.env.data_va(88317);
            // 5aa6e mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 5aa71 jmp 0x5aabe
            pc = 0x5aabe; continue;
        }
        0x5aa73 => {
            // block 5aa73
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa73 cmp r8d,0x78
            fl = sub::<32>((r8 as u32 as u64), 0x78u64).1;
            // 5aa77 ja 0x5aad7
            if crate::cov::brk(0x5aa77, fl.a()) { pc = 0x5aad7; continue; }
            pc = 0x5aa79;
        }
        0x5aa79 => {
            // block 5aa79
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa79 movsxd rax,r8d
            rax = sx::<32>((r8 as u32 as u64)) as u64;
            // 5aa7c lea rcx,[rip+0xa66ad] # 0x101130
            rcx = m.env.data_va(88317);
            // 5aa83 mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 5aa86 jmp 0x5aae3
            pc = 0x5aae3; continue;
        }
        0x5aa88 => {
            // block 5aa88
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa88 cmp r8d,0x78
            fl = sub::<32>((r8 as u32 as u64), 0x78u64).1;
            // 5aa8c ja 0x5aafe
            if crate::cov::brk(0x5aa8c, fl.a()) { pc = 0x5aafe; continue; }
            pc = 0x5aa8e;
        }
        0x5aa8e => {
            // block 5aa8e
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa8e movsxd rax,r8d
            rax = sx::<32>((r8 as u32 as u64)) as u64;
            // 5aa91 lea rcx,[rip+0xa6698] # 0x101130
            rcx = m.env.data_va(88317);
            // 5aa98 mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 5aa9b jmp 0x5ab0a
            pc = 0x5ab0a; continue;
        }
        0x5aa9d => {
            // block 5aa9d
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa9d cmp r8d,0x78
            fl = sub::<32>((r8 as u32 as u64), 0x78u64).1;
            // 5aaa1 ja 0x5ab21
            if crate::cov::brk(0x5aaa1, fl.a()) { pc = 0x5ab21; continue; }
            pc = 0x5aaa3;
        }
        0x5aaa3 => {
            // block 5aaa3
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aaa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aaa3 movsxd rax,r8d
            rax = sx::<32>((r8 as u32 as u64)) as u64;
            // 5aaa6 lea rcx,[rip+0xa6683] # 0x101130
            rcx = m.env.data_va(88317);
            // 5aaad mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 5aab0 jmp 0x5ab2d
            pc = 0x5ab2d; continue;
        }
        0x5aab2 => {
            // block 5aab2
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aab2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aab2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5aab4 cmp edi,0x7a
            fl = sub::<32>((rdi as u32 as u64), 0x7au64).1;
            // 5aab7 mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 5aabc ja 0x5aacb
            if crate::cov::brk(0x5aabc, fl.a()) { pc = 0x5aacb; continue; }
            pc = 0x5aabe;
        }
        0x5aabe => {
            // block 5aabe
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aabe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aabe movsxd rcx,edi
            rcx = sx::<32>((rdi as u32 as u64)) as u64;
            // 5aac1 lea rdx,[rip+0xa6478] # 0x100f40
            rdx = m.env.data_va(87821);
            // 5aac8 mov ecx,DWORD PTR [rdx+rcx*4]
            rcx = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5aacb;
        }
        0x5aacb => {
            // block 5aacb
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aacb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aacb imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 5aace lea eax,[rax+rax*2]
            rax = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // 5aad1 shr eax,1
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 5aad3 add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 5aad5 jmp 0x5aaf6
            pc = 0x5aaf6; continue;
        }
        0x5aad7 => {
            // block 5aad7
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aad7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aad7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5aad9 cmp edi,0x7a
            fl = sub::<32>((rdi as u32 as u64), 0x7au64).1;
            // 5aadc mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 5aae1 ja 0x5aaf0
            if crate::cov::brk(0x5aae1, fl.a()) { pc = 0x5aaf0; continue; }
            pc = 0x5aae3;
        }
        0x5aae3 => {
            // block 5aae3
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aae3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aae3 movsxd rcx,edi
            rcx = sx::<32>((rdi as u32 as u64)) as u64;
            // 5aae6 lea rdx,[rip+0xa6453] # 0x100f40
            rdx = m.env.data_va(87821);
            // 5aaed mov ecx,DWORD PTR [rdx+rcx*4]
            rcx = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5aaf0;
        }
        0x5aaf0 => {
            // block 5aaf0
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aaf0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aaf0 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 5aaf3 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0x5aaf6;
        }
        0x5aaf6 => {
            // block 5aaf6
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aaf6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aaf6 lea eax,[rax+rax*4]
            rax = (rax.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // 5aaf9 shr eax,0x3
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x3u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 5aafc jmp 0x5ab3f
            pc = 0x5ab3f; continue;
        }
        0x5aafe => {
            // block 5aafe
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aafe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aafe xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5ab00 cmp edi,0x7a
            fl = sub::<32>((rdi as u32 as u64), 0x7au64).1;
            // 5ab03 mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 5ab08 ja 0x5ab17
            if crate::cov::brk(0x5ab08, fl.a()) { pc = 0x5ab17; continue; }
            pc = 0x5ab0a;
        }
        0x5ab0a => {
            // block 5ab0a
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5ab0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab0a movsxd rcx,edi
            rcx = sx::<32>((rdi as u32 as u64)) as u64;
            // 5ab0d lea rdx,[rip+0xa642c] # 0x100f40
            rdx = m.env.data_va(87821);
            // 5ab14 mov ecx,DWORD PTR [rdx+rcx*4]
            rcx = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5ab17;
        }
        0x5ab17 => {
            // block 5ab17
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5ab17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab17 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 5ab1a lea eax,[rax+rax*2]
            rax = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // 5ab1d shr eax,1
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 5ab1f jmp 0x5ab3f
            pc = 0x5ab3f; continue;
        }
        0x5ab21 => {
            // block 5ab21
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5ab21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab21 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5ab23 cmp edi,0x7a
            fl = sub::<32>((rdi as u32 as u64), 0x7au64).1;
            // 5ab26 mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 5ab2b ja 0x5ab3a
            if crate::cov::brk(0x5ab2b, fl.a()) { pc = 0x5ab3a; continue; }
            pc = 0x5ab2d;
        }
        0x5ab2d => {
            // block 5ab2d
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5ab2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab2d movsxd rcx,edi
            rcx = sx::<32>((rdi as u32 as u64)) as u64;
            // 5ab30 lea rdx,[rip+0xa6409] # 0x100f40
            rdx = m.env.data_va(87821);
            // 5ab37 mov ecx,DWORD PTR [rdx+rcx*4]
            rcx = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5ab3a;
        }
        0x5ab3a => {
            // block 5ab3a
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5ab3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab3a imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 5ab3d add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5ab3f;
        }
        0x5ab3f => {
            // block 5ab3f
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5ab3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab3f add rsp,0x40
            { let (r, f) = add::<64>(rsp, 0x40u64);
              rsp = r;
              fl = f; }
            // 5ab43 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5ab44 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7fc0 size=496
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007fc0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x7fc0;
    loop {
        match pc {
        0x7fc0 => {
            // block 7fc0
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 7fc0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fc0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7fc1 mov rbp,rsp
            rbp = rsp;
            // 7fc4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7fc6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7fc8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 7fca push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 7fcc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7fcd sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 7fd1 mov r14,rcx
            r14 = rcx;
            // 7fd4 mov r15,rdx
            r15 = rdx;
            // 7fd7 mov rbx,rsi
            rbx = rsi;
            // 7fda mov r13,rdi
            r13 = rdi;
            // 7fdd mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 7fe4 mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 7fe7 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 7feb cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 7fee je 0x8005
            if crate::cov::brk(0x7fee, fl.e()) { pc = 0x8005; continue; }
            pc = 0x7ff0;
        }
        0x7ff0 => {
            // block 7ff0
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 7ff0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ff0 lea rdx,[rax-0x8]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff8);
            // 7ff4 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 7ff7 not rdx
            { let v = !rdx; rdx = v; }
            // 7ffa and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 7ffe add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 8001 mov QWORD PTR [rbx+0x8],rdx
            m.w64(rbx.wrapping_add(0x8), (rdx) as u64);
            pc = 0x8005;
        }
        0x8005 => {
            // block 8005
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 8005 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8005 mov esi,0x400
            rsi = (0x400u64) as u32 as u64;
            // 800a mov rdi,rbx
            rdi = rbx;
            // 800d mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 8011 call 0xeb89a
            { let r = { let t = m.env.stub_va(0x67); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 8016 mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 8019 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 801d cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 8020 je 0x8037
            if crate::cov::brk(0x8020, fl.e()) { pc = 0x8037; continue; }
            pc = 0x8022;
        }
        0x8022 => {
            // block 8022
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 8022 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8022 lea rdx,[rax-0x8]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff8);
            // 8026 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 8029 not rdx
            { let v = !rdx; rdx = v; }
            // 802c and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 8030 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 8033 mov QWORD PTR [r15+0x8],rdx
            m.w64(r15.wrapping_add(0x8), (rdx) as u64);
            pc = 0x8037;
        }
        0x8037 => {
            // block 8037
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 8037 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8037 mov esi,0x400
            rsi = (0x400u64) as u32 as u64;
            // 803c mov rdi,r15
            rdi = r15;
            // 803f mov QWORD PTR [rbp-0x40],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r15) as u64);
            // 8043 call 0xeb89a
            { let r = { let t = m.env.stub_va(0x67); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 8048 mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // 804b mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 804f cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 8052 je 0x8069
            if crate::cov::brk(0x8052, fl.e()) { pc = 0x8069; continue; }
            pc = 0x8054;
        }
        0x8054 => {
            // block 8054
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 8054 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8054 lea rdx,[rax-0x8]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff8);
            // 8058 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 805b not rdx
            { let v = !rdx; rdx = v; }
            // 805e and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 8062 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 8065 mov QWORD PTR [r14+0x8],rdx
            m.w64(r14.wrapping_add(0x8), (rdx) as u64);
            pc = 0x8069;
        }
        0x8069 => {
            // block 8069
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 8069 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8069 mov esi,0x400
            rsi = (0x400u64) as u32 as u64;
            // 806e mov rdi,r14
            rdi = r14;
            // 8071 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            // 8075 call 0xeb89a
            { let r = { let t = m.env.stub_va(0x67); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 807a mov ebx,0x200
            rbx = (0x200u64) as u32 as u64;
            // 807f mov r15d,0x8
            r15 = (0x8u64) as u32 as u64;
            // 8085 mov r12d,0x8
            r12 = (0x8u64) as u32 as u64;
            // 808b mov r14,QWORD PTR [rbp-0x48]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0x808f;
        }
        0x808f => {
            // block 808f
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 808f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 808f mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 8093 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 8098 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 809b mov rdi,r13
            rdi = r13;
            // 809e mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 80a0 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 80a4 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 80a7 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 80aa mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 80ac shr ecx,0x6
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x6u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 80af and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 80b5 cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // 80b9 mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // 80bc movsd QWORD PTR [rcx+r12*1-0x8],xmm0
            m.w64(rcx.wrapping_add(r12).wrapping_add(0xffff_ffff_ffff_fff8), x0 as u64);
            // 80c3 shr eax,0x16
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 80c6 xorps xmm0,xmm0
            x0 = 0;
            // 80c9 cvtsi2sd xmm0,eax
            x0 = set_lo64(x0, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 80cd movsd QWORD PTR [rcx+r12*1],xmm0
            m.w64(rcx.wrapping_add(r12), x0 as u64);
            // 80d3 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 80d5 add r12,0x10
            { let (r, f) = add::<64>(r12, 0x10u64);
              r12 = r;
              fl = f; }
            // 80d9 cmp ebx,0x400
            fl = sub::<32>((rbx as u32 as u64), 0x400u64).1;
            // 80df jne 0x808f
            if crate::cov::brk(0x80df, fl.ne()) { pc = 0x808f; continue; }
            pc = 0x80e1;
        }
        0x80e1 => {
            // block 80e1
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 80e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80e1 mov ebx,0x400
            rbx = (0x400u64) as u32 as u64;
            // 80e6 mov r12d,0x8
            r12 = (0x8u64) as u32 as u64;
            // 80ec mov r14,QWORD PTR [rbp-0x40]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            pc = 0x80f0;
        }
        0x80f0 => {
            // block 80f0
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 80f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80f0 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 80f4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 80f9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 80fc mov rdi,r13
            rdi = r13;
            // 80ff mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 8101 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 8105 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 8108 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 810b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 810d shr ecx,0x6
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x6u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 8110 and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 8116 cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // 811a mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // 811d movsd QWORD PTR [rcx+r15*1-0x8],xmm0
            m.w64(rcx.wrapping_add(r15).wrapping_add(0xffff_ffff_ffff_fff8), x0 as u64);
            // 8124 shr eax,0x16
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8127 xorps xmm0,xmm0
            x0 = 0;
            // 812a cvtsi2sd xmm0,eax
            x0 = set_lo64(x0, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 812e movsd QWORD PTR [rcx+r15*1],xmm0
            m.w64(rcx.wrapping_add(r15), x0 as u64);
            // 8134 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 8136 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // 813a cmp ebx,0x600
            fl = sub::<32>((rbx as u32 as u64), 0x600u64).1;
            // 8140 jne 0x80f0
            if crate::cov::brk(0x8140, fl.ne()) { pc = 0x80f0; continue; }
            pc = 0x8142;
        }
        0x8142 => {
            // block 8142
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 8142 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8142 mov ebx,0x600
            rbx = (0x600u64) as u32 as u64;
            // 8147 lea r14,[rbp-0x2c]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 814b mov r15,QWORD PTR [rbp-0x38]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x814f;
        }
        0x814f => {
            // block 814f
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 814f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 814f mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 8153 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 8158 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 815b mov rdi,r13
            rdi = r13;
            // 815e mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 8160 mov rdx,r14
            rdx = r14;
            // 8163 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 8166 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 8169 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 816b shr ecx,0x6
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x6u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 816e and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 8174 cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // 8178 mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 817b movsd QWORD PTR [rcx+r12*1-0x8],xmm0
            m.w64(rcx.wrapping_add(r12).wrapping_add(0xffff_ffff_ffff_fff8), x0 as u64);
            // 8182 shr eax,0x16
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8185 xorps xmm0,xmm0
            x0 = 0;
            // 8188 cvtsi2sd xmm0,eax
            x0 = set_lo64(x0, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 818c movsd QWORD PTR [rcx+r12*1],xmm0
            m.w64(rcx.wrapping_add(r12), x0 as u64);
            // 8192 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 8194 add r12,0x10
            { let (r, f) = add::<64>(r12, 0x10u64);
              r12 = r;
              fl = f; }
            // 8198 cmp ebx,0x800
            fl = sub::<32>((rbx as u32 as u64), 0x800u64).1;
            // 819e jne 0x814f
            if crate::cov::brk(0x819e, fl.ne()) { pc = 0x814f; continue; }
            pc = 0x81a0;
        }
        0x81a0 => {
            // block 81a0
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 81a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 81a0 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 81a4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 81a5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 81a7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 81a9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 81ab pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 81ad pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 81ae ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdb4bc size=506
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000db4bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdb4bc;
    loop {
        match pc {
        0xdb4bc => {
            // block db4bc
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db4bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db4bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // db4bd mov rbp,rsp
            rbp = rsp;
            // db4c0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // db4c2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // db4c4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // db4c6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // db4c8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // db4c9 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // db4cd mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // db4d0 mov rbx,rdi
            rbx = rdi;
            // db4d3 call 0xd74b0
            { let r = { let t = m.env.code_va(0xd74b0); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // db4d8 mov r15d,DWORD PTR [rbx+0x338]
            r15 = (m.r32(rbx.wrapping_add(0x338)) as u64) as u32 as u64;
            // db4df shr r15d,0x2
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x2u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // db4e3 mov DWORD PTR [rbp-0x3c],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((r15 as u32 as u64)) as u32);
            // db4e7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db4ea mov esi,0x2880
            rsi = (0x2880u64) as u32 as u64;
            // db4ef mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // db4f4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // db4f7 mov rdi,rbx
            rdi = rbx;
            // db4fa mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // db4fd call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // db500 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // db503 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // db505 je 0xdb659
            if crate::cov::brk(0xdb505, fl.e()) { pc = 0xdb659; continue; }
            pc = 0xdb50b;
        }
        0xdb50b => {
            // block db50b
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db50b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db50b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // db50e xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // db511 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // db514 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db516 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            pc = 0xdb51a;
        }
        0xdb51a => {
            // block db51a
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db51a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db51a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db51d mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // db522 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // db527 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // db52a mov rdi,rbx
            rdi = rbx;
            // db52d mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // db530 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // db533 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db536 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // db53b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // db540 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // db543 mov rdi,rbx
            rdi = rbx;
            // db546 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // db549 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // db54c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db54f mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // db554 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // db559 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // db55e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // db561 mov rdi,rbx
            rdi = rbx;
            // db564 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // db567 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db56a mov rdi,rbx
            rdi = rbx;
            // db56d call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // db573 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db576 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // db57b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // db580 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // db583 mov rdi,rbx
            rdi = rbx;
            // db586 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // db58a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // db58d mov rax,QWORD PTR [rbx+0x6a0]
            rax = m.r64(rbx.wrapping_add(0x6a0)) as u64;
            // db594 movzx ecx,BYTE PTR [rax+r12*4]
            rcx = (m.r8(rax.wrapping_add(r12.wrapping_mul(4))) as u64) as u32 as u64;
            // db599 movzx edx,BYTE PTR [rax+r12*4+0x1]
            rdx = (m.r8(rax.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x1)) as u64) as u32 as u64;
            // db59f movzx esi,BYTE PTR [rax+r12*4+0x2]
            rsi = (m.r8(rax.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2)) as u64) as u32 as u64;
            // db5a5 movzx eax,BYTE PTR [rax+r12*4+0x3]
            rax = (m.r8(rax.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x3)) as u64) as u32 as u64;
            // db5ab shl eax,0x18
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x18u64, fl); rax = (r) as u32 as u64; fl = f; }
            // db5ae shl esi,0x10
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x10u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // db5b1 shl edx,0x8
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x8u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // db5b4 or edx,ecx
            { let r = (rdx as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db5b6 or edx,esi
            { let r = (rdx as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db5b8 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db5ba mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // db5bd cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // db5bf jne 0xdb5c6
            if crate::cov::brk(0xdb5bf, fl.ne()) { pc = 0xdb5c6; continue; }
            pc = 0xdb5c1;
        }
        0xdb5c1 => {
            // block db5c1
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db5c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db5c1 mov ecx,DWORD PTR [rbp-0x3c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64) as u32 as u64;
            // db5c4 jmp 0xdb5e9
            pc = 0xdb5e9; continue;
        }
        0xdb5c6 => {
            // block db5c6
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db5c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db5c6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db5c8 lea rdi,[rip+0x5d814] # 0x138de3
            rdi = m.env.data_va(316848);
            // db5cf mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // db5d2 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // db5d7 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // db5db inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // db5dd mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // db5e1 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // db5e4 mov ecx,DWORD PTR [rbp-0x3c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64) as u32 as u64;
            // db5e7 jae 0xdb667
            if crate::cov::brk(0xdb5e7, fl.ae()) { pc = 0xdb667; continue; }
            pc = 0xdb5e9;
        }
        0xdb5e9 => {
            // block db5e9
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db5e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db5e9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db5eb mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // db5ee div ecx
            { let (q, r) = div::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // db5f0 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // db5f3 cmp BYTE PTR [rbx+0x378],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x378)) as u64, 0x0u64).1;
            // db5fa jne 0xdb62a
            if crate::cov::brk(0xdb5fa, fl.ne()) { pc = 0xdb62a; continue; }
            pc = 0xdb5fc;
        }
        0xdb5fc => {
            // block db5fc
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db5fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db5fc xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db5fe lea rdi,[rip+0x5d7f8] # 0x138dfd
            rdi = m.env.data_va(316874);
            // db605 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // db608 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // db60c mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // db60f call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // db614 mov rax,QWORD PTR [rip+0x64dc5] # 0x1403e0
            rax = m.r64(m.env.data_va(347053)) as u64;
            // db61b mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // db61e call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // db623 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // db626 mov r15,QWORD PTR [rbp-0x48]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0xdb62a;
        }
        0xdb62a => {
            // block db62a
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db62a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db62a add r14d,0x190
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x190u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // db631 add r12,0x64
            { let (r, f) = add::<64>(r12, 0x64u64);
              r12 = r;
              fl = f; }
            // db635 add r13d,0x2710
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x2710u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // db63c cmp r12d,ecx
            fl = sub::<32>((r12 as u32 as u64), (rcx as u32 as u64)).1;
            // db63f jb 0xdb51a
            if crate::cov::brk(0xdb63f, fl.b()) { pc = 0xdb51a; continue; }
            pc = 0xdb645;
        }
        0xdb645 => {
            // block db645
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db645 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db645 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // db649 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // db64b mov al,BYTE PTR [rbx+0x378]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0x378)) as u64) as u8 as u64);
            // db651 je 0xdb68b
            if crate::cov::brk(0xdb651, fl.e()) { pc = 0xdb68b; continue; }
            pc = 0xdb653;
        }
        0xdb653 => {
            // block db653
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db653 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db653 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // db655 jne 0xdb670
            if crate::cov::brk(0xdb655, fl.ne()) { pc = 0xdb670; continue; }
            pc = 0xdb657;
        }
        0xdb657 => {
            // block db657
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db657 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db657 jmp 0xdb675
            pc = 0xdb675; continue;
        }
        0xdb659 => {
            // block db659
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db659 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db659 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // db65c cmp BYTE PTR [rbx+0x378],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x378)) as u64, 0x0u64).1;
            // db663 jne 0xdb6a3
            if crate::cov::brk(0xdb663, fl.ne()) { pc = 0xdb6a3; continue; }
            pc = 0xdb665;
        }
        0xdb665 => {
            // block db665
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db665 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db665 jmp 0xdb692
            pc = 0xdb692; continue;
        }
        0xdb667 => {
            // block db667
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db667 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db667 cmp BYTE PTR [rbx+0x378],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x378)) as u64, 0x0u64).1;
            // db66e je 0xdb675
            if crate::cov::brk(0xdb66e, fl.e()) { pc = 0xdb675; continue; }
            pc = 0xdb670;
        }
        0xdb670 => {
            // block db670
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db670 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db670 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // db673 jmp 0xdb6a3
            pc = 0xdb6a3; continue;
        }
        0xdb675 => {
            // block db675
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db675 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db675 lea rdi,[rip+0x5d797] # 0x138e13
            rdi = m.env.data_va(316896);
            // db67c xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // db67f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db681 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // db684 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // db689 jmp 0xdb6a3
            pc = 0xdb6a3; continue;
        }
        0xdb68b => {
            // block db68b
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db68b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db68b mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // db68e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // db690 jne 0xdb6a3
            if crate::cov::brk(0xdb690, fl.ne()) { pc = 0xdb6a3; continue; }
            pc = 0xdb692;
        }
        0xdb692 => {
            // block db692
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db692 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db692 lea rdi,[rip+0x5d797] # 0x138e30
            rdi = m.env.data_va(316925);
            // db699 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db69b call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // db6a0 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xdb6a3;
        }
        0xdb6a3 => {
            // block db6a3
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db6a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db6a3 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // db6a6 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // db6aa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // db6ab pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // db6ad pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // db6af pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // db6b1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // db6b3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // db6b4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd8382 size=510
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d8382<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd8382;
    loop {
        match pc {
        0xd8382 => {
            // block d8382
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d8382 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8382 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d8383 mov rbp,rsp
            rbp = rsp;
            // d8386 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d8388 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d838a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d838c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d838e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d838f sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d8393 mov r14,rdi
            r14 = rdi;
            // d8396 lea eax,[rsi-0x1]
            rax = (rsi.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // d8399 cmp eax,0x6
            fl = sub::<32>((rax as u32 as u64), 0x6u64).1;
            // d839c ja 0xd83be
            if crate::cov::brk(0xd839c, fl.a()) { pc = 0xd83be; continue; }
            pc = 0xd839e;
        }
        0xd839e => {
            // block d839e
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d839e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d839e lea rcx,[rip+0x1bf] # 0xd8564
            rcx = m.env.code_va(0xd8564);
            // d83a5 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // d83a9 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // d83ac jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd83ae => {
            // block d83ae
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d83ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d83ae lea rax,[r14+0x344]
            rax = r14.wrapping_add(0x344);
            // d83b5 mov ecx,DWORD PTR [r14+0x368]
            rcx = (m.r32(r14.wrapping_add(0x368)) as u64) as u32 as u64;
            // d83bc jmp 0xd83cc
            pc = 0xd83cc; continue;
        }
        0xd83be => {
            // block d83be
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d83be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d83be lea rax,[r14+0x340]
            rax = r14.wrapping_add(0x340);
            // d83c5 mov ecx,DWORD PTR [r14+0x35c]
            rcx = (m.r32(r14.wrapping_add(0x35c)) as u64) as u32 as u64;
            pc = 0xd83cc;
        }
        0xd83cc => {
            // block d83cc
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d83cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d83cc mov ebx,DWORD PTR [rax]
            rbx = (m.r32(rax) as u64) as u32 as u64;
            // d83ce imul ecx,DWORD PTR [r14+0x33c]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(r14.wrapping_add(0x33c)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // d83d6 shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // d83d9 mov rdi,r14
            rdi = r14;
            // d83dc mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // d83df call 0xd74b0
            { let r = { let t = m.env.code_va(0xd74b0); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d83e4 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // d83e7 lea rax,[r14+0x378]
            rax = r14.wrapping_add(0x378);
            // d83ee test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d83f0 je 0xd84c4
            if crate::cov::brk(0xd83f0, fl.e()) { pc = 0xd84c4; continue; }
            pc = 0xd83f6;
        }
        0xd83f6 => {
            // block d83f6
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d83f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d83f6 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d83fa mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // d83fd xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd8400;
        }
        0xd8400 => {
            // block d8400
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d8400 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8400 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // d8403 mov BYTE PTR [rbp-0x2d],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3), ((rax as u8 as u64)) as u8);
            // d8406 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d8409 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d840c mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d8411 mov r13d,ecx
            r13 = ((rcx as u32 as u64)) as u32 as u64;
            // d8414 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8419 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d841c mov rdi,r14
            rdi = r14;
            // d841f mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d8421 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d8424 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d8427 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d842c mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // d8431 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8436 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8439 mov rdi,r14
            rdi = r14;
            // d843c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d843f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d8442 mov rdi,r14
            rdi = r14;
            // d8445 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d844b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d844e mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d8453 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8458 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d845b mov rdi,r14
            rdi = r14;
            // d845e lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // d8462 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d8465 mov ecx,r13d
            rcx = ((r13 as u32 as u64)) as u32 as u64;
            // d8468 cmp DWORD PTR [rbp-0x2c],0xffffffff
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64, 0xffffffffu64).1;
            // d846c mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // d846e jne 0xd84b2
            if crate::cov::brk(0xd846e, fl.ne()) { pc = 0xd84b2; continue; }
            pc = 0xd8470;
        }
        0xd8470 => {
            // block d8470
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d8470 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8470 mov r13b,BYTE PTR [rbp-0x2d]
            r13 = (r13 & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3)) as u64) as u8 as u64);
            // d8474 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d8478 cmp BYTE PTR [rax],0x0
            fl = sub::<8>(m.r8(rax) as u64, 0x0u64).1;
            // d847b jne 0xd84ac
            if crate::cov::brk(0xd847b, fl.ne()) { pc = 0xd84ac; continue; }
            pc = 0xd847d;
        }
        0xd847d => {
            // block d847d
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d847d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d847d imul eax,r15d,0x64
            { let (r, f) = imul::<32>((r15 as u32 as u64), 0x64u64); rax = (r) as u32 as u64; fl = f; }
            // d8481 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d8483 div ecx
            { let (q, r) = div::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // d8485 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // d8488 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d848a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d848c lea rdi,[rip+0x609c8] # 0x138e5b
            rdi = m.env.data_va(316968);
            // d8493 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // d8495 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d849a mov rax,QWORD PTR [rip+0x67f3f] # 0x1403e0
            rax = m.r64(m.env.data_va(347053)) as u64;
            // d84a1 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // d84a4 call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d84a9 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            pc = 0xd84ac;
        }
        0xd84ac => {
            // block d84ac
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84ac mov r12b,r13b
            r12 = (r12 & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // d84af mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            pc = 0xd84b2;
        }
        0xd84b2 => {
            // block d84b2
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84b2 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d84b4 add ebx,0x4
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x4u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d84b7 cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // d84b9 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // d84bc jb 0xd8400
            if crate::cov::brk(0xd84bc, fl.b()) { pc = 0xd8400; continue; }
            pc = 0xd84c2;
        }
        0xd84c2 => {
            // block d84c2
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84c2 jmp 0xd84cb
            pc = 0xd84cb; continue;
        }
        0xd84c4 => {
            // block d84c4
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84c4 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d84c8 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xd84cb;
        }
        0xd84cb => {
            // block d84cb
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84cb and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d84cf je 0xd84e9
            if crate::cov::brk(0xd84cf, fl.e()) { pc = 0xd84e9; continue; }
            pc = 0xd84d1;
        }
        0xd84d1 => {
            // block d84d1
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84d1 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d84d5 mov al,BYTE PTR [rax]
            rax = (rax & !0xff) | ((m.r8(rax) as u64) as u8 as u64);
            // d84d7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d84d9 jne 0xd84e9
            if crate::cov::brk(0xd84d9, fl.ne()) { pc = 0xd84e9; continue; }
            pc = 0xd84db;
        }
        0xd84db => {
            // block d84db
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84db lea rdi,[rip+0x6098d] # 0x138e6f
            rdi = m.env.data_va(316988);
            // d84e2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d84e4 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xd84e9;
        }
        0xd84e9 => {
            // block d84e9
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84e9 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d84eb mov rdi,r14
            rdi = r14;
            // d84ee call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d84f3 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // d84f6 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // d84f9 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d84fd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d84fe pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d8500 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d8502 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d8504 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d8506 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d8507 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd8508 => {
            // block d8508
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d8508 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8508 lea rax,[r14+0x354]
            rax = r14.wrapping_add(0x354);
            // d850f mov ecx,DWORD PTR [r14+0x360]
            rcx = (m.r32(r14.wrapping_add(0x360)) as u64) as u32 as u64;
            // d8516 jmp 0xd83cc
            pc = 0xd83cc; continue;
        }
        0xd851b => {
            // block d851b
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d851b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d851b lea rax,[r14+0x358]
            rax = r14.wrapping_add(0x358);
            // d8522 mov ecx,DWORD PTR [r14+0x364]
            rcx = (m.r32(r14.wrapping_add(0x364)) as u64) as u32 as u64;
            // d8529 jmp 0xd83cc
            pc = 0xd83cc; continue;
        }
        0xd852e => {
            // block d852e
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d852e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d852e lea rax,[r14+0x348]
            rax = r14.wrapping_add(0x348);
            // d8535 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // d853a jmp 0xd83cc
            pc = 0xd83cc; continue;
        }
        0xd853f => {
            // block d853f
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d853f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d853f lea rax,[r14+0x34c]
            rax = r14.wrapping_add(0x34c);
            // d8546 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // d854b jmp 0xd83cc
            pc = 0xd83cc; continue;
        }
        0xd8550 => {
            // block d8550
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d8550 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8550 lea rax,[r14+0x350]
            rax = r14.wrapping_add(0x350);
            // d8557 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // d855c jmp 0xd83cc
            pc = 0xd83cc; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x636c size=530
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000636c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x636c;
    loop {
        match pc {
        0x636c => {
            // block 636c
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 636c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 636c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 636d mov rbp,rsp
            rbp = rsp;
            // 6370 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6372 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6374 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6375 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6376 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 6379 mov r15,rdi
            r15 = rdi;
            // 637c mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 6380 call 0xce76c
            { let r = { let t = m.env.code_va(0xce76c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 6385 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 6388 jne 0x63a0
            if crate::cov::brk(0x6388, fl.ne()) { pc = 0x63a0; continue; }
            pc = 0x638a;
        }
        0x638a => {
            // block 638a
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 638a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 638d mov rdi,r15
            rdi = r15;
            // 6390 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 6393 call QWORD PTR [rax+0xf28]
            { let r = { let t = m.r64(rax.wrapping_add(0xf28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x6399;
        }
        0x6399 => {
            // block 6399
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6399 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6399 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 639b jmp 0x6571
            pc = 0x6571; continue;
        }
        0x63a0 => {
            // block 63a0
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 63a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a0 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 63a2 cmp r14d,0x9
            fl = sub::<32>((r14 as u32 as u64), 0x9u64).1;
            // 63a6 ja 0x6571
            if crate::cov::brk(0x63a6, fl.a()) { pc = 0x6571; continue; }
            pc = 0x63ac;
        }
        0x63ac => {
            // block 63ac
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 63ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ac mov eax,0xf
            rax = (0xfu64) as u32 as u64;
            // 63b1 bt eax,r14d
            fl.cf = bit::<32>((rax as u32 as u64), (r14 as u32 as u64));
            // 63b5 jb 0x6451
            if crate::cov::brk(0x63b5, fl.b()) { pc = 0x6451; continue; }
            pc = 0x63bb;
        }
        0x63bb => {
            // block 63bb
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 63bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bb mov eax,0xf0
            rax = (0xf0u64) as u32 as u64;
            // 63c0 bt eax,r14d
            fl.cf = bit::<32>((rax as u32 as u64), (r14 as u32 as u64));
            // 63c4 jae 0x64e7
            if crate::cov::brk(0x63c4, fl.ae()) { pc = 0x64e7; continue; }
            pc = 0x63ca;
        }
        0x63ca => {
            // block 63ca
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 63ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ca mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 63cd mov rdi,r15
            rdi = r15;
            // 63d0 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 63d6 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 63d8 call 0xd0489
            { let r = { let t = m.env.code_va(0xd0489); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 63dd movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 63e0 cmp eax,0x5
            fl = sub::<32>((rax as u32 as u64), 0x5u64).1;
            // 63e3 je 0x63f8
            if crate::cov::brk(0x63e3, fl.e()) { pc = 0x63f8; continue; }
            pc = 0x63e5;
        }
        0x63e5 => {
            // block 63e5
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 63e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e5 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 63e8 mov rdi,r15
            rdi = r15;
            // 63eb call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 63f1 cmp eax,0x10478350
            fl = sub::<32>((rax as u32 as u64), 0x10478350u64).1;
            // 63f6 jne 0x641d
            if crate::cov::brk(0x63f6, fl.ne()) { pc = 0x641d; continue; }
            pc = 0x63f8;
        }
        0x63f8 => {
            // block 63f8
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 63f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63f8 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 63fb xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 63fd mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 6402 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6404 mov ecx,0x10000000
            rcx = (0x10000000u64) as u32 as u64;
            // 6409 mov r8d,0x1c
            r8 = (0x1cu64) as u32 as u64;
            // 640f mov rdi,r15
            rdi = r15;
            // 6412 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 6415 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6417 je 0x6571
            if crate::cov::brk(0x6417, fl.e()) { pc = 0x6571; continue; }
            pc = 0x641d;
        }
        0x641d => {
            // block 641d
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 641d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 641d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6420 mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 6425 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 642a mov ecx,0x20000000
            rcx = (0x20000000u64) as u32 as u64;
            // 642f mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 6435 mov rdi,r15
            rdi = r15;
            // 6438 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 643b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 643d je 0x656f
            if crate::cov::brk(0x643d, fl.e()) { pc = 0x656f; continue; }
            pc = 0x6443;
        }
        0x6443 => {
            // block 6443
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6443 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6443 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6446 add r14d,0xfffffffc
            { let (r, f) = add::<32>((r14 as u32 as u64), 0xfffffffcu64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 644a mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 644f jmp 0x64ce
            pc = 0x64ce; continue;
        }
        0x6451 => {
            // block 6451
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6451 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6451 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6454 mov rdi,r15
            rdi = r15;
            // 6457 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 645d mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 645f call 0xd0489
            { let r = { let t = m.env.code_va(0xd0489); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 6464 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 6467 cmp eax,0x5
            fl = sub::<32>((rax as u32 as u64), 0x5u64).1;
            // 646a je 0x647f
            if crate::cov::brk(0x646a, fl.e()) { pc = 0x647f; continue; }
            pc = 0x646c;
        }
        0x646c => {
            // block 646c
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 646c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 646c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 646f mov rdi,r15
            rdi = r15;
            // 6472 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 6478 cmp eax,0x10478350
            fl = sub::<32>((rax as u32 as u64), 0x10478350u64).1;
            // 647d jne 0x64a4
            if crate::cov::brk(0x647d, fl.ne()) { pc = 0x64a4; continue; }
            pc = 0x647f;
        }
        0x647f => {
            // block 647f
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 647f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 647f mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6482 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 6484 mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 6489 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 648b mov ecx,0x10000000
            rcx = (0x10000000u64) as u32 as u64;
            // 6490 mov r8d,0x1c
            r8 = (0x1cu64) as u32 as u64;
            // 6496 mov rdi,r15
            rdi = r15;
            // 6499 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 649c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 649e je 0x6571
            if crate::cov::brk(0x649e, fl.e()) { pc = 0x6571; continue; }
            pc = 0x64a4;
        }
        0x64a4 => {
            // block 64a4
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 64a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 64a7 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 64a9 mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 64ae xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 64b0 mov ecx,0x20000000
            rcx = (0x20000000u64) as u32 as u64;
            // 64b5 mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 64bb mov rdi,r15
            rdi = r15;
            // 64be call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 64c1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 64c3 je 0x6571
            if crate::cov::brk(0x64c3, fl.e()) { pc = 0x6571; continue; }
            pc = 0x64c9;
        }
        0x64c9 => {
            // block 64c9
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 64c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64c9 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 64cc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x64ce;
        }
        0x64ce => {
            // block 64ce
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 64ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ce mov ecx,0xc0000000
            rcx = (0xc0000000u64) as u32 as u64;
            // 64d3 mov r8d,0x1e
            r8 = (0x1eu64) as u32 as u64;
            pc = 0x64d9;
        }
        0x64d9 => {
            // block 64d9
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 64d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64d9 mov rdi,r15
            rdi = r15;
            // 64dc mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 64df call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 64e2 jmp 0x6399
            pc = 0x6399; continue;
        }
        0x64e7 => {
            // block 64e7
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 64e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64e7 mov eax,0x300
            rax = (0x300u64) as u32 as u64;
            // 64ec bt eax,r14d
            fl.cf = bit::<32>((rax as u32 as u64), (r14 as u32 as u64));
            // 64f0 jae 0x6571
            if crate::cov::brk(0x64f0, fl.ae()) { pc = 0x6571; continue; }
            pc = 0x64f2;
        }
        0x64f2 => {
            // block 64f2
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 64f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64f2 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 64f5 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 64f7 mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 64fc xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 64fe mov ecx,0x20000000
            rcx = (0x20000000u64) as u32 as u64;
            // 6503 mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 6509 mov rdi,r15
            rdi = r15;
            // 650c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 650f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6511 je 0x6571
            if crate::cov::brk(0x6511, fl.e()) { pc = 0x6571; continue; }
            pc = 0x6513;
        }
        0x6513 => {
            // block 6513
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6513 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6513 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6516 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 6518 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 651a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 651c mov ecx,0xc0000000
            rcx = (0xc0000000u64) as u32 as u64;
            // 6521 mov r8d,0x1e
            r8 = (0x1eu64) as u32 as u64;
            // 6527 mov rdi,r15
            rdi = r15;
            // 652a call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 652d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 652f je 0x6571
            if crate::cov::brk(0x652f, fl.e()) { pc = 0x6571; continue; }
            pc = 0x6531;
        }
        0x6531 => {
            // block 6531
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6531 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6531 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6534 mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 6539 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 653e mov ecx,0x10000000
            rcx = (0x10000000u64) as u32 as u64;
            // 6543 mov r8d,0x1c
            r8 = (0x1cu64) as u32 as u64;
            // 6549 mov rdi,r15
            rdi = r15;
            // 654c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 654f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6551 je 0x656f
            if crate::cov::brk(0x6551, fl.e()) { pc = 0x656f; continue; }
            pc = 0x6553;
        }
        0x6553 => {
            // block 6553
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6553 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6553 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6556 add r14d,0xfffffff8
            { let (r, f) = add::<32>((r14 as u32 as u64), 0xfffffff8u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 655a mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 655f mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 6564 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 656a jmp 0x64d9
            pc = 0x64d9; continue;
        }
        0x656f => {
            // block 656f
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 656f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 656f xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x6571;
        }
        0x6571 => {
            // block 6571
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6571 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6571 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 6573 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6577 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6578 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 657a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 657c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 657d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x697d6 size=558
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000697d6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x697d6;
    loop {
        match pc {
        0x697d6 => {
            // block 697d6
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 697d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 697d6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 697d7 mov rbp,rsp
            rbp = rsp;
            // 697da push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 697dc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 697de push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 697e0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 697e2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 697e3 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 697e7 mov ebx,ecx
            rbx = ((rcx as u32 as u64)) as u32 as u64;
            // 697e9 mov r13,rsi
            r13 = rsi;
            // 697ec mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // 697f0 lea rax,[rdi+0x8]
            rax = rdi.wrapping_add(0x8);
            // 697f4 mov QWORD PTR [rdi+0x10],0x0
            m.w64(rdi.wrapping_add(0x10), (0x0u64) as u64);
            // 697fc mov QWORD PTR [rdi+0x8],0x0
            m.w64(rdi.wrapping_add(0x8), (0x0u64) as u64);
            // 69804 mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 69807 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 6980b mov rsi,rdx
            rsi = rdx;
            // 6980e call 0xb5094
            { let r = { let t = m.env.code_va(0xb5094); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69813 movzx eax,BYTE PTR [rbp-0x40]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 69817 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 69819 jne 0x69820
            if crate::cov::brk(0x69819, fl.ne()) { pc = 0x69820; continue; }
            pc = 0x6981b;
        }
        0x6981b => {
            // block 6981b
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6981b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6981b shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 6981e jmp 0x69824
            pc = 0x69824; continue;
        }
        0x69820 => {
            // block 69820
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69820 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69820 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x69824;
        }
        0x69824 => {
            // block 69824
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69824 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69824 cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 69827 jne 0x69863
            if crate::cov::brk(0x69827, fl.ne()) { pc = 0x69863; continue; }
            pc = 0x69829;
        }
        0x69829 => {
            // block 69829
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69829 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69829 lea rdi,[r13+0x1b0]
            rdi = r13.wrapping_add(0x1b0);
            // 69830 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 69834 call 0xebbd6
            { let r = { let t = m.env.stub_va(0x3a3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69839 add r13,0x1b8
            { let (r, f) = add::<64>(r13, 0x1b8u64);
              r13 = r;
              fl = f; }
            // 69840 cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // 69843 je 0x699c1
            if crate::cov::brk(0x69843, fl.e()) { pc = 0x699c1; continue; }
            pc = 0x69849;
        }
        0x69849 => {
            // block 69849
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69849 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69849 add rax,0x38
            { let (r, f) = add::<64>(rax, 0x38u64);
              rax = r;
              fl = f; }
            // 6984d mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 69851 mov rsi,rax
            rsi = rax;
            // 69854 call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69859 jmp 0x699c1
            pc = 0x699c1; continue;
        }
        0x6985e => {
            // block 6985e
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6985e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6985e jmp 0x699dd
            pc = 0x699dd; continue;
        }
        0x69863 => {
            // block 69863
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69863 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69863 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 69867 mov DWORD PTR [rbp-0x4c],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), ((rbx as u32 as u64)) as u32);
            // 6986a mov rbx,QWORD PTR [r13+0x1b0]
            rbx = m.r64(r13.wrapping_add(0x1b0)) as u64;
            // 69871 add r13,0x1b8
            { let (r, f) = add::<64>(r13, 0x1b8u64);
              r13 = r;
              fl = f; }
            // 69878 jmp 0x699b8
            pc = 0x699b8; continue;
        }
        0x6987d => {
            // block 6987d
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6987d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6987d movzx r8d,BYTE PTR [rbx+0x20]
            r8 = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 69882 mov r14b,r8b
            r14 = (r14 & !0xff) | (((r8 as u8 as u64)) as u8 as u64);
            // 69885 and r14b,0x1
            { let r = (r14 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r14 = (r14 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 69889 jne 0x6989a
            if crate::cov::brk(0x69889, fl.ne()) { pc = 0x6989a; continue; }
            pc = 0x6988b;
        }
        0x6988b => {
            // block 6988b
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6988b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6988b lea r9,[rbx+0x20]
            r9 = rbx.wrapping_add(0x20);
            // 6988f inc r9
            { let (r, f) = inc::<64>(r9, fl); r9 = r; fl = f; }
            // 69892 mov rsi,r8
            rsi = r8;
            // 69895 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 69898 jmp 0x698a2
            pc = 0x698a2; continue;
        }
        0x6989a => {
            // block 6989a
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6989a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6989a mov rsi,QWORD PTR [rbx+0x28]
            rsi = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 6989e mov r9,QWORD PTR [rbx+0x30]
            r9 = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x698a2;
        }
        0x698a2 => {
            // block 698a2
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698a2 movzx edi,BYTE PTR [rbp-0x40]
            rdi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 698a6 test dil,0x1
            fl = logic::<8>((rdi as u8 as u64) & 0x1u64);
            // 698aa jne 0x698b5
            if crate::cov::brk(0x698aa, fl.ne()) { pc = 0x698b5; continue; }
            pc = 0x698ac;
        }
        0x698ac => {
            // block 698ac
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698ac shr rdi,1
            { let (r, f) = shr::<64>(rdi, 0x1u64, fl); rdi = r; fl = f; }
            // 698af lea rax,[rbp-0x3f]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 698b3 jmp 0x698bd
            pc = 0x698bd; continue;
        }
        0x698b5 => {
            // block 698b5
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698b5 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 698b9 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x698bd;
        }
        0x698bd => {
            // block 698bd
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698bd cmp rsi,rdi
            fl = sub::<64>(rsi, rdi).1;
            // 698c0 jb 0x69993
            if crate::cov::brk(0x698c0, fl.b()) { pc = 0x69993; continue; }
            pc = 0x698c6;
        }
        0x698c6 => {
            // block 698c6
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698c6 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 698c9 mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 698ce je 0x69938
            if crate::cov::brk(0x698ce, fl.e()) { pc = 0x69938; continue; }
            pc = 0x698d0;
        }
        0x698d0 => {
            // block 698d0
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698d0 cmp rsi,rdi
            fl = sub::<64>(rsi, rdi).1;
            // 698d3 jl 0x69993
            if crate::cov::brk(0x698d3, fl.l()) { pc = 0x69993; continue; }
            pc = 0x698d9;
        }
        0x698d9 => {
            // block 698d9
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698d9 mov r10d,0x1
            r10 = (0x1u64) as u32 as u64;
            // 698df sub r10,rdi
            { let (r, f) = sub::<64>(r10, rdi);
              r10 = r;
              fl = f; }
            // 698e2 add r10,rsi
            { let (r, f) = add::<64>(r10, rsi);
              r10 = r;
              fl = f; }
            // 698e5 je 0x69993
            if crate::cov::brk(0x698e5, fl.e()) { pc = 0x69993; continue; }
            pc = 0x698eb;
        }
        0x698eb => {
            // block 698eb
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698eb add rsi,r9
            { let (r, f) = add::<64>(rsi, r9);
              rsi = r;
              fl = f; }
            // 698ee add r10,r9
            { let (r, f) = add::<64>(r10, r9);
              r10 = r;
              fl = f; }
            // 698f1 mov r11b,BYTE PTR [rax]
            r11 = (r11 & !0xff) | ((m.r8(rax) as u64) as u8 as u64);
            // 698f4 mov rdx,r9
            rdx = r9;
            pc = 0x698f7;
        }
        0x698f7 => {
            // block 698f7
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698f7 movzx r12d,r11b
            r12 = ((r11 as u8 as u64)) as u32 as u64;
            // 698fb movzx ecx,BYTE PTR [rdx]
            rcx = (m.r8(rdx) as u64) as u32 as u64;
            // 698fe cmp ecx,r12d
            fl = sub::<32>((rcx as u32 as u64), (r12 as u32 as u64)).1;
            // 69901 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // 69907 jne 0x69920
            if crate::cov::brk(0x69907, fl.ne()) { pc = 0x69920; continue; }
            pc = 0x69909;
        }
        0x69909 => {
            // block 69909
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69909 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69909 cmp rdi,r12
            fl = sub::<64>(rdi, r12).1;
            // 6990c je 0x6992a
            if crate::cov::brk(0x6990c, fl.e()) { pc = 0x6992a; continue; }
            pc = 0x6990e;
        }
        0x6990e => {
            // block 6990e
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6990e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6990e movzx ecx,BYTE PTR [rax+r12*1]
            rcx = (m.r8(rax.wrapping_add(r12)) as u64) as u32 as u64;
            // 69913 movzx r15d,BYTE PTR [rdx+r12*1]
            r15 = (m.r8(rdx.wrapping_add(r12)) as u64) as u32 as u64;
            // 69918 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // 6991b cmp r15d,ecx
            fl = sub::<32>((r15 as u32 as u64), (rcx as u32 as u64)).1;
            // 6991e je 0x69909
            if crate::cov::brk(0x6991e, fl.e()) { pc = 0x69909; continue; }
            pc = 0x69920;
        }
        0x69920 => {
            // block 69920
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69920 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69920 inc rdx
            { let (r, f) = inc::<64>(rdx, fl); rdx = r; fl = f; }
            // 69923 cmp rdx,r10
            fl = sub::<64>(rdx, r10).1;
            // 69926 jne 0x698f7
            if crate::cov::brk(0x69926, fl.ne()) { pc = 0x698f7; continue; }
            pc = 0x69928;
        }
        0x69928 => {
            // block 69928
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69928 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69928 jmp 0x69993
            pc = 0x69993; continue;
        }
        0x6992a => {
            // block 6992a
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6992a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6992a cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 6992d je 0x69993
            if crate::cov::brk(0x6992d, fl.e()) { pc = 0x69993; continue; }
            pc = 0x6992f;
        }
        0x6992f => {
            // block 6992f
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6992f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6992f sub rdx,r9
            { let (r, f) = sub::<64>(rdx, r9);
              rdx = r;
              fl = f; }
            // 69932 cmp rdx,0xffffffffffffffff
            fl = sub::<64>(rdx, 0xffff_ffff_ffff_ffffu64).1;
            // 69936 je 0x69993
            if crate::cov::brk(0x69936, fl.e()) { pc = 0x69993; continue; }
            pc = 0x69938;
        }
        0x69938 => {
            // block 69938
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69938 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69938 mov eax,DWORD PTR [rbp-0x4c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64) as u32 as u64;
            // 6993b cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 6993e je 0x69959
            if crate::cov::brk(0x6993e, fl.e()) { pc = 0x69959; continue; }
            pc = 0x69940;
        }
        0x69940 => {
            // block 69940
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69940 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69940 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 69943 jne 0x6996a
            if crate::cov::brk(0x69943, fl.ne()) { pc = 0x6996a; continue; }
            pc = 0x69945;
        }
        0x69945 => {
            // block 69945
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69945 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69945 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 69948 jne 0x69993
            if crate::cov::brk(0x69948, fl.ne()) { pc = 0x69993; continue; }
            pc = 0x6994a;
        }
        0x6994a => {
            // block 6994a
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6994a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6994a lea rsi,[rbx+0x38]
            rsi = rbx.wrapping_add(0x38);
            // 6994e mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 69952 call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69957 jmp 0x69993
            pc = 0x69993; continue;
        }
        0x69959 => {
            // block 69959
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69959 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69959 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 6995d add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 69960 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 69963 jne 0x6997d
            if crate::cov::brk(0x69963, fl.ne()) { pc = 0x6997d; continue; }
            pc = 0x69965;
        }
        0x69965 => {
            // block 69965
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69965 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69965 shr r8,1
            { let (r, f) = shr::<64>(r8, 0x1u64, fl); r8 = r; fl = f; }
            // 69968 jmp 0x69981
            pc = 0x69981; continue;
        }
        0x6996a => {
            // block 6996a
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6996a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6996a test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 6996c jne 0x69993
            if crate::cov::brk(0x6996c, fl.ne()) { pc = 0x69993; continue; }
            pc = 0x6996e;
        }
        0x6996e => {
            // block 6996e
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6996e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6996e lea rsi,[rbx+0x38]
            rsi = rbx.wrapping_add(0x38);
            // 69972 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 69976 call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 6997b jmp 0x69993
            pc = 0x69993; continue;
        }
        0x6997d => {
            // block 6997d
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6997d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6997d mov r8,QWORD PTR [rbx+0x28]
            r8 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x69981;
        }
        0x69981 => {
            // block 69981
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69981 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69981 cmp rdx,r8
            fl = sub::<64>(rdx, r8).1;
            // 69984 jne 0x69993
            if crate::cov::brk(0x69984, fl.ne()) { pc = 0x69993; continue; }
            pc = 0x69986;
        }
        0x69986 => {
            // block 69986
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69986 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69986 lea rsi,[rbx+0x38]
            rsi = rbx.wrapping_add(0x38);
            // 6998a mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 6998e call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x69993;
        }
        0x69993 => {
            // block 69993
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69993 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69993 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 69997 test rax,rax
            fl = logic::<64>(rax & rax);
            // 6999a je 0x699a9
            if crate::cov::brk(0x6999a, fl.e()) { pc = 0x699a9; continue; }
            pc = 0x6999c;
        }
        0x6999c => {
            // block 6999c
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6999c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6999c mov rbx,rax
            rbx = rax;
            // 6999f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 699a2 test rax,rax
            fl = logic::<64>(rax & rax);
            // 699a5 jne 0x6999c
            if crate::cov::brk(0x699a5, fl.ne()) { pc = 0x6999c; continue; }
            pc = 0x699a7;
        }
        0x699a7 => {
            // block 699a7
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699a7 jmp 0x699b8
            pc = 0x699b8; continue;
        }
        0x699a9 => {
            // block 699a9
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699a9 mov rax,rbx
            rax = rbx;
            pc = 0x699ac;
        }
        0x699ac => {
            // block 699ac
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699ac mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 699b0 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 699b3 mov rax,rbx
            rax = rbx;
            // 699b6 jne 0x699ac
            if crate::cov::brk(0x699b6, fl.ne()) { pc = 0x699ac; continue; }
            pc = 0x699b8;
        }
        0x699b8 => {
            // block 699b8
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699b8 cmp rbx,r13
            fl = sub::<64>(rbx, r13).1;
            // 699bb jne 0x6987d
            if crate::cov::brk(0x699bb, fl.ne()) { pc = 0x6987d; continue; }
            pc = 0x699c1;
        }
        0x699c1 => {
            // block 699c1
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699c1 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 699c5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 699ca mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 699ce add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 699d2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 699d3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 699d5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 699d7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 699d9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 699db pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 699dc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x699dd => {
            // block 699dd
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699dd mov rbx,rax
            rbx = rax;
            // 699e0 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 699e4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 699e9 jmp 0x699ee
            pc = 0x699ee; continue;
        }
        0x699eb => {
            // block 699eb
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699eb mov rbx,rax
            rbx = rax;
            pc = 0x699ee;
        }
        0x699ee => {
            // block 699ee
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699ee mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 699f2 mov rsi,QWORD PTR [rdi+0x8]
            rsi = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 699f6 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 699fb mov rdi,rbx
            rdi = rbx;
            // 699fe call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4555 size=591
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4555<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc4555;
    loop {
        match pc {
        0xc4555 => {
            // block c4555
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4555 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4555 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4556 mov rbp,rsp
            rbp = rsp;
            // c4559 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c455b push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c455c mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // c455f test rax,rax
            fl = logic::<64>(rax & rax);
            // c4562 mov rcx,rsi
            rcx = rsi;
            // c4565 je 0xc457b
            if crate::cov::brk(0xc4565, fl.e()) { pc = 0xc457b; continue; }
            pc = 0xc4567;
        }
        0xc4567 => {
            // block c4567
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4567 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4567 mov rdx,QWORD PTR [rsi+0x8]
            rdx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c456b test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c456e je 0xc458f
            if crate::cov::brk(0xc456e, fl.e()) { pc = 0xc458f; continue; }
            pc = 0xc4570;
        }
        0xc4570 => {
            // block c4570
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4570 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4570 mov rcx,rdx
            rcx = rdx;
            // c4573 mov rdx,QWORD PTR [rcx]
            rdx = m.r64(rcx) as u64;
            // c4576 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4579 jne 0xc4570
            if crate::cov::brk(0xc4579, fl.ne()) { pc = 0xc4570; continue; }
            pc = 0xc457b;
        }
        0xc457b => {
            // block c457b
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c457b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c457b mov rax,QWORD PTR [rcx+0x8]
            rax = m.r64(rcx.wrapping_add(0x8)) as u64;
            // c457f test rax,rax
            fl = logic::<64>(rax & rax);
            // c4582 jne 0xc4592
            if crate::cov::brk(0xc4582, fl.ne()) { pc = 0xc4592; continue; }
            pc = 0xc4584;
        }
        0xc4584 => {
            // block c4584
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4584 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4584 lea r9,[rcx+0x10]
            r9 = rcx.wrapping_add(0x10);
            // c4588 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c458b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c458d jmp 0xc45a1
            pc = 0xc45a1; continue;
        }
        0xc458f => {
            // block c458f
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c458f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c458f mov rcx,rsi
            rcx = rsi;
            pc = 0xc4592;
        }
        0xc4592 => {
            // block c4592
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4592 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4592 mov rdx,QWORD PTR [rcx+0x10]
            rdx = m.r64(rcx.wrapping_add(0x10)) as u64;
            // c4596 mov QWORD PTR [rax+0x10],rdx
            m.w64(rax.wrapping_add(0x10), (rdx) as u64);
            // c459a mov r8b,0x1
            r8 = (r8 & !0xff) | ((0x1u64) as u8 as u64);
            // c459d lea r9,[rcx+0x10]
            r9 = rcx.wrapping_add(0x10);
            pc = 0xc45a1;
        }
        0xc45a1 => {
            // block c45a1
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45a1 mov rdx,QWORD PTR [r9]
            rdx = m.r64(r9) as u64;
            // c45a4 mov rbx,QWORD PTR [rdx]
            rbx = m.r64(rdx) as u64;
            // c45a7 cmp rbx,rcx
            fl = sub::<64>(rbx, rcx).1;
            // c45aa je 0xc45b2
            if crate::cov::brk(0xc45aa, fl.e()) { pc = 0xc45b2; continue; }
            pc = 0xc45ac;
        }
        0xc45ac => {
            // block c45ac
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45ac mov QWORD PTR [rdx+0x8],rax
            m.w64(rdx.wrapping_add(0x8), (rax) as u64);
            // c45b0 jmp 0xc45c3
            pc = 0xc45c3; continue;
        }
        0xc45b2 => {
            // block c45b2
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45b2 mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // c45b5 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c45b7 cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // c45ba mov r14,rax
            r14 = rax;
            // c45bd je 0xc45c6
            if crate::cov::brk(0xc45bd, fl.e()) { pc = 0xc45c6; continue; }
            pc = 0xc45bf;
        }
        0xc45bf => {
            // block c45bf
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45bf mov rbx,QWORD PTR [rdx+0x8]
            rbx = m.r64(rdx.wrapping_add(0x8)) as u64;
            pc = 0xc45c3;
        }
        0xc45c3 => {
            // block c45c3
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45c3 mov r14,rdi
            r14 = rdi;
            pc = 0xc45c6;
        }
        0xc45c6 => {
            // block c45c6
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45c6 mov r10b,BYTE PTR [rcx+0x18]
            r10 = (r10 & !0xff) | ((m.r8(rcx.wrapping_add(0x18)) as u64) as u8 as u64);
            // c45ca cmp rcx,rsi
            fl = sub::<64>(rcx, rsi).1;
            // c45cd je 0xc4610
            if crate::cov::brk(0xc45cd, fl.e()) { pc = 0xc4610; continue; }
            pc = 0xc45cf;
        }
        0xc45cf => {
            // block c45cf
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45cf mov rdx,QWORD PTR [rsi+0x10]
            rdx = m.r64(rsi.wrapping_add(0x10)) as u64;
            // c45d3 mov QWORD PTR [r9],rdx
            m.w64(r9, (rdx) as u64);
            // c45d6 mov rdi,QWORD PTR [rsi+0x10]
            rdi = m.r64(rsi.wrapping_add(0x10)) as u64;
            // c45da cmp QWORD PTR [rdi],rsi
            fl = sub::<64>(m.r64(rdi) as u64, rsi).1;
            // c45dd je 0xc45e5
            if crate::cov::brk(0xc45dd, fl.e()) { pc = 0xc45e5; continue; }
            pc = 0xc45df;
        }
        0xc45df => {
            // block c45df
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45df mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // c45e3 jmp 0xc45e8
            pc = 0xc45e8; continue;
        }
        0xc45e5 => {
            // block c45e5
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45e5 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            pc = 0xc45e8;
        }
        0xc45e8 => {
            // block c45e8
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45e8 mov rdx,QWORD PTR [rsi]
            rdx = m.r64(rsi) as u64;
            // c45eb mov QWORD PTR [rcx],rdx
            m.w64(rcx, (rdx) as u64);
            // c45ee mov QWORD PTR [rdx+0x10],rcx
            m.w64(rdx.wrapping_add(0x10), (rcx) as u64);
            // c45f2 mov rdx,QWORD PTR [rsi+0x8]
            rdx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c45f6 mov QWORD PTR [rcx+0x8],rdx
            m.w64(rcx.wrapping_add(0x8), (rdx) as u64);
            // c45fa test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c45fd je 0xc4603
            if crate::cov::brk(0xc45fd, fl.e()) { pc = 0xc4603; continue; }
            pc = 0xc45ff;
        }
        0xc45ff => {
            // block c45ff
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45ff mov QWORD PTR [rdx+0x10],rcx
            m.w64(rdx.wrapping_add(0x10), (rcx) as u64);
            pc = 0xc4603;
        }
        0xc4603 => {
            // block c4603
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4603 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4603 mov dl,BYTE PTR [rsi+0x18]
            rdx = (rdx & !0xff) | ((m.r8(rsi.wrapping_add(0x18)) as u64) as u8 as u64);
            // c4606 mov BYTE PTR [rcx+0x18],dl
            m.w8(rcx.wrapping_add(0x18), ((rdx as u8 as u64)) as u8);
            // c4609 cmp r14,rsi
            fl = sub::<64>(r14, rsi).1;
            // c460c cmove r14,rcx
            if fl.e() { r14 = rcx; }
            pc = 0xc4610;
        }
        0xc4610 => {
            // block c4610
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4610 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4610 test r10b,r10b
            fl = logic::<8>((r10 as u8 as u64) & (r10 as u8 as u64));
            // c4613 je 0xc470d
            if crate::cov::brk(0xc4613, fl.e()) { pc = 0xc470d; continue; }
            pc = 0xc4619;
        }
        0xc4619 => {
            // block c4619
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4619 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4619 test r14,r14
            fl = logic::<64>(r14 & r14);
            // c461c je 0xc470d
            if crate::cov::brk(0xc461c, fl.e()) { pc = 0xc470d; continue; }
            pc = 0xc4622;
        }
        0xc4622 => {
            // block c4622
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4622 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4622 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // c4625 je 0xc4642
            if crate::cov::brk(0xc4625, fl.e()) { pc = 0xc4642; continue; }
            pc = 0xc4627;
        }
        0xc4627 => {
            // block c4627
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4627 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4627 mov BYTE PTR [rax+0x18],0x1
            m.w8(rax.wrapping_add(0x18), (0x1u64) as u8);
            // c462b jmp 0xc470d
            pc = 0xc470d; continue;
        }
        0xc4630 => {
            // block c4630
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4630 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4630 mov rdx,QWORD PTR [rcx+0x10]
            rdx = m.r64(rcx.wrapping_add(0x10)) as u64;
            // c4634 lea rax,[rdx+0x8]
            rax = rdx.wrapping_add(0x8);
            // c4638 cmp QWORD PTR [rdx],rcx
            fl = sub::<64>(m.r64(rdx) as u64, rcx).1;
            // c463b cmovne rax,rdx
            if fl.ne() { rax = rdx; }
            // c463f mov rbx,QWORD PTR [rax]
            rbx = m.r64(rax) as u64;
            pc = 0xc4642;
        }
        0xc4642 => {
            // block c4642
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4642 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4642 mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c4646 cmp QWORD PTR [rdi],rbx
            fl = sub::<64>(m.r64(rdi) as u64, rbx).1;
            // c4649 mov al,BYTE PTR [rbx+0x18]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0x18)) as u64) as u8 as u64);
            // c464c je 0xc46a7
            if crate::cov::brk(0xc464c, fl.e()) { pc = 0xc46a7; continue; }
            pc = 0xc464e;
        }
        0xc464e => {
            // block c464e
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c464e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c464e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c4650 jne 0xc466d
            if crate::cov::brk(0xc4650, fl.ne()) { pc = 0xc466d; continue; }
            pc = 0xc4652;
        }
        0xc4652 => {
            // block c4652
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4652 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4652 mov BYTE PTR [rbx+0x18],0x1
            m.w8(rbx.wrapping_add(0x18), (0x1u64) as u8);
            // c4656 mov BYTE PTR [rdi+0x18],0x0
            m.w8(rdi.wrapping_add(0x18), (0x0u64) as u8);
            // c465a call 0xebaaa
            { let r = { let t = m.env.stub_va(0x277); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c465f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c4662 cmp r14,rax
            fl = sub::<64>(r14, rax).1;
            // c4665 cmove r14,rbx
            if fl.e() { r14 = rbx; }
            // c4669 mov rbx,QWORD PTR [rax+0x8]
            rbx = m.r64(rax.wrapping_add(0x8)) as u64;
            pc = 0xc466d;
        }
        0xc466d => {
            // block c466d
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c466d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c466d mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // c4670 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c4673 je 0xc467f
            if crate::cov::brk(0xc4673, fl.e()) { pc = 0xc467f; continue; }
            pc = 0xc4675;
        }
        0xc4675 => {
            // block c4675
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4675 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4675 cmp BYTE PTR [rcx+0x18],0x0
            fl = sub::<8>(m.r8(rcx.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c4679 je 0xc4712
            if crate::cov::brk(0xc4679, fl.e()) { pc = 0xc4712; continue; }
            pc = 0xc467f;
        }
        0xc467f => {
            // block c467f
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c467f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c467f mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c4683 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4686 je 0xc4692
            if crate::cov::brk(0xc4686, fl.e()) { pc = 0xc4692; continue; }
            pc = 0xc4688;
        }
        0xc4688 => {
            // block c4688
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4688 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4688 cmp BYTE PTR [rax+0x18],0x0
            fl = sub::<8>(m.r8(rax.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c468c je 0xc471b
            if crate::cov::brk(0xc468c, fl.e()) { pc = 0xc471b; continue; }
            pc = 0xc4692;
        }
        0xc4692 => {
            // block c4692
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4692 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4692 mov BYTE PTR [rbx+0x18],0x0
            m.w8(rbx.wrapping_add(0x18), (0x0u64) as u8);
            // c4696 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c469a cmp rcx,r14
            fl = sub::<64>(rcx, r14).1;
            // c469d je 0xc4708
            if crate::cov::brk(0xc469d, fl.e()) { pc = 0xc4708; continue; }
            pc = 0xc469f;
        }
        0xc469f => {
            // block c469f
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c469f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c469f cmp BYTE PTR [rcx+0x18],0x0
            fl = sub::<8>(m.r8(rcx.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c46a3 jne 0xc4630
            if crate::cov::brk(0xc46a3, fl.ne()) { pc = 0xc4630; continue; }
            pc = 0xc46a5;
        }
        0xc46a5 => {
            // block c46a5
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46a5 jmp 0xc4705
            pc = 0xc4705; continue;
        }
        0xc46a7 => {
            // block c46a7
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46a7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c46a9 jne 0xc46c6
            if crate::cov::brk(0xc46a9, fl.ne()) { pc = 0xc46c6; continue; }
            pc = 0xc46ab;
        }
        0xc46ab => {
            // block c46ab
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46ab mov BYTE PTR [rbx+0x18],0x1
            m.w8(rbx.wrapping_add(0x18), (0x1u64) as u8);
            // c46af mov BYTE PTR [rdi+0x18],0x0
            m.w8(rdi.wrapping_add(0x18), (0x0u64) as u8);
            // c46b3 call 0xeb86a
            { let r = { let t = m.env.stub_va(0x37); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c46b8 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c46bc cmp r14,rax
            fl = sub::<64>(r14, rax).1;
            // c46bf cmove r14,rbx
            if fl.e() { r14 = rbx; }
            // c46c3 mov rbx,QWORD PTR [rax]
            rbx = m.r64(rax) as u64;
            pc = 0xc46c6;
        }
        0xc46c6 => {
            // block c46c6
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46c6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c46c9 test rax,rax
            fl = logic::<64>(rax & rax);
            // c46cc je 0xc46d8
            if crate::cov::brk(0xc46cc, fl.e()) { pc = 0xc46d8; continue; }
            pc = 0xc46ce;
        }
        0xc46ce => {
            // block c46ce
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46ce cmp BYTE PTR [rax+0x18],0x0
            fl = sub::<8>(m.r8(rax.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c46d2 je 0xc4759
            if crate::cov::brk(0xc46d2, fl.e()) { pc = 0xc4759; continue; }
            pc = 0xc46d8;
        }
        0xc46d8 => {
            // block c46d8
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46d8 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c46dc test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c46df je 0xc46e7
            if crate::cov::brk(0xc46df, fl.e()) { pc = 0xc46e7; continue; }
            pc = 0xc46e1;
        }
        0xc46e1 => {
            // block c46e1
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46e1 cmp BYTE PTR [rcx+0x18],0x0
            fl = sub::<8>(m.r8(rcx.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c46e5 je 0xc4754
            if crate::cov::brk(0xc46e5, fl.e()) { pc = 0xc4754; continue; }
            pc = 0xc46e7;
        }
        0xc46e7 => {
            // block c46e7
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46e7 mov BYTE PTR [rbx+0x18],0x0
            m.w8(rbx.wrapping_add(0x18), (0x0u64) as u8);
            // c46eb mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c46ef cmp rcx,r14
            fl = sub::<64>(rcx, r14).1;
            // c46f2 je 0xc46ff
            if crate::cov::brk(0xc46f2, fl.e()) { pc = 0xc46ff; continue; }
            pc = 0xc46f4;
        }
        0xc46f4 => {
            // block c46f4
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46f4 mov al,BYTE PTR [rcx+0x18]
            rax = (rax & !0xff) | ((m.r8(rcx.wrapping_add(0x18)) as u64) as u8 as u64);
            // c46f7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c46f9 jne 0xc4630
            if crate::cov::brk(0xc46f9, fl.ne()) { pc = 0xc4630; continue; }
            pc = 0xc46ff;
        }
        0xc46ff => {
            // block c46ff
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46ff mov BYTE PTR [rcx+0x18],0x1
            m.w8(rcx.wrapping_add(0x18), (0x1u64) as u8);
            // c4703 jmp 0xc470d
            pc = 0xc470d; continue;
        }
        0xc4705 => {
            // block c4705
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4705 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4705 mov r14,rcx
            r14 = rcx;
            pc = 0xc4708;
        }
        0xc4708 => {
            // block c4708
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4708 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4708 mov BYTE PTR [r14+0x18],0x1
            m.w8(r14.wrapping_add(0x18), (0x1u64) as u8);
            pc = 0xc470d;
        }
        0xc470d => {
            // block c470d
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c470d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c470d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c470e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4710 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4711 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc4712 => {
            // block c4712
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4712 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4712 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c4716 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4719 je 0xc4721
            if crate::cov::brk(0xc4719, fl.e()) { pc = 0xc4721; continue; }
            pc = 0xc471b;
        }
        0xc471b => {
            // block c471b
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c471b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c471b cmp BYTE PTR [rax+0x18],0x0
            fl = sub::<8>(m.r8(rax.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c471f je 0xc4739
            if crate::cov::brk(0xc471f, fl.e()) { pc = 0xc4739; continue; }
            pc = 0xc4721;
        }
        0xc4721 => {
            // block c4721
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4721 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4721 mov BYTE PTR [rcx+0x18],0x1
            m.w8(rcx.wrapping_add(0x18), (0x1u64) as u8);
            // c4725 mov BYTE PTR [rbx+0x18],0x0
            m.w8(rbx.wrapping_add(0x18), (0x0u64) as u8);
            // c4729 mov rdi,rbx
            rdi = rbx;
            // c472c call 0xeb86a
            { let r = { let t = m.env.stub_va(0x37); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c4731 mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c4735 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0xc4739;
        }
        0xc4739 => {
            // block c4739
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4739 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4739 mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c473d mov cl,BYTE PTR [rdi+0x18]
            rcx = (rcx & !0xff) | ((m.r8(rdi.wrapping_add(0x18)) as u64) as u8 as u64);
            // c4740 mov BYTE PTR [rbx+0x18],cl
            m.w8(rbx.wrapping_add(0x18), ((rcx as u8 as u64)) as u8);
            // c4743 mov BYTE PTR [rdi+0x18],0x1
            m.w8(rdi.wrapping_add(0x18), (0x1u64) as u8);
            // c4747 mov BYTE PTR [rax+0x18],0x1
            m.w8(rax.wrapping_add(0x18), (0x1u64) as u8);
            // c474b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c474c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c474e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c474f jmp 0xebaaa
            { let r = { let t = m.env.stub_va(0x277); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc4754 => {
            // block c4754
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4754 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4754 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4757 je 0xc475f
            if crate::cov::brk(0xc4757, fl.e()) { pc = 0xc475f; continue; }
            pc = 0xc4759;
        }
        0xc4759 => {
            // block c4759
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4759 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4759 cmp BYTE PTR [rax+0x18],0x0
            fl = sub::<8>(m.r8(rax.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c475d je 0xc477a
            if crate::cov::brk(0xc475d, fl.e()) { pc = 0xc477a; continue; }
            pc = 0xc475f;
        }
        0xc475f => {
            // block c475f
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c475f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c475f mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c4763 mov BYTE PTR [rax+0x18],0x1
            m.w8(rax.wrapping_add(0x18), (0x1u64) as u8);
            // c4767 mov BYTE PTR [rbx+0x18],0x0
            m.w8(rbx.wrapping_add(0x18), (0x0u64) as u8);
            // c476b mov rdi,rbx
            rdi = rbx;
            // c476e call 0xebaaa
            { let r = { let t = m.env.stub_va(0x277); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c4773 mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c4777 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            pc = 0xc477a;
        }
        0xc477a => {
            // block c477a
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c477a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c477a mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c477e mov cl,BYTE PTR [rdi+0x18]
            rcx = (rcx & !0xff) | ((m.r8(rdi.wrapping_add(0x18)) as u64) as u8 as u64);
            // c4781 mov BYTE PTR [rbx+0x18],cl
            m.w8(rbx.wrapping_add(0x18), ((rcx as u8 as u64)) as u8);
            // c4784 mov BYTE PTR [rdi+0x18],0x1
            m.w8(rdi.wrapping_add(0x18), (0x1u64) as u8);
            // c4788 mov BYTE PTR [rax+0x18],0x1
            m.w8(rax.wrapping_add(0x18), (0x1u64) as u8);
            // c478c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c478d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c478f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4790 jmp 0xeb86a
            { let r = { let t = m.env.stub_va(0x37); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7110 size=600
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d7110<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd7110;
    loop {
        match pc {
        0xd7110 => {
            // block d7110
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7110 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7110 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7111 mov rbp,rsp
            rbp = rsp;
            // d7114 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d7116 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d7118 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d711a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d711c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d711d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d711e mov r12,rsi
            r12 = rsi;
            // d7121 mov r14,rdi
            r14 = rdi;
            // d7124 lea rsi,[rip+0xe6655] # 0x1bd780
            rsi = m.env.data_va(859981);
            // d712b mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // d7130 mov rdi,r12
            rdi = r12;
            // d7133 call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d7138 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d713a jne 0xd722c
            if crate::cov::brk(0xd713a, fl.ne()) { pc = 0xd722c; continue; }
            pc = 0xd7140;
        }
        0xd7140 => {
            // block d7140
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7140 movzx eax,BYTE PTR [r12+0xd]
            rax = (m.r8(r12.wrapping_add(0xd)) as u64) as u32 as u64;
            // d7146 cmp eax,0x61
            fl = sub::<32>((rax as u32 as u64), 0x61u64).1;
            // d7149 jne 0xd722c
            if crate::cov::brk(0xd7149, fl.ne()) { pc = 0xd722c; continue; }
            pc = 0xd714f;
        }
        0xd714f => {
            // block d714f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d714f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d714f mov bx,WORD PTR [r12+0xe]
            rbx = (rbx & !0xffff) | ((m.r16(r12.wrapping_add(0xe)) as u64) as u16 as u64);
            // d7155 rol bx,0x8
            { let (r, f) = rol::<16>((rbx as u16 as u64), 0x8u64, fl); rbx = (rbx & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d7159 lea rsi,[r12+0x10]
            rsi = r12.wrapping_add(0x10);
            // d715e lea rdi,[r14+0x2e8]
            rdi = r14.wrapping_add(0x2e8);
            // d7165 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d716a movzx ebx,bx
            rbx = ((rbx as u16 as u64)) as u32 as u64;
            // d716d movzx eax,BYTE PTR [rbx+r12*1+0x10]
            rax = (m.r8(rbx.wrapping_add(r12).wrapping_add(0x10)) as u64) as u32 as u64;
            // d7173 cmp eax,0x62
            fl = sub::<32>((rax as u32 as u64), 0x62u64).1;
            // d7176 jne 0xd722c
            if crate::cov::brk(0xd7176, fl.ne()) { pc = 0xd722c; continue; }
            pc = 0xd717c;
        }
        0xd717c => {
            // block d717c
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d717c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d717c mov r13w,WORD PTR [rbx+r12*1+0x11]
            r13 = (r13 & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x11)) as u64) as u16 as u64);
            // d7182 rol r13w,0x8
            { let (r, f) = rol::<16>((r13 as u16 as u64), 0x8u64, fl); r13 = (r13 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d7187 lea rsi,[r12+rbx*1+0x13]
            rsi = r12.wrapping_add(rbx).wrapping_add(0x13);
            // d718c lea r15,[r14+0x300]
            r15 = r14.wrapping_add(0x300);
            // d7193 mov rdi,r15
            rdi = r15;
            // d7196 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d719b movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // d719f lea rbx,[rax+rbx*1+0x13]
            rbx = rax.wrapping_add(rbx).wrapping_add(0x13);
            // d71a4 movzx eax,BYTE PTR [r12+rbx*1]
            rax = (m.r8(r12.wrapping_add(rbx)) as u64) as u32 as u64;
            // d71a9 cmp eax,0x63
            fl = sub::<32>((rax as u32 as u64), 0x63u64).1;
            // d71ac jne 0xd722c
            if crate::cov::brk(0xd71ac, fl.ne()) { pc = 0xd722c; continue; }
            pc = 0xd71ae;
        }
        0xd71ae => {
            // block d71ae
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d71ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d71ae mov r13w,WORD PTR [rbx+r12*1+0x1]
            r13 = (r13 & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x1)) as u64) as u16 as u64);
            // d71b4 rol r13w,0x8
            { let (r, f) = rol::<16>((r13 as u16 as u64), 0x8u64, fl); r13 = (r13 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d71b9 lea rsi,[r12+rbx*1+0x3]
            rsi = r12.wrapping_add(rbx).wrapping_add(0x3);
            // d71be lea rdi,[r14+0x2b8]
            rdi = r14.wrapping_add(0x2b8);
            // d71c5 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d71ca movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // d71ce lea rbx,[rax+rbx*1+0x3]
            rbx = rax.wrapping_add(rbx).wrapping_add(0x3);
            // d71d3 movzx eax,BYTE PTR [r12+rbx*1]
            rax = (m.r8(r12.wrapping_add(rbx)) as u64) as u32 as u64;
            // d71d8 cmp eax,0x64
            fl = sub::<32>((rax as u32 as u64), 0x64u64).1;
            // d71db jne 0xd722c
            if crate::cov::brk(0xd71db, fl.ne()) { pc = 0xd722c; continue; }
            pc = 0xd71dd;
        }
        0xd71dd => {
            // block d71dd
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d71dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d71dd mov r13w,WORD PTR [rbx+r12*1+0x1]
            r13 = (r13 & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x1)) as u64) as u16 as u64);
            // d71e3 rol r13w,0x8
            { let (r, f) = rol::<16>((r13 as u16 as u64), 0x8u64, fl); r13 = (r13 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d71e8 lea rsi,[r12+rbx*1+0x3]
            rsi = r12.wrapping_add(rbx).wrapping_add(0x3);
            // d71ed lea rdi,[r14+0x2d0]
            rdi = r14.wrapping_add(0x2d0);
            // d71f4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d71f9 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // d71fd lea rax,[rax+rbx*1+0x3]
            rax = rax.wrapping_add(rbx).wrapping_add(0x3);
            // d7202 movzx ecx,BYTE PTR [r12+rax*1]
            rcx = (m.r8(r12.wrapping_add(rax)) as u64) as u32 as u64;
            // d7207 cmp ecx,0x65
            fl = sub::<32>((rcx as u32 as u64), 0x65u64).1;
            // d720a jne 0xd722c
            if crate::cov::brk(0xd720a, fl.ne()) { pc = 0xd722c; continue; }
            pc = 0xd720c;
        }
        0xd720c => {
            // block d720c
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d720c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d720c mov ecx,DWORD PTR [rax+r12*1+0x1]
            rcx = (m.r32(rax.wrapping_add(r12).wrapping_add(0x1)) as u64) as u32 as u64;
            // d7211 bswap ecx
            rcx = (rcx as u32).swap_bytes() as u64;
            // d7213 mov DWORD PTR [r14+0x36c],ecx
            m.w32(r14.wrapping_add(0x36c), ((rcx as u32 as u64)) as u32);
            // d721a mov cl,BYTE PTR [r14+0x300]
            rcx = (rcx & !0xff) | ((m.r8(r14.wrapping_add(0x300)) as u64) as u8 as u64);
            // d7221 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d7224 jne 0xd723d
            if crate::cov::brk(0xd7224, fl.ne()) { pc = 0xd723d; continue; }
            pc = 0xd7226;
        }
        0xd7226 => {
            // block d7226
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7226 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7226 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d722a jmp 0xd7244
            pc = 0xd7244; continue;
        }
        0xd722c => {
            // block d722c
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d722c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d722c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd722e;
        }
        0xd722e => {
            // block d722e
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d722e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d722e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d7232 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d7233 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d7235 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d7237 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d7239 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d723b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d723c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd723d => {
            // block d723d
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d723d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d723d mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd7244;
        }
        0xd7244 => {
            // block d7244
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7244 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7247 cmp edx,0x35
            fl = sub::<32>((rdx as u32 as u64), 0x35u64).1;
            // d724a je 0xd729b
            if crate::cov::brk(0xd724a, fl.e()) { pc = 0xd729b; continue; }
            pc = 0xd724c;
        }
        0xd724c => {
            // block d724c
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d724c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d724c test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d724e jne 0xd7256
            if crate::cov::brk(0xd724e, fl.ne()) { pc = 0xd7256; continue; }
            pc = 0xd7250;
        }
        0xd7250 => {
            // block d7250
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7250 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7250 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d7254 jmp 0xd725d
            pc = 0xd725d; continue;
        }
        0xd7256 => {
            // block d7256
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7256 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7256 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd725d;
        }
        0xd725d => {
            // block d725d
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d725d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d725d movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7260 cmp edx,0x36
            fl = sub::<32>((rdx as u32 as u64), 0x36u64).1;
            // d7263 je 0xd729b
            if crate::cov::brk(0xd7263, fl.e()) { pc = 0xd729b; continue; }
            pc = 0xd7265;
        }
        0xd7265 => {
            // block d7265
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7265 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7265 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d7267 jne 0xd726f
            if crate::cov::brk(0xd7267, fl.ne()) { pc = 0xd726f; continue; }
            pc = 0xd7269;
        }
        0xd7269 => {
            // block d7269
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7269 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7269 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d726d jmp 0xd7276
            pc = 0xd7276; continue;
        }
        0xd726f => {
            // block d726f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d726f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d726f mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd7276;
        }
        0xd7276 => {
            // block d7276
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7276 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7276 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7279 cmp edx,0x37
            fl = sub::<32>((rdx as u32 as u64), 0x37u64).1;
            // d727c je 0xd729b
            if crate::cov::brk(0xd727c, fl.e()) { pc = 0xd729b; continue; }
            pc = 0xd727e;
        }
        0xd727e => {
            // block d727e
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d727e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d727e test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d7280 jne 0xd7288
            if crate::cov::brk(0xd7280, fl.ne()) { pc = 0xd7288; continue; }
            pc = 0xd7282;
        }
        0xd7282 => {
            // block d7282
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7282 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7282 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d7286 jmp 0xd728f
            pc = 0xd728f; continue;
        }
        0xd7288 => {
            // block d7288
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7288 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7288 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd728f;
        }
        0xd728f => {
            // block d728f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d728f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d728f movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7292 cmp edx,0x78
            fl = sub::<32>((rdx as u32 as u64), 0x78u64).1;
            // d7295 jne 0xd7361
            if crate::cov::brk(0xd7295, fl.ne()) { pc = 0xd7361; continue; }
            pc = 0xd729b;
        }
        0xd729b => {
            // block d729b
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d729b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d729b test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d729d jne 0xd72a5
            if crate::cov::brk(0xd729d, fl.ne()) { pc = 0xd72a5; continue; }
            pc = 0xd729f;
        }
        0xd729f => {
            // block d729f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d729f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d729f lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72a3 jmp 0xd72ac
            pc = 0xd72ac; continue;
        }
        0xd72a5 => {
            // block d72a5
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72a5 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72ac;
        }
        0xd72ac => {
            // block d72ac
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72ac movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d72af cmp edx,0x35
            fl = sub::<32>((rdx as u32 as u64), 0x35u64).1;
            // d72b2 je 0xd731a
            if crate::cov::brk(0xd72b2, fl.e()) { pc = 0xd731a; continue; }
            pc = 0xd72b4;
        }
        0xd72b4 => {
            // block d72b4
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72b4 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d72b6 jne 0xd72be
            if crate::cov::brk(0xd72b6, fl.ne()) { pc = 0xd72be; continue; }
            pc = 0xd72b8;
        }
        0xd72b8 => {
            // block d72b8
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72b8 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72bc jmp 0xd72c5
            pc = 0xd72c5; continue;
        }
        0xd72be => {
            // block d72be
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72be mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72c5;
        }
        0xd72c5 => {
            // block d72c5
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72c5 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d72c8 cmp edx,0x36
            fl = sub::<32>((rdx as u32 as u64), 0x36u64).1;
            // d72cb jne 0xd72e7
            if crate::cov::brk(0xd72cb, fl.ne()) { pc = 0xd72e7; continue; }
            pc = 0xd72cd;
        }
        0xd72cd => {
            // block d72cd
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72cd test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d72cf jne 0xd72d7
            if crate::cov::brk(0xd72cf, fl.ne()) { pc = 0xd72d7; continue; }
            pc = 0xd72d1;
        }
        0xd72d1 => {
            // block d72d1
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72d1 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72d5 jmp 0xd72de
            pc = 0xd72de; continue;
        }
        0xd72d7 => {
            // block d72d7
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72d7 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72de;
        }
        0xd72de => {
            // block d72de
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72de movzx edx,BYTE PTR [rdx+0x1]
            rdx = (m.r8(rdx.wrapping_add(0x1)) as u64) as u32 as u64;
            // d72e2 cmp edx,0x76
            fl = sub::<32>((rdx as u32 as u64), 0x76u64).1;
            // d72e5 je 0xd731a
            if crate::cov::brk(0xd72e5, fl.e()) { pc = 0xd731a; continue; }
            pc = 0xd72e7;
        }
        0xd72e7 => {
            // block d72e7
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72e7 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d72e9 jne 0xd72f1
            if crate::cov::brk(0xd72e9, fl.ne()) { pc = 0xd72f1; continue; }
            pc = 0xd72eb;
        }
        0xd72eb => {
            // block d72eb
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72eb lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72ef jmp 0xd72f8
            pc = 0xd72f8; continue;
        }
        0xd72f1 => {
            // block d72f1
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72f1 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72f8;
        }
        0xd72f8 => {
            // block d72f8
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72f8 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d72fb cmp edx,0x37
            fl = sub::<32>((rdx as u32 as u64), 0x37u64).1;
            // d72fe jne 0xd733a
            if crate::cov::brk(0xd72fe, fl.ne()) { pc = 0xd733a; continue; }
            pc = 0xd7300;
        }
        0xd7300 => {
            // block d7300
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7300 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7300 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d7302 jne 0xd730a
            if crate::cov::brk(0xd7302, fl.ne()) { pc = 0xd730a; continue; }
            pc = 0xd7304;
        }
        0xd7304 => {
            // block d7304
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7304 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7304 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d7308 jmp 0xd7311
            pc = 0xd7311; continue;
        }
        0xd730a => {
            // block d730a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d730a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d730a mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd7311;
        }
        0xd7311 => {
            // block d7311
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7311 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7311 movzx edx,BYTE PTR [rdx+0x1]
            rdx = (m.r8(rdx.wrapping_add(0x1)) as u64) as u32 as u64;
            // d7315 cmp edx,0x6b
            fl = sub::<32>((rdx as u32 as u64), 0x6bu64).1;
            // d7318 jne 0xd733a
            if crate::cov::brk(0xd7318, fl.ne()) { pc = 0xd733a; continue; }
            pc = 0xd731a;
        }
        0xd731a => {
            // block d731a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d731a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d731a lea rdi,[rax+r12*1+0x31]
            rdi = rax.wrapping_add(r12).wrapping_add(0x31);
            pc = 0xd731f;
        }
        0xd731f => {
            // block d731f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d731f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d731f lea rsi,[rip+0xe6467] # 0x1bd78d
            rsi = m.env.data_va(859994);
            // d7326 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // d732b call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d7330 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d7332 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d7335 jmp 0xd722e
            pc = 0xd722e; continue;
        }
        0xd733a => {
            // block d733a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d733a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d733a test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d733c jne 0xd7343
            if crate::cov::brk(0xd733c, fl.ne()) { pc = 0xd7343; continue; }
            pc = 0xd733e;
        }
        0xd733e => {
            // block d733e
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d733e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d733e inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // d7341 jmp 0xd734a
            pc = 0xd734a; continue;
        }
        0xd7343 => {
            // block d7343
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7343 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7343 mov r15,QWORD PTR [r14+0x310]
            r15 = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd734a;
        }
        0xd734a => {
            // block d734a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d734a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d734a movzx ecx,BYTE PTR [r15]
            rcx = (m.r8(r15) as u64) as u32 as u64;
            // d734e cmp ecx,0x78
            fl = sub::<32>((rcx as u32 as u64), 0x78u64).1;
            // d7351 jne 0xd735a
            if crate::cov::brk(0xd7351, fl.ne()) { pc = 0xd735a; continue; }
            pc = 0xd7353;
        }
        0xd7353 => {
            // block d7353
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7353 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7353 lea rdi,[rax+r12*1+0x51]
            rdi = rax.wrapping_add(r12).wrapping_add(0x51);
            // d7358 jmp 0xd731f
            pc = 0xd731f; continue;
        }
        0xd735a => {
            // block d735a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d735a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d735a lea rdi,[rax+r12*1+0x11]
            rdi = rax.wrapping_add(r12).wrapping_add(0x11);
            // d735f jmp 0xd731f
            pc = 0xd731f; continue;
        }
        0xd7361 => {
            // block d7361
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7361 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7361 lea rdi,[rax+r12*1+0x5]
            rdi = rax.wrapping_add(r12).wrapping_add(0x5);
            // d7366 jmp 0xd731f
            pc = 0xd731f; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2fdd8 size=616
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002fdd8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2fdd8;
    loop {
        match pc {
        0x2fdd8 => {
            // block 2fdd8
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fdd8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fdd8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2fdd9 mov rbp,rsp
            rbp = rsp;
            // 2fddc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2fdde push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2fde0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2fde2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2fde4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2fde5 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 2fde9 mov DWORD PTR [rbp-0x48],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), ((r8 as u32 as u64)) as u32);
            // 2fded mov rbx,rdx
            rbx = rdx;
            // 2fdf0 mov r15,rsi
            r15 = rsi;
            // 2fdf3 mov r14,rdi
            r14 = rdi;
            // 2fdf6 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2fdf9 cmp rax,QWORD PTR [r15+0x8]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 2fdfd je 0x2ff81
            if crate::cov::brk(0x2fdfd, fl.e()) { pc = 0x2ff81; continue; }
            pc = 0x2fe03;
        }
        0x2fe03 => {
            // block 2fe03
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fe03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fe03 mov QWORD PTR [rbp-0x50],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rcx) as u64);
            // 2fe07 mov rdi,r14
            rdi = r14;
            // 2fe0a call 0x31164
            { let r = { let t = m.env.code_va(0x31164); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2fe0f lea rdi,[r14+0x18]
            rdi = r14.wrapping_add(0x18);
            // 2fe13 mov QWORD PTR [rbp-0x40],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdi) as u64);
            // 2fe17 mov rax,QWORD PTR [r14+0x18]
            rax = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2fe1b mov esi,DWORD PTR [r14+0x2b4]
            rsi = (m.r32(r14.wrapping_add(0x2b4)) as u64) as u32 as u64;
            // 2fe22 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2fe24 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2fe29 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2fe2c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2fe2f mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 2fe36 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2fe39 cmp QWORD PTR [r15+0x8],rax
            fl = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax).1;
            // 2fe3d je 0x2febe
            if crate::cov::brk(0x2fe3d, fl.e()) { pc = 0x2febe; continue; }
            pc = 0x2fe3f;
        }
        0x2fe3f => {
            // block 2fe3f
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fe3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fe3f mov QWORD PTR [rbp-0x58],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rbx) as u64);
            // 2fe43 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // 2fe49 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2fe4b lea r13,[rbp-0x2c]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            pc = 0x2fe4f;
        }
        0x2fe4f => {
            // block 2fe4f
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fe4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fe4f mov rbx,QWORD PTR [r14+0x18]
            rbx = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2fe53 mov esi,DWORD PTR [r14+0x2ac]
            rsi = (m.r32(r14.wrapping_add(0x2ac)) as u64) as u32 as u64;
            // 2fe5a movzx edx,BYTE PTR [rax+rdx*1]
            rdx = (m.r8(rax.wrapping_add(rdx)) as u64) as u32 as u64;
            // 2fe5e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2fe63 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2fe66 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2fe6a call QWORD PTR [rbx+0x30]
            { let r = { let t = m.r64(rbx.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2fe6d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2fe70 mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 2fe74 mov rdx,rcx
            rdx = rcx;
            // 2fe77 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 2fe7a cmp rdx,0x2
            fl = sub::<64>(rdx, 0x2u64).1;
            // 2fe7e jb 0x2fea4
            if crate::cov::brk(0x2fe7e, fl.b()) { pc = 0x2fea4; continue; }
            pc = 0x2fe80;
        }
        0x2fe80 => {
            // block 2fe80
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fe80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fe80 mov rax,QWORD PTR [r14+0x18]
            rax = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2fe84 mov esi,DWORD PTR [r14+0x2b0]
            rsi = (m.r32(r14.wrapping_add(0x2b0)) as u64) as u32 as u64;
            // 2fe8b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2fe90 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2fe93 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2fe97 mov rdx,r13
            rdx = r13;
            // 2fe9a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2fe9d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2fea0 mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x2fea4;
        }
        0x2fea4 => {
            // block 2fea4
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fea4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fea4 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 2fea7 mov rsi,rcx
            rsi = rcx;
            // 2feaa sub rsi,rax
            { let (r, f) = sub::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // 2fead inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // 2feb0 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 2feb3 jb 0x2fe4f
            if crate::cov::brk(0x2feb3, fl.b()) { pc = 0x2fe4f; continue; }
            pc = 0x2feb5;
        }
        0x2feb5 => {
            // block 2feb5
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2feb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2feb5 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 2feb8 mov rbx,QWORD PTR [rbp-0x58]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 2febc jne 0x2fec9
            if crate::cov::brk(0x2febc, fl.ne()) { pc = 0x2fec9; continue; }
            pc = 0x2febe;
        }
        0x2febe => {
            // block 2febe
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2febe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2febe mov rdi,r15
            rdi = r15;
            // 2fec1 call 0xebfba
            { let r = { let t = m.env.stub_va(0x787); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2fec6 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            pc = 0x2fec9;
        }
        0x2fec9 => {
            // block 2fec9
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fec9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fec9 movzx ecx,BYTE PTR [rax]
            rcx = (m.r8(rax) as u64) as u32 as u64;
            // 2fecc cmp ecx,0x21
            fl = sub::<32>((rcx as u32 as u64), 0x21u64).1;
            // 2fecf je 0x30010
            if crate::cov::brk(0x2fecf, fl.e()) { pc = 0x30010; continue; }
            pc = 0x2fed5;
        }
        0x2fed5 => {
            // block 2fed5
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fed5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fed5 cmp QWORD PTR [r15+0x8],rax
            fl = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax).1;
            // 2fed9 jne 0x2fee6
            if crate::cov::brk(0x2fed9, fl.ne()) { pc = 0x2fee6; continue; }
            pc = 0x2fedb;
        }
        0x2fedb => {
            // block 2fedb
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fedb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fedb mov rdi,r15
            rdi = r15;
            // 2fede call 0xebfba
            { let r = { let t = m.env.stub_va(0x787); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2fee3 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            pc = 0x2fee6;
        }
        0x2fee6 => {
            // block 2fee6
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fee6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fee6 movzx eax,BYTE PTR [rax]
            rax = (m.r8(rax) as u64) as u32 as u64;
            // 2fee9 cmp eax,0xdc
            fl = sub::<32>((rax as u32 as u64), 0xdcu64).1;
            // 2feee je 0x30010
            if crate::cov::brk(0x2feee, fl.e()) { pc = 0x30010; continue; }
            pc = 0x2fef4;
        }
        0x2fef4 => {
            // block 2fef4
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fef4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fef4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2fef7 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2fefb cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 2fefe je 0x2ff88
            if crate::cov::brk(0x2fefe, fl.e()) { pc = 0x2ff88; continue; }
            pc = 0x2ff04;
        }
        0x2ff04 => {
            // block 2ff04
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff04 mov esi,DWORD PTR [rbp-0x48]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 2ff07 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 2ff09 mov rdi,rcx
            rdi = rcx;
            // 2ff0c sub rdi,rax
            { let (r, f) = sub::<64>(rdi, rax);
              rdi = r;
              fl = f; }
            // 2ff0f cmp rdx,rdi
            fl = sub::<64>(rdx, rdi).1;
            // 2ff12 cmovbe edi,esi
            if fl.be() { rdi = ((rsi as u32 as u64)) as u32 as u64; } else { rdi = rdi as u32 as u64; }
            // 2ff15 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // 2ff19 test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // 2ff1b je 0x30010
            if crate::cov::brk(0x2ff1b, fl.e()) { pc = 0x30010; continue; }
            pc = 0x2ff21;
        }
        0x2ff21 => {
            // block 2ff21
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff21 mov rdx,QWORD PTR [rbp-0x48]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2ff25 dec edx
            { let (r, f) = dec::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // 2ff27 mov QWORD PTR [rbp-0x48],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdx) as u64);
            // 2ff2b xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 2ff2e jmp 0x2ff3a
            pc = 0x2ff3a; continue;
        }
        0x2ff30 => {
            // block 2ff30
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff30 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // 2ff33 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2ff36 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0x2ff3a;
        }
        0x2ff3a => {
            // block 2ff3a
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff3a mov rdx,QWORD PTR [r14+0x18]
            rdx = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2ff3e mov r13,QWORD PTR [rdx+0x30]
            r13 = m.r64(rdx.wrapping_add(0x30)) as u64;
            // 2ff42 mov r15d,DWORD PTR [r14+0x2ac]
            r15 = (m.r32(r14.wrapping_add(0x2ac)) as u64) as u32 as u64;
            // 2ff49 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 2ff4c cmp rcx,r12
            fl = sub::<64>(rcx, r12).1;
            // 2ff4f ja 0x2ff5c
            if crate::cov::brk(0x2ff4f, fl.a()) { pc = 0x2ff5c; continue; }
            pc = 0x2ff51;
        }
        0x2ff51 => {
            // block 2ff51
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff51 mov rdi,rbx
            rdi = rbx;
            // 2ff54 call 0xebfba
            { let r = { let t = m.env.stub_va(0x787); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2ff59 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            pc = 0x2ff5c;
        }
        0x2ff5c => {
            // block 2ff5c
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff5c movzx edx,BYTE PTR [rax+r12*1]
            rdx = (m.r8(rax.wrapping_add(r12)) as u64) as u32 as u64;
            // 2ff61 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2ff66 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2ff69 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2ff6d mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2ff70 call r13
            { let r = { let t = r13; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2ff73 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2ff77 cmp eax,r12d
            fl = sub::<32>((rax as u32 as u64), (r12 as u32 as u64)).1;
            // 2ff7a jne 0x2ff30
            if crate::cov::brk(0x2ff7a, fl.ne()) { pc = 0x2ff30; continue; }
            pc = 0x2ff7c;
        }
        0x2ff7c => {
            // block 2ff7c
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff7c jmp 0x30010
            pc = 0x30010; continue;
        }
        0x2ff81 => {
            // block 2ff81
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff81 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2ff83 jmp 0x30031
            pc = 0x30031; continue;
        }
        0x2ff88 => {
            // block 2ff88
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff88 mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // 2ff8f cmp DWORD PTR [rbp-0x48],0xffffffff
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, 0xffffffffu64).1;
            // 2ff93 je 0x30010
            if crate::cov::brk(0x2ff93, fl.e()) { pc = 0x30010; continue; }
            pc = 0x2ff95;
        }
        0x2ff95 => {
            // block 2ff95
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff95 not DWORD PTR [rbp-0x48]
            { let v = !m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64; m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (v) as u32); }
            // 2ff98 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 2ff9b lea r15,[rbp-0x30]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2ff9f lea r12,[rbp-0x31]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffcf);
            pc = 0x2ffa3;
        }
        0x2ffa3 => {
            // block 2ffa3
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ffa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ffa3 mov rax,QWORD PTR [r14+0x18]
            rax = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2ffa7 mov esi,DWORD PTR [r14+0x2ac]
            rsi = (m.r32(r14.wrapping_add(0x2ac)) as u64) as u32 as u64;
            // 2ffae xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2ffb0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2ffb5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2ffb8 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2ffbc mov rdi,rbx
            rdi = rbx;
            // 2ffbf call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2ffc2 mov rax,QWORD PTR [r14+0x18]
            rax = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2ffc6 mov esi,DWORD PTR [r14+0x2b0]
            rsi = (m.r32(r14.wrapping_add(0x2b0)) as u64) as u32 as u64;
            // 2ffcd mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2ffd2 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2ffd5 mov rdi,rbx
            rdi = rbx;
            // 2ffd8 mov rdx,r15
            rdx = r15;
            // 2ffdb call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2ffde test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 2ffe1 je 0x30007
            if crate::cov::brk(0x2ffe1, fl.e()) { pc = 0x30007; continue; }
            pc = 0x2ffe3;
        }
        0x2ffe3 => {
            // block 2ffe3
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ffe3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ffe3 mov al,BYTE PTR [rbp-0x30]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u8 as u64);
            // 2ffe6 mov BYTE PTR [rbp-0x31],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf), ((rax as u8 as u64)) as u8);
            // 2ffe9 mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 2ffed mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 2fff1 cmp rcx,QWORD PTR [rdi+0x10]
            fl = sub::<64>(rcx, m.r64(rdi.wrapping_add(0x10)) as u64).1;
            // 2fff5 jae 0x2ffff
            if crate::cov::brk(0x2fff5, fl.ae()) { pc = 0x2ffff; continue; }
            pc = 0x2fff7;
        }
        0x2fff7 => {
            // block 2fff7
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fff7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fff7 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // 2fff9 inc QWORD PTR [rdi+0x8]
            { let (r, f) = inc::<64>(m.r64(rdi.wrapping_add(0x8)) as u64, fl); m.w64(rdi.wrapping_add(0x8), (r) as u64); fl = f; }
            // 2fffd jmp 0x30007
            pc = 0x30007; continue;
        }
        0x2ffff => {
            // block 2ffff
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ffff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ffff mov rsi,r12
            rsi = r12;
            // 30002 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x30007;
        }
        0x30007 => {
            // block 30007
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 30007 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30007 dec r13d
            { let (r, f) = dec::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // 3000a cmp DWORD PTR [rbp-0x48],r13d
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, (r13 as u32 as u64)).1;
            // 3000e jne 0x2ffa3
            if crate::cov::brk(0x3000e, fl.ne()) { pc = 0x2ffa3; continue; }
            pc = 0x30010;
        }
        0x30010 => {
            // block 30010
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 30010 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30010 mov rax,QWORD PTR [r14+0x18]
            rax = m.r64(r14.wrapping_add(0x18)) as u64;
            // 30014 mov esi,DWORD PTR [r14+0x2b4]
            rsi = (m.r32(r14.wrapping_add(0x2b4)) as u64) as u32 as u64;
            // 3001b mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 30020 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 30025 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 30028 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 3002c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3002f mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x30031;
        }
        0x30031 => {
            // block 30031
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 30031 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30031 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 30035 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 30036 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 30038 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 3003a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3003c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 3003e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3003f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4690 size=630
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4690<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd4690;
    loop {
        match pc {
        0xd4690 => {
            // block d4690
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4690 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4690 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4691 mov rbp,rsp
            rbp = rsp;
            // d4694 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d4696 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d4698 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d469a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d469b sub rsp,0x30
            { let (r, f) = sub::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // d469f mov r14,rdx
            r14 = rdx;
            // d46a2 mov rbx,rdi
            rbx = rdi;
            // d46a5 cmp BYTE PTR [rbx+0xa],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0xa)) as u64, 0x0u64).1;
            // d46a9 je 0xd4715
            if crate::cov::brk(0xd46a9, fl.e()) { pc = 0xd4715; continue; }
            pc = 0xd46ab;
        }
        0xd46ab => {
            // block d46ab
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d46ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d46ab mov DWORD PTR [rbp-0x24],0x12
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x12u64) as u32);
            // d46b2 mov DWORD PTR [rbp-0x28],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x2u64) as u32);
            // d46b9 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // d46bd mov rdi,rbx
            rdi = rbx;
            // d46c0 call 0xd40e0
            { let r = { let t = m.env.code_va(0xd40e0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d46c5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d46c7 je 0xd4715
            if crate::cov::brk(0xd46c7, fl.e()) { pc = 0xd4715; continue; }
            pc = 0xd46c9;
        }
        0xd46c9 => {
            // block d46c9
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d46c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d46c9 mov r12d,DWORD PTR [rbp-0x24]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // d46cd xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d46d0 cmp r12d,0x11
            fl = sub::<32>((r12 as u32 as u64), 0x11u64).1;
            // d46d4 ja 0xd4718
            if crate::cov::brk(0xd46d4, fl.a()) { pc = 0xd4718; continue; }
            pc = 0xd46d6;
        }
        0xd46d6 => {
            // block d46d6
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d46d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d46d6 mov eax,0x3ffcf
            rax = (0x3ffcfu64) as u32 as u64;
            // d46db bt eax,r12d
            fl.cf = bit::<32>((rax as u32 as u64), (r12 as u32 as u64));
            // d46df jae 0xd4718
            if crate::cov::brk(0xd46df, fl.ae()) { pc = 0xd4718; continue; }
            pc = 0xd46e1;
        }
        0xd46e1 => {
            // block d46e1
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d46e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d46e1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d46e4 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // d46e8 mov rdi,rbx
            rdi = rbx;
            // d46eb call QWORD PTR [rax+0x338]
            { let r = { let t = m.r64(rax.wrapping_add(0x338)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d46f1 cmp r12d,0x11
            fl = sub::<32>((r12 as u32 as u64), 0x11u64).1;
            // d46f5 ja 0xd4728
            if crate::cov::brk(0xd46f5, fl.a()) { pc = 0xd4728; continue; }
            pc = 0xd46f7;
        }
        0xd46f7 => {
            // block d46f7
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d46f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d46f7 mov eax,0x3c30c
            rax = (0x3c30cu64) as u32 as u64;
            // d46fc bt eax,r12d
            fl.cf = bit::<32>((rax as u32 as u64), (r12 as u32 as u64));
            // d4700 jae 0xd4728
            if crate::cov::brk(0xd4700, fl.ae()) { pc = 0xd4728; continue; }
            pc = 0xd4702;
        }
        0xd4702 => {
            // block d4702
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4702 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4702 lea rdi,[r14+0x16c]
            rdi = r14.wrapping_add(0x16c);
            // d4709 mov esi,0xff
            rsi = (0xffu64) as u32 as u64;
            // d470e call 0x28d56
            { let r = { let t = m.env.code_va(0x28d56); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d4713 jmp 0xd478b
            pc = 0xd478b; continue;
        }
        0xd4715 => {
            // block d4715
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4715 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4715 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd4718;
        }
        0xd4718 => {
            // block d4718
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4718 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4718 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // d471b add rsp,0x30
            { let (r, f) = add::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // d471f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4720 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d4722 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d4724 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d4726 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4727 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd4728 => {
            // block d4728
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4728 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4728 cmp r12d,0xd
            fl = sub::<32>((r12 as u32 as u64), 0xdu64).1;
            // d472c ja 0xd478b
            if crate::cov::brk(0xd472c, fl.a()) { pc = 0xd478b; continue; }
            pc = 0xd472e;
        }
        0xd472e => {
            // block d472e
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d472e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d472e mov eax,0x3cc3
            rax = (0x3cc3u64) as u32 as u64;
            // d4733 bt eax,r12d
            fl.cf = bit::<32>((rax as u32 as u64), (r12 as u32 as u64));
            // d4737 jae 0xd478b
            if crate::cov::brk(0xd4737, fl.ae()) { pc = 0xd478b; continue; }
            pc = 0xd4739;
        }
        0xd4739 => {
            // block d4739
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4739 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4739 cmp DWORD PTR [r14+0x2a4],0xffffffff
            fl = sub::<32>(m.r32(r14.wrapping_add(0x2a4)) as u64, 0xffffffffu64).1;
            // d4741 jne 0xd4757
            if crate::cov::brk(0xd4741, fl.ne()) { pc = 0xd4757; continue; }
            pc = 0xd4743;
        }
        0xd4743 => {
            // block d4743
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4743 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4743 cmp DWORD PTR [r14+0x2a8],0xffffffff
            fl = sub::<32>(m.r32(r14.wrapping_add(0x2a8)) as u64, 0xffffffffu64).1;
            // d474b jne 0xd4757
            if crate::cov::brk(0xd474b, fl.ne()) { pc = 0xd4757; continue; }
            pc = 0xd474d;
        }
        0xd474d => {
            // block d474d
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d474d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d474d cmp DWORD PTR [r14+0x2ac],0xffffffff
            fl = sub::<32>(m.r32(r14.wrapping_add(0x2ac)) as u64, 0xffffffffu64).1;
            // d4755 je 0xd4766
            if crate::cov::brk(0xd4755, fl.e()) { pc = 0xd4766; continue; }
            pc = 0xd4757;
        }
        0xd4757 => {
            // block d4757
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4757 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4757 lea rsi,[r14+0x2a4]
            rsi = r14.wrapping_add(0x2a4);
            // d475e mov rdi,r14
            rdi = r14;
            // d4761 call 0x2b652
            { let r = { let t = m.env.code_va(0x2b652); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xd4766;
        }
        0xd4766 => {
            // block d4766
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4766 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4766 mov rsi,QWORD PTR [r14+0xc0]
            rsi = m.r64(r14.wrapping_add(0xc0)) as u64;
            // d476d test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // d4770 je 0xd478b
            if crate::cov::brk(0xd4770, fl.e()) { pc = 0xd478b; continue; }
            pc = 0xd4772;
        }
        0xd4772 => {
            // block d4772
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4772 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4772 cmp DWORD PTR [rsi],0xffffffff
            fl = sub::<32>(m.r32(rsi) as u64, 0xffffffffu64).1;
            // d4775 jne 0xd4783
            if crate::cov::brk(0xd4775, fl.ne()) { pc = 0xd4783; continue; }
            pc = 0xd4777;
        }
        0xd4777 => {
            // block d4777
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4777 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4777 cmp DWORD PTR [rsi+0x4],0xffffffff
            fl = sub::<32>(m.r32(rsi.wrapping_add(0x4)) as u64, 0xffffffffu64).1;
            // d477b jne 0xd4783
            if crate::cov::brk(0xd477b, fl.ne()) { pc = 0xd4783; continue; }
            pc = 0xd477d;
        }
        0xd477d => {
            // block d477d
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d477d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d477d cmp DWORD PTR [rsi+0x8],0xffffffff
            fl = sub::<32>(m.r32(rsi.wrapping_add(0x8)) as u64, 0xffffffffu64).1;
            // d4781 je 0xd478b
            if crate::cov::brk(0xd4781, fl.e()) { pc = 0xd478b; continue; }
            pc = 0xd4783;
        }
        0xd4783 => {
            // block d4783
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4783 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4783 mov rdi,r14
            rdi = r14;
            // d4786 call 0x2b652
            { let r = { let t = m.env.code_va(0x2b652); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xd478b;
        }
        0xd478b => {
            // block d478b
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d478b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d478b mov DWORD PTR [r14+0x2b0],r12d
            m.w32(r14.wrapping_add(0x2b0), ((r12 as u32 as u64)) as u32);
            // d4792 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d4795 mov rdi,rbx
            rdi = rbx;
            // d4798 mov rsi,r14
            rsi = r14;
            // d479b call QWORD PTR [rax+0xd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xd0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d47a1 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d47a4 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // d47a7 je 0xd4718
            if crate::cov::brk(0xd47a7, fl.e()) { pc = 0xd4718; continue; }
            pc = 0xd47ad;
        }
        0xd47ad => {
            // block d47ad
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d47ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d47ad cmp DWORD PTR [rbp-0x28],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, 0x1u64).1;
            // d47b1 jne 0xd4718
            if crate::cov::brk(0xd47b1, fl.ne()) { pc = 0xd4718; continue; }
            pc = 0xd47b7;
        }
        0xd47b7 => {
            // block d47b7
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d47b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d47b7 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // d47ba cmp r12d,0x11
            fl = sub::<32>((r12 as u32 as u64), 0x11u64).1;
            // d47be ja 0xd4718
            if crate::cov::brk(0xd47be, fl.a()) { pc = 0xd4718; continue; }
            pc = 0xd47c4;
        }
        0xd47c4 => {
            // block d47c4
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d47c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d47c4 mov eax,0x3c30c
            rax = (0x3c30cu64) as u32 as u64;
            // d47c9 bt eax,r12d
            fl.cf = bit::<32>((rax as u32 as u64), (r12 as u32 as u64));
            // d47cd jae 0xd4718
            if crate::cov::brk(0xd47cd, fl.ae()) { pc = 0xd4718; continue; }
            pc = 0xd47d3;
        }
        0xd47d3 => {
            // block d47d3
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d47d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d47d3 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // d47da mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d47dd lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // d47e1 mov esi,0x2724
            rsi = (0x2724u64) as u32 as u64;
            // d47e6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d47eb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d47ee mov rdi,rbx
            rdi = rbx;
            // d47f1 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d47f4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d47f7 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d47fb cmp DWORD PTR [rbp-0x2c],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64, 0x1u64).1;
            // d47ff jne 0xd482d
            if crate::cov::brk(0xd47ff, fl.ne()) { pc = 0xd482d; continue; }
            pc = 0xd4801;
        }
        0xd4801 => {
            // block d4801
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4801 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4801 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d4805 mov esi,0x27b5
            rsi = (0x27b5u64) as u32 as u64;
            // d480a mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d480f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d4812 mov rdi,rbx
            rdi = rbx;
            // d4815 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d4817 mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // d481a cmp eax,0xfe
            fl = sub::<32>((rax as u32 as u64), 0xfeu64).1;
            // d481f je 0xd4859
            if crate::cov::brk(0xd481f, fl.e()) { pc = 0xd4859; continue; }
            pc = 0xd4821;
        }
        0xd4821 => {
            // block d4821
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4821 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4821 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d4824 jne 0xd4860
            if crate::cov::brk(0xd4824, fl.ne()) { pc = 0xd4860; continue; }
            pc = 0xd4826;
        }
        0xd4826 => {
            // block d4826
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4826 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4826 mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // d482b jmp 0xd486f
            pc = 0xd486f; continue;
        }
        0xd482d => {
            // block d482d
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d482d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d482d lea rdx,[rbp-0x44]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // d4831 mov esi,0x27b5
            rsi = (0x27b5u64) as u32 as u64;
            // d4836 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d483b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d483e mov rdi,rbx
            rdi = rbx;
            // d4841 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d4843 mov eax,DWORD PTR [rbp-0x44]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64) as u32 as u64;
            // d4846 cmp eax,0xfe
            fl = sub::<32>((rax as u32 as u64), 0xfeu64).1;
            // d484b je 0xd48ac
            if crate::cov::brk(0xd484b, fl.e()) { pc = 0xd48ac; continue; }
            pc = 0xd484d;
        }
        0xd484d => {
            // block d484d
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d484d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d484d cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d4850 jne 0xd48b3
            if crate::cov::brk(0xd4850, fl.ne()) { pc = 0xd48b3; continue; }
            pc = 0xd4852;
        }
        0xd4852 => {
            // block d4852
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4852 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4852 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // d4857 jmp 0xd48c2
            pc = 0xd48c2; continue;
        }
        0xd4859 => {
            // block d4859
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4859 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4859 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // d485e jmp 0xd486f
            pc = 0xd486f; continue;
        }
        0xd4860 => {
            // block d4860
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4860 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4860 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d4865 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d4868 jne 0xd486f
            if crate::cov::brk(0xd4868, fl.ne()) { pc = 0xd486f; continue; }
            pc = 0xd486a;
        }
        0xd486a => {
            // block d486a
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d486a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d486a mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            pc = 0xd486f;
        }
        0xd486f => {
            // block d486f
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d486f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d486f mov DWORD PTR [rbp-0x40],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0xffffffffu64) as u32);
            // d4876 mov DWORD PTR [rbp-0x3c],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), (0xffffffffu64) as u32);
            // d487d mov DWORD PTR [rbp-0x38],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0xffffffffu64) as u32);
            // d4884 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d4888 mov rdi,r14
            rdi = r14;
            // d488b call 0x2b77c
            { let r = { let t = m.env.code_va(0x2b77c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d4890 mov rax,QWORD PTR [r14+0x16c]
            rax = m.r64(r14.wrapping_add(0x16c)) as u64;
            // d4897 test rax,rax
            fl = logic::<64>(rax & rax);
            // d489a je 0xd4718
            if crate::cov::brk(0xd489a, fl.e()) { pc = 0xd4718; continue; }
            pc = 0xd48a0;
        }
        0xd48a0 => {
            // block d48a0
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48a0 mov ecx,DWORD PTR [rbp-0x38]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // d48a3 mov DWORD PTR [rax+0x8],ecx
            m.w32(rax.wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // d48a6 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d48aa jmp 0xd48fd
            pc = 0xd48fd; continue;
        }
        0xd48ac => {
            // block d48ac
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48ac mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // d48b1 jmp 0xd48c2
            pc = 0xd48c2; continue;
        }
        0xd48b3 => {
            // block d48b3
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48b3 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // d48b8 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d48bb jne 0xd48c2
            if crate::cov::brk(0xd48bb, fl.ne()) { pc = 0xd48c2; continue; }
            pc = 0xd48bd;
        }
        0xd48bd => {
            // block d48bd
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48bd mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            pc = 0xd48c2;
        }
        0xd48c2 => {
            // block d48c2
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48c2 mov DWORD PTR [rbp-0x50],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0xffffffffu64) as u32);
            // d48c9 mov DWORD PTR [rbp-0x4c],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), (0xffffffffu64) as u32);
            // d48d0 mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // d48d7 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // d48db mov rdi,r14
            rdi = r14;
            // d48de call 0x2b77c
            { let r = { let t = m.env.code_va(0x2b77c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d48e3 mov rax,QWORD PTR [r14+0x16c]
            rax = m.r64(r14.wrapping_add(0x16c)) as u64;
            // d48ea test rax,rax
            fl = logic::<64>(rax & rax);
            // d48ed je 0xd4718
            if crate::cov::brk(0xd48ed, fl.e()) { pc = 0xd4718; continue; }
            pc = 0xd48f3;
        }
        0xd48f3 => {
            // block d48f3
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48f3 mov ecx,DWORD PTR [rbp-0x48]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // d48f6 mov DWORD PTR [rax+0x8],ecx
            m.w32(rax.wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // d48f9 mov rcx,QWORD PTR [rbp-0x50]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0xd48fd;
        }
        0xd48fd => {
            // block d48fd
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48fd mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // d4900 jmp 0xd4718
            pc = 0xd4718; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2466 size=642
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2466<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2466;
    loop {
        match pc {
        0xe2466 => {
            // block e2466
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2466 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2467 mov rbp,rsp
            rbp = rsp;
            // e246a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e246c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e246e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2470 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2471 sub rsp,0xa0
            { let (r, f) = sub::<64>(rsp, 0xa0u64);
              rsp = r;
              fl = f; }
            // e2478 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // e247b mov rbx,rsi
            rbx = rsi;
            // e247e mov r14,rdi
            r14 = rdi;
            // e2481 call 0xe49c4
            { let r = { let t = m.env.code_va(0xe49c4); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2486 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // e2489 test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e248b je 0xe2648
            if crate::cov::brk(0xe248b, fl.e()) { pc = 0xe2648; continue; }
            pc = 0xe2491;
        }
        0xe2491 => {
            // block e2491
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2491 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2491 lea rdi,[r14+0x34]
            rdi = r14.wrapping_add(0x34);
            // e2495 test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // e2498 jne 0xe249f
            if crate::cov::brk(0xe2498, fl.ne()) { pc = 0xe249f; continue; }
            pc = 0xe249a;
        }
        0xe249a => {
            // block e249a
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e249a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e249a inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e249d jmp 0xe24a3
            pc = 0xe24a3; continue;
        }
        0xe249f => {
            // block e249f
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e249f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e249f mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0xe24a3;
        }
        0xe24a3 => {
            // block e24a3
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e24a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e24a3 lea rsi,[rip+0x56fe0] # 0x13948a
            rsi = m.env.data_va(318551);
            // e24aa xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e24ad mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e24b2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e24b4 mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // e24b7 mov rcx,rbx
            rcx = rbx;
            // e24ba call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e24bf mov esi,0xfa
            rsi = (0xfau64) as u32 as u64;
            // e24c4 mov rdi,r14
            rdi = r14;
            // e24c7 call 0xe435c
            { let r = { let t = m.env.code_va(0xe435c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e24cc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e24ce je 0xe2648
            if crate::cov::brk(0xe24ce, fl.e()) { pc = 0xe2648; continue; }
            pc = 0xe24d4;
        }
        0xe24d4 => {
            // block e24d4
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e24d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e24d4 xorps xmm0,xmm0
            x0 = 0;
            // e24d7 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // e24db mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // e24e3 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e24e7 mov rdi,r14
            rdi = r14;
            // e24ea call 0xe4a2e
            { let r = { let t = m.env.code_va(0xe4a2e); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e24ef xorps xmm0,xmm0
            x0 = 0;
            // e24f2 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // e24f6 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // e24fe lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e2502 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e2506 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e250b lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e250f lea rdx,[rbp-0x60]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e2513 call 0xe171e
            { let r = { let t = m.env.code_va(0xe171e); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2518 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e251c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2521 mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e2525 cmp QWORD PTR [rbp-0x58],rbx
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64, rbx).1;
            // e2529 je 0xe262b
            if crate::cov::brk(0xe2529, fl.e()) { pc = 0xe262b; continue; }
            pc = 0xe252f;
        }
        0xe252f => {
            // block e252f
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e252f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e252f xorps xmm0,xmm0
            x0 = 0;
            // e2532 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // e2539 mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // e2541 lea rsi,[rip+0x56ecb] # 0x139413
            rsi = m.env.data_va(318432);
            // e2548 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e254f mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // e2554 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2559 lea rdx,[rbp-0xa8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e2560 lea rcx,[rbp-0x90]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e2567 mov rsi,rbx
            rsi = rbx;
            // e256a call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e256f mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e2571 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e2578 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e257d test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e257f je 0xe261f
            if crate::cov::brk(0xe257f, fl.e()) { pc = 0xe261f; continue; }
            pc = 0xe2585;
        }
        0xe2585 => {
            // block e2585
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2585 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2585 lea rsi,[rip+0x56e8e] # 0x13941a
            rsi = m.env.data_va(318439);
            // e258c lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e2593 call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2598 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e259a je 0xe265b
            if crate::cov::brk(0xe259a, fl.e()) { pc = 0xe265b; continue; }
            pc = 0xe25a0;
        }
        0xe25a0 => {
            // block e25a0
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e25a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e25a0 lea rsi,[rip+0x56e76] # 0x13941d
            rsi = m.env.data_va(318442);
            // e25a7 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e25ae call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e25b3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e25b5 jne 0xe261f
            if crate::cov::brk(0xe25b5, fl.ne()) { pc = 0xe261f; continue; }
            pc = 0xe25b7;
        }
        0xe25b7 => {
            // block e25b7
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e25b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e25b7 mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e25bb mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e25bf sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // e25c2 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e25c6 movabs rcx,0xaaaaaaaaaaaaaaab
            rcx = 0xaaaa_aaaa_aaaa_aaabu64;
            // e25d0 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // e25d4 cmp rcx,0x4
            fl = sub::<64>(rcx, 0x4u64).1;
            // e25d8 jb 0xe261f
            if crate::cov::brk(0xe25d8, fl.b()) { pc = 0xe261f; continue; }
            pc = 0xe25da;
        }
        0xe25da => {
            // block e25da
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e25da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e25da lea rsi,[rip+0x56e41] # 0x139422
            rsi = m.env.data_va(318447);
            // e25e1 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e25e8 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // e25ed call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e25f2 add rbx,0x30
            { let (r, f) = add::<64>(rbx, 0x30u64);
              rbx = r;
              fl = f; }
            // e25f6 lea rcx,[r14+0x10]
            rcx = r14.wrapping_add(0x10);
            // e25fa lea rdx,[rbp-0xc0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e2601 mov rsi,rbx
            rsi = rbx;
            // e2604 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2609 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e2610 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2615 mov DWORD PTR [r14+0x8],0x1a
            m.w32(r14.wrapping_add(0x8), (0x1au64) as u32);
            // e261d jmp 0xe2674
            pc = 0xe2674; continue;
        }
        0xe261f => {
            // block e261f
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e261f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e261f lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e2626 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xe262b;
        }
        0xe262b => {
            // block e262b
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e262b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e262b mov DWORD PTR [r14+0x8],0x17
            m.w32(r14.wrapping_add(0x8), (0x17u64) as u32);
            // e2633 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe2636;
        }
        0xe2636 => {
            // block e2636
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2636 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2636 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e263a call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e263f lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e2643 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xe2648;
        }
        0xe2648 => {
            // block e2648
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2648 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2648 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e264b add rsp,0xa0
            { let (r, f) = add::<64>(rsp, 0xa0u64);
              rsp = r;
              fl = f; }
            // e2652 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2653 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2655 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2657 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2659 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e265a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe265b => {
            // block e265b
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e265b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e265b mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e265f sub rax,QWORD PTR [rbp-0x60]
            { let (r, f) = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64);
              rax = r;
              fl = f; }
            // e2663 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // e2666 cmp rax,0x30
            fl = sub::<64>(rax, 0x30u64).1;
            // e266a je 0xe2677
            if crate::cov::brk(0xe266a, fl.e()) { pc = 0xe2677; continue; }
            pc = 0xe266c;
        }
        0xe266c => {
            // block e266c
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e266c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e266c mov DWORD PTR [r14+0x8],0x18
            m.w32(r14.wrapping_add(0x8), (0x18u64) as u32);
            pc = 0xe2674;
        }
        0xe2674 => {
            // block e2674
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2674 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2674 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe2677;
        }
        0xe2677 => {
            // block e2677
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2677 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2677 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e267e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2683 jmp 0xe2636
            pc = 0xe2636; continue;
        }
        0xe2685 => {
            // block e2685
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2685 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2685 mov rbx,rax
            rbx = rax;
            // e2688 jmp 0xe26c6
            pc = 0xe26c6; continue;
        }
        0xe268a => {
            // block e268a
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e268a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e268a mov rbx,rax
            rbx = rax;
            // e268d jmp 0xe26bd
            pc = 0xe26bd; continue;
        }
        0xe268f => {
            // block e268f
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e268f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e268f mov rbx,rax
            rbx = rax;
            // e2692 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e2696 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e269b jmp 0xe26bd
            pc = 0xe26bd; continue;
        }
        0xe269d => {
            // block e269d
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e269d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e269d mov rbx,rax
            rbx = rax;
            // e26a0 jmp 0xe26b1
            pc = 0xe26b1; continue;
        }
        0xe26a2 => {
            // block e26a2
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e26a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e26a2 mov rbx,rax
            rbx = rax;
            // e26a5 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e26ac call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xe26b1;
        }
        0xe26b1 => {
            // block e26b1
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e26b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e26b1 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e26b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xe26bd;
        }
        0xe26bd => {
            // block e26bd
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e26bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e26bd lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e26c1 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xe26c6;
        }
        0xe26c6 => {
            // block e26c6
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e26c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e26c6 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e26ca call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e26cf mov rdi,rbx
            rdi = rbx;
            // e26d2 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e26d7 mov rbx,rax
            rbx = rax;
            // e26da lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e26e1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e26e6 jmp 0xe26b1
            pc = 0xe26b1; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd69d6 size=672
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d69d6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd69d6;
    loop {
        match pc {
        0xd69d6 => {
            // block d69d6
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d69d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d69d6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d69d7 mov rbp,rsp
            rbp = rsp;
            // d69da push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d69dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d69dd mov rbx,rdi
            rbx = rdi;
            // d69e0 call 0xd6ec8
            { let r = { let t = m.env.code_va(0xd6ec8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d69e5 mov DWORD PTR [rbx+0x330],eax
            m.w32(rbx.wrapping_add(0x330), ((rax as u32 as u64)) as u32);
            // d69eb mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d69ee call 0xcdaff
            { let r = { let t = m.env.code_va(0xcdaff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d69f3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d69f5 je 0xd6a3d
            if crate::cov::brk(0xd69f5, fl.e()) { pc = 0xd6a3d; continue; }
            pc = 0xd69f7;
        }
        0xd69f7 => {
            // block d69f7
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d69f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d69f7 movabs rax,0x100000001000000
            rax = 0x100_0000_0100_0000u64;
            // d6a01 mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6a08 mov DWORD PTR [rbx+0x35c],0x80
            m.w32(rbx.wrapping_add(0x35c), (0x80u64) as u32);
            // d6a12 mov DWORD PTR [rbx+0x368],0x7f
            m.w32(rbx.wrapping_add(0x368), (0x7fu64) as u32);
            // d6a1c mov QWORD PTR [rbx+0x33c],0x10000
            m.w64(rbx.wrapping_add(0x33c), (0x10000u64) as u64);
            // d6a27 movabs rax,0xfe000000800000
            rax = 0xfe_0000_0080_0000u64;
            pc = 0xd6a31;
        }
        0xd6a31 => {
            // block d6a31
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a31 mov QWORD PTR [rbx+0x344],rax
            m.w64(rbx.wrapping_add(0x344), (rax) as u64);
            // d6a38 jmp 0xd6bad
            pc = 0xd6bad; continue;
        }
        0xd6a3d => {
            // block d6a3d
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a3d mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d6a40 call 0xcdb57
            { let r = { let t = m.env.code_va(0xcdb57); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6a45 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6a47 je 0xd6a80
            if crate::cov::brk(0xd6a47, fl.e()) { pc = 0xd6a80; continue; }
            pc = 0xd6a49;
        }
        0xd6a49 => {
            // block d6a49
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a49 cmp DWORD PTR [rbx+0x330],0x10220
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x330)) as u64, 0x10220u64).1;
            // d6a53 jne 0xd6b0f
            if crate::cov::brk(0xd6a53, fl.ne()) { pc = 0xd6b0f; continue; }
            pc = 0xd6a59;
        }
        0xd6a59 => {
            // block d6a59
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a59 movabs rax,0x100000004000000
            rax = 0x100_0000_0400_0000u64;
            // d6a63 mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6a6a mov DWORD PTR [rbx+0x35c],0x40
            m.w32(rbx.wrapping_add(0x35c), (0x40u64) as u32);
            // d6a74 mov DWORD PTR [rbx+0x368],0x3d
            m.w32(rbx.wrapping_add(0x368), (0x3du64) as u32);
            // d6a7e jmp 0xd6ab5
            pc = 0xd6ab5; continue;
        }
        0xd6a80 => {
            // block d6a80
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a80 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d6a83 call 0xcdbbf
            { let r = { let t = m.env.code_va(0xcdbbf); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6a88 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6a8a je 0xd6b63
            if crate::cov::brk(0xd6a8a, fl.e()) { pc = 0xd6b63; continue; }
            pc = 0xd6a90;
        }
        0xd6a90 => {
            // block d6a90
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a90 movabs rax,0x100000004000000
            rax = 0x100_0000_0400_0000u64;
            // d6a9a mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6aa1 mov DWORD PTR [rbx+0x35c],0x40
            m.w32(rbx.wrapping_add(0x35c), (0x40u64) as u32);
            // d6aab mov DWORD PTR [rbx+0x368],0x3c
            m.w32(rbx.wrapping_add(0x368), (0x3cu64) as u32);
            pc = 0xd6ab5;
        }
        0xd6ab5 => {
            // block d6ab5
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6ab5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ab5 mov DWORD PTR [rbx+0x360],0x40
            m.w32(rbx.wrapping_add(0x360), (0x40u64) as u32);
            // d6abf mov DWORD PTR [rbx+0x364],0x40
            m.w32(rbx.wrapping_add(0x364), (0x40u64) as u32);
            // d6ac9 mov QWORD PTR [rbx+0x33c],0x40000
            m.w64(rbx.wrapping_add(0x33c), (0x40000u64) as u64);
            // d6ad4 mov DWORD PTR [rbx+0x354],0x0
            m.w32(rbx.wrapping_add(0x354), (0x0u64) as u32);
            // d6ade mov DWORD PTR [rbx+0x358],0x0
            m.w32(rbx.wrapping_add(0x358), (0x0u64) as u32);
            // d6ae8 movabs rax,0xf8000000000000
            rax = 0xf8_0000_0000_0000u64;
            // d6af2 mov QWORD PTR [rbx+0x344],rax
            m.w64(rbx.wrapping_add(0x344), (rax) as u64);
            // d6af9 movabs rax,0xf0000000f40000
            rax = 0xf0_0000_00f4_0000u64;
            // d6b03 mov QWORD PTR [rbx+0x34c],rax
            m.w64(rbx.wrapping_add(0x34c), (rax) as u64);
            // d6b0a jmp 0xd6bad
            pc = 0xd6bad; continue;
        }
        0xd6b0f => {
            // block d6b0f
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6b0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b0f movabs rax,0x100000002000000
            rax = 0x100_0000_0200_0000u64;
            // d6b19 mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6b20 mov DWORD PTR [rbx+0x35c],0x100
            m.w32(rbx.wrapping_add(0x35c), (0x100u64) as u32);
            // d6b2a mov DWORD PTR [rbx+0x368],0xff
            m.w32(rbx.wrapping_add(0x368), (0xffu64) as u32);
            // d6b34 mov QWORD PTR [rbx+0x33c],0x10000
            m.w64(rbx.wrapping_add(0x33c), (0x10000u64) as u64);
            // d6b3f movabs rax,0xfe000000000000
            rax = 0xfe_0000_0000_0000u64;
            // d6b49 mov QWORD PTR [rbx+0x344],rax
            m.w64(rbx.wrapping_add(0x344), (rax) as u64);
            // d6b50 movabs rax,0xfc000000fd0000
            rax = 0xfc_0000_00fd_0000u64;
            // d6b5a mov QWORD PTR [rbx+0x34c],rax
            m.w64(rbx.wrapping_add(0x34c), (rax) as u64);
            // d6b61 jmp 0xd6bad
            pc = 0xd6bad; continue;
        }
        0xd6b63 => {
            // block d6b63
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6b63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b63 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d6b66 call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6b6b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6b6d je 0xd6c23
            if crate::cov::brk(0xd6b6d, fl.e()) { pc = 0xd6c23; continue; }
            pc = 0xd6b73;
        }
        0xd6b73 => {
            // block d6b73
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6b73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b73 movabs rax,0x100000004000000
            rax = 0x100_0000_0400_0000u64;
            // d6b7d mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6b84 mov DWORD PTR [rbx+0x35c],0x80
            m.w32(rbx.wrapping_add(0x35c), (0x80u64) as u32);
            // d6b8e mov DWORD PTR [rbx+0x368],0x7f
            m.w32(rbx.wrapping_add(0x368), (0x7fu64) as u32);
            // d6b98 mov QWORD PTR [rbx+0x33c],0x40000
            m.w64(rbx.wrapping_add(0x33c), (0x40000u64) as u64);
            // d6ba3 mov DWORD PTR [rbx+0x344],0x0
            m.w32(rbx.wrapping_add(0x344), (0x0u64) as u32);
            pc = 0xd6bad;
        }
        0xd6bad => {
            // block d6bad
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6bad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6bad mov rdi,QWORD PTR [rbx+0x6b8]
            rdi = m.r64(rbx.wrapping_add(0x6b8)) as u64;
            // d6bb4 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d6bb7 je 0xd6bca
            if crate::cov::brk(0xd6bb7, fl.e()) { pc = 0xd6bca; continue; }
            pc = 0xd6bb9;
        }
        0xd6bb9 => {
            // block d6bb9
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6bb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6bb9 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d6bbc call QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6bbf mov QWORD PTR [rbx+0x6b8],0x0
            m.w64(rbx.wrapping_add(0x6b8), (0x0u64) as u64);
            pc = 0xd6bca;
        }
        0xd6bca => {
            // block d6bca
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6bca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6bca mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6bcd mov rdi,rbx
            rdi = rbx;
            // d6bd0 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6bd6 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // d6bd8 call 0x2f83a
            { let r = { let t = m.env.code_va(0x2f83a); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6bdd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6bdf je 0xd6c1c
            if crate::cov::brk(0xd6bdf, fl.e()) { pc = 0xd6c1c; continue; }
            pc = 0xd6be1;
        }
        0xd6be1 => {
            // block d6be1
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6be1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6be1 mov edi,0x2c0
            rdi = (0x2c0u64) as u32 as u64;
            // d6be6 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6beb mov r14,rax
            r14 = rax;
            // d6bee mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6bf1 mov rax,QWORD PTR [rax+0x1a0]
            rax = m.r64(rax.wrapping_add(0x1a0)) as u64;
            // d6bf8 mov rdi,rbx
            rdi = rbx;
            // d6bfb call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6bfd movzx esi,ax
            rsi = ((rax as u16 as u64)) as u32 as u64;
            // d6c00 cmp BYTE PTR [rbx+0x378],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x378)) as u64, 0x0u64).1;
            // d6c07 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d6c0a movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // d6c0d mov rdi,r14
            rdi = r14;
            // d6c10 call 0x2f7d2
            { let r = { let t = m.env.code_va(0x2f7d2); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6c15 mov QWORD PTR [rbx+0x6b8],r14
            m.w64(rbx.wrapping_add(0x6b8), (r14) as u64);
            pc = 0xd6c1c;
        }
        0xd6c1c => {
            // block d6c1c
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6c1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c1c mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d6c1e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6c1f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6c21 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6c22 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd6c23 => {
            // block d6c23
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6c23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c23 movabs rax,0x100000001000000
            rax = 0x100_0000_0100_0000u64;
            // d6c2d mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6c34 mov DWORD PTR [rbx+0x35c],0x40
            m.w32(rbx.wrapping_add(0x35c), (0x40u64) as u32);
            // d6c3e mov DWORD PTR [rbx+0x368],0x3f
            m.w32(rbx.wrapping_add(0x368), (0x3fu64) as u32);
            // d6c48 mov QWORD PTR [rbx+0x33c],0x10000
            m.w64(rbx.wrapping_add(0x33c), (0x10000u64) as u64);
            // d6c53 movabs rax,0xfe000000c00000
            rax = 0xfe_0000_00c0_0000u64;
            // d6c5d jmp 0xd6a31
            pc = 0xd6a31; continue;
        }
        0xd6c62 => {
            // block d6c62
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6c62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c62 mov rbx,rax
            rbx = rax;
            // d6c65 mov rdi,r14
            rdi = r14;
            // d6c68 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6c6d mov rdi,rbx
            rdi = rbx;
            // d6c70 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51f58 size=698
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051f58<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51f58;
    loop {
        match pc {
        0x51f58 => {
            // block 51f58
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 51f58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51f58 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51f59 mov rbp,rsp
            rbp = rsp;
            // 51f5c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 51f5e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 51f60 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 51f62 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 51f64 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 51f65 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 51f69 mov r14,rdx
            r14 = rdx;
            // 51f6c mov rbx,rdi
            rbx = rdi;
            // 51f6f mov DWORD PTR [rbp-0x3c],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), (0xffffffffu64) as u32);
            // 51f76 mov DWORD PTR [rbp-0x40],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u32);
            // 51f7d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 51f80 lea rdx,[rbp-0x3c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            // 51f84 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // 51f89 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51f8e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51f91 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 51f94 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 51f97 lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 51f9b mov esi,0x27b5
            rsi = (0x27b5u64) as u32 as u64;
            // 51fa0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51fa5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51fa8 mov rdi,rbx
            rdi = rbx;
            // 51fab call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 51fae mov eax,DWORD PTR [rbp-0x40]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 51fb1 mov DWORD PTR [rbp-0x54],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((rax as u32 as u64)) as u32);
            // 51fb4 cmp eax,0xfe
            fl = sub::<32>((rax as u32 as u64), 0xfeu64).1;
            // 51fb9 jne 0x5200c
            if crate::cov::brk(0x51fb9, fl.ne()) { pc = 0x5200c; continue; }
            pc = 0x51fbb;
        }
        0x51fbb => {
            // block 51fbb
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 51fbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51fbb mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 51fc2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 51fc5 lea rdx,[rbp-0x44]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // 51fc9 mov esi,0x15
            rsi = (0x15u64) as u32 as u64;
            // 51fce mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // 51fd3 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // 51fd9 mov rdi,rbx
            rdi = rbx;
            // 51fdc call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 51fdf cmp DWORD PTR [rbp-0x44],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64, 0x0u64).1;
            // 51fe3 mov eax,0xfe020000
            rax = (0xfe020000u64) as u32 as u64;
            // 51fe8 mov ecx,0xfe000000
            rcx = (0xfe000000u64) as u32 as u64;
            // 51fed cmovne ecx,eax
            if fl.ne() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 51ff0 lea rax,[rbp-0x38]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 51ff4 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 51ff8 mov DWORD PTR [rbp-0x38],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rcx as u32 as u64)) as u32);
            // 51ffb mov r15d,0x71
            r15 = (0x71u64) as u32 as u64;
            // 52001 mov r13d,0x70
            r13 = (0x70u64) as u32 as u64;
            // 52007 jmp 0x5213d
            pc = 0x5213d; continue;
        }
        0x5200c => {
            // block 5200c
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 5200c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5200c mov DWORD PTR [rbp-0x48],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0x0u64) as u32);
            // 52013 mov edi,DWORD PTR [rbp-0x3c]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64) as u32 as u64;
            // 52016 call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5201b mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 5201e movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 52021 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 52024 jb 0x52048
            if crate::cov::brk(0x52024, fl.b()) { pc = 0x52048; continue; }
            pc = 0x52026;
        }
        0x52026 => {
            // block 52026
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 52026 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52026 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52029 lea rdx,[rbp-0x48]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 5202d mov esi,0x2724
            rsi = (0x2724u64) as u32 as u64;
            // 52032 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 52037 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5203a mov rdi,rbx
            rdi = rbx;
            // 5203d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 52040 cmp DWORD PTR [rbp-0x48],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, 0x0u64).1;
            // 52044 sete r15b
            r15 = (r15 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x52048;
        }
        0x52048 => {
            // block 52048
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 52048 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52048 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 5204b mov eax,0x1d
            rax = (0x1du64) as u32 as u64;
            // 52050 mov r12d,0x40
            r12 = (0x40u64) as u32 as u64;
            // 52056 cmovne r12d,eax
            if fl.ne() { r12 = ((rax as u32 as u64)) as u32 as u64; } else { r12 = r12 as u32 as u64; }
            // 5205a mov DWORD PTR [rbp-0x4c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), (0x0u64) as u32);
            // 52061 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52064 lea rdx,[rbp-0x4c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffb4);
            // 52068 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 5206d mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 52073 mov rdi,rbx
            rdi = rbx;
            // 52076 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 52079 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5207c cmp DWORD PTR [rbp-0x4c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64, 0x0u64).1;
            // 52080 je 0x520f9
            if crate::cov::brk(0x52080, fl.e()) { pc = 0x520f9; continue; }
            pc = 0x52082;
        }
        0x52082 => {
            // block 52082
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 52082 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52082 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52085 lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 52089 mov QWORD PTR [rbp-0x60],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rdx) as u64);
            // 5208d mov ecx,0xff
            rcx = (0xffu64) as u32 as u64;
            // 52092 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 52095 mov rdi,rbx
            rdi = rbx;
            // 52098 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 5209b call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5209e cmp DWORD PTR [rbp-0x38],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, 0x0u64).1;
            // 520a2 mov eax,DWORD PTR [r14]
            rax = (m.r32(r14) as u64) as u32 as u64;
            // 520a5 je 0x520f5
            if crate::cov::brk(0x520a5, fl.e()) { pc = 0x520f5; continue; }
            pc = 0x520a7;
        }
        0x520a7 => {
            // block 520a7
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 520a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 520a7 dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 520a9 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 520ac jbe 0x52100
            if crate::cov::brk(0x520ac, fl.be()) { pc = 0x52100; continue; }
            pc = 0x520ae;
        }
        0x520ae => {
            // block 520ae
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 520ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 520ae mov DWORD PTR [rbp-0x50],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u32);
            // 520b5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 520b8 lea rdx,[rbp-0x50]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 520bc mov ecx,0x40000
            rcx = (0x40000u64) as u32 as u64;
            // 520c1 mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 520c7 mov rdi,rbx
            rdi = rbx;
            // 520ca mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 520cd call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 520d0 cmp DWORD PTR [rbp-0x50],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64, 0x1u64).1;
            // 520d4 jne 0x520f9
            if crate::cov::brk(0x520d4, fl.ne()) { pc = 0x520f9; continue; }
            pc = 0x520d6;
        }
        0x520d6 => {
            // block 520d6
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 520d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 520d6 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 520d9 mov eax,0x6e
            rax = (0x6eu64) as u32 as u64;
            // 520de mov r13d,0xfc
            r13 = (0xfcu64) as u32 as u64;
            // 520e4 cmovne r13d,eax
            if fl.ne() { r13 = ((rax as u32 as u64)) as u32 as u64; } else { r13 = r13 as u32 as u64; }
            // 520e8 mov eax,0x6f
            rax = (0x6fu64) as u32 as u64;
            // 520ed mov r15d,0xfd
            r15 = (0xfdu64) as u32 as u64;
            // 520f3 jmp 0x5211d
            pc = 0x5211d; continue;
        }
        0x520f5 => {
            // block 520f5
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 520f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 520f5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 520f7 je 0x52100
            if crate::cov::brk(0x520f7, fl.e()) { pc = 0x52100; continue; }
            pc = 0x520f9;
        }
        0x520f9 => {
            // block 520f9
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 520f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 520f9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 520fb jmp 0x52203
            pc = 0x52203; continue;
        }
        0x52100 => {
            // block 52100
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 52100 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52100 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 52103 mov eax,0x1e
            rax = (0x1eu64) as u32 as u64;
            // 52108 mov r13d,0x41
            r13 = (0x41u64) as u32 as u64;
            // 5210e cmovne r13d,eax
            if fl.ne() { r13 = ((rax as u32 as u64)) as u32 as u64; } else { r13 = r13 as u32 as u64; }
            // 52112 mov eax,0x1f
            rax = (0x1fu64) as u32 as u64;
            // 52117 mov r15d,0x42
            r15 = (0x42u64) as u32 as u64;
            pc = 0x5211d;
        }
        0x5211d => {
            // block 5211d
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 5211d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5211d cmovne r15d,eax
            if fl.ne() { r15 = ((rax as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // 52121 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52124 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 52129 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5212c mov rdi,rbx
            rdi = rbx;
            // 5212f mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 52132 mov DWORD PTR [rbp-0x64],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), ((r12 as u32 as u64)) as u32);
            // 52136 lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 5213a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x5213d;
        }
        0x5213d => {
            // block 5213d
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 5213d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5213d mov DWORD PTR [rbp-0x58],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((r13 as u32 as u64)) as u32);
            // 52141 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52144 lea rdx,[rbp-0x34]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 52148 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 5214b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 52150 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 52153 mov rdi,rbx
            rdi = rbx;
            // 52156 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 52159 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5215c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5215f lea r13,[rbp-0x30]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 52163 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 52168 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5216b mov rdi,rbx
            rdi = rbx;
            // 5216e mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 52171 mov rdx,r13
            rdx = r13;
            // 52174 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x52177;
        }
        0x52177 => {
            // block 52177
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 52177 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52177 cmp DWORD PTR [rbp-0x54],0xfe
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffac)) as u64, 0xfeu64).1;
            // 5217e mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 52181 mov DWORD PTR [r14+0x8],eax
            m.w32(r14.wrapping_add(0x8), ((rax as u32 as u64)) as u32);
            // 52185 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 52189 mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 5218c je 0x521a6
            if crate::cov::brk(0x5218c, fl.e()) { pc = 0x521a6; continue; }
            pc = 0x5218e;
        }
        0x5218e => {
            // block 5218e
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 5218e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5218e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52191 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 52196 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 52199 mov rdi,rbx
            rdi = rbx;
            // 5219c mov esi,DWORD PTR [rbp-0x64]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // 5219f mov rdx,QWORD PTR [rbp-0x60]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 521a3 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x521a6;
        }
        0x521a6 => {
            // block 521a6
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 521a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 521a6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 521a9 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 521ae xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 521b1 mov rdi,rbx
            rdi = rbx;
            // 521b4 mov esi,DWORD PTR [rbp-0x58]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // 521b7 lea rdx,[rbp-0x34]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 521bb call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 521be mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 521c1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 521c6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 521c9 mov rdi,rbx
            rdi = rbx;
            // 521cc mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 521cf mov rdx,r13
            rdx = r13;
            // 521d2 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 521d5 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // 521d8 cmp eax,DWORD PTR [r14]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14) as u64).1;
            // 521db jne 0x521f7
            if crate::cov::brk(0x521db, fl.ne()) { pc = 0x521f7; continue; }
            pc = 0x521dd;
        }
        0x521dd => {
            // block 521dd
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 521dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 521dd mov eax,DWORD PTR [rbp-0x34]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // 521e0 cmp eax,DWORD PTR [r14+0x4]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x4)) as u64).1;
            // 521e4 jne 0x521f7
            if crate::cov::brk(0x521e4, fl.ne()) { pc = 0x521f7; continue; }
            pc = 0x521e6;
        }
        0x521e6 => {
            // block 521e6
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 521e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 521e6 mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 521e9 cmp eax,DWORD PTR [r14+0x8]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x8)) as u64).1;
            // 521ed mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 521ef je 0x521f4
            if crate::cov::brk(0x521ef, fl.e()) { pc = 0x521f4; continue; }
            pc = 0x521f1;
        }
        0x521f1 => {
            // block 521f1
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 521f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 521f1 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            pc = 0x521f4;
        }
        0x521f4 => {
            // block 521f4
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 521f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 521f4 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x521f7;
        }
        0x521f7 => {
            // block 521f7
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 521f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 521f7 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 521f9 test r12b,0x1
            fl = logic::<8>((r12 as u8 as u64) & 0x1u64);
            // 521fd je 0x52177
            if crate::cov::brk(0x521fd, fl.e()) { pc = 0x52177; continue; }
            pc = 0x52203;
        }
        0x52203 => {
            // block 52203
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 52203 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52203 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 52207 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 52208 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5220a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 5220c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5220e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 52210 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 52211 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2078 size=718
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2078<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2078;
    loop {
        match pc {
        0xe2078 => {
            // block e2078
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e2078 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2078 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2079 mov rbp,rsp
            rbp = rsp;
            // e207c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e207e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2080 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2082 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2083 sub rsp,0xc0
            { let (r, f) = sub::<64>(rsp, 0xc0u64);
              rsp = r;
              fl = f; }
            // e208a mov r14,rsi
            r14 = rsi;
            // e208d mov r15,rdi
            r15 = rdi;
            // e2090 call 0xe49c4
            { let r = { let t = m.env.code_va(0xe49c4); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2095 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // e2097 test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e2099 je 0xe225c
            if crate::cov::brk(0xe2099, fl.e()) { pc = 0xe225c; continue; }
            pc = 0xe209f;
        }
        0xe209f => {
            // block e209f
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e209f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e209f lea rdi,[r15+0x34]
            rdi = r15.wrapping_add(0x34);
            // e20a3 lea rsi,[rip+0x5739f] # 0x139449
            rsi = m.env.data_va(318486);
            // e20aa mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // e20af xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e20b1 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e20b6 mov esi,0xfa
            rsi = (0xfau64) as u32 as u64;
            // e20bb mov rdi,r15
            rdi = r15;
            // e20be call 0xe435c
            { let r = { let t = m.env.code_va(0xe435c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e20c3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e20c5 je 0xe2228
            if crate::cov::brk(0xe20c5, fl.e()) { pc = 0xe2228; continue; }
            pc = 0xe20cb;
        }
        0xe20cb => {
            // block e20cb
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e20cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e20cb xorps xmm0,xmm0
            x0 = 0;
            // e20ce movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // e20d2 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // e20da lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e20de mov rdi,r15
            rdi = r15;
            // e20e1 call 0xe4a2e
            { let r = { let t = m.env.code_va(0xe4a2e); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e20e6 xorps xmm0,xmm0
            x0 = 0;
            // e20e9 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // e20ed mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // e20f5 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e20f9 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e20fd call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2102 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e2106 lea rdx,[rbp-0x60]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e210a call 0xe171e
            { let r = { let t = m.env.code_va(0xe171e); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e210f lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e2113 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2118 mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e211c mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e2120 sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // e2123 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e2127 movabs r12,0xaaaaaaaaaaaaaaab
            r12 = 0xaaaa_aaaa_aaaa_aaabu64;
            // e2131 imul rax,r12
            { let (r, f) = imul::<64>(rax, r12); rax = r; fl = f; }
            // e2135 cmp rax,0x3
            fl = sub::<64>(rax, 0x3u64).1;
            // e2139 jb 0xe2240
            if crate::cov::brk(0xe2139, fl.b()) { pc = 0xe2240; continue; }
            pc = 0xe213f;
        }
        0xe213f => {
            // block e213f
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e213f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e213f xorps xmm0,xmm0
            x0 = 0;
            // e2142 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // e2149 mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // e2151 lea rsi,[rip+0x572bb] # 0x139413
            rsi = m.env.data_va(318432);
            // e2158 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e215f mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // e2164 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2169 lea rdx,[rbp-0xa8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e2170 lea rcx,[rbp-0x90]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e2177 mov rsi,rbx
            rsi = rbx;
            // e217a call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e217f mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e2181 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e2188 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e218d test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e218f je 0xe2234
            if crate::cov::brk(0xe218f, fl.e()) { pc = 0xe2234; continue; }
            pc = 0xe2195;
        }
        0xe2195 => {
            // block e2195
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e2195 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2195 lea rsi,[rip+0x5727e] # 0x13941a
            rsi = m.env.data_va(318439);
            // e219c lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e21a3 call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e21a8 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e21aa je 0xe226e
            if crate::cov::brk(0xe21aa, fl.e()) { pc = 0xe226e; continue; }
            pc = 0xe21b0;
        }
        0xe21b0 => {
            // block e21b0
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e21b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e21b0 lea rsi,[rip+0x57266] # 0x13941d
            rsi = m.env.data_va(318442);
            // e21b7 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e21be call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e21c3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e21c5 jne 0xe2234
            if crate::cov::brk(0xe21c5, fl.ne()) { pc = 0xe2234; continue; }
            pc = 0xe21c7;
        }
        0xe21c7 => {
            // block e21c7
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e21c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e21c7 mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e21cb mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e21cf sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // e21d2 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e21d6 imul rax,r12
            { let (r, f) = imul::<64>(rax, r12); rax = r; fl = f; }
            // e21da cmp rax,0x3
            fl = sub::<64>(rax, 0x3u64).1;
            // e21de jb 0xe2234
            if crate::cov::brk(0xe21de, fl.b()) { pc = 0xe2234; continue; }
            pc = 0xe21e0;
        }
        0xe21e0 => {
            // block e21e0
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e21e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e21e0 lea rsi,[rip+0x5723b] # 0x139422
            rsi = m.env.data_va(318447);
            // e21e7 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e21ee mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // e21f3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e21f8 add rbx,0x30
            { let (r, f) = add::<64>(rbx, 0x30u64);
              rbx = r;
              fl = f; }
            // e21fc lea rcx,[r15+0x10]
            rcx = r15.wrapping_add(0x10);
            // e2200 lea rdx,[rbp-0xd8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e2207 mov rsi,rbx
            rsi = rbx;
            // e220a call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e220f lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e2216 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e221b mov DWORD PTR [r15+0x8],0x1a
            m.w32(r15.wrapping_add(0x8), (0x1au64) as u32);
            // e2223 jmp 0xe22be
            pc = 0xe22be; continue;
        }
        0xe2228 => {
            // block e2228
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e2228 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2228 mov DWORD PTR [r15+0x8],0x15
            m.w32(r15.wrapping_add(0x8), (0x15u64) as u32);
            // e2230 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e2232 jmp 0xe225c
            pc = 0xe225c; continue;
        }
        0xe2234 => {
            // block e2234
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e2234 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2234 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e223b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xe2240;
        }
        0xe2240 => {
            // block e2240
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e2240 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2240 mov DWORD PTR [r15+0x8],0x17
            m.w32(r15.wrapping_add(0x8), (0x17u64) as u32);
            // e2248 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe224a;
        }
        0xe224a => {
            // block e224a
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e224a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e224a lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e224e call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2253 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e2257 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xe225c;
        }
        0xe225c => {
            // block e225c
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e225c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e225c mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e225e add rsp,0xc0
            { let (r, f) = add::<64>(rsp, 0xc0u64);
              rsp = r;
              fl = f; }
            // e2265 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2266 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2268 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e226a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e226c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e226d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe226e => {
            // block e226e
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e226e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e226e mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e2272 lea rsi,[rip+0x571d7] # 0x139450
            rsi = m.env.data_va(318493);
            // e2279 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e2280 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e2285 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e228a add rbx,0x30
            { let (r, f) = add::<64>(rbx, 0x30u64);
              rbx = r;
              fl = f; }
            // e228e lea rdx,[rbp-0xc0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e2295 mov rsi,rbx
            rsi = rbx;
            // e2298 mov rcx,r14
            rcx = r14;
            // e229b call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e22a0 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e22a3 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e22aa call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e22af mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // e22b1 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // e22b4 jne 0xe22c0
            if crate::cov::brk(0xe22b4, fl.ne()) { pc = 0xe22c0; continue; }
            pc = 0xe22b6;
        }
        0xe22b6 => {
            // block e22b6
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e22b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e22b6 mov DWORD PTR [r15+0x8],0x1b
            m.w32(r15.wrapping_add(0x8), (0x1bu64) as u32);
            pc = 0xe22be;
        }
        0xe22be => {
            // block e22be
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e22be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e22be xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe22c0;
        }
        0xe22c0 => {
            // block e22c0
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e22c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e22c0 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e22c7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e22cc jmp 0xe224a
            pc = 0xe224a; continue;
        }
        0xe22d1 => {
            // block e22d1
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e22d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e22d1 mov rbx,rax
            rbx = rax;
            // e22d4 jmp 0xe2312
            pc = 0xe2312; continue;
        }
        0xe22d6 => {
            // block e22d6
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e22d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e22d6 mov rbx,rax
            rbx = rax;
            // e22d9 jmp 0xe2309
            pc = 0xe2309; continue;
        }
        0xe22db => {
            // block e22db
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e22db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e22db mov rbx,rax
            rbx = rax;
            // e22de lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e22e2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e22e7 jmp 0xe2309
            pc = 0xe2309; continue;
        }
        0xe22e9 => {
            // block e22e9
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e22e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e22e9 mov rbx,rax
            rbx = rax;
            // e22ec jmp 0xe22fd
            pc = 0xe22fd; continue;
        }
        0xe22ee => {
            // block e22ee
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e22ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e22ee mov rbx,rax
            rbx = rax;
            // e22f1 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e22f8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xe22fd;
        }
        0xe22fd => {
            // block e22fd
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e22fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e22fd lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e2304 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xe2309;
        }
        0xe2309 => {
            // block e2309
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e2309 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2309 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e230d call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xe2312;
        }
        0xe2312 => {
            // block e2312
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e2312 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2312 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e2316 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e231b mov rdi,rbx
            rdi = rbx;
            // e231e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2323 mov rbx,rax
            rbx = rax;
            // e2326 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e232d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2332 jmp 0xe22fd
            pc = 0xe22fd; continue;
        }
        0xe2334 => {
            // block e2334
            #[cfg(feature = "trace")] eprintln!("f_000e2078 pc e2334 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2334 mov rbx,rax
            rbx = rax;
            // e2337 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e233e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2343 jmp 0xe22fd
            pc = 0xe22fd; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd9c18 size=750
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d9c18<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd9c18;
    loop {
        match pc {
        0xd9c18 => {
            // block d9c18
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9c18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9c18 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d9c19 mov rbp,rsp
            rbp = rsp;
            // d9c1c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d9c1e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d9c20 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d9c22 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d9c24 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d9c25 sub rsp,0x1e8
            { let (r, f) = sub::<64>(rsp, 0x1e8u64);
              rsp = r;
              fl = f; }
            // d9c2c mov r13,rsi
            r13 = rsi;
            // d9c2f mov rbx,rdi
            rbx = rdi;
            // d9c32 mov r14,QWORD PTR [rip+0x6679f] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // d9c39 mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // d9c3c mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // d9c40 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d9c43 call QWORD PTR [rax+0x100]
            { let r = { let t = m.r64(rax.wrapping_add(0x100)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9c49 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d9c4b je 0xd9d50
            if crate::cov::brk(0xd9c4b, fl.e()) { pc = 0xd9d50; continue; }
            pc = 0xd9c51;
        }
        0xd9c51 => {
            // block d9c51
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9c51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9c51 mov rdi,QWORD PTR [rbx+0x6b8]
            rdi = m.r64(rbx.wrapping_add(0x6b8)) as u64;
            // d9c58 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d9c5b je 0xd9d58
            if crate::cov::brk(0xd9c5b, fl.e()) { pc = 0xd9d58; continue; }
            pc = 0xd9c61;
        }
        0xd9c61 => {
            // block d9c61
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9c61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9c61 xorps xmm0,xmm0
            x0 = 0;
            // d9c64 movaps XMMWORD PTR [rbp-0x1e0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe20), x0);
            // d9c6b mov QWORD PTR [rbp-0x1d0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe30), (0x0u64) as u64);
            // d9c76 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d9c79 mov rax,QWORD PTR [rax+0x48]
            rax = m.r64(rax.wrapping_add(0x48)) as u64;
            // d9c7d call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9c7f mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d9c82 mov rdi,QWORD PTR [rbx+0x6b8]
            rdi = m.r64(rbx.wrapping_add(0x6b8)) as u64;
            // d9c89 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d9c8c mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d9c90 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // d9c95 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9c97 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // d9c9a mov rdi,QWORD PTR [rbx+0x6b8]
            rdi = m.r64(rbx.wrapping_add(0x6b8)) as u64;
            // d9ca1 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d9ca4 mov rax,QWORD PTR [rax+0x40]
            rax = m.r64(rax.wrapping_add(0x40)) as u64;
            // d9ca8 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d9caa call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9cac mov rdi,QWORD PTR [rbx+0x6b8]
            rdi = m.r64(rbx.wrapping_add(0x6b8)) as u64;
            // d9cb3 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d9cb6 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // d9cba lea rdx,[rbp-0x1e0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe20);
            // d9cc1 mov ecx,0x64
            rcx = (0x64u64) as u32 as u64;
            // d9cc6 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // d9cc9 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9ccb mov rdi,QWORD PTR [rbx+0x6b8]
            rdi = m.r64(rbx.wrapping_add(0x6b8)) as u64;
            // d9cd2 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // d9cd5 mov rcx,QWORD PTR [rcx+0x40]
            rcx = m.r64(rcx.wrapping_add(0x40)) as u64;
            // d9cd9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d9cdb je 0xd9e4b
            if crate::cov::brk(0xd9cdb, fl.e()) { pc = 0xd9e4b; continue; }
            pc = 0xd9ce1;
        }
        0xd9ce1 => {
            // block d9ce1
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9ce1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9ce1 movzx esi,r15b
            rsi = ((r15 as u8 as u64)) as u32 as u64;
            // d9ce5 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9ce7 lea rsi,[rip+0x5efda] # 0x138cc8
            rsi = m.env.data_va(316565);
            // d9cee mov rdi,r13
            rdi = r13;
            // d9cf1 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9cf6 mov rsi,QWORD PTR [rbp-0x1e0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            // d9cfd mov rdx,QWORD PTR [rbp-0x1d8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // d9d04 mov rax,rdx
            rax = rdx;
            // d9d07 sub rax,rsi
            { let (r, f) = sub::<64>(rax, rsi);
              rax = r;
              fl = f; }
            // d9d0a cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // d9d0e jb 0xd9e51
            if crate::cov::brk(0xd9d0e, fl.b()) { pc = 0xd9e51; continue; }
            pc = 0xd9d14;
        }
        0xd9d14 => {
            // block d9d14
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9d14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9d14 cmp DWORD PTR [rsi],0xffffffff
            fl = sub::<32>(m.r32(rsi) as u64, 0xffffffffu64).1;
            // d9d17 je 0xd9e51
            if crate::cov::brk(0xd9d17, fl.e()) { pc = 0xd9e51; continue; }
            pc = 0xd9d1d;
        }
        0xd9d1d => {
            // block d9d1d
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9d1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9d1d mov rdi,r13
            rdi = r13;
            // d9d20 call 0xebeb2
            { let r = { let t = m.env.stub_va(0x67f); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9d25 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d9d27 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d9d29 mov rdi,r13
            rdi = r13;
            // d9d2c call 0xebfa8
            { let r = { let t = m.env.stub_va(0x775); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9d31 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // d9d34 cmp rax,0xffffffffffffffff
            fl = sub::<64>(rax, 0xffff_ffff_ffff_ffffu64).1;
            // d9d38 je 0xd9e54
            if crate::cov::brk(0xd9d38, fl.e()) { pc = 0xd9e54; continue; }
            pc = 0xd9d3e;
        }
        0xd9d3e => {
            // block d9d3e
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9d3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9d3e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d9d40 mov rdi,r13
            rdi = r13;
            // d9d43 mov rsi,rax
            rsi = rax;
            // d9d46 call 0xec008
            { let r = { let t = m.env.stub_va(0x7d5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9d4b jmp 0xd9e54
            pc = 0xd9e54; continue;
        }
        0xd9d50 => {
            // block d9d50
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9d50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9d50 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d9d53 jmp 0xd9e75
            pc = 0xd9e75; continue;
        }
        0xd9d58 => {
            // block d9d58
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9d58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9d58 mov QWORD PTR [rbp-0x208],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdf8), (r13) as u64);
            // d9d5f lea r12,[rbp-0x1c0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // d9d66 mov esi,0x190
            rsi = (0x190u64) as u32 as u64;
            // d9d6b mov rdi,r12
            rdi = r12;
            // d9d6e call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9d73 mov r13d,DWORD PTR [rbx+0x350]
            r13 = (m.r32(rbx.wrapping_add(0x350)) as u64) as u32 as u64;
            // d9d7a mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // d9d7f mov rdi,rbx
            rdi = rbx;
            // d9d82 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9d87 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd9d8a;
        }
        0xd9d8a => {
            // block d9d8a
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9d8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9d8a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d9d8d mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d9d92 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d9d97 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d9d9a mov rdi,rbx
            rdi = rbx;
            // d9d9d mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // d9da0 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9da3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d9da6 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d9dab mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // d9db0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d9db5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d9db8 mov rdi,rbx
            rdi = rbx;
            // d9dbb call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9dbe mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d9dc1 mov rdi,rbx
            rdi = rbx;
            // d9dc4 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9dca mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d9dcd mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d9dd2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d9dd7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d9dda mov rdi,rbx
            rdi = rbx;
            // d9ddd mov rdx,r12
            rdx = r12;
            // d9de0 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9de3 mov eax,DWORD PTR [r12]
            rax = (m.r32(r12) as u64) as u32 as u64;
            // d9de7 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // d9dea cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // d9ded je 0xd9e0b
            if crate::cov::brk(0xd9ded, fl.e()) { pc = 0xd9e0b; continue; }
            pc = 0xd9def;
        }
        0xd9def => {
            // block d9def
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9def rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9def test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d9df1 je 0xd9e94
            if crate::cov::brk(0xd9df1, fl.e()) { pc = 0xd9e94; continue; }
            pc = 0xd9df7;
        }
        0xd9df7 => {
            // block d9df7
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9df7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9df7 add r13d,0x4
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x4u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d9dfb inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // d9dfe add r12,0x4
            { let (r, f) = add::<64>(r12, 0x4u64);
              r12 = r;
              fl = f; }
            // d9e02 cmp r14,0x64
            fl = sub::<64>(r14, 0x64u64).1;
            // d9e06 jb 0xd9d8a
            if crate::cov::brk(0xd9e06, fl.b()) { pc = 0xd9d8a; continue; }
            pc = 0xd9e08;
        }
        0xd9e08 => {
            // block d9e08
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9e08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9e08 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd9e0b;
        }
        0xd9e0b => {
            // block d9e0b
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9e0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9e0b xorps xmm0,xmm0
            x0 = 0;
            // d9e0e movaps XMMWORD PTR [rbp-0x200],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe00), x0);
            // d9e15 mov QWORD PTR [rbp-0x1f0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (0x0u64) as u64);
            pc = 0xd9e20;
        }
        0xd9e20 => {
            // block d9e20
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9e20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9e20 lea rsi,[rbp-0x200]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe00);
            // d9e27 mov rdi,QWORD PTR [rbp-0x208]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdf8)) as u64;
            // d9e2e call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9e33 lea rdi,[rbp-0x200]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe00);
            // d9e3a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9e3f mov r14,QWORD PTR [rip+0x66592] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // d9e46 mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // d9e49 jmp 0xd9e75
            pc = 0xd9e75; continue;
        }
        0xd9e4b => {
            // block d9e4b
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9e4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9e4b movzx esi,r15b
            rsi = ((r15 as u8 as u64)) as u32 as u64;
            // d9e4f call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xd9e51;
        }
        0xd9e51 => {
            // block d9e51
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9e51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9e51 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd9e54;
        }
        0xd9e54 => {
            // block d9e54
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9e54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9e54 mov rdi,QWORD PTR [rbp-0x1e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            // d9e5b test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d9e5e je 0xd9e75
            if crate::cov::brk(0xd9e5e, fl.e()) { pc = 0xd9e75; continue; }
            pc = 0xd9e60;
        }
        0xd9e60 => {
            // block d9e60
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9e60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9e60 cmp QWORD PTR [rbp-0x1d8],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64, rdi).1;
            // d9e67 je 0xd9e70
            if crate::cov::brk(0xd9e67, fl.e()) { pc = 0xd9e70; continue; }
            pc = 0xd9e69;
        }
        0xd9e69 => {
            // block d9e69
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9e69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9e69 mov QWORD PTR [rbp-0x1d8],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe28), (rdi) as u64);
            pc = 0xd9e70;
        }
        0xd9e70 => {
            // block d9e70
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9e70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9e70 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xd9e75;
        }
        0xd9e75 => {
            // block d9e75
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9e75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9e75 cmp r14,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r14, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // d9e79 jne 0xd9f01
            if crate::cov::brk(0xd9e79, fl.ne()) { pc = 0xd9f01; continue; }
            pc = 0xd9e7f;
        }
        0xd9e7f => {
            // block d9e7f
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9e7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9e7f mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // d9e82 add rsp,0x1e8
            { let (r, f) = add::<64>(rsp, 0x1e8u64);
              rsp = r;
              fl = f; }
            // d9e89 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d9e8a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d9e8c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d9e8e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d9e90 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d9e92 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d9e93 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd9e94 => {
            // block d9e94
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9e94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9e94 xorps xmm0,xmm0
            x0 = 0;
            // d9e97 movaps XMMWORD PTR [rbp-0x200],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe00), x0);
            // d9e9e mov QWORD PTR [rbp-0x1f0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (0x0u64) as u64);
            // d9ea9 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // d9eac lea rdi,[rbp-0x200]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe00);
            // d9eb3 lea rsi,[rbp-0x1c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // d9eba call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9ebf jmp 0xd9e20
            pc = 0xd9e20; continue;
        }
        0xd9ec4 => {
            // block d9ec4
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9ec4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9ec4 mov rbx,rax
            rbx = rax;
            // d9ec7 lea rdi,[rbp-0x200]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe00);
            // d9ece call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d9ed3 jmp 0xd9ef9
            pc = 0xd9ef9; continue;
        }
        0xd9ed5 => {
            // block d9ed5
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9ed5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9ed5 mov rbx,rax
            rbx = rax;
            // d9ed8 mov rdi,QWORD PTR [rbp-0x1e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            // d9edf test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d9ee2 je 0xd9ef9
            if crate::cov::brk(0xd9ee2, fl.e()) { pc = 0xd9ef9; continue; }
            pc = 0xd9ee4;
        }
        0xd9ee4 => {
            // block d9ee4
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9ee4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9ee4 cmp QWORD PTR [rbp-0x1d8],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64, rdi).1;
            // d9eeb je 0xd9ef4
            if crate::cov::brk(0xd9eeb, fl.e()) { pc = 0xd9ef4; continue; }
            pc = 0xd9eed;
        }
        0xd9eed => {
            // block d9eed
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9eed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9eed mov QWORD PTR [rbp-0x1d8],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe28), (rdi) as u64);
            pc = 0xd9ef4;
        }
        0xd9ef4 => {
            // block d9ef4
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9ef4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9ef4 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xd9ef9;
        }
        0xd9ef9 => {
            // block d9ef9
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9ef9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9ef9 mov rdi,rbx
            rdi = rbx;
            // d9efc call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xd9f01;
        }
        0xd9f01 => {
            // block d9f01
            #[cfg(feature = "trace")] eprintln!("f_000d9c18 pc d9f01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9f01 call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe5ab6 size=760
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e5ab6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe5ab6;
    loop {
        match pc {
        0xe5ab6 => {
            // block e5ab6
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5ab6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5ab6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e5ab7 mov rbp,rsp
            rbp = rsp;
            // e5aba push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e5abc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e5abe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e5abf sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e5ac3 mov r15,rdi
            r15 = rdi;
            // e5ac6 mov DWORD PTR [r15+0x88],0x0
            m.w32(r15.wrapping_add(0x88), (0x0u64) as u32);
            // e5ad1 mov QWORD PTR [r15+0x80],0x0
            m.w64(r15.wrapping_add(0x80), (0x0u64) as u64);
            // e5adc lea r14,[r15+0x20]
            r14 = r15.wrapping_add(0x20);
            // e5ae0 test BYTE PTR [r15+0x20],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x20)) as u64 & 0x1u64);
            // e5ae5 jne 0xe5aed
            if crate::cov::brk(0xe5ae5, fl.ne()) { pc = 0xe5aed; continue; }
            pc = 0xe5ae7;
        }
        0xe5ae7 => {
            // block e5ae7
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5ae7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5ae7 lea rax,[r14+0x1]
            rax = r14.wrapping_add(0x1);
            // e5aeb jmp 0xe5af1
            pc = 0xe5af1; continue;
        }
        0xe5aed => {
            // block e5aed
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5aed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5aed mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xe5af1;
        }
        0xe5af1 => {
            // block e5af1
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5af1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5af1 mov al,BYTE PTR [rax]
            rax = (rax & !0xff) | ((m.r8(rax) as u64) as u8 as u64);
            // e5af3 mov BYTE PTR [rbp-0x1a],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6), ((rax as u8 as u64)) as u8);
            // e5af6 mov BYTE PTR [rbp-0x19],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe7), (0x0u64) as u8);
            // e5afa lea rsi,[rip+0x539fe] # 0x1394ff
            rsi = m.env.data_va(318668);
            // e5b01 lea rdi,[rbp-0x1a]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe6);
            // e5b05 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e5b09 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5b0b call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e5b10 mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // e5b13 and eax,0x3
            { let r = (rax as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5b16 shl eax,0x18
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x18u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e5b19 or DWORD PTR [r15+0x88],eax
            { let r = m.r32(r15.wrapping_add(0x88)) as u64 | (rax as u32 as u64); let f = logic::<32>(r);
              m.w32(r15.wrapping_add(0x88), (r) as u32);
              fl = f; }
            // e5b20 test BYTE PTR [r15+0x20],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x20)) as u64 & 0x1u64);
            // e5b25 jne 0xe5b2d
            if crate::cov::brk(0xe5b25, fl.ne()) { pc = 0xe5b2d; continue; }
            pc = 0xe5b27;
        }
        0xe5b27 => {
            // block e5b27
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5b27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5b27 lea rax,[r14+0x1]
            rax = r14.wrapping_add(0x1);
            // e5b2b jmp 0xe5b31
            pc = 0xe5b31; continue;
        }
        0xe5b2d => {
            // block e5b2d
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5b2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5b2d mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xe5b31;
        }
        0xe5b31 => {
            // block e5b31
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5b31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5b31 mov al,BYTE PTR [rax+0x1]
            rax = (rax & !0xff) | ((m.r8(rax.wrapping_add(0x1)) as u64) as u8 as u64);
            // e5b34 mov BYTE PTR [rbp-0x1a],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6), ((rax as u8 as u64)) as u8);
            // e5b37 lea rsi,[rip+0x539c1] # 0x1394ff
            rsi = m.env.data_va(318668);
            // e5b3e lea rdi,[rbp-0x1a]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe6);
            // e5b42 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e5b46 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5b48 call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e5b4d mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // e5b50 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5b53 shl eax,0x10
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e5b56 or DWORD PTR [r15+0x88],eax
            { let r = m.r32(r15.wrapping_add(0x88)) as u64 | (rax as u32 as u64); let f = logic::<32>(r);
              m.w32(r15.wrapping_add(0x88), (r) as u32);
              fl = f; }
            // e5b5d test BYTE PTR [r15+0x20],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x20)) as u64 & 0x1u64);
            // e5b62 jne 0xe5b6a
            if crate::cov::brk(0xe5b62, fl.ne()) { pc = 0xe5b6a; continue; }
            pc = 0xe5b64;
        }
        0xe5b64 => {
            // block e5b64
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5b64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5b64 lea rax,[r14+0x1]
            rax = r14.wrapping_add(0x1);
            // e5b68 jmp 0xe5b6e
            pc = 0xe5b6e; continue;
        }
        0xe5b6a => {
            // block e5b6a
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5b6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5b6a mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xe5b6e;
        }
        0xe5b6e => {
            // block e5b6e
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5b6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5b6e mov al,BYTE PTR [rax+0x3]
            rax = (rax & !0xff) | ((m.r8(rax.wrapping_add(0x3)) as u64) as u8 as u64);
            // e5b71 mov BYTE PTR [rbp-0x1a],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6), ((rax as u8 as u64)) as u8);
            // e5b74 lea rsi,[rip+0x53984] # 0x1394ff
            rsi = m.env.data_va(318668);
            // e5b7b lea rdi,[rbp-0x1a]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe6);
            // e5b7f lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e5b83 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5b85 call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e5b8a mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // e5b8d and eax,0x7
            { let r = (rax as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5b90 shl eax,0x8
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e5b93 or DWORD PTR [r15+0x88],eax
            { let r = m.r32(r15.wrapping_add(0x88)) as u64 | (rax as u32 as u64); let f = logic::<32>(r);
              m.w32(r15.wrapping_add(0x88), (r) as u32);
              fl = f; }
            // e5b9a test BYTE PTR [r15+0x20],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x20)) as u64 & 0x1u64);
            // e5b9f jne 0xe5ba7
            if crate::cov::brk(0xe5b9f, fl.ne()) { pc = 0xe5ba7; continue; }
            pc = 0xe5ba1;
        }
        0xe5ba1 => {
            // block e5ba1
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5ba1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5ba1 lea rax,[r14+0x1]
            rax = r14.wrapping_add(0x1);
            // e5ba5 jmp 0xe5bab
            pc = 0xe5bab; continue;
        }
        0xe5ba7 => {
            // block e5ba7
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5ba7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5ba7 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xe5bab;
        }
        0xe5bab => {
            // block e5bab
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5bab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5bab mov al,BYTE PTR [rax+0x4]
            rax = (rax & !0xff) | ((m.r8(rax.wrapping_add(0x4)) as u64) as u8 as u64);
            // e5bae mov BYTE PTR [rbp-0x1a],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6), ((rax as u8 as u64)) as u8);
            // e5bb1 lea rsi,[rip+0x53947] # 0x1394ff
            rsi = m.env.data_va(318668);
            // e5bb8 lea rdi,[rbp-0x1a]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe6);
            // e5bbc lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e5bc0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5bc2 call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e5bc7 mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // e5bca and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5bcd or DWORD PTR [r15+0x88],eax
            { let r = m.r32(r15.wrapping_add(0x88)) as u64 | (rax as u32 as u64); let f = logic::<32>(r);
              m.w32(r15.wrapping_add(0x88), (r) as u32);
              fl = f; }
            // e5bd4 test BYTE PTR [r15+0x20],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x20)) as u64 & 0x1u64);
            // e5bd9 jne 0xe5be1
            if crate::cov::brk(0xe5bd9, fl.ne()) { pc = 0xe5be1; continue; }
            pc = 0xe5bdb;
        }
        0xe5bdb => {
            // block e5bdb
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5bdb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5bdb lea rax,[r14+0x1]
            rax = r14.wrapping_add(0x1);
            // e5bdf jmp 0xe5be5
            pc = 0xe5be5; continue;
        }
        0xe5be1 => {
            // block e5be1
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5be1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5be1 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xe5be5;
        }
        0xe5be5 => {
            // block e5be5
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5be5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5be5 mov al,BYTE PTR [rax+0x6]
            rax = (rax & !0xff) | ((m.r8(rax.wrapping_add(0x6)) as u64) as u8 as u64);
            // e5be8 mov BYTE PTR [rbp-0x1a],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6), ((rax as u8 as u64)) as u8);
            // e5beb lea rsi,[rip+0x5390d] # 0x1394ff
            rsi = m.env.data_va(318668);
            // e5bf2 lea rdi,[rbp-0x1a]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe6);
            // e5bf6 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e5bfa xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5bfc call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e5c01 mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // e5c04 and eax,0x7
            { let r = (rax as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5c07 shl eax,0x18
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x18u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e5c0a or DWORD PTR [r15+0x84],eax
            { let r = m.r32(r15.wrapping_add(0x84)) as u64 | (rax as u32 as u64); let f = logic::<32>(r);
              m.w32(r15.wrapping_add(0x84), (r) as u32);
              fl = f; }
            // e5c11 test BYTE PTR [r15+0x20],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x20)) as u64 & 0x1u64);
            // e5c16 jne 0xe5c1e
            if crate::cov::brk(0xe5c16, fl.ne()) { pc = 0xe5c1e; continue; }
            pc = 0xe5c18;
        }
        0xe5c18 => {
            // block e5c18
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5c18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5c18 lea rax,[r14+0x1]
            rax = r14.wrapping_add(0x1);
            // e5c1c jmp 0xe5c22
            pc = 0xe5c22; continue;
        }
        0xe5c1e => {
            // block e5c1e
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5c1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5c1e mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xe5c22;
        }
        0xe5c22 => {
            // block e5c22
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5c22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5c22 mov al,BYTE PTR [rax+0x7]
            rax = (rax & !0xff) | ((m.r8(rax.wrapping_add(0x7)) as u64) as u8 as u64);
            // e5c25 mov BYTE PTR [rbp-0x1a],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6), ((rax as u8 as u64)) as u8);
            // e5c28 lea rsi,[rip+0x538d0] # 0x1394ff
            rsi = m.env.data_va(318668);
            // e5c2f lea rdi,[rbp-0x1a]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe6);
            // e5c33 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e5c37 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5c39 call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e5c3e mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // e5c41 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5c44 shl eax,0x10
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e5c47 or DWORD PTR [r15+0x84],eax
            { let r = m.r32(r15.wrapping_add(0x84)) as u64 | (rax as u32 as u64); let f = logic::<32>(r);
              m.w32(r15.wrapping_add(0x84), (r) as u32);
              fl = f; }
            // e5c4e mov eax,DWORD PTR [r15+0x8]
            rax = (m.r32(r15.wrapping_add(0x8)) as u64) as u32 as u64;
            // e5c52 add eax,0xfffffffb
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffffbu64);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5c55 test BYTE PTR [r15+0x20],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x20)) as u64 & 0x1u64);
            // e5c5a jne 0xe5c62
            if crate::cov::brk(0xe5c5a, fl.ne()) { pc = 0xe5c62; continue; }
            pc = 0xe5c5c;
        }
        0xe5c5c => {
            // block e5c5c
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5c5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5c5c lea rcx,[r14+0x1]
            rcx = r14.wrapping_add(0x1);
            // e5c60 jmp 0xe5c66
            pc = 0xe5c66; continue;
        }
        0xe5c62 => {
            // block e5c62
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5c62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5c62 mov rcx,QWORD PTR [r15+0x30]
            rcx = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xe5c66;
        }
        0xe5c66 => {
            // block e5c66
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5c66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5c66 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // e5c69 mov al,BYTE PTR [rcx+0x9]
            rax = (rax & !0xff) | ((m.r8(rcx.wrapping_add(0x9)) as u64) as u8 as u64);
            // e5c6c mov BYTE PTR [rbp-0x1a],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6), ((rax as u8 as u64)) as u8);
            // e5c6f lea rsi,[rip+0x53889] # 0x1394ff
            rsi = m.env.data_va(318668);
            // e5c76 lea rdi,[rbp-0x1a]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe6);
            // e5c7a jae 0xe5c92
            if crate::cov::brk(0xe5c7a, fl.ae()) { pc = 0xe5c92; continue; }
            pc = 0xe5c7c;
        }
        0xe5c7c => {
            // block e5c7c
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5c7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5c7c lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // e5c80 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5c82 call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e5c87 test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // e5c8b jne 0xe5cc0
            if crate::cov::brk(0xe5c8b, fl.ne()) { pc = 0xe5cc0; continue; }
            pc = 0xe5c8d;
        }
        0xe5c8d => {
            // block e5c8d
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5c8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5c8d inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // e5c90 jmp 0xe5cc4
            pc = 0xe5cc4; continue;
        }
        0xe5c92 => {
            // block e5c92
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5c92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5c92 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e5c96 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5c98 call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e5c9d mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // e5ca0 and eax,0x3
            { let r = (rax as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5ca3 shl eax,0x8
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e5ca6 or DWORD PTR [r15+0x84],eax
            { let r = m.r32(r15.wrapping_add(0x84)) as u64 | (rax as u32 as u64); let f = logic::<32>(r);
              m.w32(r15.wrapping_add(0x84), (r) as u32);
              fl = f; }
            // e5cad test BYTE PTR [r15+0x20],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x20)) as u64 & 0x1u64);
            // e5cb2 jne 0xe5d6d
            if crate::cov::brk(0xe5cb2, fl.ne()) { pc = 0xe5d6d; continue; }
            pc = 0xe5cb8;
        }
        0xe5cb8 => {
            // block e5cb8
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5cb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5cb8 inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // e5cbb jmp 0xe5d71
            pc = 0xe5d71; continue;
        }
        0xe5cc0 => {
            // block e5cc0
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5cc0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5cc0 mov r14,QWORD PTR [r15+0x30]
            r14 = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xe5cc4;
        }
        0xe5cc4 => {
            // block e5cc4
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5cc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5cc4 mov al,BYTE PTR [r14+0xa]
            rax = (rax & !0xff) | ((m.r8(r14.wrapping_add(0xa)) as u64) as u8 as u64);
            // e5cc8 mov BYTE PTR [rbp-0x1a],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6), ((rax as u8 as u64)) as u8);
            // e5ccb lea rsi,[rip+0x5382d] # 0x1394ff
            rsi = m.env.data_va(318668);
            // e5cd2 lea rdi,[rbp-0x1a]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe6);
            // e5cd6 lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // e5cda xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5cdc call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e5ce1 mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // e5ce4 lea ecx,[rax+rax*4]
            rcx = (rax.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // e5ce7 add ecx,ecx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rcx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e5ce9 add ecx,DWORD PTR [rbp-0x28]
            { let (r, f) = add::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e5cec mov eax,DWORD PTR [r15+0x8]
            rax = (m.r32(r15.wrapping_add(0x8)) as u64) as u32 as u64;
            // e5cf0 dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e5cf2 mov edx,0x1e
            rdx = (0x1eu64) as u32 as u64;
            // e5cf7 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // e5cfa ja 0xe5d09
            if crate::cov::brk(0xe5cfa, fl.a()) { pc = 0xe5d09; continue; }
            pc = 0xe5cfc;
        }
        0xe5cfc => {
            // block e5cfc
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5cfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5cfc movsxd rdx,eax
            rdx = sx::<32>((rax as u32 as u64)) as u64;
            // e5cff lea rsi,[rip+0x43fda] # 0x129ce0
            rsi = m.env.data_va(255149);
            // e5d06 mov edx,DWORD PTR [rsi+rdx*4]
            rdx = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe5d09;
        }
        0xe5d09 => {
            // block e5d09
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5d09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5d09 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // e5d0c seta sil
            rsi = (rsi & !0xff) | (((fl.a() as u64)) as u8 as u64);
            // e5d10 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // e5d12 mov ebx,0x1d
            rbx = (0x1du64) as u32 as u64;
            // e5d17 cmovb ebx,ecx
            if fl.b() { rbx = ((rcx as u32 as u64)) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            // e5d1a jb 0xe5d2f
            if crate::cov::brk(0xe5d1a, fl.b()) { pc = 0xe5d2f; continue; }
            pc = 0xe5d1c;
        }
        0xe5d1c => {
            // block e5d1c
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5d1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5d1c test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // e5d1f jne 0xe5d2f
            if crate::cov::brk(0xe5d1f, fl.ne()) { pc = 0xe5d2f; continue; }
            pc = 0xe5d21;
        }
        0xe5d21 => {
            // block e5d21
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5d21 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e5d23 lea rcx,[rip+0x43fb6] # 0x129ce0
            rcx = m.env.data_va(255149);
            // e5d2a mov ebx,DWORD PTR [rcx+rax*4]
            rbx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // e5d2d dec ebx
            { let (r, f) = dec::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            pc = 0xe5d2f;
        }
        0xe5d2f => {
            // block e5d2f
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5d2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5d2f mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e5d31 and esi,0x1
            { let r = (rsi as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e5d34 mov rdi,r15
            rdi = r15;
            // e5d37 call 0xe5ea8
            { let r = { let t = m.env.code_va(0xe5ea8); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e5d3c mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e5d3e shr ebx,1
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x1u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e5d40 mov ecx,0xcccccccd
            rcx = (0xcccccccdu64) as u32 as u64;
            // e5d45 imul rax,rcx
            { let (r, f) = imul::<64>(rax, rcx); rax = r; fl = f; }
            // e5d49 shr rax,0x20
            { let (r, f) = shr::<64>(rax, 0x20u64, fl); rax = r; fl = f; }
            // e5d4d and eax,0x30
            { let r = (rax as u32 as u64) & 0x30u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5d50 shl eax,0x4
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e5d53 or eax,DWORD PTR [r15+0x84]
            { let r = (rax as u32 as u64) | m.r32(r15.wrapping_add(0x84)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5d5a imul rcx,rbx
            { let (r, f) = imul::<64>(rcx, rbx); rcx = r; fl = f; }
            // e5d5e shr rcx,0x23
            { let (r, f) = shr::<64>(rcx, 0x23u64, fl); rcx = r; fl = f; }
            // e5d62 add ecx,ecx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rcx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e5d64 lea ecx,[rcx+rcx*4]
            rcx = (rcx.wrapping_add(rcx.wrapping_mul(4))) as u32 as u64;
            // e5d67 sub ebx,ecx
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rcx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // e5d69 or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e5d6b jmp 0xe5d9b
            pc = 0xe5d9b; continue;
        }
        0xe5d6d => {
            // block e5d6d
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5d6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5d6d mov r14,QWORD PTR [r15+0x30]
            r14 = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xe5d71;
        }
        0xe5d71 => {
            // block e5d71
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5d71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5d71 mov al,BYTE PTR [r14+0xa]
            rax = (rax & !0xff) | ((m.r8(r14.wrapping_add(0xa)) as u64) as u8 as u64);
            // e5d75 mov BYTE PTR [rbp-0x1a],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6), ((rax as u8 as u64)) as u8);
            // e5d78 lea rsi,[rip+0x53780] # 0x1394ff
            rsi = m.env.data_va(318668);
            // e5d7f lea rdi,[rbp-0x1a]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe6);
            // e5d83 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e5d87 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5d89 call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e5d8e mov ebx,DWORD PTR [rbp-0x20]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // e5d91 and ebx,0xf
            { let r = (rbx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e5d94 or ebx,DWORD PTR [r15+0x84]
            { let r = (rbx as u32 as u64) | m.r32(r15.wrapping_add(0x84)) as u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe5d9b;
        }
        0xe5d9b => {
            // block e5d9b
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5d9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5d9b mov DWORD PTR [r15+0x84],ebx
            m.w32(r15.wrapping_add(0x84), ((rbx as u32 as u64)) as u32);
            // e5da2 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e5da6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e5da7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e5da9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e5dab pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5dac ret
            return Ret { rax, rdx, xmm0: x0 };
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
            if crate::cov::brk(0x6340e, fl.g()) { pc = 0x63438; continue; }
            pc = 0x63410;
        }
        0x63410 => {
            // block 63410
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63410 cmp esi,0x105183ff
            fl = sub::<32>((rsi as u32 as u64), 0x105183ffu64).1;
            // 63416 jg 0x63461
            if crate::cov::brk(0x63416, fl.g()) { pc = 0x63461; continue; }
            pc = 0x63418;
        }
        0x63418 => {
            // block 63418
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63418 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63418 cmp esi,0x104020ff
            fl = sub::<32>((rsi as u32 as u64), 0x104020ffu64).1;
            // 6341e jg 0x634aa
            if crate::cov::brk(0x6341e, fl.g()) { pc = 0x634aa; continue; }
            pc = 0x63424;
        }
        0x63424 => {
            // block 63424
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63424 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63424 cmp esi,0x102447ff
            fl = sub::<32>((rsi as u32 as u64), 0x102447ffu64).1;
            // 6342a jg 0x6353a
            if crate::cov::brk(0x6342a, fl.g()) { pc = 0x6353a; continue; }
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
            if crate::cov::brk(0x6343e, fl.g()) { pc = 0x6348a; continue; }
            pc = 0x63440;
        }
        0x63440 => {
            // block 63440
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63440 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63440 cmp esi,0x106466ff
            fl = sub::<32>((rsi as u32 as u64), 0x106466ffu64).1;
            // 63446 jg 0x634ce
            if crate::cov::brk(0x63446, fl.g()) { pc = 0x634ce; continue; }
            pc = 0x6344c;
        }
        0x6344c => {
            // block 6344c
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6344c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6344c cmp esi,0x10634500
            fl = sub::<32>((rsi as u32 as u64), 0x10634500u64).1;
            // 63452 jne 0x636db
            if crate::cov::brk(0x63452, fl.ne()) { pc = 0x636db; continue; }
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
            if crate::cov::brk(0x63467, fl.g()) { pc = 0x634f9; continue; }
            pc = 0x6346d;
        }
        0x6346d => {
            // block 6346d
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6346d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6346d cmp esi,0x10518400
            fl = sub::<32>((rsi as u32 as u64), 0x10518400u64).1;
            // 63473 jne 0x6356a
            if crate::cov::brk(0x63473, fl.ne()) { pc = 0x6356a; continue; }
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
            if crate::cov::brk(0x63490, fl.e()) { pc = 0x63511; continue; }
            pc = 0x63492;
        }
        0x63492 => {
            // block 63492
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63492 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63492 cmp esi,0x10710850
            fl = sub::<32>((rsi as u32 as u64), 0x10710850u64).1;
            // 63498 jne 0x63522
            if crate::cov::brk(0x63498, fl.ne()) { pc = 0x63522; continue; }
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
            if crate::cov::brk(0x634b0, fl.g()) { pc = 0x63582; continue; }
            pc = 0x634b6;
        }
        0x634b6 => {
            // block 634b6
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 634b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 634b6 cmp esi,0x10402100
            fl = sub::<32>((rsi as u32 as u64), 0x10402100u64).1;
            // 634bc jne 0x635ae
            if crate::cov::brk(0x634bc, fl.ne()) { pc = 0x635ae; continue; }
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
            if crate::cov::brk(0x634d7, fl.a()) { pc = 0x636db; continue; }
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
            if crate::cov::brk(0x634ff, fl.ne()) { pc = 0x63596; continue; }
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
            if crate::cov::brk(0x63528, fl.ne()) { pc = 0x636db; continue; }
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
            if crate::cov::brk(0x63540, fl.g()) { pc = 0x635c6; continue; }
            pc = 0x63546;
        }
        0x63546 => {
            // block 63546
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63546 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63546 cmp esi,0x102802ff
            fl = sub::<32>((rsi as u32 as u64), 0x102802ffu64).1;
            // 6354c jg 0x635f7
            if crate::cov::brk(0x6354c, fl.g()) { pc = 0x635f7; continue; }
            pc = 0x63552;
        }
        0x63552 => {
            // block 63552
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63552 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63552 cmp esi,0x10244800
            fl = sub::<32>((rsi as u32 as u64), 0x10244800u64).1;
            // 63558 jne 0x63688
            if crate::cov::brk(0x63558, fl.ne()) { pc = 0x63688; continue; }
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
            if crate::cov::brk(0x63570, fl.ne()) { pc = 0x636db; continue; }
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
            if crate::cov::brk(0x63588, fl.ne()) { pc = 0x635e2; continue; }
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
            if crate::cov::brk(0x6359c, fl.ne()) { pc = 0x636db; continue; }
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
            if crate::cov::brk(0x635b4, fl.ne()) { pc = 0x636db; continue; }
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
            if crate::cov::brk(0x635cc, fl.e()) { pc = 0x6361b; continue; }
            pc = 0x635ce;
        }
        0x635ce => {
            // block 635ce
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 635ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 635ce cmp esi,0x10352300
            fl = sub::<32>((rsi as u32 as u64), 0x10352300u64).1;
            // 635d4 jne 0x63627
            if crate::cov::brk(0x635d4, fl.ne()) { pc = 0x63627; continue; }
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
            if crate::cov::brk(0x635e8, fl.ne()) { pc = 0x636db; continue; }
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
            if crate::cov::brk(0x635fd, fl.g()) { pc = 0x63699; continue; }
            pc = 0x63603;
        }
        0x63603 => {
            // block 63603
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63603 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63603 cmp esi,0x10280300
            fl = sub::<32>((rsi as u32 as u64), 0x10280300u64).1;
            // 63609 jne 0x636af
            if crate::cov::brk(0x63609, fl.ne()) { pc = 0x636af; continue; }
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
            if crate::cov::brk(0x6362d, fl.ne()) { pc = 0x636db; continue; }
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
            if crate::cov::brk(0x6368e, fl.ne()) { pc = 0x636db; continue; }
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
            if crate::cov::brk(0x6369f, fl.ne()) { pc = 0x636c5; continue; }
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
            if crate::cov::brk(0x636b5, fl.ne()) { pc = 0x636db; continue; }
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
            if crate::cov::brk(0x636cb, fl.ne()) { pc = 0x636db; continue; }
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
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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

/// parity: P2v @0x39c0 size=796
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000039c0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x39c0;
    loop {
        match pc {
        0x39c0 => {
            // block 39c0
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 39c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 39c0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 39c1 mov rbp,rsp
            rbp = rsp;
            // 39c4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 39c6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 39c8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 39ca push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 39cc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 39cd sub rsp,0x338
            { let (r, f) = sub::<64>(rsp, 0x338u64);
              rsp = r;
              fl = f; }
            // 39d4 mov rbx,rsi
            rbx = rsi;
            // 39d7 mov r13,rdi
            r13 = rdi;
            // 39da mov r15,QWORD PTR [rip+0x13c9f7] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 39e1 mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 39e4 mov QWORD PTR [rbp-0x30],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r15) as u64);
            // 39e8 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 39ef call 0x12b8c
            { let r = { let t = m.env.code_va(0x12b8c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 39f4 mov rdi,rbx
            rdi = rbx;
            // 39f7 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 39fc lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 3a03 mov rsi,rbx
            rsi = rbx;
            // 3a06 mov rdx,rax
            rdx = rax;
            // 3a09 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3a0e lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3a15 lea rsi,[rbp-0x340]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 3a1c call 0x12c80
            { let r = { let t = m.env.code_va(0x12c80); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3a21 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 3a23 lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 3a2a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3a2f test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // 3a31 je 0x3c1e
            if crate::cov::brk(0x3a31, fl.e()) { pc = 0x3c1e; continue; }
            pc = 0x3a37;
        }
        0x3a37 => {
            // block 3a37
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3a37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3a37 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3a3e call 0x136b6
            { let r = { let t = m.env.code_va(0x136b6); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3a43 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // 3a46 lea r12d,[r14+0x200]
            r12 = (r14.wrapping_add(0x200)) as u32 as u64;
            // 3a4d mov rdi,r12
            rdi = r12;
            // 3a50 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3a55 mov esi,0xff
            rsi = (0xffu64) as u32 as u64;
            // 3a5a mov rbx,rax
            rbx = rax;
            // 3a5d mov QWORD PTR [rbp-0x350],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcb0), (rbx) as u64);
            // 3a64 mov rdi,rbx
            rdi = rbx;
            // 3a67 mov rdx,r12
            rdx = r12;
            // 3a6a call 0xec22a
            { let r = { let t = m.env.stub_va(0x9f7); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3a6f lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3a76 mov rsi,rbx
            rsi = rbx;
            // 3a79 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 3a7c call 0x1378a
            { let r = { let t = m.env.code_va(0x1378a); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3a81 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 3a83 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3a8a call 0x12b96
            { let r = { let t = m.env.code_va(0x12b96); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3a8f cmp ebx,r14d
            fl = sub::<32>((rbx as u32 as u64), (r14 as u32 as u64)).1;
            // 3a92 jne 0x3c12
            if crate::cov::brk(0x3a92, fl.ne()) { pc = 0x3c12; continue; }
            pc = 0x3a98;
        }
        0x3a98 => {
            // block 3a98
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3a98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3a98 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3a9c mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3aa0 mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // 3aa5 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 3aaa mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3aaf xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3ab2 mov rdi,r13
            rdi = r13;
            // 3ab5 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3ab7 add r14d,0x4
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x4u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 3abb shr r14d,0x2
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x2u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 3abf je 0x3c22
            if crate::cov::brk(0x3abf, fl.e()) { pc = 0x3c22; continue; }
            pc = 0x3ac5;
        }
        0x3ac5 => {
            // block 3ac5
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3ac5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3ac5 mov QWORD PTR [rbp-0x358],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fca8), (r14) as u64);
            // 3acc lea r14,[r13+0x10]
            r14 = r13.wrapping_add(0x10);
            // 3ad0 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 3ad3 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 3ad5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x3ad8;
        }
        0x3ad8 => {
            // block 3ad8
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3ad8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3ad8 mov edi,DWORD PTR [r14]
            rdi = (m.r32(r14) as u64) as u32 as u64;
            // 3adb call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3ae0 cmp ebx,0x1000000
            fl = sub::<32>((rbx as u32 as u64), 0x1000000u64).1;
            // 3ae6 sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 3ae9 and cl,al
            { let r = (rcx as u8 as u64) & (rax as u8 as u64); let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3aeb movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 3aee cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 3af1 jne 0x3b43
            if crate::cov::brk(0x3af1, fl.ne()) { pc = 0x3b43; continue; }
            pc = 0x3af3;
        }
        0x3af3 => {
            // block 3af3
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3af3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3af3 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3af7 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3afb mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // 3b00 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 3b05 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3b0a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3b0d mov rdi,r13
            rdi = r13;
            // 3b10 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3b12 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3b16 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3b1a mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 3b1f mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 3b24 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3b29 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3b2c mov rdi,r13
            rdi = r13;
            // 3b2f call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3b31 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3b35 mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 3b3c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 3b3e mov rdi,r13
            rdi = r13;
            // 3b41 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x3b43;
        }
        0x3b43 => {
            // block 3b43
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3b43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3b43 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3b47 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3b4b mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // 3b50 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3b55 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3b58 mov rdi,r13
            rdi = r13;
            // 3b5b mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 3b5d call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3b5f mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3b63 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3b67 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 3b6c mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 3b71 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3b76 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3b79 mov rdi,r13
            rdi = r13;
            // 3b7c call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3b7e mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3b82 mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 3b89 mov rdi,r13
            rdi = r13;
            // 3b8c call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3b8e mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3b92 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // 3b96 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // 3b9b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3ba0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3ba3 mov rdi,r13
            rdi = r13;
            // 3ba6 lea rdx,[rbp-0x344]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fcbc);
            // 3bad call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3baf mov eax,DWORD PTR [rbp-0x344]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcbc)) as u64) as u32 as u64;
            // 3bb5 mov rcx,QWORD PTR [rbp-0x350]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcb0)) as u64;
            // 3bbc cmp eax,DWORD PTR [rcx+r15*4]
            fl = sub::<32>((rax as u32 as u64), m.r32(rcx.wrapping_add(r15.wrapping_mul(4))) as u64).1;
            // 3bc0 je 0x3bcb
            if crate::cov::brk(0x3bc0, fl.e()) { pc = 0x3bcb; continue; }
            pc = 0x3bc2;
        }
        0x3bc2 => {
            // block 3bc2
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3bc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3bc2 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // 3bc5 cmp r12d,0x1
            fl = sub::<32>((r12 as u32 as u64), 0x1u64).1;
            // 3bc9 ja 0x3c06
            if crate::cov::brk(0x3bc9, fl.a()) { pc = 0x3c06; continue; }
            pc = 0x3bcb;
        }
        0x3bcb => {
            // block 3bcb
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3bcb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3bcb mov edx,DWORD PTR [r13+0x1c]
            rdx = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 3bcf add edx,0x4
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x4u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 3bd2 mov DWORD PTR [r13+0x1c],edx
            m.w32(r13.wrapping_add(0x1c), ((rdx as u32 as u64)) as u32);
            // 3bd6 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3bda mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3bde mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // 3be3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3be8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3beb mov rdi,r13
            rdi = r13;
            // 3bee call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3bf0 add ebx,0x4
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x4u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 3bf3 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 3bf6 mov rax,QWORD PTR [rbp-0x358]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fca8)) as u64;
            // 3bfd cmp r15d,eax
            fl = sub::<32>((r15 as u32 as u64), (rax as u32 as u64)).1;
            // 3c00 jb 0x3ad8
            if crate::cov::brk(0x3c00, fl.b()) { pc = 0x3ad8; continue; }
            pc = 0x3c06;
        }
        0x3c06 => {
            // block 3c06
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3c06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3c06 mov r15,QWORD PTR [rip+0x13c7cb] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 3c0d mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 3c10 jmp 0x3c29
            pc = 0x3c29; continue;
        }
        0x3c12 => {
            // block 3c12
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3c12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3c12 mov rdi,QWORD PTR [rbp-0x350]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcb0)) as u64;
            // 3c19 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x3c1e;
        }
        0x3c1e => {
            // block 3c1e
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3c1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3c1e xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 3c20 jmp 0x3c86
            pc = 0x3c86; continue;
        }
        0x3c22 => {
            // block 3c22
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3c22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3c22 lea r14,[r13+0x10]
            r14 = r13.wrapping_add(0x10);
            // 3c26 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x3c29;
        }
        0x3c29 => {
            // block 3c29
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3c29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3c29 mov edi,DWORD PTR [r14]
            rdi = (m.r32(r14) as u64) as u32 as u64;
            // 3c2c call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3c31 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 3c33 je 0x3c80
            if crate::cov::brk(0x3c33, fl.e()) { pc = 0x3c80; continue; }
            pc = 0x3c35;
        }
        0x3c35 => {
            // block 3c35
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3c35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3c35 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3c39 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3c3d mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // 3c42 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 3c44 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3c49 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3c4c mov rdi,r13
            rdi = r13;
            // 3c4f call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3c51 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3c55 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 3c59 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 3c5e mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 3c63 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3c68 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3c6b mov rdi,r13
            rdi = r13;
            // 3c6e call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3c70 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3c74 mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 3c7b mov rdi,r13
            rdi = r13;
            // 3c7e call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x3c80;
        }
        0x3c80 => {
            // block 3c80
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3c80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3c80 test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // 3c83 sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x3c86;
        }
        0x3c86 => {
            // block 3c86
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3c86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3c86 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3c8d call 0x14c0c
            { let r = { let t = m.env.code_va(0x14c0c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3c92 cmp r15,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 3c96 jne 0x3cd6
            if crate::cov::brk(0x3c96, fl.ne()) { pc = 0x3cd6; continue; }
            pc = 0x3c98;
        }
        0x3c98 => {
            // block 3c98
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3c98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3c98 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 3c9a add rsp,0x338
            { let (r, f) = add::<64>(rsp, 0x338u64);
              rsp = r;
              fl = f; }
            // 3ca1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3ca2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 3ca4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 3ca6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3ca8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 3caa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3cab ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x3cac => {
            // block 3cac
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3cac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3cac mov rbx,rax
            rbx = rax;
            pc = 0x3caf;
        }
        0x3caf => {
            // block 3caf
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3caf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3caf lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3cb6 call 0x14c0c
            { let r = { let t = m.env.code_va(0x14c0c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3cbb mov rdi,rbx
            rdi = rbx;
            // 3cbe call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3cc3 jmp 0x3cac
            pc = 0x3cac; continue;
        }
        0x3cc5 => {
            // block 3cc5
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3cc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3cc5 mov rbx,rax
            rbx = rax;
            // 3cc8 lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 3ccf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3cd4 jmp 0x3caf
            pc = 0x3caf; continue;
        }
        0x3cd6 => {
            // block 3cd6
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3cd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3cd6 call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe75e6 size=806
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e75e6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe75e6;
    loop {
        match pc {
        0xe75e6 => {
            // block e75e6
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e75e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e75e6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e75e7 mov rbp,rsp
            rbp = rsp;
            // e75ea cmp edi,0x106344ff
            fl = sub::<32>((rdi as u32 as u64), 0x106344ffu64).1;
            // e75f0 jg 0xe761b
            if crate::cov::brk(0xe75f0, fl.g()) { pc = 0xe761b; continue; }
            pc = 0xe75f2;
        }
        0xe75f2 => {
            // block e75f2
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e75f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e75f2 cmp edi,0x105183ff
            fl = sub::<32>((rdi as u32 as u64), 0x105183ffu64).1;
            // e75f8 jg 0xe7647
            if crate::cov::brk(0xe75f8, fl.g()) { pc = 0xe7647; continue; }
            pc = 0xe75fa;
        }
        0xe75fa => {
            // block e75fa
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e75fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e75fa cmp edi,0x102447ff
            fl = sub::<32>((rdi as u32 as u64), 0x102447ffu64).1;
            // e7600 jg 0xe768f
            if crate::cov::brk(0xe7600, fl.g()) { pc = 0xe768f; continue; }
            pc = 0xe7606;
        }
        0xe7606 => {
            // block e7606
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7606 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7606 cmp edi,0xffffffff
            fl = sub::<32>((rdi as u32 as u64), 0xffffffffu64).1;
            // e7609 jne 0xe78e4
            if crate::cov::brk(0xe7609, fl.ne()) { pc = 0xe78e4; continue; }
            pc = 0xe760f;
        }
        0xe760f => {
            // block e760f
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e760f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e760f lea rax,[rip+0x5217f] # 0x139795
            rax = m.env.data_va(319330);
            // e7616 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe761b => {
            // block e761b
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e761b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e761b cmp edi,0x106681ff
            fl = sub::<32>((rdi as u32 as u64), 0x106681ffu64).1;
            // e7621 jg 0xe766b
            if crate::cov::brk(0xe7621, fl.g()) { pc = 0xe766b; continue; }
            pc = 0xe7623;
        }
        0xe7623 => {
            // block e7623
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7623 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7623 cmp edi,0x106466ff
            fl = sub::<32>((rdi as u32 as u64), 0x106466ffu64).1;
            // e7629 jg 0xe76cb
            if crate::cov::brk(0xe7629, fl.g()) { pc = 0xe76cb; continue; }
            pc = 0xe762f;
        }
        0xe762f => {
            // block e762f
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e762f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e762f cmp edi,0x10634500
            fl = sub::<32>((rdi as u32 as u64), 0x10634500u64).1;
            // e7635 jne 0xe78e4
            if crate::cov::brk(0xe7635, fl.ne()) { pc = 0xe78e4; continue; }
            pc = 0xe763b;
        }
        0xe763b => {
            // block e763b
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e763b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e763b lea rax,[rip+0x5202d] # 0x13966f
            rax = m.env.data_va(319036);
            // e7642 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7647 => {
            // block e7647
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7647 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7647 cmp edi,0x105381ff
            fl = sub::<32>((rdi as u32 as u64), 0x105381ffu64).1;
            // e764d jg 0xe76f6
            if crate::cov::brk(0xe764d, fl.g()) { pc = 0xe76f6; continue; }
            pc = 0xe7653;
        }
        0xe7653 => {
            // block e7653
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7653 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7653 cmp edi,0x10518400
            fl = sub::<32>((rdi as u32 as u64), 0x10518400u64).1;
            // e7659 jne 0xe7726
            if crate::cov::brk(0xe7659, fl.ne()) { pc = 0xe7726; continue; }
            pc = 0xe765f;
        }
        0xe765f => {
            // block e765f
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e765f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e765f lea rax,[rip+0x51fc0] # 0x139626
            rax = m.env.data_va(318963);
            // e7666 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe766b => {
            // block e766b
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e766b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e766b cmp edi,0x1071084f
            fl = sub::<32>((rdi as u32 as u64), 0x1071084fu64).1;
            // e7671 jg 0xe770e
            if crate::cov::brk(0xe7671, fl.g()) { pc = 0xe770e; continue; }
            pc = 0xe7677;
        }
        0xe7677 => {
            // block e7677
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7677 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7677 cmp edi,0x10668200
            fl = sub::<32>((rdi as u32 as u64), 0x10668200u64).1;
            // e767d jne 0xe773e
            if crate::cov::brk(0xe767d, fl.ne()) { pc = 0xe773e; continue; }
            pc = 0xe7683;
        }
        0xe7683 => {
            // block e7683
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7683 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7683 lea rax,[rip+0x51ffa] # 0x139684
            rax = m.env.data_va(319057);
            // e768a jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe768f => {
            // block e768f
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e768f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e768f cmp edi,0x1032294f
            fl = sub::<32>((rdi as u32 as u64), 0x1032294fu64).1;
            // e7695 jg 0xe7756
            if crate::cov::brk(0xe7695, fl.g()) { pc = 0xe7756; continue; }
            pc = 0xe769b;
        }
        0xe769b => {
            // block e769b
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e769b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e769b cmp edi,0x102802ff
            fl = sub::<32>((rdi as u32 as u64), 0x102802ffu64).1;
            // e76a1 jg 0xe77b2
            if crate::cov::brk(0xe76a1, fl.g()) { pc = 0xe77b2; continue; }
            pc = 0xe76a7;
        }
        0xe76a7 => {
            // block e76a7
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76a7 cmp edi,0x10244800
            fl = sub::<32>((rdi as u32 as u64), 0x10244800u64).1;
            // e76ad je 0xe7822
            if crate::cov::brk(0xe76ad, fl.e()) { pc = 0xe7822; continue; }
            pc = 0xe76b3;
        }
        0xe76b3 => {
            // block e76b3
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76b3 cmp edi,0x10266400
            fl = sub::<32>((rdi as u32 as u64), 0x10266400u64).1;
            // e76b9 jne 0xe782e
            if crate::cov::brk(0xe76b9, fl.ne()) { pc = 0xe782e; continue; }
            pc = 0xe76bf;
        }
        0xe76bf => {
            // block e76bf
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76bf lea rax,[rip+0x51e5f] # 0x139525
            rax = m.env.data_va(318706);
            // e76c6 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe76cb => {
            // block e76cb
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76cb add edi,0xef9b9900
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xef9b9900u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // e76d1 cmp edi,0x6
            fl = sub::<32>((rdi as u32 as u64), 0x6u64).1;
            // e76d4 ja 0xe78e4
            if crate::cov::brk(0xe76d4, fl.a()) { pc = 0xe78e4; continue; }
            pc = 0xe76da;
        }
        0xe76da => {
            // block e76da
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76da lea rax,[rip+0x20f] # 0xe78f0
            rax = m.env.code_va(0xe78f0);
            // e76e1 movsxd rcx,DWORD PTR [rax+rdi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // e76e5 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // e76e8 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xe76ea => {
            // block e76ea
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76ea lea rax,[rip+0x51fa7] # 0x139698
            rax = m.env.data_va(319077);
            // e76f1 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe76f6 => {
            // block e76f6
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e76f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e76f6 cmp edi,0x10538200
            fl = sub::<32>((rdi as u32 as u64), 0x10538200u64).1;
            // e76fc jne 0xe7782
            if crate::cov::brk(0xe76fc, fl.ne()) { pc = 0xe7782; continue; }
            pc = 0xe7702;
        }
        0xe7702 => {
            // block e7702
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7702 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7702 lea rax,[rip+0x51f40] # 0x139649
            rax = m.env.data_va(318998);
            // e7709 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe770e => {
            // block e770e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e770e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e770e cmp edi,0x10710850
            fl = sub::<32>((rdi as u32 as u64), 0x10710850u64).1;
            // e7714 jne 0xe779a
            if crate::cov::brk(0xe7714, fl.ne()) { pc = 0xe779a; continue; }
            pc = 0xe771a;
        }
        0xe771a => {
            // block e771a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e771a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e771a lea rax,[rip+0x5204c] # 0x13976d
            rax = m.env.data_va(319290);
            // e7721 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7726 => {
            // block e7726
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7726 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7726 cmp edi,0x10518450
            fl = sub::<32>((rdi as u32 as u64), 0x10518450u64).1;
            // e772c jne 0xe78e4
            if crate::cov::brk(0xe772c, fl.ne()) { pc = 0xe78e4; continue; }
            pc = 0xe7732;
        }
        0xe7732 => {
            // block e7732
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7732 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7732 lea rax,[rip+0x51efd] # 0x139636
            rax = m.env.data_va(318979);
            // e7739 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe773e => {
            // block e773e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e773e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e773e cmp edi,0x10710800
            fl = sub::<32>((rdi as u32 as u64), 0x10710800u64).1;
            // e7744 jne 0xe78e4
            if crate::cov::brk(0xe7744, fl.ne()) { pc = 0xe78e4; continue; }
            pc = 0xe774a;
        }
        0xe774a => {
            // block e774a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e774a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e774a lea rax,[rip+0x52009] # 0x13975a
            rax = m.env.data_va(319271);
            // e7751 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7756 => {
            // block e7756
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7756 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7756 cmp edi,0x104020ff
            fl = sub::<32>((rdi as u32 as u64), 0x104020ffu64).1;
            // e775c jg 0xe77d2
            if crate::cov::brk(0xe775c, fl.g()) { pc = 0xe77d2; continue; }
            pc = 0xe775e;
        }
        0xe775e => {
            // block e775e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e775e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e775e cmp edi,0x10322950
            fl = sub::<32>((rdi as u32 as u64), 0x10322950u64).1;
            // e7764 je 0xe7846
            if crate::cov::brk(0xe7764, fl.e()) { pc = 0xe7846; continue; }
            pc = 0xe776a;
        }
        0xe776a => {
            // block e776a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e776a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e776a cmp edi,0x10352300
            fl = sub::<32>((rdi as u32 as u64), 0x10352300u64).1;
            // e7770 jne 0xe7852
            if crate::cov::brk(0xe7770, fl.ne()) { pc = 0xe7852; continue; }
            pc = 0xe7776;
        }
        0xe7776 => {
            // block e7776
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7776 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7776 lea rax,[rip+0x51e41] # 0x1395be
            rax = m.env.data_va(318859);
            // e777d jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7782 => {
            // block e7782
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7782 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7782 cmp edi,0x10565400
            fl = sub::<32>((rdi as u32 as u64), 0x10565400u64).1;
            // e7788 jne 0xe78e4
            if crate::cov::brk(0xe7788, fl.ne()) { pc = 0xe78e4; continue; }
            pc = 0xe778e;
        }
        0xe778e => {
            // block e778e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e778e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e778e lea rax,[rip+0x51ec7] # 0x13965c
            rax = m.env.data_va(319017);
            // e7795 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe779a => {
            // block e779a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e779a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e779a cmp edi,0x10710851
            fl = sub::<32>((rdi as u32 as u64), 0x10710851u64).1;
            // e77a0 jne 0xe78e4
            if crate::cov::brk(0xe77a0, fl.ne()) { pc = 0xe78e4; continue; }
            pc = 0xe77a6;
        }
        0xe77a6 => {
            // block e77a6
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77a6 lea rax,[rip+0x51fd4] # 0x139781
            rax = m.env.data_va(319310);
            // e77ad jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe77b2 => {
            // block e77b2
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77b2 cmp edi,0x102946ff
            fl = sub::<32>((rdi as u32 as u64), 0x102946ffu64).1;
            // e77b8 jg 0xe77f2
            if crate::cov::brk(0xe77b8, fl.g()) { pc = 0xe77f2; continue; }
            pc = 0xe77ba;
        }
        0xe77ba => {
            // block e77ba
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77ba cmp edi,0x10280300
            fl = sub::<32>((rdi as u32 as u64), 0x10280300u64).1;
            // e77c0 jne 0xe78a0
            if crate::cov::brk(0xe77c0, fl.ne()) { pc = 0xe78a0; continue; }
            pc = 0xe77c6;
        }
        0xe77c6 => {
            // block e77c6
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77c6 lea rax,[rip+0x51d7f] # 0x13954c
            rax = m.env.data_va(318745);
            // e77cd jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe77d2 => {
            // block e77d2
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77d2 cmp edi,0x104782ff
            fl = sub::<32>((rdi as u32 as u64), 0x104782ffu64).1;
            // e77d8 jg 0xe780a
            if crate::cov::brk(0xe77d8, fl.g()) { pc = 0xe780a; continue; }
            pc = 0xe77da;
        }
        0xe77da => {
            // block e77da
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77da cmp edi,0x10402100
            fl = sub::<32>((rdi as u32 as u64), 0x10402100u64).1;
            // e77e0 jne 0xe78b1
            if crate::cov::brk(0xe77e0, fl.ne()) { pc = 0xe78b1; continue; }
            pc = 0xe77e6;
        }
        0xe77e6 => {
            // block e77e6
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77e6 lea rax,[rip+0x51df6] # 0x1395e3
            rax = m.env.data_va(318896);
            // e77ed jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe77f2 => {
            // block e77f2
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77f2 cmp edi,0x10294700
            fl = sub::<32>((rdi as u32 as u64), 0x10294700u64).1;
            // e77f8 jne 0xe78c2
            if crate::cov::brk(0xe77f8, fl.ne()) { pc = 0xe78c2; continue; }
            pc = 0xe77fe;
        }
        0xe77fe => {
            // block e77fe
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e77fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e77fe lea rax,[rip+0x51d93] # 0x139598
            rax = m.env.data_va(318821);
            // e7805 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe780a => {
            // block e780a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e780a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e780a cmp edi,0x10478300
            fl = sub::<32>((rdi as u32 as u64), 0x10478300u64).1;
            // e7810 jne 0xe78d3
            if crate::cov::brk(0xe7810, fl.ne()) { pc = 0xe78d3; continue; }
            pc = 0xe7816;
        }
        0xe7816 => {
            // block e7816
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7816 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7816 lea rax,[rip+0x51de8] # 0x139605
            rax = m.env.data_va(318930);
            // e781d jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7822 => {
            // block e7822
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7822 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7822 lea rax,[rip+0x51d37] # 0x139560
            rax = m.env.data_va(318765);
            // e7829 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe782e => {
            // block e782e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e782e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e782e cmp edi,0x10266401
            fl = sub::<32>((rdi as u32 as u64), 0x10266401u64).1;
            // e7834 jne 0xe78e4
            if crate::cov::brk(0xe7834, fl.ne()) { pc = 0xe78e4; continue; }
            pc = 0xe783a;
        }
        0xe783a => {
            // block e783a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e783a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e783a lea rax,[rip+0x51cf6] # 0x139537
            rax = m.env.data_va(318724);
            // e7841 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7846 => {
            // block e7846
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7846 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7846 lea rax,[rip+0x51d5c] # 0x1395a9
            rax = m.env.data_va(318838);
            // e784d jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7852 => {
            // block e7852
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7852 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7852 cmp edi,0x10378800
            fl = sub::<32>((rdi as u32 as u64), 0x10378800u64).1;
            // e7858 jne 0xe78e4
            if crate::cov::brk(0xe7858, fl.ne()) { pc = 0xe78e4; continue; }
            pc = 0xe785e;
        }
        0xe785e => {
            // block e785e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e785e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e785e lea rax,[rip+0x51d6f] # 0x1395d4
            rax = m.env.data_va(318881);
            // e7865 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe786a => {
            // block e786a
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e786a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e786a lea rax,[rip+0x51e3d] # 0x1396ae
            rax = m.env.data_va(319099);
            // e7871 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7873 => {
            // block e7873
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7873 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7873 lea rax,[rip+0x51e4e] # 0x1396c8
            rax = m.env.data_va(319125);
            // e787a jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe787c => {
            // block e787c
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e787c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e787c lea rax,[rip+0x51e67] # 0x1396ea
            rax = m.env.data_va(319159);
            // e7883 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7885 => {
            // block e7885
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7885 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7885 lea rax,[rip+0x51e7c] # 0x139708
            rax = m.env.data_va(319189);
            // e788c jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe788e => {
            // block e788e
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e788e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e788e lea rax,[rip+0x51e91] # 0x139726
            rax = m.env.data_va(319219);
            // e7895 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe7897 => {
            // block e7897
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e7897 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7897 lea rax,[rip+0x51ea6] # 0x139744
            rax = m.env.data_va(319249);
            // e789e jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe78a0 => {
            // block e78a0
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78a0 cmp edi,0x10293000
            fl = sub::<32>((rdi as u32 as u64), 0x10293000u64).1;
            // e78a6 jne 0xe78e4
            if crate::cov::brk(0xe78a6, fl.ne()) { pc = 0xe78e4; continue; }
            pc = 0xe78a8;
        }
        0xe78a8 => {
            // block e78a8
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78a8 lea rax,[rip+0x51cc3] # 0x139572
            rax = m.env.data_va(318783);
            // e78af jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe78b1 => {
            // block e78b1
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78b1 cmp edi,0x10416000
            fl = sub::<32>((rdi as u32 as u64), 0x10416000u64).1;
            // e78b7 jne 0xe78e4
            if crate::cov::brk(0xe78b7, fl.ne()) { pc = 0xe78e4; continue; }
            pc = 0xe78b9;
        }
        0xe78b9 => {
            // block e78b9
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78b9 lea rax,[rip+0x51d36] # 0x1395f6
            rax = m.env.data_va(318915);
            // e78c0 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe78c2 => {
            // block e78c2
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78c2 cmp edi,0x10294900
            fl = sub::<32>((rdi as u32 as u64), 0x10294900u64).1;
            // e78c8 jne 0xe78e4
            if crate::cov::brk(0xe78c8, fl.ne()) { pc = 0xe78e4; continue; }
            pc = 0xe78ca;
        }
        0xe78ca => {
            // block e78ca
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78ca lea rax,[rip+0x51cb4] # 0x139585
            rax = m.env.data_va(318802);
            // e78d1 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe78d3 => {
            // block e78d3
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78d3 cmp edi,0x10478350
            fl = sub::<32>((rdi as u32 as u64), 0x10478350u64).1;
            // e78d9 jne 0xe78e4
            if crate::cov::brk(0xe78d9, fl.ne()) { pc = 0xe78e4; continue; }
            pc = 0xe78db;
        }
        0xe78db => {
            // block e78db
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78db lea rax,[rip+0x51d32] # 0x139614
            rax = m.env.data_va(318945);
            // e78e2 jmp 0xe78eb
            pc = 0xe78eb; continue;
        }
        0xe78e4 => {
            // block e78e4
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78e4 lea rax,[rip+0x513dd] # 0x138cc8
            rax = m.env.data_va(316565);
            pc = 0xe78eb;
        }
        0xe78eb => {
            // block e78eb
            #[cfg(feature = "trace")] eprintln!("f_000e75e6 pc e78eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e78eb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e78ec ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2912e size=828
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002912e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2912e;
    loop {
        match pc {
        0x2912e => {
            // block 2912e
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 2912e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2912e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2912f mov rbp,rsp
            rbp = rsp;
            // 29132 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 29134 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 29136 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 29138 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2913a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2913b sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2913f mov r15,rcx
            r15 = rcx;
            // 29142 mov r13,rdx
            r13 = rdx;
            // 29145 mov eax,DWORD PTR [rdi+0x8]
            rax = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 29148 mov DWORD PTR [r15],eax
            m.w32(r15, ((rax as u32 as u64)) as u32);
            // 2914b mov DWORD PTR [r13+0x0],eax
            m.w32(r13, ((rax as u32 as u64)) as u32);
            // 2914f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 29152 test rax,rax
            fl = logic::<64>(rax & rax);
            // 29155 je 0x29181
            if crate::cov::brk(0x29155, fl.e()) { pc = 0x29181; continue; }
            pc = 0x29157;
        }
        0x29157 => {
            // block 29157
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29157 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29157 mov edx,DWORD PTR [rdi+0x8]
            rdx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 2915a test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 2915c je 0x29181
            if crate::cov::brk(0x2915c, fl.e()) { pc = 0x29181; continue; }
            pc = 0x2915e;
        }
        0x2915e => {
            // block 2915e
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 2915e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2915e mov r12,QWORD PTR [rsi]
            r12 = m.r64(rsi) as u64;
            // 29161 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 29164 je 0x29181
            if crate::cov::brk(0x29164, fl.e()) { pc = 0x29181; continue; }
            pc = 0x29166;
        }
        0x29166 => {
            // block 29166
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29166 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29166 mov esi,DWORD PTR [rsi+0x8]
            rsi = (m.r32(rsi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 29169 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 2916c test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 2916e je 0x29184
            if crate::cov::brk(0x2916e, fl.e()) { pc = 0x29184; continue; }
            pc = 0x29170;
        }
        0x29170 => {
            // block 29170
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29170 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29170 cmp esi,edx
            fl = sub::<32>((rsi as u32 as u64), (rdx as u32 as u64)).1;
            // 29172 jne 0x29184
            if crate::cov::brk(0x29172, fl.ne()) { pc = 0x29184; continue; }
            pc = 0x29174;
        }
        0x29174 => {
            // block 29174
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29174 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29174 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 29177 cmp r12,rax
            fl = sub::<64>(r12, rax).1;
            // 2917a je 0x29184
            if crate::cov::brk(0x2917a, fl.e()) { pc = 0x29184; continue; }
            pc = 0x2917c;
        }
        0x2917c => {
            // block 2917c
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 2917c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2917c cmp edx,0x3
            fl = sub::<32>((rdx as u32 as u64), 0x3u64).1;
            // 2917f jae 0x29196
            if crate::cov::brk(0x2917f, fl.ae()) { pc = 0x29196; continue; }
            pc = 0x29181;
        }
        0x29181 => {
            // block 29181
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29181 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29181 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x29184;
        }
        0x29184 => {
            // block 29184
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29184 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29184 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 29187 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2918b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2918c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2918e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 29190 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 29192 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 29194 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29195 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x29196 => {
            // block 29196
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29196 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29196 mov DWORD PTR [r13+0x0],0x0
            m.w32(r13, (0x0u64) as u32);
            // 2919e mov ebx,DWORD PTR [rdi+0x8]
            rbx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 291a1 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 291a3 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 291a5 je 0x29184
            if crate::cov::brk(0x291a5, fl.e()) { pc = 0x29184; continue; }
            pc = 0x291a7;
        }
        0x291a7 => {
            // block 291a7
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 291a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 291a7 movzx edx,BYTE PTR [r12+rsi*1]
            rdx = (m.r8(r12.wrapping_add(rsi)) as u64) as u32 as u64;
            // 291ac movzx ecx,BYTE PTR [rax+rsi*1]
            rcx = (m.r8(rax.wrapping_add(rsi)) as u64) as u32 as u64;
            // 291b0 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 291b2 jne 0x291c5
            if crate::cov::brk(0x291b2, fl.ne()) { pc = 0x291c5; continue; }
            pc = 0x291b4;
        }
        0x291b4 => {
            // block 291b4
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 291b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 291b4 lea ecx,[rsi+0x1]
            rcx = (rsi.wrapping_add(0x1)) as u32 as u64;
            // 291b7 mov DWORD PTR [r13+0x0],ecx
            m.w32(r13, ((rcx as u32 as u64)) as u32);
            // 291bb mov ebx,DWORD PTR [rdi+0x8]
            rbx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 291be inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 291c1 cmp esi,ebx
            fl = sub::<32>((rsi as u32 as u64), (rbx as u32 as u64)).1;
            // 291c3 jb 0x291a7
            if crate::cov::brk(0x291c3, fl.b()) { pc = 0x291a7; continue; }
            pc = 0x291c5;
        }
        0x291c5 => {
            // block 291c5
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 291c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 291c5 lea r8,[r12+rsi*1]
            r8 = r12.wrapping_add(rsi);
            // 291c9 lea r9,[rax+rsi*1]
            r9 = rax.wrapping_add(rsi);
            // 291cd test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 291cf jne 0x29210
            if crate::cov::brk(0x291cf, fl.ne()) { pc = 0x29210; continue; }
            pc = 0x291d1;
        }
        0x291d1 => {
            // block 291d1
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 291d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 291d1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 291d3 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 291d5 mov esi,0x0
            rsi = (0x0u64) as u32 as u64;
            // 291da je 0x291ff
            if crate::cov::brk(0x291da, fl.e()) { pc = 0x291ff; continue; }
            pc = 0x291dc;
        }
        0x291dc => {
            // block 291dc
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 291dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 291dc movzx ecx,BYTE PTR [r8+rdx*1]
            rcx = (m.r8(r8.wrapping_add(rdx)) as u64) as u32 as u64;
            // 291e1 movzx esi,BYTE PTR [r9+rdx*1]
            rsi = (m.r8(r9.wrapping_add(rdx)) as u64) as u32 as u64;
            // 291e6 cmp esi,ecx
            fl = sub::<32>((rsi as u32 as u64), (rcx as u32 as u64)).1;
            // 291e8 je 0x291fb
            if crate::cov::brk(0x291e8, fl.e()) { pc = 0x291fb; continue; }
            pc = 0x291ea;
        }
        0x291ea => {
            // block 291ea
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 291ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 291ea lea ecx,[rdx+0x1]
            rcx = (rdx.wrapping_add(0x1)) as u32 as u64;
            // 291ed mov DWORD PTR [r13+0x0],ecx
            m.w32(r13, ((rcx as u32 as u64)) as u32);
            // 291f1 mov ebx,DWORD PTR [rdi+0x8]
            rbx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 291f4 inc rdx
            { let (r, f) = inc::<64>(rdx, fl); rdx = r; fl = f; }
            // 291f7 cmp edx,ebx
            fl = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64)).1;
            // 291f9 jb 0x291dc
            if crate::cov::brk(0x291f9, fl.b()) { pc = 0x291dc; continue; }
            pc = 0x291fb;
        }
        0x291fb => {
            // block 291fb
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 291fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 291fb mov esi,edx
            rsi = ((rdx as u32 as u64)) as u32 as u64;
            // 291fd mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            pc = 0x291ff;
        }
        0x291ff => {
            // block 291ff
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 291ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 291ff cmp esi,edx
            fl = sub::<32>((rsi as u32 as u64), (rdx as u32 as u64)).1;
            // 29201 jae 0x2920e
            if crate::cov::brk(0x29201, fl.ae()) { pc = 0x2920e; continue; }
            pc = 0x29203;
        }
        0x29203 => {
            // block 29203
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29203 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29203 dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 29205 mov DWORD PTR [r13+0x0],esi
            m.w32(r13, ((rsi as u32 as u64)) as u32);
            // 29209 mov ebx,DWORD PTR [rdi+0x8]
            rbx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 2920c jmp 0x29210
            pc = 0x29210; continue;
        }
        0x2920e => {
            // block 2920e
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 2920e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2920e mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x29210;
        }
        0x29210 => {
            // block 29210
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29210 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29210 cmp esi,ebx
            fl = sub::<32>((rsi as u32 as u64), (rbx as u32 as u64)).1;
            // 29212 je 0x29184
            if crate::cov::brk(0x29212, fl.e()) { pc = 0x29184; continue; }
            pc = 0x29218;
        }
        0x29218 => {
            // block 29218
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29218 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29218 dec ebx
            { let (r, f) = dec::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 2921a mov ecx,DWORD PTR [r15]
            rcx = (m.r32(r15) as u64) as u32 as u64;
            // 2921d mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 2921f dec edx
            { let (r, f) = dec::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // 29221 mov DWORD PTR [r15],edx
            m.w32(r15, ((rdx as u32 as u64)) as u32);
            // 29224 je 0x29268
            if crate::cov::brk(0x29224, fl.e()) { pc = 0x29268; continue; }
            pc = 0x29226;
        }
        0x29226 => {
            // block 29226
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29226 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29226 lea r9d,[rcx-0x2]
            r9 = (rcx.wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // 2922a mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 2922e lea r10,[r12+rbx*1]
            r10 = r12.wrapping_add(rbx);
            // 29232 lea r11,[rax+rbx*1]
            r11 = rax.wrapping_add(rbx);
            // 29236 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x29238;
        }
        0x29238 => {
            // block 29238
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29238 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29238 movzx ecx,BYTE PTR [r10+rsi*1]
            rcx = (m.r8(r10.wrapping_add(rsi)) as u64) as u32 as u64;
            // 2923d movzx r8d,BYTE PTR [r11+rsi*1]
            r8 = (m.r8(r11.wrapping_add(rsi)) as u64) as u32 as u64;
            // 29242 cmp r8d,ecx
            fl = sub::<32>((r8 as u32 as u64), (rcx as u32 as u64)).1;
            // 29245 jne 0x29273
            if crate::cov::brk(0x29245, fl.ne()) { pc = 0x29273; continue; }
            pc = 0x29247;
        }
        0x29247 => {
            // block 29247
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29247 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29247 lea ecx,[r9+rsi*1]
            rcx = (r9.wrapping_add(rsi)) as u32 as u64;
            // 2924b mov DWORD PTR [r15],ecx
            m.w32(r15, ((rcx as u32 as u64)) as u32);
            // 2924e dec rsi
            { let (r, f) = dec::<64>(rsi, fl); rsi = r; fl = f; }
            // 29251 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 29253 add ecx,esi
            { let (r, f) = add::<32>((rcx as u32 as u64), (rsi as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // 29255 jne 0x29238
            if crate::cov::brk(0x29255, fl.ne()) { pc = 0x29238; continue; }
            pc = 0x29257;
        }
        0x29257 => {
            // block 29257
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29257 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29257 add r12,rbx
            { let (r, f) = add::<64>(r12, rbx);
              r12 = r;
              fl = f; }
            // 2925a add r12,rsi
            { let (r, f) = add::<64>(r12, rsi);
              r12 = r;
              fl = f; }
            // 2925d add rax,rbx
            { let (r, f) = add::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // 29260 add rax,rsi
            { let (r, f) = add::<64>(rax, rsi);
              rax = r;
              fl = f; }
            // 29263 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 29266 jmp 0x29288
            pc = 0x29288; continue;
        }
        0x29268 => {
            // block 29268
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29268 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29268 add rax,rbx
            { let (r, f) = add::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // 2926b add r12,rbx
            { let (r, f) = add::<64>(r12, rbx);
              r12 = r;
              fl = f; }
            // 2926e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 29271 jmp 0x29288
            pc = 0x29288; continue;
        }
        0x29273 => {
            // block 29273
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29273 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29273 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 29277 lea r8d,[rcx+rsi*1-0x1]
            r8 = (rcx.wrapping_add(rsi).wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 2927c add r12,rbx
            { let (r, f) = add::<64>(r12, rbx);
              r12 = r;
              fl = f; }
            // 2927f add r12,rsi
            { let (r, f) = add::<64>(r12, rsi);
              r12 = r;
              fl = f; }
            // 29282 add rax,rbx
            { let (r, f) = add::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // 29285 add rax,rsi
            { let (r, f) = add::<64>(rax, rsi);
              rax = r;
              fl = f; }
            pc = 0x29288;
        }
        0x29288 => {
            // block 29288
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29288 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29288 mov ecx,DWORD PTR [rdi+0x8]
            rcx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 2928b dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 2928d cmp r8d,ecx
            fl = sub::<32>((r8 as u32 as u64), (rcx as u32 as u64)).1;
            // 29290 jne 0x29184
            if crate::cov::brk(0x29290, fl.ne()) { pc = 0x29184; continue; }
            pc = 0x29296;
        }
        0x29296 => {
            // block 29296
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29296 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29296 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 29298 test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // 2929b je 0x292bc
            if crate::cov::brk(0x2929b, fl.e()) { pc = 0x292bc; continue; }
            pc = 0x2929d;
        }
        0x2929d => {
            // block 2929d
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 2929d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2929d movzx ecx,BYTE PTR [r12]
            rcx = (m.r8(r12) as u64) as u32 as u64;
            // 292a2 movzx esi,BYTE PTR [rax]
            rsi = (m.r8(rax) as u64) as u32 as u64;
            // 292a5 cmp esi,ecx
            fl = sub::<32>((rsi as u32 as u64), (rcx as u32 as u64)).1;
            // 292a7 je 0x292b9
            if crate::cov::brk(0x292a7, fl.e()) { pc = 0x292b9; continue; }
            pc = 0x292a9;
        }
        0x292a9 => {
            // block 292a9
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 292a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 292a9 dec rax
            { let (r, f) = dec::<64>(rax, fl); rax = r; fl = f; }
            // 292ac dec r12
            { let (r, f) = dec::<64>(r12, fl); r12 = r; fl = f; }
            // 292af dec r8d
            { let (r, f) = dec::<32>((r8 as u32 as u64), fl); r8 = (r) as u32 as u64; fl = f; }
            // 292b2 mov DWORD PTR [r15],r8d
            m.w32(r15, ((r8 as u32 as u64)) as u32);
            // 292b5 jne 0x2929d
            if crate::cov::brk(0x292b5, fl.ne()) { pc = 0x2929d; continue; }
            pc = 0x292b7;
        }
        0x292b7 => {
            // block 292b7
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 292b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 292b7 jmp 0x292bc
            pc = 0x292bc; continue;
        }
        0x292b9 => {
            // block 292b9
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 292b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 292b9 mov edx,r8d
            rdx = ((r8 as u32 as u64)) as u32 as u64;
            pc = 0x292bc;
        }
        0x292bc => {
            // block 292bc
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 292bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 292bc cmp edx,DWORD PTR [rdi+0x8]
            fl = sub::<32>((rdx as u32 as u64), m.r32(rdi.wrapping_add(0x8)) as u64).1;
            // 292bf jae 0x292c6
            if crate::cov::brk(0x292bf, fl.ae()) { pc = 0x292c6; continue; }
            pc = 0x292c1;
        }
        0x292c1 => {
            // block 292c1
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 292c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 292c1 inc edx
            { let (r, f) = inc::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // 292c3 mov DWORD PTR [r15],edx
            m.w32(r15, ((rdx as u32 as u64)) as u32);
            pc = 0x292c6;
        }
        0x292c6 => {
            // block 292c6
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 292c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 292c6 mov eax,DWORD PTR [r13+0x0]
            rax = (m.r32(r13) as u64) as u32 as u64;
            // 292ca cmp edx,eax
            fl = sub::<32>((rdx as u32 as u64), (rax as u32 as u64)).1;
            // 292cc ja 0x29388
            if crate::cov::brk(0x292cc, fl.a()) { pc = 0x29388; continue; }
            pc = 0x292d2;
        }
        0x292d2 => {
            // block 292d2
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 292d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 292d2 mov rdi,QWORD PTR [rip+0x116ea7] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // 292d9 lea rsi,[rip+0x1015c3] # 0x12a8a3
            rsi = m.env.data_va(258160);
            // 292e0 mov edx,0x2c
            rdx = (0x2cu64) as u32 as u64;
            // 292e5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 292ea mov esi,DWORD PTR [r15]
            rsi = (m.r32(r15) as u64) as u32 as u64;
            // 292ed mov rdi,rax
            rdi = rax;
            // 292f0 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 292f5 lea rsi,[rip+0x1015d4] # 0x12a8d0
            rsi = m.env.data_va(258205);
            // 292fc mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 29301 mov rdi,rax
            rdi = rax;
            // 29304 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29309 mov esi,DWORD PTR [r13+0x0]
            rsi = (m.r32(r13) as u64) as u32 as u64;
            // 2930d mov rdi,rax
            rdi = rax;
            // 29310 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29315 lea rsi,[rip+0x1015bf] # 0x12a8db
            rsi = m.env.data_va(258216);
            // 2931c mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 29321 mov rdi,rax
            rdi = rax;
            // 29324 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29329 mov r12,rax
            r12 = rax;
            // 2932c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 29330 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 29334 add rsi,r12
            { let (r, f) = add::<64>(rsi, r12);
              rsi = r;
              fl = f; }
            // 29337 lea rbx,[rbp-0x30]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2933b mov rdi,rbx
            rdi = rbx;
            // 2933e call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29343 mov rsi,QWORD PTR [rip+0x116e46] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 2934a mov rdi,rbx
            rdi = rbx;
            // 2934d call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29352 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 29355 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 29359 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 2935e mov rdi,rax
            rdi = rax;
            // 29361 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29363 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 29365 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 29369 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2936e movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // 29371 mov rdi,r12
            rdi = r12;
            // 29374 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29379 mov rdi,r12
            rdi = r12;
            // 2937c call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29381 mov edx,DWORD PTR [r15]
            rdx = (m.r32(r15) as u64) as u32 as u64;
            // 29384 mov eax,DWORD PTR [r13+0x0]
            rax = (m.r32(r13) as u64) as u32 as u64;
            pc = 0x29388;
        }
        0x29388 => {
            // block 29388
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29388 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29388 mov DWORD PTR [r15],eax
            m.w32(r15, ((rax as u32 as u64)) as u32);
            // 2938b mov DWORD PTR [r13+0x0],edx
            m.w32(r13, ((rdx as u32 as u64)) as u32);
            // 2938f cmp DWORD PTR [r15],edx
            fl = sub::<32>(m.r32(r15) as u64, (rdx as u32 as u64)).1;
            // 29392 jb 0x29184
            if crate::cov::brk(0x29392, fl.b()) { pc = 0x29184; continue; }
            pc = 0x29398;
        }
        0x29398 => {
            // block 29398
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29398 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29398 mov rdi,QWORD PTR [rip+0x116de1] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // 2939f lea rsi,[rip+0x1014fd] # 0x12a8a3
            rsi = m.env.data_va(258160);
            // 293a6 mov edx,0x2c
            rdx = (0x2cu64) as u32 as u64;
            // 293ab call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 293b0 mov esi,DWORD PTR [r15]
            rsi = (m.r32(r15) as u64) as u32 as u64;
            // 293b3 mov rdi,rax
            rdi = rax;
            // 293b6 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 293bb lea rsi,[rip+0x10152c] # 0x12a8ee
            rsi = m.env.data_va(258235);
            // 293c2 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 293c7 mov rdi,rax
            rdi = rax;
            // 293ca call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 293cf mov esi,DWORD PTR [r13+0x0]
            rsi = (m.r32(r13) as u64) as u32 as u64;
            // 293d3 mov rdi,rax
            rdi = rax;
            // 293d6 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 293db lea rsi,[rip+0x1014f9] # 0x12a8db
            rsi = m.env.data_va(258216);
            // 293e2 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 293e7 mov rdi,rax
            rdi = rax;
            // 293ea call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 293ef mov rbx,rax
            rbx = rax;
            // 293f2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 293f5 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 293f9 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 293fc lea r15,[rbp-0x38]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 29400 mov rdi,r15
            rdi = r15;
            // 29403 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29408 mov rsi,QWORD PTR [rip+0x116d81] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 2940f mov rdi,r15
            rdi = r15;
            // 29412 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29417 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2941a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 2941e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 29423 mov rdi,rax
            rdi = rax;
            // 29426 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29428 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2942b lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 2942f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29434 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // 29438 mov rdi,rbx
            rdi = rbx;
            // 2943b call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29440 mov rdi,rbx
            rdi = rbx;
            // 29443 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29448 jmp 0x29184
            pc = 0x29184; continue;
        }
        0x2944d => {
            // block 2944d
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 2944d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2944d mov rbx,rax
            rbx = rax;
            // 29450 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 29454 jmp 0x2945d
            pc = 0x2945d; continue;
        }
        0x29456 => {
            // block 29456
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29456 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29456 mov rbx,rax
            rbx = rax;
            // 29459 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            pc = 0x2945d;
        }
        0x2945d => {
            // block 2945d
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 2945d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2945d call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29462 mov rdi,rbx
            rdi = rbx;
            // 29465 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6947e size=856
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006947e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6947e;
    loop {
        match pc {
        0x6947e => {
            // block 6947e
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6947e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6947e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6947f mov rbp,rsp
            rbp = rsp;
            // 69482 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 69484 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 69486 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 69488 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 6948a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6948b sub rsp,0xb8
            { let (r, f) = sub::<64>(rsp, 0xb8u64);
              rsp = r;
              fl = f; }
            // 69492 mov DWORD PTR [rbp-0xd4],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff2c), ((rcx as u32 as u64)) as u32);
            // 69498 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 6949b mov r14,rsi
            r14 = rsi;
            // 6949e mov r13,rdi
            r13 = rdi;
            // 694a1 lea rax,[r13+0x8]
            rax = r13.wrapping_add(0x8);
            // 694a5 mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 694ac mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // 694b4 mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // 694bc mov QWORD PTR [r13+0x0],rax
            m.w64(r13, (rax) as u64);
            // 694c0 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 694c3 call 0xceac8
            { let r = { let t = m.env.code_va(0xceac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 694c8 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 694ca mov QWORD PTR [rbp-0xd0],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (r14) as u64);
            // 694d1 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 694d8 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 694de lea r15,[rbp-0x2c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            pc = 0x694e2;
        }
        0x694e2 => {
            // block 694e2
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 694e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 694e2 mov rdi,r13
            rdi = r13;
            // 694e5 mov rsi,r15
            rsi = r15;
            // 694e8 call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 694ed mov DWORD PTR [rbp-0x2c],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((r14 as u32 as u64)) as u32);
            // 694f1 cmp r14d,ebx
            fl = sub::<32>((r14 as u32 as u64), (rbx as u32 as u64)).1;
            // 694f4 lea eax,[r14+0x1]
            rax = (r14.wrapping_add(0x1)) as u32 as u64;
            // 694f8 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // 694fb jbe 0x694e2
            if crate::cov::brk(0x694fb, fl.be()) { pc = 0x694e2; continue; }
            pc = 0x694fd;
        }
        0x694fd => {
            // block 694fd
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 694fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 694fd mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 69500 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69505 mov r14,QWORD PTR [rbp-0xd0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff30)) as u64;
            // 6950c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6950e je 0x6958b
            if crate::cov::brk(0x6950e, fl.e()) { pc = 0x6958b; continue; }
            pc = 0x69510;
        }
        0x69510 => {
            // block 69510
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69510 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69510 lea rsi,[rip+0xcc065] # 0x13557c
            rsi = m.env.data_va(302409);
            // 69517 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 6951b mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 69520 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69525 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 69529 lea rdx,[rbp-0x60]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 6952d mov rsi,r14
            rsi = r14;
            // 69530 call 0x69328
            { let r = { let t = m.env.code_va(0x69328); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69535 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 69539 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 6953e mov rbx,QWORD PTR [rbp-0x48]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 69542 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 69546 jmp 0x69579
            pc = 0x69579; continue;
        }
        0x69548 => {
            // block 69548
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69548 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69548 lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 6954c mov rdi,r13
            rdi = r13;
            // 6954f call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69554 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 69558 test rax,rax
            fl = logic::<64>(rax & rax);
            // 6955b je 0x6956a
            if crate::cov::brk(0x6955b, fl.e()) { pc = 0x6956a; continue; }
            pc = 0x6955d;
        }
        0x6955d => {
            // block 6955d
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6955d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6955d mov rbx,rax
            rbx = rax;
            // 69560 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 69563 test rax,rax
            fl = logic::<64>(rax & rax);
            // 69566 jne 0x6955d
            if crate::cov::brk(0x69566, fl.ne()) { pc = 0x6955d; continue; }
            pc = 0x69568;
        }
        0x69568 => {
            // block 69568
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69568 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69568 jmp 0x69579
            pc = 0x69579; continue;
        }
        0x6956a => {
            // block 6956a
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6956a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6956a mov rax,rbx
            rax = rbx;
            pc = 0x6956d;
        }
        0x6956d => {
            // block 6956d
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6956d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6956d mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 69571 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 69574 mov rax,rbx
            rax = rbx;
            // 69577 jne 0x6956d
            if crate::cov::brk(0x69577, fl.ne()) { pc = 0x6956d; continue; }
            pc = 0x69579;
        }
        0x69579 => {
            // block 69579
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69579 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69579 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 6957c jne 0x69548
            if crate::cov::brk(0x6957c, fl.ne()) { pc = 0x69548; continue; }
            pc = 0x6957e;
        }
        0x6957e => {
            // block 6957e
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6957e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6957e mov rsi,QWORD PTR [rbp-0x40]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 69582 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 69586 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x6958b;
        }
        0x6958b => {
            // block 6958b
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6958b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6958b mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 6958e call 0xcd01e
            { let r = { let t = m.env.code_va(0xcd01e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69593 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 69595 je 0x6961f
            if crate::cov::brk(0x69595, fl.e()) { pc = 0x6961f; continue; }
            pc = 0x6959b;
        }
        0x6959b => {
            // block 6959b
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6959b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6959b lea rsi,[rip+0xcc2ca] # 0x13586c
            rsi = m.env.data_va(303161);
            // 695a2 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 695a9 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 695ae call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 695b3 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 695b7 lea rdx,[rbp-0x90]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 695be mov rsi,r14
            rsi = r14;
            // 695c1 call 0x69328
            { let r = { let t = m.env.code_va(0x69328); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 695c6 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 695cd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 695d2 mov rbx,QWORD PTR [rbp-0x78]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 695d6 lea r15,[rbp-0x70]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 695da jmp 0x6960d
            pc = 0x6960d; continue;
        }
        0x695dc => {
            // block 695dc
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 695dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 695dc lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 695e0 mov rdi,r13
            rdi = r13;
            // 695e3 call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 695e8 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 695ec test rax,rax
            fl = logic::<64>(rax & rax);
            // 695ef je 0x695fe
            if crate::cov::brk(0x695ef, fl.e()) { pc = 0x695fe; continue; }
            pc = 0x695f1;
        }
        0x695f1 => {
            // block 695f1
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 695f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 695f1 mov rbx,rax
            rbx = rax;
            // 695f4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 695f7 test rax,rax
            fl = logic::<64>(rax & rax);
            // 695fa jne 0x695f1
            if crate::cov::brk(0x695fa, fl.ne()) { pc = 0x695f1; continue; }
            pc = 0x695fc;
        }
        0x695fc => {
            // block 695fc
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 695fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 695fc jmp 0x6960d
            pc = 0x6960d; continue;
        }
        0x695fe => {
            // block 695fe
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 695fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 695fe mov rax,rbx
            rax = rbx;
            pc = 0x69601;
        }
        0x69601 => {
            // block 69601
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69601 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69601 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 69605 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 69608 mov rax,rbx
            rax = rbx;
            // 6960b jne 0x69601
            if crate::cov::brk(0x6960b, fl.ne()) { pc = 0x69601; continue; }
            pc = 0x6960d;
        }
        0x6960d => {
            // block 6960d
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6960d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6960d cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 69610 jne 0x695dc
            if crate::cov::brk(0x69610, fl.ne()) { pc = 0x695dc; continue; }
            pc = 0x69612;
        }
        0x69612 => {
            // block 69612
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69612 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69612 mov rsi,QWORD PTR [rbp-0x70]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 69616 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 6961a call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x6961f;
        }
        0x6961f => {
            // block 6961f
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6961f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6961f mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 69622 call 0xce3db
            { let r = { let t = m.env.code_va(0xce3db); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69627 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 69629 je 0x69693
            if crate::cov::brk(0x69629, fl.e()) { pc = 0x69693; continue; }
            pc = 0x6962b;
        }
        0x6962b => {
            // block 6962b
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6962b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6962b mov DWORD PTR [rbp-0x94],0x900
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c), (0x900u64) as u32);
            // 69635 mov ebx,0x901
            rbx = (0x901u64) as u32 as u64;
            // 6963a lea r15,[rbp-0x94]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff6c);
            pc = 0x69641;
        }
        0x69641 => {
            // block 69641
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69641 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69641 mov rdi,r13
            rdi = r13;
            // 69644 mov rsi,r15
            rsi = r15;
            // 69647 call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 6964c mov DWORD PTR [rbp-0x94],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c), ((rbx as u32 as u64)) as u32);
            // 69652 cmp ebx,0x908
            fl = sub::<32>((rbx as u32 as u64), 0x908u64).1;
            // 69658 lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 6965b mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 6965d jb 0x69641
            if crate::cov::brk(0x6965d, fl.b()) { pc = 0x69641; continue; }
            pc = 0x6965f;
        }
        0x6965f => {
            // block 6965f
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6965f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6965f mov DWORD PTR [rbp-0x98],0x90e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0x90eu64) as u32);
            // 69669 mov ebx,0x90f
            rbx = (0x90fu64) as u32 as u64;
            // 6966e lea r15,[rbp-0x98]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            pc = 0x69675;
        }
        0x69675 => {
            // block 69675
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69675 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69675 mov rdi,r13
            rdi = r13;
            // 69678 mov rsi,r15
            rsi = r15;
            // 6967b call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69680 mov DWORD PTR [rbp-0x98],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), ((rbx as u32 as u64)) as u32);
            // 69686 cmp ebx,0x919
            fl = sub::<32>((rbx as u32 as u64), 0x919u64).1;
            // 6968c lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 6968f mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 69691 jb 0x69675
            if crate::cov::brk(0x69691, fl.b()) { pc = 0x69675; continue; }
            pc = 0x69693;
        }
        0x69693 => {
            // block 69693
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69693 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69693 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 69696 call 0xce392
            { let r = { let t = m.env.code_va(0xce392); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 6969b mov eax,DWORD PTR [rbp-0xd4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff2c)) as u64) as u32 as u64;
            // 696a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 696a3 je 0x6973c
            if crate::cov::brk(0x696a3, fl.e()) { pc = 0x6973c; continue; }
            pc = 0x696a9;
        }
        0x696a9 => {
            // block 696a9
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 696a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 696a9 lea rsi,[rip+0xce6a6] # 0x137d56
            rsi = m.env.data_va(312611);
            // 696b0 lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // 696b7 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 696bc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 696c1 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 696c8 lea rdx,[rbp-0xc8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // 696cf mov rsi,r14
            rsi = r14;
            // 696d2 call 0x69328
            { let r = { let t = m.env.code_va(0x69328); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 696d7 lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // 696de call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 696e3 mov rbx,QWORD PTR [rbp-0xb0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 696ea lea r14,[rbp-0xa8]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 696f1 jmp 0x69724
            pc = 0x69724; continue;
        }
        0x696f3 => {
            // block 696f3
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 696f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 696f3 lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 696f7 mov rdi,r13
            rdi = r13;
            // 696fa call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 696ff mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 69703 test rax,rax
            fl = logic::<64>(rax & rax);
            // 69706 je 0x69715
            if crate::cov::brk(0x69706, fl.e()) { pc = 0x69715; continue; }
            pc = 0x69708;
        }
        0x69708 => {
            // block 69708
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69708 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69708 mov rbx,rax
            rbx = rax;
            // 6970b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6970e test rax,rax
            fl = logic::<64>(rax & rax);
            // 69711 jne 0x69708
            if crate::cov::brk(0x69711, fl.ne()) { pc = 0x69708; continue; }
            pc = 0x69713;
        }
        0x69713 => {
            // block 69713
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69713 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69713 jmp 0x69724
            pc = 0x69724; continue;
        }
        0x69715 => {
            // block 69715
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69715 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69715 mov rax,rbx
            rax = rbx;
            pc = 0x69718;
        }
        0x69718 => {
            // block 69718
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69718 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69718 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 6971c cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 6971f mov rax,rbx
            rax = rbx;
            // 69722 jne 0x69718
            if crate::cov::brk(0x69722, fl.ne()) { pc = 0x69718; continue; }
            pc = 0x69724;
        }
        0x69724 => {
            // block 69724
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69724 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69724 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 69727 jne 0x696f3
            if crate::cov::brk(0x69727, fl.ne()) { pc = 0x696f3; continue; }
            pc = 0x69729;
        }
        0x69729 => {
            // block 69729
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69729 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69729 mov rsi,QWORD PTR [rbp-0xa8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            // 69730 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 69737 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x6973c;
        }
        0x6973c => {
            // block 6973c
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6973c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6973c mov rax,r13
            rax = r13;
            // 6973f add rsp,0xb8
            { let (r, f) = add::<64>(rsp, 0xb8u64);
              rsp = r;
              fl = f; }
            // 69746 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 69747 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 69749 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 6974b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6974d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6974f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 69750 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x69751 => {
            // block 69751
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69751 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69751 jmp 0x69793
            pc = 0x69793; continue;
        }
        0x69753 => {
            // block 69753
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69753 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69753 jmp 0x69793
            pc = 0x69793; continue;
        }
        0x69755 => {
            // block 69755
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69755 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69755 jmp 0x69793
            pc = 0x69793; continue;
        }
        0x69757 => {
            // block 69757
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69757 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69757 mov r14,rax
            r14 = rax;
            // 6975a mov rsi,QWORD PTR [rbp-0x40]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 6975e lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 69762 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69767 jmp 0x69796
            pc = 0x69796; continue;
        }
        0x69769 => {
            // block 69769
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69769 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69769 mov r14,rax
            r14 = rax;
            // 6976c mov rsi,QWORD PTR [rbp-0x70]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 69770 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 69774 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69779 jmp 0x69796
            pc = 0x69796; continue;
        }
        0x6977b => {
            // block 6977b
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6977b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6977b mov r14,rax
            r14 = rax;
            // 6977e mov rsi,QWORD PTR [rbp-0xa8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            // 69785 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 6978c call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69791 jmp 0x69796
            pc = 0x69796; continue;
        }
        0x69793 => {
            // block 69793
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69793 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69793 mov r14,rax
            r14 = rax;
            pc = 0x69796;
        }
        0x69796 => {
            // block 69796
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69796 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69796 mov rax,QWORD PTR [rbp-0xe0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff20)) as u64;
            // 6979d mov rsi,QWORD PTR [rax]
            rsi = m.r64(rax) as u64;
            // 697a0 mov rdi,r13
            rdi = r13;
            // 697a3 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 697a8 mov rdi,r14
            rdi = r14;
            // 697ab call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 697b0 mov r14,rax
            r14 = rax;
            // 697b3 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 697b7 jmp 0x697cf
            pc = 0x697cf; continue;
        }
        0x697b9 => {
            // block 697b9
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 697b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 697b9 mov r14,rax
            r14 = rax;
            // 697bc lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 697c3 jmp 0x697cf
            pc = 0x697cf; continue;
        }
        0x697c5 => {
            // block 697c5
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 697c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 697c5 mov r14,rax
            r14 = rax;
            // 697c8 lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            pc = 0x697cf;
        }
        0x697cf => {
            // block 697cf
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 697cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 697cf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 697d4 jmp 0x69796
            pc = 0x69796; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
