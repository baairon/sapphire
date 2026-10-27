//! Mechanically lifted functions (scripts/lift_run.py), module h003. Each one is checked against the original
//! in the world harness (tests/world_h003.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x12c80 size=520
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00012c80<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x12c80;
    loop {
        match pc {
        0x12c80 => {
            // block 12c80
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12c80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12c80 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12c81 mov rbp,rsp
            rbp = rsp;
            // 12c84 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12c86 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12c88 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 12c8a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12c8c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 12c8d sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 12c94 mov r15,rsi
            r15 = rsi;
            // 12c97 mov rbx,rdi
            rbx = rdi;
            // 12c9a mov r12,QWORD PTR [rip+0x12d737] # 0x1403d8
            r12 = m.r64(m.env.data_va(347045)) as u64;
            // 12ca1 mov r12,QWORD PTR [r12]
            r12 = m.r64(r12) as u64;
            // 12ca5 mov QWORD PTR [rbp-0x30],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r12) as u64);
            // 12ca9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 12cac call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12caf test BYTE PTR [r15],0x1
            fl = logic::<8>(m.r8(r15) as u64 & 0x1u64);
            // 12cb3 jne 0x12cbb
            if fl.ne() { pc = 0x12cbb; continue; }
            pc = 0x12cb5;
        }
        0x12cb5 => {
            // block 12cb5
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12cb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12cb5 lea rdi,[r15+0x1]
            rdi = r15.wrapping_add(0x1);
            // 12cb9 jmp 0x12cbf
            pc = 0x12cbf; continue;
        }
        0x12cbb => {
            // block 12cbb
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12cbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12cbb mov rdi,QWORD PTR [r15+0x10]
            rdi = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x12cbf;
        }
        0x12cbf => {
            // block 12cbf
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12cbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12cbf lea rsi,[rbp-0x148]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 12cc6 call 0xec2a2
            { let r = { let t = m.env.stub_va(0xa6f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12ccb mov eax,DWORD PTR [rbp-0xe8]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff18)) as u64) as u32 as u64;
            // 12cd1 mov DWORD PTR [rbx+0x2e4],eax
            m.w32(rbx.wrapping_add(0x2e4), ((rax as u32 as u64)) as u32);
            // 12cd7 lea r14,[rbx+0x8]
            r14 = rbx.wrapping_add(0x8);
            // 12cdb test BYTE PTR [r15],0x1
            fl = logic::<8>(m.r8(r15) as u64 & 0x1u64);
            // 12cdf jne 0x12ce6
            if fl.ne() { pc = 0x12ce6; continue; }
            pc = 0x12ce1;
        }
        0x12ce1 => {
            // block 12ce1
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12ce1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12ce1 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 12ce4 jmp 0x12cea
            pc = 0x12cea; continue;
        }
        0x12ce6 => {
            // block 12ce6
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12ce6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12ce6 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x12cea;
        }
        0x12cea => {
            // block 12cea
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12cea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12cea mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 12cef mov rdi,r14
            rdi = r14;
            // 12cf2 mov rsi,r15
            rsi = r15;
            // 12cf5 call 0xeb918
            { let r = { let t = m.env.stub_va(0xe5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12cfa mov DWORD PTR [rbp-0x14c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feb4), (0x0u64) as u32);
            // 12d04 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 12d07 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 12d0b test BYTE PTR [rax+r14*1+0x20],0x5
            fl = logic::<8>(m.r8(rax.wrapping_add(r14).wrapping_add(0x20)) as u64 & 0x5u64);
            // 12d11 je 0x12d2b
            if fl.e() { pc = 0x12d2b; continue; }
            pc = 0x12d13;
        }
        0x12d13 => {
            // block 12d13
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d13 lea rdi,[rbx+0x2c8]
            rdi = rbx.wrapping_add(0x2c8);
            // 12d1a lea rsi,[rip+0x1172d3] # 0x129ff4
            rsi = m.env.data_va(255937);
            // 12d21 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12d26 jmp 0x12e5c
            pc = 0x12e5c; continue;
        }
        0x12d2b => {
            // block 12d2b
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d2b lea r15,[rbx+0x248]
            r15 = rbx.wrapping_add(0x248);
            // 12d32 mov r13d,0xb7
            r13 = (0xb7u64) as u32 as u64;
            // 12d38 lea r12,[rbp-0x14d]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_feb3);
            pc = 0x12d3f;
        }
        0x12d3f => {
            // block 12d3f
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d3f mov rdi,r14
            rdi = r14;
            // 12d42 call 0xec03e
            { let r = { let t = m.env.stub_va(0x80b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12d47 mov BYTE PTR [rbp-0x14d],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_feb3), ((rax as u8 as u64)) as u8);
            // 12d4d mov rcx,QWORD PTR [rbx+0x250]
            rcx = m.r64(rbx.wrapping_add(0x250)) as u64;
            // 12d54 cmp rcx,QWORD PTR [rbx+0x258]
            fl = sub::<64>(rcx, m.r64(rbx.wrapping_add(0x258)) as u64).1;
            // 12d5b jae 0x12d68
            if fl.ae() { pc = 0x12d68; continue; }
            pc = 0x12d5d;
        }
        0x12d5d => {
            // block 12d5d
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d5d mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // 12d5f inc QWORD PTR [rbx+0x250]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x250)) as u64, fl); m.w64(rbx.wrapping_add(0x250), (r) as u64); fl = f; }
            // 12d66 jmp 0x12d73
            pc = 0x12d73; continue;
        }
        0x12d68 => {
            // block 12d68
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d68 mov rdi,r15
            rdi = r15;
            // 12d6b mov rsi,r12
            rsi = r12;
            // 12d6e call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x12d73;
        }
        0x12d73 => {
            // block 12d73
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d73 dec r13d
            { let (r, f) = dec::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // 12d76 jne 0x12d3f
            if fl.ne() { pc = 0x12d3f; continue; }
            pc = 0x12d78;
        }
        0x12d78 => {
            // block 12d78
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d78 lea r15,[rbx+0x2c8]
            r15 = rbx.wrapping_add(0x2c8);
            // 12d7f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 12d82 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 12d89 lea rdx,[rbp-0x14c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_feb4);
            // 12d90 mov rsi,rbx
            rsi = rbx;
            // 12d93 call QWORD PTR [rax+0x90]
            { let r = { let t = m.r64(rax.wrapping_add(0x90)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12d99 test BYTE PTR [rbx+0x2c8],0x1
            fl = logic::<8>(m.r8(rbx.wrapping_add(0x2c8)) as u64 & 0x1u64);
            // 12da0 mov r12,QWORD PTR [rip+0x12d631] # 0x1403d8
            r12 = m.r64(m.env.data_va(347045)) as u64;
            // 12da7 mov r12,QWORD PTR [r12]
            r12 = m.r64(r12) as u64;
            // 12dab jne 0x12db5
            if fl.ne() { pc = 0x12db5; continue; }
            pc = 0x12dad;
        }
        0x12dad => {
            // block 12dad
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12dad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12dad mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // 12db3 jmp 0x12dca
            pc = 0x12dca; continue;
        }
        0x12db5 => {
            // block 12db5
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12db5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12db5 mov rax,QWORD PTR [rbx+0x2d8]
            rax = m.r64(rbx.wrapping_add(0x2d8)) as u64;
            // 12dbc mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 12dbf mov QWORD PTR [rbx+0x2d0],0x0
            m.w64(rbx.wrapping_add(0x2d0), (0x0u64) as u64);
            pc = 0x12dca;
        }
        0x12dca => {
            // block 12dca
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12dca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12dca xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 12dcc mov rdi,r15
            rdi = r15;
            // 12dcf call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12dd4 mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 12ddb mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 12ddf mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 12de6 mov rcx,QWORD PTR [rbp-0x168]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 12ded mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 12df1 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 12df4 xorps xmm0,xmm0
            x0 = 0;
            // 12df7 movaps XMMWORD PTR [rbp-0x170],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe90), x0);
            // 12dfe mov QWORD PTR [rbp-0x160],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (0x0u64) as u64);
            // 12e09 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 12e10 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12e15 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 12e19 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 12e1b jne 0x12e22
            if fl.ne() { pc = 0x12e22; continue; }
            pc = 0x12e1d;
        }
        0x12e1d => {
            // block 12e1d
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e1d shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 12e20 jmp 0x12e29
            pc = 0x12e29; continue;
        }
        0x12e22 => {
            // block 12e22
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e22 mov rax,QWORD PTR [rbx+0x2d0]
            rax = m.r64(rbx.wrapping_add(0x2d0)) as u64;
            pc = 0x12e29;
        }
        0x12e29 => {
            // block 12e29
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e29 test rax,rax
            fl = logic::<64>(rax & rax);
            // 12e2c jne 0x12e5c
            if fl.ne() { pc = 0x12e5c; continue; }
            pc = 0x12e2e;
        }
        0x12e2e => {
            // block 12e2e
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e2e cmp BYTE PTR [rbx+0x2e9],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x2e9)) as u64, 0x0u64).1;
            // 12e35 jne 0x12e55
            if fl.ne() { pc = 0x12e55; continue; }
            pc = 0x12e37;
        }
        0x12e37 => {
            // block 12e37
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e37 lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 12e3e mov rsi,r14
            rsi = r14;
            // 12e41 call 0xec04a
            { let r = { let t = m.env.stub_va(0x817); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12e46 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // 12e49 sub eax,DWORD PTR [rbp-0x14c]
            { let (r, f) = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_feb4)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 12e4f mov DWORD PTR [rbx+0x260],eax
            m.w32(rbx.wrapping_add(0x260), ((rax as u32 as u64)) as u32);
            pc = 0x12e55;
        }
        0x12e55 => {
            // block 12e55
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e55 mov BYTE PTR [rbx+0x2e8],0x1
            m.w8(rbx.wrapping_add(0x2e8), (0x1u64) as u8);
            pc = 0x12e5c;
        }
        0x12e5c => {
            // block 12e5c
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e5c mov al,BYTE PTR [rbx+0x2e8]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0x2e8)) as u64) as u8 as u64);
            // 12e62 cmp r12,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 12e66 jne 0x12e7a
            if fl.ne() { pc = 0x12e7a; continue; }
            pc = 0x12e68;
        }
        0x12e68 => {
            // block 12e68
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e68 add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 12e6f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 12e70 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 12e72 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 12e74 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 12e76 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 12e78 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 12e79 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x12e7a => {
            // block 12e7a
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e7a call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12e7f mov rdi,rax
            rdi = rax;
            // 12e82 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x32792 size=506
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00032792<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x32792;
    loop {
        match pc {
        0x32792 => {
            // block 32792
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32792 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32792 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 32793 mov rbp,rsp
            rbp = rsp;
            // 32796 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 32798 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3279a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 3279c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3279d mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // 327a0 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 327a3 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // 327a6 mov rbx,rdi
            rbx = rdi;
            // 327a9 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 327b1 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 327b8 mov DWORD PTR [rbx+0x10],0xc
            m.w32(rbx.wrapping_add(0x10), (0xcu64) as u32);
            // 327bf mov DWORD PTR [rbx+0x14],0x0
            m.w32(rbx.wrapping_add(0x14), (0x0u64) as u32);
            // 327c6 mov DWORD PTR [rbx+0x18],0x20
            m.w32(rbx.wrapping_add(0x18), (0x20u64) as u32);
            // 327cd mov DWORD PTR [rbx+0x1c],0x3
            m.w32(rbx.wrapping_add(0x1c), (0x3u64) as u32);
            // 327d4 mov BYTE PTR [rbx+0x20],0x0
            m.w8(rbx.wrapping_add(0x20), (0x0u64) as u8);
            // 327d8 mov WORD PTR [rbx+0x34],0x0
            m.w16(rbx.wrapping_add(0x34), (0x0u64) as u16);
            // 327de mov QWORD PTR [rbx+0x2c],0x0
            m.w64(rbx.wrapping_add(0x2c), (0x0u64) as u64);
            // 327e6 mov QWORD PTR [rbx+0x24],0x0
            m.w64(rbx.wrapping_add(0x24), (0x0u64) as u64);
            // 327ee mov DWORD PTR [rbx+0x38],0x12
            m.w32(rbx.wrapping_add(0x38), (0x12u64) as u32);
            // 327f5 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 327f8 call 0x5a914
            { let r = { let t = m.env.code_va(0x5a914); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 327fd cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 32801 ja 0x32960
            if fl.a() { pc = 0x32960; continue; }
            pc = 0x32807;
        }
        0x32807 => {
            // block 32807
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32807 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32807 cmp r14d,0x1f
            fl = sub::<32>((r14 as u32 as u64), 0x1fu64).1;
            // 3280b ja 0x32960
            if fl.a() { pc = 0x32960; continue; }
            pc = 0x32811;
        }
        0x32811 => {
            // block 32811
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32811 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32811 cmp eax,0xb
            fl = sub::<32>((rax as u32 as u64), 0xbu64).1;
            // 32814 ja 0x32960
            if fl.a() { pc = 0x32960; continue; }
            pc = 0x3281a;
        }
        0x3281a => {
            // block 3281a
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3281a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3281a cmp r14d,0x17
            fl = sub::<32>((r14 as u32 as u64), 0x17u64).1;
            // 3281e sete dl
            rdx = (rdx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 32821 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 32824 and ecx,0xffffffef
            { let r = (rcx as u32 as u64) & 0xffffffefu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 32827 cmp ecx,0xa
            fl = sub::<32>((rcx as u32 as u64), 0xau64).1;
            // 3282a sete dil
            rdi = (rdi & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 3282e lea ecx,[r14-0x1b]
            rcx = (r14.wrapping_add(0xffff_ffff_ffff_ffe5)) as u32 as u64;
            // 32832 cmp ecx,0x5
            fl = sub::<32>((rcx as u32 as u64), 0x5u64).1;
            // 32835 setb cl
            rcx = (rcx & !0xff) | (((fl.b() as u64)) as u8 as u64);
            // 32838 lea esi,[r15-0x1]
            rsi = (r15.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 3283c cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 3283f ja 0x3284c
            if fl.a() { pc = 0x3284c; continue; }
            pc = 0x32841;
        }
        0x32841 => {
            // block 32841
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32841 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32841 or dl,dil
            { let r = (rdx as u8 as u64) | (rdi as u8 as u64); let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 32844 or cl,dl
            { let r = (rcx as u8 as u64) | (rdx as u8 as u64); let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 32846 jne 0x32960
            if fl.ne() { pc = 0x32960; continue; }
            pc = 0x3284c;
        }
        0x3284c => {
            // block 3284c
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3284c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3284c mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 3284f mov r8d,eax
            r8 = ((rax as u32 as u64)) as u32 as u64;
            // 32852 imul rdi,r8,0x780
            { let (r, f) = imul::<64>(r8, 0x780u64); rdi = r; fl = f; }
            // 32859 lea rdx,[rip+0x18fe10] # 0x1c2670
            rdx = m.env.data_va(880189);
            // 32860 add rdx,rdi
            { let (r, f) = add::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // 32863 imul rcx,rcx,0x3c
            { let (r, f) = imul::<64>(rcx, 0x3cu64); rcx = r; fl = f; }
            // 32867 mov edi,DWORD PTR [rcx+rdx*1+0x38]
            rdi = (m.r32(rcx.wrapping_add(rdx).wrapping_add(0x38)) as u64) as u32 as u64;
            // 3286b mov DWORD PTR [rbx+0x38],edi
            m.w32(rbx.wrapping_add(0x38), ((rdi as u32 as u64)) as u32);
            // 3286e mov rdi,QWORD PTR [rcx+rdx*1+0x30]
            rdi = m.r64(rcx.wrapping_add(rdx).wrapping_add(0x30)) as u64;
            // 32873 mov QWORD PTR [rbx+0x30],rdi
            m.w64(rbx.wrapping_add(0x30), (rdi) as u64);
            // 32877 mov rdi,QWORD PTR [rcx+rdx*1+0x28]
            rdi = m.r64(rcx.wrapping_add(rdx).wrapping_add(0x28)) as u64;
            // 3287c mov QWORD PTR [rbx+0x28],rdi
            m.w64(rbx.wrapping_add(0x28), (rdi) as u64);
            // 32880 mov rdi,QWORD PTR [rcx+rdx*1+0x20]
            rdi = m.r64(rcx.wrapping_add(rdx).wrapping_add(0x20)) as u64;
            // 32885 mov QWORD PTR [rbx+0x20],rdi
            m.w64(rbx.wrapping_add(0x20), (rdi) as u64);
            // 32889 mov rdi,QWORD PTR [rcx+rdx*1+0x18]
            rdi = m.r64(rcx.wrapping_add(rdx).wrapping_add(0x18)) as u64;
            // 3288e mov QWORD PTR [rbx+0x18],rdi
            m.w64(rbx.wrapping_add(0x18), (rdi) as u64);
            // 32892 mov rdi,QWORD PTR [rcx+rdx*1+0x10]
            rdi = m.r64(rcx.wrapping_add(rdx).wrapping_add(0x10)) as u64;
            // 32897 mov QWORD PTR [rbx+0x10],rdi
            m.w64(rbx.wrapping_add(0x10), (rdi) as u64);
            // 3289b mov rdi,QWORD PTR [rcx+rdx*1]
            rdi = m.r64(rcx.wrapping_add(rdx)) as u64;
            // 3289f mov rcx,QWORD PTR [rcx+rdx*1+0x8]
            rcx = m.r64(rcx.wrapping_add(rdx).wrapping_add(0x8)) as u64;
            // 328a4 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 328a8 mov QWORD PTR [rbx],rdi
            m.w64(rbx, (rdi) as u64);
            // 328ab mov DWORD PTR [rbx+0x14],r12d
            m.w32(rbx.wrapping_add(0x14), ((r12 as u32 as u64)) as u32);
            // 328af mov DWORD PTR [rbx+0x10],eax
            m.w32(rbx.wrapping_add(0x10), ((rax as u32 as u64)) as u32);
            // 328b2 mov DWORD PTR [rbx+0x18],r14d
            m.w32(rbx.wrapping_add(0x18), ((r14 as u32 as u64)) as u32);
            // 328b6 mov DWORD PTR [rbx+0x1c],r15d
            m.w32(rbx.wrapping_add(0x1c), ((r15 as u32 as u64)) as u32);
            // 328ba mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 328bc and ecx,0xfffffffe
            { let r = (rcx as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 328bf cmp ecx,0x6
            fl = sub::<32>((rcx as u32 as u64), 0x6u64).1;
            // 328c2 sete BYTE PTR [rbx+0x20]
            m.w8(rbx.wrapping_add(0x20), ((fl.e() as u64)) as u8);
            // 328c6 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 328c9 ja 0x3293f
            if fl.a() { pc = 0x3293f; continue; }
            pc = 0x328cb;
        }
        0x328cb => {
            // block 328cb
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 328cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 328cb mov ecx,DWORD PTR [rbx]
            rcx = (m.r32(rbx) as u64) as u32 as u64;
            // 328cd cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // 328d0 jbe 0x328d8
            if fl.be() { pc = 0x328d8; continue; }
            pc = 0x328d2;
        }
        0x328d2 => {
            // block 328d2
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 328d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 328d2 mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            // 328d4 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 328d6 jmp 0x3293a
            pc = 0x3293a; continue;
        }
        0x328d8 => {
            // block 328d8
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 328d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 328d8 lea rax,[rip+0x8d] # 0x3296c
            rax = m.env.code_va(0x3296c);
            // 328df movsxd rdx,DWORD PTR [rax+r8*4]
            rdx = sx::<32>(m.r32(rax.wrapping_add(r8.wrapping_mul(4))) as u64) as u64;
            // 328e3 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 328e6 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x328e8 => {
            // block 328e8
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 328e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 328e8 cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 328ec sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 328ef movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 328f2 lea eax,[rax+rax*1+0x458]
            rax = (rax.wrapping_add(rax).wrapping_add(0x458)) as u32 as u64;
            // 328f9 mov DWORD PTR [rbx],eax
            m.w32(rbx, ((rax as u32 as u64)) as u32);
            // 328fb jmp 0x3293a
            pc = 0x3293a; continue;
        }
        0x328fd => {
            // block 328fd
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 328fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 328fd mov DWORD PTR [rbx],0x2e4
            m.w32(rbx, (0x2e4u64) as u32);
            // 32903 mov eax,0x2e4
            rax = (0x2e4u64) as u32 as u64;
            // 32908 jmp 0x3293a
            pc = 0x3293a; continue;
        }
        0x3290a => {
            // block 3290a
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3290a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3290a cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 3290e mov edx,0x202
            rdx = (0x202u64) as u32 as u64;
            // 32913 mov eax,0x1fc
            rax = (0x1fcu64) as u32 as u64;
            // 32918 jmp 0x32928
            pc = 0x32928; continue;
        }
        0x3291a => {
            // block 3291a
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3291a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3291a cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 3291e mov edx,0x264
            rdx = (0x264u64) as u32 as u64;
            // 32923 mov eax,0x256
            rax = (0x256u64) as u32 as u64;
            pc = 0x32928;
        }
        0x32928 => {
            // block 32928
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32928 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32928 cmove eax,edx
            if fl.e() { rax = ((rdx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 3292b mov DWORD PTR [rbx],eax
            m.w32(rbx, ((rax as u32 as u64)) as u32);
            // 3292d jmp 0x3293a
            pc = 0x3293a; continue;
        }
        0x3292f => {
            // block 3292f
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3292f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3292f mov DWORD PTR [rbx],0x634
            m.w32(rbx, (0x634u64) as u32);
            // 32935 mov eax,0x634
            rax = (0x634u64) as u32 as u64;
            pc = 0x3293a;
        }
        0x3293a => {
            // block 3293a
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3293a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3293a sub eax,ecx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 3293c mov DWORD PTR [rbx+0xc],eax
            m.w32(rbx.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            pc = 0x3293f;
        }
        0x3293f => {
            // block 3293f
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3293f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3293f cmp r14d,0x1f
            fl = sub::<32>((r14 as u32 as u64), 0x1fu64).1;
            // 32943 ja 0x32960
            if fl.a() { pc = 0x32960; continue; }
            pc = 0x32945;
        }
        0x32945 => {
            // block 32945
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32945 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32945 mov eax,0xfc800400
            rax = (0xfc800400u64) as u32 as u64;
            // 3294a bt eax,r14d
            fl.cf = bit::<32>((rax as u32 as u64), (r14 as u32 as u64));
            // 3294e jae 0x32960
            if fl.ae() { pc = 0x32960; continue; }
            pc = 0x32950;
        }
        0x32950 => {
            // block 32950
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32950 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32950 mov rdi,rbx
            rdi = rbx;
            // 32953 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32954 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 32956 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 32958 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 3295a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3295b jmp 0x325ca
            { let r = { let t = m.env.code_va(0x325ca); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x32960 => {
            // block 32960
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32960 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32960 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32961 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 32963 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 32965 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 32967 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32968 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11344 size=502
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011344<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x11344;
    loop {
        match pc {
        0x11344 => {
            // block 11344
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11344 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11344 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11345 mov rbp,rsp
            rbp = rsp;
            // 11348 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1134a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1134c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1134e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 11350 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11351 sub rsp,0x78
            { let (r, f) = sub::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // 11355 mov rbx,rdi
            rbx = rdi;
            // 11358 mov rdi,QWORD PTR [rbx+0x40]
            rdi = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 1135c xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 1135f test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 11362 je 0x114ff
            if fl.e() { pc = 0x114ff; continue; }
            pc = 0x11368;
        }
        0x11368 => {
            // block 11368
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11368 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11368 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 11370 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 11378 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 1137b lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 1137f call QWORD PTR [rax+0x20]
            { let r = { let t = m.r64(rax.wrapping_add(0x20)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 11382 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 11384 jne 0x114ff
            if fl.ne() { pc = 0x114ff; continue; }
            pc = 0x1138a;
        }
        0x1138a => {
            // block 1138a
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 1138a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1138a mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 1138e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 11391 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 11395 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 11398 mov r14d,0xffffffff
            r14 = (0xffffffffu64) as u32 as u64;
            // 1139e test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 113a0 jne 0x114e8
            if fl.ne() { pc = 0x114e8; continue; }
            pc = 0x113a6;
        }
        0x113a6 => {
            // block 113a6
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 113a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 113a6 lea rax,[rbx+0x8]
            rax = rbx.wrapping_add(0x8);
            // 113aa mov QWORD PTR [rbp-0xa0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (rax) as u64);
            // 113b1 lea r14,[rbp-0x90]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            pc = 0x113b8;
        }
        0x113b8 => {
            // block 113b8
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 113b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 113b8 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 113bc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 113bf lea rsi,[rbp-0x48]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 113c3 lea rdx,[rbp-0x50]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 113c7 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 113ca cvtsi2sd xmm1,QWORD PTR [rbp-0x50]
            x1 = set_lo64(x1, (sx::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64) as f64).to_bits());
            // 113d0 cvtsi2sd xmm0,QWORD PTR [rbp-0x48]
            x0 = set_lo64(x0, (sx::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as f64).to_bits());
            // 113d6 divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 113da movsd QWORD PTR [rbp-0x98],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), x1 as u64);
            // 113e2 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 113e6 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 113e9 lea rsi,[rbp-0x58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 113ed call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 113f0 mov rdi,QWORD PTR [rbp-0x58]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 113f4 call 0x1153a
            { let r = { let t = m.env.code_va(0x1153a); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 113f9 mov r12,rax
            r12 = rax;
            // 113fc mov r13,QWORD PTR [rbp-0x40]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 11400 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 11404 mov rdi,r13
            rdi = r13;
            // 11407 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1140a mov r15,rax
            r15 = rax;
            // 1140d mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 11411 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 11414 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 11417 mov rdi,r14
            rdi = r14;
            // 1141a mov rsi,rbx
            rsi = rbx;
            // 1141d mov rdx,r12
            rdx = r12;
            // 11420 mov rcx,r13
            rcx = r13;
            // 11423 mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // 11426 mov r9d,eax
            r9 = ((rax as u32 as u64)) as u32 as u64;
            // 11429 movsd xmm0,QWORD PTR [rip+0xee757] # 0xffb88
            x0 = m.r64(m.env.data_va(82773)) as u128;
            // 11431 movsd xmm1,QWORD PTR [rbp-0x98]
            x1 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u128;
            // 11439 call 0xe0562
            { let r = { let t = m.env.code_va(0xe0562); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1143e mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 11442 cmp rdi,QWORD PTR [rbx+0x18]
            fl = sub::<64>(rdi, m.r64(rbx.wrapping_add(0x18)) as u64).1;
            // 11446 je 0x11457
            if fl.e() { pc = 0x11457; continue; }
            pc = 0x11448;
        }
        0x11448 => {
            // block 11448
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11448 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11448 mov rsi,r14
            rsi = r14;
            // 1144b call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 11450 add QWORD PTR [rbx+0x10],0x38
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, 0x38u64);
              m.w64(rbx.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 11455 jmp 0x11466
            pc = 0x11466; continue;
        }
        0x11457 => {
            // block 11457
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11457 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11457 mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 1145e mov rsi,r14
            rsi = r14;
            // 11461 call 0xeb8ee
            { let r = { let t = m.env.stub_va(0xbb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x11466;
        }
        0x11466 => {
            // block 11466
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11466 mov rdi,r14
            rdi = r14;
            // 11469 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1146e mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 11472 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 11475 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 11479 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1147c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 1147e je 0x113b8
            if fl.e() { pc = 0x113b8; continue; }
            pc = 0x11484;
        }
        0x11484 => {
            // block 11484
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11484 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11484 mov r14,QWORD PTR [rbx+0x8]
            r14 = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 11488 mov r15,QWORD PTR [rbx+0x10]
            r15 = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 1148c mov rax,r15
            rax = r15;
            // 1148f sub rax,r14
            { let (r, f) = sub::<64>(rax, r14);
              rax = r;
              fl = f; }
            // 11492 mov rcx,rax
            rcx = rax;
            // 11495 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 11499 movabs rbx,0x6db6db6db6db6db7
            rbx = 0x6db6_db6d_b6db_6db7u64;
            // 114a3 imul rbx,rcx
            { let (r, f) = imul::<64>(rbx, rcx); rbx = r; fl = f; }
            // 114a7 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // 114aa test rax,rax
            fl = logic::<64>(rax & rax);
            // 114ad mov r12d,0x0
            r12 = (0x0u64) as u32 as u64;
            // 114b3 jle 0x114c3
            if fl.le() { pc = 0x114c3; continue; }
            pc = 0x114b5;
        }
        0x114b5 => {
            // block 114b5
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114b5 mov rdi,rbx
            rdi = rbx;
            // 114b8 call 0xeb8d6
            { let r = { let t = m.env.stub_va(0xa3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 114bd mov r12,rax
            r12 = rax;
            // 114c0 mov r9,rdx
            r9 = rdx;
            pc = 0x114c3;
        }
        0x114c3 => {
            // block 114c3
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114c3 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 114c7 mov rdi,r14
            rdi = r14;
            // 114ca mov rsi,r15
            rsi = r15;
            // 114cd mov rcx,rbx
            rcx = rbx;
            // 114d0 mov r8,r12
            r8 = r12;
            // 114d3 call 0xeb8a6
            { let r = { let t = m.env.stub_va(0x73); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 114d8 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 114db je 0x114e5
            if fl.e() { pc = 0x114e5; continue; }
            pc = 0x114dd;
        }
        0x114dd => {
            // block 114dd
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114dd mov rdi,r12
            rdi = r12;
            // 114e0 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x114e5;
        }
        0x114e5 => {
            // block 114e5
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114e5 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x114e8;
        }
        0x114e8 => {
            // block 114e8
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114e8 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 114ec test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 114ef je 0x114ff
            if fl.e() { pc = 0x114ff; continue; }
            pc = 0x114f1;
        }
        0x114f1 => {
            // block 114f1
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114f1 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 114f4 call QWORD PTR [rax+0x10]
            { let r = { let t = m.r64(rax.wrapping_add(0x10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 114f7 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            pc = 0x114ff;
        }
        0x114ff => {
            // block 114ff
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114ff mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 11502 add rsp,0x78
            { let (r, f) = add::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // 11506 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11507 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 11509 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1150b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1150d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1150f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11510 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x11511 => {
            // block 11511
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11511 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11511 mov rbx,rax
            rbx = rax;
            // 11514 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 1151b call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x11520;
        }
        0x11520 => {
            // block 11520
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11520 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11520 mov rdi,rbx
            rdi = rbx;
            // 11523 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 11528 mov rbx,rax
            rbx = rax;
            // 1152b test r12,r12
            fl = logic::<64>(r12 & r12);
            // 1152e je 0x11520
            if fl.e() { pc = 0x11520; continue; }
            pc = 0x11530;
        }
        0x11530 => {
            // block 11530
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11530 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11530 mov rdi,r12
            rdi = r12;
            // 11533 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 11538 jmp 0x11520
            pc = 0x11520; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe850c size=492
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e850c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe850c;
    loop {
        match pc {
        0xe850c => {
            // block e850c
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e850c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e850c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e850d mov rbp,rsp
            rbp = rsp;
            // e8510 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e8512 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e8514 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e8516 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e8518 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e8519 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e851a mov r12,rsi
            r12 = rsi;
            // e851d mov r14,rdi
            r14 = rdi;
            // e8520 lea rsi,[r12+0x2a8]
            rsi = r12.wrapping_add(0x2a8);
            // e8528 movzx r15d,BYTE PTR [r12+0x2a8]
            r15 = (m.r8(r12.wrapping_add(0x2a8)) as u64) as u32 as u64;
            // e8531 mov r9b,r15b
            r9 = (r9 & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // e8534 and r9b,0x1
            { let r = (r9 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r9 = (r9 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e8538 jne 0xe8546
            if fl.ne() { pc = 0xe8546; continue; }
            pc = 0xe853a;
        }
        0xe853a => {
            // block e853a
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e853a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e853a lea r11,[rsi+0x1]
            r11 = rsi.wrapping_add(0x1);
            // e853e mov rcx,r15
            rcx = r15;
            // e8541 shr rcx,1
            { let (r, f) = shr::<64>(rcx, 0x1u64, fl); rcx = r; fl = f; }
            // e8544 jmp 0xe8556
            pc = 0xe8556; continue;
        }
        0xe8546 => {
            // block e8546
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8546 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8546 mov rcx,QWORD PTR [r12+0x2b0]
            rcx = m.r64(r12.wrapping_add(0x2b0)) as u64;
            // e854e mov r11,QWORD PTR [r12+0x2b8]
            r11 = m.r64(r12.wrapping_add(0x2b8)) as u64;
            pc = 0xe8556;
        }
        0xe8556 => {
            // block e8556
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8556 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8556 mov rdx,0xffffffffffffffff
            rdx = 0xffff_ffff_ffff_ffffu64;
            // e855d cmp rcx,0xe
            fl = sub::<64>(rcx, 0xeu64).1;
            // e8561 jb 0xe85bd
            if fl.b() { pc = 0xe85bd; continue; }
            pc = 0xe8563;
        }
        0xe8563 => {
            // block e8563
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8563 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8563 cmp rcx,0xe
            fl = sub::<64>(rcx, 0xeu64).1;
            // e8567 jl 0xe85bd
            if fl.l() { pc = 0xe85bd; continue; }
            pc = 0xe8569;
        }
        0xe8569 => {
            // block e8569
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8569 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8569 mov rax,rcx
            rax = rcx;
            // e856c add rax,0xfffffffffffffff3
            { let (r, f) = add::<64>(rax, 0xffff_ffff_ffff_fff3u64);
              rax = r;
              fl = f; }
            // e8570 je 0xe85bd
            if fl.e() { pc = 0xe85bd; continue; }
            pc = 0xe8572;
        }
        0xe8572 => {
            // block e8572
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8572 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8572 lea r8,[r11+rcx*1]
            r8 = r11.wrapping_add(rcx);
            // e8576 lea r10,[r11+rcx*1-0xd]
            r10 = r11.wrapping_add(rcx).wrapping_add(0xffff_ffff_ffff_fff3);
            // e857b lea r13,[rip+0x54cb8] # 0x13d23a
            r13 = m.env.data_va(334343);
            // e8582 mov rcx,r11
            rcx = r11;
            pc = 0xe8585;
        }
        0xe8585 => {
            // block e8585
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8585 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8585 movzx eax,BYTE PTR [rcx]
            rax = (m.r8(rcx) as u64) as u32 as u64;
            // e8588 cmp eax,0x50
            fl = sub::<32>((rax as u32 as u64), 0x50u64).1;
            // e858b mov edi,0x1
            rdi = (0x1u64) as u32 as u64;
            // e8590 jne 0xe85a8
            if fl.ne() { pc = 0xe85a8; continue; }
            pc = 0xe8592;
        }
        0xe8592 => {
            // block e8592
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8592 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8592 cmp rdi,0xe
            fl = sub::<64>(rdi, 0xeu64).1;
            // e8596 je 0xe85b2
            if fl.e() { pc = 0xe85b2; continue; }
            pc = 0xe8598;
        }
        0xe8598 => {
            // block e8598
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8598 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8598 movzx eax,BYTE PTR [rdi+r13*1]
            rax = (m.r8(rdi.wrapping_add(r13)) as u64) as u32 as u64;
            // e859d movzx ebx,BYTE PTR [rcx+rdi*1]
            rbx = (m.r8(rcx.wrapping_add(rdi)) as u64) as u32 as u64;
            // e85a1 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e85a4 cmp ebx,eax
            fl = sub::<32>((rbx as u32 as u64), (rax as u32 as u64)).1;
            // e85a6 je 0xe8592
            if fl.e() { pc = 0xe8592; continue; }
            pc = 0xe85a8;
        }
        0xe85a8 => {
            // block e85a8
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85a8 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // e85ab cmp rcx,r10
            fl = sub::<64>(rcx, r10).1;
            // e85ae jne 0xe8585
            if fl.ne() { pc = 0xe8585; continue; }
            pc = 0xe85b0;
        }
        0xe85b0 => {
            // block e85b0
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85b0 jmp 0xe85bd
            pc = 0xe85bd; continue;
        }
        0xe85b2 => {
            // block e85b2
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85b2 cmp rcx,r8
            fl = sub::<64>(rcx, r8).1;
            // e85b5 je 0xe85bd
            if fl.e() { pc = 0xe85bd; continue; }
            pc = 0xe85b7;
        }
        0xe85b7 => {
            // block e85b7
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85b7 sub rcx,r11
            { let (r, f) = sub::<64>(rcx, r11);
              rcx = r;
              fl = f; }
            // e85ba mov rdx,rcx
            rdx = rcx;
            pc = 0xe85bd;
        }
        0xe85bd => {
            // block e85bd
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85bd test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // e85c0 jne 0xe85ce
            if fl.ne() { pc = 0xe85ce; continue; }
            pc = 0xe85c2;
        }
        0xe85c2 => {
            // block e85c2
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85c2 lea r8,[rsi+0x1]
            r8 = rsi.wrapping_add(0x1);
            // e85c6 mov rdi,r15
            rdi = r15;
            // e85c9 shr rdi,1
            { let (r, f) = shr::<64>(rdi, 0x1u64, fl); rdi = r; fl = f; }
            // e85cc jmp 0xe85de
            pc = 0xe85de; continue;
        }
        0xe85ce => {
            // block e85ce
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85ce mov rdi,QWORD PTR [r12+0x2b0]
            rdi = m.r64(r12.wrapping_add(0x2b0)) as u64;
            // e85d6 mov r8,QWORD PTR [r12+0x2b8]
            r8 = m.r64(r12.wrapping_add(0x2b8)) as u64;
            pc = 0xe85de;
        }
        0xe85de => {
            // block e85de
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85de mov rcx,0xfffffffffffffffe
            rcx = 0xffff_ffff_ffff_fffeu64;
            // e85e5 cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e85e9 jb 0xe864f
            if fl.b() { pc = 0xe864f; continue; }
            pc = 0xe85eb;
        }
        0xe85eb => {
            // block e85eb
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85eb cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e85ef jl 0xe864f
            if fl.l() { pc = 0xe864f; continue; }
            pc = 0xe85f1;
        }
        0xe85f1 => {
            // block e85f1
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85f1 mov rax,rdi
            rax = rdi;
            // e85f4 add rax,0xfffffffffffffffd
            { let (r, f) = add::<64>(rax, 0xffff_ffff_ffff_fffdu64);
              rax = r;
              fl = f; }
            // e85f8 je 0xe864f
            if fl.e() { pc = 0xe864f; continue; }
            pc = 0xe85fa;
        }
        0xe85fa => {
            // block e85fa
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85fa lea rax,[r8+rdi*1]
            rax = r8.wrapping_add(rdi);
            // e85fe mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // e8602 lea r11,[r8+rdi*1-0x3]
            r11 = r8.wrapping_add(rdi).wrapping_add(0xffff_ffff_ffff_fffd);
            // e8607 lea r13,[rip+0x54c3b] # 0x13d249
            r13 = m.env.data_va(334358);
            // e860e mov rbx,r8
            rbx = r8;
            pc = 0xe8611;
        }
        0xe8611 => {
            // block e8611
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8611 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8611 movzx eax,BYTE PTR [rbx]
            rax = (m.r8(rbx) as u64) as u32 as u64;
            // e8614 cmp eax,0x44
            fl = sub::<32>((rax as u32 as u64), 0x44u64).1;
            // e8617 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // e861c jne 0xe8636
            if fl.ne() { pc = 0xe8636; continue; }
            pc = 0xe861e;
        }
        0xe861e => {
            // block e861e
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e861e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e861e cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // e8622 je 0xe8640
            if fl.e() { pc = 0xe8640; continue; }
            pc = 0xe8624;
        }
        0xe8624 => {
            // block e8624
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8624 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8624 movzx edi,BYTE PTR [rax+r13*1]
            rdi = (m.r8(rax.wrapping_add(r13)) as u64) as u32 as u64;
            // e8629 movzx r10d,BYTE PTR [rbx+rax*1]
            r10 = (m.r8(rbx.wrapping_add(rax)) as u64) as u32 as u64;
            // e862e inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // e8631 cmp r10d,edi
            fl = sub::<32>((r10 as u32 as u64), (rdi as u32 as u64)).1;
            // e8634 je 0xe861e
            if fl.e() { pc = 0xe861e; continue; }
            pc = 0xe8636;
        }
        0xe8636 => {
            // block e8636
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8636 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8636 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e8639 cmp rbx,r11
            fl = sub::<64>(rbx, r11).1;
            // e863c jne 0xe8611
            if fl.ne() { pc = 0xe8611; continue; }
            pc = 0xe863e;
        }
        0xe863e => {
            // block e863e
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e863e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e863e jmp 0xe864f
            pc = 0xe864f; continue;
        }
        0xe8640 => {
            // block e8640
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8640 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8640 cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // e8644 je 0xe864f
            if fl.e() { pc = 0xe864f; continue; }
            pc = 0xe8646;
        }
        0xe8646 => {
            // block e8646
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8646 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8646 sub rbx,r8
            { let (r, f) = sub::<64>(rbx, r8);
              rbx = r;
              fl = f; }
            // e8649 dec rbx
            { let (r, f) = dec::<64>(rbx, fl); rbx = r; fl = f; }
            // e864c mov rcx,rbx
            rcx = rbx;
            pc = 0xe864f;
        }
        0xe864f => {
            // block e864f
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e864f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e864f test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // e8652 jne 0xe865d
            if fl.ne() { pc = 0xe865d; continue; }
            pc = 0xe8654;
        }
        0xe8654 => {
            // block e8654
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8654 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8654 lea r10,[rsi+0x1]
            r10 = rsi.wrapping_add(0x1);
            // e8658 shr r15,1
            { let (r, f) = shr::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // e865b jmp 0xe866d
            pc = 0xe866d; continue;
        }
        0xe865d => {
            // block e865d
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e865d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e865d mov r15,QWORD PTR [r12+0x2b0]
            r15 = m.r64(r12.wrapping_add(0x2b0)) as u64;
            // e8665 mov r10,QWORD PTR [r12+0x2b8]
            r10 = m.r64(r12.wrapping_add(0x2b8)) as u64;
            pc = 0xe866d;
        }
        0xe866d => {
            // block e866d
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e866d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e866d mov r11,0xffffffffffffffff
            r11 = 0xffff_ffff_ffff_ffffu64;
            // e8674 cmp r15,0xe
            fl = sub::<64>(r15, 0xeu64).1;
            // e8678 jb 0xe86d7
            if fl.b() { pc = 0xe86d7; continue; }
            pc = 0xe867a;
        }
        0xe867a => {
            // block e867a
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e867a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e867a cmp r15,0xe
            fl = sub::<64>(r15, 0xeu64).1;
            // e867e jl 0xe86d7
            if fl.l() { pc = 0xe86d7; continue; }
            pc = 0xe8680;
        }
        0xe8680 => {
            // block e8680
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8680 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8680 mov rax,r15
            rax = r15;
            // e8683 add rax,0xfffffffffffffff3
            { let (r, f) = add::<64>(rax, 0xffff_ffff_ffff_fff3u64);
              rax = r;
              fl = f; }
            // e8687 je 0xe86d7
            if fl.e() { pc = 0xe86d7; continue; }
            pc = 0xe8689;
        }
        0xe8689 => {
            // block e8689
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8689 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8689 lea r8,[r10+r15*1]
            r8 = r10.wrapping_add(r15);
            // e868d lea r9,[r10+r15*1-0xd]
            r9 = r10.wrapping_add(r15).wrapping_add(0xffff_ffff_ffff_fff3);
            // e8692 lea r15,[rip+0x54ba1] # 0x13d23a
            r15 = m.env.data_va(334343);
            // e8699 mov r12,r10
            r12 = r10;
            pc = 0xe869c;
        }
        0xe869c => {
            // block e869c
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e869c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e869c movzx eax,BYTE PTR [r12]
            rax = (m.r8(r12) as u64) as u32 as u64;
            // e86a1 cmp eax,0x50
            fl = sub::<32>((rax as u32 as u64), 0x50u64).1;
            // e86a4 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // e86a9 jne 0xe86c2
            if fl.ne() { pc = 0xe86c2; continue; }
            pc = 0xe86ab;
        }
        0xe86ab => {
            // block e86ab
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86ab cmp rax,0xe
            fl = sub::<64>(rax, 0xeu64).1;
            // e86af je 0xe86cc
            if fl.e() { pc = 0xe86cc; continue; }
            pc = 0xe86b1;
        }
        0xe86b1 => {
            // block e86b1
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86b1 movzx ebx,BYTE PTR [rax+r15*1]
            rbx = (m.r8(rax.wrapping_add(r15)) as u64) as u32 as u64;
            // e86b6 movzx edi,BYTE PTR [r12+rax*1]
            rdi = (m.r8(r12.wrapping_add(rax)) as u64) as u32 as u64;
            // e86bb inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // e86be cmp edi,ebx
            fl = sub::<32>((rdi as u32 as u64), (rbx as u32 as u64)).1;
            // e86c0 je 0xe86ab
            if fl.e() { pc = 0xe86ab; continue; }
            pc = 0xe86c2;
        }
        0xe86c2 => {
            // block e86c2
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86c2 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // e86c5 cmp r12,r9
            fl = sub::<64>(r12, r9).1;
            // e86c8 jne 0xe869c
            if fl.ne() { pc = 0xe869c; continue; }
            pc = 0xe86ca;
        }
        0xe86ca => {
            // block e86ca
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86ca jmp 0xe86d7
            pc = 0xe86d7; continue;
        }
        0xe86cc => {
            // block e86cc
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86cc cmp r12,r8
            fl = sub::<64>(r12, r8).1;
            // e86cf je 0xe86d7
            if fl.e() { pc = 0xe86d7; continue; }
            pc = 0xe86d1;
        }
        0xe86d1 => {
            // block e86d1
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86d1 sub r12,r10
            { let (r, f) = sub::<64>(r12, r10);
              r12 = r;
              fl = f; }
            // e86d4 mov r11,r12
            r11 = r12;
            pc = 0xe86d7;
        }
        0xe86d7 => {
            // block e86d7
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86d7 sub rcx,r11
            { let (r, f) = sub::<64>(rcx, r11);
              rcx = r;
              fl = f; }
            // e86da mov rdi,r14
            rdi = r14;
            // e86dd mov r8,rsi
            r8 = rsi;
            // e86e0 call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e86e5 mov rax,r14
            rax = r14;
            // e86e8 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e86ec pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e86ed pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e86ef pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e86f1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e86f3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e86f5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e86f6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a9e size=488
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001a9e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a9e;
    loop {
        match pc {
        0x1a9e => {
            // block 1a9e
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1a9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a9e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a9f mov rbp,rsp
            rbp = rsp;
            // 1aa2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1aa4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1aa6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1aa7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1aa8 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 1aab mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 1aad mov r14,rdi
            r14 = rdi;
            // 1ab0 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // 1ab4 call 0xcb6a1
            { let r = { let t = m.env.code_va(0xcb6a1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1ab9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1abb je 0x1b2f
            if fl.e() { pc = 0x1b2f; continue; }
            pc = 0x1abd;
        }
        0x1abd => {
            // block 1abd
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1abd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1abd mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1ac0 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 1ac4 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1ac6 mov rdi,r14
            rdi = r14;
            // 1ac9 call QWORD PTR [rax+0x478]
            { let r = { let t = m.r64(rax.wrapping_add(0x478)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1acf mov ecx,DWORD PTR [rbp-0x1c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 1ad2 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1ad5 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 1ad9 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 1adb je 0x1b36
            if fl.e() { pc = 0x1b36; continue; }
            pc = 0x1add;
        }
        0x1add => {
            // block 1add
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1add rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1add test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // 1adf je 0x1b36
            if fl.e() { pc = 0x1b36; continue; }
            pc = 0x1ae1;
        }
        0x1ae1 => {
            // block 1ae1
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1ae1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ae1 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1ae6 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 1aeb cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 1aef jg 0x1b97
            if fl.g() { pc = 0x1b97; continue; }
            pc = 0x1af5;
        }
        0x1af5 => {
            // block 1af5
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1af5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1af5 mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 1afa mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 1b00 mov rdi,r14
            rdi = r14;
            // 1b03 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b05 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1b08 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1b0d mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 1b12 mov ecx,0x200000
            rcx = (0x200000u64) as u32 as u64;
            // 1b17 mov r8d,0x15
            r8 = (0x15u64) as u32 as u64;
            // 1b1d mov rdi,r14
            rdi = r14;
            // 1b20 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b23 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1b26 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1b2b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1b2d jmp 0x1b87
            pc = 0x1b87; continue;
        }
        0x1b2f => {
            // block 1b2f
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1b2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b2f xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1b31 jmp 0x1c79
            pc = 0x1c79; continue;
        }
        0x1b36 => {
            // block 1b36
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1b36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b36 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1b3b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1b3d test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 1b3f je 0x1bd1
            if fl.e() { pc = 0x1bd1; continue; }
            pc = 0x1b45;
        }
        0x1b45 => {
            // block 1b45
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1b45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b45 cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 1b49 jg 0x1bfe
            if fl.g() { pc = 0x1bfe; continue; }
            pc = 0x1b4f;
        }
        0x1b4f => {
            // block 1b4f
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1b4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b4f mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 1b54 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 1b5a mov rdi,r14
            rdi = r14;
            // 1b5d call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b5f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1b62 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1b67 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1b69 mov ecx,0x200000
            rcx = (0x200000u64) as u32 as u64;
            // 1b6e mov r8d,0x15
            r8 = (0x15u64) as u32 as u64;
            // 1b74 mov rdi,r14
            rdi = r14;
            // 1b77 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b7a mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1b7d mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1b82 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            pc = 0x1b87;
        }
        0x1b87 => {
            // block 1b87
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1b87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b87 mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // 1b8c mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // 1b92 jmp 0x1c41
            pc = 0x1c41; continue;
        }
        0x1b97 => {
            // block 1b97
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1b97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b97 mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 1b9c mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 1ba2 mov rdi,r14
            rdi = r14;
            // 1ba5 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1ba7 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1baa mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1baf mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 1bb4 mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 1bb9 mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            // 1bbf mov rdi,r14
            rdi = r14;
            // 1bc2 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1bc5 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1bc8 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1bcd xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1bcf jmp 0x1c36
            pc = 0x1c36; continue;
        }
        0x1bd1 => {
            // block 1bd1
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1bd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1bd1 cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 1bd5 jg 0x1c4b
            if fl.g() { pc = 0x1c4b; continue; }
            pc = 0x1bd7;
        }
        0x1bd7 => {
            // block 1bd7
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1bd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1bd7 mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 1bdc mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 1be2 mov rdi,r14
            rdi = r14;
            // 1be5 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1be7 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1bea mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1bef xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1bf1 mov ecx,0x200000
            rcx = (0x200000u64) as u32 as u64;
            // 1bf6 mov r8d,0x15
            r8 = (0x15u64) as u32 as u64;
            // 1bfc jmp 0x1c70
            pc = 0x1c70; continue;
        }
        0x1bfe => {
            // block 1bfe
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1bfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1bfe mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 1c03 mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 1c09 mov rdi,r14
            rdi = r14;
            // 1c0c call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1c0e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1c11 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1c16 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1c18 mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 1c1d mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            // 1c23 mov rdi,r14
            rdi = r14;
            // 1c26 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1c29 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1c2c mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1c31 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            pc = 0x1c36;
        }
        0x1c36 => {
            // block 1c36
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1c36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1c36 mov ecx,0x1000
            rcx = (0x1000u64) as u32 as u64;
            // 1c3b mov r8d,0xc
            r8 = (0xcu64) as u32 as u64;
            pc = 0x1c41;
        }
        0x1c41 => {
            // block 1c41
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1c41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1c41 mov rdi,r14
            rdi = r14;
            // 1c44 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1c47 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1c49 jmp 0x1c79
            pc = 0x1c79; continue;
        }
        0x1c4b => {
            // block 1c4b
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1c4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1c4b mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 1c50 mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 1c56 mov rdi,r14
            rdi = r14;
            // 1c59 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1c5b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1c5e mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1c63 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1c65 mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 1c6a mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            pc = 0x1c70;
        }
        0x1c70 => {
            // block 1c70
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1c70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1c70 mov rdi,r14
            rdi = r14;
            // 1c73 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1c76 xor bl,0x1
            { let r = (rbx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rbx = (rbx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            pc = 0x1c79;
        }
        0x1c79 => {
            // block 1c79
            #[cfg(feature = "trace")] eprintln!("f_00001a9e pc 1c79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1c79 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 1c7b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1c7f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1c80 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1c82 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1c84 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1c85 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7586 size=478
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d7586<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd7586;
    loop {
        match pc {
        0xd7586 => {
            // block d7586
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7586 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7586 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7587 mov rbp,rsp
            rbp = rsp;
            // d758a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d758c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d758e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d7590 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d7592 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d7593 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d7597 mov r14,rdi
            r14 = rdi;
            // d759a mov rdi,QWORD PTR [r14+0x6b8]
            rdi = m.r64(r14.wrapping_add(0x6b8)) as u64;
            // d75a1 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d75a4 je 0xd7642
            if fl.e() { pc = 0xd7642; continue; }
            pc = 0xd75aa;
        }
        0xd75aa => {
            // block d75aa
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d75aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d75aa xorps xmm0,xmm0
            x0 = 0;
            // d75ad movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // d75b1 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // d75b9 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d75bc mov rax,QWORD PTR [rax+0x48]
            rax = m.r64(rax.wrapping_add(0x48)) as u64;
            // d75c0 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d75c2 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d75c5 mov rdi,QWORD PTR [r14+0x6b8]
            rdi = m.r64(r14.wrapping_add(0x6b8)) as u64;
            // d75cc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d75cf mov rax,QWORD PTR [rax+0x40]
            rax = m.r64(rax.wrapping_add(0x40)) as u64;
            // d75d3 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d75d5 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d75d7 mov rdi,QWORD PTR [r14+0x6b8]
            rdi = m.r64(r14.wrapping_add(0x6b8)) as u64;
            // d75de mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d75e1 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d75e5 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // d75ea call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d75ec mov rdi,QWORD PTR [r14+0x6b8]
            rdi = m.r64(r14.wrapping_add(0x6b8)) as u64;
            // d75f3 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // d75f6 mov rbx,QWORD PTR [rcx+0x10]
            rbx = m.r64(rcx.wrapping_add(0x10)) as u64;
            // d75fa lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d75fe mov ecx,0x100
            rcx = (0x100u64) as u32 as u64;
            // d7603 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d7605 call rbx
            { let r = { let t = rbx; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7607 mov rdi,QWORD PTR [r14+0x6b8]
            rdi = m.r64(r14.wrapping_add(0x6b8)) as u64;
            // d760e mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // d7611 mov rcx,QWORD PTR [rcx+0x40]
            rcx = m.r64(rcx.wrapping_add(0x40)) as u64;
            // d7615 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d7617 je 0xd770e
            if fl.e() { pc = 0xd770e; continue; }
            pc = 0xd761d;
        }
        0xd761d => {
            // block d761d
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d761d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d761d movzx esi,r15b
            rsi = ((r15 as u8 as u64)) as u32 as u64;
            // d7621 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7623 add r14,0x318
            { let (r, f) = add::<64>(r14, 0x318u64);
              r14 = r;
              fl = f; }
            // d762a mov rsi,QWORD PTR [rbp-0x40]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d762e mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d7632 mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // d7635 mov rdi,r14
            rdi = r14;
            // d7638 call 0xebeb2
            { let r = { let t = m.env.stub_va(0x67f); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d763d jmp 0xd7717
            pc = 0xd7717; continue;
        }
        0xd7642 => {
            // block d7642
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7642 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7642 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d7645 cmp DWORD PTR [r14+0x330],0x10220
            fl = sub::<32>(m.r32(r14.wrapping_add(0x330)) as u64, 0x10220u64).1;
            // d7650 jne 0xd772f
            if fl.ne() { pc = 0xd772f; continue; }
            pc = 0xd7656;
        }
        0xd7656 => {
            // block d7656
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7656 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7656 mov r12d,DWORD PTR [r14+0x34c]
            r12 = (m.r32(r14.wrapping_add(0x34c)) as u64) as u32 as u64;
            // d765d mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // d7662 mov rdi,r14
            rdi = r14;
            // d7665 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d766a mov edi,0x100
            rdi = (0x100u64) as u32 as u64;
            // d766f call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7674 mov r15,rax
            r15 = rax;
            // d7677 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd7679;
        }
        0xd7679 => {
            // block d7679
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7679 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7679 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d767c lea edx,[r12+rbx*1]
            rdx = (r12.wrapping_add(rbx)) as u32 as u64;
            // d7680 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d7685 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d768a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d768d mov rdi,r14
            rdi = r14;
            // d7690 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7693 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d7696 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d769b mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // d76a0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d76a5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d76a8 mov rdi,r14
            rdi = r14;
            // d76ab call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d76ae mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d76b1 mov rdi,r14
            rdi = r14;
            // d76b4 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d76ba mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d76bd lea rdx,[r15+rbx*1]
            rdx = r15.wrapping_add(rbx);
            // d76c1 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d76c6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d76cb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d76ce mov rdi,r14
            rdi = r14;
            // d76d1 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d76d4 cmp DWORD PTR [r15+rbx*1],0x0
            fl = sub::<32>(m.r32(r15.wrapping_add(rbx)) as u64, 0x0u64).1;
            // d76d9 je 0xd76e8
            if fl.e() { pc = 0xd76e8; continue; }
            pc = 0xd76db;
        }
        0xd76db => {
            // block d76db
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d76db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d76db inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // d76de add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // d76e2 cmp r13d,0x40
            fl = sub::<32>((r13 as u32 as u64), 0x40u64).1;
            // d76e6 jb 0xd7679
            if fl.b() { pc = 0xd7679; continue; }
            pc = 0xd76e8;
        }
        0xd76e8 => {
            // block d76e8
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d76e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d76e8 lea rdi,[r14+0x318]
            rdi = r14.wrapping_add(0x318);
            // d76ef mov rsi,r15
            rsi = r15;
            // d76f2 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d76f7 mov rdi,r15
            rdi = r15;
            // d76fa call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d76ff xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d7701 mov rdi,r14
            rdi = r14;
            // d7704 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7709 mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // d770c jmp 0xd772f
            pc = 0xd772f; continue;
        }
        0xd770e => {
            // block d770e
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d770e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d770e movzx esi,r15b
            rsi = ((r15 as u8 as u64)) as u32 as u64;
            // d7712 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7714 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd7717;
        }
        0xd7717 => {
            // block d7717
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7717 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7717 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d771b test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d771e je 0xd772f
            if fl.e() { pc = 0xd772f; continue; }
            pc = 0xd7720;
        }
        0xd7720 => {
            // block d7720
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7720 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7720 cmp QWORD PTR [rbp-0x38],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, rdi).1;
            // d7724 je 0xd772a
            if fl.e() { pc = 0xd772a; continue; }
            pc = 0xd7726;
        }
        0xd7726 => {
            // block d7726
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7726 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7726 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            pc = 0xd772a;
        }
        0xd772a => {
            // block d772a
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d772a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d772a call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd772f;
        }
        0xd772f => {
            // block d772f
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d772f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d772f mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // d7732 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d7736 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d7737 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d7739 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d773b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d773d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d773f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d7740 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd7741 => {
            // block d7741
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7741 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7741 mov rbx,rax
            rbx = rax;
            // d7744 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d7748 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d774b je 0xd775c
            if fl.e() { pc = 0xd775c; continue; }
            pc = 0xd774d;
        }
        0xd774d => {
            // block d774d
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d774d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d774d cmp QWORD PTR [rbp-0x38],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, rdi).1;
            // d7751 je 0xd7757
            if fl.e() { pc = 0xd7757; continue; }
            pc = 0xd7753;
        }
        0xd7753 => {
            // block d7753
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7753 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7753 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            pc = 0xd7757;
        }
        0xd7757 => {
            // block d7757
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7757 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7757 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd775c;
        }
        0xd775c => {
            // block d775c
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d775c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d775c mov rdi,rbx
            rdi = rbx;
            // d775f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd8cba size=466
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d8cba<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd8cba;
    loop {
        match pc {
        0xd8cba => {
            // block d8cba
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8cba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8cba push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d8cbb mov rbp,rsp
            rbp = rsp;
            // d8cbe push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d8cc0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d8cc2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d8cc4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d8cc6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d8cc7 sub rsp,0xa8
            { let (r, f) = sub::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // d8cce mov r13,rdi
            r13 = rdi;
            // d8cd1 mov rax,QWORD PTR [rip+0x67700] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // d8cd8 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // d8cdb mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // d8cdf mov ecx,0x2000050
            rcx = (0x2000050u64) as u32 as u64;
            // d8ce4 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d8ce7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd8ce9;
        }
        0xd8ce9 => {
            // block d8ce9
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8ce9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8ce9 mov QWORD PTR [rbp-0xb0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (rcx) as u64);
            // d8cf0 mov DWORD PTR [rbp-0xbc],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff44), ((rax as u32 as u64)) as u32);
            // d8cf6 mov QWORD PTR [rbp-0xb8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (r15) as u64);
            // d8cfd mov BYTE PTR [rbp-0xa0],0x3a
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x3au64) as u8);
            // d8d04 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // d8d09 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8d0b lea rdi,[rbp-0x9f]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff61);
            // d8d12 lea rbx,[rip+0x6018a] # 0x138ea3
            rbx = m.env.data_va(317040);
            // d8d19 mov rsi,rbx
            rsi = rbx;
            // d8d1c call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8d21 mov r14d,r15d
            r14 = ((r15 as u32 as u64)) as u32 as u64;
            // d8d24 shr r14d,0x8
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x8u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // d8d28 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // d8d2c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8d2e lea rdi,[rbp-0x9d]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff63);
            // d8d35 mov rsi,rbx
            rsi = rbx;
            // d8d38 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8d3d movzx edx,r15b
            rdx = ((r15 as u8 as u64)) as u32 as u64;
            // d8d41 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8d43 lea rdi,[rbp-0x9b]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff65);
            // d8d4a mov rsi,rbx
            rsi = rbx;
            // d8d4d call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8d52 lea r14d,[r15+r14*1+0x10]
            r14 = (r15.wrapping_add(r14).wrapping_add(0x10)) as u32 as u64;
            // d8d57 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d8d59 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8d5b lea rdi,[rbp-0x99]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff67);
            // d8d62 mov rsi,rbx
            rsi = rbx;
            // d8d65 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8d6a xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d8d6d lea r12,[rbp-0x97]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff69);
            pc = 0xd8d74;
        }
        0xd8d74 => {
            // block d8d74
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8d74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8d74 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d8d78 mov rcx,QWORD PTR [rbp-0xb0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // d8d7f lea edx,[rcx+r15*1]
            rdx = (rcx.wrapping_add(r15)) as u32 as u64;
            // d8d83 mov esi,0x5b
            rsi = (0x5bu64) as u32 as u64;
            // d8d88 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8d8d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8d90 mov rdi,r13
            rdi = r13;
            // d8d93 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8d96 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d8d9a mov esi,0x64
            rsi = (0x64u64) as u32 as u64;
            // d8d9f mov rdi,r13
            rdi = r13;
            // d8da2 call QWORD PTR [rax+0x120]
            { let r = { let t = m.r64(rax.wrapping_add(0x120)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8da8 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d8dac mov esi,0x5c
            rsi = (0x5cu64) as u32 as u64;
            // d8db1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8db6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8db9 mov rdi,r13
            rdi = r13;
            // d8dbc lea rdx,[rbp-0xa4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff5c);
            // d8dc3 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8dc6 mov eax,DWORD PTR [rbp-0xa4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff5c)) as u64) as u32 as u64;
            // d8dcc movzx ebx,ah
            rbx = (((rax >> 8) as u8 as u64)) as u32 as u64;
            // d8dcf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8dd1 mov rdi,r12
            rdi = r12;
            // d8dd4 lea rsi,[rip+0x600c8] # 0x138ea3
            rsi = m.env.data_va(317040);
            // d8ddb mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d8ddd call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8de2 movzx r14d,r14b
            r14 = ((r14 as u8 as u64)) as u32 as u64;
            // d8de6 add r14d,ebx
            { let (r, f) = add::<32>((r14 as u32 as u64), (rbx as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // d8de9 add r12,0x2
            { let (r, f) = add::<64>(r12, 0x2u64);
              r12 = r;
              fl = f; }
            // d8ded add r15d,0x100
            { let (r, f) = add::<32>((r15 as u32 as u64), 0x100u64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d8df4 cmp r15d,0x1000
            fl = sub::<32>((r15 as u32 as u64), 0x1000u64).1;
            // d8dfb jne 0xd8d74
            if fl.ne() { pc = 0xd8d74; continue; }
            pc = 0xd8e01;
        }
        0xd8e01 => {
            // block d8e01
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8e01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8e01 mov r15,QWORD PTR [rbp-0xb8]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // d8e08 movzx r15d,r15w
            r15 = ((r15 as u16 as u64)) as u32 as u64;
            // d8e0c mov rbx,QWORD PTR [rbp-0xb0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // d8e13 add ebx,0x1000
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x1000u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d8e19 add r15d,0x10
            { let (r, f) = add::<32>((r15 as u32 as u64), 0x10u64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d8e1d neg r14d
            { let (r, f) = neg::<32>((r14 as u32 as u64)); r14 = (r) as u32 as u64; fl = f; }
            // d8e20 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // d8e24 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8e26 lea rdi,[rbp-0x77]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff89);
            // d8e2a lea rsi,[rip+0x60072] # 0x138ea3
            rsi = m.env.data_va(317040);
            // d8e31 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8e36 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d8e3d call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8e42 mov eax,DWORD PTR [rbp-0xbc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff44)) as u64) as u32 as u64;
            // d8e48 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d8e4a cmp eax,0x10
            fl = sub::<32>((rax as u32 as u64), 0x10u64).1;
            // d8e4d mov rcx,rbx
            rcx = rbx;
            // d8e50 jne 0xd8ce9
            if fl.ne() { pc = 0xd8ce9; continue; }
            pc = 0xd8e56;
        }
        0xd8e56 => {
            // block d8e56
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8e56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8e56 lea rdi,[rip+0x6005a] # 0x138eb7
            rdi = m.env.data_va(317060);
            // d8e5d call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8e62 mov rax,QWORD PTR [rip+0x6756f] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // d8e69 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // d8e6c cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // d8e70 jne 0xd8e86
            if fl.ne() { pc = 0xd8e86; continue; }
            pc = 0xd8e72;
        }
        0xd8e72 => {
            // block d8e72
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8e72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8e72 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d8e74 add rsp,0xa8
            { let (r, f) = add::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // d8e7b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d8e7c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d8e7e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d8e80 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d8e82 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d8e84 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d8e85 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd8e86 => {
            // block d8e86
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8e86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8e86 call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x323b2 size=454
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000323b2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x323b2;
    loop {
        match pc {
        0x323b2 => {
            // block 323b2
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 323b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 323b2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 323b3 mov rbp,rsp
            rbp = rsp;
            // 323b6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 323b7 mov QWORD PTR [rdi+0x8],0x0
            m.w64(rdi.wrapping_add(0x8), (0x0u64) as u64);
            // 323bf mov QWORD PTR [rdi],0x0
            m.w64(rdi, (0x0u64) as u64);
            // 323c6 mov DWORD PTR [rdi+0x10],0xc
            m.w32(rdi.wrapping_add(0x10), (0xcu64) as u32);
            // 323cd mov DWORD PTR [rdi+0x14],0x0
            m.w32(rdi.wrapping_add(0x14), (0x0u64) as u32);
            // 323d4 mov DWORD PTR [rdi+0x18],0x20
            m.w32(rdi.wrapping_add(0x18), (0x20u64) as u32);
            // 323db mov DWORD PTR [rdi+0x1c],0x3
            m.w32(rdi.wrapping_add(0x1c), (0x3u64) as u32);
            // 323e2 mov BYTE PTR [rdi+0x20],0x0
            m.w8(rdi.wrapping_add(0x20), (0x0u64) as u8);
            // 323e6 mov WORD PTR [rdi+0x34],0x0
            m.w16(rdi.wrapping_add(0x34), (0x0u64) as u16);
            // 323ec mov QWORD PTR [rdi+0x2c],0x0
            m.w64(rdi.wrapping_add(0x2c), (0x0u64) as u64);
            // 323f4 mov QWORD PTR [rdi+0x24],0x0
            m.w64(rdi.wrapping_add(0x24), (0x0u64) as u64);
            // 323fc mov DWORD PTR [rdi+0x38],0x12
            m.w32(rdi.wrapping_add(0x38), (0x12u64) as u32);
            // 32403 cmp esi,0xb
            fl = sub::<32>((rsi as u32 as u64), 0xbu64).1;
            // 32406 ja 0x32555
            if fl.a() { pc = 0x32555; continue; }
            pc = 0x3240c;
        }
        0x3240c => {
            // block 3240c
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3240c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3240c cmp edx,0x1f
            fl = sub::<32>((rdx as u32 as u64), 0x1fu64).1;
            // 3240f ja 0x32555
            if fl.a() { pc = 0x32555; continue; }
            pc = 0x32415;
        }
        0x32415 => {
            // block 32415
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32415 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32415 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 32418 ja 0x32555
            if fl.a() { pc = 0x32555; continue; }
            pc = 0x3241e;
        }
        0x3241e => {
            // block 3241e
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3241e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3241e cmp edx,0x17
            fl = sub::<32>((rdx as u32 as u64), 0x17u64).1;
            // 32421 sete r11b
            r11 = (r11 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 32425 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 32427 and eax,0xffffffef
            { let r = (rax as u32 as u64) & 0xffffffefu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 3242a cmp eax,0xa
            fl = sub::<32>((rax as u32 as u64), 0xau64).1;
            // 3242d sete r8b
            r8 = (r8 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 32431 lea eax,[rdx-0x1b]
            rax = (rdx.wrapping_add(0xffff_ffff_ffff_ffe5)) as u32 as u64;
            // 32434 cmp eax,0x5
            fl = sub::<32>((rax as u32 as u64), 0x5u64).1;
            // 32437 setb r9b
            r9 = (r9 & !0xff) | (((fl.b() as u64)) as u8 as u64);
            // 3243b lea r10d,[rcx-0x1]
            r10 = (rcx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 3243f cmp r10d,0x1
            fl = sub::<32>((r10 as u32 as u64), 0x1u64).1;
            // 32443 ja 0x32451
            if fl.a() { pc = 0x32451; continue; }
            pc = 0x32445;
        }
        0x32445 => {
            // block 32445
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32445 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32445 or r11b,r8b
            { let r = (r11 as u8 as u64) | (r8 as u8 as u64); let f = logic::<8>(r);
              r11 = (r11 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 32448 or r9b,r11b
            { let r = (r9 as u8 as u64) | (r11 as u8 as u64); let f = logic::<8>(r);
              r9 = (r9 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3244b jne 0x32555
            if fl.ne() { pc = 0x32555; continue; }
            pc = 0x32451;
        }
        0x32451 => {
            // block 32451
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32451 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32451 mov r8d,edx
            r8 = ((rdx as u32 as u64)) as u32 as u64;
            // 32454 mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // 32457 imul r11,r9,0x780
            { let (r, f) = imul::<64>(r9, 0x780u64); r11 = r; fl = f; }
            // 3245e lea rbx,[rip+0x19020b] # 0x1c2670
            rbx = m.env.data_va(880189);
            // 32465 add rbx,r11
            { let (r, f) = add::<64>(rbx, r11);
              rbx = r;
              fl = f; }
            // 32468 imul r8,r8,0x3c
            { let (r, f) = imul::<64>(r8, 0x3cu64); r8 = r; fl = f; }
            // 3246c mov eax,DWORD PTR [r8+rbx*1+0x38]
            rax = (m.r32(r8.wrapping_add(rbx).wrapping_add(0x38)) as u64) as u32 as u64;
            // 32471 mov DWORD PTR [rdi+0x38],eax
            m.w32(rdi.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32474 mov rax,QWORD PTR [r8+rbx*1+0x30]
            rax = m.r64(r8.wrapping_add(rbx).wrapping_add(0x30)) as u64;
            // 32479 mov QWORD PTR [rdi+0x30],rax
            m.w64(rdi.wrapping_add(0x30), (rax) as u64);
            // 3247d mov rax,QWORD PTR [r8+rbx*1+0x28]
            rax = m.r64(r8.wrapping_add(rbx).wrapping_add(0x28)) as u64;
            // 32482 mov QWORD PTR [rdi+0x28],rax
            m.w64(rdi.wrapping_add(0x28), (rax) as u64);
            // 32486 mov rax,QWORD PTR [r8+rbx*1+0x20]
            rax = m.r64(r8.wrapping_add(rbx).wrapping_add(0x20)) as u64;
            // 3248b mov QWORD PTR [rdi+0x20],rax
            m.w64(rdi.wrapping_add(0x20), (rax) as u64);
            // 3248f mov rax,QWORD PTR [r8+rbx*1+0x18]
            rax = m.r64(r8.wrapping_add(rbx).wrapping_add(0x18)) as u64;
            // 32494 mov QWORD PTR [rdi+0x18],rax
            m.w64(rdi.wrapping_add(0x18), (rax) as u64);
            // 32498 mov rax,QWORD PTR [r8+rbx*1+0x10]
            rax = m.r64(r8.wrapping_add(rbx).wrapping_add(0x10)) as u64;
            // 3249d mov QWORD PTR [rdi+0x10],rax
            m.w64(rdi.wrapping_add(0x10), (rax) as u64);
            // 324a1 mov rax,QWORD PTR [r8+rbx*1]
            rax = m.r64(r8.wrapping_add(rbx)) as u64;
            // 324a5 mov rbx,QWORD PTR [r8+rbx*1+0x8]
            rbx = m.r64(r8.wrapping_add(rbx).wrapping_add(0x8)) as u64;
            // 324aa mov QWORD PTR [rdi+0x8],rbx
            m.w64(rdi.wrapping_add(0x8), (rbx) as u64);
            // 324ae mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 324b1 mov DWORD PTR [rdi+0x10],esi
            m.w32(rdi.wrapping_add(0x10), ((rsi as u32 as u64)) as u32);
            // 324b4 mov DWORD PTR [rdi+0x18],edx
            m.w32(rdi.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // 324b7 mov DWORD PTR [rdi+0x1c],ecx
            m.w32(rdi.wrapping_add(0x1c), ((rcx as u32 as u64)) as u32);
            // 324ba mov r8d,DWORD PTR [rdi]
            r8 = (m.r32(rdi) as u64) as u32 as u64;
            // 324bd cmp r10d,0x1
            fl = sub::<32>((r10 as u32 as u64), 0x1u64).1;
            // 324c1 ja 0x3253a
            if fl.a() { pc = 0x3253a; continue; }
            pc = 0x324c3;
        }
        0x324c3 => {
            // block 324c3
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 324c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 324c3 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 324c6 jbe 0x324d0
            if fl.be() { pc = 0x324d0; continue; }
            pc = 0x324c8;
        }
        0x324c8 => {
            // block 324c8
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 324c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 324c8 mov DWORD PTR [rdi],r8d
            m.w32(rdi, ((r8 as u32 as u64)) as u32);
            // 324cb mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // 324ce jmp 0x3252f
            pc = 0x3252f; continue;
        }
        0x324d0 => {
            // block 324d0
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 324d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 324d0 lea rax,[rip+0x81] # 0x32558
            rax = m.env.code_va(0x32558);
            // 324d7 movsxd rsi,DWORD PTR [rax+r9*4]
            rsi = sx::<32>(m.r32(rax.wrapping_add(r9.wrapping_mul(4))) as u64) as u64;
            // 324db add rsi,rax
            { let (r, f) = add::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // 324de jmp rsi
            pc = (rsi.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x324e0 => {
            // block 324e0
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 324e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 324e0 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 324e3 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 324e6 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 324e9 lea eax,[rax+rax*1+0x458]
            rax = (rax.wrapping_add(rax).wrapping_add(0x458)) as u32 as u64;
            // 324f0 mov DWORD PTR [rdi],eax
            m.w32(rdi, ((rax as u32 as u64)) as u32);
            // 324f2 jmp 0x3252f
            pc = 0x3252f; continue;
        }
        0x324f4 => {
            // block 324f4
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 324f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 324f4 mov DWORD PTR [rdi],0x2e4
            m.w32(rdi, (0x2e4u64) as u32);
            // 324fa mov eax,0x2e4
            rax = (0x2e4u64) as u32 as u64;
            // 324ff jmp 0x3252f
            pc = 0x3252f; continue;
        }
        0x32501 => {
            // block 32501
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32501 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32501 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 32504 mov ecx,0x202
            rcx = (0x202u64) as u32 as u64;
            // 32509 mov eax,0x1fc
            rax = (0x1fcu64) as u32 as u64;
            // 3250e jmp 0x3251d
            pc = 0x3251d; continue;
        }
        0x32510 => {
            // block 32510
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32510 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32510 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 32513 mov ecx,0x264
            rcx = (0x264u64) as u32 as u64;
            // 32518 mov eax,0x256
            rax = (0x256u64) as u32 as u64;
            pc = 0x3251d;
        }
        0x3251d => {
            // block 3251d
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3251d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3251d cmove eax,ecx
            if fl.e() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 32520 mov DWORD PTR [rdi],eax
            m.w32(rdi, ((rax as u32 as u64)) as u32);
            // 32522 jmp 0x3252f
            pc = 0x3252f; continue;
        }
        0x32524 => {
            // block 32524
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32524 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32524 mov DWORD PTR [rdi],0x634
            m.w32(rdi, (0x634u64) as u32);
            // 3252a mov eax,0x634
            rax = (0x634u64) as u32 as u64;
            pc = 0x3252f;
        }
        0x3252f => {
            // block 3252f
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3252f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3252f mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 32531 sub ecx,r8d
            { let (r, f) = sub::<32>((rcx as u32 as u64), (r8 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // 32534 mov DWORD PTR [rdi+0xc],ecx
            m.w32(rdi.wrapping_add(0xc), ((rcx as u32 as u64)) as u32);
            // 32537 mov r8d,eax
            r8 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x3253a;
        }
        0x3253a => {
            // block 3253a
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3253a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3253a test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // 3253d je 0x32555
            if fl.e() { pc = 0x32555; continue; }
            pc = 0x3253f;
        }
        0x3253f => {
            // block 3253f
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3253f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3253f cmp edx,0x1f
            fl = sub::<32>((rdx as u32 as u64), 0x1fu64).1;
            // 32542 ja 0x32555
            if fl.a() { pc = 0x32555; continue; }
            pc = 0x32544;
        }
        0x32544 => {
            // block 32544
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32544 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32544 mov eax,0xfc800400
            rax = (0xfc800400u64) as u32 as u64;
            // 32549 bt eax,edx
            fl.cf = bit::<32>((rax as u32 as u64), (rdx as u32 as u64));
            // 3254c jae 0x32555
            if fl.ae() { pc = 0x32555; continue; }
            pc = 0x3254e;
        }
        0x3254e => {
            // block 3254e
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3254e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3254e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3254f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32550 jmp 0x325ca
            { let r = { let t = m.env.code_va(0x325ca); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x32555 => {
            // block 32555
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32555 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32555 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32556 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32557 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc62d4 size=435
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c62d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut x10: u128 = 0;
    let mut x11: u128 = 0;
    let mut x12: u128 = 0;
    let mut x13: u128 = 0;
    let mut x14: u128 = 0;
    let mut x15: u128 = 0;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc62d4;
    loop {
        match pc {
        0xc62d4 => {
            // block c62d4
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c62d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c62d5 mov rbp,rsp
            rbp = rsp;
            // c62d8 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c62da jle 0xc6485
            if fl.le() { pc = 0xc6485; continue; }
            pc = 0xc62e0;
        }
        0xc62e0 => {
            // block c62e0
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c62e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62e0 add ecx,ecx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rcx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c62e2 and ecx,0xfffffffc
            { let r = (rcx as u32 as u64) & 0xfffffffcu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c62e5 movsxd rax,ecx
            rax = sx::<32>((rcx as u32 as u64)) as u64;
            // c62e8 lea r10,[rsi+rax*2]
            r10 = rsi.wrapping_add(rax.wrapping_mul(2));
            // c62ec mov r9d,edx
            r9 = ((rdx as u32 as u64)) as u32 as u64;
            // c62ef xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // c62f1 movsd xmm10,QWORD PTR [rip+0x3caf6] # 0x102df0
            x10 = m.r64(m.env.data_va(95677)) as u128;
            // c62fa movapd xmm11,XMMWORD PTR [rip+0x3cb2d] # 0x102e30
            x11 = m.r128(m.env.data_va(95741));
            // c6303 movapd xmm12,XMMWORD PTR [rip+0x3cb34] # 0x102e40
            x12 = m.r128(m.env.data_va(95757));
            // c630c movapd xmm13,XMMWORD PTR [rip+0x3cb3b] # 0x102e50
            x13 = m.r128(m.env.data_va(95773));
            // c6315 movsd xmm14,QWORD PTR [rip+0x3caaa] # 0x102dc8
            x14 = m.r64(m.env.data_va(95637)) as u128;
            // c631e movsd xmm15,QWORD PTR [rip+0x3caa9] # 0x102dd0
            x15 = m.r64(m.env.data_va(95645)) as u128;
            // c6327 movsd xmm9,QWORD PTR [rip+0x3caa8] # 0x102dd8
            x9 = m.r64(m.env.data_va(95653)) as u128;
            // c6330 movapd xmm1,XMMWORD PTR [rip+0x3cac8] # 0x102e00
            x1 = m.r128(m.env.data_va(95693));
            // c6338 movapd xmm2,XMMWORD PTR [rip+0x3cad0] # 0x102e10
            x2 = m.r128(m.env.data_va(95709));
            // c6340 movapd xmm3,XMMWORD PTR [rip+0x3cad8] # 0x102e20
            x3 = m.r128(m.env.data_va(95725));
            pc = 0xc6348;
        }
        0xc6348 => {
            // block c6348
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6348 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6348 movzx ecx,WORD PTR [rdi+rdx*8+0x4]
            rcx = (m.r16(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x4)) as u64) as u32 as u64;
            // c634d xorps xmm4,xmm4
            x4 = 0;
            // c6350 cvtsi2sd xmm4,ecx
            x4 = set_lo64(x4, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c6354 movzx ecx,WORD PTR [rdi+rdx*8+0x2]
            rcx = (m.r16(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x2)) as u64) as u32 as u64;
            // c6359 movzx esi,WORD PTR [rdi+rdx*8]
            rsi = (m.r16(rdi.wrapping_add(rdx.wrapping_mul(8))) as u64) as u32 as u64;
            // c635d movapd xmm6,xmm4
            x6 = x4;
            // c6361 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // c6364 je 0xc63d0
            if fl.e() { pc = 0xc63d0; continue; }
            pc = 0xc6366;
        }
        0xc6366 => {
            // block c6366
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6366 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6366 mulsd xmm6,QWORD PTR [rip+0x3ca72] # 0x102de0
            x6 = set_lo64(x6, (f64_of(x6) * f64_of((m.r64(m.env.data_va(95661)) as u128))).to_bits());
            // c636e movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // c6371 cvtsi2sd xmm7,ecx
            x7 = set_lo64(x7, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c6375 movapd xmm5,xmm7
            x5 = x7;
            // c6379 unpcklpd xmm5,xmm4
            x5 = (x5 & u64::MAX as u128) | ((x4 & u64::MAX as u128) << 64);
            // c637d unpcklpd xmm4,xmm7
            x4 = (x4 & u64::MAX as u128) | ((x7 & u64::MAX as u128) << 64);
            // c6381 mulsd xmm7,QWORD PTR [rip+0x3ca5f] # 0x102de8
            x7 = set_lo64(x7, (f64_of(x7) * f64_of((m.r64(m.env.data_va(95669)) as u128))).to_bits());
            // c6389 addsd xmm7,xmm6
            x7 = set_lo64(x7, (f64_of(x7) + f64_of(x6)).to_bits());
            // c638d movzx ecx,si
            rcx = ((rsi as u16 as u64)) as u32 as u64;
            // c6390 xorps xmm6,xmm6
            x6 = 0;
            // c6393 cvtsi2sd xmm6,ecx
            x6 = set_lo64(x6, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c6397 movddup xmm8,xmm6
            { let v = (x6) as u64; x8 = (v as u128) | ((v as u128) << 64); }
            // c639c mulsd xmm6,xmm10
            x6 = set_lo64(x6, (f64_of(x6) * f64_of(x10)).to_bits());
            // c63a1 addsd xmm6,xmm7
            x6 = set_lo64(x6, (f64_of(x6) + f64_of(x7)).to_bits());
            // c63a5 cvttsd2si rsi,xmm6
            rsi = cvtt64(f64_of(x6)) as u64;
            // c63aa add esi,0x40
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x40u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // c63ad mulpd xmm5,xmm11
            { let a = x5; let b = x11; x5 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63b2 mulpd xmm4,xmm12
            { let a = x4; let b = x12; x4 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63b7 subpd xmm5,xmm4
            { let a = x5; let b = x4; x5 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63bb mulpd xmm8,xmm13
            { let a = x8; let b = x13; x8 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63c0 movapd xmm4,xmm5
            x4 = x5;
            // c63c4 addpd xmm4,xmm8
            { let a = x4; let b = x8; x4 = ((f64::from_bits(a as u64) + f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) + f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63c9 subpd xmm5,xmm8
            { let a = x5; let b = x8; x5 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63ce jmp 0xc6429
            pc = 0xc6429; continue;
        }
        0xc63d0 => {
            // block c63d0
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c63d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c63d0 mulsd xmm6,xmm14
            x6 = set_lo64(x6, (f64_of(x6) * f64_of(x14)).to_bits());
            // c63d5 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // c63d8 cvtsi2sd xmm7,ecx
            x7 = set_lo64(x7, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c63dc movapd xmm5,xmm7
            x5 = x7;
            // c63e0 unpcklpd xmm5,xmm4
            x5 = (x5 & u64::MAX as u128) | ((x4 & u64::MAX as u128) << 64);
            // c63e4 unpcklpd xmm4,xmm7
            x4 = (x4 & u64::MAX as u128) | ((x7 & u64::MAX as u128) << 64);
            // c63e8 mulsd xmm7,xmm15
            x7 = set_lo64(x7, (f64_of(x7) * f64_of(x15)).to_bits());
            // c63ed addsd xmm7,xmm6
            x7 = set_lo64(x7, (f64_of(x7) + f64_of(x6)).to_bits());
            // c63f1 movzx ecx,si
            rcx = ((rsi as u16 as u64)) as u32 as u64;
            // c63f4 xorps xmm6,xmm6
            x6 = 0;
            // c63f7 cvtsi2sd xmm6,ecx
            x6 = set_lo64(x6, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c63fb movddup xmm0,xmm6
            { let v = (x6) as u64; x0 = (v as u128) | ((v as u128) << 64); }
            // c63ff mulsd xmm6,xmm9
            x6 = set_lo64(x6, (f64_of(x6) * f64_of(x9)).to_bits());
            // c6404 addsd xmm6,xmm7
            x6 = set_lo64(x6, (f64_of(x6) + f64_of(x7)).to_bits());
            // c6408 cvttsd2si rsi,xmm6
            rsi = cvtt64(f64_of(x6)) as u64;
            // c640d mulpd xmm5,xmm1
            { let a = x5; let b = x1; x5 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c6411 mulpd xmm4,xmm2
            { let a = x4; let b = x2; x4 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c6415 subpd xmm5,xmm4
            { let a = x5; let b = x4; x5 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c6419 mulpd xmm0,xmm3
            { let a = x0; let b = x3; x0 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c641d movapd xmm4,xmm5
            x4 = x5;
            // c6421 addpd xmm4,xmm0
            { let a = x4; let b = x0; x4 = ((f64::from_bits(a as u64) + f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) + f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c6425 subpd xmm5,xmm0
            { let a = x5; let b = x0; x5 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            pc = 0xc6429;
        }
        0xc6429 => {
            // block c6429
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6429 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6429 movsd xmm5,xmm4
            x5 = set_lo64(x5, x4 as u64);
            // c642d and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // c6433 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // c6436 jne 0xc6471
            if fl.ne() { pc = 0xc6471; continue; }
            pc = 0xc6438;
        }
        0xc6438 => {
            // block c6438
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6438 cvttsd2si rcx,xmm5
            rcx = cvtt64(f64_of(x5)) as u64;
            // c643d add ecx,0x200
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x200u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c6443 and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c6449 shufpd xmm5,xmm5,0x1
            { let a = x5; let b = x5; x5 = (((a >> 64)) & u64::MAX as u128) | (((b) & u64::MAX as u128) << 64); }
            // c644e cvttsd2si rax,xmm5
            rax = cvtt64(f64_of(x5)) as u64;
            // c6453 add eax,0x200
            { let (r, f) = add::<32>((rax as u32 as u64), 0x200u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6458 and eax,0x3ff
            { let r = (rax as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c645d mov WORD PTR [r10],cx
            m.w16(r10, ((rcx as u16 as u64)) as u16);
            // c6461 mov WORD PTR [r10+0x2],si
            m.w16(r10.wrapping_add(0x2), ((rsi as u16 as u64)) as u16);
            // c6466 mov WORD PTR [r10+0x4],ax
            m.w16(r10.wrapping_add(0x4), ((rax as u16 as u64)) as u16);
            // c646b add r10,0x6
            { let (r, f) = add::<64>(r10, 0x6u64);
              r10 = r;
              fl = f; }
            // c646f jmp 0xc6479
            pc = 0xc6479; continue;
        }
        0xc6471 => {
            // block c6471
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6471 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6471 mov WORD PTR [r10],si
            m.w16(r10, ((rsi as u16 as u64)) as u16);
            // c6475 add r10,0x2
            { let (r, f) = add::<64>(r10, 0x2u64);
              r10 = r;
              fl = f; }
            pc = 0xc6479;
        }
        0xc6479 => {
            // block c6479
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6479 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6479 inc rdx
            { let (r, f) = inc::<64>(rdx, fl); rdx = r; fl = f; }
            // c647c cmp r9d,edx
            fl = sub::<32>((r9 as u32 as u64), (rdx as u32 as u64)).1;
            // c647f jne 0xc6348
            if fl.ne() { pc = 0xc6348; continue; }
            pc = 0xc6485;
        }
        0xc6485 => {
            // block c6485
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6485 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6485 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c6486 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2f626 size=428
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002f626<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2f626;
    loop {
        match pc {
        0x2f626 => {
            // block 2f626
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f626 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f626 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2f627 mov rbp,rsp
            rbp = rsp;
            // 2f62a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2f62c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2f62d sub rsp,0x90
            { let (r, f) = sub::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // 2f634 mov r14,rsi
            r14 = rsi;
            // 2f637 mov rbx,rdi
            rbx = rdi;
            // 2f63a xorps xmm0,xmm0
            x0 = 0;
            // 2f63d movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // 2f641 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // 2f649 mov BYTE PTR [rbp-0x31],0x5
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf), (0x5u64) as u8);
            // 2f64d lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2f651 lea rsi,[rbp-0x31]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcf);
            // 2f655 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f65a xorps xmm0,xmm0
            x0 = 0;
            // 2f65d movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 2f661 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 2f669 movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // 2f66d mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 2f675 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 2f67c lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2f680 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f685 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 2f68c lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2f690 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f695 lea rsi,[rbp-0x88]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 2f69c lea rdx,[rbp-0xa0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 2f6a3 lea rcx,[rbp-0x70]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 2f6a7 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 2f6ad mov rdi,rbx
            rdi = rbx;
            // 2f6b0 call 0x2fdd8
            { let r = { let t = m.env.code_va(0x2fdd8); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f6b5 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2f6b7 mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 2f6be test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f6c1 je 0x2f6d8
            if fl.e() { pc = 0x2f6d8; continue; }
            pc = 0x2f6c3;
        }
        0x2f6c3 => {
            // block 2f6c3
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6c3 cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 2f6ca je 0x2f6d3
            if fl.e() { pc = 0x2f6d3; continue; }
            pc = 0x2f6cc;
        }
        0x2f6cc => {
            // block 2f6cc
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6cc mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x2f6d3;
        }
        0x2f6d3 => {
            // block 2f6d3
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6d3 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f6d8;
        }
        0x2f6d8 => {
            // block 2f6d8
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6d8 mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 2f6df test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f6e2 je 0x2f6f3
            if fl.e() { pc = 0x2f6f3; continue; }
            pc = 0x2f6e4;
        }
        0x2f6e4 => {
            // block 2f6e4
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6e4 cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 2f6e8 je 0x2f6ee
            if fl.e() { pc = 0x2f6ee; continue; }
            pc = 0x2f6ea;
        }
        0x2f6ea => {
            // block 2f6ea
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6ea mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x2f6ee;
        }
        0x2f6ee => {
            // block 2f6ee
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6ee call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f6f3;
        }
        0x2f6f3 => {
            // block 2f6f3
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6f3 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // 2f6f5 je 0x2f70b
            if fl.e() { pc = 0x2f70b; continue; }
            pc = 0x2f6f7;
        }
        0x2f6f7 => {
            // block 2f6f7
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6f7 mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 2f6fb mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 2f6ff cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 2f702 je 0x2f70f
            if fl.e() { pc = 0x2f70f; continue; }
            pc = 0x2f704;
        }
        0x2f704 => {
            // block 2f704
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f704 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f704 mov al,BYTE PTR [rdi]
            rax = (rax & !0xff) | ((m.r8(rdi) as u64) as u8 as u64);
            // 2f706 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // 2f709 jmp 0x2f717
            pc = 0x2f717; continue;
        }
        0x2f70b => {
            // block 2f70b
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f70b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f70b mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x2f70f;
        }
        0x2f70f => {
            // block 2f70f
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f70f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f70f test rax,rax
            fl = logic::<64>(rax & rax);
            // 2f712 mov rdi,rax
            rdi = rax;
            // 2f715 je 0x2f726
            if fl.e() { pc = 0x2f726; continue; }
            pc = 0x2f717;
        }
        0x2f717 => {
            // block 2f717
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f717 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f717 cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 2f71b je 0x2f721
            if fl.e() { pc = 0x2f721; continue; }
            pc = 0x2f71d;
        }
        0x2f71d => {
            // block 2f71d
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f71d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f71d mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x2f721;
        }
        0x2f721 => {
            // block 2f721
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f721 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f721 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f726;
        }
        0x2f726 => {
            // block 2f726
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f726 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f726 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 2f72a test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f72d je 0x2f73e
            if fl.e() { pc = 0x2f73e; continue; }
            pc = 0x2f72f;
        }
        0x2f72f => {
            // block 2f72f
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f72f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f72f cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 2f733 je 0x2f739
            if fl.e() { pc = 0x2f739; continue; }
            pc = 0x2f735;
        }
        0x2f735 => {
            // block 2f735
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f735 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f735 mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x2f739;
        }
        0x2f739 => {
            // block 2f739
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f739 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f739 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f73e;
        }
        0x2f73e => {
            // block 2f73e
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f73e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f73e mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 2f740 add rsp,0x90
            { let (r, f) = add::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // 2f747 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f748 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f74a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f74b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2f74c => {
            // block 2f74c
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f74c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f74c mov rbx,rax
            rbx = rax;
            // 2f74f jmp 0x2f7b2
            pc = 0x2f7b2; continue;
        }
        0x2f751 => {
            // block 2f751
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f751 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f751 mov rbx,rax
            rbx = rax;
            // 2f754 jmp 0x2f79a
            pc = 0x2f79a; continue;
        }
        0x2f756 => {
            // block 2f756
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f756 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f756 mov rbx,rax
            rbx = rax;
            // 2f759 jmp 0x2f77f
            pc = 0x2f77f; continue;
        }
        0x2f75b => {
            // block 2f75b
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f75b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f75b mov rbx,rax
            rbx = rax;
            // 2f75e mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 2f765 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f768 je 0x2f77f
            if fl.e() { pc = 0x2f77f; continue; }
            pc = 0x2f76a;
        }
        0x2f76a => {
            // block 2f76a
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f76a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f76a cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 2f771 je 0x2f77a
            if fl.e() { pc = 0x2f77a; continue; }
            pc = 0x2f773;
        }
        0x2f773 => {
            // block 2f773
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f773 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f773 mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x2f77a;
        }
        0x2f77a => {
            // block 2f77a
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f77a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f77a call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f77f;
        }
        0x2f77f => {
            // block 2f77f
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f77f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f77f mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 2f786 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f789 je 0x2f79a
            if fl.e() { pc = 0x2f79a; continue; }
            pc = 0x2f78b;
        }
        0x2f78b => {
            // block 2f78b
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f78b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f78b cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 2f78f je 0x2f795
            if fl.e() { pc = 0x2f795; continue; }
            pc = 0x2f791;
        }
        0x2f791 => {
            // block 2f791
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f791 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f791 mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x2f795;
        }
        0x2f795 => {
            // block 2f795
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f795 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f795 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f79a;
        }
        0x2f79a => {
            // block 2f79a
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f79a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f79a mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 2f79e test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f7a1 je 0x2f7b2
            if fl.e() { pc = 0x2f7b2; continue; }
            pc = 0x2f7a3;
        }
        0x2f7a3 => {
            // block 2f7a3
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7a3 cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 2f7a7 je 0x2f7ad
            if fl.e() { pc = 0x2f7ad; continue; }
            pc = 0x2f7a9;
        }
        0x2f7a9 => {
            // block 2f7a9
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7a9 mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x2f7ad;
        }
        0x2f7ad => {
            // block 2f7ad
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7ad call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f7b2;
        }
        0x2f7b2 => {
            // block 2f7b2
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7b2 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 2f7b6 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f7b9 je 0x2f7ca
            if fl.e() { pc = 0x2f7ca; continue; }
            pc = 0x2f7bb;
        }
        0x2f7bb => {
            // block 2f7bb
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7bb cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 2f7bf je 0x2f7c5
            if fl.e() { pc = 0x2f7c5; continue; }
            pc = 0x2f7c1;
        }
        0x2f7c1 => {
            // block 2f7c1
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7c1 mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x2f7c5;
        }
        0x2f7c5 => {
            // block 2f7c5
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7c5 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f7ca;
        }
        0x2f7ca => {
            // block 2f7ca
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7ca mov rdi,rbx
            rdi = rbx;
            // 2f7cd call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x17062 size=416
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00017062<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x17062;
    loop {
        match pc {
        0x17062 => {
            // block 17062
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 17062 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17062 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 17063 mov rbp,rsp
            rbp = rsp;
            // 17066 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 17068 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1706a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1706c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1706e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1706f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 17070 mov r15,rdx
            r15 = rdx;
            // 17073 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 17076 mov rbx,rdi
            rbx = rdi;
            // 17079 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 1707b je 0x17086
            if fl.e() { pc = 0x17086; continue; }
            pc = 0x1707d;
        }
        0x1707d => {
            // block 1707d
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 1707d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1707d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17080 mov rdi,rbx
            rdi = rbx;
            // 17083 call QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x17086;
        }
        0x17086 => {
            // block 17086
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 17086 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17086 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17089 mov rdi,rbx
            rdi = rbx;
            // 1708c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1708f mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 17092 mov rbx,QWORD PTR [rax]
            rbx = m.r64(rax) as u64;
            // 17095 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 17099 sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // 1709c sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 170a0 movabs rdx,0xef7bdef7bdef7bdf
            rdx = 0xef7b_def7_bdef_7bdfu64;
            // 170aa imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // 170ae cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 170b1 jae 0x171f1
            if fl.ae() { pc = 0x171f1; continue; }
            pc = 0x170b7;
        }
        0x170b7 => {
            // block 170b7
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 170b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 170b7 imul r12,rcx,0xf8
            { let (r, f) = imul::<64>(rcx, 0xf8u64); r12 = r; fl = f; }
            // 170be lea r13,[rbx+r12*1]
            r13 = rbx.wrapping_add(r12);
            // 170c2 mov rax,QWORD PTR [rbx+r12*1+0x10]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x10)) as u64;
            // 170c7 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 170cb mov rax,QWORD PTR [rbx+r12*1]
            rax = m.r64(rbx.wrapping_add(r12)) as u64;
            // 170cf mov rcx,QWORD PTR [rbx+r12*1+0x8]
            rcx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x8)) as u64;
            // 170d4 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 170d8 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 170db lea rdi,[r15+0x18]
            rdi = r15.wrapping_add(0x18);
            // 170df lea rsi,[rbx+r12*1+0x18]
            rsi = rbx.wrapping_add(r12).wrapping_add(0x18);
            // 170e4 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 170e9 mov ax,WORD PTR [rbx+r12*1+0x5c]
            rax = (rax & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x5c)) as u64) as u16 as u64);
            // 170ef mov WORD PTR [r15+0x5c],ax
            m.w16(r15.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 170f4 mov eax,DWORD PTR [rbx+r12*1+0x58]
            rax = (m.r32(rbx.wrapping_add(r12).wrapping_add(0x58)) as u64) as u32 as u64;
            // 170f9 mov DWORD PTR [r15+0x58],eax
            m.w32(r15.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 170fd mov rax,QWORD PTR [rbx+r12*1+0x50]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x50)) as u64;
            // 17102 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // 17106 mov rax,QWORD PTR [rbx+r12*1+0x48]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x48)) as u64;
            // 1710b mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // 1710f mov rax,QWORD PTR [rbx+r12*1+0x40]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x40)) as u64;
            // 17114 mov QWORD PTR [r15+0x40],rax
            m.w64(r15.wrapping_add(0x40), (rax) as u64);
            // 17118 mov rax,QWORD PTR [rbx+r12*1+0x30]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x30)) as u64;
            // 1711d mov rcx,QWORD PTR [rbx+r12*1+0x38]
            rcx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x38)) as u64;
            // 17122 mov QWORD PTR [r15+0x38],rcx
            m.w64(r15.wrapping_add(0x38), (rcx) as u64);
            // 17126 mov QWORD PTR [r15+0x30],rax
            m.w64(r15.wrapping_add(0x30), (rax) as u64);
            // 1712a cmp r13,r15
            fl = sub::<64>(r13, r15).1;
            // 1712d je 0x171ac
            if fl.e() { pc = 0x171ac; continue; }
            pc = 0x1712f;
        }
        0x1712f => {
            // block 1712f
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 1712f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1712f lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 17133 mov rsi,QWORD PTR [rbx+r12*1+0x60]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0x60)) as u64;
            // 17138 mov rdx,QWORD PTR [rbx+r12*1+0x68]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x68)) as u64;
            // 1713d call 0xeba1a
            { let r = { let t = m.env.stub_va(0x1e7); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17142 lea rdi,[r15+0x78]
            rdi = r15.wrapping_add(0x78);
            // 17146 mov rsi,QWORD PTR [rbx+r12*1+0x78]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0x78)) as u64;
            // 1714b mov rdx,QWORD PTR [rbx+r12*1+0x80]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x80)) as u64;
            // 17153 call 0xeba62
            { let r = { let t = m.env.stub_va(0x22f); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17158 lea rdi,[r15+0x90]
            rdi = r15.wrapping_add(0x90);
            // 1715f mov rsi,QWORD PTR [rbx+r12*1+0x90]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0x90)) as u64;
            // 17167 mov rdx,QWORD PTR [rbx+r12*1+0x98]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x98)) as u64;
            // 1716f call 0xeba3e
            { let r = { let t = m.env.stub_va(0x20b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17174 lea rdi,[r15+0xa8]
            rdi = r15.wrapping_add(0xa8);
            // 1717b mov rsi,QWORD PTR [rbx+r12*1+0xa8]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0xa8)) as u64;
            // 17183 mov rdx,QWORD PTR [rbx+r12*1+0xb0]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0xb0)) as u64;
            // 1718b call 0xeb9f6
            { let r = { let t = m.env.stub_va(0x1c3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17190 lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 17197 mov rsi,QWORD PTR [rbx+r12*1+0xc0]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0xc0)) as u64;
            // 1719f mov rdx,QWORD PTR [rbx+r12*1+0xc8]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0xc8)) as u64;
            // 171a7 call 0xeb9f6
            { let r = { let t = m.env.stub_va(0x1c3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x171ac;
        }
        0x171ac => {
            // block 171ac
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 171ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 171ac mov eax,DWORD PTR [rbx+r12*1+0xf0]
            rax = (m.r32(rbx.wrapping_add(r12).wrapping_add(0xf0)) as u64) as u32 as u64;
            // 171b4 mov DWORD PTR [r15+0xf0],eax
            m.w32(r15.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 171bb mov rax,QWORD PTR [rbx+r12*1+0xe8]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0xe8)) as u64;
            // 171c3 mov QWORD PTR [r15+0xe8],rax
            m.w64(r15.wrapping_add(0xe8), (rax) as u64);
            // 171ca mov rax,QWORD PTR [rbx+r12*1+0xd8]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0xd8)) as u64;
            // 171d2 mov rcx,QWORD PTR [rbx+r12*1+0xe0]
            rcx = m.r64(rbx.wrapping_add(r12).wrapping_add(0xe0)) as u64;
            // 171da mov QWORD PTR [r15+0xe0],rcx
            m.w64(r15.wrapping_add(0xe0), (rcx) as u64);
            // 171e1 mov QWORD PTR [r15+0xd8],rax
            m.w64(r15.wrapping_add(0xd8), (rax) as u64);
            // 171e8 cmp DWORD PTR [r15+0x4],r14d
            fl = sub::<32>(m.r32(r15.wrapping_add(0x4)) as u64, (r14 as u32 as u64)).1;
            // 171ec sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 171ef jmp 0x171f3
            pc = 0x171f3; continue;
        }
        0x171f1 => {
            // block 171f1
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 171f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 171f1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x171f3;
        }
        0x171f3 => {
            // block 171f3
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 171f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 171f3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 171f7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 171f8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 171fa pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 171fc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 171fe pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 17200 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 17201 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x332f0 size=410
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000332f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x332f0;
    loop {
        match pc {
        0x332f0 => {
            // block 332f0
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 332f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 332f1 mov rbp,rsp
            rbp = rsp;
            // 332f4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 332f6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 332f8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 332fa push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 332fc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 332fd sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 33301 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 33305 mov r9,rdx
            r9 = rdx;
            // 33308 mov QWORD PTR [rbp-0x48],r9
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r9) as u64);
            // 3330c mov r13,rdi
            r13 = rdi;
            // 3330f mov rcx,QWORD PTR [rsi]
            rcx = m.r64(rsi) as u64;
            // 33312 mov rbx,QWORD PTR [rsi+0x8]
            rbx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // 33316 cmp rbx,rcx
            fl = sub::<64>(rbx, rcx).1;
            // 33319 je 0x33330
            if fl.e() { pc = 0x33330; continue; }
            pc = 0x3331b;
        }
        0x3331b => {
            // block 3331b
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3331b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3331b lea rdx,[rbx-0x4]
            rdx = rbx.wrapping_add(0xffff_ffff_ffff_fffc);
            // 3331f sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 33322 not rdx
            rdx = !rdx;
            // 33325 and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 33329 add rdx,rbx
            { let (r, f) = add::<64>(rdx, rbx);
              rdx = r;
              fl = f; }
            // 3332c mov QWORD PTR [rsi+0x8],rdx
            m.w64(rsi.wrapping_add(0x8), (rdx) as u64);
            pc = 0x33330;
        }
        0x33330 => {
            // block 33330
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33330 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33330 mov edx,DWORD PTR [r13+0x0]
            rdx = (m.r32(r13) as u64) as u32 as u64;
            // 33334 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 33336 je 0x33478
            if fl.e() { pc = 0x33478; continue; }
            pc = 0x3333c;
        }
        0x3333c => {
            // block 3333c
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3333c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3333c cmp DWORD PTR [r13+0x4],0x0
            fl = sub::<32>(m.r32(r13.wrapping_add(0x4)) as u64, 0x0u64).1;
            // 33341 je 0x33478
            if fl.e() { pc = 0x33478; continue; }
            pc = 0x33347;
        }
        0x33347 => {
            // block 33347
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33347 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33347 mov r14w,WORD PTR [r13+0x34]
            r14 = (r14 & !0xffff) | ((m.r16(r13.wrapping_add(0x34)) as u64) as u16 as u64);
            // 3334c test r14w,r14w
            fl = logic::<16>((r14 as u16 as u64) & (r14 as u16 as u64));
            // 33350 je 0x33478
            if fl.e() { pc = 0x33478; continue; }
            pc = 0x33356;
        }
        0x33356 => {
            // block 33356
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33356 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33356 mov ecx,DWORD PTR [r13+0x24]
            rcx = (m.r32(r13.wrapping_add(0x24)) as u64) as u32 as u64;
            // 3335a test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 3335c je 0x33478
            if fl.e() { pc = 0x33478; continue; }
            pc = 0x33362;
        }
        0x33362 => {
            // block 33362
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33362 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33362 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33364 test r9,r9
            fl = logic::<64>(r9 & r9);
            // 33367 je 0x3347a
            if fl.e() { pc = 0x3347a; continue; }
            pc = 0x3336d;
        }
        0x3336d => {
            // block 3336d
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3336d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3336d cmp QWORD PTR [rbp-0x40],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64, 0x0u64).1;
            // 33372 je 0x3347a
            if fl.e() { pc = 0x3347a; continue; }
            pc = 0x33378;
        }
        0x33378 => {
            // block 33378
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33378 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33378 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 3337a add ebx,0xffffffff
            { let (r, f) = add::<32>((rbx as u32 as u64), 0xffffffffu64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 3337d jae 0x33478
            if fl.ae() { pc = 0x33478; continue; }
            pc = 0x33383;
        }
        0x33383 => {
            // block 33383
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33383 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33383 imul ecx,ebx
            { let (r, f) = imul::<32>((rcx as u32 as u64), (rbx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // 33386 add rcx,r9
            { let (r, f) = add::<64>(rcx, r9);
              rcx = r;
              fl = f; }
            // 33389 je 0x33478
            if fl.e() { pc = 0x33478; continue; }
            pc = 0x3338f;
        }
        0x3338f => {
            // block 3338f
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3338f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3338f mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 33391 cmp r9,QWORD PTR [rbp-0x40]
            fl = sub::<64>(r9, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64).1;
            // 33395 je 0x3347a
            if fl.e() { pc = 0x3347a; continue; }
            pc = 0x3339b;
        }
        0x3339b => {
            // block 3339b
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3339b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3339b mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 3339f test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // 333a2 cmove r8d,edx
            if fl.e() { r8 = ((rdx as u32 as u64)) as u32 as u64; } else { r8 = r8 as u32 as u64; }
            // 333a6 cmp r8d,edx
            fl = sub::<32>((r8 as u32 as u64), (rdx as u32 as u64)).1;
            // 333a9 cmova r8d,edx
            if fl.a() { r8 = ((rdx as u32 as u64)) as u32 as u64; } else { r8 = r8 as u32 as u64; }
            // 333ad mov DWORD PTR [rbp-0x4c],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), ((r8 as u32 as u64)) as u32);
            // 333b1 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x333b3;
        }
        0x333b3 => {
            // block 333b3
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 333b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 333b3 cmp DWORD PTR [rbp-0x4c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64, 0x0u64).1;
            // 333b7 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 333be je 0x3345d
            if fl.e() { pc = 0x3345d; continue; }
            pc = 0x333c4;
        }
        0x333c4 => {
            // block 333c4
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 333c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 333c4 movzx r12d,si
            r12 = ((rsi as u16 as u64)) as u32 as u64;
            // 333c8 mov DWORD PTR [rbp-0x5c],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rsi as u32 as u64)) as u32);
            // 333cb mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            pc = 0x333d0;
        }
        0x333d0 => {
            // block 333d0
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 333d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 333d0 lea r15d,[rbx-0x1]
            r15 = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 333d4 mov rdi,r13
            rdi = r13;
            // 333d7 mov rsi,r9
            rsi = r9;
            // 333da mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 333dd mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // 333e0 call 0x330d0
            { let r = { let t = m.env.code_va(0x330d0); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 333e5 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 333e9 mov rdi,r13
            rdi = r13;
            // 333ec mov rsi,QWORD PTR [rbp-0x40]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 333f0 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 333f3 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // 333f6 call 0x330d0
            { let r = { let t = m.env.code_va(0x330d0); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 333fb xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 333fd movzx edx,r14w
            rdx = ((r14 as u16 as u64)) as u32 as u64;
            // 33401 cmp edx,r12d
            fl = sub::<32>((rdx as u32 as u64), (r12 as u32 as u64)).1;
            // 33404 jbe 0x3340b
            if fl.be() { pc = 0x3340b; continue; }
            pc = 0x33406;
        }
        0x33406 => {
            // block 33406
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33406 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33406 mov ecx,DWORD PTR [r13+r12*4+0x24]
            rcx = (m.r32(r13.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x24)) as u64) as u32 as u64;
            pc = 0x3340b;
        }
        0x3340b => {
            // block 3340b
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3340b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3340b mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 3340d mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 33411 mov rsi,rax
            rsi = rax;
            // 33414 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33419 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 3341b je 0x33441
            if fl.e() { pc = 0x33441; continue; }
            pc = 0x3341d;
        }
        0x3341d => {
            // block 3341d
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3341d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3341d mov rdi,QWORD PTR [rbp-0x58]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 33421 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 33425 cmp rax,QWORD PTR [rdi+0x10]
            fl = sub::<64>(rax, m.r64(rdi.wrapping_add(0x10)) as u64).1;
            // 33429 je 0x33438
            if fl.e() { pc = 0x33438; continue; }
            pc = 0x3342b;
        }
        0x3342b => {
            // block 3342b
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3342b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3342b mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 3342e add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 33432 mov QWORD PTR [rdi+0x8],rax
            m.w64(rdi.wrapping_add(0x8), (rax) as u64);
            // 33436 jmp 0x33441
            pc = 0x33441; continue;
        }
        0x33438 => {
            // block 33438
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33438 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 3343c call 0xebb3a
            { let r = { let t = m.env.stub_va(0x307); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x33441;
        }
        0x33441 => {
            // block 33441
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33441 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33441 mov DWORD PTR [rbp-0x2c],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rbx as u32 as u64)) as u32);
            // 33444 movzx r14d,WORD PTR [r13+0x34]
            r14 = (m.r16(r13.wrapping_add(0x34)) as u64) as u32 as u64;
            // 33449 cmp ebx,DWORD PTR [rbp-0x4c]
            fl = sub::<32>((rbx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64).1;
            // 3344c lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 3344f mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 33451 mov r9,QWORD PTR [rbp-0x48]
            r9 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 33455 jb 0x333d0
            if fl.b() { pc = 0x333d0; continue; }
            pc = 0x3345b;
        }
        0x3345b => {
            // block 3345b
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3345b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3345b jmp 0x33460
            pc = 0x33460; continue;
        }
        0x3345d => {
            // block 3345d
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3345d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3345d mov DWORD PTR [rbp-0x5c],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rsi as u32 as u64)) as u32);
            pc = 0x33460;
        }
        0x33460 => {
            // block 33460
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33460 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33460 mov esi,DWORD PTR [rbp-0x5c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // 33463 inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 33465 movzx edx,si
            rdx = ((rsi as u16 as u64)) as u32 as u64;
            // 33468 movzx ecx,r14w
            rcx = ((r14 as u16 as u64)) as u32 as u64;
            // 3346c cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 3346e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 33470 jb 0x333b3
            if fl.b() { pc = 0x333b3; continue; }
            pc = 0x33476;
        }
        0x33476 => {
            // block 33476
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33476 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33476 jmp 0x3347a
            pc = 0x3347a; continue;
        }
        0x33478 => {
            // block 33478
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33478 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33478 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x3347a;
        }
        0x3347a => {
            // block 3347a
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3347a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3347a add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 3347e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3347f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 33481 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 33483 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 33485 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 33487 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 33488 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0eb0 size=404
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0eb0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe0eb0;
    loop {
        match pc {
        0xe0eb0 => {
            // block e0eb0
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0eb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0eb0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0eb1 mov rbp,rsp
            rbp = rsp;
            // e0eb4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e0eb6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0eb8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e0eba push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e0ebc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0ebd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0ebe mov DWORD PTR [rbp-0x30],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rdx as u32 as u64)) as u32);
            // e0ec1 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e0ec3 mov r14,rdi
            r14 = rdi;
            // e0ec6 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0ec9 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e0ecc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0ece call QWORD PTR [rax+0x3c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3c0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ed4 mov r12,rax
            r12 = rax;
            // e0ed7 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0eda mov rdi,r14
            rdi = r14;
            // e0edd call QWORD PTR [rax+0x1a90]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a90)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ee3 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e0ee6 jne 0xe0f0f
            if fl.ne() { pc = 0xe0f0f; continue; }
            pc = 0xe0ee8;
        }
        0xe0ee8 => {
            // block e0ee8
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0ee8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0ee8 shr r12,0x20
            { let (r, f) = shr::<64>(r12, 0x20u64, fl); r12 = r; fl = f; }
            // e0eec cmp r12d,0x2cf
            fl = sub::<32>((r12 as u32 as u64), 0x2cfu64).1;
            // e0ef3 jg 0xe0f46
            if fl.g() { pc = 0xe0f46; continue; }
            pc = 0xe0ef5;
        }
        0xe0ef5 => {
            // block e0ef5
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0ef5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0ef5 cmp r12d,0x1e6
            fl = sub::<32>((r12 as u32 as u64), 0x1e6u64).1;
            // e0efc jne 0xe0f6c
            if fl.ne() { pc = 0xe0f6c; continue; }
            pc = 0xe0efe;
        }
        0xe0efe => {
            // block e0efe
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0efe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0efe mov r15d,0x8
            r15 = (0x8u64) as u32 as u64;
            // e0f04 mov r13d,0xf4
            r13 = (0xf4u64) as u32 as u64;
            // e0f0a jmp 0xe0f95
            pc = 0xe0f95; continue;
        }
        0xe0f0f => {
            // block e0f0f
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f0f test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e0f11 jne 0xe1034
            if fl.ne() { pc = 0xe1034; continue; }
            pc = 0xe0f17;
        }
        0xe0f17 => {
            // block e0f17
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f17 cmp r12d,0x2d0
            fl = sub::<32>((r12 as u32 as u64), 0x2d0u64).1;
            // e0f1e je 0xe0f89
            if fl.e() { pc = 0xe0f89; continue; }
            pc = 0xe0f20;
        }
        0xe0f20 => {
            // block e0f20
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f20 cmp r12d,0x500
            fl = sub::<32>((r12 as u32 as u64), 0x500u64).1;
            // e0f27 je 0xe0f3b
            if fl.e() { pc = 0xe0f3b; continue; }
            pc = 0xe0f29;
        }
        0xe0f29 => {
            // block e0f29
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f29 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e0f2c cmp r12d,0x780
            fl = sub::<32>((r12 as u32 as u64), 0x780u64).1;
            // e0f33 mov r15d,0x0
            r15 = (0x0u64) as u32 as u64;
            // e0f39 jne 0xe0f95
            if fl.ne() { pc = 0xe0f95; continue; }
            pc = 0xe0f3b;
        }
        0xe0f3b => {
            // block e0f3b
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f3b mov r15d,0x8
            r15 = (0x8u64) as u32 as u64;
            // e0f41 mov r13d,r12d
            r13 = ((r12 as u32 as u64)) as u32 as u64;
            // e0f44 jmp 0xe0f95
            pc = 0xe0f95; continue;
        }
        0xe0f46 => {
            // block e0f46
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f46 cmp r12d,0x2d0
            fl = sub::<32>((r12 as u32 as u64), 0x2d0u64).1;
            // e0f4d je 0xe0f5e
            if fl.e() { pc = 0xe0f5e; continue; }
            pc = 0xe0f4f;
        }
        0xe0f4f => {
            // block e0f4f
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f4f cmp r12d,0x438
            fl = sub::<32>((r12 as u32 as u64), 0x438u64).1;
            // e0f56 mov r15d,0x0
            r15 = (0x0u64) as u32 as u64;
            // e0f5c jne 0xe0f95
            if fl.ne() { pc = 0xe0f95; continue; }
            pc = 0xe0f5e;
        }
        0xe0f5e => {
            // block e0f5e
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f5e inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // e0f61 mov r15d,0x7
            r15 = (0x7u64) as u32 as u64;
            // e0f67 mov r13d,r12d
            r13 = ((r12 as u32 as u64)) as u32 as u64;
            // e0f6a jmp 0xe0f95
            pc = 0xe0f95; continue;
        }
        0xe0f6c => {
            // block e0f6c
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f6c cmp r12d,0x240
            fl = sub::<32>((r12 as u32 as u64), 0x240u64).1;
            // e0f73 mov r15d,0x0
            r15 = (0x0u64) as u32 as u64;
            // e0f79 jne 0xe0f95
            if fl.ne() { pc = 0xe0f95; continue; }
            pc = 0xe0f7b;
        }
        0xe0f7b => {
            // block e0f7b
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f7b mov r15d,0x8
            r15 = (0x8u64) as u32 as u64;
            // e0f81 mov r13d,0x121
            r13 = (0x121u64) as u32 as u64;
            // e0f87 jmp 0xe0f95
            pc = 0xe0f95; continue;
        }
        0xe0f89 => {
            // block e0f89
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f89 mov r15d,0x8
            r15 = (0x8u64) as u32 as u64;
            // e0f8f mov r13d,0x5a0
            r13 = (0x5a0u64) as u32 as u64;
            pc = 0xe0f95;
        }
        0xe0f95 => {
            // block e0f95
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f95 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0f98 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // e0f9c mov rdi,r14
            rdi = r14;
            // e0f9f call QWORD PTR [rax+0x1ad8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ad8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0fa5 mov ecx,0xc0000000
            rcx = (0xc0000000u64) as u32 as u64;
            // e0faa and ecx,DWORD PTR [rbp-0x2c]
            { let r = (rcx as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0fad mov DWORD PTR [rbp-0x2c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rcx as u32 as u64)) as u32);
            // e0fb0 imul ebx,r13d
            { let (r, f) = imul::<32>((rbx as u32 as u64), (r13 as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // e0fb4 add ebx,0x8000
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x8000u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0fba shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0fbd mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e0fc2 mov edi,DWORD PTR [rbp-0x30]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // e0fc5 test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // e0fc7 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // e0fcc je 0xe1007
            if fl.e() { pc = 0xe1007; continue; }
            pc = 0xe0fce;
        }
        0xe0fce => {
            // block e0fce
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0fce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0fce mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // e0fd1 shr eax,1
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0fd3 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // e0fd6 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0fd8 cmp ebx,eax
            fl = sub::<32>((rbx as u32 as u64), (rax as u32 as u64)).1;
            // e0fda cmovbe edx,ebx
            if fl.be() { rdx = ((rbx as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // e0fdd shr r13d,0x2
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x2u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // e0fe1 cmp edx,r13d
            fl = sub::<32>((rdx as u32 as u64), (r13 as u32 as u64)).1;
            // e0fe4 cmovbe r13d,edx
            if fl.be() { r13 = ((rdx as u32 as u64)) as u32 as u64; } else { r13 = r13 as u32 as u64; }
            // e0fe8 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // e0feb mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // e0ff0 je 0xe1007
            if fl.e() { pc = 0xe1007; continue; }
            pc = 0xe0ff2;
        }
        0xe0ff2 => {
            // block e0ff2
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0ff2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0ff2 dec r13d
            { let (r, f) = dec::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // e0ff5 imul r13d,edi
            { let (r, f) = imul::<32>((r13 as u32 as u64), (rdi as u32 as u64)); r13 = (r) as u32 as u64; fl = f; }
            // e0ff9 add r13d,0x8001
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x8001u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e1000 shr r13d,0x10
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x10u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // e1004 mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            pc = 0xe1007;
        }
        0xe1007 => {
            // block e1007
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e1007 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1007 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e1009 cmovne esi,eax
            if fl.ne() { rsi = ((rax as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // e100c mov eax,0x1fff
            rax = (0x1fffu64) as u32 as u64;
            // e1011 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e1013 div esi
            { let (q, r) = div::<32>(rdx, rax, (rsi as u32 as u64)); rax = q; rdx = r; }
            // e1015 shr esi,1
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x1u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e1017 mov rdx,QWORD PTR [r14]
            rdx = m.r64(r14) as u64;
            // e101a shl eax,0x10
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e101d or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e101f add ebx,r15d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r15 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1022 sub ebx,esi
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rsi as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1024 shl ebx,0x2
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x2u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e1027 or ebx,ecx
            { let r = (rbx as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1029 mov rdi,r14
            rdi = r14;
            // e102c mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e102e call QWORD PTR [rdx+0x1ad0]
            { let r = { let t = m.r64(rdx.wrapping_add(0x1ad0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe1034;
        }
        0xe1034 => {
            // block e1034
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e1034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1034 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e1038 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e1039 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e103b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e103d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e103f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e1041 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e1042 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e7ee size=396
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e7ee<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x2e7ee;
    loop {
        match pc {
        0x2e7ee => {
            // block 2e7ee
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e7ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7ee push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e7ef mov rbp,rsp
            rbp = rsp;
            // 2e7f2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e7f4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e7f6 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2e7f8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e7fa push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e7fb sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 2e7ff mov r14,rcx
            r14 = rcx;
            // 2e802 mov r13d,edx
            r13 = ((rdx as u32 as u64)) as u32 as u64;
            // 2e805 mov r12,rsi
            r12 = rsi;
            // 2e808 mov r15,rdi
            r15 = rdi;
            // 2e80b mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 2e810 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2e813 je 0x2e955
            if fl.e() { pc = 0x2e955; continue; }
            pc = 0x2e819;
        }
        0x2e819 => {
            // block 2e819
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e819 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e819 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e81c mov rdi,r15
            rdi = r15;
            // 2e81f mov rsi,r12
            rsi = r12;
            // 2e822 call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e828 mov ebx,0x2
            rbx = (0x2u64) as u32 as u64;
            // 2e82d test rax,rax
            fl = logic::<64>(rax & rax);
            // 2e830 je 0x2e955
            if fl.e() { pc = 0x2e955; continue; }
            pc = 0x2e836;
        }
        0x2e836 => {
            // block 2e836
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e836 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e836 mov QWORD PTR [rbp-0x48],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r14) as u64);
            // 2e83a mov rdi,r12
            rdi = r12;
            // 2e83d call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e842 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2e846 mov rdi,rbx
            rdi = rbx;
            // 2e849 mov rsi,r12
            rsi = r12;
            // 2e84c mov rdx,rax
            rdx = rax;
            // 2e84f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e854 lea rsi,[rip+0xfc32d] # 0x12ab88
            rsi = m.env.data_va(258901);
            // 2e85b mov rdi,rbx
            rdi = rbx;
            // 2e85e call 0xebff6
            { let r = { let t = m.env.stub_va(0x7c3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e863 cmp r13d,0x4
            fl = sub::<32>((r13 as u32 as u64), 0x4u64).1;
            // 2e867 jae 0x2e8b8
            if fl.ae() { pc = 0x2e8b8; continue; }
            pc = 0x2e869;
        }
        0x2e869 => {
            // block 2e869
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e869 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e869 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e86c mov rax,QWORD PTR [rax+0x48]
            rax = m.r64(rax.wrapping_add(0x48)) as u64;
            // 2e870 mov rdi,r15
            rdi = r15;
            // 2e873 mov rsi,r12
            rsi = r12;
            // 2e876 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e878 mov DWORD PTR [rbp-0x4c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), ((rax as u32 as u64)) as u32);
            // 2e87b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e87e mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // 2e882 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 2e885 mov rdi,r15
            rdi = r15;
            // 2e888 mov rsi,r12
            rsi = r12;
            // 2e88b call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e88d mov DWORD PTR [rbp-0x50],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), ((rax as u32 as u64)) as u32);
            // 2e890 movsxd rax,r13d
            rax = sx::<32>((r13 as u32 as u64)) as u64;
            // 2e893 lea rcx,[rip+0xd1516] # 0xffdb0
            rcx = m.env.data_va(83325);
            // 2e89a mov r13d,DWORD PTR [rcx+rax*4]
            r13 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 2e89e lea rax,[rip+0xd149b] # 0xffd40
            rax = m.env.data_va(83213);
            pc = 0x2e8a5;
        }
        0x2e8a5 => {
            // block 2e8a5
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e8a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e8a5 cmp DWORD PTR [rax+r14*8],r13d
            fl = sub::<32>(m.r32(rax.wrapping_add(r14.wrapping_mul(8))) as u64, (r13 as u32 as u64)).1;
            // 2e8a9 je 0x2e8cb
            if fl.e() { pc = 0x2e8cb; continue; }
            pc = 0x2e8ab;
        }
        0x2e8ab => {
            // block 2e8ab
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e8ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e8ab inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // 2e8ae xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 2e8b0 cmp r14d,0x3
            fl = sub::<32>((r14 as u32 as u64), 0x3u64).1;
            // 2e8b4 jbe 0x2e8a5
            if fl.be() { pc = 0x2e8a5; continue; }
            pc = 0x2e8b6;
        }
        0x2e8b6 => {
            // block 2e8b6
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e8b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e8b6 jmp 0x2e8d0
            pc = 0x2e8d0; continue;
        }
        0x2e8b8 => {
            // block 2e8b8
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e8b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e8b8 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2e8bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e8c1 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 2e8c6 jmp 0x2e955
            pc = 0x2e955; continue;
        }
        0x2e8cb => {
            // block 2e8cb
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e8cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e8cb mov ebx,DWORD PTR [rax+r14*8+0x4]
            rbx = (m.r32(rax.wrapping_add(r14.wrapping_mul(8)).wrapping_add(0x4)) as u64) as u32 as u64;
            pc = 0x2e8d0;
        }
        0x2e8d0 => {
            // block 2e8d0
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e8d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e8d0 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e8d3 mov rax,QWORD PTR [rax+0x58]
            rax = m.r64(rax.wrapping_add(0x58)) as u64;
            // 2e8d7 mov rdi,r15
            rdi = r15;
            // 2e8da mov rsi,r12
            rsi = r12;
            // 2e8dd call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e8df mov edx,DWORD PTR [rbp-0x4c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64) as u32 as u64;
            // 2e8e2 imul ebx,edx
            { let (r, f) = imul::<32>((rbx as u32 as u64), (rdx as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // 2e8e5 mov r8d,ebx
            r8 = ((rbx as u32 as u64)) as u32 as u64;
            // 2e8e8 sar r8d,0x1f
            { let (r, f) = sar::<32>((r8 as u32 as u64), 0x1fu64, fl); r8 = (r) as u32 as u64; fl = f; }
            // 2e8ec shr r8d,0x1d
            { let (r, f) = shr::<32>((r8 as u32 as u64), 0x1du64, fl); r8 = (r) as u32 as u64; fl = f; }
            // 2e8f0 add r8d,ebx
            { let (r, f) = add::<32>((r8 as u32 as u64), (rbx as u32 as u64));
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2e8f3 sar r8d,0x3
            { let (r, f) = sar::<32>((r8 as u32 as u64), 0x3u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // 2e8f7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e8fa mov rax,QWORD PTR [rax+0x60]
            rax = m.r64(rax.wrapping_add(0x60)) as u64;
            // 2e8fe test BYTE PTR [rbp-0x40],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64 & 0x1u64);
            // 2e902 jne 0x2e90a
            if fl.ne() { pc = 0x2e90a; continue; }
            pc = 0x2e904;
        }
        0x2e904 => {
            // block 2e904
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e904 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e904 lea r9,[rbp-0x3f]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 2e908 jmp 0x2e90e
            pc = 0x2e90e; continue;
        }
        0x2e90a => {
            // block 2e90a
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e90a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e90a mov r9,QWORD PTR [rbp-0x30]
            r9 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x2e90e;
        }
        0x2e90e => {
            // block 2e90e
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e90e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e90e mov DWORD PTR [rsp+0x8],r13d
            m.w32(rsp.wrapping_add(0x8), ((r13 as u32 as u64)) as u32);
            // 2e913 mov DWORD PTR [rsp+0x18],0x0
            m.w32(rsp.wrapping_add(0x18), (0x0u64) as u32);
            // 2e91b mov DWORD PTR [rsp+0x10],0x0
            m.w32(rsp.wrapping_add(0x10), (0x0u64) as u32);
            // 2e923 mov DWORD PTR [rsp],0x1
            m.w32(rsp, (0x1u64) as u32);
            // 2e92a movsd xmm1,QWORD PTR [rip+0xd1256] # 0xffb88
            x1 = m.r64(m.env.data_va(82773)) as u128;
            // 2e932 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2e934 mov rdi,r15
            rdi = r15;
            // 2e937 mov ecx,DWORD PTR [rbp-0x50]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64) as u32 as u64;
            // 2e93a call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e93c mov rcx,QWORD PTR [rbp-0x48]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2e940 mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 2e943 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2e946 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 2e949 movzx ebx,al
            rbx = ((rax as u8 as u64)) as u32 as u64;
            // 2e94c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2e950 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x2e955;
        }
        0x2e955 => {
            // block 2e955
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e955 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e955 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 2e957 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 2e95b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e95c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e95e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2e960 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e962 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e964 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e965 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2e966 => {
            // block 2e966
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e966 mov rbx,rax
            rbx = rax;
            // 2e969 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2e96d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e972 mov rdi,rbx
            rdi = rbx;
            // 2e975 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1582c size=386
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001582c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1582c;
    loop {
        match pc {
        0x1582c => {
            // block 1582c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1582c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1582c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1582d mov rbp,rsp
            rbp = rsp;
            // 15830 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15832 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15834 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15836 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15838 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15839 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 1583d mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 15841 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 15844 je 0x15980
            if fl.e() { pc = 0x15980; continue; }
            pc = 0x1584a;
        }
        0x1584a => {
            // block 1584a
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1584a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1584a mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1584e movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            // 15851 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 15854 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 15858 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 1585b mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1585f mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 15863 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 15867 mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // 1586b mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 1586f lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // 15873 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 15877 jmp 0x1587c
            pc = 0x1587c; continue;
        }
        0x15879 => {
            // block 15879
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15879 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15879 mov rbx,rax
            rbx = rax;
            pc = 0x1587c;
        }
        0x1587c => {
            // block 1587c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1587c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1587c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 1587f mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 15883 cmove r13,QWORD PTR [rbp-0x30]
            if fl.e() { r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 15888 movzx eax,BYTE PTR [rbx+0x20]
            rax = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 1588c mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1588f and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 15893 jne 0x1589d
            if fl.ne() { pc = 0x1589d; continue; }
            pc = 0x15895;
        }
        0x15895 => {
            // block 15895
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15895 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15895 mov r14,rax
            r14 = rax;
            // 15898 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // 1589b jmp 0x158a1
            pc = 0x158a1; continue;
        }
        0x1589d => {
            // block 1589d
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1589d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1589d mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x158a1;
        }
        0x158a1 => {
            // block 158a1
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158a1 lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // 158a5 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 158a8 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 158ac cmove rdi,QWORD PTR [rbp-0x50]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // 158b1 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 158b4 jne 0x158c0
            if fl.ne() { pc = 0x158c0; continue; }
            pc = 0x158b6;
        }
        0x158b6 => {
            // block 158b6
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158b6 lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // 158ba mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // 158be jmp 0x158c8
            pc = 0x158c8; continue;
        }
        0x158c0 => {
            // block 158c0
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158c0 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // 158c4 mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x158c8;
        }
        0x158c8 => {
            // block 158c8
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158c8 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 158cb mov rdx,r13
            rdx = r13;
            // 158ce cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // 158d2 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 158d5 je 0x158ec
            if fl.e() { pc = 0x158ec; continue; }
            pc = 0x158d7;
        }
        0x158d7 => {
            // block 158d7
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158d7 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 158db call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 158e0 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 158e2 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 158e6 je 0x158ec
            if fl.e() { pc = 0x158ec; continue; }
            pc = 0x158e8;
        }
        0x158e8 => {
            // block 158e8
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158e8 jns 0x158fe
            if fl.ns() { pc = 0x158fe; continue; }
            pc = 0x158ea;
        }
        0x158ea => {
            // block 158ea
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158ea jmp 0x158f1
            pc = 0x158f1; continue;
        }
        0x158ec => {
            // block 158ec
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158ec cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 158ef jae 0x158fe
            if fl.ae() { pc = 0x158fe; continue; }
            pc = 0x158f1;
        }
        0x158f1 => {
            // block 158f1
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158f1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 158f4 test rax,rax
            fl = logic::<64>(rax & rax);
            // 158f7 jne 0x15879
            if fl.ne() { pc = 0x15879; continue; }
            pc = 0x158f9;
        }
        0x158f9 => {
            // block 158f9
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158f9 jmp 0x15995
            pc = 0x15995; continue;
        }
        0x158fe => {
            // block 158fe
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158fe test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 15901 jne 0x1590b
            if fl.ne() { pc = 0x1590b; continue; }
            pc = 0x15903;
        }
        0x15903 => {
            // block 15903
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15903 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15903 mov r14,rbx
            r14 = rbx;
            // 15906 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 15909 jmp 0x15912
            pc = 0x15912; continue;
        }
        0x1590b => {
            // block 1590b
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1590b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1590b mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 1590f mov r14,rbx
            r14 = rbx;
            pc = 0x15912;
        }
        0x15912 => {
            // block 15912
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15912 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15912 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 15916 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 15919 mov rbx,QWORD PTR [rbp-0x38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 1591d cmove rbx,QWORD PTR [rbp-0x30]
            if fl.e() { rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 15922 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 15925 jne 0x1592c
            if fl.ne() { pc = 0x1592c; continue; }
            pc = 0x15927;
        }
        0x15927 => {
            // block 15927
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15927 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15927 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 1592a jmp 0x15930
            pc = 0x15930; continue;
        }
        0x1592c => {
            // block 1592c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1592c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1592c mov rdi,QWORD PTR [r14+0x30]
            rdi = m.r64(r14.wrapping_add(0x30)) as u64;
            pc = 0x15930;
        }
        0x15930 => {
            // block 15930
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15930 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15930 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 15933 mov rdx,rax
            rdx = rax;
            // 15936 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // 1593a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 1593d je 0x1595e
            if fl.e() { pc = 0x1595e; continue; }
            pc = 0x1593f;
        }
        0x1593f => {
            // block 1593f
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1593f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1593f test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 15942 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 15946 cmove rsi,QWORD PTR [rbp-0x50]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // 1594b mov r12,rax
            r12 = rax;
            // 1594e call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15953 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 15955 mov rax,r12
            rax = r12;
            // 15958 je 0x1595e
            if fl.e() { pc = 0x1595e; continue; }
            pc = 0x1595a;
        }
        0x1595a => {
            // block 1595a
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1595a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1595a js 0x15963
            if fl.s() { pc = 0x15963; continue; }
            pc = 0x1595c;
        }
        0x1595c => {
            // block 1595c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1595c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1595c jmp 0x1598c
            pc = 0x1598c; continue;
        }
        0x1595e => {
            // block 1595e
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1595e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1595e cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // 15961 jae 0x1598c
            if fl.ae() { pc = 0x1598c; continue; }
            pc = 0x15963;
        }
        0x15963 => {
            // block 15963
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15963 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 15967 test rax,rax
            fl = logic::<64>(rax & rax);
            // 1596a jne 0x15879
            if fl.ne() { pc = 0x15879; continue; }
            pc = 0x15970;
        }
        0x15970 => {
            // block 15970
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15970 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15970 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 15974 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // 15977 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 1597b mov rax,r14
            rax = r14;
            // 1597e jmp 0x1599f
            pc = 0x1599f; continue;
        }
        0x15980 => {
            // block 15980
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15980 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15980 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 15984 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 15987 mov rax,rdi
            rax = rdi;
            // 1598a jmp 0x1599f
            pc = 0x1599f; continue;
        }
        0x1598c => {
            // block 1598c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1598c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1598c mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 15990 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // 15993 jmp 0x1599f
            pc = 0x1599f; continue;
        }
        0x15995 => {
            // block 15995
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15995 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15995 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 15999 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // 1599c mov rax,rbx
            rax = rbx;
            pc = 0x1599f;
        }
        0x1599f => {
            // block 1599f
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1599f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1599f add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 159a3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 159a4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 159a6 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 159a8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 159aa pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 159ac pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 159ad ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc548e size=386
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c548e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc548e;
    loop {
        match pc {
        0xc548e => {
            // block c548e
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c548e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c548e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c548f mov rbp,rsp
            rbp = rsp;
            // c5492 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c5494 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c5496 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c5498 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c549a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c549b sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // c549f mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c54a3 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c54a6 je 0xc55e2
            if fl.e() { pc = 0xc55e2; continue; }
            pc = 0xc54ac;
        }
        0xc54ac => {
            // block c54ac
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c54ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c54ac mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // c54b0 movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            // c54b3 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c54b6 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c54ba shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // c54bd mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // c54c1 mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // c54c5 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // c54c9 mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // c54cd mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // c54d1 lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // c54d5 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // c54d9 jmp 0xc54de
            pc = 0xc54de; continue;
        }
        0xc54db => {
            // block c54db
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c54db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c54db mov rbx,rax
            rbx = rax;
            pc = 0xc54de;
        }
        0xc54de => {
            // block c54de
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c54de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c54de test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c54e1 mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c54e5 cmove r13,QWORD PTR [rbp-0x30]
            if fl.e() { r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // c54ea movzx eax,BYTE PTR [rbx+0x20]
            rax = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // c54ee mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c54f1 and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c54f5 jne 0xc54ff
            if fl.ne() { pc = 0xc54ff; continue; }
            pc = 0xc54f7;
        }
        0xc54f7 => {
            // block c54f7
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c54f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c54f7 mov r14,rax
            r14 = rax;
            // c54fa shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // c54fd jmp 0xc5503
            pc = 0xc5503; continue;
        }
        0xc54ff => {
            // block c54ff
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c54ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c54ff mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xc5503;
        }
        0xc5503 => {
            // block c5503
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5503 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5503 lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // c5507 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c550a mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // c550e cmove rdi,QWORD PTR [rbp-0x50]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // c5513 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c5516 jne 0xc5522
            if fl.ne() { pc = 0xc5522; continue; }
            pc = 0xc5518;
        }
        0xc5518 => {
            // block c5518
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5518 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5518 lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // c551c mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // c5520 jmp 0xc552a
            pc = 0xc552a; continue;
        }
        0xc5522 => {
            // block c5522
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5522 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5522 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // c5526 mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xc552a;
        }
        0xc552a => {
            // block c552a
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c552a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c552a cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // c552d mov rdx,r13
            rdx = r13;
            // c5530 cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // c5534 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c5537 je 0xc554e
            if fl.e() { pc = 0xc554e; continue; }
            pc = 0xc5539;
        }
        0xc5539 => {
            // block c5539
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5539 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5539 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // c553d call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5542 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c5544 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // c5548 je 0xc554e
            if fl.e() { pc = 0xc554e; continue; }
            pc = 0xc554a;
        }
        0xc554a => {
            // block c554a
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c554a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c554a jns 0xc5560
            if fl.ns() { pc = 0xc5560; continue; }
            pc = 0xc554c;
        }
        0xc554c => {
            // block c554c
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c554c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c554c jmp 0xc5553
            pc = 0xc5553; continue;
        }
        0xc554e => {
            // block c554e
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c554e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c554e cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // c5551 jae 0xc5560
            if fl.ae() { pc = 0xc5560; continue; }
            pc = 0xc5553;
        }
        0xc5553 => {
            // block c5553
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5553 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5553 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c5556 test rax,rax
            fl = logic::<64>(rax & rax);
            // c5559 jne 0xc54db
            if fl.ne() { pc = 0xc54db; continue; }
            pc = 0xc555b;
        }
        0xc555b => {
            // block c555b
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c555b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c555b jmp 0xc55f7
            pc = 0xc55f7; continue;
        }
        0xc5560 => {
            // block c5560
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5560 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c5563 jne 0xc556d
            if fl.ne() { pc = 0xc556d; continue; }
            pc = 0xc5565;
        }
        0xc5565 => {
            // block c5565
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5565 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5565 mov r14,rbx
            r14 = rbx;
            // c5568 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // c556b jmp 0xc5574
            pc = 0xc5574; continue;
        }
        0xc556d => {
            // block c556d
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c556d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c556d mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // c5571 mov r14,rbx
            r14 = rbx;
            pc = 0xc5574;
        }
        0xc5574 => {
            // block c5574
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5574 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5574 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // c5578 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c557b mov rbx,QWORD PTR [rbp-0x38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c557f cmove rbx,QWORD PTR [rbp-0x30]
            if fl.e() { rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // c5584 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c5587 jne 0xc558e
            if fl.ne() { pc = 0xc558e; continue; }
            pc = 0xc5589;
        }
        0xc5589 => {
            // block c5589
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5589 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5589 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // c558c jmp 0xc5592
            pc = 0xc5592; continue;
        }
        0xc558e => {
            // block c558e
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c558e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c558e mov rdi,QWORD PTR [r14+0x30]
            rdi = m.r64(r14.wrapping_add(0x30)) as u64;
            pc = 0xc5592;
        }
        0xc5592 => {
            // block c5592
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5592 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5592 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // c5595 mov rdx,rax
            rdx = rax;
            // c5598 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // c559c test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c559f je 0xc55c0
            if fl.e() { pc = 0xc55c0; continue; }
            pc = 0xc55a1;
        }
        0xc55a1 => {
            // block c55a1
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55a1 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c55a4 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // c55a8 cmove rsi,QWORD PTR [rbp-0x50]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // c55ad mov r12,rax
            r12 = rax;
            // c55b0 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c55b5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c55b7 mov rax,r12
            rax = r12;
            // c55ba je 0xc55c0
            if fl.e() { pc = 0xc55c0; continue; }
            pc = 0xc55bc;
        }
        0xc55bc => {
            // block c55bc
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55bc js 0xc55c5
            if fl.s() { pc = 0xc55c5; continue; }
            pc = 0xc55be;
        }
        0xc55be => {
            // block c55be
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55be jmp 0xc55ee
            pc = 0xc55ee; continue;
        }
        0xc55c0 => {
            // block c55c0
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55c0 cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // c55c3 jae 0xc55ee
            if fl.ae() { pc = 0xc55ee; continue; }
            pc = 0xc55c5;
        }
        0xc55c5 => {
            // block c55c5
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55c5 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // c55c9 test rax,rax
            fl = logic::<64>(rax & rax);
            // c55cc jne 0xc54db
            if fl.ne() { pc = 0xc54db; continue; }
            pc = 0xc55d2;
        }
        0xc55d2 => {
            // block c55d2
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55d2 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c55d6 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // c55d9 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // c55dd mov rax,r14
            rax = r14;
            // c55e0 jmp 0xc5601
            pc = 0xc5601; continue;
        }
        0xc55e2 => {
            // block c55e2
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55e2 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c55e6 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // c55e9 mov rax,rdi
            rax = rdi;
            // c55ec jmp 0xc5601
            pc = 0xc5601; continue;
        }
        0xc55ee => {
            // block c55ee
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55ee mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c55f2 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // c55f5 jmp 0xc5601
            pc = 0xc5601; continue;
        }
        0xc55f7 => {
            // block c55f7
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55f7 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c55fb mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // c55fe mov rax,rbx
            rax = rbx;
            pc = 0xc5601;
        }
        0xc5601 => {
            // block c5601
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5601 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5601 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // c5605 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c5606 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c5608 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c560a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c560c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c560e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c560f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe8310 size=380
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e8310<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe8310;
    loop {
        match pc {
        0xe8310 => {
            // block e8310
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8310 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8310 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e8311 mov rbp,rsp
            rbp = rsp;
            // e8314 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e8316 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e8317 sub rsp,0x340
            { let (r, f) = sub::<64>(rsp, 0x340u64);
              rsp = r;
              fl = f; }
            // e831e mov rbx,rdi
            rbx = rdi;
            // e8321 mov r14,QWORD PTR [rip+0x580b0] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // e8328 mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // e832b mov QWORD PTR [rbp-0x18],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (r14) as u64);
            // e832f mov WORD PTR [rbp-0x312],0x0
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_fcee), (0x0u64) as u16);
            // e8338 xorps xmm0,xmm0
            x0 = 0;
            // e833b movaps XMMWORD PTR [rbp-0x330],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fcd0), x0);
            // e8342 mov QWORD PTR [rbp-0x320],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce0), (0x0u64) as u64);
            // e834d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e8350 mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // e8354 lea rsi,[rbp-0x330]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fcd0);
            // e835b lea rcx,[rbp-0x312]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_fcee);
            // e8362 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e8364 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e8367 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8369 mov rax,QWORD PTR [rbp-0x328]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd8)) as u64;
            // e8370 cmp rax,QWORD PTR [rbp-0x330]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64).1;
            // e8377 je 0xe8401
            if fl.e() { pc = 0xe8401; continue; }
            pc = 0xe837d;
        }
        0xe837d => {
            // block e837d
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e837d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e837d lea rdi,[rbp-0x310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf0);
            // e8384 call 0x12b8c
            { let r = { let t = m.env.code_va(0x12b8c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8389 mov rdx,QWORD PTR [rbp-0x330]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64;
            // e8390 lea rdi,[rbp-0x348]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb8);
            // e8397 lea rsi,[rbp-0x310]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fcf0);
            // e839e call 0x12e88
            { let r = { let t = m.env.code_va(0x12e88); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e83a3 lea rdi,[rbp-0x348]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb8);
            // e83aa call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e83af lea rdi,[rbx+0x2c0]
            rdi = rbx.wrapping_add(0x2c0);
            // e83b6 lea rsi,[rbp-0xa8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e83bd call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e83c2 lea rdi,[rbx+0x2f0]
            rdi = rbx.wrapping_add(0x2f0);
            // e83c9 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e83cd call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e83d2 lea rdi,[rbx+0x308]
            rdi = rbx.wrapping_add(0x308);
            // e83d9 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e83dd call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e83e2 lea rdi,[rbx+0x2d8]
            rdi = rbx.wrapping_add(0x2d8);
            // e83e9 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e83f0 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e83f5 lea rdi,[rbp-0x310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf0);
            // e83fc call 0x14c0c
            { let r = { let t = m.env.code_va(0x14c0c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe8401;
        }
        0xe8401 => {
            // block e8401
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8401 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8401 mov rsi,QWORD PTR [rbx+0x268]
            rsi = m.r64(rbx.wrapping_add(0x268)) as u64;
            // e8408 add rbx,0x2a8
            { let (r, f) = add::<64>(rbx, 0x2a8u64);
              rbx = r;
              fl = f; }
            // e840f mov rdi,rbx
            rdi = rbx;
            // e8412 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8417 mov rdi,QWORD PTR [rbp-0x330]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64;
            // e841e test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e8421 je 0xe8438
            if fl.e() { pc = 0xe8438; continue; }
            pc = 0xe8423;
        }
        0xe8423 => {
            // block e8423
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8423 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8423 cmp QWORD PTR [rbp-0x328],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd8)) as u64, rdi).1;
            // e842a je 0xe8433
            if fl.e() { pc = 0xe8433; continue; }
            pc = 0xe842c;
        }
        0xe842c => {
            // block e842c
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e842c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e842c mov QWORD PTR [rbp-0x328],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcd8), (rdi) as u64);
            pc = 0xe8433;
        }
        0xe8433 => {
            // block e8433
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8433 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8433 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe8438;
        }
        0xe8438 => {
            // block e8438
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8438 cmp r14,QWORD PTR [rbp-0x18]
            fl = sub::<64>(r14, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64).1;
            // e843c jne 0xe8487
            if fl.ne() { pc = 0xe8487; continue; }
            pc = 0xe843e;
        }
        0xe843e => {
            // block e843e
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e843e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e843e add rsp,0x340
            { let (r, f) = add::<64>(rsp, 0x340u64);
              rsp = r;
              fl = f; }
            // e8445 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e8446 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e8448 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e8449 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe844a => {
            // block e844a
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e844a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e844a mov rbx,rax
            rbx = rax;
            // e844d lea rdi,[rbp-0x310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf0);
            // e8454 call 0x14c0c
            { let r = { let t = m.env.code_va(0x14c0c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8459 jmp 0xe845e
            pc = 0xe845e; continue;
        }
        0xe845b => {
            // block e845b
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e845b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e845b mov rbx,rax
            rbx = rax;
            pc = 0xe845e;
        }
        0xe845e => {
            // block e845e
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e845e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e845e mov rdi,QWORD PTR [rbp-0x330]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64;
            // e8465 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e8468 je 0xe847f
            if fl.e() { pc = 0xe847f; continue; }
            pc = 0xe846a;
        }
        0xe846a => {
            // block e846a
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e846a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e846a cmp QWORD PTR [rbp-0x328],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd8)) as u64, rdi).1;
            // e8471 je 0xe847a
            if fl.e() { pc = 0xe847a; continue; }
            pc = 0xe8473;
        }
        0xe8473 => {
            // block e8473
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8473 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8473 mov QWORD PTR [rbp-0x328],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcd8), (rdi) as u64);
            pc = 0xe847a;
        }
        0xe847a => {
            // block e847a
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e847a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e847a call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe847f;
        }
        0xe847f => {
            // block e847f
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e847f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e847f mov rdi,rbx
            rdi = rbx;
            // e8482 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe8487;
        }
        0xe8487 => {
            // block e8487
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8487 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8487 call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10b02 size=372
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010b02<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x10b02;
    loop {
        match pc {
        0x10b02 => {
            // block 10b02
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10b02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10b02 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10b03 mov rbp,rsp
            rbp = rsp;
            // 10b06 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10b08 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10b0a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 10b0c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 10b0e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10b0f sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10b13 mov r15,rdi
            r15 = rdi;
            // 10b16 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10b1a mov r13,QWORD PTR [r15+0x18]
            r13 = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10b1e cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 10b21 jne 0x10c5b
            if fl.ne() { pc = 0x10c5b; continue; }
            pc = 0x10b27;
        }
        0x10b27 => {
            // block 10b27
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10b27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10b27 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 10b2b mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 10b2e mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10b32 mov rax,rbx
            rax = rbx;
            // 10b35 sub rax,rcx
            { let (r, f) = sub::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 10b38 jbe 0x10b8d
            if fl.be() { pc = 0x10b8d; continue; }
            pc = 0x10b3a;
        }
        0x10b3a => {
            // block 10b3a
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10b3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10b3a sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 10b3e lea rcx,[rax+0x1]
            rcx = rax.wrapping_add(0x1);
            // 10b42 shr rcx,0x3f
            { let (r, f) = shr::<64>(rcx, 0x3fu64, fl); rcx = r; fl = f; }
            // 10b46 lea r14,[rax+rcx*1+0x1]
            r14 = rax.wrapping_add(rcx).wrapping_add(0x1);
            // 10b4b sar r14,1
            { let (r, f) = sar::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // 10b4e lea rax,[r14*8+0x0]
            rax = r14.wrapping_mul(8);
            // 10b56 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10b5a mov rdi,rbx
            rdi = rbx;
            // 10b5d sub rdi,rax
            { let (r, f) = sub::<64>(rdi, rax);
              rdi = r;
              fl = f; }
            // 10b60 sub r12,rbx
            { let (r, f) = sub::<64>(r12, rbx);
              r12 = r;
              fl = f; }
            // 10b63 mov r13,r12
            r13 = r12;
            // 10b66 shr r13,0x3
            { let (r, f) = shr::<64>(r13, 0x3u64, fl); r13 = r; fl = f; }
            // 10b6a mov rsi,rbx
            rsi = rbx;
            // 10b6d mov rdx,r12
            rdx = r12;
            // 10b70 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10b75 sub r13,r14
            { let (r, f) = sub::<64>(r13, r14);
              r13 = r;
              fl = f; }
            // 10b78 lea r12,[rbx+r13*8]
            r12 = rbx.wrapping_add(r13.wrapping_mul(8));
            // 10b7c mov QWORD PTR [r15+0x10],r12
            m.w64(r15.wrapping_add(0x10), (r12) as u64);
            // 10b80 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 10b84 sub QWORD PTR [r15+0x8],rax
            { let (r, f) = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 10b88 jmp 0x10c57
            pc = 0x10c57; continue;
        }
        0x10b8d => {
            // block 10b8d
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10b8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10b8d mov rax,r12
            rax = r12;
            // 10b90 sub rax,rcx
            { let (r, f) = sub::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 10b93 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10b97 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 10b9d cmovne r14,rax
            if fl.ne() { r14 = rax; }
            // 10ba1 movabs rax,0x7ffffffffffffffc
            rax = 0x7fff_ffff_ffff_fffcu64;
            // 10bab and rax,r14
            { let r = rax & r14; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 10bae mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10bb2 lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 10bb6 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 10bba mov QWORD PTR [rbp-0x68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rcx) as u64);
            // 10bbe lea rdi,[r14*8+0x0]
            rdi = r14.wrapping_mul(8);
            // 10bc6 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10bcb mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 10bcf mov rcx,QWORD PTR [rbp-0x60]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 10bd3 lea rcx,[rax+rcx*2]
            rcx = rax.wrapping_add(rcx.wrapping_mul(2));
            // 10bd7 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10bdb mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 10bdf lea rdx,[rax+r14*8]
            rdx = rax.wrapping_add(r14.wrapping_mul(8));
            // 10be3 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // 10be7 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 10bea je 0x10c24
            if fl.e() { pc = 0x10c24; continue; }
            pc = 0x10bec;
        }
        0x10bec => {
            // block 10bec
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10bec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10bec mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 10bef mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 10bf2 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 10bf6 add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 10bfa mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10bfe add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 10c02 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // 10c05 jne 0x10bec
            if fl.ne() { pc = 0x10bec; continue; }
            pc = 0x10c07;
        }
        0x10c07 => {
            // block 10c07
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10c07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c07 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // 10c0a mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10c0e mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 10c12 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 10c16 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10c1a mov r13,QWORD PTR [r15+0x18]
            r13 = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10c1e mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 10c22 jmp 0x10c2b
            pc = 0x10c2b; continue;
        }
        0x10c24 => {
            // block 10c24
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10c24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c24 mov rsi,rcx
            rsi = rcx;
            // 10c27 mov rdi,QWORD PTR [rbp-0x68]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            pc = 0x10c2b;
        }
        0x10c2b => {
            // block 10c2b
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10c2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c2b mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 10c2e mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 10c32 mov QWORD PTR [r15+0x8],rsi
            m.w64(r15.wrapping_add(0x8), (rsi) as u64);
            // 10c36 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 10c3a mov QWORD PTR [r15+0x10],rcx
            m.w64(r15.wrapping_add(0x10), (rcx) as u64);
            // 10c3e mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 10c42 mov QWORD PTR [r15+0x18],rdx
            m.w64(r15.wrapping_add(0x18), (rdx) as u64);
            // 10c46 mov QWORD PTR [rbp-0x38],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r13) as u64);
            // 10c4a lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10c4e call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10c53 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x10c57;
        }
        0x10c57 => {
            // block 10c57
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10c57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c57 mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x10c5b;
        }
        0x10c5b => {
            // block 10c5b
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10c5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c5b mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // 10c5e mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 10c62 add QWORD PTR [r15+0x10],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 10c67 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10c6b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 10c6c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 10c6e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 10c70 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10c72 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 10c74 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10c75 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa32 size=370
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000a32<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xa32;
    loop {
        match pc {
        0xa32 => {
            // block a32
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc a32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a32 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a33 mov rbp,rsp
            rbp = rsp;
            // a36 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // a38 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // a3a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // a3b sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // a3f mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // a42 mov rbx,rdi
            rbx = rdi;
            // a45 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a48 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4e mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // a51 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a54 lea rcx,[rbp-0x30]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a58 mov QWORD PTR [rsp],rcx
            m.w64(rsp, (rcx) as u64);
            // a5c lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // a60 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // a64 lea rcx,[rbp-0x24]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // a68 lea r8,[rbp-0x28]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // a6c lea r9,[rbp-0x2c]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // a70 mov rdi,rbx
            rdi = rbx;
            // a73 call QWORD PTR [rax+0xd10]
            { let r = { let t = m.r64(rax.wrapping_add(0xd10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a79 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // a7b je 0xb26
            if fl.e() { pc = 0xb26; continue; }
            pc = 0xa81;
        }
        0xa81 => {
            // block a81
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc a81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a81 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // a84 call 0xce033
            { let r = { let t = m.env.code_va(0xce033); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a89 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // a8c neg ecx
            { let (r, f) = neg::<32>((rcx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // a8e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // a90 cmovne ecx,r14d
            if fl.ne() { rcx = ((r14 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // a94 add ecx,DWORD PTR [rbp-0x1c]
            { let (r, f) = add::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // a97 mov DWORD PTR [rbp-0x1c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rcx as u32 as u64)) as u32);
            // a9a mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // a9d cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // a9f jg 0xaa8
            if fl.g() { pc = 0xaa8; continue; }
            pc = 0xaa1;
        }
        0xaa1 => {
            // block aa1
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc aa1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa1 mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // aa4 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // aa6 jge 0xaab
            if fl.ge() { pc = 0xaab; continue; }
            pc = 0xaa8;
        }
        0xaa8 => {
            // block aa8
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc aa8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa8 mov DWORD PTR [rbp-0x1c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rax as u32 as u64)) as u32);
            pc = 0xaab;
        }
        0xaab => {
            // block aab
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc aab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // aae lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // ab2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ab4 mov rdi,rbx
            rdi = rbx;
            // ab7 call QWORD PTR [rax+0xd60]
            { let r = { let t = m.r64(rax.wrapping_add(0xd60)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abd mov eax,DWORD PTR [rbp-0x34]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // ac0 movzx ecx,ax
            rcx = ((rax as u16 as u64)) as u32 as u64;
            // ac3 mov esi,DWORD PTR [rbp-0x1c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // ac6 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // ac9 cmp ecx,esi
            fl = sub::<32>((rcx as u32 as u64), (rsi as u32 as u64)).1;
            // acb je 0xb95
            if fl.e() { pc = 0xb95; continue; }
            pc = 0xad1;
        }
        0xad1 => {
            // block ad1
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc ad1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad1 lea edx,[rcx+0x1]
            rdx = (rcx.wrapping_add(0x1)) as u32 as u64;
            // ad4 cmp edx,esi
            fl = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64)).1;
            // ad6 jne 0xb2b
            if fl.ne() { pc = 0xb2b; continue; }
            pc = 0xad8;
        }
        0xad8 => {
            // block ad8
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc ad8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad8 and eax,0xffff0000
            { let r = (rax as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // add add esi,0x2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ae0 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ae2 mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // ae5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ae8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aea mov rdi,rbx
            rdi = rbx;
            // aed call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af6 lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // afa mov rdi,rbx
            rdi = rbx;
            // afd call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03 lea r15,[rbp-0x3c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            pc = 0xb07;
        }
        0xb07 => {
            // block b07
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b07 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0a mov rdi,rbx
            rdi = rbx;
            // b0d mov rsi,r15
            rsi = r15;
            // b10 call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b16 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // b19 cmp eax,DWORD PTR [rbp-0x3c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64).1;
            // b1c jne 0xb07
            if fl.ne() { pc = 0xb07; continue; }
            pc = 0xb1e;
        }
        0xb1e => {
            // block b1e
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b1e mov esi,DWORD PTR [rbp-0x34]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // b21 add esi,0xfffffffe
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffffeu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b24 jmp 0xb84
            pc = 0xb84; continue;
        }
        0xb26 => {
            // block b26
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b26 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b29 jmp 0xb95
            pc = 0xb95; continue;
        }
        0xb2b => {
            // block b2b
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2b dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // b2d and eax,0xffff0000
            { let r = (rax as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b32 mov DWORD PTR [rbp-0x34],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rax as u32 as u64)) as u32);
            // b35 cmp ecx,esi
            fl = sub::<32>((rcx as u32 as u64), (rsi as u32 as u64)).1;
            // b37 jne 0xb82
            if fl.ne() { pc = 0xb82; continue; }
            pc = 0xb39;
        }
        0xb39 => {
            // block b39
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b39 add esi,0xfffffffe
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffffeu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b3c or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b3e mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // b41 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b44 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b46 mov rdi,rbx
            rdi = rbx;
            // b49 call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b52 lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b56 mov rdi,rbx
            rdi = rbx;
            // b59 call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5f lea r15,[rbp-0x3c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            pc = 0xb63;
        }
        0xb63 => {
            // block b63
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b66 mov rdi,rbx
            rdi = rbx;
            // b69 mov rsi,r15
            rsi = r15;
            // b6c call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b72 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // b75 cmp eax,DWORD PTR [rbp-0x3c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64).1;
            // b78 jne 0xb63
            if fl.ne() { pc = 0xb63; continue; }
            pc = 0xb7a;
        }
        0xb7a => {
            // block b7a
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a mov esi,DWORD PTR [rbp-0x34]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // b7d add esi,0x2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b80 jmp 0xb84
            pc = 0xb84; continue;
        }
        0xb82 => {
            // block b82
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b82 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0xb84;
        }
        0xb84 => {
            // block b84
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b84 mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // b87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b8a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b8c mov rdi,rbx
            rdi = rbx;
            // b8f call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb95;
        }
        0xb95 => {
            // block b95
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b95 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b98 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // b9c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b9d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10f58 size=366
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010f58<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x10f58;
    loop {
        match pc {
        0x10f58 => {
            // block 10f58
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 10f58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f58 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10f59 mov rbp,rsp
            rbp = rsp;
            // 10f5c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10f5e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10f60 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 10f62 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 10f64 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10f65 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10f69 mov r14,rsi
            r14 = rsi;
            // 10f6c mov r15,rdi
            r15 = rdi;
            // 10f6f mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 10f72 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10f76 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 10f79 jne 0x110ab
            if fl.ne() { pc = 0x110ab; continue; }
            pc = 0x10f7f;
        }
        0x10f7f => {
            // block 10f7f
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 10f7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f7f mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10f83 mov rcx,QWORD PTR [r15+0x18]
            rcx = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10f87 cmp r12,rcx
            fl = sub::<64>(r12, rcx).1;
            // 10f8a jae 0x10fd9
            if fl.ae() { pc = 0x10fd9; continue; }
            pc = 0x10f8c;
        }
        0x10f8c => {
            // block 10f8c
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 10f8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f8c sub rcx,r12
            { let (r, f) = sub::<64>(rcx, r12);
              rcx = r;
              fl = f; }
            // 10f8f sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 10f93 lea rax,[rcx+0x1]
            rax = rcx.wrapping_add(0x1);
            // 10f97 shr rax,0x3f
            { let (r, f) = shr::<64>(rax, 0x3fu64, fl); rax = r; fl = f; }
            // 10f9b lea rbx,[rcx+rax*1+0x1]
            rbx = rcx.wrapping_add(rax).wrapping_add(0x1);
            // 10fa0 sar rbx,1
            { let (r, f) = sar::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 10fa3 mov rdx,r12
            rdx = r12;
            // 10fa6 sub rdx,r13
            { let (r, f) = sub::<64>(rdx, r13);
              rdx = r;
              fl = f; }
            // 10fa9 mov rax,rdx
            rax = rdx;
            // 10fac shr rax,0x3
            { let (r, f) = shr::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 10fb0 mov rcx,rbx
            rcx = rbx;
            // 10fb3 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 10fb6 lea r12,[r12+rcx*8]
            r12 = r12.wrapping_add(rcx.wrapping_mul(8));
            // 10fba mov rdi,r12
            rdi = r12;
            // 10fbd mov rsi,r13
            rsi = r13;
            // 10fc0 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10fc5 mov QWORD PTR [r15+0x8],r12
            m.w64(r15.wrapping_add(0x8), (r12) as u64);
            // 10fc9 shl rbx,0x3
            { let (r, f) = shl::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // 10fcd add QWORD PTR [r15+0x10],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, rbx);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 10fd1 mov r13,r12
            r13 = r12;
            // 10fd4 jmp 0x110ab
            pc = 0x110ab; continue;
        }
        0x10fd9 => {
            // block 10fd9
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 10fd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10fd9 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10fdd mov QWORD PTR [rbp-0x58],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r14) as u64);
            // 10fe1 mov rax,rcx
            rax = rcx;
            // 10fe4 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 10fe7 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10feb mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 10ff1 cmovne r14,rax
            if fl.ne() { r14 = rax; }
            // 10ff5 lea rbx,[r14+r14*1+0x6]
            rbx = r14.wrapping_add(r14).wrapping_add(0x6);
            // 10ffa and rbx,0xfffffffffffffff8
            { let r = rbx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 10ffe mov rax,QWORD PTR [r15+0x20]
            rax = m.r64(r15.wrapping_add(0x20)) as u64;
            // 11002 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 11006 lea rdi,[r14*8+0x0]
            rdi = r14.wrapping_mul(8);
            // 1100e mov QWORD PTR [rbp-0x68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rcx) as u64);
            // 11012 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11017 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 1101b add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 1101e mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // 11022 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 11026 lea rcx,[rax+r14*8]
            rcx = rax.wrapping_add(r14.wrapping_mul(8));
            // 1102a mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 1102e cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 11031 je 0x11070
            if fl.e() { pc = 0x11070; continue; }
            pc = 0x11033;
        }
        0x11033 => {
            // block 11033
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 11033 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11033 mov r14,QWORD PTR [rbp-0x58]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x11037;
        }
        0x11037 => {
            // block 11037
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 11037 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11037 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 1103b mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 1103e mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 11042 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 11046 mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // 1104a add r13,0x8
            { let (r, f) = add::<64>(r13, 0x8u64);
              r13 = r;
              fl = f; }
            // 1104e cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 11051 jne 0x11037
            if fl.ne() { pc = 0x11037; continue; }
            pc = 0x11053;
        }
        0x11053 => {
            // block 11053
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 11053 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11053 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // 11056 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1105a mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 1105e mov rdx,QWORD PTR [rbp-0x48]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 11062 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 11066 mov rsi,QWORD PTR [r15+0x18]
            rsi = m.r64(r15.wrapping_add(0x18)) as u64;
            // 1106a mov rcx,QWORD PTR [rbp-0x38]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 1106e jmp 0x1107f
            pc = 0x1107f; continue;
        }
        0x11070 => {
            // block 11070
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 11070 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11070 mov rdx,rbx
            rdx = rbx;
            // 11073 mov r14,QWORD PTR [rbp-0x58]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 11077 mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1107b mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            pc = 0x1107f;
        }
        0x1107f => {
            // block 1107f
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 1107f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1107f mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 11082 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 11086 mov QWORD PTR [r15+0x8],rdx
            m.w64(r15.wrapping_add(0x8), (rdx) as u64);
            // 1108a mov QWORD PTR [rbp-0x48],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r13) as u64);
            // 1108e mov QWORD PTR [r15+0x10],rbx
            m.w64(r15.wrapping_add(0x10), (rbx) as u64);
            // 11092 mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 11096 mov QWORD PTR [r15+0x18],rcx
            m.w64(r15.wrapping_add(0x18), (rcx) as u64);
            // 1109a mov QWORD PTR [rbp-0x38],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rsi) as u64);
            // 1109e lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 110a2 call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 110a7 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x110ab;
        }
        0x110ab => {
            // block 110ab
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 110ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 110ab mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 110ae mov QWORD PTR [r13-0x8],rax
            m.w64(r13.wrapping_add(0xffff_ffff_ffff_fff8), (rax) as u64);
            // 110b2 add QWORD PTR [r15+0x8],0xfffffffffffffff8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, 0xffff_ffff_ffff_fff8u64);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 110b7 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 110bb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 110bc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 110be pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 110c0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 110c2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 110c4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 110c5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1261c size=361
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001261c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1261c;
    loop {
        match pc {
        0x1261c => {
            // block 1261c
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 1261c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1261c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1261d mov rbp,rsp
            rbp = rsp;
            // 12620 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12622 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12624 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 12626 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12628 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 12629 sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 1262d mov r15,rdx
            r15 = rdx;
            // 12630 mov rax,rsi
            rax = rsi;
            // 12633 mov rbx,QWORD PTR [rbp+0x10]
            rbx = m.r64(rbp.wrapping_add(0x10)) as u64;
            // 12637 cmp r8,r9
            fl = sub::<64>(r8, r9).1;
            // 1263a jle 0x12673
            if fl.le() { pc = 0x12673; continue; }
            pc = 0x1263c;
        }
        0x1263c => {
            // block 1263c
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 1263c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1263c mov QWORD PTR [rbp-0x60],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rcx) as u64);
            // 12640 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 12644 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 12647 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 1264a je 0x126b0
            if fl.e() { pc = 0x126b0; continue; }
            pc = 0x1264c;
        }
        0x1264c => {
            // block 1264c
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 1264c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1264c mov r13,rbx
            r13 = rbx;
            // 1264f mov r12,rax
            r12 = rax;
            // 12652 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            pc = 0x12656;
        }
        0x12656 => {
            // block 12656
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12656 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12656 mov rdi,r13
            rdi = r13;
            // 12659 mov rsi,r12
            rsi = r12;
            // 1265c call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12661 inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // 12664 add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 12668 add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 1266c cmp r15,r12
            fl = sub::<64>(r15, r12).1;
            // 1266f jne 0x12656
            if fl.ne() { pc = 0x12656; continue; }
            pc = 0x12671;
        }
        0x12671 => {
            // block 12671
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12671 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12671 jmp 0x126b7
            pc = 0x126b7; continue;
        }
        0x12673 => {
            // block 12673
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12673 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12673 mov QWORD PTR [rbp-0x58],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r15) as u64);
            // 12677 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 1267a cmp rdi,rax
            fl = sub::<64>(rdi, rax).1;
            // 1267d je 0x12711
            if fl.e() { pc = 0x12711; continue; }
            pc = 0x12683;
        }
        0x12683 => {
            // block 12683
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12683 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12683 mov r13,rbx
            r13 = rbx;
            // 12686 mov r12,rdi
            r12 = rdi;
            // 12689 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            pc = 0x1268d;
        }
        0x1268d => {
            // block 1268d
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 1268d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1268d mov r15,rax
            r15 = rax;
            // 12690 mov rdi,r13
            rdi = r13;
            // 12693 mov rsi,r12
            rsi = r12;
            // 12696 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1269b inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // 1269e add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 126a2 add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 126a6 mov rax,r15
            rax = r15;
            // 126a9 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 126ac jne 0x1268d
            if fl.ne() { pc = 0x1268d; continue; }
            pc = 0x126ae;
        }
        0x126ae => {
            // block 126ae
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 126ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 126ae jmp 0x12718
            pc = 0x12718; continue;
        }
        0x126b0 => {
            // block 126b0
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 126b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 126b0 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 126b4 mov r13,rbx
            r13 = rbx;
            pc = 0x126b7;
        }
        0x126b7 => {
            // block 126b7
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 126b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 126b7 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 126bb mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 126bf mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 126c3 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 126c7 mov QWORD PTR [rbp-0x40],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r15) as u64);
            // 126cb mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 126cf mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 126d3 mov QWORD PTR [rsp+0x18],rcx
            m.w64(rsp.wrapping_add(0x18), (rcx) as u64);
            // 126d8 mov QWORD PTR [rsp+0x10],rax
            m.w64(rsp.wrapping_add(0x10), (rax) as u64);
            // 126dd mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 126e1 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 126e5 mov QWORD PTR [rsp+0x8],rcx
            m.w64(rsp.wrapping_add(0x8), (rcx) as u64);
            // 126ea mov QWORD PTR [rsp],rax
            m.w64(rsp, (rax) as u64);
            // 126ee mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 126f2 mov QWORD PTR [rsp+0x20],rax
            m.w64(rsp.wrapping_add(0x20), (rax) as u64);
            // 126f7 mov rdi,r13
            rdi = r13;
            // 126fa mov rsi,r13
            rsi = r13;
            // 126fd mov rdx,rbx
            rdx = rbx;
            // 12700 mov rcx,rbx
            rcx = rbx;
            // 12703 mov r8,QWORD PTR [rbp-0x58]
            r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 12707 mov r9,r8
            r9 = r8;
            // 1270a call 0xeb8d0
            { let r = { let t = m.env.stub_va(0x9d); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1270f jmp 0x1272e
            pc = 0x1272e; continue;
        }
        0x12711 => {
            // block 12711
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12711 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12711 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 12715 mov r13,rbx
            r13 = rbx;
            pc = 0x12718;
        }
        0x12718 => {
            // block 12718
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12718 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12718 mov rdi,rbx
            rdi = rbx;
            // 1271b mov rsi,r13
            rsi = r13;
            // 1271e mov rdx,rax
            rdx = rax;
            // 12721 mov rcx,QWORD PTR [rbp-0x58]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 12725 mov r8,QWORD PTR [rbp-0x50]
            r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 12729 call 0xebf24
            { let r = { let t = m.env.stub_va(0x6f1); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1272e;
        }
        0x1272e => {
            // block 1272e
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 1272e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1272e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 12731 je 0x12749
            if fl.e() { pc = 0x12749; continue; }
            pc = 0x12733;
        }
        0x12733 => {
            // block 12733
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12733 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12733 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 12736 je 0x12749
            if fl.e() { pc = 0x12749; continue; }
            pc = 0x12738;
        }
        0x12738 => {
            // block 12738
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12738 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12738 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1273b mov rdi,rbx
            rdi = rbx;
            // 1273e call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12740 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12744 dec r14
            { let (r, f) = dec::<64>(r14, fl); r14 = r; fl = f; }
            // 12747 jne 0x12738
            if fl.ne() { pc = 0x12738; continue; }
            pc = 0x12749;
        }
        0x12749 => {
            // block 12749
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12749 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12749 add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 1274d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1274e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 12750 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 12752 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 12754 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 12756 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 12757 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x12758 => {
            // block 12758
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12758 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12758 jmp 0x1275a
            pc = 0x1275a; continue;
        }
        0x1275a => {
            // block 1275a
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 1275a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1275a mov r15,rax
            r15 = rax;
            // 1275d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 12760 je 0x12778
            if fl.e() { pc = 0x12778; continue; }
            pc = 0x12762;
        }
        0x12762 => {
            // block 12762
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12762 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12762 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 12765 je 0x12778
            if fl.e() { pc = 0x12778; continue; }
            pc = 0x12767;
        }
        0x12767 => {
            // block 12767
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12767 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12767 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1276a mov rdi,rbx
            rdi = rbx;
            // 1276d call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1276f add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12773 dec r14
            { let (r, f) = dec::<64>(r14, fl); r14 = r; fl = f; }
            // 12776 jne 0x12767
            if fl.ne() { pc = 0x12767; continue; }
            pc = 0x12778;
        }
        0x12778 => {
            // block 12778
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12778 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12778 mov rdi,r15
            rdi = r15;
            // 1277b call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12780 jmp 0x1275a
            pc = 0x1275a; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d18 size=358
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002d18<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2d18;
    loop {
        match pc {
        0x2d18 => {
            // block 2d18
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d18 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d19 mov rbp,rsp
            rbp = rsp;
            // 2d1c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2d1e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2d20 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d21 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2d22 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 2d24 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 2d27 mov r15,rdi
            r15 = rdi;
            // 2d2a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2d2d mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 2d2f call QWORD PTR [rax+0x1d30]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d35 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2d37 je 0x2d40
            if fl.e() { pc = 0x2d40; continue; }
            pc = 0x2d39;
        }
        0x2d39 => {
            // block 2d39
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d39 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2d3b jmp 0x2e72
            pc = 0x2e72; continue;
        }
        0x2d40 => {
            // block 2d40
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d40 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2d43 mov rdi,r15
            rdi = r15;
            // 2d46 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d4c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2d4e je 0x2d65
            if fl.e() { pc = 0x2d65; continue; }
            pc = 0x2d50;
        }
        0x2d50 => {
            // block 2d50
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d50 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2d53 movzx ecx,bx
            rcx = ((rbx as u16 as u64)) as u32 as u64;
            // 2d56 lea rdx,[rip+0xfc3e3] # 0xff140
            rdx = m.env.data_va(80141);
            // 2d5d mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2d60 jmp 0x2e61
            pc = 0x2e61; continue;
        }
        0x2d65 => {
            // block 2d65
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d65 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2d68 mov rdi,r15
            rdi = r15;
            // 2d6b call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d71 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 2d73 call 0xcc7a3
            { let r = { let t = m.env.code_va(0xcc7a3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d78 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2d7a je 0x2e59
            if fl.e() { pc = 0x2e59; continue; }
            pc = 0x2d80;
        }
        0x2d80 => {
            // block 2d80
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d80 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2d83 mov rdi,r15
            rdi = r15;
            // 2d86 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d8c mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 2d8e call 0xcf034
            { let r = { let t = m.env.code_va(0xcf034); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d93 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 2d96 je 0x2e40
            if fl.e() { pc = 0x2e40; continue; }
            pc = 0x2d9c;
        }
        0x2d9c => {
            // block 2d9c
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d9c cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 2d9f je 0x2e0e
            if fl.e() { pc = 0x2e0e; continue; }
            pc = 0x2da1;
        }
        0x2da1 => {
            // block 2da1
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2da1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2da1 cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // 2da4 jne 0x2e59
            if fl.ne() { pc = 0x2e59; continue; }
            pc = 0x2daa;
        }
        0x2daa => {
            // block 2daa
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2daa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2daa mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2dad mov esi,0x1e4
            rsi = (0x1e4u64) as u32 as u64;
            // 2db2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2db7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2dba mov rdi,r15
            rdi = r15;
            // 2dbd mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2dc0 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dc3 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2dc6 mov esi,0x1e3
            rsi = (0x1e3u64) as u32 as u64;
            // 2dcb mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2dd0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2dd3 mov rdi,r15
            rdi = r15;
            // 2dd6 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2dd9 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ddc mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2ddf mov esi,0x1e2
            rsi = (0x1e2u64) as u32 as u64;
            // 2de4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2de9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2dec mov rdi,r15
            rdi = r15;
            // 2def mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2df2 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2df5 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2df8 mov esi,0x1e1
            rsi = (0x1e1u64) as u32 as u64;
            // 2dfd mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2e02 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2e05 mov rdi,r15
            rdi = r15;
            // 2e08 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2e0b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e0e;
        }
        0x2e0e => {
            // block 2e0e
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2e0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e0e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e11 mov esi,0x1e0
            rsi = (0x1e0u64) as u32 as u64;
            // 2e16 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2e1b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2e1e mov rdi,r15
            rdi = r15;
            // 2e21 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2e24 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e27 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e2a mov esi,0x1df
            rsi = (0x1dfu64) as u32 as u64;
            // 2e2f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2e34 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2e37 mov rdi,r15
            rdi = r15;
            // 2e3a mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2e3d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e40;
        }
        0x2e40 => {
            // block 2e40
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2e40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e40 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e43 mov esi,0x1de
            rsi = (0x1deu64) as u32 as u64;
            // 2e48 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2e4d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2e50 mov rdi,r15
            rdi = r15;
            // 2e53 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2e56 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e59;
        }
        0x2e59 => {
            // block 2e59
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2e59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e59 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e5c mov esi,0xe
            rsi = (0xeu64) as u32 as u64;
            pc = 0x2e61;
        }
        0x2e61 => {
            // block 2e61
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2e61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e61 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2e66 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2e69 mov rdi,r15
            rdi = r15;
            // 2e6c mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2e6f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e72;
        }
        0x2e72 => {
            // block 2e72
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2e72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e72 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2e76 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e77 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e79 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e7b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e7c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd415c size=352
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d415c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd415c;
    loop {
        match pc {
        0xd415c => {
            // block d415c
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d415c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d415c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d415d mov rbp,rsp
            rbp = rsp;
            // d4160 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d4162 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d4164 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d4166 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d4168 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d4169 sub rsp,0x138
            { let (r, f) = sub::<64>(rsp, 0x138u64);
              rsp = r;
              fl = f; }
            // d4170 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // d4173 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d4176 mov r15,rdi
            r15 = rdi;
            // d4179 cmp r14d,0x7
            fl = sub::<32>((r14 as u32 as u64), 0x7u64).1;
            // d417d ja 0xd42a5
            if fl.a() { pc = 0xd42a5; continue; }
            pc = 0xd4183;
        }
        0xd4183 => {
            // block d4183
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d4183 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4183 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // d4186 mov eax,0x9
            rax = (0x9u64) as u32 as u64;
            // d418b mov ebx,0x2
            rbx = (0x2u64) as u32 as u64;
            // d4190 cmovne ebx,eax
            if fl.ne() { rbx = ((rax as u32 as u64)) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            // d4193 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d4196 call 0x5c6f2
            { let r = { let t = m.env.code_va(0x5c6f2); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d419b lea r13,[rbp-0x88]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // d41a2 mov rdi,r13
            rdi = r13;
            // d41a5 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // d41a7 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // d41a9 call 0x26090
            { let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d41ae mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d41b1 call 0x5c70e
            { let r = { let t = m.env.code_va(0x5c70e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d41b6 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // d41bd mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // d41bf mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // d41c1 call 0x26090
            { let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d41c6 mov DWORD PTR [rbp-0xec],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff14), (0x2u64) as u32);
            // d41d0 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // d41d7 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d41d9 call 0x2a0c4
            { let r = { let t = m.env.code_va(0x2a0c4); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d41de mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d41e1 mov rdi,r15
            rdi = r15;
            // d41e4 mov rsi,r13
            rsi = r13;
            // d41e7 call QWORD PTR [rax+0xc8]
            { let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d41ed mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d41f0 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d41f3 mov rdi,r15
            rdi = r15;
            // d41f6 lea rsi,[rbp-0xe8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // d41fd call QWORD PTR [rax+0xc8]
            { let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4203 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d4205 or bl,r13b
            { let r = (rbx as u8 as u64) | (r13 as u8 as u64); let f = logic::<8>(r);
              rbx = (rbx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d4208 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // d420b jne 0xd42a7
            if fl.ne() { pc = 0xd42a7; continue; }
            pc = 0xd4211;
        }
        0xd4211 => {
            // block d4211
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d4211 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4211 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d4213 xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d4215 jne 0xd42a7
            if fl.ne() { pc = 0xd42a7; continue; }
            pc = 0xd421b;
        }
        0xd421b => {
            // block d421b
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d421b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d421b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d421e lea rdx,[rbp-0xec]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff14);
            // d4225 mov rdi,r15
            rdi = r15;
            // d4228 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d422b call QWORD PTR [rax+0x370]
            { let r = { let t = m.r64(rax.wrapping_add(0x370)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4231 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d4233 mov eax,DWORD PTR [rbp-0xec]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff14)) as u64) as u32 as u64;
            // d4239 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d423c jne 0xd4255
            if fl.ne() { pc = 0xd4255; continue; }
            pc = 0xd423e;
        }
        0xd423e => {
            // block d423e
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d423e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d423e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d4241 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d4243 mov rdi,r15
            rdi = r15;
            // d4246 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d4249 call QWORD PTR [rax+0xbe8]
            { let r = { let t = m.r64(rax.wrapping_add(0xbe8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d424f mov eax,DWORD PTR [rbp-0xec]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff14)) as u64) as u32 as u64;
            pc = 0xd4255;
        }
        0xd4255 => {
            // block d4255
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d4255 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4255 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d4257 jne 0xd426a
            if fl.ne() { pc = 0xd426a; continue; }
            pc = 0xd4259;
        }
        0xd4259 => {
            // block d4259
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d4259 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4259 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d425c xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d425e mov rdi,r15
            rdi = r15;
            // d4261 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d4264 call QWORD PTR [rax+0xbd8]
            { let r = { let t = m.r64(rax.wrapping_add(0xbd8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd426a;
        }
        0xd426a => {
            // block d426a
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d426a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d426a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d426d lea rdx,[rbp-0x160]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // d4274 mov rdi,r15
            rdi = r15;
            // d4277 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d427a call QWORD PTR [rax+0xc60]
            { let r = { let t = m.r64(rax.wrapping_add(0xc60)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4280 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d4282 je 0xd42a7
            if fl.e() { pc = 0xd42a7; continue; }
            pc = 0xd4284;
        }
        0xd4284 => {
            // block d4284
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d4284 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4284 cmp DWORD PTR [rbp-0x13c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fec4)) as u64, 0x0u64).1;
            // d428b je 0xd42a7
            if fl.e() { pc = 0xd42a7; continue; }
            pc = 0xd428d;
        }
        0xd428d => {
            // block d428d
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d428d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d428d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d4290 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d4295 mov rdi,r15
            rdi = r15;
            // d4298 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d429b call QWORD PTR [rax+0xc38]
            { let r = { let t = m.r64(rax.wrapping_add(0xc38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d42a1 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d42a3 jmp 0xd42a7
            pc = 0xd42a7; continue;
        }
        0xd42a5 => {
            // block d42a5
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d42a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d42a5 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd42a7;
        }
        0xd42a7 => {
            // block d42a7
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d42a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d42a7 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d42a9 add rsp,0x138
            { let (r, f) = add::<64>(rsp, 0x138u64);
              rsp = r;
              fl = f; }
            // d42b0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d42b1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d42b3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d42b5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d42b7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d42b9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d42ba ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x69328 size=342
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00069328<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x69328;
    loop {
        match pc {
        0x69328 => {
            // block 69328
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69328 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69328 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 69329 mov rbp,rsp
            rbp = rsp;
            // 6932c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6932e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 69330 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 69332 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 69334 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 69335 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 69339 mov r12,rdx
            r12 = rdx;
            // 6933c mov r15,rsi
            r15 = rsi;
            // 6933f mov r14,rdi
            r14 = rdi;
            // 69342 lea rax,[r14+0x8]
            rax = r14.wrapping_add(0x8);
            // 69346 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 6934a mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // 69352 mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // 6935a mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 6935d lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 69364 mov rsi,r12
            rsi = r12;
            // 69367 call 0xebbd6
            { let r = { let t = m.env.stub_va(0x3a3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6936c mov rbx,rax
            rbx = rax;
            // 6936f add r15,0x1a0
            { let (r, f) = add::<64>(r15, 0x1a0u64);
              r15 = r;
              fl = f; }
            // 69376 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 69379 je 0x69454
            if fl.e() { pc = 0x69454; continue; }
            pc = 0x6937f;
        }
        0x6937f => {
            // block 6937f
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6937f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6937f lea rax,[r12+0x1]
            rax = r12.wrapping_add(0x1);
            // 69384 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            pc = 0x69388;
        }
        0x69388 => {
            // block 69388
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69388 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69388 movzx edx,BYTE PTR [rbx+0x20]
            rdx = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 6938c mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 6938e and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 69390 jne 0x69397
            if fl.ne() { pc = 0x69397; continue; }
            pc = 0x69392;
        }
        0x69392 => {
            // block 69392
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69392 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69392 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 69395 jmp 0x6939b
            pc = 0x6939b; continue;
        }
        0x69397 => {
            // block 69397
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69397 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69397 mov rdx,QWORD PTR [rbx+0x28]
            rdx = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x6939b;
        }
        0x6939b => {
            // block 6939b
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6939b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6939b movzx esi,BYTE PTR [r12]
            rsi = (m.r8(r12) as u64) as u32 as u64;
            // 693a0 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 693a3 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 693a6 jne 0x693ad
            if fl.ne() { pc = 0x693ad; continue; }
            pc = 0x693a8;
        }
        0x693a8 => {
            // block 693a8
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693a8 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 693ab jmp 0x693b2
            pc = 0x693b2; continue;
        }
        0x693ad => {
            // block 693ad
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693ad mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            pc = 0x693b2;
        }
        0x693b2 => {
            // block 693b2
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693b2 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 693b5 jne 0x69454
            if fl.ne() { pc = 0x69454; continue; }
            pc = 0x693bb;
        }
        0x693bb => {
            // block 693bb
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693bb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 693bd jne 0x693c8
            if fl.ne() { pc = 0x693c8; continue; }
            pc = 0x693bf;
        }
        0x693bf => {
            // block 693bf
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693bf lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // 693c3 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 693c6 jmp 0x693cc
            pc = 0x693cc; continue;
        }
        0x693c8 => {
            // block 693c8
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693c8 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x693cc;
        }
        0x693cc => {
            // block 693cc
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693cc test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 693ce mov rsi,QWORD PTR [r12+0x10]
            rsi = m.r64(r12.wrapping_add(0x10)) as u64;
            // 693d3 cmove rsi,QWORD PTR [rbp-0x30]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 693d8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 693da jne 0x693f8
            if fl.ne() { pc = 0x693f8; continue; }
            pc = 0x693dc;
        }
        0x693dc => {
            // block 693dc
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693dc test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 693df je 0x69406
            if fl.e() { pc = 0x69406; continue; }
            pc = 0x693e1;
        }
        0x693e1 => {
            // block 693e1
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693e1 movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // 693e4 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 693e7 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 693e9 jne 0x69454
            if fl.ne() { pc = 0x69454; continue; }
            pc = 0x693eb;
        }
        0x693eb => {
            // block 693eb
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693eb inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 693ee inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 693f1 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 693f4 jne 0x693e1
            if fl.ne() { pc = 0x693e1; continue; }
            pc = 0x693f6;
        }
        0x693f6 => {
            // block 693f6
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693f6 jmp 0x69406
            pc = 0x69406; continue;
        }
        0x693f8 => {
            // block 693f8
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693f8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 693fb je 0x69406
            if fl.e() { pc = 0x69406; continue; }
            pc = 0x693fd;
        }
        0x693fd => {
            // block 693fd
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693fd call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69402 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 69404 jne 0x69454
            if fl.ne() { pc = 0x69454; continue; }
            pc = 0x69406;
        }
        0x69406 => {
            // block 69406
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69406 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69406 lea r13,[rbx+0x38]
            r13 = rbx.wrapping_add(0x38);
            // 6940a mov rdi,r14
            rdi = r14;
            // 6940d mov rsi,r13
            rsi = r13;
            // 69410 call 0xebccc
            { let r = { let t = m.env.stub_va(0x499); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69415 cmp rax,QWORD PTR [rbp-0x38]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64).1;
            // 69419 jne 0x69426
            if fl.ne() { pc = 0x69426; continue; }
            pc = 0x6941b;
        }
        0x6941b => {
            // block 6941b
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6941b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6941b mov rdi,r14
            rdi = r14;
            // 6941e mov rsi,r13
            rsi = r13;
            // 69421 call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x69426;
        }
        0x69426 => {
            // block 69426
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69426 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69426 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 6942a test rax,rax
            fl = logic::<64>(rax & rax);
            // 6942d je 0x6943c
            if fl.e() { pc = 0x6943c; continue; }
            pc = 0x6942f;
        }
        0x6942f => {
            // block 6942f
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6942f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6942f mov rbx,rax
            rbx = rax;
            // 69432 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 69435 test rax,rax
            fl = logic::<64>(rax & rax);
            // 69438 jne 0x6942f
            if fl.ne() { pc = 0x6942f; continue; }
            pc = 0x6943a;
        }
        0x6943a => {
            // block 6943a
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6943a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6943a jmp 0x6944b
            pc = 0x6944b; continue;
        }
        0x6943c => {
            // block 6943c
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6943c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6943c mov rax,rbx
            rax = rbx;
            pc = 0x6943f;
        }
        0x6943f => {
            // block 6943f
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6943f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6943f mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 69443 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 69446 mov rax,rbx
            rax = rbx;
            // 69449 jne 0x6943f
            if fl.ne() { pc = 0x6943f; continue; }
            pc = 0x6944b;
        }
        0x6944b => {
            // block 6944b
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6944b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6944b cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 6944e jne 0x69388
            if fl.ne() { pc = 0x69388; continue; }
            pc = 0x69454;
        }
        0x69454 => {
            // block 69454
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69454 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69454 mov rax,r14
            rax = r14;
            // 69457 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 6945b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6945c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 6945e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 69460 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 69462 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 69464 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 69465 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x69466 => {
            // block 69466
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69466 mov rbx,rax
            rbx = rax;
            // 69469 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 6946d mov rdi,r14
            rdi = r14;
            // 69470 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69475 mov rdi,rbx
            rdi = rbx;
            // 69478 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4444 size=340
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4444<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd4444;
    loop {
        match pc {
        0xd4444 => {
            // block d4444
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d4444 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4444 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4445 mov rbp,rsp
            rbp = rsp;
            // d4448 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d444a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d444c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d444e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d444f sub rsp,0xd0
            { let (r, f) = sub::<64>(rsp, 0xd0u64);
              rsp = r;
              fl = f; }
            // d4456 mov r15,rdx
            r15 = rdx;
            // d4459 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d445c mov r12,rdi
            r12 = rdi;
            // d445f mov rdi,r15
            rdi = r15;
            // d4462 call 0x2a25e
            { let r = { let t = m.env.code_va(0x2a25e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4467 lea rbx,[r15+0x20]
            rbx = r15.wrapping_add(0x20);
            // d446b mov rdi,r12
            rdi = r12;
            // d446e mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d4471 mov rdx,rbx
            rdx = rbx;
            // d4474 call 0xd40e0
            { let r = { let t = m.env.code_va(0xd40e0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4479 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d447b je 0xd455c
            if fl.e() { pc = 0xd455c; continue; }
            pc = 0xd4481;
        }
        0xd4481 => {
            // block d4481
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d4481 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4481 mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // d4483 cmp eax,0x11
            fl = sub::<32>((rax as u32 as u64), 0x11u64).1;
            // d4486 ja 0xd4560
            if fl.a() { pc = 0xd4560; continue; }
            pc = 0xd448c;
        }
        0xd448c => {
            // block d448c
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d448c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d448c mov ecx,0x3ffcf
            rcx = (0x3ffcfu64) as u32 as u64;
            // d4491 bt ecx,eax
            fl.cf = bit::<32>((rcx as u32 as u64), (rax as u32 as u64));
            // d4494 jae 0xd4560
            if fl.ae() { pc = 0xd4560; continue; }
            pc = 0xd449a;
        }
        0xd449a => {
            // block d449a
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d449a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d449a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d449e mov rdi,r12
            rdi = r12;
            // d44a1 mov rsi,r15
            rsi = r15;
            // d44a4 call QWORD PTR [rax+0xd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xd0)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d44aa mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d44ac mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d44ae test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d44b0 jne 0xd4588
            if fl.ne() { pc = 0xd4588; continue; }
            pc = 0xd44b6;
        }
        0xd44b6 => {
            // block d44b6
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d44b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d44b6 xorps xmm0,xmm0
            x0 = 0;
            // d44b9 movaps XMMWORD PTR [rbp-0xb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff50), x0);
            // d44c0 movaps XMMWORD PTR [rbp-0xc0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff40), x0);
            // d44c7 movaps XMMWORD PTR [rbp-0xd0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff30), x0);
            // d44ce movaps XMMWORD PTR [rbp-0xe0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff20), x0);
            // d44d5 movaps XMMWORD PTR [rbp-0xf0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff10), x0);
            // d44dc mov DWORD PTR [rbp-0x98],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0x0u64) as u32);
            // d44e6 mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // d44f1 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d44f4 call 0x5c6f2
            { let r = { let t = m.env.code_va(0x5c6f2); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d44f9 mov DWORD PTR [rbp-0xf0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), ((rax as u32 as u64)) as u32);
            // d44ff mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // d4503 lea rdx,[rbp-0xf0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // d450a mov rdi,r12
            rdi = r12;
            // d450d mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d450f call QWORD PTR [rcx+0xc18]
            { let r = { let t = m.r64(rcx.wrapping_add(0xc18)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4515 cmp DWORD PTR [rbp-0xec],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff14)) as u64, 0x0u64).1;
            // d451c jne 0xd454b
            if fl.ne() { pc = 0xd454b; continue; }
            pc = 0xd451e;
        }
        0xd451e => {
            // block d451e
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d451e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d451e mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d4521 call 0x5c70e
            { let r = { let t = m.env.code_va(0x5c70e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4526 mov DWORD PTR [rbp-0xf0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), ((rax as u32 as u64)) as u32);
            // d452c mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // d4530 lea rdx,[rbp-0xf0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // d4537 mov rdi,r12
            rdi = r12;
            // d453a mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d453c call QWORD PTR [rcx+0xc18]
            { let r = { let t = m.r64(rcx.wrapping_add(0xc18)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4542 cmp DWORD PTR [rbp-0xec],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff14)) as u64, 0x0u64).1;
            // d4549 je 0xd4560
            if fl.e() { pc = 0xd4560; continue; }
            pc = 0xd454b;
        }
        0xd454b => {
            // block d454b
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d454b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d454b lea rsi,[rbp-0xf0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // d4552 mov rdi,r15
            rdi = r15;
            // d4555 call 0x2a1d8
            { let r = { let t = m.env.code_va(0x2a1d8); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d455a jmp 0xd4588
            pc = 0xd4588; continue;
        }
        0xd455c => {
            // block d455c
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d455c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d455c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d455e jmp 0xd4588
            pc = 0xd4588; continue;
        }
        0xd4560 => {
            // block d4560
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d4560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4560 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d4563 call 0x5c70e
            { let r = { let t = m.env.code_va(0x5c70e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4568 lea rbx,[rbp-0x90]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d456f mov rdi,rbx
            rdi = rbx;
            // d4572 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d4574 call 0x2a0c4
            { let r = { let t = m.env.code_va(0x2a0c4); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4579 mov ecx,0x1c
            rcx = (0x1cu64) as u32 as u64;
            // d457e mov rdi,r15
            rdi = r15;
            // d4581 mov rsi,rbx
            rsi = rbx;
            // d4584 rep movs DWORD PTR [rdi],DWORD PTR [rsi]
            while rcx != 0 { let v = m.r32(rsi); m.w32(rdi, v); rsi = rsi.wrapping_add(4); rdi = rdi.wrapping_add(4); rcx = rcx.wrapping_sub(1); }
            // d4586 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xd4588;
        }
        0xd4588 => {
            // block d4588
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d4588 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4588 add rsp,0xd0
            { let (r, f) = add::<64>(rsp, 0xd0u64);
              rsp = r;
              fl = f; }
            // d458f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4590 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d4592 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d4594 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d4596 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4597 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5855b size=335
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005855b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5855b;
    loop {
        match pc {
        0x5855b => {
            // block 5855b
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5855b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5855b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5855c mov rbp,rsp
            rbp = rsp;
            // 5855f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 58561 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 58563 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 58565 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 58567 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 58568 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 58569 mov rbx,rsi
            rbx = rsi;
            // 5856c mov r14,rdi
            r14 = rdi;
            // 5856f mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 58572 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 58576 cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // 58579 je 0x58593
            if fl.e() { pc = 0x58593; continue; }
            pc = 0x5857b;
        }
        0x5857b => {
            // block 5857b
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5857b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5857b lea rsi,[rcx-0x2]
            rsi = rcx.wrapping_add(0xffff_ffff_ffff_fffe);
            // 5857f sub rsi,rdi
            { let (r, f) = sub::<64>(rsi, rdi);
              rsi = r;
              fl = f; }
            // 58582 not rsi
            rsi = !rsi;
            // 58585 and rsi,0xfffffffffffffffe
            { let r = rsi & 0xffff_ffff_ffff_fffeu64; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // 58589 add rsi,rcx
            { let (r, f) = add::<64>(rsi, rcx);
              rsi = r;
              fl = f; }
            // 5858c mov QWORD PTR [rbx+0x8],rsi
            m.w64(rbx.wrapping_add(0x8), (rsi) as u64);
            // 58590 mov rdi,rsi
            rdi = rsi;
            pc = 0x58593;
        }
        0x58593 => {
            // block 58593
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58593 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58593 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58595 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 58598 je 0x5869b
            if fl.e() { pc = 0x5869b; continue; }
            pc = 0x5859e;
        }
        0x5859e => {
            // block 5859e
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5859e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5859e cmp edx,0x6
            fl = sub::<32>((rdx as u32 as u64), 0x6u64).1;
            // 585a1 jb 0x5869b
            if fl.b() { pc = 0x5869b; continue; }
            pc = 0x585a7;
        }
        0x585a7 => {
            // block 585a7
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 585a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 585a7 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 585a9 mov edx,0xaaaaaaab
            rdx = (0xaaaaaaabu64) as u32 as u64;
            // 585ae imul rdx,rcx
            { let (r, f) = imul::<64>(rdx, rcx); rdx = r; fl = f; }
            // 585b2 shr rdx,0x22
            { let (r, f) = shr::<64>(rdx, 0x22u64, fl); rdx = r; fl = f; }
            // 585b6 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 585b8 lea ecx,[rdx+rdx*2]
            rcx = (rdx.wrapping_add(rdx.wrapping_mul(2))) as u32 as u64;
            // 585bb shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 585be mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 585c0 cmp ecx,0x6
            fl = sub::<32>((rcx as u32 as u64), 0x6u64).1;
            // 585c3 jb 0x5869b
            if fl.b() { pc = 0x5869b; continue; }
            pc = 0x585c9;
        }
        0x585c9 => {
            // block 585c9
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 585c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 585c9 mov ecx,ecx
            rcx = ((rcx as u32 as u64)) as u32 as u64;
            // 585cb mov r12d,0xaaaaaaab
            r12 = (0xaaaaaaabu64) as u32 as u64;
            // 585d1 imul r12,rcx
            { let (r, f) = imul::<64>(r12, rcx); r12 = r; fl = f; }
            // 585d5 shr r12,0x22
            { let (r, f) = shr::<64>(r12, 0x22u64, fl); r12 = r; fl = f; }
            // 585d9 mov r13d,0x1
            r13 = (0x1u64) as u32 as u64;
            // 585df lea r15,[rbp-0x2a]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd6);
            // 585e3 jmp 0x585ec
            pc = 0x585ec; continue;
        }
        0x585e5 => {
            // block 585e5
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 585e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 585e5 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 585e9 inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            pc = 0x585ec;
        }
        0x585ec => {
            // block 585ec
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 585ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 585ec mov ecx,DWORD PTR [r14+r13*4-0x4]
            rcx = (m.r32(r14.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 585f1 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 585f3 and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 585f9 mov WORD PTR [rbp-0x2a],si
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd6), ((rsi as u16 as u64)) as u16);
            // 585fd mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 58601 cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // 58604 jae 0x58613
            if fl.ae() { pc = 0x58613; continue; }
            pc = 0x58606;
        }
        0x58606 => {
            // block 58606
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58606 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58606 mov WORD PTR [rdi],si
            m.w16(rdi, ((rsi as u16 as u64)) as u16);
            // 58609 add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // 5860d mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // 58611 jmp 0x5862b
            pc = 0x5862b; continue;
        }
        0x58613 => {
            // block 58613
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58613 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58613 mov rdi,rbx
            rdi = rbx;
            // 58616 mov rsi,r15
            rsi = r15;
            // 58619 call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5861e mov ecx,DWORD PTR [r14+r13*4-0x4]
            rcx = (m.r32(r14.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 58623 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 58627 mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0x5862b;
        }
        0x5862b => {
            // block 5862b
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5862b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5862b mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 5862d shr esi,0xa
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xau64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 58630 and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 58636 mov WORD PTR [rbp-0x2c],si
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rsi as u16 as u64)) as u16);
            // 5863a cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // 5863d jae 0x5864c
            if fl.ae() { pc = 0x5864c; continue; }
            pc = 0x5863f;
        }
        0x5863f => {
            // block 5863f
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5863f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5863f mov WORD PTR [rdi],si
            m.w16(rdi, ((rsi as u16 as u64)) as u16);
            // 58642 add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // 58646 mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // 5864a jmp 0x58665
            pc = 0x58665; continue;
        }
        0x5864c => {
            // block 5864c
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5864c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5864c mov rdi,rbx
            rdi = rbx;
            // 5864f lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 58653 call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58658 mov ecx,DWORD PTR [r14+r13*4-0x4]
            rcx = (m.r32(r14.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 5865d mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 58661 mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0x58665;
        }
        0x58665 => {
            // block 58665
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58665 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58665 shr ecx,0x14
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x14u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 58668 and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5866e mov WORD PTR [rbp-0x2e],cx
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd2), ((rcx as u16 as u64)) as u16);
            // 58672 cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // 58675 jae 0x58684
            if fl.ae() { pc = 0x58684; continue; }
            pc = 0x58677;
        }
        0x58677 => {
            // block 58677
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58677 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58677 mov WORD PTR [rdi],cx
            m.w16(rdi, ((rcx as u16 as u64)) as u16);
            // 5867a add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // 5867e mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // 58682 jmp 0x58690
            pc = 0x58690; continue;
        }
        0x58684 => {
            // block 58684
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58684 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58684 mov rdi,rbx
            rdi = rbx;
            // 58687 lea rsi,[rbp-0x2e]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd2);
            // 5868b call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x58690;
        }
        0x58690 => {
            // block 58690
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58690 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58690 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 58693 jb 0x585e5
            if fl.b() { pc = 0x585e5; continue; }
            pc = 0x58699;
        }
        0x58699 => {
            // block 58699
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58699 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58699 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x5869b;
        }
        0x5869b => {
            // block 5869b
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5869b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5869b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5869f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 586a0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 586a2 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 586a4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 586a6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 586a8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 586a9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x95bc size=320
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000095bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x95bc;
    loop {
        match pc {
        0x95bc => {
            // block 95bc
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 95bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 95bd mov rbp,rsp
            rbp = rsp;
            // 95c0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 95c2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 95c3 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 95c7 mov r14,rsi
            r14 = rsi;
            // 95ca mov rbx,rdi
            rbx = rdi;
            // 95cd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 95d0 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 95d4 call QWORD PTR [rax+0xde0]
            { let r = { let t = m.r64(rax.wrapping_add(0xde0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 95da mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 95dd lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 95e1 mov rdi,rbx
            rdi = rbx;
            // 95e4 call QWORD PTR [rax+0xdc0]
            { let r = { let t = m.r64(rax.wrapping_add(0xdc0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 95ea mov DWORD PTR [r14],0x25
            m.w32(r14, (0x25u64) as u32);
            // 95f1 mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 95f4 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // 95f8 jbe 0x9605
            if fl.be() { pc = 0x9605; continue; }
            pc = 0x95fa;
        }
        0x95fa => {
            // block 95fa
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 95fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95fa xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x95fc;
        }
        0x95fc => {
            // block 95fc
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 95fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95fc add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 9600 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 9601 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 9603 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 9604 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x9605 => {
            // block 9605
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9605 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9605 lea rcx,[rip+0xdc] # 0x96e8
            rcx = m.env.code_va(0x96e8);
            // 960c movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 9610 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 9613 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x9615 => {
            // block 9615
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9615 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9615 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 9617 mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 961a cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 961d jne 0x967c
            if fl.ne() { pc = 0x967c; continue; }
            pc = 0x961f;
        }
        0x961f => {
            // block 961f
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 961f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 961f mov DWORD PTR [r14],0x1
            m.w32(r14, (0x1u64) as u32);
            // 9626 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x9628 => {
            // block 9628
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9628 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9628 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 962a mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 962d cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 9630 jne 0x9691
            if fl.ne() { pc = 0x9691; continue; }
            pc = 0x9632;
        }
        0x9632 => {
            // block 9632
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9632 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9632 mov DWORD PTR [r14],0x3
            m.w32(r14, (0x3u64) as u32);
            // 9639 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x963b => {
            // block 963b
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 963b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 963b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 963d mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 9640 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 9643 je 0x96a6
            if fl.e() { pc = 0x96a6; continue; }
            pc = 0x9645;
        }
        0x9645 => {
            // block 9645
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9645 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9645 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // 9648 jne 0x96be
            if fl.ne() { pc = 0x96be; continue; }
            pc = 0x964a;
        }
        0x964a => {
            // block 964a
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 964a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 964a mov DWORD PTR [r14],0x5
            m.w32(r14, (0x5u64) as u32);
            // 9651 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x9653 => {
            // block 9653
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9653 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9653 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 9655 mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 9658 cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 965b je 0x96b2
            if fl.e() { pc = 0x96b2; continue; }
            pc = 0x965d;
        }
        0x965d => {
            // block 965d
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 965d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 965d cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // 9660 jne 0x96d2
            if fl.ne() { pc = 0x96d2; continue; }
            pc = 0x9662;
        }
        0x9662 => {
            // block 9662
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9662 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9662 mov DWORD PTR [r14],0x7
            m.w32(r14, (0x7u64) as u32);
            // 9669 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x966b => {
            // block 966b
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 966b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 966b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 966d cmp DWORD PTR [rbp-0x18],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64, 0x1u64).1;
            // 9671 jne 0x95fc
            if fl.ne() { pc = 0x95fc; continue; }
            pc = 0x9673;
        }
        0x9673 => {
            // block 9673
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9673 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9673 mov DWORD PTR [r14],0x1e
            m.w32(r14, (0x1eu64) as u32);
            // 967a jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x967c => {
            // block 967c
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 967c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 967c cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 967f jne 0x95fc
            if fl.ne() { pc = 0x95fc; continue; }
            pc = 0x9685;
        }
        0x9685 => {
            // block 9685
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9685 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9685 mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            // 968c jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x9691 => {
            // block 9691
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9691 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9691 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 9694 jne 0x95fc
            if fl.ne() { pc = 0x95fc; continue; }
            pc = 0x969a;
        }
        0x969a => {
            // block 969a
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 969a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 969a mov DWORD PTR [r14],0x2
            m.w32(r14, (0x2u64) as u32);
            // 96a1 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x96a6 => {
            // block 96a6
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96a6 mov DWORD PTR [r14],0x17
            m.w32(r14, (0x17u64) as u32);
            // 96ad jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x96b2 => {
            // block 96b2
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96b2 mov DWORD PTR [r14],0x18
            m.w32(r14, (0x18u64) as u32);
            // 96b9 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x96be => {
            // block 96be
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96be test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 96c0 jne 0x95fc
            if fl.ne() { pc = 0x95fc; continue; }
            pc = 0x96c6;
        }
        0x96c6 => {
            // block 96c6
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96c6 mov DWORD PTR [r14],0x4
            m.w32(r14, (0x4u64) as u32);
            // 96cd jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x96d2 => {
            // block 96d2
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96d2 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 96d4 jne 0x95fc
            if fl.ne() { pc = 0x95fc; continue; }
            pc = 0x96da;
        }
        0x96da => {
            // block 96da
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96da mov DWORD PTR [r14],0x6
            m.w32(r14, (0x6u64) as u32);
            // 96e1 jmp 0x95fc
            pc = 0x95fc; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe3f00 size=312
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e3f00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe3f00;
    loop {
        match pc {
        0xe3f00 => {
            // block e3f00
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e3f01 mov rbp,rsp
            rbp = rsp;
            // e3f04 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e3f06 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e3f08 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e3f0a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e3f0c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e3f0d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e3f0e mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // e3f11 mov r12,rsi
            r12 = rsi;
            // e3f14 mov r14,rdi
            r14 = rdi;
            // e3f17 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e3f1b mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // e3f20 mov rsi,r14
            rsi = r14;
            // e3f23 call 0xec050
            { let r = { let t = m.env.stub_va(0x81d); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3f28 cmp BYTE PTR [rbp-0x30],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, 0x0u64).1;
            // e3f2c je 0xe400e
            if fl.e() { pc = 0xe400e; continue; }
            pc = 0xe3f32;
        }
        0xe3f32 => {
            // block e3f32
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f32 test BYTE PTR [r12],0x1
            fl = logic::<8>(m.r8(r12) as u64 & 0x1u64);
            // e3f37 jne 0xe3f47
            if fl.ne() { pc = 0xe3f47; continue; }
            pc = 0xe3f39;
        }
        0xe3f39 => {
            // block e3f39
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f39 mov WORD PTR [r12],0x0
            m.w16(r12, (0x0u64) as u16);
            // e3f40 lea r13,[r12+0x8]
            r13 = r12.wrapping_add(0x8);
            // e3f45 jmp 0xe3f5d
            pc = 0xe3f5d; continue;
        }
        0xe3f47 => {
            // block e3f47
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f47 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // e3f4c mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // e3f4f lea r13,[r12+0x8]
            r13 = r12.wrapping_add(0x8);
            // e3f54 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xe3f5d;
        }
        0xe3f5d => {
            // block e3f5d
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f5d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e3f5f movzx r15d,r15b
            r15 = ((r15 as u8 as u64)) as u32 as u64;
            pc = 0xe3f63;
        }
        0xe3f63 => {
            // block e3f63
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f63 mov rbx,rax
            rbx = rax;
            // e3f66 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e3f69 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e3f6d mov rdi,QWORD PTR [rax+r14*1+0x28]
            rdi = m.r64(rax.wrapping_add(r14).wrapping_add(0x28)) as u64;
            // e3f72 mov rax,QWORD PTR [rdi+0x18]
            rax = m.r64(rdi.wrapping_add(0x18)) as u64;
            // e3f76 cmp rax,QWORD PTR [rdi+0x20]
            fl = sub::<64>(rax, m.r64(rdi.wrapping_add(0x20)) as u64).1;
            // e3f7a je 0xe3f89
            if fl.e() { pc = 0xe3f89; continue; }
            pc = 0xe3f7c;
        }
        0xe3f7c => {
            // block e3f7c
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f7c lea rcx,[rax+0x1]
            rcx = rax.wrapping_add(0x1);
            // e3f80 mov QWORD PTR [rdi+0x18],rcx
            m.w64(rdi.wrapping_add(0x18), (rcx) as u64);
            // e3f84 movzx eax,BYTE PTR [rax]
            rax = (m.r8(rax) as u64) as u32 as u64;
            // e3f87 jmp 0xe3f9c
            pc = 0xe3f9c; continue;
        }
        0xe3f89 => {
            // block e3f89
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f89 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e3f8c mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // e3f90 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3f92 mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // e3f97 cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // e3f9a je 0xe3fcb
            if fl.e() { pc = 0xe3fcb; continue; }
            pc = 0xe3f9c;
        }
        0xe3f9c => {
            // block e3f9c
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f9c movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // e3f9f xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e3fa1 cmp edx,r15d
            fl = sub::<32>((rdx as u32 as u64), (r15 as u32 as u64)).1;
            // e3fa4 je 0xe3fc8
            if fl.e() { pc = 0xe3fc8; continue; }
            pc = 0xe3fa6;
        }
        0xe3fa6 => {
            // block e3fa6
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3fa6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3fa6 movsx esi,al
            rsi = (sx::<8>((rax as u8 as u64)) as u64) as u32 as u64;
            // e3fa9 mov rdi,r12
            rdi = r12;
            // e3fac call 0xec020
            { let r = { let t = m.env.stub_va(0x7ed); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3fb1 lea rax,[rbx+0x1]
            rax = rbx.wrapping_add(0x1);
            // e3fb5 test BYTE PTR [r12],0x1
            fl = logic::<8>(m.r8(r12) as u64 & 0x1u64);
            // e3fba je 0xe3f63
            if fl.e() { pc = 0xe3f63; continue; }
            pc = 0xe3fbc;
        }
        0xe3fbc => {
            // block e3fbc
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3fbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3fbc mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // e3fc1 cmp QWORD PTR [r13+0x0],0xffffffffffffffef
            fl = sub::<64>(m.r64(r13) as u64, 0xffff_ffff_ffff_ffefu64).1;
            // e3fc6 jne 0xe3f63
            if fl.ne() { pc = 0xe3f63; continue; }
            pc = 0xe3fc8;
        }
        0xe3fc8 => {
            // block e3fc8
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3fc8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3fc8 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            pc = 0xe3fcb;
        }
        0xe3fcb => {
            // block e3fcb
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3fcb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3fcb mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // e3fcd or esi,0x4
            { let r = (rsi as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e3fd0 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // e3fd3 cmovne esi,ecx
            if fl.ne() { rsi = ((rcx as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // e3fd6 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e3fd9 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e3fdd lea rdi,[r14+rax*1]
            rdi = r14.wrapping_add(rax);
            // e3fe1 or esi,DWORD PTR [r14+rax*1+0x20]
            { let r = (rsi as u32 as u64) | m.r32(r14.wrapping_add(rax).wrapping_add(0x20)) as u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e3fe6 call 0xec0fe
            { let r = { let t = m.env.stub_va(0x8cb); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3feb jmp 0xe400e
            pc = 0xe400e; continue;
        }
        0xe3fed => {
            // block e3fed
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3fed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3fed jmp 0xe3fef
            pc = 0xe3fef; continue;
        }
        0xe3fef => {
            // block e3fef
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3fef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3fef mov rbx,r14
            rbx = r14;
            // e3ff2 mov rdi,rax
            rdi = rax;
            // e3ff5 call 0xec158
            { let r = { let t = m.env.stub_va(0x925); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3ffa mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e3ffd add rbx,QWORD PTR [rax-0x18]
            { let (r, f) = add::<64>(rbx, m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64);
              rbx = r;
              fl = f; }
            // e4001 mov rdi,rbx
            rdi = rbx;
            // e4004 call 0xec0f2
            { let r = { let t = m.env.stub_va(0x8bf); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4009 call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe400e;
        }
        0xe400e => {
            // block e400e
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e400e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e400e mov rax,r14
            rax = r14;
            // e4011 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e4015 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4016 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e4018 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e401a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e401c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e401e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e401f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe4020 => {
            // block e4020
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e4020 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4020 mov rbx,rax
            rbx = rax;
            // e4023 call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4028 mov rdi,rbx
            rdi = rbx;
            // e402b call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4030 mov rdi,rax
            rdi = rax;
            // e4033 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7b00 size=304
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007b00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x7b00;
    loop {
        match pc {
        0x7b00 => {
            // block 7b00
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7b01 mov rbp,rsp
            rbp = rsp;
            // 7b04 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7b06 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7b08 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 7b0a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 7b0c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7b0d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7b11 mov r15,rsi
            r15 = rsi;
            // 7b14 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 7b18 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 7b1b je 0x7c1b
            if fl.e() { pc = 0x7c1b; continue; }
            pc = 0x7b21;
        }
        0x7b21 => {
            // block 7b21
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b21 add r15,0x8
            { let (r, f) = add::<64>(r15, 0x8u64);
              r15 = r;
              fl = f; }
            // 7b25 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 7b2a mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 7b2e mov r14,QWORD PTR [rbp-0x38]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x7b32;
        }
        0x7b32 => {
            // block 7b32
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b32 movsd xmm0,QWORD PTR [r15-0x8]
            x0 = m.r64(r15.wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7b38 addsd xmm0,QWORD PTR [rip+0xf6d98] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 7b40 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 7b45 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 7b47 mov ebx,0x0
            rbx = (0x0u64) as u32 as u64;
            // 7b4c js 0x7b5d
            if fl.s() { pc = 0x7b5d; continue; }
            pc = 0x7b4e;
        }
        0x7b4e => {
            // block 7b4e
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b4e cmp eax,0x3ff
            fl = sub::<32>((rax as u32 as u64), 0x3ffu64).1;
            // 7b53 mov ecx,0x3ff
            rcx = (0x3ffu64) as u32 as u64;
            // 7b58 cmovg eax,ecx
            if fl.g() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 7b5b mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x7b5d;
        }
        0x7b5d => {
            // block 7b5d
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b5d mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7b61 lea r12,[rax+0x1ff]
            r12 = rax.wrapping_add(0x1ff);
            // 7b68 movsd xmm0,QWORD PTR [r15]
            x0 = m.r64(r15) as u128;
            // 7b6d addsd xmm0,QWORD PTR [rip+0xf6d63] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 7b75 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7b7a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7b7c test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7b7e js 0x7b90
            if fl.s() { pc = 0x7b90; continue; }
            pc = 0x7b80;
        }
        0x7b80 => {
            // block 7b80
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b80 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7b86 mov eax,0x3ff
            rax = (0x3ffu64) as u32 as u64;
            // 7b8b cmovg ecx,eax
            if fl.g() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7b8e mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7b90;
        }
        0x7b90 => {
            // block 7b90
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b90 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7b93 shl ebx,0x6
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x6u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 7b96 add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // 7b98 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7b9b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 7b9e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7ba3 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7ba6 mov rdi,r14
            rdi = r14;
            // 7ba9 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bac mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bae call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bb1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7bb3 je 0x7c1e
            if fl.e() { pc = 0x7c1e; continue; }
            pc = 0x7bb5;
        }
        0x7bb5 => {
            // block 7bb5
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7bb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bb5 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7bb8 add r12,0x200
            { let (r, f) = add::<64>(r12, 0x200u64);
              r12 = r;
              fl = f; }
            // 7bbf xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 7bc2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7bc7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7bca mov rdi,r14
            rdi = r14;
            // 7bcd mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bd0 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bd2 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bd5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7bd7 je 0x7c1e
            if fl.e() { pc = 0x7c1e; continue; }
            pc = 0x7bd9;
        }
        0x7bd9 => {
            // block 7bd9
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7bd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bd9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7bdc add r12d,0x200
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x200u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 7be3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7be8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7beb mov rdi,r14
            rdi = r14;
            // 7bee mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bf1 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bf3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bf6 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 7bf9 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7bfd cmp rax,0x1ff
            fl = sub::<64>(rax, 0x1ffu64).1;
            // 7c03 ja 0x7c1e
            if fl.a() { pc = 0x7c1e; continue; }
            pc = 0x7c05;
        }
        0x7c05 => {
            // block 7c05
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c05 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // 7c09 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 7c0c mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 7c10 test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // 7c13 jne 0x7b32
            if fl.ne() { pc = 0x7b32; continue; }
            pc = 0x7c19;
        }
        0x7c19 => {
            // block 7c19
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c19 jmp 0x7c1e
            pc = 0x7c1e; continue;
        }
        0x7c1b => {
            // block 7c1b
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c1b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x7c1e;
        }
        0x7c1e => {
            // block 7c1e
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c1e mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 7c21 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7c25 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7c26 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 7c28 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 7c2a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 7c2c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 7c2e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7c2f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x65094 size=302
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00065094<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x65094;
    loop {
        match pc {
        0x65094 => {
            // block 65094
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 65094 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65094 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 65095 mov rbp,rsp
            rbp = rsp;
            // 65098 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6509a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6509c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 6509e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 650a0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 650a1 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 650a5 mov r14,rcx
            r14 = rcx;
            // 650a8 mov rbx,rdx
            rbx = rdx;
            // 650ab mov r13,rdi
            r13 = rdi;
            // 650ae xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 650b0 test r13,r13
            fl = logic::<64>(r13 & r13);
            // 650b3 je 0x6519f
            if fl.e() { pc = 0x6519f; continue; }
            pc = 0x650b9;
        }
        0x650b9 => {
            // block 650b9
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 650b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 650b9 mov rax,r14
            rax = r14;
            // 650bc sub rax,rsi
            { let (r, f) = sub::<64>(rax, rsi);
              rax = r;
              fl = f; }
            // 650bf mov rcx,QWORD PTR [r8+0x18]
            rcx = m.r64(r8.wrapping_add(0x18)) as u64;
            // 650c3 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 650c6 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 650c9 cmovg r15,rcx
            if fl.g() { r15 = rcx; }
            // 650cd mov r12,rbx
            r12 = rbx;
            // 650d0 sub r12,rsi
            { let (r, f) = sub::<64>(r12, rsi);
              r12 = r;
              fl = f; }
            // 650d3 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 650d6 jle 0x6510f
            if fl.le() { pc = 0x6510f; continue; }
            pc = 0x650d8;
        }
        0x650d8 => {
            // block 650d8
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 650d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 650d8 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 650dc mov rdi,r13
            rdi = r13;
            // 650df mov rdx,r12
            rdx = r12;
            // 650e2 mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // 650e6 mov QWORD PTR [rbp-0x48],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r14) as u64);
            // 650ea mov rbx,r8
            rbx = r8;
            // 650ed mov r14d,r9d
            r14 = ((r9 as u32 as u64)) as u32 as u64;
            // 650f0 call QWORD PTR [rax+0x60]
            { let r = { let t = m.r64(rax.wrapping_add(0x60)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 650f3 mov r9d,r14d
            r9 = ((r14 as u32 as u64)) as u32 as u64;
            // 650f6 mov r8,rbx
            r8 = rbx;
            // 650f9 mov r14,QWORD PTR [rbp-0x48]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 650fd mov rbx,QWORD PTR [rbp-0x50]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 65101 mov rcx,rax
            rcx = rax;
            // 65104 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 65106 cmp rcx,r12
            fl = sub::<64>(rcx, r12).1;
            // 65109 jne 0x6519f
            if fl.ne() { pc = 0x6519f; continue; }
            pc = 0x6510f;
        }
        0x6510f => {
            // block 6510f
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 6510f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6510f test r15,r15
            fl = logic::<64>(r15 & r15);
            // 65112 jle 0x6516c
            if fl.le() { pc = 0x6516c; continue; }
            pc = 0x65114;
        }
        0x65114 => {
            // block 65114
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 65114 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65114 mov QWORD PTR [rbp-0x48],r8
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r8) as u64);
            // 65118 movsx edx,r9b
            rdx = (sx::<8>((r9 as u8 as u64)) as u64) as u32 as u64;
            // 6511c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 65120 mov rsi,r15
            rsi = r15;
            // 65123 call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65128 test BYTE PTR [rbp-0x40],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64 & 0x1u64);
            // 6512c jne 0x65137
            if fl.ne() { pc = 0x65137; continue; }
            pc = 0x6512e;
        }
        0x6512e => {
            // block 6512e
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 6512e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6512e mov r12,r14
            r12 = r14;
            // 65131 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 65135 jmp 0x6513e
            pc = 0x6513e; continue;
        }
        0x65137 => {
            // block 65137
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 65137 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65137 mov r12,r14
            r12 = r14;
            // 6513a mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x6513e;
        }
        0x6513e => {
            // block 6513e
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 6513e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6513e mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 65142 mov rax,QWORD PTR [rax+0x60]
            rax = m.r64(rax.wrapping_add(0x60)) as u64;
            // 65146 mov rdi,r13
            rdi = r13;
            // 65149 mov rdx,r15
            rdx = r15;
            // 6514c call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6514e mov r14,rax
            r14 = rax;
            // 65151 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 65155 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6515a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6515c cmp r14,r15
            fl = sub::<64>(r14, r15).1;
            // 6515f cmovne r13,rax
            if fl.ne() { r13 = rax; }
            // 65163 mov r8,QWORD PTR [rbp-0x48]
            r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 65167 mov r14,r12
            r14 = r12;
            // 6516a jne 0x6519f
            if fl.ne() { pc = 0x6519f; continue; }
            pc = 0x6516c;
        }
        0x6516c => {
            // block 6516c
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 6516c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6516c sub r14,rbx
            { let (r, f) = sub::<64>(r14, rbx);
              r14 = r;
              fl = f; }
            // 6516f test r14,r14
            fl = logic::<64>(r14 & r14);
            // 65172 jle 0x65194
            if fl.le() { pc = 0x65194; continue; }
            pc = 0x65174;
        }
        0x65174 => {
            // block 65174
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 65174 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65174 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 65178 mov rdi,r13
            rdi = r13;
            // 6517b mov rsi,rbx
            rsi = rbx;
            // 6517e mov rdx,r14
            rdx = r14;
            // 65181 mov rbx,r8
            rbx = r8;
            // 65184 call QWORD PTR [rax+0x60]
            { let r = { let t = m.r64(rax.wrapping_add(0x60)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65187 mov r8,rbx
            r8 = rbx;
            // 6518a mov rcx,rax
            rcx = rax;
            // 6518d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6518f cmp rcx,r14
            fl = sub::<64>(rcx, r14).1;
            // 65192 jne 0x6519f
            if fl.ne() { pc = 0x6519f; continue; }
            pc = 0x65194;
        }
        0x65194 => {
            // block 65194
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 65194 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65194 mov QWORD PTR [r8+0x18],0x0
            m.w64(r8.wrapping_add(0x18), (0x0u64) as u64);
            // 6519c mov rax,r13
            rax = r13;
            pc = 0x6519f;
        }
        0x6519f => {
            // block 6519f
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 6519f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6519f add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 651a3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 651a4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 651a6 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 651a8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 651aa pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 651ac pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 651ad ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x651ae => {
            // block 651ae
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 651ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 651ae mov rbx,rax
            rbx = rax;
            // 651b1 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 651b5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 651ba mov rdi,rbx
            rdi = rbx;
            // 651bd call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
