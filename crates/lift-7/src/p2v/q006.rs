//! Mechanically lifted functions (scripts/lift_run.py), module q006. Each one is checked against the original
//! in the world harness (tests/world_q006.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xbc1e6 size=380
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc1e6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbc1e6;
    loop {
        match pc {
        0xbc1e6 => {
            // block bc1e6
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc1e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1e6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc1e7 mov rbp,rsp
            rbp = rsp;
            // bc1ea push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc1ec push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc1ee push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // bc1f0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc1f2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc1f3 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // bc1f7 mov rbx,rsi
            rbx = rsi;
            // bc1fa mov r15d,edi
            r15 = ((rdi as u32 as u64)) as u32 as u64;
            // bc1fd lea r14,[rbx+0x8]
            r14 = rbx.wrapping_add(0x8);
            // bc201 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc205 mov rdi,rbx
            rdi = rbx;
            // bc208 call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc20d mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // bc215 mov QWORD PTR [rbx],r14
            m.w64(rbx, (r14) as u64);
            // bc218 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // bc220 lea rax,[rbp-0x38]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bc224 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // bc22c mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // bc234 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // bc238 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bc23c mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // bc23f call 0xbc038
            { let r = { let t = m.env.code_va(0xbc038); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc244 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // bc246 je 0xbc313
            if fl.e() { pc = 0xbc313; continue; }
            pc = 0xbc24c;
        }
        0xbc24c => {
            // block bc24c
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc24c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc24c mov r13,QWORD PTR [rbp-0x40]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // bc250 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // bc253 lea rax,[rbp-0x38]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bc257 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // bc25a je 0xbc316
            if fl.e() { pc = 0xbc316; continue; }
            pc = 0xbc260;
        }
        0xbc260 => {
            // block bc260
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc260 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc260 lea r15,[rbp-0x50]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // bc264 lea r12,[rbp-0x58]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            pc = 0xbc268;
        }
        0xbc268 => {
            // block bc268
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc268 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc268 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // bc270 mov QWORD PTR [r15],0x0
            m.w64(r15, (0x0u64) as u64);
            // bc277 mov QWORD PTR [rbp-0x58],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r15) as u64);
            // bc27b mov edi,DWORD PTR [r13+0x1c]
            rdi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // bc27f mov rsi,r12
            rsi = r12;
            // bc282 call 0xbc13c
            { let r = { let t = m.env.code_va(0xbc13c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc287 mov r12,QWORD PTR [rbp-0x58]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // bc28b jmp 0xbc2c5
            pc = 0xbc2c5; continue;
        }
        0xbc28d => {
            // block bc28d
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc28d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc28d lea rdx,[r12+0x1c]
            rdx = r12.wrapping_add(0x1c);
            // bc292 mov rdi,rbx
            rdi = rbx;
            // bc295 mov rsi,r14
            rsi = r14;
            // bc298 call 0xebd92
            { let r = { let t = m.env.stub_va(0x55f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc29d mov rax,QWORD PTR [r12+0x8]
            rax = m.r64(r12.wrapping_add(0x8)) as u64;
            // bc2a2 test rax,rax
            fl = logic::<64>(rax & rax);
            // bc2a5 je 0xbc2b5
            if fl.e() { pc = 0xbc2b5; continue; }
            pc = 0xbc2a7;
        }
        0xbc2a7 => {
            // block bc2a7
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc2a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc2a7 mov r12,rax
            r12 = rax;
            // bc2aa mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // bc2ae test rax,rax
            fl = logic::<64>(rax & rax);
            // bc2b1 jne 0xbc2a7
            if fl.ne() { pc = 0xbc2a7; continue; }
            pc = 0xbc2b3;
        }
        0xbc2b3 => {
            // block bc2b3
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc2b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc2b3 jmp 0xbc2c5
            pc = 0xbc2c5; continue;
        }
        0xbc2b5 => {
            // block bc2b5
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc2b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc2b5 mov rax,r12
            rax = r12;
            pc = 0xbc2b8;
        }
        0xbc2b8 => {
            // block bc2b8
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc2b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc2b8 mov r12,QWORD PTR [rax+0x10]
            r12 = m.r64(rax.wrapping_add(0x10)) as u64;
            // bc2bc cmp QWORD PTR [r12],rax
            fl = sub::<64>(m.r64(r12) as u64, rax).1;
            // bc2c0 mov rax,r12
            rax = r12;
            // bc2c3 jne 0xbc2b8
            if fl.ne() { pc = 0xbc2b8; continue; }
            pc = 0xbc2c5;
        }
        0xbc2c5 => {
            // block bc2c5
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc2c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc2c5 cmp r12,r15
            fl = sub::<64>(r12, r15).1;
            // bc2c8 jne 0xbc28d
            if fl.ne() { pc = 0xbc28d; continue; }
            pc = 0xbc2ca;
        }
        0xbc2ca => {
            // block bc2ca
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc2ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc2ca mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // bc2ce lea r12,[rbp-0x58]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // bc2d2 mov rdi,r12
            rdi = r12;
            // bc2d5 call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc2da mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bc2de test rax,rax
            fl = logic::<64>(rax & rax);
            // bc2e1 je 0xbc2f1
            if fl.e() { pc = 0xbc2f1; continue; }
            pc = 0xbc2e3;
        }
        0xbc2e3 => {
            // block bc2e3
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc2e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc2e3 mov r13,rax
            r13 = rax;
            // bc2e6 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // bc2ea test rax,rax
            fl = logic::<64>(rax & rax);
            // bc2ed jne 0xbc2e3
            if fl.ne() { pc = 0xbc2e3; continue; }
            pc = 0xbc2ef;
        }
        0xbc2ef => {
            // block bc2ef
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc2ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc2ef jmp 0xbc301
            pc = 0xbc301; continue;
        }
        0xbc2f1 => {
            // block bc2f1
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc2f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc2f1 mov rax,r13
            rax = r13;
            pc = 0xbc2f4;
        }
        0xbc2f4 => {
            // block bc2f4
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc2f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc2f4 mov r13,QWORD PTR [rax+0x10]
            r13 = m.r64(rax.wrapping_add(0x10)) as u64;
            // bc2f8 cmp QWORD PTR [r13+0x0],rax
            fl = sub::<64>(m.r64(r13) as u64, rax).1;
            // bc2fc mov rax,r13
            rax = r13;
            // bc2ff jne 0xbc2f4
            if fl.ne() { pc = 0xbc2f4; continue; }
            pc = 0xbc301;
        }
        0xbc301 => {
            // block bc301
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc301 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc301 lea rax,[rbp-0x38]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bc305 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // bc308 jne 0xbc268
            if fl.ne() { pc = 0xbc268; continue; }
            pc = 0xbc30e;
        }
        0xbc30e => {
            // block bc30e
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc30e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc30e mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // bc311 jmp 0xbc316
            pc = 0xbc316; continue;
        }
        0xbc313 => {
            // block bc313
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc313 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc313 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xbc316;
        }
        0xbc316 => {
            // block bc316
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc316 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc316 mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // bc31a lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bc31e call 0xebd80
            { let r = { let t = m.env.stub_va(0x54d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc323 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // bc326 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // bc32a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc32b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc32d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // bc32f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc331 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc333 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc334 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xbc335 => {
            // block bc335
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc335 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc335 mov rbx,rax
            rbx = rax;
            // bc338 mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // bc33c lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // bc340 call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbc345;
        }
        0xbc345 => {
            // block bc345
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc345 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc345 mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // bc349 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bc34d call 0xebd80
            { let r = { let t = m.env.stub_va(0x54d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc352 mov rdi,rbx
            rdi = rbx;
            // bc355 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc35a jmp 0xbc335
            pc = 0xbc335; continue;
        }
        0xbc35c => {
            // block bc35c
            #[cfg(feature = "trace")] eprintln!("f_000bc1e6 pc bc35c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc35c mov rbx,rax
            rbx = rax;
            // bc35f jmp 0xbc345
            pc = 0xbc345; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1e08 size=376
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001e08<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1e08;
    loop {
        match pc {
        0x1e08 => {
            // block 1e08
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1e08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1e08 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1e09 mov rbp,rsp
            rbp = rsp;
            // 1e0c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1e0e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1e10 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1e11 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 1e15 mov r15,rsi
            r15 = rsi;
            // 1e18 mov rbx,rdi
            rbx = rdi;
            // 1e1b mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 1e22 mov DWORD PTR [rbp-0x20],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u32);
            // 1e29 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 1e30 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1e33 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 1e37 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 1e39 mov ecx,0x1c00
            rcx = (0x1c00u64) as u32 as u64;
            // 1e3e mov r8d,0xa
            r8 = (0xau64) as u32 as u64;
            // 1e44 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1e47 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1e4a mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 1e4d mov DWORD PTR [r15],eax
            m.w32(r15, ((rax as u32 as u64)) as u32);
            // 1e50 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 1e53 call 0xcf034
            { let r = { let t = m.env.code_va(0xcf034); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1e58 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 1e5b ja 0x1e71
            if fl.a() { pc = 0x1e71; continue; }
            pc = 0x1e5d;
        }
        0x1e5d => {
            // block 1e5d
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1e5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1e5d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1e60 mov rdi,rbx
            rdi = rbx;
            // 1e63 call QWORD PTR [rax+0x100]
            { let r = { let t = m.r64(rax.wrapping_add(0x100)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1e69 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1e6b je 0x1f58
            if fl.e() { pc = 0x1f58; continue; }
            pc = 0x1e71;
        }
        0x1e71 => {
            // block 1e71
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1e71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1e71 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1e74 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // 1e78 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1e7d mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 1e82 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 1e85 mov rdi,rbx
            rdi = rbx;
            // 1e88 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1e8b cmp DWORD PTR [rbp-0x20],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64, 0x0u64).1;
            // 1e8f je 0x1f58
            if fl.e() { pc = 0x1f58; continue; }
            pc = 0x1e95;
        }
        0x1e95 => {
            // block 1e95
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1e95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1e95 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 1e98 cmp rax,0x5
            fl = sub::<64>(rax, 0x5u64).1;
            // 1e9c ja 0x1f58
            if fl.a() { pc = 0x1f58; continue; }
            pc = 0x1ea2;
        }
        0x1ea2 => {
            // block 1ea2
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1ea2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ea2 lea rcx,[rip+0xbf] # 0x1f68
            rcx = m.env.code_va(0x1f68);
            // 1ea9 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 1ead add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 1eb0 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x1eb2 => {
            // block 1eb2
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1eb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1eb2 mov DWORD PTR [r15],0x8
            m.w32(r15, (0x8u64) as u32);
            // 1eb9 jmp 0x1f58
            pc = 0x1f58; continue;
        }
        0x1ebe => {
            // block 1ebe
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1ebe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ebe mov DWORD PTR [r15],0x9
            m.w32(r15, (0x9u64) as u32);
            // 1ec5 jmp 0x1f58
            pc = 0x1f58; continue;
        }
        0x1eca => {
            // block 1eca
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1eca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1eca mov DWORD PTR [r15],0xa
            m.w32(r15, (0xau64) as u32);
            // 1ed1 jmp 0x1f58
            pc = 0x1f58; continue;
        }
        0x1ed6 => {
            // block 1ed6
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1ed6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ed6 mov DWORD PTR [r15],0xb
            m.w32(r15, (0xbu64) as u32);
            // 1edd jmp 0x1f58
            pc = 0x1f58; continue;
        }
        0x1edf => {
            // block 1edf
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1edf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1edf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1ee2 mov rdi,rbx
            rdi = rbx;
            // 1ee5 call QWORD PTR [rax+0x100]
            { let r = { let t = m.r64(rax.wrapping_add(0x100)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1eeb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1eed je 0x1f0c
            if fl.e() { pc = 0x1f0c; continue; }
            pc = 0x1eef;
        }
        0x1eef => {
            // block 1eef
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1eef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1eef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1ef2 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 1ef6 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1efb mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 1f00 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 1f06 mov rdi,rbx
            rdi = rbx;
            // 1f09 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1f0c;
        }
        0x1f0c => {
            // block 1f0c
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1f0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f0c cmp DWORD PTR [rbp-0x24],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64, 0x0u64).1;
            // 1f10 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 1f13 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 1f16 or eax,0xc
            { let r = (rax as u32 as u64) | 0xcu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 1f19 jmp 0x1f55
            pc = 0x1f55; continue;
        }
        0x1f1b => {
            // block 1f1b
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1f1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f1b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1f1e mov rdi,rbx
            rdi = rbx;
            // 1f21 call QWORD PTR [rax+0x100]
            { let r = { let t = m.r64(rax.wrapping_add(0x100)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1f27 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1f29 je 0x1f48
            if fl.e() { pc = 0x1f48; continue; }
            pc = 0x1f2b;
        }
        0x1f2b => {
            // block 1f2b
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1f2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f2b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1f2e lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 1f32 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1f37 mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 1f3c mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 1f42 mov rdi,rbx
            rdi = rbx;
            // 1f45 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1f48;
        }
        0x1f48 => {
            // block 1f48
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1f48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f48 cmp DWORD PTR [rbp-0x24],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64, 0x0u64).1;
            // 1f4c sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 1f4f movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 1f52 or eax,0xe
            { let r = (rax as u32 as u64) | 0xeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x1f55;
        }
        0x1f55 => {
            // block 1f55
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1f55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f55 mov DWORD PTR [r15],eax
            m.w32(r15, ((rax as u32 as u64)) as u32);
            pc = 0x1f58;
        }
        0x1f58 => {
            // block 1f58
            #[cfg(feature = "trace")] eprintln!("f_00001e08 pc 1f58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f58 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 1f5b add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 1f5f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1f60 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1f62 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1f64 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1f65 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcf3c size=372
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cf3c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xcf3c;
    loop {
        match pc {
        0xcf3c => {
            // block cf3c
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cf3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cf3c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cf3d mov rbp,rsp
            rbp = rsp;
            // cf40 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // cf42 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // cf43 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // cf47 mov r14,rsi
            r14 = rsi;
            // cf4a mov rbx,rdi
            rbx = rdi;
            // cf4d mov DWORD PTR [rbp-0x18],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (0xcu64) as u32);
            // cf54 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // cf57 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // cf5b call QWORD PTR [rax+0x14c8]
            { let r = { let t = m.r64(rax.wrapping_add(0x14c8)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // cf61 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // cf63 je 0xcf75
            if fl.e() { pc = 0xcf75; continue; }
            pc = 0xcf65;
        }
        0xcf65 => {
            // block cf65
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cf65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cf65 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // cf68 lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // cf6c mov rdi,rbx
            rdi = rbx;
            // cf6f call QWORD PTR [rax+0x14b8]
            { let r = { let t = m.r64(rax.wrapping_add(0x14b8)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xcf75;
        }
        0xcf75 => {
            // block cf75
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cf75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cf75 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // cf77 je 0xd033
            if fl.e() { pc = 0xd033; continue; }
            pc = 0xcf7d;
        }
        0xcf7d => {
            // block cf7d
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cf7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cf7d mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // cf80 cmp rcx,0x3
            fl = sub::<64>(rcx, 0x3u64).1;
            // cf84 ja 0xd033
            if fl.a() { pc = 0xd033; continue; }
            pc = 0xcf8a;
        }
        0xcf8a => {
            // block cf8a
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cf8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cf8a lea rdx,[rip+0xdf] # 0xd070
            rdx = m.env.code_va(0xd070);
            // cf91 movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // cf95 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // cf98 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xcf9a => {
            // block cf9a
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cf9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cf9a mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // cf9d cmp ecx,0x8
            fl = sub::<32>((rcx as u32 as u64), 0x8u64).1;
            // cfa0 jne 0xd000
            if fl.ne() { pc = 0xd000; continue; }
            pc = 0xcfa2;
        }
        0xcfa2 => {
            // block cfa2
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfa2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfa2 mov DWORD PTR [r14],0xc
            m.w32(r14, (0xcu64) as u32);
            // cfa9 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xcfae => {
            // block cfae
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfae mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // cfb1 cmp ecx,0x8
            fl = sub::<32>((rcx as u32 as u64), 0x8u64).1;
            // cfb4 jne 0xd00e
            if fl.ne() { pc = 0xd00e; continue; }
            pc = 0xcfb6;
        }
        0xcfb6 => {
            // block cfb6
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfb6 mov DWORD PTR [r14],0xe
            m.w32(r14, (0xeu64) as u32);
            // cfbd jmp 0xd062
            pc = 0xd062; continue;
        }
        0xcfc2 => {
            // block cfc2
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfc2 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // cfc4 mov edx,DWORD PTR [rbp-0x14]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // cfc7 dec edx
            { let (r, f) = dec::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // cfc9 cmp edx,0xb
            fl = sub::<32>((rdx as u32 as u64), 0xbu64).1;
            // cfcc ja 0xd064
            if fl.a() { pc = 0xd064; continue; }
            pc = 0xcfd2;
        }
        0xcfd2 => {
            // block cfd2
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfd2 lea rsi,[rip+0xa7] # 0xd080
            rsi = m.env.code_va(0xd080);
            // cfd9 movsxd rdx,DWORD PTR [rsi+rdx*4]
            rdx = sx::<32>(m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // cfdd add rdx,rsi
            { let (r, f) = add::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // cfe0 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xcfe2 => {
            // block cfe2
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfe2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfe2 mov DWORD PTR [r14],0x4
            m.w32(r14, (0x4u64) as u32);
            // cfe9 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xcfeb => {
            // block cfeb
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfeb mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // cfee test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // cff0 je 0xd01c
            if fl.e() { pc = 0xd01c; continue; }
            pc = 0xcff2;
        }
        0xcff2 => {
            // block cff2
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cff2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cff2 cmp ecx,0x8
            fl = sub::<32>((rcx as u32 as u64), 0x8u64).1;
            // cff5 jne 0xd025
            if fl.ne() { pc = 0xd025; continue; }
            pc = 0xcff7;
        }
        0xcff7 => {
            // block cff7
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cff7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cff7 mov DWORD PTR [r14],0x7
            m.w32(r14, (0x7u64) as u32);
            // cffe jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd000 => {
            // block d000
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d000 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d000 cmp ecx,0xc
            fl = sub::<32>((rcx as u32 as u64), 0xcu64).1;
            // d003 jne 0xd033
            if fl.ne() { pc = 0xd033; continue; }
            pc = 0xd005;
        }
        0xd005 => {
            // block d005
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d005 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d005 mov DWORD PTR [r14],0x9
            m.w32(r14, (0x9u64) as u32);
            // d00c jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd00e => {
            // block d00e
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d00e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d00e cmp ecx,0xc
            fl = sub::<32>((rcx as u32 as u64), 0xcu64).1;
            // d011 jne 0xd033
            if fl.ne() { pc = 0xd033; continue; }
            pc = 0xd013;
        }
        0xd013 => {
            // block d013
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d013 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d013 mov DWORD PTR [r14],0xb
            m.w32(r14, (0xbu64) as u32);
            // d01a jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd01c => {
            // block d01c
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d01c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d01c mov DWORD PTR [r14],0x8
            m.w32(r14, (0x8u64) as u32);
            // d023 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd025 => {
            // block d025
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d025 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d025 cmp ecx,0xc
            fl = sub::<32>((rcx as u32 as u64), 0xcu64).1;
            // d028 jne 0xd033
            if fl.ne() { pc = 0xd033; continue; }
            pc = 0xd02a;
        }
        0xd02a => {
            // block d02a
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d02a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d02a mov DWORD PTR [r14],0x6
            m.w32(r14, (0x6u64) as u32);
            // d031 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd033 => {
            // block d033
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d033 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d033 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d035 jmp 0xd064
            pc = 0xd064; continue;
        }
        0xd037 => {
            // block d037
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d037 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d037 mov DWORD PTR [r14],0x5
            m.w32(r14, (0x5u64) as u32);
            // d03e jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd040 => {
            // block d040
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d040 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d040 mov DWORD PTR [r14],0x1
            m.w32(r14, (0x1u64) as u32);
            // d047 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd049 => {
            // block d049
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d049 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d049 mov DWORD PTR [r14],0x2
            m.w32(r14, (0x2u64) as u32);
            // d050 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd052 => {
            // block d052
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d052 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d052 mov DWORD PTR [r14],0x3
            m.w32(r14, (0x3u64) as u32);
            // d059 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd05b => {
            // block d05b
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d05b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d05b mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            pc = 0xd062;
        }
        0xd062 => {
            // block d062
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d062 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d062 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0xd064;
        }
        0xd064 => {
            // block d064
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d064 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d064 mov al,cl
            rax = (rax & !0xff) | (((rcx as u8 as u64)) as u8 as u64);
            // d066 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d06a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d06b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d06d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d06e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10c76 size=366
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010c76<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x10c76;
    loop {
        match pc {
        0x10c76 => {
            // block 10c76
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10c76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c76 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10c77 mov rbp,rsp
            rbp = rsp;
            // 10c7a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10c7c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10c7e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 10c80 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 10c82 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10c83 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10c87 mov r14,rsi
            r14 = rsi;
            // 10c8a mov r15,rdi
            r15 = rdi;
            // 10c8d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 10c90 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10c94 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 10c97 jne 0x10dc9
            if fl.ne() { pc = 0x10dc9; continue; }
            pc = 0x10c9d;
        }
        0x10c9d => {
            // block 10c9d
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10c9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c9d mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10ca1 mov rcx,QWORD PTR [r15+0x18]
            rcx = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10ca5 cmp r12,rcx
            fl = sub::<64>(r12, rcx).1;
            // 10ca8 jae 0x10cf7
            if fl.ae() { pc = 0x10cf7; continue; }
            pc = 0x10caa;
        }
        0x10caa => {
            // block 10caa
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10caa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10caa sub rcx,r12
            { let (r, f) = sub::<64>(rcx, r12);
              rcx = r;
              fl = f; }
            // 10cad sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 10cb1 lea rax,[rcx+0x1]
            rax = rcx.wrapping_add(0x1);
            // 10cb5 shr rax,0x3f
            { let (r, f) = shr::<64>(rax, 0x3fu64, fl); rax = r; fl = f; }
            // 10cb9 lea rbx,[rcx+rax*1+0x1]
            rbx = rcx.wrapping_add(rax).wrapping_add(0x1);
            // 10cbe sar rbx,1
            { let (r, f) = sar::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 10cc1 mov rdx,r12
            rdx = r12;
            // 10cc4 sub rdx,r13
            { let (r, f) = sub::<64>(rdx, r13);
              rdx = r;
              fl = f; }
            // 10cc7 mov rax,rdx
            rax = rdx;
            // 10cca shr rax,0x3
            { let (r, f) = shr::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 10cce mov rcx,rbx
            rcx = rbx;
            // 10cd1 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 10cd4 lea r12,[r12+rcx*8]
            r12 = r12.wrapping_add(rcx.wrapping_mul(8));
            // 10cd8 mov rdi,r12
            rdi = r12;
            // 10cdb mov rsi,r13
            rsi = r13;
            // 10cde call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10ce3 mov QWORD PTR [r15+0x8],r12
            m.w64(r15.wrapping_add(0x8), (r12) as u64);
            // 10ce7 shl rbx,0x3
            { let (r, f) = shl::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // 10ceb add QWORD PTR [r15+0x10],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, rbx);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 10cef mov r13,r12
            r13 = r12;
            // 10cf2 jmp 0x10dc9
            pc = 0x10dc9; continue;
        }
        0x10cf7 => {
            // block 10cf7
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10cf7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10cf7 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10cfb mov QWORD PTR [rbp-0x58],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r14) as u64);
            // 10cff mov rax,rcx
            rax = rcx;
            // 10d02 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 10d05 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10d09 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 10d0f cmovne r14,rax
            if fl.ne() { r14 = rax; }
            // 10d13 lea rbx,[r14+r14*1+0x6]
            rbx = r14.wrapping_add(r14).wrapping_add(0x6);
            // 10d18 and rbx,0xfffffffffffffff8
            { let r = rbx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 10d1c lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 10d20 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 10d24 lea rdi,[r14*8+0x0]
            rdi = r14.wrapping_mul(8);
            // 10d2c mov QWORD PTR [rbp-0x68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rcx) as u64);
            // 10d30 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10d35 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 10d39 add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 10d3c mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // 10d40 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 10d44 lea rcx,[rax+r14*8]
            rcx = rax.wrapping_add(r14.wrapping_mul(8));
            // 10d48 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 10d4c cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 10d4f je 0x10d8e
            if fl.e() { pc = 0x10d8e; continue; }
            pc = 0x10d51;
        }
        0x10d51 => {
            // block 10d51
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10d51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10d51 mov r14,QWORD PTR [rbp-0x58]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x10d55;
        }
        0x10d55 => {
            // block 10d55
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10d55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10d55 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 10d59 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 10d5c mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 10d60 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 10d64 mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // 10d68 add r13,0x8
            { let (r, f) = add::<64>(r13, 0x8u64);
              r13 = r;
              fl = f; }
            // 10d6c cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 10d6f jne 0x10d55
            if fl.ne() { pc = 0x10d55; continue; }
            pc = 0x10d71;
        }
        0x10d71 => {
            // block 10d71
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10d71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10d71 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // 10d74 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10d78 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 10d7c mov rdx,QWORD PTR [rbp-0x48]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 10d80 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10d84 mov rsi,QWORD PTR [r15+0x18]
            rsi = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10d88 mov rcx,QWORD PTR [rbp-0x38]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 10d8c jmp 0x10d9d
            pc = 0x10d9d; continue;
        }
        0x10d8e => {
            // block 10d8e
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10d8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10d8e mov rdx,rbx
            rdx = rbx;
            // 10d91 mov r14,QWORD PTR [rbp-0x58]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 10d95 mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 10d99 mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            pc = 0x10d9d;
        }
        0x10d9d => {
            // block 10d9d
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10d9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10d9d mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 10da0 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 10da4 mov QWORD PTR [r15+0x8],rdx
            m.w64(r15.wrapping_add(0x8), (rdx) as u64);
            // 10da8 mov QWORD PTR [rbp-0x48],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r13) as u64);
            // 10dac mov QWORD PTR [r15+0x10],rbx
            m.w64(r15.wrapping_add(0x10), (rbx) as u64);
            // 10db0 mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 10db4 mov QWORD PTR [r15+0x18],rcx
            m.w64(r15.wrapping_add(0x18), (rcx) as u64);
            // 10db8 mov QWORD PTR [rbp-0x38],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rsi) as u64);
            // 10dbc lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10dc0 call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10dc5 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x10dc9;
        }
        0x10dc9 => {
            // block 10dc9
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10dc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10dc9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 10dcc mov QWORD PTR [r13-0x8],rax
            m.w64(r13.wrapping_add(0xffff_ffff_ffff_fff8), (rax) as u64);
            // 10dd0 add QWORD PTR [r15+0x8],0xfffffffffffffff8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, 0xffff_ffff_ffff_fff8u64);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 10dd5 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10dd9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 10dda pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 10ddc pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 10dde pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10de0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 10de2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10de3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe73ea size=362
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e73ea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe73ea;
    loop {
        match pc {
        0xe73ea => {
            // block e73ea
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e73ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e73ea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e73eb mov rbp,rsp
            rbp = rsp;
            // e73ee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e73f0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e73f2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e73f4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e73f6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e73f7 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e73fb mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // e73fe mov r15,rsi
            r15 = rsi;
            // e7401 mov QWORD PTR [rbp-0x30],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r15) as u64);
            // e7405 mov rbx,rdi
            rbx = rdi;
            // e7408 lea rsi,[rip+0x518b9] # 0x138cc8
            rsi = m.env.data_va(316565);
            // e740f xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e7412 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e7414 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7419 xor r14b,0x1
            { let r = (r14 as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              r14 = (r14 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e741d mov DWORD PTR [rbp-0x34],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((r14 as u32 as u64)) as u32);
            // e7421 lea r14,[r15+0x5c]
            r14 = r15.wrapping_add(0x5c);
            // e7425 mov DWORD PTR [rbp-0x38],0xf
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0xfu64) as u32);
            pc = 0xe742c;
        }
        0xe742c => {
            // block e742c
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e742c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e742c mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // e742f cmp r12d,0x3
            fl = sub::<32>((r12 as u32 as u64), 0x3u64).1;
            // e7433 jne 0xe7466
            if fl.ne() { pc = 0xe7466; continue; }
            pc = 0xe7435;
        }
        0xe7435 => {
            // block e7435
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e7435 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7435 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e7439 mov eax,DWORD PTR [rax+0x8]
            rax = (m.r32(rax.wrapping_add(0x8)) as u64) as u32 as u64;
            // e743c add eax,0xfffffffb
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffffbu64);
              rax = (r) as u32 as u64;
              fl = f; }
            // e743f cmp eax,0x3
            fl = sub::<32>((rax as u32 as u64), 0x3u64).1;
            // e7442 seta al
            rax = (rax & !0xff) | (((fl.a() as u64)) as u8 as u64);
            // e7445 mov ecx,DWORD PTR [rbp-0x34]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // e7448 or al,cl
            { let r = (rax as u8 as u64) | (rcx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e744a jne 0xe7466
            if fl.ne() { pc = 0xe7466; continue; }
            pc = 0xe744c;
        }
        0xe744c => {
            // block e744c
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e744c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e744c mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // e744f and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e7452 add eax,0xd
            { let (r, f) = add::<32>((rax as u32 as u64), 0xdu64);
              rax = (r) as u32 as u64;
              fl = f; }
            // e7455 mov DWORD PTR [rbp-0x38],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rax as u32 as u64)) as u32);
            // e7458 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // e745b shr eax,0x1f
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1fu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e745e add eax,r15d
            { let (r, f) = add::<32>((rax as u32 as u64), (r15 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e7461 sar eax,1
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e7463 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xe7466;
        }
        0xe7466 => {
            // block e7466
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e7466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7466 movsxd rax,r15d
            rax = sx::<32>((r15 as u32 as u64)) as u64;
            // e7469 imul rax,rax,0x66666667
            { let (r, f) = imul::<64>(rax, 0x66666667u64); rax = r; fl = f; }
            // e7470 mov rcx,rax
            rcx = rax;
            // e7473 shr rcx,0x3f
            { let (r, f) = shr::<64>(rcx, 0x3fu64, fl); rcx = r; fl = f; }
            // e7477 sar rax,0x22
            { let (r, f) = sar::<64>(rax, 0x22u64, fl); rax = r; fl = f; }
            // e747b lea r13d,[rax+rcx*1]
            r13 = (rax.wrapping_add(rcx)) as u32 as u64;
            // e747f add r13d,r13d
            { let (r, f) = add::<32>((r13 as u32 as u64), (r13 as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // e7482 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e7484 cmp r15d,0xfffffff7
            fl = sub::<32>((r15 as u32 as u64), 0xfffffff7u64).1;
            // e7488 jl 0xe749d
            if fl.l() { pc = 0xe749d; continue; }
            pc = 0xe748a;
        }
        0xe748a => {
            // block e748a
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e748a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e748a add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e748c cmp r15d,0x9f
            fl = sub::<32>((r15 as u32 as u64), 0x9fu64).1;
            // e7493 mov ecx,0xd
            rcx = (0xdu64) as u32 as u64;
            // e7498 cmovg eax,ecx
            if fl.g() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // e749b mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xe749d;
        }
        0xe749d => {
            // block e749d
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e749d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e749d add edx,0x30
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x30u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e74a0 movsx esi,dl
            rsi = (sx::<8>((rdx as u8 as u64)) as u64) as u32 as u64;
            // e74a3 mov rdi,rbx
            rdi = rbx;
            // e74a6 call 0xec020
            { let r = { let t = m.env.stub_va(0x7ed); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e74ab lea eax,[r13+r13*4+0x0]
            rax = (r13.wrapping_add(r13.wrapping_mul(4))) as u32 as u64;
            // e74b0 sub r15d,eax
            { let (r, f) = sub::<32>((r15 as u32 as u64), (rax as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // e74b3 mov al,0x30
            rax = (rax & !0xff) | ((0x30u64) as u8 as u64);
            // e74b5 js 0xe74be
            if fl.s() { pc = 0xe74be; continue; }
            pc = 0xe74b7;
        }
        0xe74b7 => {
            // block e74b7
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e74b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e74b7 add r15d,0x30
            { let (r, f) = add::<32>((r15 as u32 as u64), 0x30u64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // e74bb mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            pc = 0xe74be;
        }
        0xe74be => {
            // block e74be
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e74be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e74be movsx esi,al
            rsi = (sx::<8>((rax as u8 as u64)) as u64) as u32 as u64;
            // e74c1 mov rdi,rbx
            rdi = rbx;
            // e74c4 call 0xec020
            { let r = { let t = m.env.stub_va(0x7ed); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e74c9 cmp r12,0x2
            fl = sub::<64>(r12, 0x2u64).1;
            // e74cd jg 0xe7506
            if fl.g() { pc = 0xe7506; continue; }
            pc = 0xe74cf;
        }
        0xe74cf => {
            // block e74cf
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e74cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e74cf cmp r12d,0x2
            fl = sub::<32>((r12 as u32 as u64), 0x2u64).1;
            // e74d3 jne 0xe74f7
            if fl.ne() { pc = 0xe74f7; continue; }
            pc = 0xe74d5;
        }
        0xe74d5 => {
            // block e74d5
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e74d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e74d5 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e74d9 mov eax,DWORD PTR [rax+0x8]
            rax = (m.r32(rax.wrapping_add(0x8)) as u64) as u32 as u64;
            // e74dc cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // e74df je 0xe74e6
            if fl.e() { pc = 0xe74e6; continue; }
            pc = 0xe74e1;
        }
        0xe74e1 => {
            // block e74e1
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e74e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e74e1 cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // e74e4 jne 0xe74f7
            if fl.ne() { pc = 0xe74f7; continue; }
            pc = 0xe74e6;
        }
        0xe74e6 => {
            // block e74e6
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e74e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e74e6 mov rdi,rbx
            rdi = rbx;
            // e74e9 lea rsi,[rip+0x50963] # 0x137e53
            rsi = m.env.data_va(312864);
            // e74f0 call 0xebff6
            { let r = { let t = m.env.stub_va(0x7c3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e74f5 jmp 0xe7506
            pc = 0xe7506; continue;
        }
        0xe74f7 => {
            // block e74f7
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e74f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e74f7 mov rdi,rbx
            rdi = rbx;
            // e74fa lea rsi,[rip+0x42927] # 0x129e28
            rsi = m.env.data_va(255477);
            // e7501 call 0xebff6
            { let r = { let t = m.env.stub_va(0x7c3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe7506;
        }
        0xe7506 => {
            // block e7506
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e7506 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7506 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // e7509 add r14,0xfffffffffffffffc
            { let (r, f) = add::<64>(r14, 0xffff_ffff_ffff_fffcu64);
              r14 = r;
              fl = f; }
            // e750d cmp r12,0x4
            fl = sub::<64>(r12, 0x4u64).1;
            // e7511 jl 0xe742c
            if fl.l() { pc = 0xe742c; continue; }
            pc = 0xe7517;
        }
        0xe7517 => {
            // block e7517
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e7517 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7517 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // e751a cmp eax,0xe
            fl = sub::<32>((rax as u32 as u64), 0xeu64).1;
            // e751d ja 0xe752d
            if fl.a() { pc = 0xe752d; continue; }
            pc = 0xe751f;
        }
        0xe751f => {
            // block e751f
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e751f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e751f add eax,0x30
            { let (r, f) = add::<32>((rax as u32 as u64), 0x30u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // e7522 movsx esi,al
            rsi = (sx::<8>((rax as u8 as u64)) as u64) as u32 as u64;
            // e7525 mov rdi,rbx
            rdi = rbx;
            // e7528 call 0xec020
            { let r = { let t = m.env.stub_va(0x7ed); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe752d;
        }
        0xe752d => {
            // block e752d
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e752d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e752d mov rax,rbx
            rax = rbx;
            // e7530 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e7534 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e7535 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e7537 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e7539 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e753b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e753d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e753e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe753f => {
            // block e753f
            #[cfg(feature = "trace")] eprintln!("f_000e73ea pc e753f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e753f mov r14,rax
            r14 = rax;
            // e7542 mov rdi,rbx
            rdi = rbx;
            // e7545 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e754a mov rdi,r14
            rdi = r14;
            // e754d call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7552 jmp 0xe753f
            pc = 0xe753f; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe5874 size=358
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e5874<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe5874;
    loop {
        match pc {
        0xe5874 => {
            // block e5874
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5874 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5874 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e5875 mov rbp,rsp
            rbp = rsp;
            // e5878 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e587a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e587c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e587e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e5880 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e5881 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e5883 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e5885 jne 0xe588a
            if fl.ne() { pc = 0xe588a; continue; }
            pc = 0xe5887;
        }
        0xe5887 => {
            // block e5887
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5887 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5887 mov eax,DWORD PTR [rdi+0x8]
            rax = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            pc = 0xe588a;
        }
        0xe588a => {
            // block e588a
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e588a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e588a dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e588c mov r11d,0x1e
            r11 = (0x1eu64) as u32 as u64;
            // e5892 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // e5895 ja 0xe58a4
            if fl.a() { pc = 0xe58a4; continue; }
            pc = 0xe5897;
        }
        0xe5897 => {
            // block e5897
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5897 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5897 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e5899 lea rbx,[rip+0x44440] # 0x129ce0
            rbx = m.env.data_va(255149);
            // e58a0 mov r11d,DWORD PTR [rbx+rax*4]
            r11 = (m.r32(rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe58a4;
        }
        0xe58a4 => {
            // block e58a4
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e58a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e58a4 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e58a6 jne 0xe58ab
            if fl.ne() { pc = 0xe58ab; continue; }
            pc = 0xe58a8;
        }
        0xe58a8 => {
            // block e58a8
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e58a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e58a8 mov edx,DWORD PTR [rdi+0x8]
            rdx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            pc = 0xe58ab;
        }
        0xe58ab => {
            // block e58ab
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e58ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e58ab mov r15,QWORD PTR [rbp+0x10]
            r15 = m.r64(rbp.wrapping_add(0x10)) as u64;
            // e58af imul r14d,r11d,0x3c
            { let (r, f) = imul::<32>((r11 as u32 as u64), 0x3cu64); r14 = (r) as u32 as u64; fl = f; }
            // e58b3 cmp edx,0x4
            fl = sub::<32>((rdx as u32 as u64), 0x4u64).1;
            // e58b6 je 0xe58c1
            if fl.e() { pc = 0xe58c1; continue; }
            pc = 0xe58b8;
        }
        0xe58b8 => {
            // block e58b8
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e58b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e58b8 cmp edx,0x8
            fl = sub::<32>((rdx as u32 as u64), 0x8u64).1;
            // e58bb jne 0xe5997
            if fl.ne() { pc = 0xe5997; continue; }
            pc = 0xe58c1;
        }
        0xe58c1 => {
            // block e58c1
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e58c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e58c1 cmp DWORD PTR [rdi+0x8],0x8
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x8)) as u64, 0x8u64).1;
            // e58c5 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // e58c8 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // e58cb lea eax,[rax+rax*1+0x2]
            rax = (rax.wrapping_add(rax).wrapping_add(0x2)) as u32 as u64;
            // e58cf mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // e58d2 mov r12d,r11d
            r12 = ((r11 as u32 as u64)) as u32 as u64;
            // e58d5 sub r12d,eax
            { let (r, f) = sub::<32>((r12 as u32 as u64), (rax as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // e58d8 imul edi,r11d,0x3b
            { let (r, f) = imul::<32>((r11 as u32 as u64), 0x3bu64); rdi = (r) as u32 as u64; fl = f; }
            // e58dc add edi,r12d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r12 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // e58df lea ebx,[rdi+rdi*8]
            rbx = (rdi.wrapping_add(rdi.wrapping_mul(8))) as u32 as u64;
            // e58e2 add ebx,r14d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r14 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // e58e5 lea eax,[rbx+rbx*1]
            rax = (rbx.wrapping_add(rbx)) as u32 as u64;
            // e58e8 lea r13d,[rax+rax*2]
            r13 = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // e58ec mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e58ee shl eax,0x4
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e58f1 lea r10d,[rax+rax*8]
            r10 = (rax.wrapping_add(rax.wrapping_mul(8))) as u32 as u64;
            // e58f5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e58f7 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e58f9 div r10d
            { let (q, r) = div::<32>(rdx, rax, (r10 as u32 as u64)); rax = q; rdx = r; }
            // e58fc mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e58fe xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5900 div r13d
            { let (q, r) = div::<32>(rdx, rax, (r13 as u32 as u64)); rax = q; rdx = r; }
            // e5903 mov esi,edx
            rsi = ((rdx as u32 as u64)) as u32 as u64;
            // e5905 mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            // e5907 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5909 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e590b div ebx
            { let (q, r) = div::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // e590d mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // e590f add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e5911 lea esi,[rax+rax*4]
            rsi = (rax.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // e5914 mov DWORD PTR [r8],esi
            m.w32(r8, ((rsi as u32 as u64)) as u32);
            // e5917 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e5919 sub eax,r14d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r14 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e591c jb 0xe592c
            if fl.b() { pc = 0xe592c; continue; }
            pc = 0xe591e;
        }
        0xe591e => {
            // block e591e
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e591e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e591e or esi,0x1
            { let r = (rsi as u32 as u64) | 0x1u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e5921 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5923 div edi
            { let (q, r) = div::<32>(rdx, rax, (rdi as u32 as u64)); rax = q; rdx = r; }
            // e5925 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // e5927 add eax,esi
            { let (r, f) = add::<32>((rax as u32 as u64), (rsi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e5929 mov DWORD PTR [r8],eax
            m.w32(r8, ((rax as u32 as u64)) as u32);
            pc = 0xe592c;
        }
        0xe592c => {
            // block e592c
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e592c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e592c mov DWORD PTR [r9],0x0
            m.w32(r9, (0x0u64) as u32);
            // e5933 mov eax,DWORD PTR [r8]
            rax = (m.r32(r8) as u64) as u32 as u64;
            // e5936 mov edx,0xcccccccd
            rdx = (0xcccccccdu64) as u32 as u64;
            // e593b imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // e593f shr rdx,0x23
            { let (r, f) = shr::<64>(rdx, 0x23u64, fl); rdx = r; fl = f; }
            // e5943 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5945 lea edx,[rdx+rdx*4]
            rdx = (rdx.wrapping_add(rdx.wrapping_mul(4))) as u32 as u64;
            // e5948 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // e594a je 0xe5961
            if fl.e() { pc = 0xe5961; continue; }
            pc = 0xe594c;
        }
        0xe594c => {
            // block e594c
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e594c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e594c mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e594e sub eax,r12d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r12 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e5951 jb 0xe596d
            if fl.b() { pc = 0xe596d; continue; }
            pc = 0xe5953;
        }
        0xe5953 => {
            // block e5953
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5953 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5953 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5955 div r11d
            { let (q, r) = div::<32>(rdx, rax, (r11 as u32 as u64)); rax = q; rdx = r; }
            // e5958 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // e595a inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e595c mov DWORD PTR [r9],eax
            m.w32(r9, ((rax as u32 as u64)) as u32);
            // e595f jmp 0xe596d
            pc = 0xe596d; continue;
        }
        0xe5961 => {
            // block e5961
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5961 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5961 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5963 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e5965 div r11d
            { let (q, r) = div::<32>(rdx, rax, (r11 as u32 as u64)); rax = q; rdx = r; }
            // e5968 mov DWORD PTR [r9],eax
            m.w32(r9, ((rax as u32 as u64)) as u32);
            // e596b mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0xe596d;
        }
        0xe596d => {
            // block e596d
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e596d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e596d mov DWORD PTR [r15],ecx
            m.w32(r15, ((rcx as u32 as u64)) as u32);
            // e5970 cmp DWORD PTR [r9],0x0
            fl = sub::<32>(m.r32(r9) as u64, 0x0u64).1;
            // e5974 jne 0xe59ce
            if fl.ne() { pc = 0xe59ce; continue; }
            pc = 0xe5976;
        }
        0xe5976 => {
            // block e5976
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5976 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5976 mov eax,DWORD PTR [r8]
            rax = (m.r32(r8) as u64) as u32 as u64;
            // e5979 mov edx,0xcccccccd
            rdx = (0xcccccccdu64) as u32 as u64;
            // e597e imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // e5982 shr rdx,0x23
            { let (r, f) = shr::<64>(rdx, 0x23u64, fl); rdx = r; fl = f; }
            // e5986 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5988 lea edx,[rdx+rdx*4]
            rdx = (rdx.wrapping_add(rdx.wrapping_mul(4))) as u32 as u64;
            // e598b cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // e598d je 0xe59ce
            if fl.e() { pc = 0xe59ce; continue; }
            pc = 0xe598f;
        }
        0xe598f => {
            // block e598f
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e598f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e598f add ecx,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e5992 mov DWORD PTR [r15],ecx
            m.w32(r15, ((rcx as u32 as u64)) as u32);
            // e5995 jmp 0xe59ce
            pc = 0xe59ce; continue;
        }
        0xe5997 => {
            // block e5997
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5997 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5997 imul edi,r11d,0xe10
            { let (r, f) = imul::<32>((r11 as u32 as u64), 0xe10u64); rdi = (r) as u32 as u64; fl = f; }
            // e599e imul ebx,r11d,0x15180
            { let (r, f) = imul::<32>((r11 as u32 as u64), 0x15180u64); rbx = (r) as u32 as u64; fl = f; }
            // e59a5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e59a7 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e59a9 div ebx
            { let (q, r) = div::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // e59ab mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e59ad xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e59af div edi
            { let (q, r) = div::<32>(rdx, rax, (rdi as u32 as u64)); rax = q; rdx = r; }
            // e59b1 mov esi,edx
            rsi = ((rdx as u32 as u64)) as u32 as u64;
            // e59b3 mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            // e59b5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e59b7 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e59b9 div r14d
            { let (q, r) = div::<32>(rdx, rax, (r14 as u32 as u64)); rax = q; rdx = r; }
            // e59bc mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // e59be mov DWORD PTR [r8],eax
            m.w32(r8, ((rax as u32 as u64)) as u32);
            // e59c1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e59c3 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e59c5 div r11d
            { let (q, r) = div::<32>(rdx, rax, (r11 as u32 as u64)); rax = q; rdx = r; }
            // e59c8 mov DWORD PTR [r9],eax
            m.w32(r9, ((rax as u32 as u64)) as u32);
            // e59cb mov DWORD PTR [r15],edx
            m.w32(r15, ((rdx as u32 as u64)) as u32);
            pc = 0xe59ce;
        }
        0xe59ce => {
            // block e59ce
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e59ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e59ce pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e59cf pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e59d1 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e59d3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e59d5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e59d7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e59d8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43edc size=354
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043edc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43edc;
    loop {
        match pc {
        0x43edc => {
            // block 43edc
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43edc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43edc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43edd mov rbp,rsp
            rbp = rsp;
            // 43ee0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 43ee2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43ee4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 43ee6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 43ee8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43ee9 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 43eed mov r12,rdx
            r12 = rdx;
            // 43ef0 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 43ef3 mov r15,rdi
            r15 = rdi;
            // 43ef6 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 43efd mov BYTE PTR [rbp-0x2d],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3), (0x0u64) as u8);
            // 43f01 lea rbx,[r12+0x8]
            rbx = r12.wrapping_add(0x8);
            // 43f06 mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 43f0b mov rdi,r12
            rdi = r12;
            // 43f0e call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f13 mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 43f1c mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 43f20 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 43f29 cmp r14d,0x7
            fl = sub::<32>((r14 as u32 as u64), 0x7u64).1;
            // 43f2d ja 0x43fae
            if fl.a() { pc = 0x43fae; continue; }
            pc = 0x43f2f;
        }
        0x43f2f => {
            // block 43f2f
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f2f movzx ebx,r14w
            rbx = ((r14 as u16 as u64)) as u32 as u64;
            // 43f33 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43f37 call 0xcf820
            { let r = { let t = m.env.code_va(0xcf820); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f3c movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 43f3f cmp ebx,eax
            fl = sub::<32>((rbx as u32 as u64), (rax as u32 as u64)).1;
            // 43f41 jae 0x43fae
            if fl.ae() { pc = 0x43fae; continue; }
            pc = 0x43f43;
        }
        0x43f43 => {
            // block 43f43
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f43 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43f46 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 43f4a mov rdi,r15
            rdi = r15;
            // 43f4d mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 43f50 call QWORD PTR [rax+0x688]
            { let r = { let t = m.r64(rax.wrapping_add(0x688)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f56 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43f58 je 0x43fae
            if fl.e() { pc = 0x43fae; continue; }
            pc = 0x43f5a;
        }
        0x43f5a => {
            // block 43f5a
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f5a mov ebx,DWORD PTR [rbp-0x2c]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 43f5d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43f60 lea rdx,[rbp-0x2d]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd3);
            // 43f64 mov rdi,r15
            rdi = r15;
            // 43f67 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 43f6a call QWORD PTR [rax+0x7e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x7e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f70 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 43f72 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43f74 mov r13d,0x0
            r13 = (0x0u64) as u32 as u64;
            // 43f7a je 0x43fb1
            if fl.e() { pc = 0x43fb1; continue; }
            pc = 0x43f7c;
        }
        0x43f7c => {
            // block 43f7c
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f7c cmp BYTE PTR [rbp-0x2d],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3)) as u64, 0x0u64).1;
            // 43f80 je 0x43fc3
            if fl.e() { pc = 0x43fc3; continue; }
            pc = 0x43f82;
        }
        0x43f82 => {
            // block 43f82
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f82 mov r15d,ebx
            r15 = ((rbx as u32 as u64)) as u32 as u64;
            // 43f85 shr r15d,1
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x1u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // 43f88 mov bx,0x1
            rbx = (rbx & !0xffff) | ((0x1u64) as u16 as u64);
            // 43f8c lea r14,[rbp-0x34]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            pc = 0x43f90;
        }
        0x43f90 => {
            // block 43f90
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f90 mov DWORD PTR [rbp-0x34],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rcx as u32 as u64)) as u32);
            // 43f93 mov rdi,r12
            rdi = r12;
            // 43f96 mov rsi,r14
            rsi = r14;
            // 43f99 call 0xebb58
            { let r = { let t = m.env.stub_va(0x325); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f9e movzx ecx,bx
            rcx = ((rbx as u16 as u64)) as u32 as u64;
            // 43fa1 lea ebx,[rbx+0x1]
            rbx = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 43fa4 cmp ecx,r15d
            fl = sub::<32>((rcx as u32 as u64), (r15 as u32 as u64)).1;
            // 43fa7 mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // 43faa jbe 0x43f90
            if fl.be() { pc = 0x43f90; continue; }
            pc = 0x43fac;
        }
        0x43fac => {
            // block 43fac
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fac jmp 0x43fb1
            pc = 0x43fb1; continue;
        }
        0x43fae => {
            // block 43fae
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fae xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x43fb1;
        }
        0x43fb1 => {
            // block 43fb1
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fb1 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 43fb4 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 43fb8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43fb9 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 43fbb pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 43fbd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43fbf pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 43fc1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43fc2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x43fc3 => {
            // block 43fc3
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fc3 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43fc7 call 0xcc811
            { let r = { let t = m.env.code_va(0xcc811); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43fcc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43fce je 0x44036
            if fl.e() { pc = 0x44036; continue; }
            pc = 0x43fd0;
        }
        0x43fd0 => {
            // block 43fd0
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fd0 mov DWORD PTR [rbp-0x38],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u32);
            // 43fd7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43fda cmp r14d,0x3
            fl = sub::<32>((r14 as u32 as u64), 0x3u64).1;
            // 43fde setg cl
            rcx = (rcx & !0xff) | (((fl.g() as u64)) as u8 as u64);
            // 43fe1 movzx esi,cl
            rsi = ((rcx as u8 as u64)) as u32 as u64;
            // 43fe4 or esi,0x1f0
            { let r = (rsi as u32 as u64) | 0x1f0u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 43fea lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 43fee xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 43ff1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 43ff6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 43ff9 mov rdi,r15
            rdi = r15;
            // 43ffc call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43fff test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44001 je 0x43fb1
            if fl.e() { pc = 0x43fb1; continue; }
            pc = 0x44003;
        }
        0x44003 => {
            // block 44003
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 44003 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44003 mov DWORD PTR [rbp-0x3c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), (0x0u64) as u32);
            // 4400a shl r14d,0x3
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x3u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 4400e lea r15,[rbp-0x3c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            pc = 0x44012;
        }
        0x44012 => {
            // block 44012
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 44012 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44012 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // 44015 lea ecx,[r14+r13*1]
            rcx = (r14.wrapping_add(r13)) as u32 as u64;
            // 44019 bt eax,ecx
            fl.cf = bit::<32>((rax as u32 as u64), (rcx as u32 as u64));
            // 4401c jae 0x44029
            if fl.ae() { pc = 0x44029; continue; }
            pc = 0x4401e;
        }
        0x4401e => {
            // block 4401e
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 4401e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4401e mov rdi,r12
            rdi = r12;
            // 44021 mov rsi,r15
            rsi = r15;
            // 44024 call 0xebb52
            { let r = { let t = m.env.stub_va(0x31f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x44029;
        }
        0x44029 => {
            // block 44029
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 44029 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44029 inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // 4402c mov DWORD PTR [rbp-0x3c],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((r13 as u32 as u64)) as u32);
            // 44030 cmp r13d,0x8
            fl = sub::<32>((r13 as u32 as u64), 0x8u64).1;
            // 44034 jb 0x44012
            if fl.b() { pc = 0x44012; continue; }
            pc = 0x44036;
        }
        0x44036 => {
            // block 44036
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 44036 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44036 mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // 44039 jmp 0x43fb1
            pc = 0x43fb1; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x64f38 size=348
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00064f38<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x64f38;
    loop {
        match pc {
        0x64f38 => {
            // block 64f38
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 64f38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64f38 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 64f39 mov rbp,rsp
            rbp = rsp;
            // 64f3c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 64f3e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 64f40 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 64f42 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 64f44 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 64f45 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 64f49 mov r14,rdx
            r14 = rdx;
            // 64f4c mov r15,rsi
            r15 = rsi;
            // 64f4f mov rbx,rdi
            rbx = rdi;
            // 64f52 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 64f56 mov rsi,rbx
            rsi = rbx;
            // 64f59 call 0xec074
            { let r = { let t = m.env.stub_va(0x841); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64f5e cmp BYTE PTR [rbp-0x40],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64, 0x0u64).1;
            // 64f62 je 0x65021
            if fl.e() { pc = 0x65021; continue; }
            pc = 0x64f68;
        }
        0x64f68 => {
            // block 64f68
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 64f68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64f68 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 64f6b mov r12,QWORD PTR [rax-0x18]
            r12 = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 64f6f mov rdi,QWORD PTR [r12+rbx*1+0x28]
            rdi = m.r64(r12.wrapping_add(rbx).wrapping_add(0x28)) as u64;
            // 64f74 mov eax,0xb0
            rax = (0xb0u64) as u32 as u64;
            // 64f79 and eax,DWORD PTR [r12+rbx*1+0x8]
            { let r = (rax as u32 as u64) & m.r32(r12.wrapping_add(rbx).wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 64f7e cmp eax,0x20
            fl = sub::<32>((rax as u32 as u64), 0x20u64).1;
            // 64f81 mov r13,r15
            r13 = r15;
            // 64f84 jne 0x64f8a
            if fl.ne() { pc = 0x64f8a; continue; }
            pc = 0x64f86;
        }
        0x64f86 => {
            // block 64f86
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 64f86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64f86 lea r13,[r15+r14*1]
            r13 = r15.wrapping_add(r14);
            pc = 0x64f8a;
        }
        0x64f8a => {
            // block 64f8a
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 64f8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64f8a lea r8,[rbx+r12*1]
            r8 = rbx.wrapping_add(r12);
            // 64f8e mov eax,DWORD PTR [rbx+r12*1+0x90]
            rax = (m.r32(rbx.wrapping_add(r12).wrapping_add(0x90)) as u64) as u32 as u64;
            // 64f96 cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // 64f99 jne 0x64ff0
            if fl.ne() { pc = 0x64ff0; continue; }
            pc = 0x64f9b;
        }
        0x64f9b => {
            // block 64f9b
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 64f9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64f9b mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // 64f9f lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 64fa3 mov rsi,r8
            rsi = r8;
            // 64fa6 mov QWORD PTR [rbp-0x50],r8
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r8) as u64);
            // 64faa call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64faf mov rsi,QWORD PTR [rip+0xdb1da] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 64fb6 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 64fba call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64fbf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 64fc2 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 64fc6 mov esi,0x20
            rsi = (0x20u64) as u32 as u64;
            // 64fcb mov rdi,rax
            rdi = rax;
            // 64fce call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64fd0 mov BYTE PTR [rbp-0x51],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffaf), ((rax as u8 as u64)) as u8);
            // 64fd3 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 64fd7 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64fdc movsx eax,BYTE PTR [rbp-0x51]
            rax = (sx::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffaf)) as u64) as u64) as u32 as u64;
            // 64fe0 mov DWORD PTR [rbx+r12*1+0x90],eax
            m.w32(rbx.wrapping_add(r12).wrapping_add(0x90), ((rax as u32 as u64)) as u32);
            // 64fe8 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 64fec mov r8,QWORD PTR [rbp-0x50]
            r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0x64ff0;
        }
        0x64ff0 => {
            // block 64ff0
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 64ff0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ff0 add r14,r15
            { let (r, f) = add::<64>(r14, r15);
              r14 = r;
              fl = f; }
            // 64ff3 movsx r9d,al
            r9 = (sx::<8>((rax as u8 as u64)) as u64) as u32 as u64;
            // 64ff7 mov rsi,r15
            rsi = r15;
            // 64ffa mov rdx,r13
            rdx = r13;
            // 64ffd mov rcx,r14
            rcx = r14;
            // 65000 call 0x65094
            { let r = { let t = m.env.code_va(0x65094); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65005 test rax,rax
            fl = logic::<64>(rax & rax);
            // 65008 jne 0x65021
            if fl.ne() { pc = 0x65021; continue; }
            pc = 0x6500a;
        }
        0x6500a => {
            // block 6500a
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 6500a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6500a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6500d mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 65011 lea rdi,[rbx+rax*1]
            rdi = rbx.wrapping_add(rax);
            // 65015 mov esi,DWORD PTR [rbx+rax*1+0x20]
            rsi = (m.r32(rbx.wrapping_add(rax).wrapping_add(0x20)) as u64) as u32 as u64;
            // 65019 or esi,0x5
            { let r = (rsi as u32 as u64) | 0x5u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 6501c call 0xec0fe
            { let r = { let t = m.env.stub_va(0x8cb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x65021;
        }
        0x65021 => {
            // block 65021
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 65021 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65021 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 65025 call 0xec07a
            { let r = { let t = m.env.stub_va(0x847); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6502a;
        }
        0x6502a => {
            // block 6502a
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 6502a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6502a mov rax,rbx
            rax = rbx;
            // 6502d add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 65031 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 65032 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 65034 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 65036 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 65038 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6503a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6503b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x6503c => {
            // block 6503c
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 6503c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6503c mov r14,rax
            r14 = rax;
            // 6503f jmp 0x65052
            pc = 0x65052; continue;
        }
        0x65041 => {
            // block 65041
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 65041 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65041 mov r14,rax
            r14 = rax;
            // 65044 jmp 0x6505b
            pc = 0x6505b; continue;
        }
        0x65046 => {
            // block 65046
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 65046 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65046 mov r14,rax
            r14 = rax;
            // 65049 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 6504d call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x65052;
        }
        0x65052 => {
            // block 65052
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 65052 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65052 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 65056 call 0xec07a
            { let r = { let t = m.env.stub_va(0x847); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6505b;
        }
        0x6505b => {
            // block 6505b
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 6505b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6505b mov r15,rbx
            r15 = rbx;
            // 6505e mov rdi,r14
            rdi = r14;
            // 65061 call 0xec158
            { let r = { let t = m.env.stub_va(0x925); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65066 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 65069 add r15,QWORD PTR [rax-0x18]
            { let (r, f) = add::<64>(r15, m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64);
              r15 = r;
              fl = f; }
            // 6506d mov rdi,r15
            rdi = r15;
            // 65070 call 0xec0f2
            { let r = { let t = m.env.stub_va(0x8bf); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65075 call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6507a jmp 0x6502a
            pc = 0x6502a; continue;
        }
        0x6507c => {
            // block 6507c
            #[cfg(feature = "trace")] eprintln!("f_00064f38 pc 6507c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6507c mov rbx,rax
            rbx = rax;
            // 6507f call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65084 mov rdi,rbx
            rdi = rbx;
            // 65087 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6508c mov rdi,rax
            rdi = rax;
            // 6508f call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5504 size=340
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005504<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5504;
    loop {
        match pc {
        0x5504 => {
            // block 5504
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 5504 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5504 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5505 mov rbp,rsp
            rbp = rsp;
            // 5508 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 550a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 550c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 550e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 550f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 5513 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 5516 mov r14,rsi
            r14 = rsi;
            // 5519 mov rbx,rdi
            rbx = rdi;
            // 551c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 551f call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5525 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 5528 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 552a cmove r12d,r15d
            if fl.e() { r12 = ((r15 as u32 as u64)) as u32 as u64; } else { r12 = r12 as u32 as u64; }
            // 552e mov DWORD PTR [rbp-0x24],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0xcu64) as u32);
            // 5535 mov DWORD PTR [rbp-0x28],0x12
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x12u64) as u32);
            // 553c mov DWORD PTR [r14],0x3
            m.w32(r14, (0x3u64) as u32);
            // 5543 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5546 mov rdi,rbx
            rdi = rbx;
            // 5549 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 554c call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5552 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5554 jne 0x5618
            if fl.ne() { pc = 0x5618; continue; }
            pc = 0x555a;
        }
        0x555a => {
            // block 555a
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 555a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 555a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 555d lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 5561 mov rdi,rbx
            rdi = rbx;
            // 5564 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 5567 call QWORD PTR [rax+0x3e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x3e8)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 556d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5570 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 5574 mov rdi,rbx
            rdi = rbx;
            // 5577 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 557a call QWORD PTR [rax+0x380]
            { let r = { let t = m.r64(rax.wrapping_add(0x380)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5580 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 5583 mov ecx,DWORD PTR [rbp-0x24]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 5586 cmp rcx,0xb
            fl = sub::<64>(rcx, 0xbu64).1;
            // 558a ja 0x5618
            if fl.a() { pc = 0x5618; continue; }
            pc = 0x5590;
        }
        0x5590 => {
            // block 5590
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 5590 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5590 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 5595 lea rdx,[rip+0x8c] # 0x5628
            rdx = m.env.code_va(0x5628);
            // 559c movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // 55a0 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // 55a3 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x55a5 => {
            // block 55a5
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55a5 mov eax,DWORD PTR [rbp-0x28]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 55a8 lea ecx,[rax-0x4]
            rcx = (rax.wrapping_add(0xffff_ffff_ffff_fffc)) as u32 as u64;
            // 55ab cmp ecx,0x9
            fl = sub::<32>((rcx as u32 as u64), 0x9u64).1;
            // 55ae ja 0x55c0
            if fl.a() { pc = 0x55c0; continue; }
            pc = 0x55b0;
        }
        0x55b0 => {
            // block 55b0
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55b0 movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // 55b3 lea rdx,[rip+0xfa276] # 0xff830
            rdx = m.env.data_va(81917);
            // 55ba mov r15b,BYTE PTR [rcx+rdx*1]
            r15 = (r15 & !0xff) | ((m.r8(rcx.wrapping_add(rdx)) as u64) as u8 as u64);
            // 55be jmp 0x55c3
            pc = 0x55c3; continue;
        }
        0x55c0 => {
            // block 55c0
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55c0 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x55c3;
        }
        0x55c3 => {
            // block 55c3
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55c3 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 55c6 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 55c9 jmp 0x55f7
            pc = 0x55f7; continue;
        }
        0x55cb => {
            // block 55cb
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55cb cmp DWORD PTR [rbp-0x28],0x9
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, 0x9u64).1;
            // 55cf jmp 0x5605
            pc = 0x5605; continue;
        }
        0x55d1 => {
            // block 55d1
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55d1 mov eax,DWORD PTR [rbp-0x28]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 55d4 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 55d6 and ecx,0xfffffff7
            { let r = (rcx as u32 as u64) & 0xfffffff7u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 55d9 cmp ecx,0x6
            fl = sub::<32>((rcx as u32 as u64), 0x6u64).1;
            // 55dc sete r15b
            r15 = (r15 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 55e0 cmp eax,0xe
            fl = sub::<32>((rax as u32 as u64), 0xeu64).1;
            // 55e3 jmp 0x55f7
            pc = 0x55f7; continue;
        }
        0x55e5 => {
            // block 55e5
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55e5 mov eax,DWORD PTR [rbp-0x28]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 55e8 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 55ea and ecx,0xfffffff7
            { let r = (rcx as u32 as u64) & 0xfffffff7u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 55ed cmp ecx,0x7
            fl = sub::<32>((rcx as u32 as u64), 0x7u64).1;
            // 55f0 sete r15b
            r15 = (r15 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 55f4 cmp eax,0xf
            fl = sub::<32>((rax as u32 as u64), 0xfu64).1;
            pc = 0x55f7;
        }
        0x55f7 => {
            // block 55f7
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55f7 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 55fa movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 55fd inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 55ff jmp 0x5609
            pc = 0x5609; continue;
        }
        0x5601 => {
            // block 5601
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 5601 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5601 cmp DWORD PTR [rbp-0x28],0xc
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, 0xcu64).1;
            pc = 0x5605;
        }
        0x5605 => {
            // block 5605
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 5605 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5605 sete r15b
            r15 = (r15 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x5609;
        }
        0x5609 => {
            // block 5609
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 5609 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5609 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 560b test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // 560f cmovne ecx,eax
            if fl.ne() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 5612 mov DWORD PTR [r14],ecx
            m.w32(r14, ((rcx as u32 as u64)) as u32);
            // 5615 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x5618;
        }
        0x5618 => {
            // block 5618
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 5618 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5618 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 561b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 561f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5620 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5622 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5624 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5626 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5627 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc75d9 size=339
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c75d9<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc75d9;
    loop {
        match pc {
        0xc75d9 => {
            // block c75d9
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c75d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c75d9 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c75da mov rbp,rsp
            rbp = rsp;
            // c75dd push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c75df push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c75e1 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c75e3 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c75e5 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c75e6 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c75e8 je 0xc7707
            if fl.e() { pc = 0xc7707; continue; }
            pc = 0xc75ee;
        }
        0xc75ee => {
            // block c75ee
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c75ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c75ee and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c75f1 lea r9,[rdi+0x2]
            r9 = rdi.wrapping_add(0x2);
            // c75f5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // c75f8 mov r10d,0xaaaaaaab
            r10 = (0xaaaaaaabu64) as u32 as u64;
            // c75fe lea r11,[rip+0x10f] # 0xc7714
            r11 = m.env.code_va(0xc7714);
            pc = 0xc7605;
        }
        0xc7605 => {
            // block c7605
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7605 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7605 cmp r8d,0x2
            fl = sub::<32>((r8 as u32 as u64), 0x2u64).1;
            // c7609 je 0xc7617
            if fl.e() { pc = 0xc7617; continue; }
            pc = 0xc760b;
        }
        0xc760b => {
            // block c760b
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c760b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c760b cmp r8d,0x3
            fl = sub::<32>((r8 as u32 as u64), 0x3u64).1;
            // c760f jne 0xc7627
            if fl.ne() { pc = 0xc7627; continue; }
            pc = 0xc7611;
        }
        0xc7611 => {
            // block c7611
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7611 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7611 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // c7613 jne 0xc761b
            if fl.ne() { pc = 0xc761b; continue; }
            pc = 0xc7615;
        }
        0xc7615 => {
            // block c7615
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7615 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7615 jmp 0xc763c
            pc = 0xc763c; continue;
        }
        0xc7617 => {
            // block c7617
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7617 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7617 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // c7619 jne 0xc763c
            if fl.ne() { pc = 0xc763c; continue; }
            pc = 0xc761b;
        }
        0xc761b => {
            // block c761b
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c761b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c761b lea r14,[rdi+r12*8+0x2]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x2);
            // c7620 lea r15,[rdi+r12*8+0x4]
            r15 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4);
            // c7625 jmp 0xc7663
            pc = 0xc7663; continue;
        }
        0xc7627 => {
            // block c7627
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7627 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7627 cmp r8d,0x4
            fl = sub::<32>((r8 as u32 as u64), 0x4u64).1;
            // c762b jne 0xc7642
            if fl.ne() { pc = 0xc7642; continue; }
            pc = 0xc762d;
        }
        0xc762d => {
            // block c762d
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c762d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c762d test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // c762f jne 0xc7650
            if fl.ne() { pc = 0xc7650; continue; }
            pc = 0xc7631;
        }
        0xc7631 => {
            // block c7631
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7631 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7631 lea r14,[rdi+r12*8+0x2]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x2);
            // c7636 lea r15,[rdi+r12*8]
            r15 = rdi.wrapping_add(r12.wrapping_mul(8));
            // c763a jmp 0xc7663
            pc = 0xc7663; continue;
        }
        0xc763c => {
            // block c763c
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c763c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c763c lea r14,[rdi+r12*8]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8));
            // c7640 jmp 0xc7655
            pc = 0xc7655; continue;
        }
        0xc7642 => {
            // block c7642
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7642 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7642 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // c7644 jne 0xc765c
            if fl.ne() { pc = 0xc765c; continue; }
            pc = 0xc7646;
        }
        0xc7646 => {
            // block c7646
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7646 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7646 lea r14,[rdi+r12*8+0x4]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4);
            // c764b mov r15,r9
            r15 = r9;
            // c764e jmp 0xc7663
            pc = 0xc7663; continue;
        }
        0xc7650 => {
            // block c7650
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7650 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7650 lea r14,[rdi+r12*8+0x4]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4);
            pc = 0xc7655;
        }
        0xc7655 => {
            // block c7655
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7655 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7655 lea r15,[rdi+r12*8+0x2]
            r15 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x2);
            // c765a jmp 0xc7663
            pc = 0xc7663; continue;
        }
        0xc765c => {
            // block c765c
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c765c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c765c lea r15,[rdi+r12*8]
            r15 = rdi.wrapping_add(r12.wrapping_mul(8));
            // c7660 mov r14,r9
            r14 = r9;
            pc = 0xc7663;
        }
        0xc7663 => {
            // block c7663
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7663 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7663 mov r13d,r12d
            r13 = ((r12 as u32 as u64)) as u32 as u64;
            // c7666 imul r13,r10
            { let (r, f) = imul::<64>(r13, r10); r13 = r; fl = f; }
            // c766a mov rax,r13
            rax = r13;
            // c766d shr rax,0x22
            { let (r, f) = shr::<64>(rax, 0x22u64, fl); rax = r; fl = f; }
            // c7671 add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c7673 lea ebx,[rax+rax*2]
            rbx = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // c7676 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // c7679 sub eax,ebx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c767b cmp eax,0x5
            fl = sub::<32>((rax as u32 as u64), 0x5u64).1;
            // c767e ja 0xc76f7
            if fl.a() { pc = 0xc76f7; continue; }
            pc = 0xc7680;
        }
        0xc7680 => {
            // block c7680
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7680 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7680 movzx r14d,WORD PTR [r14]
            r14 = (m.r16(r14) as u64) as u32 as u64;
            // c7684 movzx r15d,WORD PTR [r15]
            r15 = (m.r16(r15) as u64) as u32 as u64;
            // c7688 shr r13,0x21
            { let (r, f) = shr::<64>(r13, 0x21u64, fl); r13 = r; fl = f; }
            // c768c movsxd rax,DWORD PTR [r11+rax*4]
            rax = sx::<32>(m.r32(r11.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // c7690 add rax,r11
            { let (r, f) = add::<64>(rax, r11);
              rax = r;
              fl = f; }
            // c7693 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc7695 => {
            // block c7695
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7695 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7695 shr r14d,0x4
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x4u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c7699 and r14d,0xffc
            { let r = (r14 as u32 as u64) & 0xffcu64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // c76a0 mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c76a3 mov DWORD PTR [rsi+rax*4],r14d
            m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), ((r14 as u32 as u64)) as u32);
            // c76a7 jmp 0xc76f7
            pc = 0xc76f7; continue;
        }
        0xc76a9 => {
            // block c76a9
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76a9 shl r15d,0x6
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x6u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // c76ad and r15d,0x3ff000
            { let r = (r15 as u32 as u64) & 0x3ff000u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c76b4 jmp 0xc76f0
            pc = 0xc76f0; continue;
        }
        0xc76b6 => {
            // block c76b6
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76b6 and r14d,0xffffffc0
            { let r = (r14 as u32 as u64) & 0xffffffc0u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // c76ba shl r14d,0x10
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x10u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c76be jmp 0xc76df
            pc = 0xc76df; continue;
        }
        0xc76c0 => {
            // block c76c0
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76c0 shr r15d,0x4
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x4u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // c76c4 and r15d,0xffc
            { let r = (r15 as u32 as u64) & 0xffcu64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c76cb mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c76ce mov DWORD PTR [rsi+rax*4],r15d
            m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), ((r15 as u32 as u64)) as u32);
            // c76d2 jmp 0xc76f7
            pc = 0xc76f7; continue;
        }
        0xc76d4 => {
            // block c76d4
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76d4 shl r14d,0x6
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x6u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c76d8 and r14d,0x3ff000
            { let r = (r14 as u32 as u64) & 0x3ff000u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc76df;
        }
        0xc76df => {
            // block c76df
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76df mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c76e2 or DWORD PTR [rsi+rax*4],r14d
            { let r = m.r32(rsi.wrapping_add(rax.wrapping_mul(4))) as u64 | (r14 as u32 as u64); let f = logic::<32>(r);
              m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), (r) as u32);
              fl = f; }
            // c76e6 jmp 0xc76f7
            pc = 0xc76f7; continue;
        }
        0xc76e8 => {
            // block c76e8
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76e8 and r15d,0xffffffc0
            { let r = (r15 as u32 as u64) & 0xffffffc0u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c76ec shl r15d,0x10
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x10u64, fl); r15 = (r) as u32 as u64; fl = f; }
            pc = 0xc76f0;
        }
        0xc76f0 => {
            // block c76f0
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76f0 mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c76f3 or DWORD PTR [rsi+rax*4],r15d
            { let r = m.r32(rsi.wrapping_add(rax.wrapping_mul(4))) as u64 | (r15 as u32 as u64); let f = logic::<32>(r);
              m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), (r) as u32);
              fl = f; }
            pc = 0xc76f7;
        }
        0xc76f7 => {
            // block c76f7
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76f7 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // c76fa add r9,0x8
            { let (r, f) = add::<64>(r9, 0x8u64);
              r9 = r;
              fl = f; }
            // c76fe cmp edx,r12d
            fl = sub::<32>((rdx as u32 as u64), (r12 as u32 as u64)).1;
            // c7701 jne 0xc7605
            if fl.ne() { pc = 0xc7605; continue; }
            pc = 0xc7707;
        }
        0xc7707 => {
            // block c7707
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7707 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7707 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7708 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c770a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c770c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c770e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c7710 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7711 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2cbac size=330
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002cbac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2cbac;
    loop {
        match pc {
        0x2cbac => {
            // block 2cbac
            #[cfg(feature = "trace")] eprintln!("f_0002cbac pc 2cbac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cbac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2cbad mov rbp,rsp
            rbp = rsp;
            // 2cbb0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2cbb2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2cbb4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2cbb6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2cbb8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2cbb9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2cbba mov r14,rdx
            r14 = rdx;
            // 2cbbd mov r15,rsi
            r15 = rsi;
            // 2cbc0 mov rbx,rdi
            rbx = rdi;
            // 2cbc3 mov DWORD PTR [rbx],0x0
            m.w32(rbx, (0x0u64) as u32);
            // 2cbc9 mov DWORD PTR [rbx+0x4],0x0
            m.w32(rbx.wrapping_add(0x4), (0x0u64) as u32);
            // 2cbd0 mov DWORD PTR [rbx+0x8],0x0
            m.w32(rbx.wrapping_add(0x8), (0x0u64) as u32);
            // 2cbd7 mov DWORD PTR [rbx+0xc],0x0
            m.w32(rbx.wrapping_add(0xc), (0x0u64) as u32);
            // 2cbde mov DWORD PTR [rbx+0x10],0x7c
            m.w32(rbx.wrapping_add(0x10), (0x7cu64) as u32);
            // 2cbe5 mov DWORD PTR [rbx+0x14],0x8
            m.w32(rbx.wrapping_add(0x14), (0x8u64) as u32);
            // 2cbec mov DWORD PTR [rbx+0x18],0x0
            m.w32(rbx.wrapping_add(0x18), (0x0u64) as u32);
            // 2cbf3 mov DWORD PTR [rbx+0x1c],0x0
            m.w32(rbx.wrapping_add(0x1c), (0x0u64) as u32);
            // 2cbfa movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 2cbfd mov DWORD PTR [rbx+0x20],eax
            m.w32(rbx.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // 2cc00 lea r12,[rbx+0x24]
            r12 = rbx.wrapping_add(0x24);
            // 2cc04 mov QWORD PTR [rbx+0x44],0x0
            m.w64(rbx.wrapping_add(0x44), (0x0u64) as u64);
            // 2cc0c mov QWORD PTR [rbx+0x3c],0x0
            m.w64(rbx.wrapping_add(0x3c), (0x0u64) as u64);
            // 2cc14 mov QWORD PTR [rbx+0x34],0x0
            m.w64(rbx.wrapping_add(0x34), (0x0u64) as u64);
            // 2cc1c mov QWORD PTR [rbx+0x2c],0x0
            m.w64(rbx.wrapping_add(0x2c), (0x0u64) as u64);
            // 2cc24 mov QWORD PTR [rbx+0x24],0x0
            m.w64(rbx.wrapping_add(0x24), (0x0u64) as u64);
            // 2cc2c mov esi,0x10
            rsi = (0x10u64) as u32 as u64;
            // 2cc31 mov rdi,r12
            rdi = r12;
            // 2cc34 call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cc39 mov rdi,QWORD PTR [rbx+0x24]
            rdi = m.r64(rbx.wrapping_add(0x24)) as u64;
            // 2cc3d test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2cc40 je 0x2cc4a
            if fl.e() { pc = 0x2cc4a; continue; }
            pc = 0x2cc42;
        }
        0x2cc42 => {
            // block 2cc42
            #[cfg(feature = "trace")] eprintln!("f_0002cbac pc 2cc42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cc42 mov esi,DWORD PTR [rbx+0x2c]
            rsi = (m.r32(rbx.wrapping_add(0x2c)) as u64) as u32 as u64;
            // 2cc45 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2cc4a;
        }
        0x2cc4a => {
            // block 2cc4a
            #[cfg(feature = "trace")] eprintln!("f_0002cbac pc 2cc4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cc4a lea r13,[rbx+0x4c]
            r13 = rbx.wrapping_add(0x4c);
            // 2cc4e mov QWORD PTR [rbx+0x6c],0x0
            m.w64(rbx.wrapping_add(0x6c), (0x0u64) as u64);
            // 2cc56 mov QWORD PTR [rbx+0x64],0x0
            m.w64(rbx.wrapping_add(0x64), (0x0u64) as u64);
            // 2cc5e mov QWORD PTR [rbx+0x5c],0x0
            m.w64(rbx.wrapping_add(0x5c), (0x0u64) as u64);
            // 2cc66 mov QWORD PTR [rbx+0x54],0x0
            m.w64(rbx.wrapping_add(0x54), (0x0u64) as u64);
            // 2cc6e mov QWORD PTR [rbx+0x4c],0x0
            m.w64(rbx.wrapping_add(0x4c), (0x0u64) as u64);
            // 2cc76 mov esi,0x10
            rsi = (0x10u64) as u32 as u64;
            // 2cc7b mov rdi,r13
            rdi = r13;
            // 2cc7e call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cc83 mov rdi,QWORD PTR [r13+0x0]
            rdi = m.r64(r13) as u64;
            // 2cc87 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2cc8a je 0x2cc94
            if fl.e() { pc = 0x2cc94; continue; }
            pc = 0x2cc8c;
        }
        0x2cc8c => {
            // block 2cc8c
            #[cfg(feature = "trace")] eprintln!("f_0002cbac pc 2cc8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cc8c mov esi,DWORD PTR [rbx+0x54]
            rsi = (m.r32(rbx.wrapping_add(0x54)) as u64) as u32 as u64;
            // 2cc8f call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2cc94;
        }
        0x2cc94 => {
            // block 2cc94
            #[cfg(feature = "trace")] eprintln!("f_0002cbac pc 2cc94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cc94 movabs rax,0x6e74763200000000
            rax = 0x6e74_7632_0000_0000u64;
            // 2cc9e mov QWORD PTR [rbx+0x74],rax
            m.w64(rbx.wrapping_add(0x74), (rax) as u64);
            // 2cca2 mov rax,QWORD PTR [rbx+0x24]
            rax = m.r64(rbx.wrapping_add(0x24)) as u64;
            // 2cca6 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2cca9 je 0x2ccb9
            if fl.e() { pc = 0x2ccb9; continue; }
            pc = 0x2ccab;
        }
        0x2ccab => {
            // block 2ccab
            #[cfg(feature = "trace")] eprintln!("f_0002cbac pc 2ccab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ccab mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 2ccae mov rdx,QWORD PTR [r15+0x8]
            rdx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 2ccb2 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 2ccb6 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            pc = 0x2ccb9;
        }
        0x2ccb9 => {
            // block 2ccb9
            #[cfg(feature = "trace")] eprintln!("f_0002cbac pc 2ccb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ccb9 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 2ccbd test rax,rax
            fl = logic::<64>(rax & rax);
            // 2ccc0 je 0x2ccd0
            if fl.e() { pc = 0x2ccd0; continue; }
            pc = 0x2ccc2;
        }
        0x2ccc2 => {
            // block 2ccc2
            #[cfg(feature = "trace")] eprintln!("f_0002cbac pc 2ccc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ccc2 mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // 2ccc5 mov rdx,QWORD PTR [r14+0x8]
            rdx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 2ccc9 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 2cccd mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            pc = 0x2ccd0;
        }
        0x2ccd0 => {
            // block 2ccd0
            #[cfg(feature = "trace")] eprintln!("f_0002cbac pc 2ccd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ccd0 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2ccd4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2ccd5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2ccd7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2ccd9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2ccdb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2ccdd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ccde ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2ccdf => {
            // block 2ccdf
            #[cfg(feature = "trace")] eprintln!("f_0002cbac pc 2ccdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ccdf mov rbx,rax
            rbx = rax;
            // 2cce2 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2cce4 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2cce6 mov rdi,r12
            rdi = r12;
            // 2cce9 call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ccee mov rdi,rbx
            rdi = rbx;
            // 2ccf1 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9608 size=312
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9608<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe9608;
    loop {
        match pc {
        0xe9608 => {
            // block e9608
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9608 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9608 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9609 mov rbp,rsp
            rbp = rsp;
            // e960c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e960e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9610 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9611 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e9612 mov r14,rsi
            r14 = rsi;
            // e9615 mov rbx,rdi
            rbx = rdi;
            // e9618 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // e961f movzx eax,BYTE PTR [rdi]
            rax = (m.r8(rdi) as u64) as u32 as u64;
            // e9622 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // e9624 jne 0xe962b
            if fl.ne() { pc = 0xe962b; continue; }
            pc = 0xe9626;
        }
        0xe9626 => {
            // block e9626
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9626 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9626 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // e9629 jmp 0xe962f
            pc = 0xe962f; continue;
        }
        0xe962b => {
            // block e962b
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e962b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e962b mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            pc = 0xe962f;
        }
        0xe962f => {
            // block e962f
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e962f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e962f test rax,rax
            fl = logic::<64>(rax & rax);
            // e9632 je 0xe9662
            if fl.e() { pc = 0xe9662; continue; }
            pc = 0xe9634;
        }
        0xe9634 => {
            // block e9634
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9634 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9634 lea rsi,[rip+0x407ed] # 0x129e28
            rsi = m.env.data_va(255477);
            // e963b call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9640 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e9642 je 0xe9662
            if fl.e() { pc = 0xe9662; continue; }
            pc = 0xe9644;
        }
        0xe9644 => {
            // block e9644
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9644 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9644 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // e964b mov WORD PTR [rbp-0x1e],0x0
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffe2), (0x0u64) as u16);
            // e9651 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // e9658 test BYTE PTR [rdi],0x1
            fl = logic::<8>(m.r8(rdi) as u64 & 0x1u64);
            // e965b jne 0xe966e
            if fl.ne() { pc = 0xe966e; continue; }
            pc = 0xe965d;
        }
        0xe965d => {
            // block e965d
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e965d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e965d inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e9660 jmp 0xe9672
            pc = 0xe9672; continue;
        }
        0xe9662 => {
            // block e9662
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9662 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9662 mov DWORD PTR [r14+0x8],0x0
            m.w32(r14.wrapping_add(0x8), (0x0u64) as u32);
            // e966a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e966c jmp 0xe96ca
            pc = 0xe96ca; continue;
        }
        0xe966e => {
            // block e966e
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e966e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e966e mov rdi,QWORD PTR [rdi+0x10]
            rdi = m.r64(rdi.wrapping_add(0x10)) as u64;
            pc = 0xe9672;
        }
        0xe9672 => {
            // block e9672
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9672 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9672 lea r15,[r14+0x2]
            r15 = r14.wrapping_add(0x2);
            // e9676 lea rsi,[rip+0x53caa] # 0x13d327
            rsi = m.env.data_va(334580);
            // e967d lea rdx,[rbp-0x1e]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe2);
            // e9681 lea r8,[rbp-0x1c]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e9685 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9687 mov rcx,r15
            rcx = r15;
            // e968a call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e968f mov al,BYTE PTR [rbp-0x1e]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe2)) as u64) as u8 as u64);
            // e9692 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // e9695 mov DWORD PTR [r14+0x4],0x0
            m.w32(r14.wrapping_add(0x4), (0x0u64) as u32);
            // e969d mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // e96a0 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // e96a4 ja 0xe96c0
            if fl.a() { pc = 0xe96c0; continue; }
            pc = 0xe96a6;
        }
        0xe96a6 => {
            // block e96a6
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96a6 lea rcx,[rip+0x7f] # 0xe972c
            rcx = m.env.code_va(0xe972c);
            // e96ad movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // e96b1 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // e96b4 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xe96b6 => {
            // block e96b6
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96b6 mov DWORD PTR [r14+0x8],0x1
            m.w32(r14.wrapping_add(0x8), (0x1u64) as u32);
            // e96be jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
        0xe96c0 => {
            // block e96c0
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96c0 mov DWORD PTR [r14+0x8],0x0
            m.w32(r14.wrapping_add(0x8), (0x0u64) as u32);
            pc = 0xe96c8;
        }
        0xe96c8 => {
            // block e96c8
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96c8 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe96ca;
        }
        0xe96ca => {
            // block e96ca
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96ca add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e96ce pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e96cf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e96d1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e96d3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e96d4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe96d5 => {
            // block e96d5
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96d5 mov DWORD PTR [r14+0x8],0x2
            m.w32(r14.wrapping_add(0x8), (0x2u64) as u32);
            // e96dd jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
        0xe96df => {
            // block e96df
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96df mov DWORD PTR [r14+0x8],0x3
            m.w32(r14.wrapping_add(0x8), (0x3u64) as u32);
            // e96e7 jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
        0xe96e9 => {
            // block e96e9
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96e9 mov DWORD PTR [r14+0x8],0x4
            m.w32(r14.wrapping_add(0x8), (0x4u64) as u32);
            // e96f1 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // e96f8 test BYTE PTR [rdi],0x1
            fl = logic::<8>(m.r8(rdi) as u64 & 0x1u64);
            // e96fb jne 0xe9702
            if fl.ne() { pc = 0xe9702; continue; }
            pc = 0xe96fd;
        }
        0xe96fd => {
            // block e96fd
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96fd inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e9700 jmp 0xe9706
            pc = 0xe9706; continue;
        }
        0xe9702 => {
            // block e9702
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9702 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9702 mov rdi,QWORD PTR [rdi+0x10]
            rdi = m.r64(rdi.wrapping_add(0x10)) as u64;
            pc = 0xe9706;
        }
        0xe9706 => {
            // block e9706
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9706 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9706 lea rsi,[rip+0x53c2a] # 0x13d337
            rsi = m.env.data_va(334596);
            // e970d lea rdx,[rbp-0x1e]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe2);
            // e9711 lea r8,[rbp-0x1c]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e9715 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9717 mov r9,r15
            r9 = r15;
            // e971a mov rcx,r15
            rcx = r15;
            // e971d call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9722 mov al,BYTE PTR [rbp-0x1e]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe2)) as u64) as u8 as u64);
            // e9725 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // e9728 jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x331bc size=308
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000331bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x331bc;
    loop {
        match pc {
        0x331bc => {
            // block 331bc
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 331bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 331bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 331bd mov rbp,rsp
            rbp = rsp;
            // 331c0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 331c2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 331c4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 331c6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 331c8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 331c9 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 331cd mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 331d1 mov QWORD PTR [rbp-0x30],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rdx) as u64);
            // 331d5 mov QWORD PTR [rbp-0x40],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rsi) as u64);
            // 331d9 mov r13,rdi
            r13 = rdi;
            // 331dc mov DWORD PTR [rcx],0xffffffff
            m.w32(rcx, (0xffffffffu64) as u32);
            // 331e2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 331e4 cmp rsi,rdx
            fl = sub::<64>(rsi, rdx).1;
            // 331e7 je 0x332e0
            if fl.e() { pc = 0x332e0; continue; }
            pc = 0x331ed;
        }
        0x331ed => {
            // block 331ed
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 331ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 331ed test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 331f0 je 0x332e0
            if fl.e() { pc = 0x332e0; continue; }
            pc = 0x331f6;
        }
        0x331f6 => {
            // block 331f6
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 331f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 331f6 cmp QWORD PTR [rbp-0x30],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, 0x0u64).1;
            // 331fb je 0x332e0
            if fl.e() { pc = 0x332e0; continue; }
            pc = 0x33201;
        }
        0x33201 => {
            // block 33201
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33201 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33201 cmp DWORD PTR [r13+0x0],0x0
            fl = sub::<32>(m.r32(r13) as u64, 0x0u64).1;
            // 33206 je 0x332de
            if fl.e() { pc = 0x332de; continue; }
            pc = 0x3320c;
        }
        0x3320c => {
            // block 3320c
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 3320c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3320c cmp DWORD PTR [r13+0x4],0x0
            fl = sub::<32>(m.r32(r13.wrapping_add(0x4)) as u64, 0x0u64).1;
            // 33211 je 0x332de
            if fl.e() { pc = 0x332de; continue; }
            pc = 0x33217;
        }
        0x33217 => {
            // block 33217
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33217 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33217 mov cx,WORD PTR [r13+0x34]
            rcx = (rcx & !0xffff) | ((m.r16(r13.wrapping_add(0x34)) as u64) as u16 as u64);
            // 3321c test cx,cx
            fl = logic::<16>((rcx as u16 as u64) & (rcx as u16 as u64));
            // 3321f je 0x332de
            if fl.e() { pc = 0x332de; continue; }
            pc = 0x33225;
        }
        0x33225 => {
            // block 33225
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33225 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33225 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33227 cmp DWORD PTR [r13+0x24],0x0
            fl = sub::<32>(m.r32(r13.wrapping_add(0x24)) as u64, 0x0u64).1;
            // 3322c je 0x332e0
            if fl.e() { pc = 0x332e0; continue; }
            pc = 0x33232;
        }
        0x33232 => {
            // block 33232
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33232 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33232 movzx eax,cx
            rax = ((rcx as u16 as u64)) as u32 as u64;
            // 33235 mov DWORD PTR [rbp-0x44],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), ((rax as u32 as u64)) as u32);
            // 33238 mov DWORD PTR [rbp-0x48],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0x0u64) as u32);
            pc = 0x3323f;
        }
        0x3323f => {
            // block 3323f
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 3323f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3323f mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 33243 mov DWORD PTR [rax],0x0
            m.w32(rax, (0x0u64) as u32);
            // 33249 cmp DWORD PTR [r13+0x0],0x0
            fl = sub::<32>(m.r32(r13) as u64, 0x0u64).1;
            // 3324e je 0x332bc
            if fl.e() { pc = 0x332bc; continue; }
            pc = 0x33250;
        }
        0x33250 => {
            // block 33250
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33250 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33250 mov eax,DWORD PTR [rbp-0x48]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 33253 movzx ebx,ax
            rbx = ((rax as u16 as u64)) as u32 as u64;
            // 33256 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            pc = 0x3325c;
        }
        0x3325c => {
            // block 3325c
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 3325c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3325c lea r15d,[r12-0x1]
            r15 = (r12.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 33261 mov rdi,r13
            rdi = r13;
            // 33264 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 33267 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 33269 call 0x330d0
            { let r = { let t = m.env.code_va(0x330d0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3326e mov r14,rax
            r14 = rax;
            // 33271 mov rdi,r13
            rdi = r13;
            // 33274 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 33278 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 3327b mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 3327d call 0x330d0
            { let r = { let t = m.env.code_va(0x330d0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33282 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 33284 cmp DWORD PTR [rbp-0x44],ebx
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64, (rbx as u32 as u64)).1;
            // 33287 jbe 0x3328e
            if fl.be() { pc = 0x3328e; continue; }
            pc = 0x33289;
        }
        0x33289 => {
            // block 33289
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33289 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33289 mov ecx,DWORD PTR [r13+rbx*4+0x24]
            rcx = (m.r32(r13.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0x24)) as u64) as u32 as u64;
            pc = 0x3328e;
        }
        0x3328e => {
            // block 3328e
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 3328e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3328e mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 33290 mov rdi,r14
            rdi = r14;
            // 33293 mov rsi,rax
            rsi = rax;
            // 33296 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3329b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 3329d mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 3329f test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 332a1 mov rsi,QWORD PTR [rbp-0x40]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 332a5 jne 0x332e0
            if fl.ne() { pc = 0x332e0; continue; }
            pc = 0x332a7;
        }
        0x332a7 => {
            // block 332a7
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332a7 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 332ab mov DWORD PTR [rax],r12d
            m.w32(rax, ((r12 as u32 as u64)) as u32);
            // 332ae cmp r12d,DWORD PTR [r13+0x0]
            fl = sub::<32>((r12 as u32 as u64), m.r32(r13) as u64).1;
            // 332b2 lea eax,[r12+0x1]
            rax = (r12.wrapping_add(0x1)) as u32 as u64;
            // 332b7 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // 332ba jb 0x3325c
            if fl.b() { pc = 0x3325c; continue; }
            pc = 0x332bc;
        }
        0x332bc => {
            // block 332bc
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332bc mov eax,DWORD PTR [rbp-0x48]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 332bf inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 332c1 mov DWORD PTR [rbp-0x48],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), ((rax as u32 as u64)) as u32);
            // 332c4 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 332c7 cmp eax,DWORD PTR [rbp-0x44]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64).1;
            // 332ca jb 0x3323f
            if fl.b() { pc = 0x3323f; continue; }
            pc = 0x332d0;
        }
        0x332d0 => {
            // block 332d0
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332d0 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 332d4 mov DWORD PTR [rax],0xffffffff
            m.w32(rax, (0xffffffffu64) as u32);
            // 332da mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 332dc jmp 0x332e0
            pc = 0x332e0; continue;
        }
        0x332de => {
            // block 332de
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332de xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x332e0;
        }
        0x332e0 => {
            // block 332e0
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332e0 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 332e4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 332e5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 332e7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 332e9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 332eb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 332ed pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 332ee ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8168 size=304
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8168<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc8168;
    loop {
        match pc {
        0xc8168 => {
            // block c8168
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8168 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8168 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8169 mov rbp,rsp
            rbp = rsp;
            // c816c mov r11,r8
            r11 = r8;
            // c816f mov r10d,esi
            r10 = ((rsi as u32 as u64)) as u32 as u64;
            // c8172 add edi,0xffffffe6
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xffffffe6u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c8175 cmp edi,0x3
            fl = sub::<32>((rdi as u32 as u64), 0x3u64).1;
            // c8178 jbe 0xc8190
            if fl.be() { pc = 0xc8190; continue; }
            pc = 0xc817a;
        }
        0xc817a => {
            // block c817a
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c817a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c817a imul r10d,ecx
            { let (r, f) = imul::<32>((r10 as u32 as u64), (rcx as u32 as u64)); r10 = (r) as u32 as u64; fl = f; }
            // c817e add r11,r10
            { let (r, f) = add::<64>(r11, r10);
              r11 = r;
              fl = f; }
            // c8181 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // c8183 mov rdi,r11
            rdi = r11;
            // c8186 mov rsi,r9
            rsi = r9;
            // c8189 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc818e;
        }
        0xc818e => {
            // block c818e
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c818e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c818e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c818f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc8190 => {
            // block c8190
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8190 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8190 lea rax,[rip+0xf1] # 0xc8288
            rax = m.env.code_va(0xc8288);
            // c8197 movsxd rsi,DWORD PTR [rax+rdi*4]
            rsi = sx::<32>(m.r32(rax.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // c819b add rsi,rax
            { let (r, f) = add::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // c819e jmp rsi
            pc = (rsi.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc81a0 => {
            // block c81a0
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81a0 lea eax,[rdx+0x2]
            rax = (rdx.wrapping_add(0x2)) as u32 as u64;
            // c81a3 mov ecx,0xaaaaaaab
            rcx = (0xaaaaaaabu64) as u32 as u64;
            // c81a8 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // c81ac shr rcx,0x21
            { let (r, f) = shr::<64>(rcx, 0x21u64, fl); rcx = r; fl = f; }
            // c81b0 imul ecx,r10d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r10 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c81b4 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c81b7 add r11,rcx
            { let (r, f) = add::<64>(r11, rcx);
              r11 = r;
              fl = f; }
            // c81ba mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // c81c0 mov rdi,r9
            rdi = r9;
            // c81c3 mov rsi,r11
            rsi = r11;
            // c81c6 mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // c81c9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c81ca jmp 0xc75d9
            { let r = { let t = m.env.code_va(0xc75d9); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
        0xc81cf => {
            // block c81cf
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81cf test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c81d1 je 0xc818e
            if fl.e() { pc = 0xc818e; continue; }
            pc = 0xc81d3;
        }
        0xc81d3 => {
            // block c81d3
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81d3 mov r8d,r10d
            r8 = ((r10 as u32 as u64)) as u32 as u64;
            // c81d6 and r8d,0x1
            { let r = (r8 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c81da lea rax,[r9+0x2]
            rax = r9.wrapping_add(0x2);
            // c81de mov ecx,ecx
            rcx = ((rcx as u32 as u64)) as u32 as u64;
            // c81e0 mov esi,0xaaaaaaab
            rsi = (0xaaaaaaabu64) as u32 as u64;
            // c81e5 imul rsi,rcx
            { let (r, f) = imul::<64>(rsi, rcx); rsi = r; fl = f; }
            // c81e9 shr rsi,0x21
            { let (r, f) = shr::<64>(rsi, 0x21u64, fl); rsi = r; fl = f; }
            // c81ed imul esi,r10d
            { let (r, f) = imul::<32>((rsi as u32 as u64), (r10 as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            // c81f1 add r11,rsi
            { let (r, f) = add::<64>(r11, rsi);
              r11 = r;
              fl = f; }
            // c81f4 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // c81f6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xc81f8;
        }
        0xc81f8 => {
            // block c81f8
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81f8 test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // c81fb jne 0xc8206
            if fl.ne() { pc = 0xc8206; continue; }
            pc = 0xc81fd;
        }
        0xc81fd => {
            // block c81fd
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81fd lea rsi,[rax+0x2]
            rsi = rax.wrapping_add(0x2);
            // c8201 mov rdi,rax
            rdi = rax;
            // c8204 jmp 0xc820f
            pc = 0xc820f; continue;
        }
        0xc8206 => {
            // block c8206
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8206 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8206 lea rsi,[r9+rdx*8+0x2]
            rsi = r9.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x2);
            // c820b lea rdi,[r9+rdx*8]
            rdi = r9.wrapping_add(rdx.wrapping_mul(8));
            pc = 0xc820f;
        }
        0xc820f => {
            // block c820f
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c820f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c820f test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // c8212 cmove rdi,rsi
            if fl.e() { rdi = rsi; }
            // c8216 mov si,WORD PTR [rdi]
            rsi = (rsi & !0xffff) | ((m.r16(rdi) as u64) as u16 as u64);
            // c8219 mov WORD PTR [r11+rdx*2],si
            m.w16(r11.wrapping_add(rdx.wrapping_mul(2)), ((rsi as u16 as u64)) as u16);
            // c821e inc rdx
            { let (r, f) = inc::<64>(rdx, fl); rdx = r; fl = f; }
            // c8221 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c8225 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // c8227 jne 0xc81f8
            if fl.ne() { pc = 0xc81f8; continue; }
            pc = 0xc8229;
        }
        0xc8229 => {
            // block c8229
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8229 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8229 jmp 0xc818e
            pc = 0xc818e; continue;
        }
        0xc822e => {
            // block c822e
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c822e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c822e lea eax,[rdx+0x2]
            rax = (rdx.wrapping_add(0x2)) as u32 as u64;
            // c8231 mov ecx,0xaaaaaaab
            rcx = (0xaaaaaaabu64) as u32 as u64;
            // c8236 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // c823a shr rcx,0x21
            { let (r, f) = shr::<64>(rcx, 0x21u64, fl); rcx = r; fl = f; }
            // c823e imul ecx,r10d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r10 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c8242 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c8245 add r11,rcx
            { let (r, f) = add::<64>(r11, rcx);
              r11 = r;
              fl = f; }
            // c8248 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // c824e mov rdi,r9
            rdi = r9;
            // c8251 mov rsi,r11
            rsi = r11;
            // c8254 mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // c8257 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8258 jmp 0xc77be
            { let r = { let t = m.env.code_va(0xc77be); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
        0xc825d => {
            // block c825d
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c825d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c825d mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c825f mov ecx,0xaaaaaaab
            rcx = (0xaaaaaaabu64) as u32 as u64;
            // c8264 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // c8268 shr rcx,0x21
            { let (r, f) = shr::<64>(rcx, 0x21u64, fl); rcx = r; fl = f; }
            // c826c imul ecx,r10d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r10 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c8270 add r11,rcx
            { let (r, f) = add::<64>(r11, rcx);
              r11 = r;
              fl = f; }
            // c8273 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // c8279 mov rdi,r9
            rdi = r9;
            // c827c mov rsi,r11
            rsi = r11;
            // c827f mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // c8282 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8283 jmp 0xc7978
            { let r = { let t = m.env.code_va(0xc7978); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4982 size=300
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004982<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4982;
    loop {
        match pc {
        0x4982 => {
            // block 4982
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4982 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4982 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4983 mov rbp,rsp
            rbp = rsp;
            // 4986 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4988 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 498a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 498c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 498e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 498f sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 4993 mov r12,rcx
            r12 = rcx;
            // 4996 mov r15,rdx
            r15 = rdx;
            // 4999 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 499b mov r14,rdi
            r14 = rdi;
            // 499e mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 49a5 mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // 49ac mov DWORD PTR [rbp-0x34],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), (0x0u64) as u32);
            // 49b3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 49b6 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49bc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 49be je 0x49c8
            if fl.e() { pc = 0x49c8; continue; }
            pc = 0x49c0;
        }
        0x49c0 => {
            // block 49c0
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 49c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49c0 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 49c3 jmp 0x4a9b
            pc = 0x4a9b; continue;
        }
        0x49c8 => {
            // block 49c8
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 49c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49c8 mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 49cc mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 49cf mov ebx,ebx
            rbx = ((rbx as u32 as u64)) as u32 as u64;
            // 49d1 lea rcx,[rip+0xfa798] # 0xff170
            rcx = m.env.data_va(80189);
            // 49d8 mov r13d,DWORD PTR [rcx+rbx*4]
            r13 = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 49dc lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 49e0 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 49e3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 49e8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 49eb mov rdi,r14
            rdi = r14;
            // 49ee mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 49f1 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49f4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 49f6 je 0x4a9b
            if fl.e() { pc = 0x4a9b; continue; }
            pc = 0x49fc;
        }
        0x49fc => {
            // block 49fc
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 49fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49fc lea rax,[rip+0xfa79d] # 0xff1a0
            rax = m.env.data_va(80237);
            // 4a03 mov eax,DWORD PTR [rax+rbx*4]
            rax = (m.r32(rax.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4a06 test DWORD PTR [rbp-0x2c],eax
            fl = logic::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & (rax as u32 as u64));
            // 4a09 je 0x4a31
            if fl.e() { pc = 0x4a31; continue; }
            pc = 0x4a0b;
        }
        0x4a0b => {
            // block 4a0b
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a0b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4a0e lea rcx,[rip+0xfa7bb] # 0xff1d0
            rcx = m.env.data_va(80285);
            // 4a15 mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4a18 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 4a1c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 4a1f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4a24 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4a27 mov rdi,r14
            rdi = r14;
            // 4a2a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a2d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4a2f je 0x4a9b
            if fl.e() { pc = 0x4a9b; continue; }
            pc = 0x4a31;
        }
        0x4a31 => {
            // block 4a31
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a31 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4a34 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 4a38 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 4a3b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4a40 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4a43 mov rdi,r14
            rdi = r14;
            // 4a46 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4a49 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a4c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4a4e je 0x4a9b
            if fl.e() { pc = 0x4a9b; continue; }
            pc = 0x4a50;
        }
        0x4a50 => {
            // block 4a50
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a50 lea rax,[rip+0xfa7a9] # 0xff200
            rax = m.env.data_va(80333);
            // 4a57 mov eax,DWORD PTR [rax+rbx*4]
            rax = (m.r32(rax.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4a5a test DWORD PTR [rbp-0x2c],eax
            fl = logic::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & (rax as u32 as u64));
            // 4a5d je 0x4a85
            if fl.e() { pc = 0x4a85; continue; }
            pc = 0x4a5f;
        }
        0x4a5f => {
            // block 4a5f
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a5f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4a62 lea rcx,[rip+0xfa7c7] # 0xff230
            rcx = m.env.data_va(80381);
            // 4a69 mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4a6c lea rdx,[rbp-0x34]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 4a70 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 4a73 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4a78 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4a7b mov rdi,r14
            rdi = r14;
            // 4a7e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a81 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4a83 je 0x4a9b
            if fl.e() { pc = 0x4a9b; continue; }
            pc = 0x4a85;
        }
        0x4a85 => {
            // block 4a85
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a85 mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 4a88 bswap eax
            rax = (rax as u32).swap_bytes() as u64;
            // 4a8a mov DWORD PTR [r15],eax
            m.w32(r15, ((rax as u32 as u64)) as u32);
            // 4a8d mov eax,DWORD PTR [rbp-0x34]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // 4a90 bswap eax
            rax = (rax as u32).swap_bytes() as u64;
            // 4a92 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 4a96 mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            // 4a98 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x4a9b;
        }
        0x4a9b => {
            // block 4a9b
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a9b mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 4a9e add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 4aa2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4aa3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4aa5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4aa7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4aa9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4aab pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4aac ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x29800 size=296
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00029800<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x29800;
    loop {
        match pc {
        0x29800 => {
            // block 29800
            #[cfg(feature = "trace")] eprintln!("f_00029800 pc 29800 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29800 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 29801 mov rbp,rsp
            rbp = rsp;
            // 29804 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 29806 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 29807 mov r14,rsi
            r14 = rsi;
            // 2980a mov rbx,rdi
            rbx = rdi;
            // 2980d cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 29810 je 0x2991f
            if fl.e() { pc = 0x2991f; continue; }
            pc = 0x29816;
        }
        0x29816 => {
            // block 29816
            #[cfg(feature = "trace")] eprintln!("f_00029800 pc 29816 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29816 lea rdi,[rbx+0x50]
            rdi = rbx.wrapping_add(0x50);
            // 2981a lea rsi,[r14+0x50]
            rsi = r14.wrapping_add(0x50);
            // 2981e call 0x28e5e
            { let r = { let t = m.env.code_va(0x28e5e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29823 mov rax,QWORD PTR [r14+0x18]
            rax = m.r64(r14.wrapping_add(0x18)) as u64;
            // 29827 mov QWORD PTR [rbx+0x18],rax
            m.w64(rbx.wrapping_add(0x18), (rax) as u64);
            // 2982b mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // 2982f mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 29833 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 29836 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 2983a mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 2983e mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 29841 mov rax,QWORD PTR [r14+0x20]
            rax = m.r64(r14.wrapping_add(0x20)) as u64;
            // 29845 mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // 29849 mov eax,DWORD PTR [r14+0x28]
            rax = (m.r32(r14.wrapping_add(0x28)) as u64) as u32 as u64;
            // 2984d mov DWORD PTR [rbx+0x28],eax
            m.w32(rbx.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 29850 mov rax,QWORD PTR [r14+0x2c]
            rax = m.r64(r14.wrapping_add(0x2c)) as u64;
            // 29854 mov QWORD PTR [rbx+0x2c],rax
            m.w64(rbx.wrapping_add(0x2c), (rax) as u64);
            // 29858 movups xmm0,XMMWORD PTR [r14+0x34]
            x0 = m.r128(r14.wrapping_add(0x34));
            // 2985d movups XMMWORD PTR [rbx+0x34],xmm0
            m.w128(rbx.wrapping_add(0x34), x0);
            // 29861 mov eax,DWORD PTR [r14+0x44]
            rax = (m.r32(r14.wrapping_add(0x44)) as u64) as u32 as u64;
            // 29865 mov DWORD PTR [rbx+0x44],eax
            m.w32(rbx.wrapping_add(0x44), ((rax as u32 as u64)) as u32);
            // 29868 mov eax,DWORD PTR [r14+0x48]
            rax = (m.r32(r14.wrapping_add(0x48)) as u64) as u32 as u64;
            // 2986c mov DWORD PTR [rbx+0x48],eax
            m.w32(rbx.wrapping_add(0x48), ((rax as u32 as u64)) as u32);
            // 2986f mov eax,DWORD PTR [r14+0x4c]
            rax = (m.r32(r14.wrapping_add(0x4c)) as u64) as u32 as u64;
            // 29873 mov DWORD PTR [rbx+0x4c],eax
            m.w32(rbx.wrapping_add(0x4c), ((rax as u32 as u64)) as u32);
            // 29876 mov rax,QWORD PTR [r14+0x78]
            rax = m.r64(r14.wrapping_add(0x78)) as u64;
            // 2987a mov QWORD PTR [rbx+0x78],rax
            m.w64(rbx.wrapping_add(0x78), (rax) as u64);
            // 2987e mov eax,DWORD PTR [r14+0x80]
            rax = (m.r32(r14.wrapping_add(0x80)) as u64) as u32 as u64;
            // 29885 mov DWORD PTR [rbx+0x80],eax
            m.w32(rbx.wrapping_add(0x80), ((rax as u32 as u64)) as u32);
            // 2988b movups xmm0,XMMWORD PTR [r14+0x84]
            x0 = m.r128(r14.wrapping_add(0x84));
            // 29893 movups XMMWORD PTR [rbx+0x84],xmm0
            m.w128(rbx.wrapping_add(0x84), x0);
            // 2989a mov eax,DWORD PTR [r14+0x94]
            rax = (m.r32(r14.wrapping_add(0x94)) as u64) as u32 as u64;
            // 298a1 mov DWORD PTR [rbx+0x94],eax
            m.w32(rbx.wrapping_add(0x94), ((rax as u32 as u64)) as u32);
            // 298a7 mov eax,DWORD PTR [r14+0x98]
            rax = (m.r32(r14.wrapping_add(0x98)) as u64) as u32 as u64;
            // 298ae mov DWORD PTR [rbx+0x98],eax
            m.w32(rbx.wrapping_add(0x98), ((rax as u32 as u64)) as u32);
            // 298b4 mov eax,DWORD PTR [r14+0x9c]
            rax = (m.r32(r14.wrapping_add(0x9c)) as u64) as u32 as u64;
            // 298bb mov DWORD PTR [rbx+0x9c],eax
            m.w32(rbx.wrapping_add(0x9c), ((rax as u32 as u64)) as u32);
            // 298c1 mov eax,DWORD PTR [r14+0xa0]
            rax = (m.r32(r14.wrapping_add(0xa0)) as u64) as u32 as u64;
            // 298c8 mov DWORD PTR [rbx+0xa0],eax
            m.w32(rbx.wrapping_add(0xa0), ((rax as u32 as u64)) as u32);
            // 298ce mov eax,DWORD PTR [r14+0xa4]
            rax = (m.r32(r14.wrapping_add(0xa4)) as u64) as u32 as u64;
            // 298d5 mov DWORD PTR [rbx+0xa4],eax
            m.w32(rbx.wrapping_add(0xa4), ((rax as u32 as u64)) as u32);
            // 298db mov rax,QWORD PTR [r14+0xa8]
            rax = m.r64(r14.wrapping_add(0xa8)) as u64;
            // 298e2 mov QWORD PTR [rbx+0xa8],rax
            m.w64(rbx.wrapping_add(0xa8), (rax) as u64);
            // 298e9 mov eax,DWORD PTR [r14+0xb0]
            rax = (m.r32(r14.wrapping_add(0xb0)) as u64) as u32 as u64;
            // 298f0 mov DWORD PTR [rbx+0xb0],eax
            m.w32(rbx.wrapping_add(0xb0), ((rax as u32 as u64)) as u32);
            // 298f6 mov eax,DWORD PTR [r14+0xbc]
            rax = (m.r32(r14.wrapping_add(0xbc)) as u64) as u32 as u64;
            // 298fd mov DWORD PTR [rbx+0xbc],eax
            m.w32(rbx.wrapping_add(0xbc), ((rax as u32 as u64)) as u32);
            // 29903 mov rax,QWORD PTR [r14+0xb4]
            rax = m.r64(r14.wrapping_add(0xb4)) as u64;
            // 2990a mov QWORD PTR [rbx+0xb4],rax
            m.w64(rbx.wrapping_add(0xb4), (rax) as u64);
            // 29911 mov rax,QWORD PTR [r14+0xc0]
            rax = m.r64(r14.wrapping_add(0xc0)) as u64;
            // 29918 mov QWORD PTR [rbx+0xc0],rax
            m.w64(rbx.wrapping_add(0xc0), (rax) as u64);
            pc = 0x2991f;
        }
        0x2991f => {
            // block 2991f
            #[cfg(feature = "trace")] eprintln!("f_00029800 pc 2991f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2991f mov rax,rbx
            rax = rbx;
            // 29922 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 29923 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 29925 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29926 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
