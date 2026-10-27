//! Mechanically lifted functions (scripts/lift_run.py), module q003. Each one is checked against the original
//! in the world harness (tests/world_q003.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xb2ada size=1228
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b2ada<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xb2ada;
    loop {
        match pc {
        0xb2ada => {
            // block b2ada
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2ada rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2ada push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b2adb mov rbp,rsp
            rbp = rsp;
            // b2ade push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b2ae0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b2ae2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b2ae4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b2ae6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b2ae7 sub rsp,0x168
            { let (r, f) = sub::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // b2aee mov DWORD PTR [rbp-0x170],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe90), ((rcx as u32 as u64)) as u32);
            // b2af4 mov QWORD PTR [rbp-0x178],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rdi) as u64);
            // b2afb lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b2b02 lea r15,[rbp-0x160]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b2b09 mov rax,QWORD PTR [rip+0x8d6c0] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // b2b10 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // b2b14 mov QWORD PTR [rbp-0x168],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rcx) as u64);
            // b2b1b add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // b2b1f mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // b2b26 mov rsi,r15
            rsi = r15;
            // b2b29 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2b2e mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // b2b36 mov DWORD PTR [rbp-0x68],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0xffffffffu64) as u32);
            // b2b3d mov rbx,QWORD PTR [rip+0x8d87c] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // b2b44 lea r14,[rbx+0x18]
            r14 = rbx.wrapping_add(0x18);
            // b2b48 mov QWORD PTR [rbp-0x168],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (r14) as u64);
            // b2b4f add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // b2b53 mov QWORD PTR [rbp-0xf8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rbx) as u64);
            // b2b5a mov rdi,r15
            rdi = r15;
            // b2b5d call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2b62 mov QWORD PTR [rbp-0x188],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (r14) as u64);
            // b2b69 mov QWORD PTR [rbp-0x180],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rbx) as u64);
            // b2b70 mov rax,QWORD PTR [rip+0x8d831] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // b2b77 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // b2b7b mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // b2b82 mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // b2b89 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // b2b94 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // b2b9f mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // b2baa mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // b2bb5 mov DWORD PTR [rbp-0x100],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x10u64) as u32);
            // b2bbf xorps xmm0,xmm0
            x0 = 0;
            // b2bc2 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // b2bc6 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // b2bce lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b2bd2 mov rdi,r15
            rdi = r15;
            // b2bd5 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2bda lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b2bde call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2be3 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // b2be5 lea r14,[rbp-0x40]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            pc = 0xb2be9;
        }
        0xb2be9 => {
            // block b2be9
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2be9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2be9 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // b2bee lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b2bf5 lea rsi,[rip+0x8249f] # 0x13509b
            rsi = m.env.data_va(301160);
            // b2bfc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2c01 lea r13d,[rbx+0x1]
            r13 = (rbx.wrapping_add(0x1)) as u32 as u64;
            // b2c05 mov rdi,rax
            rdi = rax;
            // b2c08 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // b2c0b call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2c10 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // b2c15 mov rdi,rax
            rdi = rax;
            // b2c18 lea rsi,[rip+0x824f9] # 0x135118
            rsi = m.env.data_va(301285);
            // b2c1f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2c24 mov ecx,DWORD PTR [rbp-0x170]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64) as u32 as u64;
            // b2c2a bt ecx,ebx
            fl.cf = bit::<32>((rcx as u32 as u64), (rbx as u32 as u64));
            // b2c2d lea rsi,[rip+0x810d1] # 0x133d05
            rsi = m.env.data_va(296146);
            // b2c34 lea rcx,[rip+0x77fe7] # 0x12ac22
            rcx = m.env.data_va(259055);
            // b2c3b cmovb rsi,rcx
            if fl.b() { rsi = rcx; }
            // b2c3f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b2c44 mov rdi,rax
            rdi = rax;
            // b2c47 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2c4c mov r12,rax
            r12 = rax;
            // b2c4f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b2c53 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b2c57 add rsi,r12
            { let (r, f) = add::<64>(rsi, r12);
              rsi = r;
              fl = f; }
            // b2c5a mov rdi,r14
            rdi = r14;
            // b2c5d call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2c62 mov rdi,r14
            rdi = r14;
            // b2c65 mov rsi,QWORD PTR [rip+0x8d524] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b2c6c call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2c71 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b2c74 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b2c78 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b2c7d mov rdi,rax
            rdi = rax;
            // b2c80 call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2c82 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b2c84 mov rdi,r14
            rdi = r14;
            // b2c87 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2c8c movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // b2c8f mov rdi,r12
            rdi = r12;
            // b2c92 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2c97 mov rdi,r12
            rdi = r12;
            // b2c9a call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2c9f cmp r13d,0x4
            fl = sub::<32>((r13 as u32 as u64), 0x4u64).1;
            // b2ca3 mov ebx,r13d
            rbx = ((r13 as u32 as u64)) as u32 as u64;
            // b2ca6 jb 0xb2be9
            if fl.b() { pc = 0xb2be9; continue; }
            pc = 0xb2cac;
        }
        0xb2cac => {
            // block b2cac
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2cac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2cac lea rsi,[rip+0x82471] # 0x135124
            rsi = m.env.data_va(301297);
            // b2cb3 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b2cba mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // b2cbf call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2cc4 mov ecx,DWORD PTR [rbp-0x170]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64) as u32 as u64;
            // b2cca test cl,0x10
            fl = logic::<8>((rcx as u8 as u64) & 0x10u64);
            // b2ccd lea rsi,[rip+0x81031] # 0x133d05
            rsi = m.env.data_va(296146);
            // b2cd4 lea rcx,[rip+0x77f47] # 0x12ac22
            rcx = m.env.data_va(259055);
            // b2cdb cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b2cdf mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b2ce4 mov rdi,rax
            rdi = rax;
            // b2ce7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2cec mov r14,rax
            r14 = rax;
            // b2cef mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b2cf2 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b2cf6 add rsi,r14
            { let (r, f) = add::<64>(rsi, r14);
              rsi = r;
              fl = f; }
            // b2cf9 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b2cfd call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2d02 mov rsi,QWORD PTR [rip+0x8d487] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b2d09 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b2d0d call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2d12 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b2d15 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b2d19 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b2d1e mov rdi,rax
            rdi = rax;
            // b2d21 call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2d23 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b2d26 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b2d2a call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2d2f movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // b2d33 mov rdi,r14
            rdi = r14;
            // b2d36 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2d3b mov rdi,r14
            rdi = r14;
            // b2d3e call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2d43 mov r15d,0x1b
            r15 = (0x1bu64) as u32 as u64;
            // b2d49 lea r14,[rbp-0x30]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0xb2d4d;
        }
        0xb2d4d => {
            // block b2d4d
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2d4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2d4d mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // b2d52 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b2d59 lea rsi,[rip+0x8233b] # 0x13509b
            rsi = m.env.data_va(301160);
            // b2d60 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2d65 lea r13d,[r15-0x1a]
            r13 = (r15.wrapping_add(0xffff_ffff_ffff_ffe6)) as u32 as u64;
            // b2d69 mov rdi,rax
            rdi = rax;
            // b2d6c mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // b2d6f call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2d74 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // b2d79 mov rdi,rax
            rdi = rax;
            // b2d7c lea rsi,[rip+0x823b6] # 0x135139
            rsi = m.env.data_va(301318);
            // b2d83 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2d88 mov ecx,DWORD PTR [rbp-0x170]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64) as u32 as u64;
            // b2d8e bt ecx,r15d
            fl.cf = bit::<32>((rcx as u32 as u64), (r15 as u32 as u64));
            // b2d92 lea rsi,[rip+0x80f6c] # 0x133d05
            rsi = m.env.data_va(296146);
            // b2d99 lea rcx,[rip+0x77e82] # 0x12ac22
            rcx = m.env.data_va(259055);
            // b2da0 cmovb rsi,rcx
            if fl.b() { rsi = rcx; }
            // b2da4 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b2da9 mov rdi,rax
            rdi = rax;
            // b2dac call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2db1 mov r12,rax
            r12 = rax;
            // b2db4 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b2db8 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b2dbc add rsi,r12
            { let (r, f) = add::<64>(rsi, r12);
              rsi = r;
              fl = f; }
            // b2dbf mov rdi,r14
            rdi = r14;
            // b2dc2 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2dc7 mov rdi,r14
            rdi = r14;
            // b2dca mov rsi,QWORD PTR [rip+0x8d3bf] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b2dd1 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2dd6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b2dd9 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b2ddd mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b2de2 mov rdi,rax
            rdi = rax;
            // b2de5 call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2de7 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b2de9 mov rdi,r14
            rdi = r14;
            // b2dec call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2df1 movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // b2df4 mov rdi,r12
            rdi = r12;
            // b2df7 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2dfc mov rdi,r12
            rdi = r12;
            // b2dff call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2e04 inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // b2e07 cmp r13d,0x4
            fl = sub::<32>((r13 as u32 as u64), 0x4u64).1;
            // b2e0b jb 0xb2d4d
            if fl.b() { pc = 0xb2d4d; continue; }
            pc = 0xb2e11;
        }
        0xb2e11 => {
            // block b2e11
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2e11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2e11 lea rsi,[rip+0x8232c] # 0x135144
            rsi = m.env.data_va(301329);
            // b2e18 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b2e1f mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // b2e24 lea r15,[rbp-0x160]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b2e2b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2e30 cmp DWORD PTR [rbp-0x170],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64, 0x0u64).1;
            // b2e37 lea rsi,[rip+0x80ec7] # 0x133d05
            rsi = m.env.data_va(296146);
            // b2e3e lea rcx,[rip+0x77ddd] # 0x12ac22
            rcx = m.env.data_va(259055);
            // b2e45 cmovs rsi,rcx
            if fl.s() { rsi = rcx; }
            // b2e49 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b2e4e mov rdi,rax
            rdi = rax;
            // b2e51 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2e56 mov rdi,QWORD PTR [rbp-0x178]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // b2e5d mov rsi,r15
            rsi = r15;
            // b2e60 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2e65 mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // b2e6c mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b2e73 mov rax,QWORD PTR [rbp-0x180]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // b2e7a mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // b2e81 mov rax,QWORD PTR [rbp-0x190]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // b2e88 mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // b2e8f lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // b2e96 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2e9b mov rdi,r15
            rdi = r15;
            // b2e9e call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2ea3 mov rsi,QWORD PTR [rip+0x8d35e] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b2eaa add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b2eae lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b2eb5 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2eba lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b2ec1 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2ec6 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // b2ecd add rsp,0x168
            { let (r, f) = add::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // b2ed4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b2ed5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b2ed7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b2ed9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b2edb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b2edd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b2ede ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb2edf => {
            // block b2edf
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2edf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2edf mov rbx,rax
            rbx = rax;
            // b2ee2 jmp 0xb2f11
            pc = 0xb2f11; continue;
        }
        0xb2ee4 => {
            // block b2ee4
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2ee4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2ee4 mov rbx,rax
            rbx = rax;
            // b2ee7 lea r15,[rbp-0x160]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b2eee jmp 0xb2f11
            pc = 0xb2f11; continue;
        }
        0xb2ef0 => {
            // block b2ef0
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2ef0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2ef0 mov rbx,rax
            rbx = rax;
            // b2ef3 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b2ef7 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2efc jmp 0xb2f11
            pc = 0xb2f11; continue;
        }
        0xb2efe => {
            // block b2efe
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2efe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2efe mov rbx,rax
            rbx = rax;
            // b2f01 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b2f05 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2f0a lea r15,[rbp-0x160]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            pc = 0xb2f11;
        }
        0xb2f11 => {
            // block b2f11
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2f11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2f11 mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // b2f18 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b2f1f mov rax,QWORD PTR [rbp-0x180]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // b2f26 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // b2f2d mov rax,QWORD PTR [rbp-0x190]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // b2f34 mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            pc = 0xb2f3b;
        }
        0xb2f3b => {
            // block b2f3b
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2f3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2f3b lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // b2f42 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2f47 mov rdi,r15
            rdi = r15;
            // b2f4a call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb2f4f;
        }
        0xb2f4f => {
            // block b2f4f
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2f4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2f4f mov rsi,QWORD PTR [rip+0x8d2b2] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b2f56 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b2f5a lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b2f61 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb2f66;
        }
        0xb2f66 => {
            // block b2f66
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2f66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2f66 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b2f6d call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2f72 mov rdi,rbx
            rdi = rbx;
            // b2f75 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2f7a jmp 0xb2edf
            pc = 0xb2edf; continue;
        }
        0xb2f7f => {
            // block b2f7f
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2f7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2f7f mov rbx,rax
            rbx = rax;
            // b2f82 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b2f86 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2f8b jmp 0xb2f11
            pc = 0xb2f11; continue;
        }
        0xb2f8d => {
            // block b2f8d
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2f8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2f8d mov rbx,rax
            rbx = rax;
            // b2f90 jmp 0xb2f66
            pc = 0xb2f66; continue;
        }
        0xb2f92 => {
            // block b2f92
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2f92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2f92 mov rbx,rax
            rbx = rax;
            // b2f95 jmp 0xb2f4f
            pc = 0xb2f4f; continue;
        }
        0xb2f97 => {
            // block b2f97
            #[cfg(feature = "trace")] eprintln!("f_000b2ada pc b2f97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2f97 mov rbx,rax
            rbx = rax;
            // b2f9a lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b2f9e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2fa3 jmp 0xb2f3b
            pc = 0xb2f3b; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb43d4 size=1208
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b43d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xb43d4;
    loop {
        match pc {
        0xb43d4 => {
            // block b43d4
            #[cfg(feature = "trace")] eprintln!("f_000b43d4 pc b43d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b43d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b43d5 mov rbp,rsp
            rbp = rsp;
            // b43d8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b43da push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b43dc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b43de push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b43e0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b43e1 sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // b43e8 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // b43eb mov QWORD PTR [rbp-0x160],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rdi) as u64);
            // b43f2 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // b43f9 lea r12,[rbp-0x150]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b4400 mov rax,QWORD PTR [rip+0x8bdc9] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // b4407 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // b440b mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // b4412 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // b4416 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // b441d mov rsi,r12
            rsi = r12;
            // b4420 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4425 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // b442d mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // b4434 mov r15,QWORD PTR [rip+0x8bf85] # 0x1403c0
            r15 = m.r64(m.env.data_va(347021)) as u64;
            // b443b lea rbx,[r15+0x18]
            rbx = r15.wrapping_add(0x18);
            // b443f mov QWORD PTR [rbp-0x158],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rbx) as u64);
            // b4446 add r15,0x40
            { let (r, f) = add::<64>(r15, 0x40u64);
              r15 = r;
              fl = f; }
            // b444a mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // b4451 mov rdi,r12
            rdi = r12;
            // b4454 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4459 mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // b4460 mov r13,QWORD PTR [rip+0x8bf41] # 0x1403a8
            r13 = m.r64(m.env.data_va(346997)) as u64;
            // b4467 add r13,0x10
            { let (r, f) = add::<64>(r13, 0x10u64);
              r13 = r;
              fl = f; }
            // b446b mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            // b4472 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // b447d mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // b4488 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // b4493 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // b449e mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // b44a8 xorps xmm0,xmm0
            x0 = 0;
            // b44ab movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // b44af mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // b44b7 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b44bb mov rdi,r12
            rdi = r12;
            // b44be call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b44c3 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b44c7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b44cc lea rsi,[rip+0x80dfc] # 0x1352cf
            rsi = m.env.data_va(301724);
            // b44d3 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // b44da mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // b44df call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b44e4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b44e7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b44eb mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b44ef and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b44f2 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b44f5 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b44f9 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b44fc mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4500 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b4508 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b450b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b450f mov QWORD PTR [rcx+rax*1+0x18],0x3
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x3u64) as u64);
            // b4518 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b451b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b451f mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b452a mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // b452d and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b4533 mov rdi,rax
            rdi = rax;
            // b4536 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b453b mov r12,rax
            r12 = rax;
            // b453e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b4542 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4546 mov ecx,DWORD PTR [rax+r12*1+0x8]
            rcx = (m.r32(rax.wrapping_add(r12).wrapping_add(0x8)) as u64) as u32 as u64;
            // b454b and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b454e or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b4551 mov DWORD PTR [rax+r12*1+0x8],ecx
            m.w32(rax.wrapping_add(r12).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // b4556 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b455a mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b455e mov DWORD PTR [rax+r12*1+0x90],0x20
            m.w32(rax.wrapping_add(r12).wrapping_add(0x90), (0x20u64) as u32);
            // b456a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b456e mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4572 and DWORD PTR [rax+r12*1+0x8],0xffffbfff
            { let r = m.r32(rax.wrapping_add(r12).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rax.wrapping_add(r12).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b457b mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b457f mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4583 add rsi,r12
            { let (r, f) = add::<64>(rsi, r12);
              rsi = r;
              fl = f; }
            // b4586 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b458a call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b458f mov rsi,QWORD PTR [rip+0x8bbfa] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b4596 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b459a call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b459f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b45a2 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b45a6 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b45ab mov rdi,rax
            rdi = rax;
            // b45ae call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b45b0 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b45b2 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b45b6 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b45bb movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // b45be mov rdi,r12
            rdi = r12;
            // b45c1 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b45c6 mov rdi,r12
            rdi = r12;
            // b45c9 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b45ce lea rsi,[rip+0x80d0a] # 0x1352df
            rsi = m.env.data_va(301740);
            // b45d5 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // b45da mov rdi,r12
            rdi = r12;
            // b45dd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b45e2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b45e5 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b45e9 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b45ed and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b45f0 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b45f3 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b45f7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b45fa mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b45fe or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b4606 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4609 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b460d mov QWORD PTR [rcx+rax*1+0x18],0x3
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x3u64) as u64);
            // b4616 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4619 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b461d mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b4628 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // b462b shr esi,0xa
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xau64, fl); rsi = (r) as u32 as u64; fl = f; }
            // b462e and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b4634 add esi,0xffffffc0
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffffc0u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b4637 mov rdi,rax
            rdi = rax;
            // b463a call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b463f mov rbx,rax
            rbx = rax;
            // b4642 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b4645 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4649 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // b464d and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b4650 or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b4653 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // b4657 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b465a mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b465e mov DWORD PTR [rax+rbx*1+0x90],0x20
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x20u64) as u32);
            // b4669 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b466c mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4670 and DWORD PTR [rax+rbx*1+0x8],0xffffbfff
            { let r = m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b4678 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b467b mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b467f add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b4682 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b4686 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b468b mov rsi,QWORD PTR [rip+0x8bafe] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b4692 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b4696 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b469b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b469e mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b46a2 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b46a7 mov rdi,rax
            rdi = rax;
            // b46aa call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b46ac mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b46af lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b46b3 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b46b8 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // b46bc mov rdi,rbx
            rdi = rbx;
            // b46bf call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b46c4 mov rdi,rbx
            rdi = rbx;
            // b46c7 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b46cc lea rsi,[rip+0x80c1b] # 0x1352ee
            rsi = m.env.data_va(301755);
            // b46d3 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // b46d8 mov rdi,rbx
            rdi = rbx;
            // b46db call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b46e0 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b46e3 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b46e7 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b46eb and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b46ee or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b46f1 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b46f5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b46f8 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b46fc or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b4704 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4707 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b470b mov QWORD PTR [rcx+rax*1+0x18],0x3
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x3u64) as u64);
            // b4714 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4717 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b471b mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b4726 shr r14d,0x14
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x14u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // b472a and r14d,0x3ff
            { let r = (r14 as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b4731 mov rdi,rax
            rdi = rax;
            // b4734 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // b4737 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b473c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b473f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4743 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b4747 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b474a or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b474d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b4751 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4754 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4758 mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // b4763 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4766 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b476a and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b4772 mov rdi,QWORD PTR [rbp-0x160]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // b4779 lea rsi,[rbp-0x150]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b4780 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4785 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // b478c mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // b4793 mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // b479a mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            // b47a1 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // b47a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b47ad lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b47b4 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b47b9 mov rsi,QWORD PTR [rip+0x8ba48] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b47c0 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b47c4 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // b47cb call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b47d0 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // b47d7 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b47dc mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // b47e3 add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // b47ea pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b47eb pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b47ed pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b47ef pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b47f1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b47f3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b47f4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb47f5 => {
            // block b47f5
            #[cfg(feature = "trace")] eprintln!("f_000b43d4 pc b47f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b47f5 mov rbx,rax
            rbx = rax;
            pc = 0xb47f8;
        }
        0xb47f8 => {
            // block b47f8
            #[cfg(feature = "trace")] eprintln!("f_000b43d4 pc b47f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b47f8 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // b47ff mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // b4806 mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // b480d mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            pc = 0xb4814;
        }
        0xb4814 => {
            // block b4814
            #[cfg(feature = "trace")] eprintln!("f_000b43d4 pc b4814 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4814 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // b481b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4820 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b4827 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb482c;
        }
        0xb482c => {
            // block b482c
            #[cfg(feature = "trace")] eprintln!("f_000b43d4 pc b482c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b482c mov rsi,QWORD PTR [rip+0x8b9d5] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b4833 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b4837 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // b483e call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb4843;
        }
        0xb4843 => {
            // block b4843
            #[cfg(feature = "trace")] eprintln!("f_000b43d4 pc b4843 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4843 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // b484a call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b484f mov rdi,rbx
            rdi = rbx;
            // b4852 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4857 mov rbx,rax
            rbx = rax;
            // b485a lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b485e call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4863 jmp 0xb47f8
            pc = 0xb47f8; continue;
        }
        0xb4865 => {
            // block b4865
            #[cfg(feature = "trace")] eprintln!("f_000b43d4 pc b4865 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4865 mov rbx,rax
            rbx = rax;
            // b4868 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b486c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4871 jmp 0xb47f8
            pc = 0xb47f8; continue;
        }
        0xb4873 => {
            // block b4873
            #[cfg(feature = "trace")] eprintln!("f_000b43d4 pc b4873 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4873 mov rbx,rax
            rbx = rax;
            // b4876 jmp 0xb4843
            pc = 0xb4843; continue;
        }
        0xb4878 => {
            // block b4878
            #[cfg(feature = "trace")] eprintln!("f_000b43d4 pc b4878 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4878 mov rbx,rax
            rbx = rax;
            // b487b jmp 0xb482c
            pc = 0xb482c; continue;
        }
        0xb487d => {
            // block b487d
            #[cfg(feature = "trace")] eprintln!("f_000b43d4 pc b487d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b487d mov rbx,rax
            rbx = rax;
            // b4880 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b4884 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4889 jmp 0xb4814
            pc = 0xb4814; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdd560 size=1168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000dd560<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xdd560;
    loop {
        match pc {
        0xdd560 => {
            // block dd560
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd560 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dd561 mov rbp,rsp
            rbp = rsp;
            // dd564 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dd566 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dd568 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dd56a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dd56c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dd56d sub rsp,0xc088
            { let (r, f) = sub::<64>(rsp, 0xc088u64);
              rsp = r;
              fl = f; }
            // dd574 mov r13,rdi
            r13 = rdi;
            // dd577 mov rax,QWORD PTR [rip+0x62e5a] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // dd57e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // dd581 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // dd585 mov QWORD PTR [rbp-0xc038],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_3fc8), (0x0u64) as u64);
            // dd590 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // dd594 lea rsi,[rbp-0xc03c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_3fc4);
            // dd59b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dd59d call QWORD PTR [rax+0x3f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd5a3 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // dd5a7 lea rsi,[rbp-0xc040]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_3fc0);
            // dd5ae xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dd5b0 mov rdi,r13
            rdi = r13;
            // dd5b3 call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd5b9 mov r15d,DWORD PTR [rbp-0xc040]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fc0)) as u64) as u32 as u64;
            // dd5c0 cmp r15d,0xa
            fl = sub::<32>((r15 as u32 as u64), 0xau64).1;
            // dd5c4 sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // dd5c7 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // dd5cb lea rsi,[rbp-0xc049]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_3fb7);
            // dd5d2 lea rdx,[rbp-0xc04a]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_3fb6);
            // dd5d9 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dd5db mov rdi,r13
            rdi = r13;
            // dd5de call QWORD PTR [rax+0x5e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x5e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd5e4 mov esi,DWORD PTR [rbp-0xc03c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fc4)) as u64) as u32 as u64;
            // dd5ea mov edx,DWORD PTR [r13+0x274]
            rdx = (m.r32(r13.wrapping_add(0x274)) as u64) as u32 as u64;
            // dd5f1 movzx r9d,BYTE PTR [rbp-0xc04a]
            r9 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_3fb6)) as u64) as u32 as u64;
            // dd5f9 movzx ecx,BYTE PTR [rbp-0xc049]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_3fb7)) as u64) as u32 as u64;
            // dd600 movzx r8d,bl
            r8 = ((rbx as u8 as u64)) as u32 as u64;
            // dd604 lea rdi,[rbp-0xc088]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_3f78);
            // dd60b call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd610 mov eax,DWORD PTR [rbp-0xc084]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f7c)) as u64) as u32 as u64;
            // dd616 mov DWORD PTR [rbp-0xc048],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3fb8), ((rax as u32 as u64)) as u32);
            // dd61c mov eax,DWORD PTR [rbp-0xc080]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f80)) as u64) as u32 as u64;
            // dd622 mov DWORD PTR [rbp-0xc044],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc), ((rax as u32 as u64)) as u32);
            // dd628 movzx eax,WORD PTR [r13+0x270]
            rax = (m.r16(r13.wrapping_add(0x270)) as u64) as u32 as u64;
            // dd630 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // dd632 mov cx,0x40
            rcx = (rcx & !0xffff) | ((0x40u64) as u16 as u64);
            // dd636 mov r12w,0x3ac
            r12 = (r12 & !0xffff) | ((0x3acu64) as u16 as u64);
            // dd63b cmove r12w,cx
            if fl.e() { r12 = (r12 & !0xffff) | (((rcx as u16 as u64)) as u16 as u64); }
            // dd640 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // dd643 shl r14d,0x5
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x5u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // dd647 not r14d
            r14 = (!(r14 as u32 as u64)) as u32 as u64;
            // dd64a and r14d,0x40
            { let r = (r14 as u32 as u64) & 0x40u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // dd64e test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // dd650 mov ax,0x200
            rax = (rax & !0xffff) | ((0x200u64) as u16 as u64);
            // dd654 mov cx,0x3c0
            rcx = (rcx & !0xffff) | ((0x3c0u64) as u16 as u64);
            // dd658 cmove cx,ax
            if fl.e() { rcx = (rcx & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            // dd65c mov WORD PTR [rbp-0xc098],cx
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_3f68), ((rcx as u16 as u64)) as u16);
            // dd663 cmp r15d,0xa
            fl = sub::<32>((r15 as u32 as u64), 0xau64).1;
            // dd667 mov r15,r13
            r15 = r13;
            // dd66a mov r13d,DWORD PTR [rbp-0xc088]
            r13 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f78)) as u64) as u32 as u64;
            // dd671 jne 0xdd68d
            if fl.ne() { pc = 0xdd68d; continue; }
            pc = 0xdd673;
        }
        0xdd673 => {
            // block dd673
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd673 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd673 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dd676 lea rsi,[rbp-0xc048]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_3fb8);
            // dd67d lea rdx,[rbp-0xc044]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_3fbc);
            // dd684 mov rdi,r15
            rdi = r15;
            // dd687 call QWORD PTR [rax+0x1a00]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a00)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdd68d;
        }
        0xdd68d => {
            // block dd68d
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd68d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd68d add r14d,0x1c0
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x1c0u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // dd694 mov DWORD PTR [rbp-0xc08c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3f74), (0x0u64) as u32);
            // dd69e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dd6a1 lea rdx,[rbp-0xc08c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_3f74);
            // dd6a8 mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // dd6ad mov ecx,0x800
            rcx = (0x800u64) as u32 as u64;
            // dd6b2 mov r8d,0xb
            r8 = (0xbu64) as u32 as u64;
            // dd6b8 mov rdi,r15
            rdi = r15;
            // dd6bb call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd6be cmp DWORD PTR [r15+0x274],0x0
            fl = sub::<32>(m.r32(r15.wrapping_add(0x274)) as u64, 0x0u64).1;
            // dd6c6 jne 0xdd6fe
            if fl.ne() { pc = 0xdd6fe; continue; }
            pc = 0xdd6c8;
        }
        0xdd6c8 => {
            // block dd6c8
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd6c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd6c8 mov eax,DWORD PTR [rbp-0xc08c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f74)) as u64) as u32 as u64;
            // dd6ce test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // dd6d0 je 0xdd6fe
            if fl.e() { pc = 0xdd6fe; continue; }
            pc = 0xdd6d2;
        }
        0xdd6d2 => {
            // block dd6d2
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd6d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd6d2 and r13d,0xfffe
            { let r = (r13 as u32 as u64) & 0xfffeu64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // dd6d9 shr DWORD PTR [rbp-0xc048],1
            { let (r, f) = shr::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fb8)) as u64, 0x1u64, fl); m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3fb8), (r) as u32); fl = f; }
            // dd6df shr r13d,1
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x1u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // dd6e2 mov eax,DWORD PTR [rbp-0xc03c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fc4)) as u64) as u32 as u64;
            // dd6e8 lea ecx,[rax-0x2]
            rcx = (rax.wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // dd6eb cmp ecx,0x4
            fl = sub::<32>((rcx as u32 as u64), 0x4u64).1;
            // dd6ee jb 0xdd6f8
            if fl.b() { pc = 0xdd6f8; continue; }
            pc = 0xdd6f0;
        }
        0xdd6f0 => {
            // block dd6f0
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd6f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd6f0 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // dd6f2 jne 0xdd9bf
            if fl.ne() { pc = 0xdd9bf; continue; }
            pc = 0xdd6f8;
        }
        0xdd6f8 => {
            // block dd6f8
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd6f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd6f8 shr DWORD PTR [rbp-0xc044],1
            { let (r, f) = shr::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc)) as u64, 0x1u64, fl); m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc), (r) as u32); fl = f; }
            pc = 0xdd6fe;
        }
        0xdd6fe => {
            // block dd6fe
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd6fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd6fe mov rbx,QWORD PTR [r15+0x280]
            rbx = m.r64(r15.wrapping_add(0x280)) as u64;
            // dd705 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // dd707 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // dd70a jne 0xdd752
            if fl.ne() { pc = 0xdd752; continue; }
            pc = 0xdd70c;
        }
        0xdd70c => {
            // block dd70c
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd70c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd70c cmp BYTE PTR [r15+0x279],0x0
            fl = sub::<8>(m.r8(r15.wrapping_add(0x279)) as u64, 0x0u64).1;
            // dd714 je 0xdd72f
            if fl.e() { pc = 0xdd72f; continue; }
            pc = 0xdd716;
        }
        0xdd716 => {
            // block dd716
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd716 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd716 movzx edi,r13w
            rdi = ((r13 as u16 as u64)) as u32 as u64;
            // dd71a imul edi,DWORD PTR [rbp-0xc044]
            { let (r, f) = imul::<32>((rdi as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc)) as u64); rdi = (r) as u32 as u64; fl = f; }
            // dd721 shl rdi,0x2
            { let (r, f) = shl::<64>(rdi, 0x2u64, fl); rdi = r; fl = f; }
            // dd725 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd72a mov rbx,rax
            rbx = rax;
            // dd72d jmp 0xdd752
            pc = 0xdd752; continue;
        }
        0xdd72f => {
            // block dd72f
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd72f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd72f mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dd732 mov esi,DWORD PTR [r15+0x250]
            rsi = (m.r32(r15.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd739 lea rdx,[rbp-0xc038]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_3fc8);
            // dd740 mov rdi,r15
            rdi = r15;
            // dd743 call QWORD PTR [rax+0xa10]
            { let r = { let t = m.r64(rax.wrapping_add(0xa10)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd749 mov rbx,QWORD PTR [rbp-0xc038]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_3fc8)) as u64;
            // dd750 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xdd752;
        }
        0xdd752 => {
            // block dd752
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd752 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd752 mov QWORD PTR [rbp-0xc0a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_3f58), (rax) as u64);
            // dd759 lea rdi,[rbp-0xa030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // dd760 mov esi,0x800
            rsi = (0x800u64) as u32 as u64;
            // dd765 call 0x5932c
            { let r = { let t = m.env.code_va(0x5932c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd76a mov WORD PTR [rbp-0xa030],r14w
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_5fd0), ((r14 as u16 as u64)) as u16);
            // dd772 mov WORD PTR [rbp-0xa02e],r12w
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_5fd2), ((r12 as u16 as u64)) as u16);
            // dd77a mov di,WORD PTR [rbp-0xc098]
            rdi = (rdi & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_3f68)) as u64) as u16 as u64);
            // dd781 mov WORD PTR [rbp-0xa02c],di
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_5fd4), ((rdi as u16 as u64)) as u16);
            // dd788 mov eax,DWORD PTR [rbp-0xc048]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fb8)) as u64) as u32 as u64;
            // dd78e lea ecx,[rax+rax*1-0x1]
            rcx = (rax.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // dd792 mov WORD PTR [rbp+rcx*2-0xa030],r12w
            m.w16(rbp.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((r12 as u16 as u64)) as u16);
            // dd79b lea ecx,[rax+rax*1-0x2]
            rcx = (rax.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // dd79f mov WORD PTR [rbp+rcx*2-0xa030],di
            m.w16(rbp.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((rdi as u16 as u64)) as u16);
            // dd7a7 movzx ecx,WORD PTR [r15+0x270]
            rcx = (m.r16(r15.wrapping_add(0x270)) as u64) as u32 as u64;
            // dd7af mov si,r12w
            rsi = (rsi & !0xffff) | (((r12 as u16 as u64)) as u16 as u64);
            // dd7b3 mov r12,r15
            r12 = r15;
            // dd7b6 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // dd7b9 lea ecx,[rax+rax*1-0x3]
            rcx = (rax.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_fffd)) as u32 as u64;
            // dd7bd mov dx,0x40
            rdx = (rdx & !0xffff) | ((0x40u64) as u16 as u64);
            // dd7c1 cmovne dx,si
            if fl.ne() { rdx = (rdx & !0xffff) | (((rsi as u16 as u64)) as u16 as u64); }
            // dd7c5 mov WORD PTR [rbp+rcx*2-0xa030],dx
            m.w16(rbp.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((rdx as u16 as u64)) as u16);
            // dd7cd lea eax,[rax+rax*1-0x4]
            rax = (rax.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_fffc)) as u32 as u64;
            // dd7d1 mov WORD PTR [rbp+rax*2-0xa030],r14w
            m.w16(rbp.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((r14 as u16 as u64)) as u16);
            // dd7da movzx esi,si
            rsi = ((rsi as u16 as u64)) as u32 as u64;
            // dd7dd movzx edx,r14w
            rdx = ((r14 as u16 as u64)) as u32 as u64;
            // dd7e1 movzx ecx,di
            rcx = ((rdi as u16 as u64)) as u32 as u64;
            // dd7e4 lea r14,[rbp-0xc030]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_3fd0);
            // dd7eb mov r8d,0x800
            r8 = (0x800u64) as u32 as u64;
            // dd7f1 mov rdi,r14
            rdi = r14;
            // dd7f4 call 0x59372
            { let r = { let t = m.env.code_va(0x59372); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd7f9 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // dd7fd mov QWORD PTR [rbp-0xc098],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_3f68), (rax) as u64);
            // dd804 test r13w,r13w
            fl = logic::<16>((r13 as u16 as u64) & (r13 as u16 as u64));
            // dd808 je 0xdd8af
            if fl.e() { pc = 0xdd8af; continue; }
            pc = 0xdd80e;
        }
        0xdd80e => {
            // block dd80e
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd80e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd80e mov rax,QWORD PTR [rbp-0xc098]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_3f68)) as u64;
            // dd815 lea eax,[rax-0x1]
            rax = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // dd818 mov DWORD PTR [rbp-0xc09c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3f64), ((rax as u32 as u64)) as u32);
            // dd81e xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // dd821 lea r13,[rbp-0x8030]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            pc = 0xdd828;
        }
        0xdd828 => {
            // block dd828
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd828 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd828 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // dd82b mov rsi,r14
            rsi = r14;
            // dd82e je 0xdd865
            if fl.e() { pc = 0xdd865; continue; }
            pc = 0xdd830;
        }
        0xdd830 => {
            // block dd830
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd830 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd830 cmp DWORD PTR [rbp-0xc09c],r15d
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f64)) as u64, (r15 as u32 as u64)).1;
            // dd837 mov rsi,r14
            rsi = r14;
            // dd83a je 0xdd865
            if fl.e() { pc = 0xdd865; continue; }
            pc = 0xdd83c;
        }
        0xdd83c => {
            // block dd83c
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd83c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd83c cmp r15d,DWORD PTR [rbp-0xc07c]
            fl = sub::<32>((r15 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f84)) as u64).1;
            // dd843 mov rsi,r14
            rsi = r14;
            // dd846 je 0xdd865
            if fl.e() { pc = 0xdd865; continue; }
            pc = 0xdd848;
        }
        0xdd848 => {
            // block dd848
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd848 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd848 cmp DWORD PTR [rbp-0xc03c],0x5
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fc4)) as u64, 0x5u64).1;
            // dd84f lea rax,[rbp-0xa030]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // dd856 mov rsi,rax
            rsi = rax;
            // dd859 cmove rsi,r14
            if fl.e() { rsi = r14; }
            // dd85d cmp r15d,0x20
            fl = sub::<32>((r15 as u32 as u64), 0x20u64).1;
            // dd861 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            pc = 0xdd865;
        }
        0xdd865 => {
            // block dd865
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd865 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd865 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd869 mov ecx,DWORD PTR [rbp-0xc048]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fb8)) as u64) as u32 as u64;
            // dd86f mov rdi,r12
            rdi = r12;
            // dd872 mov rdx,r13
            rdx = r13;
            // dd875 call QWORD PTR [rax+0x19e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x19e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd87b mov edx,DWORD PTR [rbp-0xc044]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc)) as u64) as u32 as u64;
            // dd881 shl edx,0x2
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // dd884 mov rdi,rbx
            rdi = rbx;
            // dd887 mov rsi,r13
            rsi = r13;
            // dd88a call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd88f mov eax,DWORD PTR [rbp-0xc044]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc)) as u64) as u32 as u64;
            // dd895 lea rbx,[rbx+rax*4]
            rbx = rbx.wrapping_add(rax.wrapping_mul(4));
            // dd899 inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // dd89c movzx eax,r15w
            rax = ((r15 as u16 as u64)) as u32 as u64;
            // dd8a0 mov rcx,QWORD PTR [rbp-0xc098]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_3f68)) as u64;
            // dd8a7 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // dd8a9 jne 0xdd828
            if fl.ne() { pc = 0xdd828; continue; }
            pc = 0xdd8af;
        }
        0xdd8af => {
            // block dd8af
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd8af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd8af mov r15,QWORD PTR [rbp-0xc0a8]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_3f58)) as u64;
            // dd8b6 test r15,r15
            fl = logic::<64>(r15 & r15);
            // dd8b9 je 0xdd939
            if fl.e() { pc = 0xdd939; continue; }
            pc = 0xdd8bb;
        }
        0xdd8bb => {
            // block dd8bb
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd8bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd8bb mov al,BYTE PTR [r12+0x279]
            rax = (rax & !0xff) | ((m.r8(r12.wrapping_add(0x279)) as u64) as u8 as u64);
            // dd8c3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // dd8c5 je 0xdd939
            if fl.e() { pc = 0xdd939; continue; }
            pc = 0xdd8c7;
        }
        0xdd8c7 => {
            // block dd8c7
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd8c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd8c7 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd8cb mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd8d3 lea rdx,[rbp-0xc090]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_3f70);
            // dd8da mov rdi,r12
            rdi = r12;
            // dd8dd call QWORD PTR [rax+0x538]
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd8e3 mov r14d,DWORD PTR [rbp-0xc090]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f70)) as u64) as u32 as u64;
            // dd8ea mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd8ee mov rdi,r12
            rdi = r12;
            // dd8f1 call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd8f7 cmp r14d,eax
            fl = sub::<32>((r14 as u32 as u64), (rax as u32 as u64)).1;
            // dd8fa jbe 0xdd90a
            if fl.be() { pc = 0xdd90a; continue; }
            pc = 0xdd8fc;
        }
        0xdd8fc => {
            // block dd8fc
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd8fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd8fc mov DWORD PTR [rbp-0xc090],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3f70), (0x0u64) as u32);
            // dd906 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dd908 jmp 0xdd910
            pc = 0xdd910; continue;
        }
        0xdd90a => {
            // block dd90a
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd90a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd90a mov esi,DWORD PTR [rbp-0xc090]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f70)) as u64) as u32 as u64;
            pc = 0xdd910;
        }
        0xdd910 => {
            // block dd910
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd910 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd910 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd914 mov rcx,QWORD PTR [rbp-0xc098]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_3f68)) as u64;
            // dd91b imul ecx,DWORD PTR [rbp-0xc044]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // dd922 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // dd925 mov rdi,r12
            rdi = r12;
            // dd928 mov rdx,r15
            rdx = r15;
            // dd92b call QWORD PTR [rax+0x2c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x2c0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd931 mov rdi,r15
            rdi = r15;
            // dd934 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdd939;
        }
        0xdd939 => {
            // block dd939
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd939 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd939 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd93d movzx esi,BYTE PTR [r12+0x278]
            rsi = (m.r8(r12.wrapping_add(0x278)) as u64) as u32 as u64;
            // dd946 mov rdi,r12
            rdi = r12;
            // dd949 call QWORD PTR [rax+0x570]
            { let r = { let t = m.r64(rax.wrapping_add(0x570)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd94f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd953 mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd95b mov edx,DWORD PTR [r12+0x274]
            rdx = (m.r32(r12.wrapping_add(0x274)) as u64) as u32 as u64;
            // dd963 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // dd968 mov rdi,r12
            rdi = r12;
            // dd96b call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd971 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd975 mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd97d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dd97f mov rdi,r12
            rdi = r12;
            // dd982 call QWORD PTR [rax+0x4f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4f8)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd988 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd98c mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd994 mov rdi,r12
            rdi = r12;
            // dd997 call QWORD PTR [rax+0x618]
            { let r = { let t = m.r64(rax.wrapping_add(0x618)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd99d mov rax,QWORD PTR [rip+0x62a34] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // dd9a4 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // dd9a7 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // dd9ab jne 0xdd9eb
            if fl.ne() { pc = 0xdd9eb; continue; }
            pc = 0xdd9ad;
        }
        0xdd9ad => {
            // block dd9ad
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd9ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd9ad add rsp,0xc088
            { let (r, f) = add::<64>(rsp, 0xc088u64);
              rsp = r;
              fl = f; }
            // dd9b4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dd9b5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dd9b7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dd9b9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dd9bb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dd9bd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dd9be ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xdd9bf => {
            // block dd9bf
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd9bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd9bf cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // dd9c2 jne 0xdd9d3
            if fl.ne() { pc = 0xdd9d3; continue; }
            pc = 0xdd9c4;
        }
        0xdd9c4 => {
            // block dd9c4
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd9c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd9c4 mov DWORD PTR [rbp-0xc044],0x1b0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc), (0x1b0u64) as u32);
            // dd9ce jmp 0xdd6fe
            pc = 0xdd6fe; continue;
        }
        0xdd9d3 => {
            // block dd9d3
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd9d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd9d3 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dd9d6 lea rsi,[rip+0x5b9bf] # 0x13939c
            rsi = m.env.data_va(318313);
            // dd9dd mov rdi,r15
            rdi = r15;
            // dd9e0 call QWORD PTR [rax+0x170]
            { let r = { let t = m.r64(rax.wrapping_add(0x170)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd9e6 jmp 0xdd6fe
            pc = 0xdd6fe; continue;
        }
        0xdd9eb => {
            // block dd9eb
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd9eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd9eb call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x273d2 size=1125
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000273d2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x273d2;
    loop {
        match pc {
        0x273d2 => {
            // block 273d2
            #[cfg(feature = "trace")] eprintln!("f_000273d2 pc 273d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 273d2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 273d3 mov rbp,rsp
            rbp = rsp;
            // 273d6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 273d7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 273d8 mov rbx,rsi
            rbx = rsi;
            // 273db mov rax,rdi
            rax = rdi;
            // 273de mov rdi,rbx
            rdi = rbx;
            // 273e1 mov rsi,rax
            rsi = rax;
            // 273e4 call 0x26172
            { let r = { let t = m.env.code_va(0x26172); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 273e9 lea rsi,[rip+0x1030b9] # 0x12a4a9
            rsi = m.env.data_va(257142);
            // 273f0 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 273f5 mov rdi,rax
            rdi = rax;
            // 273f8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 273fd mov rsi,QWORD PTR [rbx+0x20]
            rsi = m.r64(rbx.wrapping_add(0x20)) as u64;
            // 27401 mov rdi,rax
            rdi = rax;
            // 27404 call 0xec0b0
            { let r = { let t = m.env.stub_va(0x87d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27409 lea rsi,[rip+0x1030a3] # 0x12a4b3
            rsi = m.env.data_va(257152);
            // 27410 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 27415 mov rdi,rax
            rdi = rax;
            // 27418 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2741d mov esi,DWORD PTR [rbx+0x28]
            rsi = (m.r32(rbx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 27420 mov rdi,rax
            rdi = rax;
            // 27423 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27428 lea rsi,[rip+0x10308d] # 0x12a4bc
            rsi = m.env.data_va(257161);
            // 2742f mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 27434 mov rdi,rax
            rdi = rax;
            // 27437 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2743c mov rsi,QWORD PTR [rbx+0x2c]
            rsi = m.r64(rbx.wrapping_add(0x2c)) as u64;
            // 27440 mov rdi,rax
            rdi = rax;
            // 27443 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27448 lea rsi,[rip+0x103078] # 0x12a4c7
            rsi = m.env.data_va(257172);
            // 2744f mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 27454 mov rdi,rax
            rdi = rax;
            // 27457 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2745c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2745f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 27463 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 27467 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2746a or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2746d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 27471 mov esi,DWORD PTR [rbx+0x34]
            rsi = (m.r32(rbx.wrapping_add(0x34)) as u64) as u32 as u64;
            // 27474 mov rdi,rax
            rdi = rax;
            // 27477 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2747c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2747f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 27483 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 27487 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2748a or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2748d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 27491 lea rsi,[rip+0x10303b] # 0x12a4d3
            rsi = m.env.data_va(257184);
            // 27498 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 2749d mov rdi,rax
            rdi = rax;
            // 274a0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 274a5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 274a8 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 274ac mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 274b0 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 274b3 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 274b6 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 274ba mov esi,DWORD PTR [rbx+0x38]
            rsi = (m.r32(rbx.wrapping_add(0x38)) as u64) as u32 as u64;
            // 274bd mov rdi,rax
            rdi = rax;
            // 274c0 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 274c5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 274c8 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 274cc mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 274d0 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 274d3 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 274d6 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 274da lea rsi,[rip+0x103001] # 0x12a4e2
            rsi = m.env.data_va(257199);
            // 274e1 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 274e6 mov rdi,rax
            rdi = rax;
            // 274e9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 274ee mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 274f1 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 274f5 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 274f9 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 274fc or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 274ff mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 27503 mov esi,DWORD PTR [rbx+0x3c]
            rsi = (m.r32(rbx.wrapping_add(0x3c)) as u64) as u32 as u64;
            // 27506 mov rdi,rax
            rdi = rax;
            // 27509 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2750e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 27511 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 27515 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 27519 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2751c or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2751f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 27523 lea rsi,[rip+0x102fc7] # 0x12a4f1
            rsi = m.env.data_va(257214);
            // 2752a mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 2752f mov rdi,rax
            rdi = rax;
            // 27532 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27537 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2753a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2753e mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 27542 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 27545 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 27548 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2754c mov esi,DWORD PTR [rbx+0x44]
            rsi = (m.r32(rbx.wrapping_add(0x44)) as u64) as u32 as u64;
            // 2754f mov rdi,rax
            rdi = rax;
            // 27552 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27557 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2755a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2755e mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 27562 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 27565 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 27568 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2756c lea rsi,[rip+0x102f8e] # 0x12a501
            rsi = m.env.data_va(257230);
            // 27573 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 27578 mov rdi,rax
            rdi = rax;
            // 2757b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27580 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 27583 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 27587 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2758b and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2758e or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 27591 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 27595 mov esi,DWORD PTR [rbx+0x40]
            rsi = (m.r32(rbx.wrapping_add(0x40)) as u64) as u32 as u64;
            // 27598 mov rdi,rax
            rdi = rax;
            // 2759b call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 275a0 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 275a3 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 275a7 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 275ab and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 275ae or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 275b1 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 275b5 lea rsi,[rip+0x102f55] # 0x12a511
            rsi = m.env.data_va(257246);
            // 275bc mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 275c1 mov rdi,rax
            rdi = rax;
            // 275c4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 275c9 mov esi,DWORD PTR [rbx+0x48]
            rsi = (m.r32(rbx.wrapping_add(0x48)) as u64) as u32 as u64;
            // 275cc mov rdi,rax
            rdi = rax;
            // 275cf call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 275d4 lea rsi,[rip+0x102f41] # 0x12a51c
            rsi = m.env.data_va(257257);
            // 275db mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 275e0 mov rdi,rax
            rdi = rax;
            // 275e3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 275e8 mov esi,DWORD PTR [rbx+0x4c]
            rsi = (m.r32(rbx.wrapping_add(0x4c)) as u64) as u32 as u64;
            // 275eb mov rdi,rax
            rdi = rax;
            // 275ee call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 275f3 lea rsi,[rip+0x102f2d] # 0x12a527
            rsi = m.env.data_va(257268);
            // 275fa mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 275ff mov rdi,rax
            rdi = rax;
            // 27602 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27607 mov rsi,QWORD PTR [rbx+0x78]
            rsi = m.r64(rbx.wrapping_add(0x78)) as u64;
            // 2760b mov rdi,rax
            rdi = rax;
            // 2760e call 0xec0b0
            { let r = { let t = m.env.stub_va(0x87d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27613 lea rsi,[rip+0x102f17] # 0x12a531
            rsi = m.env.data_va(257278);
            // 2761a mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 2761f mov rdi,rax
            rdi = rax;
            // 27622 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27627 mov esi,DWORD PTR [rbx+0x80]
            rsi = (m.r32(rbx.wrapping_add(0x80)) as u64) as u32 as u64;
            // 2762d mov rdi,rax
            rdi = rax;
            // 27630 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27635 lea rsi,[rip+0x102efe] # 0x12a53a
            rsi = m.env.data_va(257287);
            // 2763c mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 27641 mov rdi,rax
            rdi = rax;
            // 27644 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27649 mov rsi,QWORD PTR [rbx+0x84]
            rsi = m.r64(rbx.wrapping_add(0x84)) as u64;
            // 27650 mov rdi,rax
            rdi = rax;
            // 27653 call 0xec0b0
            { let r = { let t = m.env.stub_va(0x87d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27658 lea rsi,[rip+0x102ee8] # 0x12a547
            rsi = m.env.data_va(257300);
            // 2765f mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 27664 mov rdi,rax
            rdi = rax;
            // 27667 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2766c mov rsi,QWORD PTR [rbx+0x8c]
            rsi = m.r64(rbx.wrapping_add(0x8c)) as u64;
            // 27673 mov rdi,rax
            rdi = rax;
            // 27676 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2767b lea rsi,[rip+0x102ed5] # 0x12a557
            rsi = m.env.data_va(257316);
            // 27682 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 27687 mov rdi,rax
            rdi = rax;
            // 2768a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2768f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 27692 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 27696 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2769a and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2769d or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 276a0 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 276a4 mov esi,DWORD PTR [rbx+0x94]
            rsi = (m.r32(rbx.wrapping_add(0x94)) as u64) as u32 as u64;
            // 276aa mov rdi,rax
            rdi = rax;
            // 276ad call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 276b2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 276b5 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 276b9 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 276bd and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 276c0 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 276c3 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 276c7 lea rsi,[rip+0x102e98] # 0x12a566
            rsi = m.env.data_va(257331);
            // 276ce mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 276d3 mov rdi,rax
            rdi = rax;
            // 276d6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 276db mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 276de mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 276e2 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 276e6 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 276e9 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 276ec mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 276f0 mov esi,DWORD PTR [rbx+0x98]
            rsi = (m.r32(rbx.wrapping_add(0x98)) as u64) as u32 as u64;
            // 276f6 mov rdi,rax
            rdi = rax;
            // 276f9 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 276fe mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 27701 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 27705 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 27709 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2770c or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2770f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 27713 lea rsi,[rip+0x102e5c] # 0x12a576
            rsi = m.env.data_va(257347);
            // 2771a mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 2771f mov rdi,rax
            rdi = rax;
            // 27722 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27727 mov esi,DWORD PTR [rbx+0x9c]
            rsi = (m.r32(rbx.wrapping_add(0x9c)) as u64) as u32 as u64;
            // 2772d mov rdi,rax
            rdi = rax;
            // 27730 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27735 lea rsi,[rip+0x102e46] # 0x12a582
            rsi = m.env.data_va(257359);
            // 2773c mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 27741 mov rdi,rax
            rdi = rax;
            // 27744 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27749 mov esi,DWORD PTR [rbx+0xa0]
            rsi = (m.r32(rbx.wrapping_add(0xa0)) as u64) as u32 as u64;
            // 2774f mov rdi,rax
            rdi = rax;
            // 27752 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27757 lea rsi,[rip+0x102e2f] # 0x12a58d
            rsi = m.env.data_va(257370);
            // 2775e mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 27763 mov rdi,rax
            rdi = rax;
            // 27766 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2776b mov esi,DWORD PTR [rbx+0xa4]
            rsi = (m.r32(rbx.wrapping_add(0xa4)) as u64) as u32 as u64;
            // 27771 mov rdi,rax
            rdi = rax;
            // 27774 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27779 lea rsi,[rip+0x102e17] # 0x12a597
            rsi = m.env.data_va(257380);
            // 27780 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 27785 mov rdi,rax
            rdi = rax;
            // 27788 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2778d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 27790 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 27794 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 27798 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2779b or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2779e mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 277a2 mov rsi,QWORD PTR [rbx+0xa8]
            rsi = m.r64(rbx.wrapping_add(0xa8)) as u64;
            // 277a9 mov rdi,rax
            rdi = rax;
            // 277ac call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 277b1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 277b4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 277b8 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 277bc and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 277bf or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 277c2 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 277c6 lea rsi,[rip+0x102dd9] # 0x12a5a6
            rsi = m.env.data_va(257395);
            // 277cd mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 277d2 mov rdi,rax
            rdi = rax;
            // 277d5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 277da mov esi,DWORD PTR [rbx+0xb0]
            rsi = (m.r32(rbx.wrapping_add(0xb0)) as u64) as u32 as u64;
            // 277e0 mov rdi,rax
            rdi = rax;
            // 277e3 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 277e8 lea rsi,[rip+0x102dc1] # 0x12a5b0
            rsi = m.env.data_va(257405);
            // 277ef mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 277f4 mov rdi,rax
            rdi = rax;
            // 277f7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 277fc lea rsi,[rbx+0xb4]
            rsi = rbx.wrapping_add(0xb4);
            // 27803 mov rdi,rax
            rdi = rax;
            // 27806 call 0x26f9d
            { let r = { let t = m.env.code_va(0x26f9d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2780b lea rsi,[rip+0x102a82] # 0x12a294
            rsi = m.env.data_va(256609);
            // 27812 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 27817 mov rdi,rax
            rdi = rax;
            // 2781a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2781f add rbx,0xc0
            { let (r, f) = add::<64>(rbx, 0xc0u64);
              rbx = r;
              fl = f; }
            // 27826 mov rdi,rax
            rdi = rax;
            // 27829 mov rsi,rbx
            rsi = rbx;
            // 2782c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 27830 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 27831 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 27832 jmp 0x26804
            { let r = { let t = m.env.code_va(0x26804); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeab9c size=1082
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000eab9c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xeab9c;
    loop {
        match pc {
        0xeab9c => {
            // block eab9c
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eab9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab9c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eab9d mov rbp,rsp
            rbp = rsp;
            // eaba0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eaba2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eaba4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // eaba6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // eaba8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eaba9 sub rsp,0x2a8
            { let (r, f) = sub::<64>(rsp, 0x2a8u64);
              rsp = r;
              fl = f; }
            // eabb0 mov rbx,rsi
            rbx = rsi;
            // eabb3 mov r14,rdi
            r14 = rdi;
            // eabb6 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // eabb9 call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eabbf mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // eabc2 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // eabc5 jne 0xeafa3
            if fl.ne() { pc = 0xeafa3; continue; }
            pc = 0xeabcb;
        }
        0xeabcb => {
            // block eabcb
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eabcb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eabcb mov rbx,QWORD PTR [rbx+0x30]
            rbx = m.r64(rbx.wrapping_add(0x30)) as u64;
            // eabcf xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // eabd2 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // eabd5 je 0xeafa3
            if fl.e() { pc = 0xeafa3; continue; }
            pc = 0xeabdb;
        }
        0xeabdb => {
            // block eabdb
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eabdb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eabdb lea r12,[rbp-0x2b0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eabe2 mov rdi,r12
            rdi = r12;
            // eabe5 call 0xb6722
            { let r = { let t = m.env.code_va(0xb6722); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eabea mov edi,DWORD PTR [r14+0x5c]
            rdi = (m.r32(r14.wrapping_add(0x5c)) as u64) as u32 as u64;
            // eabee mov rsi,r12
            rsi = r12;
            // eabf1 call 0x17202
            { let r = { let t = m.env.code_va(0x17202); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eabf6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eabf8 je 0xeaf97
            if fl.e() { pc = 0xeaf97; continue; }
            pc = 0xeabfe;
        }
        0xeabfe => {
            // block eabfe
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eabfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eabfe mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // eac00 mov DWORD PTR [rbp-0x2b4],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c), ((rax as u32 as u64)) as u32);
            // eac06 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // eac08 je 0xeaf97
            if fl.e() { pc = 0xeaf97; continue; }
            pc = 0xeac0e;
        }
        0xeac0e => {
            // block eac0e
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac0e lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eac15 call 0xb6914
            { let r = { let t = m.env.code_va(0xb6914); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eac1a mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // eac1d mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // eac20 call 0xcc7a3
            { let r = { let t = m.env.code_va(0xcc7a3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eac25 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eac27 je 0xeac47
            if fl.e() { pc = 0xeac47; continue; }
            pc = 0xeac29;
        }
        0xeac29 => {
            // block eac29
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac29 lea rsi,[r14+0x68]
            rsi = r14.wrapping_add(0x68);
            // eac2d lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eac34 call 0xe90c
            { let r = { let t = m.env.code_va(0xe90c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eac39 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eac40 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eac42 call 0xe8cc
            { let r = { let t = m.env.code_va(0xe8cc); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeac47;
        }
        0xeac47 => {
            // block eac47
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac47 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // eac4a call 0xcc147
            { let r = { let t = m.env.code_va(0xcc147); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eac4f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eac51 jne 0xeac5b
            if fl.ne() { pc = 0xeac5b; continue; }
            pc = 0xeac53;
        }
        0xeac53 => {
            // block eac53
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac53 mov DWORD PTR [r14+0x60],0x1
            m.w32(r14.wrapping_add(0x60), (0x1u64) as u32);
            pc = 0xeac5b;
        }
        0xeac5b => {
            // block eac5b
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac5b mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // eac5e mov esi,DWORD PTR [rbp-0x2b4]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c)) as u64) as u32 as u64;
            // eac64 call 0xc8c60
            { let r = { let t = m.env.code_va(0xc8c60); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eac69 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eac6b je 0xeaf97
            if fl.e() { pc = 0xeaf97; continue; }
            pc = 0xeac71;
        }
        0xeac71 => {
            // block eac71
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac71 mov ebx,DWORD PTR [r14+0x60]
            rbx = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // eac75 cmp ebx,DWORD PTR [r14+0x64]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x64)) as u64).1;
            // eac79 jg 0xeacca
            if fl.g() { pc = 0xeacca; continue; }
            pc = 0xeac7b;
        }
        0xeac7b => {
            // block eac7b
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac7b lea r13,[rbp-0x2b0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            pc = 0xeac82;
        }
        0xeac82 => {
            // block eac82
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac82 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eac84 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eac86 mov rdi,r13
            rdi = r13;
            // eac89 mov esi,DWORD PTR [rbp-0x2b4]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c)) as u64) as u32 as u64;
            // eac8f mov r8d,ebx
            r8 = ((rbx as u32 as u64)) as u32 as u64;
            // eac92 call 0x78
            { let r = { let t = m.env.code_va(0x78); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eac97 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eac99 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eac9b mov rdi,r13
            rdi = r13;
            // eac9e mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // eaca0 call 0x50a0
            { let r = { let t = m.env.code_va(0x50a0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaca5 movzx esi,bx
            rsi = ((rbx as u16 as u64)) as u32 as u64;
            // eaca8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eacaa mov rdi,r13
            rdi = r13;
            // eacad call 0xe70c
            { let r = { let t = m.env.code_va(0xe70c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eacb2 movzx esi,bx
            rsi = ((rbx as u16 as u64)) as u32 as u64;
            // eacb5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eacb7 mov rdi,r13
            rdi = r13;
            // eacba call 0xe7ec
            { let r = { let t = m.env.code_va(0xe7ec); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eacbf cmp ebx,DWORD PTR [r14+0x64]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x64)) as u64).1;
            // eacc3 lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // eacc6 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // eacc8 jl 0xeac82
            if fl.l() { pc = 0xeac82; continue; }
            pc = 0xeacca;
        }
        0xeacca => {
            // block eacca
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eacca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eacca lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eacd1 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // eacd6 call 0x1d1a
            { let r = { let t = m.env.code_va(0x1d1a); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eacdb mov r13d,0x2
            r13 = (0x2u64) as u32 as u64;
            // eace1 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // eace6 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // eace9 call 0xc9e94
            { let r = { let t = m.env.code_va(0xc9e94); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eacee test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eacf0 jne 0xead06
            if fl.ne() { pc = 0xead06; continue; }
            pc = 0xeacf2;
        }
        0xeacf2 => {
            // block eacf2
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eacf2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eacf2 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // eacf7 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // eacfa call 0xc9e94
            { let r = { let t = m.env.code_va(0xc9e94); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eacff movzx r13d,al
            r13 = ((rax as u8 as u64)) as u32 as u64;
            // ead03 inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            pc = 0xead06;
        }
        0xead06 => {
            // block ead06
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc ead06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ead06 mov ebx,DWORD PTR [r14+0x60]
            rbx = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // ead0a cmp ebx,DWORD PTR [r14+0x64]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x64)) as u64).1;
            // ead0e jg 0xead61
            if fl.g() { pc = 0xead61; continue; }
            pc = 0xead10;
        }
        0xead10 => {
            // block ead10
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc ead10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ead10 mov DWORD PTR [rbp-0x2b8],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd48), ((r12 as u32 as u64)) as u32);
            // ead17 lea r12,[rbp-0x2b0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            pc = 0xead1e;
        }
        0xead1e => {
            // block ead1e
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc ead1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ead1e mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // ead20 lea rdx,[r14+rax*4+0x6c]
            rdx = r14.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x6c);
            // ead25 mov rdi,r12
            rdi = r12;
            // ead28 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // ead2a call 0x2522
            { let r = { let t = m.env.code_va(0x2522); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead2f mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // ead34 mov rdi,r12
            rdi = r12;
            // ead37 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // ead39 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // ead3c call 0x23fc
            { let r = { let t = m.env.code_va(0x23fc); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead41 mov rdi,r12
            rdi = r12;
            // ead44 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // ead46 call 0x2964
            { let r = { let t = m.env.code_va(0x2964); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead4b cmp ebx,DWORD PTR [r14+0x64]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x64)) as u64).1;
            // ead4f lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // ead52 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // ead54 jl 0xead1e
            if fl.l() { pc = 0xead1e; continue; }
            pc = 0xead56;
        }
        0xead56 => {
            // block ead56
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc ead56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ead56 mov ebx,DWORD PTR [r14+0x60]
            rbx = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // ead5a mov r12d,DWORD PTR [rbp-0x2b8]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd48)) as u64) as u32 as u64;
            pc = 0xead61;
        }
        0xead61 => {
            // block ead61
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc ead61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ead61 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // ead68 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // ead6a call 0x6806e
            { let r = { let t = m.env.code_va(0x6806e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead6f mov edi,DWORD PTR [rbp-0x2b4]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c)) as u64) as u32 as u64;
            // ead75 call 0x5a914
            { let r = { let t = m.env.code_va(0x5a914); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead7a mov DWORD PTR [rbp-0x2b4],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c), ((rax as u32 as u64)) as u32);
            // ead80 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // ead83 call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead88 mov WORD PTR [rbp-0x2c2],ax
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_fd3e), ((rax as u16 as u64)) as u16);
            // ead8f mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // ead92 call 0x5caed
            { let r = { let t = m.env.code_va(0x5caed); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead97 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // ead9a mov ebx,DWORD PTR [r14+0x60]
            rbx = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // ead9e mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // eada1 call 0xcfa3b
            { let r = { let t = m.env.code_va(0xcfa3b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eada6 movzx ecx,bx
            rcx = ((rbx as u16 as u64)) as u32 as u64;
            // eada9 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // eadac cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // eadae jbe 0xeadc7
            if fl.be() { pc = 0xeadc7; continue; }
            pc = 0xeadb0;
        }
        0xeadb0 => {
            // block eadb0
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eadb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eadb0 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eadb7 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eadbc mov r15d,0xffffffff
            r15 = (0xffffffffu64) as u32 as u64;
            // eadc2 jmp 0xeafa3
            pc = 0xeafa3; continue;
        }
        0xeadc7 => {
            // block eadc7
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eadc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eadc7 mov edi,DWORD PTR [r14+0x60]
            rdi = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // eadcb mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // eadce xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // eadd0 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // eadd3 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eadd5 call 0xbbacd
            { let r = { let t = m.env.code_va(0xbbacd); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eadda mov DWORD PTR [rbp-0x2c0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd40), ((rax as u32 as u64)) as u32);
            // eade0 mov edi,DWORD PTR [r14+0x60]
            rdi = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // eade4 movzx esi,r13b
            rsi = ((r13 as u8 as u64)) as u32 as u64;
            // eade8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eadea call 0xbbb07
            { let r = { let t = m.env.code_va(0xbbb07); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eadef mov DWORD PTR [rbp-0x2bc],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd44), ((rax as u32 as u64)) as u32);
            // eadf5 test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // eadf8 je 0xeae1b
            if fl.e() { pc = 0xeae1b; continue; }
            pc = 0xeadfa;
        }
        0xeadfa => {
            // block eadfa
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eadfa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eadfa mov edi,DWORD PTR [r14+0x60]
            rdi = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // eadfe xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eae00 call 0xbba5d
            { let r = { let t = m.env.code_va(0xbba5d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eae05 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eae0c xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eae0e mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // eae10 mov edx,DWORD PTR [rbp-0x2bc]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd44)) as u64) as u32 as u64;
            // eae16 call 0x8e30
            { let r = { let t = m.env.code_va(0x8e30); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeae1b;
        }
        0xeae1b => {
            // block eae1b
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eae1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eae1b lea rsi,[r14+0x38]
            rsi = r14.wrapping_add(0x38);
            // eae1f lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eae26 call 0x914a
            { let r = { let t = m.env.code_va(0x914a); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eae2b lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eae32 call 0x9046
            { let r = { let t = m.env.code_va(0x9046); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eae37 mov edi,DWORD PTR [r14+0x60]
            rdi = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // eae3b xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eae3d call 0xbba5d
            { let r = { let t = m.env.code_va(0xbba5d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eae42 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eae49 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eae4b mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // eae4d mov edx,DWORD PTR [rbp-0x2bc]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd44)) as u64) as u32 as u64;
            // eae53 call 0x8e30
            { let r = { let t = m.env.code_va(0x8e30); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eae58 mov BYTE PTR [rbp-0x2c3],r13b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fd3d), ((r13 as u8 as u64)) as u8);
            // eae5f mov DWORD PTR [rbp-0x2b8],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd48), ((r12 as u32 as u64)) as u32);
            // eae66 mov ax,WORD PTR [rbp-0x2c2]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_fd3e)) as u64) as u16 as u64);
            // eae6d test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // eae70 je 0xeaee5
            if fl.e() { pc = 0xeaee5; continue; }
            pc = 0xeae72;
        }
        0xeae72 => {
            // block eae72
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eae72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eae72 movzx r12d,ax
            r12 = ((rax as u16 as u64)) as u32 as u64;
            // eae76 cmp BYTE PTR [rbp-0x2c3],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd3d)) as u64, 0x0u64).1;
            // eae7d mov ebx,DWORD PTR [rbp-0x2bc]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd44)) as u64) as u32 as u64;
            // eae83 cmovne ebx,DWORD PTR [rbp-0x2c0]
            if fl.ne() { rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd40)) as u64) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            // eae8a xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // eae8d lea r14,[rbp-0x2b0]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            pc = 0xeae94;
        }
        0xeae94 => {
            // block eae94
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eae94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eae94 mov edi,DWORD PTR [rbp-0x2b8]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd48)) as u64) as u32 as u64;
            // eae9a call 0xcd5c3
            { let r = { let t = m.env.code_va(0xcd5c3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eae9f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaea1 je 0xeaeb3
            if fl.e() { pc = 0xeaeb3; continue; }
            pc = 0xeaea3;
        }
        0xeaea3 => {
            // block eaea3
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaea3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaea3 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // eaea8 mov rdi,r14
            rdi = r14;
            // eaeab mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // eaeae call 0xd778
            { let r = { let t = m.env.code_va(0xd778); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeaeb3;
        }
        0xeaeb3 => {
            // block eaeb3
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaeb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaeb3 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eaeb5 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // eaeb8 call 0xbbc9b
            { let r = { let t = m.env.code_va(0xbbc9b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaebd xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eaebf mov rdi,r14
            rdi = r14;
            // eaec2 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // eaec4 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // eaec6 call 0x8e30
            { let r = { let t = m.env.code_va(0x8e30); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaecb movzx esi,r13w
            rsi = ((r13 as u16 as u64)) as u32 as u64;
            // eaecf mov rdi,r14
            rdi = r14;
            // eaed2 mov edx,DWORD PTR [rbp-0x2b4]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c)) as u64) as u32 as u64;
            // eaed8 call 0x6c96
            { let r = { let t = m.env.code_va(0x6c96); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaedd inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // eaee0 cmp r13d,r12d
            fl = sub::<32>((r13 as u32 as u64), (r12 as u32 as u64)).1;
            // eaee3 jb 0xeae94
            if fl.b() { pc = 0xeae94; continue; }
            pc = 0xeaee5;
        }
        0xeaee5 => {
            // block eaee5
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaee5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaee5 mov esi,0x20
            rsi = (0x20u64) as u32 as u64;
            // eaeea mov ebx,DWORD PTR [rbp-0x2b8]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd48)) as u64) as u32 as u64;
            // eaef0 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // eaef2 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaef7 mov r14b,BYTE PTR [rbp-0x2c3]
            r14 = (r14 & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd3d)) as u64) as u8 as u64);
            // eaefe test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaf00 je 0xeaf30
            if fl.e() { pc = 0xeaf30; continue; }
            pc = 0xeaf02;
        }
        0xeaf02 => {
            // block eaf02
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf02 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            // eaf04 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eaf06 mov edx,0x63
            rdx = (0x63u64) as u32 as u64;
            // eaf0b call 0xbbc2c
            { let r = { let t = m.env.code_va(0xbbc2c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaf10 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // eaf13 mov edx,DWORD PTR [rbp-0x2bc]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd44)) as u64) as u32 as u64;
            // eaf19 cmovne edx,DWORD PTR [rbp-0x2c0]
            if fl.ne() { rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd40)) as u64) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // eaf20 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eaf27 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eaf29 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // eaf2b call 0x8e30
            { let r = { let t = m.env.code_va(0x8e30); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeaf30;
        }
        0xeaf30 => {
            // block eaf30
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf30 mov esi,0x23
            rsi = (0x23u64) as u32 as u64;
            // eaf35 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // eaf37 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaf3c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaf3e jne 0xeaf70
            if fl.ne() { pc = 0xeaf70; continue; }
            pc = 0xeaf40;
        }
        0xeaf40 => {
            // block eaf40
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf40 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // eaf45 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // eaf47 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaf4c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaf4e jne 0xeaf70
            if fl.ne() { pc = 0xeaf70; continue; }
            pc = 0xeaf50;
        }
        0xeaf50 => {
            // block eaf50
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf50 mov esi,0x5d
            rsi = (0x5du64) as u32 as u64;
            // eaf55 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // eaf57 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaf5c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaf5e jne 0xeaf70
            if fl.ne() { pc = 0xeaf70; continue; }
            pc = 0xeaf60;
        }
        0xeaf60 => {
            // block eaf60
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf60 mov esi,0x6b
            rsi = (0x6bu64) as u32 as u64;
            // eaf65 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // eaf67 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaf6c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaf6e je 0xeaf97
            if fl.e() { pc = 0xeaf97; continue; }
            pc = 0xeaf70;
        }
        0xeaf70 => {
            // block eaf70
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf70 mov edi,0x1
            rdi = (0x1u64) as u32 as u64;
            // eaf75 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eaf77 mov edx,0x63
            rdx = (0x63u64) as u32 as u64;
            // eaf7c call 0xbbc2c
            { let r = { let t = m.env.code_va(0xbbc2c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaf81 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eaf88 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eaf8a mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // eaf8c mov edx,DWORD PTR [rbp-0x2c0]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd40)) as u64) as u32 as u64;
            // eaf92 call 0x8e30
            { let r = { let t = m.env.code_va(0x8e30); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeaf97;
        }
        0xeaf97 => {
            // block eaf97
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf97 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eaf9e call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeafa3;
        }
        0xeafa3 => {
            // block eafa3
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eafa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eafa3 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // eafa6 add rsp,0x2a8
            { let (r, f) = add::<64>(rsp, 0x2a8u64);
              rsp = r;
              fl = f; }
            // eafad pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eafae pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eafb0 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // eafb2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eafb4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eafb6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eafb7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xeafb8 => {
            // block eafb8
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eafb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eafb8 jmp 0xeafbe
            pc = 0xeafbe; continue;
        }
        0xeafba => {
            // block eafba
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eafba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eafba jmp 0xeafbe
            pc = 0xeafbe; continue;
        }
        0xeafbc => {
            // block eafbc
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eafbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eafbc jmp 0xeafbe
            pc = 0xeafbe; continue;
        }
        0xeafbe => {
            // block eafbe
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eafbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eafbe mov rbx,rax
            rbx = rax;
            // eafc1 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eafc8 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eafcd mov rdi,rbx
            rdi = rbx;
            // eafd0 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x54852 size=1032
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_00054852<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x54852;
    loop {
        match pc {
        0x54852 => {
            // block 54852
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54852 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54852 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 54853 mov rbp,rsp
            rbp = rsp;
            // 54856 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 54858 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5485a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 5485c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 5485e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5485f sub rsp,0x188
            { let (r, f) = sub::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 54866 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 54868 mov r15,QWORD PTR [rip+0xebb69] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 5486f mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 54872 mov QWORD PTR [rbp-0x30],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r15) as u64);
            // 54876 cmp ebx,0x20
            fl = sub::<32>((rbx as u32 as u64), 0x20u64).1;
            // 54879 jb 0x54a60
            if fl.b() { pc = 0x54a60; continue; }
            pc = 0x5487f;
        }
        0x5487f => {
            // block 5487f
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 5487f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5487f test BYTE PTR [rip+0x173a11],0x20 # 0x1c8297
            fl = logic::<8>(m.r8(m.env.data_va(903780)) as u64 & 0x20u64);
            // 54886 je 0x54ba3
            if fl.e() { pc = 0x54ba3; continue; }
            pc = 0x5488c;
        }
        0x5488c => {
            // block 5488c
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 5488c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5488c lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 54893 lea r13,[rbp-0x160]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 5489a mov rax,QWORD PTR [rip+0xeb92f] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 548a1 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 548a5 mov QWORD PTR [rbp-0x168],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rcx) as u64);
            // 548ac add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 548b0 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // 548b7 mov rsi,r13
            rsi = r13;
            // 548ba call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 548bf mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // 548c7 mov DWORD PTR [rbp-0x68],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0xffffffffu64) as u32);
            // 548ce mov r14,QWORD PTR [rip+0xebaeb] # 0x1403c0
            r14 = m.r64(m.env.data_va(347021)) as u64;
            // 548d5 lea rax,[r14+0x18]
            rax = r14.wrapping_add(0x18);
            // 548d9 mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 548e0 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 548e7 add r14,0x40
            { let (r, f) = add::<64>(r14, 0x40u64);
              r14 = r;
              fl = f; }
            // 548eb mov QWORD PTR [rbp-0xf8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r14) as u64);
            // 548f2 mov rdi,r13
            rdi = r13;
            // 548f5 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 548fa mov r12,QWORD PTR [rip+0xebaa7] # 0x1403a8
            r12 = m.r64(m.env.data_va(346997)) as u64;
            // 54901 add r12,0x10
            { let (r, f) = add::<64>(r12, 0x10u64);
              r12 = r;
              fl = f; }
            // 54905 mov QWORD PTR [rbp-0x160],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r12) as u64);
            // 5490c mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // 54917 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 54922 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 5492d mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 54938 mov DWORD PTR [rbp-0x100],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x10u64) as u32);
            // 54942 xorps xmm0,xmm0
            x0 = 0;
            // 54945 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 54949 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 54951 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 54955 mov rdi,r13
            rdi = r13;
            // 54958 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5495d lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 54961 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54966 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5496b mov r13,rax
            r13 = rax;
            // 5496e lea rsi,[rip+0xd634c] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 54975 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 5497c mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 54981 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54986 lea rsi,[rip+0xd66f3] # 0x12b080
            rsi = m.env.data_va(260173);
            // 5498d mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 54992 mov rdi,rax
            rdi = rax;
            // 54995 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5499a lea rsi,[rip+0xd5fdf] # 0x12a980
            rsi = m.env.data_va(258381);
            // 549a1 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 549a6 mov rdi,rax
            rdi = rax;
            // 549a9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 549ae lea rsi,[rip+0xd5ac0] # 0x12a475
            rsi = m.env.data_va(257090);
            // 549b5 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 549ba mov rdi,rax
            rdi = rax;
            // 549bd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 549c2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 549c5 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 549c9 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 549cd and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 549d0 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 549d3 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 549d7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 549da mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 549de mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 549e7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 549ea mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 549ee mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 549f9 mov rdi,rax
            rdi = rax;
            // 549fc mov rsi,r13
            rsi = r13;
            // 549ff call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54a04 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 54a07 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 54a0b mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 54a0f and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 54a12 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 54a15 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 54a19 lea rsi,[rip+0xd5f60] # 0x12a980
            rsi = m.env.data_va(258381);
            // 54a20 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 54a25 mov rdi,rax
            rdi = rax;
            // 54a28 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54a2d mov r13,rax
            r13 = rax;
            // 54a30 lea rsi,[rip+0xd68e9] # 0x12b320
            rsi = m.env.data_va(260845);
            // 54a37 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 54a3e mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 54a43 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54a48 movzx edx,BYTE PTR [rbp-0x180]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64) as u32 as u64;
            // 54a4f test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 54a52 jne 0x54aa9
            if fl.ne() { pc = 0x54aa9; continue; }
            pc = 0x54a54;
        }
        0x54a54 => {
            // block 54a54
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54a54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54a54 lea rsi,[rbp-0x17f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe81);
            // 54a5b shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 54a5e jmp 0x54ab7
            pc = 0x54ab7; continue;
        }
        0x54a60 => {
            // block 54a60
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54a60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54a60 movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // 54a63 mov edi,DWORD PTR [rdi+rax*4]
            rdi = (m.r32(rdi.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 54a66 test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // 54a68 je 0x54ba3
            if fl.e() { pc = 0x54ba3; continue; }
            pc = 0x54a6e;
        }
        0x54a6e => {
            // block 54a6e
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54a6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54a6e xorps xmm0,xmm0
            x0 = 0;
            // 54a71 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 54a75 mov DWORD PTR [rbp-0x19c],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe64), (0x2u64) as u32);
            // 54a7f lea r8,[rbp-0x40]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 54a83 lea r9,[rbp-0x19c]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_fe64);
            // 54a8a mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 54a8f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 54a91 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 54a93 call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54a98 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 54a9a mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 54a9c test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 54a9e jne 0x54ba3
            if fl.ne() { pc = 0x54ba3; continue; }
            pc = 0x54aa4;
        }
        0x54aa4 => {
            // block 54aa4
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54aa4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54aa4 jmp 0x54ba5
            pc = 0x54ba5; continue;
        }
        0x54aa9 => {
            // block 54aa9
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54aa9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54aa9 mov rdx,QWORD PTR [rbp-0x178]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 54ab0 mov rsi,QWORD PTR [rbp-0x170]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            pc = 0x54ab7;
        }
        0x54ab7 => {
            // block 54ab7
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54ab7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54ab7 mov rdi,r13
            rdi = r13;
            // 54aba call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54abf lea rsi,[rip+0xd5eba] # 0x12a980
            rsi = m.env.data_va(258381);
            // 54ac6 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 54acb mov rdi,rax
            rdi = rax;
            // 54ace call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54ad3 lea rsi,[rip+0xd685c] # 0x12b336
            rsi = m.env.data_va(260867);
            // 54ada mov edx,0x36
            rdx = (0x36u64) as u32 as u64;
            // 54adf mov rdi,rax
            rdi = rax;
            // 54ae2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54ae7 movzx esi,bx
            rsi = ((rbx as u16 as u64)) as u32 as u64;
            // 54aea mov rdi,rax
            rdi = rax;
            // 54aed call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54af2 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 54af9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54afe lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // 54b05 lea rsi,[rbp-0x160]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 54b0c call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54b11 test BYTE PTR [rbp-0x198],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64 & 0x1u64);
            // 54b18 jne 0x54b23
            if fl.ne() { pc = 0x54b23; continue; }
            pc = 0x54b1a;
        }
        0x54b1a => {
            // block 54b1a
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54b1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54b1a lea rdx,[rbp-0x197]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe69);
            // 54b21 jmp 0x54b2a
            pc = 0x54b2a; continue;
        }
        0x54b23 => {
            // block 54b23
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54b23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54b23 mov rdx,QWORD PTR [rbp-0x188]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            pc = 0x54b2a;
        }
        0x54b2a => {
            // block 54b2a
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54b2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54b2a lea rsi,[rip+0xd61e8] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 54b31 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 54b36 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 54b38 lea rbx,[rbp-0x160]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 54b3f call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54b44 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // 54b4b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54b50 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 54b57 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 54b5e mov QWORD PTR [rbp-0xf8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r14) as u64);
            // 54b65 mov QWORD PTR [rbp-0x160],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r12) as u64);
            // 54b6c lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 54b73 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54b78 mov rdi,rbx
            rdi = rbx;
            // 54b7b call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54b80 mov rsi,QWORD PTR [rip+0xeb681] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 54b87 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 54b8b lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 54b92 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54b97 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 54b9e call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x54ba3;
        }
        0x54ba3 => {
            // block 54ba3
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54ba3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54ba3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x54ba5;
        }
        0x54ba5 => {
            // block 54ba5
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54ba5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54ba5 cmp r15,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 54ba9 jne 0x54bce
            if fl.ne() { pc = 0x54bce; continue; }
            pc = 0x54bab;
        }
        0x54bab => {
            // block 54bab
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54bab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54bab add rsp,0x188
            { let (r, f) = add::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 54bb2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 54bb3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 54bb5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 54bb7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 54bb9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 54bbb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 54bbc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x54bbd => {
            // block 54bbd
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54bbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54bbd mov rbx,rax
            rbx = rax;
            // 54bc0 jmp 0x54bfa
            pc = 0x54bfa; continue;
        }
        0x54bc2 => {
            // block 54bc2
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54bc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54bc2 mov rbx,rax
            rbx = rax;
            // 54bc5 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 54bcc jmp 0x54bf5
            pc = 0x54bf5; continue;
        }
        0x54bce => {
            // block 54bce
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54bce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54bce call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x54bd8 => {
            // block 54bd8
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54bd8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54bd8 mov rbx,rax
            rbx = rax;
            // 54bdb jmp 0x54c2e
            pc = 0x54c2e; continue;
        }
        0x54bdd => {
            // block 54bdd
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54bdd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54bdd mov rbx,rax
            rbx = rax;
            // 54be0 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 54be4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54be9 jmp 0x54c16
            pc = 0x54c16; continue;
        }
        0x54beb => {
            // block 54beb
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54beb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54beb mov rbx,rax
            rbx = rax;
            // 54bee lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            pc = 0x54bf5;
        }
        0x54bf5 => {
            // block 54bf5
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54bf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54bf5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x54bfa;
        }
        0x54bfa => {
            // block 54bfa
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54bfa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54bfa mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 54c01 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 54c08 mov QWORD PTR [rbp-0xf8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r14) as u64);
            // 54c0f mov QWORD PTR [rbp-0x160],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r12) as u64);
            pc = 0x54c16;
        }
        0x54c16 => {
            // block 54c16
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54c16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54c16 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 54c1d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54c22 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 54c29 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x54c2e;
        }
        0x54c2e => {
            // block 54c2e
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54c2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54c2e mov rsi,QWORD PTR [rip+0xeb5d3] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 54c35 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 54c39 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 54c40 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x54c45;
        }
        0x54c45 => {
            // block 54c45
            #[cfg(feature = "trace")] eprintln!("f_00054852 pc 54c45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54c45 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 54c4c call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 54c51 mov rdi,rbx
            rdi = rbx;
            // 54c54 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xaeb0c size=1006
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000aeb0c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xaeb0c;
    loop {
        match pc {
        0xaeb0c => {
            // block aeb0c
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aeb0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aeb0c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // aeb0d mov rbp,rsp
            rbp = rsp;
            // aeb10 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // aeb12 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // aeb14 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // aeb16 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // aeb18 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // aeb19 sub rsp,0x198
            { let (r, f) = sub::<64>(rsp, 0x198u64);
              rsp = r;
              fl = f; }
            // aeb20 mov DWORD PTR [rbp-0x190],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe70), ((rcx as u32 as u64)) as u32);
            // aeb26 mov DWORD PTR [rbp-0x194],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe6c), ((rdx as u32 as u64)) as u32);
            // aeb2c mov r12,rdi
            r12 = rdi;
            // aeb2f lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // aeb36 lea r14,[rbp-0x150]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // aeb3d mov rax,QWORD PTR [rip+0x9168c] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // aeb44 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // aeb48 mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // aeb4f add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // aeb53 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // aeb5a mov rsi,r14
            rsi = r14;
            // aeb5d call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aeb62 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // aeb6a mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // aeb71 mov rbx,QWORD PTR [rip+0x91848] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // aeb78 lea r15,[rbx+0x18]
            r15 = rbx.wrapping_add(0x18);
            // aeb7c mov QWORD PTR [rbp-0x158],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (r15) as u64);
            // aeb83 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // aeb87 mov QWORD PTR [rbp-0xe8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rbx) as u64);
            // aeb8e mov rdi,r14
            rdi = r14;
            // aeb91 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aeb96 mov QWORD PTR [rbp-0x1b8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe48), (r12) as u64);
            // aeb9d mov QWORD PTR [rbp-0x1a8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (r15) as u64);
            // aeba4 mov QWORD PTR [rbp-0x1a0],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe60), (rbx) as u64);
            // aebab mov rax,QWORD PTR [rip+0x917f6] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // aebb2 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // aebb6 mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            // aebbd mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // aebc4 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // aebcf mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // aebda mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // aebe5 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // aebf0 mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // aebfa xorps xmm0,xmm0
            x0 = 0;
            // aebfd movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // aec01 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // aec09 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // aec0d mov rdi,r14
            rdi = r14;
            // aec10 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aec15 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // aec19 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aec1e lea r13,[rip+0x992cb] # 0x147ef0
            r13 = m.env.data_va(378557);
            // aec25 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // aec28 lea r12,[rip+0x119949] # 0x1c8578
            r12 = m.env.data_va(904517);
            // aec2f lea r14,[rbp-0x38]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            pc = 0xaec33;
        }
        0xaec33 => {
            // block aec33
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aec33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aec33 mov eax,DWORD PTR [rbp-0x194]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe6c)) as u64) as u32 as u64;
            // aec39 mov DWORD PTR [rbp-0x38],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rax as u32 as u64)) as u32);
            // aec3c mov DWORD PTR [rbp-0x34],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((r15 as u32 as u64)) as u32);
            // aec40 mov rdi,r12
            rdi = r12;
            // aec43 mov rsi,r14
            rsi = r14;
            // aec46 call 0xebbdc
            { let r = { let t = m.env.stub_va(0x3a9); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aec4b mov esi,0xffffffff
            rsi = (0xffffffffu64) as u32 as u64;
            // aec50 lea rcx,[rip+0x119929] # 0x1c8580
            rcx = m.env.data_va(904525);
            // aec57 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // aec5a je 0xaec5f
            if fl.e() { pc = 0xaec5f; continue; }
            pc = 0xaec5c;
        }
        0xaec5c => {
            // block aec5c
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aec5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aec5c mov esi,DWORD PTR [rax+0x24]
            rsi = (m.r32(rax.wrapping_add(0x24)) as u64) as u32 as u64;
            pc = 0xaec5f;
        }
        0xaec5f => {
            // block aec5f
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aec5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aec5f mov cl,BYTE PTR [r13+0x0]
            rcx = (rcx & !0xff) | ((m.r8(r13) as u64) as u8 as u64);
            // aec63 mov ebx,DWORD PTR [rbp-0x190]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64) as u32 as u64;
            // aec69 shr ebx,cl
            { let (r, f) = shr::<32>((rbx as u32 as u64), (rcx as u8 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // aec6b lea eax,[rsi-0x1]
            rax = (rsi.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // aec6e cmp eax,0x84
            fl = sub::<32>((rax as u32 as u64), 0x84u64).1;
            // aec73 ja 0xaed31
            if fl.a() { pc = 0xaed31; continue; }
            pc = 0xaec79;
        }
        0xaec79 => {
            // block aec79
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aec79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aec79 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aec7b lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // aec82 call 0x5d8db
            { let r = { let t = m.env.code_va(0x5d8db); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aec87 movzx edx,BYTE PTR [rbp-0x170]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64) as u32 as u64;
            // aec8e mov r14,r12
            r14 = r12;
            // aec91 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // aec94 jne 0xaeca2
            if fl.ne() { pc = 0xaeca2; continue; }
            pc = 0xaec96;
        }
        0xaec96 => {
            // block aec96
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aec96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aec96 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // aec99 lea rsi,[rbp-0x16f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe91);
            // aeca0 jmp 0xaecb0
            pc = 0xaecb0; continue;
        }
        0xaeca2 => {
            // block aeca2
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aeca2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aeca2 mov rdx,QWORD PTR [rbp-0x168]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // aeca9 mov rsi,QWORD PTR [rbp-0x160]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            pc = 0xaecb0;
        }
        0xaecb0 => {
            // block aecb0
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aecb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aecb0 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // aecb7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aecbc mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // aecc1 mov rdi,rax
            rdi = rax;
            // aecc4 lea rsi,[rip+0x860c1] # 0x134d8c
            rsi = m.env.data_va(300377);
            // aeccb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aecd0 mov r12,rax
            r12 = rax;
            // aecd3 movzx esi,bl
            rsi = ((rbx as u8 as u64)) as u32 as u64;
            // aecd6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aecd8 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // aecdf call 0x5e61c
            { let r = { let t = m.env.code_va(0x5e61c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aece4 movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // aeceb test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // aecee jne 0xaecfc
            if fl.ne() { pc = 0xaecfc; continue; }
            pc = 0xaecf0;
        }
        0xaecf0 => {
            // block aecf0
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aecf0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aecf0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // aecf3 lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // aecfa jmp 0xaed0a
            pc = 0xaed0a; continue;
        }
        0xaecfc => {
            // block aecfc
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aecfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aecfc mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // aed03 mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0xaed0a;
        }
        0xaed0a => {
            // block aed0a
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aed0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aed0a mov rdi,r12
            rdi = r12;
            // aed0d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aed12 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // aed19 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aed1e lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // aed25 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aed2a mov r12,r14
            r12 = r14;
            // aed2d lea r14,[rbp-0x38]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            pc = 0xaed31;
        }
        0xaed31 => {
            // block aed31
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aed31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aed31 cmp r15,0x2
            fl = sub::<64>(r15, 0x2u64).1;
            // aed35 ja 0xaed9c
            if fl.a() { pc = 0xaed9c; continue; }
            pc = 0xaed37;
        }
        0xaed37 => {
            // block aed37
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aed37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aed37 mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // aed3e mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // aed42 lea rax,[rbp-0x158]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // aed49 add rsi,rax
            { let (r, f) = add::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // aed4c lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // aed50 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aed55 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // aed59 mov rsi,QWORD PTR [rip+0x91430] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // aed60 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aed65 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // aed68 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // aed6c mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // aed71 mov rdi,rax
            rdi = rax;
            // aed74 call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aed76 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // aed78 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // aed7c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aed81 movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // aed84 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // aed8b call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aed90 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // aed97 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaed9c;
        }
        0xaed9c => {
            // block aed9c
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aed9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aed9c inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // aed9f add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // aeda3 cmp r15,0x4
            fl = sub::<64>(r15, 0x4u64).1;
            // aeda7 jb 0xaec33
            if fl.b() { pc = 0xaec33; continue; }
            pc = 0xaedad;
        }
        0xaedad => {
            // block aedad
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aedad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aedad mov r14,QWORD PTR [rbp-0x1b8]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            // aedb4 mov rdi,r14
            rdi = r14;
            // aedb7 lea rbx,[rbp-0x150]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // aedbe mov rsi,rbx
            rsi = rbx;
            // aedc1 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aedc6 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // aedcd mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // aedd4 mov rax,QWORD PTR [rbp-0x1a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // aeddb mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // aede2 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // aede9 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // aedf0 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // aedf7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aedfc mov rdi,rbx
            rdi = rbx;
            // aedff call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aee04 mov rsi,QWORD PTR [rip+0x913fd] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // aee0b add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // aee0f lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // aee16 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aee1b lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // aee22 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aee27 mov rax,r14
            rax = r14;
            // aee2a add rsp,0x198
            { let (r, f) = add::<64>(rsp, 0x198u64);
              rsp = r;
              fl = f; }
            // aee31 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // aee32 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // aee34 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // aee36 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // aee38 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // aee3a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // aee3b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xaee3c => {
            // block aee3c
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aee3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aee3c mov rbx,rax
            rbx = rax;
            // aee3f jmp 0xaee6f
            pc = 0xaee6f; continue;
        }
        0xaee41 => {
            // block aee41
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aee41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aee41 mov rbx,rax
            rbx = rax;
            // aee44 jmp 0xaee63
            pc = 0xaee63; continue;
        }
        0xaee46 => {
            // block aee46
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aee46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aee46 mov rbx,rax
            rbx = rax;
            // aee49 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // aee4d call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aee52 jmp 0xaee6f
            pc = 0xaee6f; continue;
        }
        0xaee54 => {
            // block aee54
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aee54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aee54 mov rbx,rax
            rbx = rax;
            // aee57 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // aee5e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaee63;
        }
        0xaee63 => {
            // block aee63
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aee63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aee63 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // aee6a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaee6f;
        }
        0xaee6f => {
            // block aee6f
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aee6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aee6f mov rax,QWORD PTR [rbp-0x1a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // aee76 mov rcx,QWORD PTR [rbp-0x1a8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // aee7d mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // aee84 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // aee8b mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // aee92 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            pc = 0xaee99;
        }
        0xaee99 => {
            // block aee99
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aee99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aee99 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // aeea0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aeea5 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // aeeac call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaeeb1;
        }
        0xaeeb1 => {
            // block aeeb1
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aeeb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aeeb1 mov rsi,QWORD PTR [rip+0x91350] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // aeeb8 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // aeebc lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // aeec3 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaeec8;
        }
        0xaeec8 => {
            // block aeec8
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aeec8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aeec8 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // aeecf call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aeed4 mov rdi,rbx
            rdi = rbx;
            // aeed7 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aeedc mov rbx,rax
            rbx = rax;
            // aeedf jmp 0xaeec8
            pc = 0xaeec8; continue;
        }
        0xaeee1 => {
            // block aeee1
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aeee1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aeee1 mov rbx,rax
            rbx = rax;
            // aeee4 jmp 0xaeeb1
            pc = 0xaeeb1; continue;
        }
        0xaeee6 => {
            // block aeee6
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aeee6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aeee6 mov rbx,rax
            rbx = rax;
            // aeee9 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // aeeed call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aeef2 jmp 0xaee99
            pc = 0xaee99; continue;
        }
        0xaeef4 => {
            // block aeef4
            #[cfg(feature = "trace")] eprintln!("f_000aeb0c pc aeef4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aeef4 jmp 0xaee3c
            pc = 0xaee3c; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdd17c size=996
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000dd17c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xdd17c;
    loop {
        match pc {
        0xdd17c => {
            // block dd17c
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd17c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd17c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dd17d mov rbp,rsp
            rbp = rsp;
            // dd180 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dd182 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dd184 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dd186 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dd188 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dd189 sub rsp,0x4078
            { let (r, f) = sub::<64>(rsp, 0x4078u64);
              rsp = r;
              fl = f; }
            // dd190 mov r12,rdi
            r12 = rdi;
            // dd193 mov rax,QWORD PTR [rip+0x6323e] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // dd19a mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // dd19d mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // dd1a1 mov QWORD PTR [rbp-0x4038],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_bfc8), (0x0u64) as u64);
            // dd1ac mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd1b0 lea rsi,[rbp-0x403c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_bfc4);
            // dd1b7 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dd1b9 call QWORD PTR [rax+0x3f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd1bf mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd1c3 lea rsi,[rbp-0x4040]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_bfc0);
            // dd1ca xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dd1cc mov rdi,r12
            rdi = r12;
            // dd1cf call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd1d5 cmp DWORD PTR [rbp-0x4040],0xa
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bfc0)) as u64, 0xau64).1;
            // dd1dc sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // dd1df mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd1e3 lea rsi,[rbp-0x4041]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_bfbf);
            // dd1ea lea rdx,[rbp-0x4042]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_bfbe);
            // dd1f1 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dd1f3 mov rdi,r12
            rdi = r12;
            // dd1f6 call QWORD PTR [rax+0x5e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x5e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd1fc mov esi,DWORD PTR [rbp-0x403c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bfc4)) as u64) as u32 as u64;
            // dd202 mov edx,DWORD PTR [r12+0x274]
            rdx = (m.r32(r12.wrapping_add(0x274)) as u64) as u32 as u64;
            // dd20a movzx r9d,BYTE PTR [rbp-0x4042]
            r9 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_bfbe)) as u64) as u32 as u64;
            // dd212 movzx ecx,BYTE PTR [rbp-0x4041]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_bfbf)) as u64) as u32 as u64;
            // dd219 movzx r8d,bl
            r8 = ((rbx as u8 as u64)) as u32 as u64;
            // dd21d lea rdi,[rbp-0x4080]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_bf80);
            // dd224 call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd229 mov eax,DWORD PTR [rbp-0x407c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf84)) as u64) as u32 as u64;
            // dd22f mov DWORD PTR [rbp-0x4084],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_bf7c), ((rax as u32 as u64)) as u32);
            // dd235 mov eax,DWORD PTR [rbp-0x4078]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf88)) as u64) as u32 as u64;
            // dd23b mov DWORD PTR [rbp-0x4088],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_bf78), ((rax as u32 as u64)) as u32);
            // dd241 mov r15d,DWORD PTR [rbp-0x4080]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf80)) as u64) as u32 as u64;
            // dd248 cmp DWORD PTR [rbp-0x4040],0xa
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bfc0)) as u64, 0xau64).1;
            // dd24f jne 0xdd26c
            if fl.ne() { pc = 0xdd26c; continue; }
            pc = 0xdd251;
        }
        0xdd251 => {
            // block dd251
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd251 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd251 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd255 lea rsi,[rbp-0x4084]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_bf7c);
            // dd25c lea rdx,[rbp-0x4088]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_bf78);
            // dd263 mov rdi,r12
            rdi = r12;
            // dd266 call QWORD PTR [rax+0x1a00]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a00)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdd26c;
        }
        0xdd26c => {
            // block dd26c
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd26c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd26c mov rbx,QWORD PTR [r12+0x280]
            rbx = m.r64(r12.wrapping_add(0x280)) as u64;
            // dd274 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // dd276 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // dd279 jne 0xdd2c3
            if fl.ne() { pc = 0xdd2c3; continue; }
            pc = 0xdd27b;
        }
        0xdd27b => {
            // block dd27b
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd27b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd27b cmp BYTE PTR [r12+0x279],0x0
            fl = sub::<8>(m.r8(r12.wrapping_add(0x279)) as u64, 0x0u64).1;
            // dd284 je 0xdd29e
            if fl.e() { pc = 0xdd29e; continue; }
            pc = 0xdd286;
        }
        0xdd286 => {
            // block dd286
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd286 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd286 mov edi,DWORD PTR [rbp-0x4088]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf78)) as u64) as u32 as u64;
            // dd28c imul edi,r15d
            { let (r, f) = imul::<32>((rdi as u32 as u64), (r15 as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            // dd290 shl rdi,0x2
            { let (r, f) = shl::<64>(rdi, 0x2u64, fl); rdi = r; fl = f; }
            // dd294 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd299 mov rbx,rax
            rbx = rax;
            // dd29c jmp 0xdd2c3
            pc = 0xdd2c3; continue;
        }
        0xdd29e => {
            // block dd29e
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd29e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd29e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd2a2 mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd2aa lea rdx,[rbp-0x4038]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_bfc8);
            // dd2b1 mov rdi,r12
            rdi = r12;
            // dd2b4 call QWORD PTR [rax+0xa10]
            { let r = { let t = m.r64(rax.wrapping_add(0xa10)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd2ba mov rbx,QWORD PTR [rbp-0x4038]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_bfc8)) as u64;
            // dd2c1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xdd2c3;
        }
        0xdd2c3 => {
            // block dd2c3
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd2c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd2c3 mov QWORD PTR [rbp-0x4098],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_bf68), (rax) as u64);
            // dd2ca mov eax,DWORD PTR [r12+0x274]
            rax = (m.r32(r12.wrapping_add(0x274)) as u64) as u32 as u64;
            // dd2d2 cmp eax,0x14
            fl = sub::<32>((rax as u32 as u64), 0x14u64).1;
            // dd2d5 ja 0xdd360
            if fl.a() { pc = 0xdd360; continue; }
            pc = 0xdd2db;
        }
        0xdd2db => {
            // block dd2db
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd2db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd2db mov ecx,0x11b0dc
            rcx = (0x11b0dcu64) as u32 as u64;
            // dd2e0 bt ecx,eax
            fl.cf = bit::<32>((rcx as u32 as u64), (rax as u32 as u64));
            // dd2e3 jae 0xdd2ff
            if fl.ae() { pc = 0xdd2ff; continue; }
            pc = 0xdd2e5;
        }
        0xdd2e5 => {
            // block dd2e5
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd2e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd2e5 mov esi,DWORD PTR [rbp-0x4088]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf78)) as u64) as u32 as u64;
            // dd2eb imul esi,r15d
            { let (r, f) = imul::<32>((rsi as u32 as u64), (r15 as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            // dd2ef shl esi,0x2
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x2u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // dd2f2 mov rdi,rbx
            rdi = rbx;
            // dd2f5 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd2fa jmp 0xdd427
            pc = 0xdd427; continue;
        }
        0xdd2ff => {
            // block dd2ff
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd2ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd2ff mov ecx,0x4b23
            rcx = (0x4b23u64) as u32 as u64;
            // dd304 bt ecx,eax
            fl.cf = bit::<32>((rcx as u32 as u64), (rax as u32 as u64));
            // dd307 jae 0xdd360
            if fl.ae() { pc = 0xdd360; continue; }
            pc = 0xdd309;
        }
        0xdd309 => {
            // block dd309
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd309 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd309 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xdd30b;
        }
        0xdd30b => {
            // block dd30b
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd30b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd30b mov WORD PTR [rbp+rcx*2-0x4030],0x200
            m.w16(rbp.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_bfd0), (0x200u64) as u16);
            // dd315 mov WORD PTR [rbp+rcx*2-0x402e],0x40
            m.w16(rbp.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_bfd2), (0x40u64) as u16);
            // dd31f add rcx,0x2
            { let (r, f) = add::<64>(rcx, 0x2u64);
              rcx = r;
              fl = f; }
            // dd323 cmp rcx,0x1000
            fl = sub::<64>(rcx, 0x1000u64).1;
            // dd32a jl 0xdd30b
            if fl.l() { pc = 0xdd30b; continue; }
            pc = 0xdd32c;
        }
        0xdd32c => {
            // block dd32c
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd32c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd32c cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // dd32f jne 0xdd379
            if fl.ne() { pc = 0xdd379; continue; }
            pc = 0xdd331;
        }
        0xdd331 => {
            // block dd331
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd331 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd331 mov edx,DWORD PTR [rbp-0x4084]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf7c)) as u64) as u32 as u64;
            // dd337 lea rdi,[rbp-0x4030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_bfd0);
            // dd33e lea r13,[rbp-0x2030]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_dfd0);
            // dd345 mov rsi,r13
            rsi = r13;
            // dd348 call 0x59033
            { let r = { let t = m.env.code_va(0x59033); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd34d mov esi,DWORD PTR [rbp-0x4088]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf78)) as u64) as u32 as u64;
            // dd353 mov rdi,r13
            rdi = r13;
            // dd356 call 0x58c54
            { let r = { let t = m.env.code_va(0x58c54); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd35b jmp 0xdd3f5
            pc = 0xdd3f5; continue;
        }
        0xdd360 => {
            // block dd360
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd360 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd360 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd364 lea rsi,[rip+0x5c005] # 0x139370
            rsi = m.env.data_va(318269);
            // dd36b mov rdi,r12
            rdi = r12;
            // dd36e call QWORD PTR [rax+0x170]
            { let r = { let t = m.r64(rax.wrapping_add(0x170)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd374 jmp 0xdd427
            pc = 0xdd427; continue;
        }
        0xdd379 => {
            // block dd379
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd379 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd379 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // dd37b jne 0xdd398
            if fl.ne() { pc = 0xdd398; continue; }
            pc = 0xdd37d;
        }
        0xdd37d => {
            // block dd37d
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd37d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd37d mov edx,DWORD PTR [rbp-0x4084]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf7c)) as u64) as u32 as u64;
            // dd383 lea rdi,[rbp-0x4030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_bfd0);
            // dd38a lea rsi,[rbp-0x2030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_dfd0);
            // dd391 call 0x59033
            { let r = { let t = m.env.code_va(0x59033); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd396 jmp 0xdd3f5
            pc = 0xdd3f5; continue;
        }
        0xdd398 => {
            // block dd398
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd398 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd398 cmp eax,0xe
            fl = sub::<32>((rax as u32 as u64), 0xeu64).1;
            // dd39b jne 0xdd3b8
            if fl.ne() { pc = 0xdd3b8; continue; }
            pc = 0xdd39d;
        }
        0xdd39d => {
            // block dd39d
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd39d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd39d mov edx,DWORD PTR [rbp-0x4084]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf7c)) as u64) as u32 as u64;
            // dd3a3 lea rdi,[rbp-0x4030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_bfd0);
            // dd3aa lea rsi,[rbp-0x2030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_dfd0);
            // dd3b1 call 0x6744c
            { let r = { let t = m.env.code_va(0x6744c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd3b6 jmp 0xdd3f5
            pc = 0xdd3f5; continue;
        }
        0xdd3b8 => {
            // block dd3b8
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd3b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd3b8 mov edx,DWORD PTR [rbp-0x4084]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf7c)) as u64) as u32 as u64;
            // dd3be lea rdi,[rbp-0x4030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_bfd0);
            // dd3c5 lea rsi,[rbp-0x2030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_dfd0);
            // dd3cc call 0x674ee
            { let r = { let t = m.env.code_va(0x674ee); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd3d1 mov eax,DWORD PTR [r12+0x274]
            rax = (m.r32(r12.wrapping_add(0x274)) as u64) as u32 as u64;
            // dd3d9 cmp eax,0x5
            fl = sub::<32>((rax as u32 as u64), 0x5u64).1;
            // dd3dc je 0xdd3e3
            if fl.e() { pc = 0xdd3e3; continue; }
            pc = 0xdd3de;
        }
        0xdd3de => {
            // block dd3de
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd3de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd3de cmp eax,0xa
            fl = sub::<32>((rax as u32 as u64), 0xau64).1;
            // dd3e1 jne 0xdd3f5
            if fl.ne() { pc = 0xdd3f5; continue; }
            pc = 0xdd3e3;
        }
        0xdd3e3 => {
            // block dd3e3
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd3e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd3e3 mov esi,DWORD PTR [rbp-0x4084]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf7c)) as u64) as u32 as u64;
            // dd3e9 lea rdi,[rbp-0x2030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dfd0);
            // dd3f0 call 0x67511
            { let r = { let t = m.env.code_va(0x67511); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdd3f5;
        }
        0xdd3f5 => {
            // block dd3f5
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd3f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd3f5 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // dd3f8 je 0xdd427
            if fl.e() { pc = 0xdd427; continue; }
            pc = 0xdd3fa;
        }
        0xdd3fa => {
            // block dd3fa
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd3fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd3fa mov edx,DWORD PTR [rbp-0x4088]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf78)) as u64) as u32 as u64;
            // dd400 lea r13,[rbp-0x2030]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_dfd0);
            // dd407 mov r14d,r15d
            r14 = ((r15 as u32 as u64)) as u32 as u64;
            pc = 0xdd40a;
        }
        0xdd40a => {
            // block dd40a
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd40a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd40a shl edx,0x2
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // dd40d mov rdi,rbx
            rdi = rbx;
            // dd410 mov rsi,r13
            rsi = r13;
            // dd413 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd418 mov edx,DWORD PTR [rbp-0x4088]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf78)) as u64) as u32 as u64;
            // dd41e lea rbx,[rbx+rdx*4]
            rbx = rbx.wrapping_add(rdx.wrapping_mul(4));
            // dd422 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // dd425 jne 0xdd40a
            if fl.ne() { pc = 0xdd40a; continue; }
            pc = 0xdd427;
        }
        0xdd427 => {
            // block dd427
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd427 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd427 mov rbx,QWORD PTR [rbp-0x4098]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_bf68)) as u64;
            // dd42e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // dd431 mov r14,QWORD PTR [rip+0x62fa0] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // dd438 mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // dd43b je 0xdd4d4
            if fl.e() { pc = 0xdd4d4; continue; }
            pc = 0xdd441;
        }
        0xdd441 => {
            // block dd441
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd441 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd441 mov al,BYTE PTR [r12+0x279]
            rax = (rax & !0xff) | ((m.r8(r12.wrapping_add(0x279)) as u64) as u8 as u64);
            // dd449 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // dd44b je 0xdd4d4
            if fl.e() { pc = 0xdd4d4; continue; }
            pc = 0xdd451;
        }
        0xdd451 => {
            // block dd451
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd451 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd451 cmp BYTE PTR [r12+0x27b],0x0
            fl = sub::<8>(m.r8(r12.wrapping_add(0x27b)) as u64, 0x0u64).1;
            // dd45a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd45e mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd466 lea rdx,[rbp-0x408c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_bf74);
            // dd46d mov rdi,r12
            rdi = r12;
            // dd470 je 0xdd47a
            if fl.e() { pc = 0xdd47a; continue; }
            pc = 0xdd472;
        }
        0xdd472 => {
            // block dd472
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd472 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd472 call QWORD PTR [rax+0x538]
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd478 jmp 0xdd480
            pc = 0xdd480; continue;
        }
        0xdd47a => {
            // block dd47a
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd47a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd47a call QWORD PTR [rax+0x550]
            { let r = { let t = m.r64(rax.wrapping_add(0x550)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdd480;
        }
        0xdd480 => {
            // block dd480
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd480 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd480 mov r13d,DWORD PTR [rbp-0x408c]
            r13 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf74)) as u64) as u32 as u64;
            // dd487 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd48b mov rdi,r12
            rdi = r12;
            // dd48e call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd494 cmp r13d,eax
            fl = sub::<32>((r13 as u32 as u64), (rax as u32 as u64)).1;
            // dd497 jbe 0xdd4a7
            if fl.be() { pc = 0xdd4a7; continue; }
            pc = 0xdd499;
        }
        0xdd499 => {
            // block dd499
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd499 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd499 mov DWORD PTR [rbp-0x408c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_bf74), (0x0u64) as u32);
            // dd4a3 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dd4a5 jmp 0xdd4ad
            pc = 0xdd4ad; continue;
        }
        0xdd4a7 => {
            // block dd4a7
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd4a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd4a7 mov esi,DWORD PTR [rbp-0x408c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf74)) as u64) as u32 as u64;
            pc = 0xdd4ad;
        }
        0xdd4ad => {
            // block dd4ad
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd4ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd4ad mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd4b1 imul r15d,DWORD PTR [rbp-0x4088]
            { let (r, f) = imul::<32>((r15 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_bf78)) as u64); r15 = (r) as u32 as u64; fl = f; }
            // dd4b9 shl r15d,0x2
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x2u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // dd4bd mov rdi,r12
            rdi = r12;
            // dd4c0 mov rdx,rbx
            rdx = rbx;
            // dd4c3 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // dd4c6 call QWORD PTR [rax+0x2c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x2c0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd4cc mov rdi,rbx
            rdi = rbx;
            // dd4cf call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdd4d4;
        }
        0xdd4d4 => {
            // block dd4d4
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd4d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd4d4 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd4d8 movzx esi,BYTE PTR [r12+0x278]
            rsi = (m.r8(r12.wrapping_add(0x278)) as u64) as u32 as u64;
            // dd4e1 mov rdi,r12
            rdi = r12;
            // dd4e4 call QWORD PTR [rax+0x570]
            { let r = { let t = m.r64(rax.wrapping_add(0x570)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd4ea mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd4ee mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd4f6 mov edx,DWORD PTR [r12+0x274]
            rdx = (m.r32(r12.wrapping_add(0x274)) as u64) as u32 as u64;
            // dd4fe mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // dd503 mov rdi,r12
            rdi = r12;
            // dd506 call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd50c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd510 mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd518 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dd51a mov rdi,r12
            rdi = r12;
            // dd51d call QWORD PTR [rax+0x4f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4f8)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd523 cmp BYTE PTR [r12+0x27b],0x0
            fl = sub::<8>(m.r8(r12.wrapping_add(0x27b)) as u64, 0x0u64).1;
            // dd52c je 0xdd543
            if fl.e() { pc = 0xdd543; continue; }
            pc = 0xdd52e;
        }
        0xdd52e => {
            // block dd52e
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd52e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd52e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd532 mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd53a mov rdi,r12
            rdi = r12;
            // dd53d call QWORD PTR [rax+0x618]
            { let r = { let t = m.r64(rax.wrapping_add(0x618)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdd543;
        }
        0xdd543 => {
            // block dd543
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd543 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd543 cmp r14,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r14, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // dd547 jne 0xdd55b
            if fl.ne() { pc = 0xdd55b; continue; }
            pc = 0xdd549;
        }
        0xdd549 => {
            // block dd549
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd549 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd549 add rsp,0x4078
            { let (r, f) = add::<64>(rsp, 0x4078u64);
              rsp = r;
              fl = f; }
            // dd550 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dd551 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dd553 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dd555 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dd557 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dd559 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dd55a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xdd55b => {
            // block dd55b
            #[cfg(feature = "trace")] eprintln!("f_000dd17c pc dd55b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd55b call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xab2cc size=966
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ab2cc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xab2cc;
    loop {
        match pc {
        0xab2cc => {
            // block ab2cc
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab2cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab2cc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ab2cd mov rbp,rsp
            rbp = rsp;
            // ab2d0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ab2d2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ab2d4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ab2d6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ab2d8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ab2d9 sub rsp,0x168
            { let (r, f) = sub::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // ab2e0 mov DWORD PTR [rbp-0x164],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe9c), ((rcx as u32 as u64)) as u32);
            // ab2e6 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // ab2e9 mov r12,rdi
            r12 = rdi;
            // ab2ec lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // ab2f3 lea r15,[rbp-0x150]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // ab2fa mov rax,QWORD PTR [rip+0x94ecf] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // ab301 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // ab305 mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // ab30c add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // ab310 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // ab317 mov rsi,r15
            rsi = r15;
            // ab31a call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab31f mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // ab327 mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // ab32e mov rbx,QWORD PTR [rip+0x9508b] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // ab335 lea r13,[rbx+0x18]
            r13 = rbx.wrapping_add(0x18);
            // ab339 mov QWORD PTR [rbp-0x158],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (r13) as u64);
            // ab340 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // ab344 mov QWORD PTR [rbp-0xe8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rbx) as u64);
            // ab34b mov rdi,r15
            rdi = r15;
            // ab34e call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab353 mov QWORD PTR [rbp-0x178],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (r13) as u64);
            // ab35a mov QWORD PTR [rbp-0x170],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rbx) as u64);
            // ab361 mov QWORD PTR [rbp-0x188],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (r12) as u64);
            // ab368 mov rax,QWORD PTR [rip+0x95039] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // ab36f add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // ab373 mov QWORD PTR [rbp-0x180],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rax) as u64);
            // ab37a mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // ab381 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // ab38c mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // ab397 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // ab3a2 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // ab3ad mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // ab3b7 xorps xmm0,xmm0
            x0 = 0;
            // ab3ba movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // ab3be mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // ab3c6 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // ab3ca mov rdi,r15
            rdi = r15;
            // ab3cd call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab3d2 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // ab3d6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab3db cmp r14d,0x1f0
            fl = sub::<32>((r14 as u32 as u64), 0x1f0u64).1;
            // ab3e2 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // ab3e5 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // ab3e8 lea eax,[rax*4+0x1]
            rax = (rax.wrapping_mul(4).wrapping_add(0x1)) as u32 as u64;
            // ab3ef mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // ab3f6 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // ab3f9 lea r14,[rbp-0x158]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            pc = 0xab400;
        }
        0xab400 => {
            // block ab400
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab400 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab400 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // ab405 mov rdi,r14
            rdi = r14;
            // ab408 lea rsi,[rip+0x89581] # 0x134990
            rsi = m.env.data_va(299357);
            // ab40f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab414 mov rcx,QWORD PTR [rbp-0x160]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // ab41b lea esi,[r12+rcx*1]
            rsi = (r12.wrapping_add(rcx)) as u32 as u64;
            // ab41f mov rdi,rax
            rdi = rax;
            // ab422 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab427 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // ab42c mov rdi,rax
            rdi = rax;
            // ab42f lea rsi,[rip+0x88b7c] # 0x133fb2
            rsi = m.env.data_va(296831);
            // ab436 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab43b lea ecx,[r12*8+0x0]
            rcx = (r12.wrapping_mul(8)) as u32 as u64;
            // ab443 mov ebx,DWORD PTR [rbp-0x164]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe9c)) as u64) as u32 as u64;
            // ab449 shr ebx,cl
            { let (r, f) = shr::<32>((rbx as u32 as u64), (rcx as u8 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // ab44b and ebx,0xff
            { let r = (rbx as u32 as u64) & 0xffu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // ab451 je 0xab4c4
            if fl.e() { pc = 0xab4c4; continue; }
            pc = 0xab453;
        }
        0xab453 => {
            // block ab453
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab453 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab453 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // ab458 mov rdi,r14
            rdi = r14;
            // ab45b lea rsi,[rip+0x89543] # 0x1349a5
            rsi = m.env.data_va(299378);
            // ab462 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab467 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // ab46a mov r13d,0x1
            r13 = (0x1u64) as u32 as u64;
            pc = 0xab470;
        }
        0xab470 => {
            // block ab470
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab470 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab470 bt ebx,r15d
            fl.cf = bit::<32>((rbx as u32 as u64), (r15 as u32 as u64));
            // ab474 jae 0xab4b5
            if fl.ae() { pc = 0xab4b5; continue; }
            pc = 0xab476;
        }
        0xab476 => {
            // block ab476
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab476 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab476 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // ab47b mov rdi,r14
            rdi = r14;
            // ab47e lea rsi,[rip+0x89531] # 0x1349b6
            rsi = m.env.data_va(299395);
            // ab485 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab48a mov rdi,rax
            rdi = rax;
            // ab48d mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // ab490 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab495 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // ab49a mov rdi,rax
            rdi = rax;
            // ab49d lea rsi,[rip+0x7f4e0] # 0x12a984
            rsi = m.env.data_va(258385);
            // ab4a4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab4a9 lea esi,[r13+0x1]
            rsi = (r13.wrapping_add(0x1)) as u32 as u64;
            // ab4ad mov rdi,rax
            rdi = rax;
            // ab4b0 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab4b5;
        }
        0xab4b5 => {
            // block ab4b5
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab4b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab4b5 inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // ab4b8 add r13d,0x2
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x2u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // ab4bc cmp r15d,0x8
            fl = sub::<32>((r15 as u32 as u64), 0x8u64).1;
            // ab4c0 jb 0xab470
            if fl.b() { pc = 0xab470; continue; }
            pc = 0xab4c2;
        }
        0xab4c2 => {
            // block ab4c2
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab4c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab4c2 jmp 0xab4d8
            pc = 0xab4d8; continue;
        }
        0xab4c4 => {
            // block ab4c4
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab4c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab4c4 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // ab4c9 mov rdi,r14
            rdi = r14;
            // ab4cc lea rsi,[rip+0x894cb] # 0x13499e
            rsi = m.env.data_va(299371);
            // ab4d3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab4d8;
        }
        0xab4d8 => {
            // block ab4d8
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab4d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab4d8 cmp r12d,0x2
            fl = sub::<32>((r12 as u32 as u64), 0x2u64).1;
            // ab4dc ja 0xab534
            if fl.a() { pc = 0xab534; continue; }
            pc = 0xab4de;
        }
        0xab4de => {
            // block ab4de
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab4de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab4de mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // ab4e5 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // ab4e9 add rsi,r14
            { let (r, f) = add::<64>(rsi, r14);
              rsi = r;
              fl = f; }
            // ab4ec lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // ab4f0 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab4f5 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // ab4f9 mov rsi,QWORD PTR [rip+0x94c90] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // ab500 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab505 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // ab508 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // ab50c mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // ab511 mov rdi,rax
            rdi = rax;
            // ab514 call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab516 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // ab518 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // ab51c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab521 movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // ab524 mov rdi,r14
            rdi = r14;
            // ab527 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab52c mov rdi,r14
            rdi = r14;
            // ab52f call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab534;
        }
        0xab534 => {
            // block ab534
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab534 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab534 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // ab537 cmp r12d,0x4
            fl = sub::<32>((r12 as u32 as u64), 0x4u64).1;
            // ab53b jb 0xab400
            if fl.b() { pc = 0xab400; continue; }
            pc = 0xab541;
        }
        0xab541 => {
            // block ab541
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab541 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab541 mov r14,QWORD PTR [rbp-0x188]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // ab548 mov rdi,r14
            rdi = r14;
            // ab54b lea rbx,[rbp-0x150]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // ab552 mov rsi,rbx
            rsi = rbx;
            // ab555 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab55a mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // ab561 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // ab568 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // ab56f mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // ab576 mov rax,QWORD PTR [rbp-0x180]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // ab57d mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // ab584 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // ab58b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab590 mov rdi,rbx
            rdi = rbx;
            // ab593 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab598 mov rsi,QWORD PTR [rip+0x94c69] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // ab59f add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // ab5a3 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // ab5aa call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab5af lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // ab5b6 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab5bb mov rax,r14
            rax = r14;
            // ab5be add rsp,0x168
            { let (r, f) = add::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // ab5c5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ab5c6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ab5c8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ab5ca pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ab5cc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ab5ce pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ab5cf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xab5d0 => {
            // block ab5d0
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab5d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab5d0 mov r14,rax
            r14 = rax;
            // ab5d3 lea rbx,[rbp-0x150]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            pc = 0xab5da;
        }
        0xab5da => {
            // block ab5da
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab5da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab5da mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // ab5e1 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // ab5e8 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // ab5ef mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // ab5f6 mov rax,QWORD PTR [rbp-0x180]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // ab5fd mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // ab604 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // ab60b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab610 mov rdi,rbx
            rdi = rbx;
            // ab613 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab618;
        }
        0xab618 => {
            // block ab618
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab618 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab618 mov rsi,QWORD PTR [rip+0x94be9] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // ab61f add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // ab623 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // ab62a call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab62f;
        }
        0xab62f => {
            // block ab62f
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab62f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab62f lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // ab636 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab63b mov rdi,r14
            rdi = r14;
            // ab63e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab643 jmp 0xab5d0
            pc = 0xab5d0; continue;
        }
        0xab645 => {
            // block ab645
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab645 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab645 mov r14,rax
            r14 = rax;
            // ab648 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // ab64c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab651 lea rbx,[rbp-0x150]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // ab658 jmp 0xab5da
            pc = 0xab5da; continue;
        }
        0xab65a => {
            // block ab65a
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab65a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab65a mov r14,rax
            r14 = rax;
            // ab65d jmp 0xab62f
            pc = 0xab62f; continue;
        }
        0xab65f => {
            // block ab65f
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab65f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab65f mov r14,rax
            r14 = rax;
            // ab662 jmp 0xab618
            pc = 0xab618; continue;
        }
        0xab664 => {
            // block ab664
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab664 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab664 mov r14,rax
            r14 = rax;
            // ab667 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // ab66b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab670 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // ab677 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab67c lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // ab683 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab688 jmp 0xab618
            pc = 0xab618; continue;
        }
        0xab68a => {
            // block ab68a
            #[cfg(feature = "trace")] eprintln!("f_000ab2cc pc ab68a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab68a mov r14,rax
            r14 = rax;
            // ab68d jmp 0xab5da
            pc = 0xab5da; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa4ede size=958
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a4ede<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xa4ede;
    loop {
        match pc {
        0xa4ede => {
            // block a4ede
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a4ede rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a4ede push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a4edf mov rbp,rsp
            rbp = rsp;
            // a4ee2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // a4ee4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // a4ee6 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // a4ee8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // a4eea push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // a4eeb sub rsp,0x168
            { let (r, f) = sub::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // a4ef2 mov r14d,r8d
            r14 = ((r8 as u32 as u64)) as u32 as u64;
            // a4ef5 mov r13d,ecx
            r13 = ((rcx as u32 as u64)) as u32 as u64;
            // a4ef8 mov QWORD PTR [rbp-0x170],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rdi) as u64);
            // a4eff lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a4f06 lea r15,[rbp-0x150]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a4f0d mov rax,QWORD PTR [rip+0x9b2bc] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // a4f14 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // a4f18 mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // a4f1f add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // a4f23 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a4f2a mov rsi,r15
            rsi = r15;
            // a4f2d call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4f32 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // a4f3a mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // a4f41 mov rbx,QWORD PTR [rip+0x9b478] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // a4f48 lea r12,[rbx+0x18]
            r12 = rbx.wrapping_add(0x18);
            // a4f4c mov QWORD PTR [rbp-0x158],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (r12) as u64);
            // a4f53 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // a4f57 mov QWORD PTR [rbp-0xe8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rbx) as u64);
            // a4f5e mov rdi,r15
            rdi = r15;
            // a4f61 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4f66 mov QWORD PTR [rbp-0x180],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (r12) as u64);
            // a4f6d mov QWORD PTR [rbp-0x178],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rbx) as u64);
            // a4f74 mov rax,QWORD PTR [rip+0x9b42d] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // a4f7b add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // a4f7f mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            // a4f86 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // a4f8d mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // a4f98 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // a4fa3 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // a4fae mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // a4fb9 mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // a4fc3 xorps xmm0,xmm0
            x0 = 0;
            // a4fc6 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // a4fca mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // a4fd2 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a4fd6 mov rdi,r15
            rdi = r15;
            // a4fd9 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4fde lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a4fe2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4fe7 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // a4fea call 0xcd5c3
            { let r = { let t = m.env.code_va(0xcd5c3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4fef test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // a4ff1 je 0xa5142
            if fl.e() { pc = 0xa5142; continue; }
            pc = 0xa4ff7;
        }
        0xa4ff7 => {
            // block a4ff7
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a4ff7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a4ff7 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // a4ffa call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4fff mov r15w,ax
            r15 = (r15 & !0xffff) | (((rax as u16 as u64)) as u16 as u64);
            // a5003 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // a5006 call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a500b movzx ecx,ax
            rcx = ((rax as u16 as u64)) as u32 as u64;
            // a500e movzx eax,r15w
            rax = ((r15 as u16 as u64)) as u32 as u64;
            // a5012 cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // a5014 cmovbe ax,cx
            if fl.be() { rax = (rax & !0xffff) | (((rcx as u16 as u64)) as u16 as u64); }
            // a5018 test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // a501b je 0xa515c
            if fl.e() { pc = 0xa515c; continue; }
            pc = 0xa5021;
        }
        0xa5021 => {
            // block a5021
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a5021 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5021 mov ecx,r13d
            rcx = ((r13 as u32 as u64)) as u32 as u64;
            // a5024 shr ecx,0x14
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x14u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // a5027 and ecx,0xf0
            { let r = (rcx as u32 as u64) & 0xf0u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // a502d shr r13d,0x1c
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x1cu64, fl); r13 = (r) as u32 as u64; fl = f; }
            // a5031 or r13d,ecx
            { let r = (r13 as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // a5034 mov DWORD PTR [rbp-0x160],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fea0), ((r13 as u32 as u64)) as u32);
            // a503b movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // a503e mov DWORD PTR [rbp-0x164],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe9c), ((rax as u32 as u64)) as u32);
            // a5044 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // a5047 lea r15,[rbp-0x158]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a504e lea r14,[rbp-0x30]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0xa5052;
        }
        0xa5052 => {
            // block a5052
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a5052 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5052 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // a5057 mov cl,r12b
            rcx = (rcx & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // a505a shl ebx,cl
            { let (r, f) = shl::<32>((rbx as u32 as u64), (rcx as u8 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // a505c mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // a5061 mov rdi,r15
            rdi = r15;
            // a5064 lea rsi,[rip+0x8ef42] # 0x133fad
            rsi = m.env.data_va(296826);
            // a506b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5070 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a5073 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5077 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // a507b and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a507e or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a5081 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // a5085 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // a5088 movzx esi,r12w
            rsi = ((r12 as u16 as u64)) as u32 as u64;
            // a508c mov rdi,rax
            rdi = rax;
            // a508f call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5094 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // a5099 mov rdi,rax
            rdi = rax;
            // a509c lea rsi,[rip+0x8ef0f] # 0x133fb2
            rsi = m.env.data_va(296831);
            // a50a3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a50a8 and ebx,DWORD PTR [rbp-0x160]
            { let r = (rbx as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // a50ae setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // a50b1 lea rsi,[rip+0x8ef0d] # 0x133fc5
            rsi = m.env.data_va(296850);
            // a50b8 lea rdx,[rip+0x8eef6] # 0x133fb5
            rdx = m.env.data_va(296834);
            // a50bf cmovne rsi,rdx
            if fl.ne() { rsi = rdx; }
            // a50c3 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // a50c6 lea rdx,[rcx+rcx*1+0xd]
            rdx = rcx.wrapping_add(rcx).wrapping_add(0xd);
            // a50cb mov rdi,rax
            rdi = rax;
            // a50ce call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a50d3 cmp r12d,DWORD PTR [rbp-0x164]
            fl = sub::<32>((r12 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe9c)) as u64).1;
            // a50da jae 0xa5174
            if fl.ae() { pc = 0xa5174; continue; }
            pc = 0xa50e0;
        }
        0xa50e0 => {
            // block a50e0
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a50e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a50e0 mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // a50e7 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a50eb add rsi,r15
            { let (r, f) = add::<64>(rsi, r15);
              rsi = r;
              fl = f; }
            // a50ee mov rdi,r14
            rdi = r14;
            // a50f1 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a50f6 mov rdi,r14
            rdi = r14;
            // a50f9 mov rsi,QWORD PTR [rip+0x9b090] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a5100 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5105 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a5108 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a510c mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a5111 mov rdi,rax
            rdi = rax;
            // a5114 call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5116 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a5119 mov rdi,r14
            rdi = r14;
            // a511c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5121 movsx esi,r13b
            rsi = (sx::<8>((r13 as u8 as u64)) as u64) as u32 as u64;
            // a5125 mov rdi,r15
            rdi = r15;
            // a5128 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a512d mov rdi,r15
            rdi = r15;
            // a5130 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5135 jmp 0xa5052
            pc = 0xa5052; continue;
        }
        0xa513a => {
            // block a513a
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a513a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a513a mov rbx,rax
            rbx = rax;
            // a513d jmp 0xa5211
            pc = 0xa5211; continue;
        }
        0xa5142 => {
            // block a5142
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a5142 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5142 lea rsi,[rip+0x8eea5] # 0x133fee
            rsi = m.env.data_va(296891);
            // a5149 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a5150 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // a5155 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a515a jmp 0xa5174
            pc = 0xa5174; continue;
        }
        0xa515c => {
            // block a515c
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a515c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a515c lea rsi,[rip+0x8ee70] # 0x133fd3
            rsi = m.env.data_va(296864);
            // a5163 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a516a mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // a516f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa5174;
        }
        0xa5174 => {
            // block a5174
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a5174 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5174 mov rdi,QWORD PTR [rbp-0x170]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // a517b lea rsi,[rbp-0x150]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a5182 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5187 mov rax,QWORD PTR [rbp-0x180]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // a518e mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // a5195 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // a519c mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a51a3 mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // a51aa mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // a51b1 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // a51b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a51bd lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a51c4 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a51c9 mov rsi,QWORD PTR [rip+0x9b038] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a51d0 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a51d4 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a51db call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a51e0 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a51e7 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a51ec mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // a51f3 add rsp,0x168
            { let (r, f) = add::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // a51fa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // a51fb pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // a51fd pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // a51ff pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // a5201 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // a5203 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a5204 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xa5205 => {
            // block a5205
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a5205 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5205 mov rbx,rax
            rbx = rax;
            // a5208 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a520c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa5211;
        }
        0xa5211 => {
            // block a5211
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a5211 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5211 mov rax,QWORD PTR [rbp-0x180]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // a5218 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // a521f mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // a5226 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a522d mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // a5234 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            pc = 0xa523b;
        }
        0xa523b => {
            // block a523b
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a523b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a523b lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // a5242 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5247 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a524e call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa5253;
        }
        0xa5253 => {
            // block a5253
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a5253 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5253 mov rsi,QWORD PTR [rip+0x9afae] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a525a add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a525e lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a5265 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa526a;
        }
        0xa526a => {
            // block a526a
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a526a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a526a lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a5271 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5276 mov rdi,rbx
            rdi = rbx;
            // a5279 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a527e jmp 0xa513a
            pc = 0xa513a; continue;
        }
        0xa5283 => {
            // block a5283
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a5283 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5283 mov rbx,rax
            rbx = rax;
            // a5286 jmp 0xa526a
            pc = 0xa526a; continue;
        }
        0xa5288 => {
            // block a5288
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a5288 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5288 mov rbx,rax
            rbx = rax;
            // a528b jmp 0xa5253
            pc = 0xa5253; continue;
        }
        0xa528d => {
            // block a528d
            #[cfg(feature = "trace")] eprintln!("f_000a4ede pc a528d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a528d mov rbx,rax
            rbx = rax;
            // a5290 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a5294 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5299 jmp 0xa523b
            pc = 0xa523b; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7764 size=948
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949
pub fn f_000d7764<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xd7764;
    loop {
        match pc {
        0xd7764 => {
            // block d7764
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7764 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7764 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7765 mov rbp,rsp
            rbp = rsp;
            // d7768 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d776a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d776c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d776e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d7770 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d7771 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d7772 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d7774 mov r15,rdi
            r15 = rdi;
            // d7777 cmp QWORD PTR [r15+0x288],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x288)) as u64, 0x0u64).1;
            // d777f je 0xd7aa6
            if fl.e() { pc = 0xd7aa6; continue; }
            pc = 0xd7785;
        }
        0xd7785 => {
            // block d7785
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7785 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7785 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7788 mov rdi,r15
            rdi = r15;
            // d778b call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7791 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d7793 je 0xd7ab9
            if fl.e() { pc = 0xd7ab9; continue; }
            pc = 0xd7799;
        }
        0xd7799 => {
            // block d7799
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7799 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7799 mov eax,DWORD PTR [r15+0x370]
            rax = (m.r32(r15.wrapping_add(0x370)) as u64) as u32 as u64;
            // d77a0 lea ecx,[rax-0x1]
            rcx = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // d77a3 cmp ecx,0x6
            fl = sub::<32>((rcx as u32 as u64), 0x6u64).1;
            // d77a6 ja 0xd77d1
            if fl.a() { pc = 0xd77d1; continue; }
            pc = 0xd77a8;
        }
        0xd77a8 => {
            // block d77a8
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d77a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d77a8 lea rdx,[rip+0x34d] # 0xd7afc
            rdx = m.env.code_va(0xd7afc);
            // d77af movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // d77b3 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // d77b6 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd77b8 => {
            // block d77b8
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d77b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d77b8 mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d77bb mov r12d,DWORD PTR [r15+0x344]
            r12 = (m.r32(r15.wrapping_add(0x344)) as u64) as u32 as u64;
            // d77c2 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d77c5 mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // d77ca mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // d77cf jmp 0xd7820
            pc = 0xd7820; continue;
        }
        0xd77d1 => {
            // block d77d1
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d77d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d77d1 mov r12d,DWORD PTR [r15+0x340]
            r12 = (m.r32(r15.wrapping_add(0x340)) as u64) as u32 as u64;
            // d77d8 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d77da je 0xd77e1
            if fl.e() { pc = 0xd77e1; continue; }
            pc = 0xd77dc;
        }
        0xd77dc => {
            // block d77dc
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d77dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d77dc mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d77df jmp 0xd784f
            pc = 0xd784f; continue;
        }
        0xd77e1 => {
            // block d77e1
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d77e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d77e1 mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d77e4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d77e7 mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // d77ec xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d77ee jmp 0xd7820
            pc = 0xd7820; continue;
        }
        0xd77f0 => {
            // block d77f0
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d77f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d77f0 mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d77f3 mov r12d,DWORD PTR [r15+0x354]
            r12 = (m.r32(r15.wrapping_add(0x354)) as u64) as u32 as u64;
            // d77fa mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d77fd mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // d7802 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d7807 jmp 0xd7820
            pc = 0xd7820; continue;
        }
        0xd7809 => {
            // block d7809
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7809 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7809 mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d780c mov r12d,DWORD PTR [r15+0x358]
            r12 = (m.r32(r15.wrapping_add(0x358)) as u64) as u32 as u64;
            // d7813 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7816 mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // d781b mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            pc = 0xd7820;
        }
        0xd7820 => {
            // block d7820
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7820 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7820 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7825 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7828 mov rdi,r15
            rdi = r15;
            // d782b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d782e jmp 0xd784f
            pc = 0xd784f; continue;
        }
        0xd7830 => {
            // block d7830
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7830 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7830 lea rax,[r15+0x348]
            rax = r15.wrapping_add(0x348);
            // d7837 jmp 0xd7849
            pc = 0xd7849; continue;
        }
        0xd7839 => {
            // block d7839
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7839 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7839 lea rax,[r15+0x34c]
            rax = r15.wrapping_add(0x34c);
            // d7840 jmp 0xd7849
            pc = 0xd7849; continue;
        }
        0xd7842 => {
            // block d7842
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7842 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7842 lea rax,[r15+0x350]
            rax = r15.wrapping_add(0x350);
            pc = 0xd7849;
        }
        0xd7849 => {
            // block d7849
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7849 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7849 mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d784c mov r12d,DWORD PTR [rax]
            r12 = (m.r32(rax) as u64) as u32 as u64;
            pc = 0xd784f;
        }
        0xd784f => {
            // block d784f
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d784f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d784f mov esi,DWORD PTR [r15+0x370]
            rsi = (m.r32(r15.wrapping_add(0x370)) as u64) as u32 as u64;
            // d7856 mov rdi,r15
            rdi = r15;
            // d7859 call 0xd7b18
            { let r = { let t = m.env.code_va(0xd7b18); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d785e mov esi,DWORD PTR [r15+0x370]
            rsi = (m.r32(r15.wrapping_add(0x370)) as u64) as u32 as u64;
            // d7865 mov rdi,r15
            rdi = r15;
            // d7868 call 0xd74b0
            { let r = { let t = m.env.code_va(0xd74b0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d786d mov rbx,QWORD PTR [r15+0x288]
            rbx = m.r64(r15.wrapping_add(0x288)) as u64;
            // d7874 mov r14d,0x100
            r14 = (0x100u64) as u32 as u64;
            // d787a add r14d,DWORD PTR [r15+0x298]
            { let (r, f) = add::<32>((r14 as u32 as u64), m.r32(r15.wrapping_add(0x298)) as u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d7881 shr r14d,0x8
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x8u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // d7885 mov DWORD PTR [rbp-0x2c],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((r14 as u32 as u64)) as u32);
            // d7889 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d788c mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // d7891 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // d7896 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d789b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d789e mov rdi,r15
            rdi = r15;
            // d78a1 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d78a4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d78a7 mov esi,0x2880
            rsi = (0x2880u64) as u32 as u64;
            // d78ac mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d78b1 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d78b4 mov rdi,r15
            rdi = r15;
            // d78b7 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d78ba call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d78bd test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // d78c0 je 0xd797c
            if fl.e() { pc = 0xd797c; continue; }
            pc = 0xd78c6;
        }
        0xd78c6 => {
            // block d78c6
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78c6 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d78c9 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd78cc;
        }
        0xd78cc => {
            // block d78cc
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78cc mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // d78d0 call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d78d5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d78d7 je 0xd7903
            if fl.e() { pc = 0xd7903; continue; }
            pc = 0xd78d9;
        }
        0xd78d9 => {
            // block d78d9
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78d9 cmp r12d,DWORD PTR [r15+0x338]
            fl = sub::<32>((r12 as u32 as u64), m.r32(r15.wrapping_add(0x338)) as u64).1;
            // d78e0 jne 0xd7903
            if fl.ne() { pc = 0xd7903; continue; }
            pc = 0xd78e2;
        }
        0xd78e2 => {
            // block d78e2
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78e2 cmp DWORD PTR [r15+0x370],0x1
            fl = sub::<32>(m.r32(r15.wrapping_add(0x370)) as u64, 0x1u64).1;
            // d78ea jne 0xd78f3
            if fl.ne() { pc = 0xd78f3; continue; }
            pc = 0xd78ec;
        }
        0xd78ec => {
            // block d78ec
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78ec mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // d78f1 jmp 0xd78f8
            pc = 0xd78f8; continue;
        }
        0xd78f3 => {
            // block d78f3
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78f3 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            pc = 0xd78f8;
        }
        0xd78f8 => {
            // block d78f8
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78f8 mov rdi,r15
            rdi = r15;
            // d78fb call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7900 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd7903;
        }
        0xd7903 => {
            // block d7903
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7903 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7903 mov rdi,r15
            rdi = r15;
            // d7906 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // d7909 mov rdx,rbx
            rdx = rbx;
            // d790c call 0xd7e14
            { let r = { let t = m.env.code_va(0xd7e14); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7911 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7914 mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // d7919 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d791e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7921 mov rdi,r15
            rdi = r15;
            // d7924 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // d7927 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d792a cmp BYTE PTR [r15+0x378],0x0
            fl = sub::<8>(m.r8(r15.wrapping_add(0x378)) as u64, 0x0u64).1;
            // d7932 jne 0xd795d
            if fl.ne() { pc = 0xd795d; continue; }
            pc = 0xd7934;
        }
        0xd7934 => {
            // block d7934
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7934 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7934 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d7936 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // d7939 div DWORD PTR [rbp-0x2c]
            { let (q, r) = div::<32>(rdx, rax, m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64); rax = q; rdx = r; }
            // d793c mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d793e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d7940 lea rdi,[rip+0x61402] # 0x138d49
            rdi = m.env.data_va(316694);
            // d7947 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // d7949 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d794e mov rax,QWORD PTR [rip+0x68a8b] # 0x1403e0
            rax = m.r64(m.env.data_va(347053)) as u64;
            // d7955 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // d7958 call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd795d;
        }
        0xd795d => {
            // block d795d
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d795d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d795d inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // d7960 add r12d,0x100
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x100u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d7967 add rbx,0x100
            { let (r, f) = add::<64>(rbx, 0x100u64);
              rbx = r;
              fl = f; }
            // d796e add r14d,0x64
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x64u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d7972 cmp r13d,DWORD PTR [rbp-0x2c]
            fl = sub::<32>((r13 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64).1;
            // d7976 jb 0xd78cc
            if fl.b() { pc = 0xd78cc; continue; }
            pc = 0xd797c;
        }
        0xd797c => {
            // block d797c
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d797c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d797c cmp BYTE PTR [r15+0x378],0x0
            fl = sub::<8>(m.r8(r15.wrapping_add(0x378)) as u64, 0x0u64).1;
            // d7984 jne 0xd7994
            if fl.ne() { pc = 0xd7994; continue; }
            pc = 0xd7986;
        }
        0xd7986 => {
            // block d7986
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7986 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7986 lea rdi,[rip+0x613d2] # 0x138d5f
            rdi = m.env.data_va(316716);
            // d798d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d798f call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd7994;
        }
        0xd7994 => {
            // block d7994
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7994 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7997 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d799c mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d79a1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d79a6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d79a9 mov rdi,r15
            rdi = r15;
            // d79ac call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d79af mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d79b2 mov rdi,r15
            rdi = r15;
            // d79b5 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d79bb mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d79be mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // d79c3 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // d79c8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d79cd xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d79d0 mov rdi,r15
            rdi = r15;
            // d79d3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d79d6 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d79d9 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d79de mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d79e3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d79e8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d79eb mov rdi,r15
            rdi = r15;
            // d79ee call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d79f1 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d79f4 mov rdi,r15
            rdi = r15;
            // d79f7 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d79fd mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // d7a00 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d7a02 je 0xd7a25
            if fl.e() { pc = 0xd7a25; continue; }
            pc = 0xd7a04;
        }
        0xd7a04 => {
            // block d7a04
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7a04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7a04 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d7a06 mov rdi,r15
            rdi = r15;
            // d7a09 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a0e mov esi,DWORD PTR [r15+0x370]
            rsi = (m.r32(r15.wrapping_add(0x370)) as u64) as u32 as u64;
            // d7a15 mov rdi,r15
            rdi = r15;
            // d7a18 call 0xd7eca
            { let r = { let t = m.env.code_va(0xd7eca); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a1d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d7a1f je 0xd7acc
            if fl.e() { pc = 0xd7acc; continue; }
            pc = 0xd7a25;
        }
        0xd7a25 => {
            // block d7a25
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7a25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7a25 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7a28 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d7a2d mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d7a32 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7a37 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7a3a mov rdi,r15
            rdi = r15;
            // d7a3d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a40 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7a43 mov rdi,r15
            rdi = r15;
            // d7a46 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a4c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7a4f mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // d7a54 mov edx,0x9c
            rdx = (0x9cu64) as u32 as u64;
            // d7a59 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7a5e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7a61 mov rdi,r15
            rdi = r15;
            // d7a64 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a67 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7a6a mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d7a6f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d7a74 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7a79 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7a7c mov rdi,r15
            rdi = r15;
            // d7a7f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a82 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7a85 mov rdi,r15
            rdi = r15;
            // d7a88 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a8e xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d7a90 mov rdi,r15
            rdi = r15;
            // d7a93 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d7a97 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d7a98 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d7a9a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d7a9c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d7a9e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d7aa0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d7aa1 jmp 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
        0xd7aa6 => {
            // block d7aa6
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7aa6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7aa6 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // d7aab call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7ab0 lea rcx,[rip+0x615e9] # 0x1390a0
            rcx = m.env.data_va(317549);
            // d7ab7 jmp 0xd7ae7
            pc = 0xd7ae7; continue;
        }
        0xd7ab9 => {
            // block d7ab9
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7ab9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7ab9 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // d7abe call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7ac3 lea rcx,[rip+0x61656] # 0x139120
            rcx = m.env.data_va(317677);
            // d7aca jmp 0xd7ae7
            pc = 0xd7ae7; continue;
        }
        0xd7acc => {
            // block d7acc
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7acc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7acc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d7ace mov rdi,r15
            rdi = r15;
            // d7ad1 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7ad6 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // d7adb call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7ae0 lea rcx,[rip+0x612a1] # 0x138d88
            rcx = m.env.data_va(316757);
            pc = 0xd7ae7;
        }
        0xd7ae7 => {
            // block d7ae7
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7ae7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7ae7 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // d7aea mov rsi,QWORD PTR [rip+0x686e7] # 0x1401d8
            rsi = m.r64(m.env.data_va(346533)) as u64;
            // d7af1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d7af3 mov rdi,rax
            rdi = rax;
            // d7af6 call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x27ca3 size=933
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00027ca3<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x27ca3;
    loop {
        match pc {
        0x27ca3 => {
            // block 27ca3
            #[cfg(feature = "trace")] eprintln!("f_00027ca3 pc 27ca3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27ca3 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 27ca4 mov rbp,rsp
            rbp = rsp;
            // 27ca7 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 27ca9 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 27cab push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 27cac sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 27cb0 mov r15,rsi
            r15 = rsi;
            // 27cb3 mov r14,rdi
            r14 = rdi;
            // 27cb6 mov esi,DWORD PTR [r15+0x248]
            rsi = (m.r32(r15.wrapping_add(0x248)) as u64) as u32 as u64;
            // 27cbd lea rbx,[rbp-0x30]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 27cc1 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 27cc6 mov rdi,rbx
            rdi = rbx;
            // 27cc9 call 0x61aed
            { let r = { let t = m.env.code_va(0x61aed); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x27cce;
        }
        0x27cce => {
            // block 27cce
            #[cfg(feature = "trace")] eprintln!("f_00027ca3 pc 27cce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27cce mov esi,0x20
            rsi = (0x20u64) as u32 as u64;
            // 27cd3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 27cd5 mov rdi,rbx
            rdi = rbx;
            // 27cd8 call 0xebfa8
            { let r = { let t = m.env.stub_va(0x775); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27cdd cmp rax,0xffffffffffffffff
            fl = sub::<64>(rax, 0xffff_ffff_ffff_ffffu64).1;
            // 27ce1 je 0x27d09
            if fl.e() { pc = 0x27d09; continue; }
            pc = 0x27ce3;
        }
        0x27ce3 => {
            // block 27ce3
            #[cfg(feature = "trace")] eprintln!("f_00027ca3 pc 27ce3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27ce3 mov esi,0x20
            rsi = (0x20u64) as u32 as u64;
            // 27ce8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 27cea mov rdi,rbx
            rdi = rbx;
            // 27ced call 0xebfa8
            { let r = { let t = m.env.stub_va(0x775); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27cf2 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 27cf7 mov rdi,rbx
            rdi = rbx;
            // 27cfa mov rsi,rax
            rsi = rax;
            // 27cfd call 0xebfde
            { let r = { let t = m.env.stub_va(0x7ab); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27d02 jmp 0x27cce
            pc = 0x27cce; continue;
        }
        0x27d04 => {
            // block 27d04
            #[cfg(feature = "trace")] eprintln!("f_00027ca3 pc 27d04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27d04 jmp 0x28034
            pc = 0x28034; continue;
        }
        0x27d09 => {
            // block 27d09
            #[cfg(feature = "trace")] eprintln!("f_00027ca3 pc 27d09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27d09 mov rdi,r15
            rdi = r15;
            // 27d0c mov rsi,r14
            rsi = r14;
            // 27d0f call 0x26172
            { let r = { let t = m.env.code_va(0x26172); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27d14 lea rsi,[rip+0x102943] # 0x12a65e
            rsi = m.env.data_va(257579);
            // 27d1b mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 27d20 mov rdi,rax
            rdi = rax;
            // 27d23 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27d28 lea rdi,[r15+0x20]
            rdi = r15.wrapping_add(0x20);
            // 27d2c mov rsi,rax
            rsi = rax;
            // 27d2f call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27d34 lea rsi,[rip+0x102929] # 0x12a664
            rsi = m.env.data_va(257585);
            // 27d3b mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 27d40 mov rdi,rax
            rdi = rax;
            // 27d43 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27d48 lea rdi,[r15+0x48]
            rdi = r15.wrapping_add(0x48);
            // 27d4c mov rsi,rax
            rsi = rax;
            // 27d4f call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27d54 lea rsi,[rip+0x10290f] # 0x12a66a
            rsi = m.env.data_va(257591);
            // 27d5b mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 27d60 mov rdi,rax
            rdi = rax;
            // 27d63 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27d68 lea rdi,[r15+0x70]
            rdi = r15.wrapping_add(0x70);
            // 27d6c mov rsi,rax
            rsi = rax;
            // 27d6f call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27d74 lea rsi,[rip+0x1028f7] # 0x12a672
            rsi = m.env.data_va(257599);
            // 27d7b mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 27d80 mov rdi,rax
            rdi = rax;
            // 27d83 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27d88 lea rdi,[r15+0x98]
            rdi = r15.wrapping_add(0x98);
            // 27d8f mov rsi,rax
            rsi = rax;
            // 27d92 call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27d97 lea rsi,[rip+0x1028dc] # 0x12a67a
            rsi = m.env.data_va(257607);
            // 27d9e mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 27da3 mov rdi,rax
            rdi = rax;
            // 27da6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27dab lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 27db2 mov rsi,rax
            rsi = rax;
            // 27db5 call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27dba lea rsi,[rip+0x1028c1] # 0x12a682
            rsi = m.env.data_va(257615);
            // 27dc1 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 27dc6 mov rdi,rax
            rdi = rax;
            // 27dc9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27dce lea rsi,[rip+0x1028af] # 0x12a684
            rsi = m.env.data_va(257617);
            // 27dd5 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 27dda mov rdi,rax
            rdi = rax;
            // 27ddd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27de2 mov rsi,QWORD PTR [r15+0x1fc]
            rsi = m.r64(r15.wrapping_add(0x1fc)) as u64;
            // 27de9 mov rdi,rax
            rdi = rax;
            // 27dec call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27df1 lea rsi,[rip+0x102895] # 0x12a68d
            rsi = m.env.data_va(257626);
            // 27df8 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 27dfd mov rdi,rax
            rdi = rax;
            // 27e00 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27e05 mov esi,DWORD PTR [r15+0x204]
            rsi = (m.r32(r15.wrapping_add(0x204)) as u64) as u32 as u64;
            // 27e0c mov rdi,rax
            rdi = rax;
            // 27e0f call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27e14 lea rsi,[rip+0x10287e] # 0x12a699
            rsi = m.env.data_va(257638);
            // 27e1b mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 27e20 mov rdi,rax
            rdi = rax;
            // 27e23 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27e28 lea rsi,[r15+0x208]
            rsi = r15.wrapping_add(0x208);
            // 27e2f mov rdi,rax
            rdi = rax;
            // 27e32 call 0x27bf8
            { let r = { let t = m.env.code_va(0x27bf8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27e37 lea rsi,[rip+0x102864] # 0x12a6a2
            rsi = m.env.data_va(257647);
            // 27e3e mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 27e43 mov rdi,rax
            rdi = rax;
            // 27e46 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27e4b lea rsi,[r15+0x218]
            rsi = r15.wrapping_add(0x218);
            // 27e52 mov rdi,rax
            rdi = rax;
            // 27e55 call 0x2847e
            { let r = { let t = m.env.code_va(0x2847e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27e5a lea rsi,[rip+0x10284a] # 0x12a6ab
            rsi = m.env.data_va(257656);
            // 27e61 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 27e66 mov rdi,rax
            rdi = rax;
            // 27e69 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27e6e movzx edx,BYTE PTR [rbp-0x30]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 27e72 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 27e75 jne 0x27e80
            if fl.ne() { pc = 0x27e80; continue; }
            pc = 0x27e77;
        }
        0x27e77 => {
            // block 27e77
            #[cfg(feature = "trace")] eprintln!("f_00027ca3 pc 27e77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27e77 lea rsi,[rbp-0x2f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd1);
            // 27e7b shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 27e7e jmp 0x27e88
            pc = 0x27e88; continue;
        }
        0x27e80 => {
            // block 27e80
            #[cfg(feature = "trace")] eprintln!("f_00027ca3 pc 27e80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27e80 mov rdx,QWORD PTR [rbp-0x28]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 27e84 mov rsi,QWORD PTR [rbp-0x20]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            pc = 0x27e88;
        }
        0x27e88 => {
            // block 27e88
            #[cfg(feature = "trace")] eprintln!("f_00027ca3 pc 27e88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27e88 mov rdi,rax
            rdi = rax;
            // 27e8b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27e90 lea rsi,[rip+0x10281a] # 0x12a6b1
            rsi = m.env.data_va(257662);
            // 27e97 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 27e9c mov rdi,rax
            rdi = rax;
            // 27e9f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27ea4 cmp DWORD PTR [r15+0x24c],0x1
            fl = sub::<32>(m.r32(r15.wrapping_add(0x24c)) as u64, 0x1u64).1;
            // 27eac lea rcx,[rip+0x102804] # 0x12a6b7
            rcx = m.env.data_va(257668);
            // 27eb3 lea rsi,[rip+0x102802] # 0x12a6bc
            rsi = m.env.data_va(257673);
            // 27eba cmove rsi,rcx
            if fl.e() { rsi = rcx; }
            // 27ebe mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 27ec3 mov rdi,rax
            rdi = rax;
            // 27ec6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27ecb lea rsi,[rip+0x1027ef] # 0x12a6c1
            rsi = m.env.data_va(257678);
            // 27ed2 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 27ed7 mov rdi,rax
            rdi = rax;
            // 27eda call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27edf lea rsi,[r15+0x250]
            rsi = r15.wrapping_add(0x250);
            // 27ee6 mov rdi,rax
            rdi = rax;
            // 27ee9 call 0x28220
            { let r = { let t = m.env.code_va(0x28220); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27eee lea rsi,[rip+0x1027d6] # 0x12a6cb
            rsi = m.env.data_va(257688);
            // 27ef5 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 27efa mov rdi,rax
            rdi = rax;
            // 27efd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27f02 mov esi,DWORD PTR [r15+0x26c]
            rsi = (m.r32(r15.wrapping_add(0x26c)) as u64) as u32 as u64;
            // 27f09 mov rdi,rax
            rdi = rax;
            // 27f0c call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27f11 lea rsi,[rip+0x1027bd] # 0x12a6d5
            rsi = m.env.data_va(257698);
            // 27f18 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 27f1d mov rdi,rax
            rdi = rax;
            // 27f20 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27f25 mov esi,DWORD PTR [r15+0x298]
            rsi = (m.r32(r15.wrapping_add(0x298)) as u64) as u32 as u64;
            // 27f2c mov rdi,rax
            rdi = rax;
            // 27f2f call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27f34 lea rsi,[rip+0x1027a0] # 0x12a6db
            rsi = m.env.data_va(257704);
            // 27f3b mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 27f40 mov rdi,rax
            rdi = rax;
            // 27f43 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27f48 mov esi,DWORD PTR [r15+0x29c]
            rsi = (m.r32(r15.wrapping_add(0x29c)) as u64) as u32 as u64;
            // 27f4f mov rdi,rax
            rdi = rax;
            // 27f52 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27f57 lea rsi,[rip+0x102786] # 0x12a6e4
            rsi = m.env.data_va(257713);
            // 27f5e mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 27f63 mov rdi,rax
            rdi = rax;
            // 27f66 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27f6b mov esi,DWORD PTR [r15+0x2a0]
            rsi = (m.r32(r15.wrapping_add(0x2a0)) as u64) as u32 as u64;
            // 27f72 mov rdi,rax
            rdi = rax;
            // 27f75 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27f7a lea rsi,[rip+0x10276c] # 0x12a6ed
            rsi = m.env.data_va(257722);
            // 27f81 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 27f86 mov rdi,rax
            rdi = rax;
            // 27f89 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27f8e lea rsi,[r15+0x2a4]
            rsi = r15.wrapping_add(0x2a4);
            // 27f95 mov rdi,rax
            rdi = rax;
            // 27f98 call 0x26f9d
            { let r = { let t = m.env.code_va(0x26f9d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27f9d lea rsi,[rip+0x102751] # 0x12a6f5
            rsi = m.env.data_va(257730);
            // 27fa4 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 27fa9 mov rdi,rax
            rdi = rax;
            // 27fac call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27fb1 mov esi,DWORD PTR [r15+0x2b0]
            rsi = (m.r32(r15.wrapping_add(0x2b0)) as u64) as u32 as u64;
            // 27fb8 mov rdi,rax
            rdi = rax;
            // 27fbb call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27fc0 lea rsi,[rip+0x102734] # 0x12a6fb
            rsi = m.env.data_va(257736);
            // 27fc7 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 27fcc mov rdi,rax
            rdi = rax;
            // 27fcf call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27fd4 lea rsi,[r15+0xe8]
            rsi = r15.wrapping_add(0xe8);
            // 27fdb mov rdi,rax
            rdi = rax;
            // 27fde call 0x28048
            { let r = { let t = m.env.code_va(0x28048); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27fe3 lea rsi,[rip+0x1024a4] # 0x12a48e
            rsi = m.env.data_va(257115);
            // 27fea mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 27fef mov rdi,rax
            rdi = rax;
            // 27ff2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27ff7 lea rsi,[rip+0x102296] # 0x12a294
            rsi = m.env.data_va(256609);
            // 27ffe mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 28003 mov rdi,rax
            rdi = rax;
            // 28006 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2800b add r15,0x2b4
            { let (r, f) = add::<64>(r15, 0x2b4u64);
              r15 = r;
              fl = f; }
            // 28012 mov rdi,rax
            rdi = rax;
            // 28015 mov rsi,r15
            rsi = r15;
            // 28018 call 0x26804
            { let r = { let t = m.env.code_va(0x26804); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2801d lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 28021 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28026 mov rax,r14
            rax = r14;
            // 28029 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2802d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2802e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28030 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 28032 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28033 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x28034 => {
            // block 28034
            #[cfg(feature = "trace")] eprintln!("f_00027ca3 pc 28034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28034 mov rbx,rax
            rbx = rax;
            // 28037 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2803b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28040 mov rdi,rbx
            rdi = rbx;
            // 28043 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2b046 size=924
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002b046<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x2b046;
    loop {
        match pc {
        0x2b046 => {
            // block 2b046
            #[cfg(feature = "trace")] eprintln!("f_0002b046 pc 2b046 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b046 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2b047 mov rbp,rsp
            rbp = rsp;
            // 2b04a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2b04c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2b04e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2b050 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2b052 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2b053 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2b054 mov rbx,rdi
            rbx = rdi;
            // 2b057 mov r10d,DWORD PTR [rbp+0x20]
            r10 = (m.r32(rbp.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2b05b mov rdi,QWORD PTR [rbp+0x18]
            rdi = m.r64(rbp.wrapping_add(0x18)) as u64;
            // 2b05f mov eax,DWORD PTR [rbp+0x10]
            rax = (m.r32(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // 2b062 mov DWORD PTR [rbx],0x0
            m.w32(rbx, (0x0u64) as u32);
            // 2b068 mov DWORD PTR [rbx+0x4],0x0
            m.w32(rbx.wrapping_add(0x4), (0x0u64) as u32);
            // 2b06f mov DWORD PTR [rbx+0x8],0x0
            m.w32(rbx.wrapping_add(0x8), (0x0u64) as u32);
            // 2b076 mov DWORD PTR [rbx+0xc],0x0
            m.w32(rbx.wrapping_add(0xc), (0x0u64) as u32);
            // 2b07d mov DWORD PTR [rbx+0x10],0x2bc
            m.w32(rbx.wrapping_add(0x10), (0x2bcu64) as u32);
            // 2b084 mov DWORD PTR [rbx+0x14],0x8
            m.w32(rbx.wrapping_add(0x14), (0x8u64) as u32);
            // 2b08b mov DWORD PTR [rbx+0x18],0x0
            m.w32(rbx.wrapping_add(0x18), (0x0u64) as u32);
            // 2b092 mov DWORD PTR [rbx+0x1c],0x0
            m.w32(rbx.wrapping_add(0x1c), (0x0u64) as u32);
            // 2b099 mov QWORD PTR [rbx+0x20],rsi
            m.w64(rbx.wrapping_add(0x20), (rsi) as u64);
            // 2b09d mov DWORD PTR [rbx+0x28],edx
            m.w32(rbx.wrapping_add(0x28), ((rdx as u32 as u64)) as u32);
            // 2b0a0 mov DWORD PTR [rbx+0x44],0x0
            m.w32(rbx.wrapping_add(0x44), (0x0u64) as u32);
            // 2b0a7 mov QWORD PTR [rbx+0x3c],0x0
            m.w64(rbx.wrapping_add(0x3c), (0x0u64) as u64);
            // 2b0af mov QWORD PTR [rbx+0x34],0x0
            m.w64(rbx.wrapping_add(0x34), (0x0u64) as u64);
            // 2b0b7 mov QWORD PTR [rbx+0x2c],0x0
            m.w64(rbx.wrapping_add(0x2c), (0x0u64) as u64);
            // 2b0bf mov QWORD PTR [rbx+0x48],rcx
            m.w64(rbx.wrapping_add(0x48), (rcx) as u64);
            // 2b0c3 mov DWORD PTR [rbx+0x50],r8d
            m.w32(rbx.wrapping_add(0x50), ((r8 as u32 as u64)) as u32);
            // 2b0c7 mov DWORD PTR [rbx+0x6c],0x0
            m.w32(rbx.wrapping_add(0x6c), (0x0u64) as u32);
            // 2b0ce mov QWORD PTR [rbx+0x64],0x0
            m.w64(rbx.wrapping_add(0x64), (0x0u64) as u64);
            // 2b0d6 mov QWORD PTR [rbx+0x5c],0x0
            m.w64(rbx.wrapping_add(0x5c), (0x0u64) as u64);
            // 2b0de mov QWORD PTR [rbx+0x54],0x0
            m.w64(rbx.wrapping_add(0x54), (0x0u64) as u64);
            // 2b0e6 mov QWORD PTR [rbx+0x70],r9
            m.w64(rbx.wrapping_add(0x70), (r9) as u64);
            // 2b0ea mov DWORD PTR [rbx+0x78],eax
            m.w32(rbx.wrapping_add(0x78), ((rax as u32 as u64)) as u32);
            // 2b0ed mov DWORD PTR [rbx+0x94],0x0
            m.w32(rbx.wrapping_add(0x94), (0x0u64) as u32);
            // 2b0f7 mov QWORD PTR [rbx+0x8c],0x0
            m.w64(rbx.wrapping_add(0x8c), (0x0u64) as u64);
            // 2b102 mov QWORD PTR [rbx+0x84],0x0
            m.w64(rbx.wrapping_add(0x84), (0x0u64) as u64);
            // 2b10d mov QWORD PTR [rbx+0x7c],0x0
            m.w64(rbx.wrapping_add(0x7c), (0x0u64) as u64);
            // 2b115 mov QWORD PTR [rbx+0x98],rdi
            m.w64(rbx.wrapping_add(0x98), (rdi) as u64);
            // 2b11c mov DWORD PTR [rbx+0xa0],r10d
            m.w32(rbx.wrapping_add(0xa0), ((r10 as u32 as u64)) as u32);
            // 2b123 lea r14,[rbx+0xc0]
            r14 = rbx.wrapping_add(0xc0);
            // 2b12a mov DWORD PTR [rbx+0xe4],0x0
            m.w32(rbx.wrapping_add(0xe4), (0x0u64) as u32);
            // 2b134 mov QWORD PTR [rbx+0xdc],0x0
            m.w64(rbx.wrapping_add(0xdc), (0x0u64) as u64);
            // 2b13f mov QWORD PTR [rbx+0xd4],0x0
            m.w64(rbx.wrapping_add(0xd4), (0x0u64) as u64);
            // 2b14a mov QWORD PTR [rbx+0xcc],0x0
            m.w64(rbx.wrapping_add(0xcc), (0x0u64) as u64);
            // 2b155 mov QWORD PTR [rbx+0xc4],0x0
            m.w64(rbx.wrapping_add(0xc4), (0x0u64) as u64);
            // 2b160 mov QWORD PTR [rbx+0xbc],0x0
            m.w64(rbx.wrapping_add(0xbc), (0x0u64) as u64);
            // 2b16b mov QWORD PTR [rbx+0xb4],0x0
            m.w64(rbx.wrapping_add(0xb4), (0x0u64) as u64);
            // 2b176 mov QWORD PTR [rbx+0xac],0x0
            m.w64(rbx.wrapping_add(0xac), (0x0u64) as u64);
            // 2b181 mov QWORD PTR [rbx+0xa4],0x0
            m.w64(rbx.wrapping_add(0xa4), (0x0u64) as u64);
            // 2b18c mov esi,0x144
            rsi = (0x144u64) as u32 as u64;
            // 2b191 mov rdi,r14
            rdi = r14;
            // 2b194 call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b199 mov rdi,QWORD PTR [r14]
            rdi = m.r64(r14) as u64;
            // 2b19c test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2b19f je 0x2b1ac
            if fl.e() { pc = 0x2b1ac; continue; }
            pc = 0x2b1a1;
        }
        0x2b1a1 => {
            // block 2b1a1
            #[cfg(feature = "trace")] eprintln!("f_0002b046 pc 2b1a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b1a1 mov esi,DWORD PTR [rbx+0xc8]
            rsi = (m.r32(rbx.wrapping_add(0xc8)) as u64) as u32 as u64;
            // 2b1a7 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2b1ac;
        }
        0x2b1ac => {
            // block 2b1ac
            #[cfg(feature = "trace")] eprintln!("f_0002b046 pc 2b1ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b1ac mov DWORD PTR [rbx+0xe8],0x0
            m.w32(rbx.wrapping_add(0xe8), (0x0u64) as u32);
            // 2b1b6 mov QWORD PTR [rbx+0xec],0x0
            m.w64(rbx.wrapping_add(0xec), (0x0u64) as u64);
            // 2b1c1 movabs rax,0x11400000000
            rax = 0x0_0114_0000_0000u64;
            // 2b1cb mov QWORD PTR [rbx+0xf4],rax
            m.w64(rbx.wrapping_add(0xf4), (rax) as u64);
            // 2b1d2 mov DWORD PTR [rbx+0xfc],0x8
            m.w32(rbx.wrapping_add(0xfc), (0x8u64) as u32);
            // 2b1dc lea rdi,[rbx+0x11c]
            rdi = rbx.wrapping_add(0x11c);
            // 2b1e3 mov DWORD PTR [rbx+0x118],0x0
            m.w32(rbx.wrapping_add(0x118), (0x0u64) as u32);
            // 2b1ed mov QWORD PTR [rbx+0x110],0x0
            m.w64(rbx.wrapping_add(0x110), (0x0u64) as u64);
            // 2b1f8 mov QWORD PTR [rbx+0x108],0x0
            m.w64(rbx.wrapping_add(0x108), (0x0u64) as u64);
            // 2b203 mov QWORD PTR [rbx+0x100],0x0
            m.w64(rbx.wrapping_add(0x100), (0x0u64) as u64);
            // 2b20e call 0x2946a
            { let r = { let t = m.env.code_va(0x2946a); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b213 mov DWORD PTR [rbx+0x1f4],0x0
            m.w32(rbx.wrapping_add(0x1f4), (0x0u64) as u32);
            // 2b21d mov QWORD PTR [rbx+0x1ec],0x0
            m.w64(rbx.wrapping_add(0x1ec), (0x0u64) as u64);
            // 2b228 mov QWORD PTR [rbx+0x1e4],0x0
            m.w64(rbx.wrapping_add(0x1e4), (0x0u64) as u64);
            // 2b233 mov DWORD PTR [rbx+0x1f8],0x0
            m.w32(rbx.wrapping_add(0x1f8), (0x0u64) as u32);
            // 2b23d mov DWORD PTR [rbx+0x214],0x0
            m.w32(rbx.wrapping_add(0x214), (0x0u64) as u32);
            // 2b247 mov QWORD PTR [rbx+0x20c],0x0
            m.w64(rbx.wrapping_add(0x20c), (0x0u64) as u64);
            // 2b252 mov QWORD PTR [rbx+0x204],0x0
            m.w64(rbx.wrapping_add(0x204), (0x0u64) as u64);
            // 2b25d mov QWORD PTR [rbx+0x1fc],0x0
            m.w64(rbx.wrapping_add(0x1fc), (0x0u64) as u64);
            // 2b268 mov DWORD PTR [rbx+0x218],0x4
            m.w32(rbx.wrapping_add(0x218), (0x4u64) as u32);
            // 2b272 mov QWORD PTR [rbx+0x294],0x0
            m.w64(rbx.wrapping_add(0x294), (0x0u64) as u64);
            // 2b27d mov QWORD PTR [rbx+0x28c],0x0
            m.w64(rbx.wrapping_add(0x28c), (0x0u64) as u64);
            // 2b288 mov QWORD PTR [rbx+0x284],0x0
            m.w64(rbx.wrapping_add(0x284), (0x0u64) as u64);
            // 2b293 mov QWORD PTR [rbx+0x27c],0x0
            m.w64(rbx.wrapping_add(0x27c), (0x0u64) as u64);
            // 2b29e mov QWORD PTR [rbx+0x274],0x0
            m.w64(rbx.wrapping_add(0x274), (0x0u64) as u64);
            // 2b2a9 mov QWORD PTR [rbx+0x26c],0x0
            m.w64(rbx.wrapping_add(0x26c), (0x0u64) as u64);
            // 2b2b4 mov QWORD PTR [rbx+0x264],0x0
            m.w64(rbx.wrapping_add(0x264), (0x0u64) as u64);
            // 2b2bf mov QWORD PTR [rbx+0x25c],0x0
            m.w64(rbx.wrapping_add(0x25c), (0x0u64) as u64);
            // 2b2ca mov QWORD PTR [rbx+0x254],0x0
            m.w64(rbx.wrapping_add(0x254), (0x0u64) as u64);
            // 2b2d5 mov QWORD PTR [rbx+0x24c],0x0
            m.w64(rbx.wrapping_add(0x24c), (0x0u64) as u64);
            // 2b2e0 mov QWORD PTR [rbx+0x244],0x0
            m.w64(rbx.wrapping_add(0x244), (0x0u64) as u64);
            // 2b2eb mov QWORD PTR [rbx+0x23c],0x0
            m.w64(rbx.wrapping_add(0x23c), (0x0u64) as u64);
            // 2b2f6 mov QWORD PTR [rbx+0x234],0x0
            m.w64(rbx.wrapping_add(0x234), (0x0u64) as u64);
            // 2b301 mov QWORD PTR [rbx+0x22c],0x0
            m.w64(rbx.wrapping_add(0x22c), (0x0u64) as u64);
            // 2b30c mov QWORD PTR [rbx+0x224],0x0
            m.w64(rbx.wrapping_add(0x224), (0x0u64) as u64);
            // 2b317 mov QWORD PTR [rbx+0x21c],0x0
            m.w64(rbx.wrapping_add(0x21c), (0x0u64) as u64);
            // 2b322 mov DWORD PTR [rbx+0x29c],0x1
            m.w32(rbx.wrapping_add(0x29c), (0x1u64) as u32);
            // 2b32c mov QWORD PTR [rbx+0x2a8],0xffffffffffffffff
            m.w64(rbx.wrapping_add(0x2a8), (0xffff_ffff_ffff_ffffu64) as u64);
            // 2b337 mov QWORD PTR [rbx+0x2a0],0xffffffffffffffff
            m.w64(rbx.wrapping_add(0x2a0), (0xffff_ffff_ffff_ffffu64) as u64);
            // 2b342 mov QWORD PTR [rbx+0x2b0],0x12
            m.w64(rbx.wrapping_add(0x2b0), (0x12u64) as u64);
            // 2b34d mov DWORD PTR [rbx+0x2b8],0x0
            m.w32(rbx.wrapping_add(0x2b8), (0x0u64) as u32);
            // 2b357 mov rdi,QWORD PTR [rbx+0xc0]
            rdi = m.r64(rbx.wrapping_add(0xc0)) as u64;
            // 2b35e test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2b361 je 0x2b373
            if fl.e() { pc = 0x2b373; continue; }
            pc = 0x2b363;
        }
        0x2b363 => {
            // block 2b363
            #[cfg(feature = "trace")] eprintln!("f_0002b046 pc 2b363 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b363 mov edx,DWORD PTR [rbx+0xc8]
            rdx = (m.r32(rbx.wrapping_add(0xc8)) as u64) as u32 as u64;
            // 2b369 mov esi,0xff
            rsi = (0xffu64) as u32 as u64;
            // 2b36e call 0xec22a
            { let r = { let t = m.env.stub_va(0x9f7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2b373;
        }
        0x2b373 => {
            // block 2b373
            #[cfg(feature = "trace")] eprintln!("f_0002b046 pc 2b373 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b373 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2b377 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2b378 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2b37a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2b37c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2b37e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2b380 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2b381 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2b382 => {
            // block 2b382
            #[cfg(feature = "trace")] eprintln!("f_0002b046 pc 2b382 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b382 mov r15,rax
            r15 = rax;
            // 2b385 jmp 0x2b396
            pc = 0x2b396; continue;
        }
        0x2b387 => {
            // block 2b387
            #[cfg(feature = "trace")] eprintln!("f_0002b046 pc 2b387 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b387 mov r15,rax
            r15 = rax;
            // 2b38a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2b38c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2b38e mov rdi,r14
            rdi = r14;
            // 2b391 call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2b396;
        }
        0x2b396 => {
            // block 2b396
            #[cfg(feature = "trace")] eprintln!("f_0002b046 pc 2b396 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b396 lea r14,[rbx+0x48]
            r14 = rbx.wrapping_add(0x48);
            // 2b39a lea r12,[rbx+0x70]
            r12 = rbx.wrapping_add(0x70);
            // 2b39e lea r13,[rbx+0x20]
            r13 = rbx.wrapping_add(0x20);
            // 2b3a2 add rbx,0x98
            { let (r, f) = add::<64>(rbx, 0x98u64);
              rbx = r;
              fl = f; }
            // 2b3a9 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2b3ab xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2b3ad mov rdi,rbx
            rdi = rbx;
            // 2b3b0 call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b3b5 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2b3b7 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2b3b9 mov rdi,r12
            rdi = r12;
            // 2b3bc call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b3c1 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2b3c3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2b3c5 mov rdi,r14
            rdi = r14;
            // 2b3c8 call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b3cd xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2b3cf xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2b3d1 mov rdi,r13
            rdi = r13;
            // 2b3d4 call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b3d9 mov rdi,r15
            rdi = r15;
            // 2b3dc call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc86a9 size=906
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c86a9<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xc86a9;
    loop {
        match pc {
        0xc86a9 => {
            // block c86a9
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c86a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c86a9 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c86aa mov rbp,rsp
            rbp = rsp;
            // c86ad push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c86af push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c86b1 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c86b3 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c86b5 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c86b6 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // c86ba mov DWORD PTR [rbp-0x30],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rcx as u32 as u64)) as u32);
            // c86bd mov r13d,edx
            r13 = ((rdx as u32 as u64)) as u32 as u64;
            // c86c0 mov r15,rsi
            r15 = rsi;
            // c86c3 mov r14,rdi
            r14 = rdi;
            // c86c6 lea eax,[r13+0x4]
            rax = (r13.wrapping_add(0x4)) as u32 as u64;
            // c86ca movsxd rdi,eax
            rdi = sx::<32>((rax as u32 as u64)) as u64;
            // c86cd add rdi,rdi
            { let (r, f) = add::<64>(rdi, rdi);
              rdi = r;
              fl = f; }
            // c86d0 mov rax,0xffffffffffffffff
            rax = 0xffff_ffff_ffff_ffffu64;
            // c86d7 cmovb rdi,rax
            if fl.b() { rdi = rax; }
            // c86db call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c86e0 mov rbx,rax
            rbx = rax;
            // c86e3 mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c86e6 shr eax,0x1f
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1fu64, fl); rax = (r) as u32 as u64; fl = f; }
            // c86e9 add eax,r13d
            { let (r, f) = add::<32>((rax as u32 as u64), (r13 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c86ec sar eax,1
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c86ee mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // c86f2 lea eax,[rax+0x4]
            rax = (rax.wrapping_add(0x4)) as u32 as u64;
            // c86f5 movsxd r12,eax
            r12 = sx::<32>((rax as u32 as u64)) as u64;
            // c86f8 add r12,r12
            { let (r, f) = add::<64>(r12, r12);
              r12 = r;
              fl = f; }
            // c86fb mov rax,0xffffffffffffffff
            rax = 0xffff_ffff_ffff_ffffu64;
            // c8702 cmovb r12,rax
            if fl.b() { r12 = rax; }
            // c8706 mov rdi,r12
            rdi = r12;
            // c8709 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c870e mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // c8712 mov rdi,r12
            rdi = r12;
            // c8715 mov r12,rbx
            r12 = rbx;
            // c8718 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c871d mov r8,rax
            r8 = rax;
            // c8720 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // c8723 jle 0xc8775
            if fl.le() { pc = 0xc8775; continue; }
            pc = 0xc8725;
        }
        0xc8725 => {
            // block c8725
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c8725 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8725 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c8727 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c8729 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0xc872d;
        }
        0xc872d => {
            // block c872d
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c872d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c872d mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // c872f shr esi,0x1f
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x1fu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // c8732 add esi,ecx
            { let (r, f) = add::<32>((rsi as u32 as u64), (rcx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // c8734 sar esi,1
            { let (r, f) = sar::<32>((rsi as u32 as u64), 0x1u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // c8736 inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // c8738 test cl,0x1
            fl = logic::<8>((rcx as u8 as u64) & 0x1u64);
            // c873b movsxd rdx,eax
            rdx = sx::<32>((rax as u32 as u64)) as u64;
            // c873e movzx edx,WORD PTR [r14+rdx*2]
            rdx = (m.r16(r14.wrapping_add(rdx.wrapping_mul(2))) as u64) as u32 as u64;
            // c8743 movsxd rsi,esi
            rsi = sx::<32>((rsi as u32 as u64)) as u64;
            // c8746 jne 0xc874e
            if fl.ne() { pc = 0xc874e; continue; }
            pc = 0xc8748;
        }
        0xc8748 => {
            // block c8748
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c8748 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8748 mov WORD PTR [rdi+rsi*2],dx
            m.w16(rdi.wrapping_add(rsi.wrapping_mul(2)), ((rdx as u16 as u64)) as u16);
            // c874c jmp 0xc8753
            pc = 0xc8753; continue;
        }
        0xc874e => {
            // block c874e
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c874e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c874e mov WORD PTR [r8+rsi*2],dx
            m.w16(r8.wrapping_add(rsi.wrapping_mul(2)), ((rdx as u16 as u64)) as u16);
            pc = 0xc8753;
        }
        0xc8753 => {
            // block c8753
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c8753 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8753 lea edx,[rax+0x1]
            rdx = (rax.wrapping_add(0x1)) as u32 as u64;
            // c8756 movsxd rdx,edx
            rdx = sx::<32>((rdx as u32 as u64)) as u64;
            // c8759 mov dx,WORD PTR [r14+rdx*2]
            rdx = (rdx & !0xffff) | ((m.r16(r14.wrapping_add(rdx.wrapping_mul(2))) as u64) as u16 as u64);
            // c875e mov WORD PTR [r12+rax*1+0x2],dx
            m.w16(r12.wrapping_add(rax).wrapping_add(0x2), ((rdx as u16 as u64)) as u16);
            // c8764 inc ecx
            { let (r, f) = inc::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // c8766 add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // c876a cmp r13d,ecx
            fl = sub::<32>((r13 as u32 as u64), (rcx as u32 as u64)).1;
            // c876d jne 0xc872d
            if fl.ne() { pc = 0xc872d; continue; }
            pc = 0xc876f;
        }
        0xc876f => {
            // block c876f
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c876f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c876f movzx eax,WORD PTR [r12+0x2]
            rax = (m.r16(r12.wrapping_add(0x2)) as u64) as u32 as u64;
            pc = 0xc8775;
        }
        0xc8775 => {
            // block c8775
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c8775 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8775 mov WORD PTR [r12],ax
            m.w16(r12, ((rax as u16 as u64)) as u16);
            // c877a movsxd rax,r13d
            rax = sx::<32>((r13 as u32 as u64)) as u64;
            // c877d mov ax,WORD PTR [r12+rax*2]
            rax = (rax & !0xffff) | ((m.r16(r12.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // c8782 lea ecx,[r13+0x2]
            rcx = (r13.wrapping_add(0x2)) as u32 as u64;
            // c8786 movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // c8789 mov WORD PTR [r12+rcx*2],ax
            m.w16(r12.wrapping_add(rcx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // c878e lea ecx,[r13+0x1]
            rcx = (r13.wrapping_add(0x1)) as u32 as u64;
            // c8792 movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // c8795 mov WORD PTR [r12+rcx*2],ax
            m.w16(r12.wrapping_add(rcx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // c879a shl r13d,0x10
            { let (r, f) = shl::<32>((r13 as u32 as u64), 0x10u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // c879e mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c87a1 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c87a2 mov ecx,DWORD PTR [rbp-0x30]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // c87a5 idiv ecx
            { let (q, r) = idiv::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // c87a7 mov DWORD PTR [rbp-0x4c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), ((rax as u32 as u64)) as u32);
            // c87aa test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // c87ac jle 0xc8867
            if fl.le() { pc = 0xc8867; continue; }
            pc = 0xc87b2;
        }
        0xc87b2 => {
            // block c87b2
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c87b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c87b2 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // c87b7 xor r11d,r11d
            { let r = (r11 as u32 as u64) ^ (r11 as u32 as u64); let f = logic::<32>(r);
              r11 = (r) as u32 as u64;
              fl = f; }
            // c87ba lea r9,[rip+0x80bff] # 0x1493c0
            r9 = m.env.data_va(383885);
            // c87c1 mov r14w,0x4
            r14 = (r14 & !0xffff) | ((0x4u64) as u16 as u64);
            // c87c6 mov r10d,DWORD PTR [rbp-0x30]
            r10 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            pc = 0xc87ca;
        }
        0xc87ca => {
            // block c87ca
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c87ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c87ca mov edx,r11d
            rdx = ((r11 as u32 as u64)) as u32 as u64;
            // c87cd sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // c87d0 mov esi,r11d
            rsi = ((r11 as u32 as u64)) as u32 as u64;
            // c87d3 shr esi,0xb
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xbu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // c87d6 and esi,0x1f
            { let r = (rsi as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // c87d9 inc edx
            { let (r, f) = inc::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // c87db movsxd r13,edx
            r13 = sx::<32>((rdx as u32 as u64)) as u64;
            // c87de movsx ebx,WORD PTR [r12+r13*2-0x2]
            rbx = (sx::<16>(m.r16(r12.wrapping_add(r13.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // c87e4 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // c87e9 sub edi,esi
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rsi as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c87eb imul ebx,DWORD PTR [r9+rdi*4]
            { let (r, f) = imul::<32>((rbx as u32 as u64), m.r32(r9.wrapping_add(rdi.wrapping_mul(4))) as u64); rbx = (r) as u32 as u64; fl = f; }
            // c87f0 movsx edi,WORD PTR [r12+r13*2]
            rdi = (sx::<16>(m.r16(r12.wrapping_add(r13.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // c87f5 mov edx,0x40
            rdx = (0x40u64) as u32 as u64;
            // c87fa sub edx,esi
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c87fc imul edi,DWORD PTR [r9+rdx*4]
            { let (r, f) = imul::<32>((rdi as u32 as u64), m.r32(r9.wrapping_add(rdx.wrapping_mul(4))) as u64); rdi = (r) as u32 as u64; fl = f; }
            // c8801 add edi,ebx
            { let (r, f) = add::<32>((rdi as u32 as u64), (rbx as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c8803 movsx edx,WORD PTR [r12+r13*2+0x2]
            rdx = (sx::<16>(m.r16(r12.wrapping_add(r13.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // c8809 mov ebx,0x60
            rbx = (0x60u64) as u32 as u64;
            // c880e sub ebx,esi
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rsi as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c8810 imul edx,DWORD PTR [r9+rbx*4]
            { let (r, f) = imul::<32>((rdx as u32 as u64), m.r32(r9.wrapping_add(rbx.wrapping_mul(4))) as u64); rdx = (r) as u32 as u64; fl = f; }
            // c8815 add edx,edi
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdi as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8817 movsx ebx,WORD PTR [r12+r13*2+0x4]
            rbx = (sx::<16>(m.r16(r12.wrapping_add(r13.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // c881d mov edi,0x80
            rdi = (0x80u64) as u32 as u64;
            // c8822 sub edi,esi
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rsi as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c8824 imul ebx,DWORD PTR [r9+rdi*4]
            { let (r, f) = imul::<32>((rbx as u32 as u64), m.r32(r9.wrapping_add(rdi.wrapping_mul(4))) as u64); rbx = (r) as u32 as u64; fl = f; }
            // c8829 add ebx,edx
            { let (r, f) = add::<32>((rbx as u32 as u64), (rdx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c882b mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // c882d and edx,0xffff0000
            { let r = (rdx as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8833 mov si,0x3fb
            rsi = (rsi & !0xffff) | ((0x3fbu64) as u16 as u64);
            // c8837 cmp edx,0x3fb0000
            fl = sub::<32>((rdx as u32 as u64), 0x3fb0000u64).1;
            // c883d jg 0xc8850
            if fl.g() { pc = 0xc8850; continue; }
            pc = 0xc883f;
        }
        0xc883f => {
            // block c883f
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c883f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c883f sar ebx,0x10
            { let (r, f) = sar::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c8842 cmp edx,0x40000
            fl = sub::<32>((rdx as u32 as u64), 0x40000u64).1;
            // c8848 cmovl bx,r14w
            if fl.l() { rbx = (rbx & !0xffff) | (((r14 as u16 as u64)) as u16 as u64); }
            // c884d mov si,bx
            rsi = (rsi & !0xffff) | (((rbx as u16 as u64)) as u16 as u64);
            pc = 0xc8850;
        }
        0xc8850 => {
            // block c8850
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c8850 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8850 movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // c8853 mov WORD PTR [r15+rcx*2],si
            m.w16(r15.wrapping_add(rcx.wrapping_mul(2)), ((rsi as u16 as u64)) as u16);
            // c8858 add ecx,0x2
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x2u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c885b add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c885e dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // c8861 jne 0xc87ca
            if fl.ne() { pc = 0xc87ca; continue; }
            pc = 0xc8867;
        }
        0xc8867 => {
            // block c8867
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c8867 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8867 mov QWORD PTR [rbp-0x60],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (r12) as u64);
            // c886b mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // c886f mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c8873 mov ax,WORD PTR [rdi+0x2]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x2)) as u64) as u16 as u64);
            // c8877 mov WORD PTR [rdi],ax
            m.w16(rdi, ((rax as u16 as u64)) as u16);
            // c887a mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // c887e movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // c8881 mov cx,WORD PTR [rdi+rax*2]
            rcx = (rcx & !0xffff) | ((m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // c8885 lea edx,[rsi+0x2]
            rdx = (rsi.wrapping_add(0x2)) as u32 as u64;
            // c8888 movsxd rdx,edx
            rdx = sx::<32>((rdx as u32 as u64)) as u64;
            // c888b mov WORD PTR [rdi+rdx*2],cx
            m.w16(rdi.wrapping_add(rdx.wrapping_mul(2)), ((rcx as u16 as u64)) as u16);
            // c888f inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // c8891 movsxd rsi,esi
            rsi = sx::<32>((rsi as u32 as u64)) as u64;
            // c8894 mov WORD PTR [rdi+rsi*2],cx
            m.w16(rdi.wrapping_add(rsi.wrapping_mul(2)), ((rcx as u16 as u64)) as u16);
            // c8898 mov QWORD PTR [rbp-0x40],r8
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r8) as u64);
            // c889c mov cx,WORD PTR [r8+0x2]
            rcx = (rcx & !0xffff) | ((m.r16(r8.wrapping_add(0x2)) as u64) as u16 as u64);
            // c88a1 mov WORD PTR [r8],cx
            m.w16(r8, ((rcx as u16 as u64)) as u16);
            // c88a5 mov ax,WORD PTR [r8+rax*2]
            rax = (rax & !0xffff) | ((m.r16(r8.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // c88aa mov WORD PTR [r8+rdx*2],ax
            m.w16(r8.wrapping_add(rdx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // c88af mov WORD PTR [r8+rsi*2],ax
            m.w16(r8.wrapping_add(rsi.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // c88b4 mov ecx,DWORD PTR [rbp-0x30]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // c88b7 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // c88ba jl 0xc8a0a
            if fl.l() { pc = 0xc8a0a; continue; }
            pc = 0xc88c0;
        }
        0xc88c0 => {
            // block c88c0
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c88c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c88c0 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c88c2 shr eax,0x1f
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1fu64, fl); rax = (r) as u32 as u64; fl = f; }
            // c88c5 add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c88c7 sar eax,1
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c88c9 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // c88cb mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // c88cf mov r15d,0x2
            r15 = (0x2u64) as u32 as u64;
            // c88d5 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // c88d8 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc88db;
        }
        0xc88db => {
            // block c88db
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c88db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c88db mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // c88de sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c88e1 mov ebx,r14d
            rbx = ((r14 as u32 as u64)) as u32 as u64;
            // c88e4 shr ebx,0xb
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xbu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c88e7 and ebx,0x1f
            { let r = (rbx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c88ea inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // c88ec movsxd r11,eax
            r11 = sx::<32>((rax as u32 as u64)) as u64;
            // c88ef mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c88f3 movsx eax,WORD PTR [rsi+r11*2-0x2]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // c88f9 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // c88fe sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c8900 lea rdx,[rip+0x80ab9] # 0x1493c0
            rdx = m.env.data_va(383885);
            // c8907 mov rdi,rdx
            rdi = rdx;
            // c890a mov ecx,DWORD PTR [rdi+rcx*4]
            rcx = (m.r32(rdi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // c890d mov DWORD PTR [rbp-0x30],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rcx as u32 as u64)) as u32);
            // c8910 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c8913 movsx ecx,WORD PTR [rsi+r11*2]
            rcx = (sx::<16>(m.r16(rsi.wrapping_add(r11.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // c8918 mov edx,0x40
            rdx = (0x40u64) as u32 as u64;
            // c891d sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c891f mov r13d,DWORD PTR [rdi+rdx*4]
            r13 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // c8923 imul ecx,r13d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r13 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c8927 add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c8929 movsx eax,WORD PTR [rsi+r11*2+0x2]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // c892f mov edx,0x60
            rdx = (0x60u64) as u32 as u64;
            // c8934 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8936 mov r9d,DWORD PTR [rdi+rdx*4]
            r9 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // c893a imul eax,r9d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r9 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c893e add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c8940 movsx r10d,WORD PTR [rsi+r11*2+0x4]
            r10 = (sx::<16>(m.r16(rsi.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // c8946 mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // c894b sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c894d mov r8d,DWORD PTR [rdi+rcx*4]
            r8 = (m.r32(rdi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // c8951 imul r10d,r8d
            { let (r, f) = imul::<32>((r10 as u32 as u64), (r8 as u32 as u64)); r10 = (r) as u32 as u64; fl = f; }
            // c8955 add r10d,eax
            { let (r, f) = add::<32>((r10 as u32 as u64), (rax as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // c8958 mov ebx,r10d
            rbx = ((r10 as u32 as u64)) as u32 as u64;
            // c895b and ebx,0xffff0000
            { let r = (rbx as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c8961 mov si,0x3fb
            rsi = (rsi & !0xffff) | ((0x3fbu64) as u16 as u64);
            // c8965 cmp ebx,0x3fb0000
            fl = sub::<32>((rbx as u32 as u64), 0x3fb0000u64).1;
            // c896b mov ax,0x3fb
            rax = (rax & !0xffff) | ((0x3fbu64) as u16 as u64);
            // c896f jg 0xc8988
            if fl.g() { pc = 0xc8988; continue; }
            pc = 0xc8971;
        }
        0xc8971 => {
            // block c8971
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c8971 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8971 sar r10d,0x10
            { let (r, f) = sar::<32>((r10 as u32 as u64), 0x10u64, fl); r10 = (r) as u32 as u64; fl = f; }
            // c8975 cmp ebx,0x40000
            fl = sub::<32>((rbx as u32 as u64), 0x40000u64).1;
            // c897b mov ax,0x4
            rax = (rax & !0xffff) | ((0x4u64) as u16 as u64);
            // c897f cmovl r10w,ax
            if fl.l() { r10 = (r10 & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            // c8984 mov ax,r10w
            rax = (rax & !0xffff) | (((r10 as u16 as u64)) as u16 as u64);
            pc = 0xc8988;
        }
        0xc8988 => {
            // block c8988
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c8988 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8988 lea ecx,[r15-0x2]
            rcx = (r15.wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // c898c movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // c898f mov rdx,QWORD PTR [rbp-0x48]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // c8993 mov WORD PTR [rdx+rcx*2],ax
            m.w16(rdx.wrapping_add(rcx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // c8997 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // c899b movsx eax,WORD PTR [rbx+r11*2-0x2]
            rax = (sx::<16>(m.r16(rbx.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // c89a1 imul eax,DWORD PTR [rbp-0x30]
            { let (r, f) = imul::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64); rax = (r) as u32 as u64; fl = f; }
            // c89a5 movsx ecx,WORD PTR [rbx+r11*2]
            rcx = (sx::<16>(m.r16(rbx.wrapping_add(r11.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // c89aa imul ecx,r13d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r13 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c89ae add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c89b0 movsx eax,WORD PTR [rbx+r11*2+0x2]
            rax = (sx::<16>(m.r16(rbx.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // c89b6 imul eax,r9d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r9 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c89ba add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c89bc movsx edi,WORD PTR [rbx+r11*2+0x4]
            rdi = (sx::<16>(m.r16(rbx.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // c89c2 imul edi,r8d
            { let (r, f) = imul::<32>((rdi as u32 as u64), (r8 as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            // c89c6 add edi,eax
            { let (r, f) = add::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c89c8 mov ebx,edi
            rbx = ((rdi as u32 as u64)) as u32 as u64;
            // c89ca and ebx,0xffff0000
            { let r = (rbx as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c89d0 cmp ebx,0x3fb0000
            fl = sub::<32>((rbx as u32 as u64), 0x3fb0000u64).1;
            // c89d6 jg 0xc89ec
            if fl.g() { pc = 0xc89ec; continue; }
            pc = 0xc89d8;
        }
        0xc89d8 => {
            // block c89d8
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c89d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c89d8 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c89db cmp ebx,0x40000
            fl = sub::<32>((rbx as u32 as u64), 0x40000u64).1;
            // c89e1 mov ax,0x4
            rax = (rax & !0xffff) | ((0x4u64) as u16 as u64);
            // c89e5 cmovl di,ax
            if fl.l() { rdi = (rdi & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            // c89e9 mov si,di
            rsi = (rsi & !0xffff) | (((rdi as u16 as u64)) as u16 as u64);
            pc = 0xc89ec;
        }
        0xc89ec => {
            // block c89ec
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c89ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c89ec movsxd rax,r15d
            rax = sx::<32>((r15 as u32 as u64)) as u64;
            // c89ef mov WORD PTR [rdx+rax*2],si
            m.w16(rdx.wrapping_add(rax.wrapping_mul(2)), ((rsi as u16 as u64)) as u16);
            // c89f3 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // c89f6 add r15d,0x4
            { let (r, f) = add::<32>((r15 as u32 as u64), 0x4u64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c89fa mov eax,DWORD PTR [rbp-0x4c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64) as u32 as u64;
            // c89fd add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // c8a00 cmp r12,QWORD PTR [rbp-0x58]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64).1;
            // c8a04 jl 0xc88db
            if fl.l() { pc = 0xc88db; continue; }
            pc = 0xc8a0a;
        }
        0xc8a0a => {
            // block c8a0a
            #[cfg(feature = "trace")] eprintln!("f_000c86a9 pc c8a0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8a0a mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c8a0e call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c8a13 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c8a17 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c8a1c mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // c8a20 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // c8a24 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c8a25 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c8a27 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c8a29 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c8a2b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c8a2d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8a2e jmp 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe26e8 size=890
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e26e8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xe26e8;
    loop {
        match pc {
        0xe26e8 => {
            // block e26e8
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e26e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e26e8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e26e9 mov rbp,rsp
            rbp = rsp;
            // e26ec push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e26ee push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e26f0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e26f2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e26f4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e26f5 sub rsp,0xe8
            { let (r, f) = sub::<64>(rsp, 0xe8u64);
              rsp = r;
              fl = f; }
            // e26fc mov QWORD PTR [rbp-0x108],r9
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (r9) as u64);
            // e2703 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // e2706 mov DWORD PTR [rbp-0xfc],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff04), ((rcx as u32 as u64)) as u32);
            // e270c mov r13d,edx
            r13 = ((rdx as u32 as u64)) as u32 as u64;
            // e270f mov r14,rsi
            r14 = rsi;
            // e2712 mov rbx,rdi
            rbx = rdi;
            // e2715 call 0xe49c4
            { let r = { let t = m.env.code_va(0xe49c4); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e271a mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // e2720 test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e2722 je 0xe2942
            if fl.e() { pc = 0xe2942; continue; }
            pc = 0xe2728;
        }
        0xe2728 => {
            // block e2728
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2728 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2728 lea rdi,[rbx+0x34]
            rdi = rbx.wrapping_add(0x34);
            // e272c test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // e2730 jne 0xe2737
            if fl.ne() { pc = 0xe2737; continue; }
            pc = 0xe2732;
        }
        0xe2732 => {
            // block e2732
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2732 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2732 inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // e2735 jmp 0xe273b
            pc = 0xe273b; continue;
        }
        0xe2737 => {
            // block e2737
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2737 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2737 mov r14,QWORD PTR [r14+0x10]
            r14 = m.r64(r14.wrapping_add(0x10)) as u64;
            pc = 0xe273b;
        }
        0xe273b => {
            // block e273b
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e273b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e273b mov DWORD PTR [rsp],r15d
            m.w32(rsp, ((r15 as u32 as u64)) as u32);
            // e273f lea rsi,[rip+0x56d0f] # 0x139455
            rsi = m.env.data_va(318498);
            // e2746 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e2749 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // e274e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e2750 mov r8d,r13d
            r8 = ((r13 as u32 as u64)) as u32 as u64;
            // e2753 mov r9d,DWORD PTR [rbp-0xfc]
            r9 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff04)) as u64) as u32 as u64;
            // e275a mov rcx,r14
            rcx = r14;
            // e275d call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2762 mov esi,0xfa
            rsi = (0xfau64) as u32 as u64;
            // e2767 mov rdi,rbx
            rdi = rbx;
            // e276a call 0xe435c
            { let r = { let t = m.env.code_va(0xe435c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e276f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e2771 je 0xe2942
            if fl.e() { pc = 0xe2942; continue; }
            pc = 0xe2777;
        }
        0xe2777 => {
            // block e2777
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2777 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2777 xorps xmm0,xmm0
            x0 = 0;
            // e277a movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // e277e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // e2786 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e278a mov rdi,rbx
            rdi = rbx;
            // e278d call 0xe4a2e
            { let r = { let t = m.env.code_va(0xe4a2e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2792 mov r15,rbx
            r15 = rbx;
            // e2795 xorps xmm0,xmm0
            x0 = 0;
            // e2798 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // e279c mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // e27a4 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e27a8 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e27ac call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e27b1 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e27b5 lea rdx,[rbp-0x60]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e27b9 call 0xe171e
            { let r = { let t = m.env.code_va(0xe171e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e27be lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e27c2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e27c7 mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e27cb cmp QWORD PTR [rbp-0x58],rbx
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64, rbx).1;
            // e27cf je 0xe2925
            if fl.e() { pc = 0xe2925; continue; }
            pc = 0xe27d5;
        }
        0xe27d5 => {
            // block e27d5
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e27d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e27d5 xorps xmm0,xmm0
            x0 = 0;
            // e27d8 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // e27df mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // e27e7 lea rsi,[rip+0x56c25] # 0x139413
            rsi = m.env.data_va(318432);
            // e27ee lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e27f5 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // e27fa call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e27ff lea rdx,[rbp-0xa8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e2806 lea rcx,[rbp-0x90]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e280d mov rsi,rbx
            rsi = rbx;
            // e2810 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2815 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e2817 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e281e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2823 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e2825 je 0xe2919
            if fl.e() { pc = 0xe2919; continue; }
            pc = 0xe282b;
        }
        0xe282b => {
            // block e282b
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e282b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e282b lea rsi,[rip+0x56be8] # 0x13941a
            rsi = m.env.data_va(318439);
            // e2832 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e2839 call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e283e test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e2840 je 0xe2957
            if fl.e() { pc = 0xe2957; continue; }
            pc = 0xe2846;
        }
        0xe2846 => {
            // block e2846
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2846 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2846 lea rsi,[rip+0x56bd0] # 0x13941d
            rsi = m.env.data_va(318442);
            // e284d lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e2854 call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2859 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e285b jne 0xe2919
            if fl.ne() { pc = 0xe2919; continue; }
            pc = 0xe2861;
        }
        0xe2861 => {
            // block e2861
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2861 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2861 mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e2865 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e2869 sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // e286c sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e2870 movabs rcx,0xaaaaaaaaaaaaaaab
            rcx = 0xaaaa_aaaa_aaaa_aaabu64;
            // e287a imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // e287e cmp rcx,0x4
            fl = sub::<64>(rcx, 0x4u64).1;
            // e2882 jb 0xe2919
            if fl.b() { pc = 0xe2919; continue; }
            pc = 0xe2888;
        }
        0xe2888 => {
            // block e2888
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2888 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2888 lea rsi,[rip+0x56b93] # 0x139422
            rsi = m.env.data_va(318447);
            // e288f lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // e2896 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // e289b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e28a0 add rbx,0x30
            { let (r, f) = add::<64>(rbx, 0x30u64);
              rbx = r;
              fl = f; }
            // e28a4 lea rcx,[r15+0x10]
            rcx = r15.wrapping_add(0x10);
            // e28a8 lea rdx,[rbp-0xe0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // e28af mov rsi,rbx
            rsi = rbx;
            // e28b2 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e28b7 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // e28be call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e28c3 mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e28c7 lea rsi,[rip+0x56bb6] # 0x139484
            rsi = m.env.data_va(318545);
            // e28ce lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // e28d5 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // e28da call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e28df add rbx,0x48
            { let (r, f) = add::<64>(rbx, 0x48u64);
              rbx = r;
              fl = f; }
            // e28e3 lea rdx,[rbp-0xf8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // e28ea lea rcx,[rbp-0xc4]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff3c);
            // e28f1 mov rsi,rbx
            rsi = rbx;
            // e28f4 call 0xe2a62
            { let r = { let t = m.env.code_va(0xe2a62); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e28f9 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // e2900 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2905 mov DWORD PTR [r15+0x8],0x1a
            m.w32(r15.wrapping_add(0x8), (0x1au64) as u32);
            // e290d mov r12d,DWORD PTR [rbp-0xc4]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff3c)) as u64) as u32 as u64;
            // e2914 jmp 0xe29cc
            pc = 0xe29cc; continue;
        }
        0xe2919 => {
            // block e2919
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2919 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2919 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e2920 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe2925;
        }
        0xe2925 => {
            // block e2925
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2925 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2925 mov DWORD PTR [r15+0x8],0x17
            m.w32(r15.wrapping_add(0x8), (0x17u64) as u32);
            // e292d xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe2930;
        }
        0xe2930 => {
            // block e2930
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2930 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2930 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e2934 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2939 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e293d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe2942;
        }
        0xe2942 => {
            // block e2942
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2942 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2942 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // e2945 add rsp,0xe8
            { let (r, f) = add::<64>(rsp, 0xe8u64);
              rsp = r;
              fl = f; }
            // e294c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e294d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e294f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e2951 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2953 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2955 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2956 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe2957 => {
            // block e2957
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2957 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2957 mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e295b mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e295f sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // e2962 cmp rax,0x48
            fl = sub::<64>(rax, 0x48u64).1;
            // e2966 jne 0xe29c1
            if fl.ne() { pc = 0xe29c1; continue; }
            pc = 0xe2968;
        }
        0xe2968 => {
            // block e2968
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2968 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2968 lea rsi,[rip+0x56b11] # 0x139480
            rsi = m.env.data_va(318541);
            // e296f lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e2976 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // e297b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2980 add rbx,0x30
            { let (r, f) = add::<64>(rbx, 0x30u64);
              rbx = r;
              fl = f; }
            // e2984 lea rdx,[rbp-0xc0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e298b mov rsi,rbx
            rsi = rbx;
            // e298e mov rcx,QWORD PTR [rbp-0x108]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef8)) as u64;
            // e2995 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e299a mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e299d lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e29a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e29a9 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // e29af test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // e29b2 jne 0xe29cc
            if fl.ne() { pc = 0xe29cc; continue; }
            pc = 0xe29b4;
        }
        0xe29b4 => {
            // block e29b4
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e29b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e29b4 mov DWORD PTR [r15+0x8],0x19
            m.w32(r15.wrapping_add(0x8), (0x19u64) as u32);
            // e29bc xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e29bf jmp 0xe29cc
            pc = 0xe29cc; continue;
        }
        0xe29c1 => {
            // block e29c1
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e29c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e29c1 mov DWORD PTR [r15+0x8],0x18
            m.w32(r15.wrapping_add(0x8), (0x18u64) as u32);
            // e29c9 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe29cc;
        }
        0xe29cc => {
            // block e29cc
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e29cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e29cc lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e29d3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e29d8 jmp 0xe2930
            pc = 0xe2930; continue;
        }
        0xe29dd => {
            // block e29dd
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e29dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e29dd mov rbx,rax
            rbx = rax;
            // e29e0 jmp 0xe2a1e
            pc = 0xe2a1e; continue;
        }
        0xe29e2 => {
            // block e29e2
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e29e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e29e2 mov rbx,rax
            rbx = rax;
            // e29e5 jmp 0xe2a15
            pc = 0xe2a15; continue;
        }
        0xe29e7 => {
            // block e29e7
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e29e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e29e7 mov rbx,rax
            rbx = rax;
            // e29ea lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e29ee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e29f3 jmp 0xe2a15
            pc = 0xe2a15; continue;
        }
        0xe29f5 => {
            // block e29f5
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e29f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e29f5 mov rbx,rax
            rbx = rax;
            // e29f8 jmp 0xe2a09
            pc = 0xe2a09; continue;
        }
        0xe29fa => {
            // block e29fa
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e29fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e29fa mov rbx,rax
            rbx = rax;
            // e29fd lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e2a04 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe2a09;
        }
        0xe2a09 => {
            // block e2a09
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2a09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2a09 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e2a10 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe2a15;
        }
        0xe2a15 => {
            // block e2a15
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2a15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2a15 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e2a19 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe2a1e;
        }
        0xe2a1e => {
            // block e2a1e
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2a1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2a1e lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e2a22 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2a27 mov rdi,rbx
            rdi = rbx;
            // e2a2a call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2a2f mov rbx,rax
            rbx = rax;
            // e2a32 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e2a39 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2a3e jmp 0xe2a09
            pc = 0xe2a09; continue;
        }
        0xe2a40 => {
            // block e2a40
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2a40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2a40 mov rbx,rax
            rbx = rax;
            // e2a43 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // e2a4a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2a4f jmp 0xe2a09
            pc = 0xe2a09; continue;
        }
        0xe2a51 => {
            // block e2a51
            #[cfg(feature = "trace")] eprintln!("f_000e26e8 pc e2a51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2a51 mov rbx,rax
            rbx = rax;
            // e2a54 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // e2a5b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2a60 jmp 0xe2a09
            pc = 0xe2a09; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xab69c size=884
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ab69c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xab69c;
    loop {
        match pc {
        0xab69c => {
            // block ab69c
            #[cfg(feature = "trace")] eprintln!("f_000ab69c pc ab69c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab69c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ab69d mov rbp,rsp
            rbp = rsp;
            // ab6a0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ab6a2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ab6a4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ab6a6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ab6a8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ab6a9 sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // ab6b0 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // ab6b3 mov QWORD PTR [rbp-0x160],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rdi) as u64);
            // ab6ba lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // ab6c1 lea r12,[rbp-0x150]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // ab6c8 mov rax,QWORD PTR [rip+0x94b01] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // ab6cf lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // ab6d3 mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // ab6da add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // ab6de mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // ab6e5 mov rsi,r12
            rsi = r12;
            // ab6e8 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab6ed mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // ab6f5 mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // ab6fc mov r15,QWORD PTR [rip+0x94cbd] # 0x1403c0
            r15 = m.r64(m.env.data_va(347021)) as u64;
            // ab703 lea rbx,[r15+0x18]
            rbx = r15.wrapping_add(0x18);
            // ab707 mov QWORD PTR [rbp-0x158],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rbx) as u64);
            // ab70e add r15,0x40
            { let (r, f) = add::<64>(r15, 0x40u64);
              r15 = r;
              fl = f; }
            // ab712 mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // ab719 mov rdi,r12
            rdi = r12;
            // ab71c call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab721 mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // ab728 mov r13,QWORD PTR [rip+0x94c79] # 0x1403a8
            r13 = m.r64(m.env.data_va(346997)) as u64;
            // ab72f add r13,0x10
            { let (r, f) = add::<64>(r13, 0x10u64);
              r13 = r;
              fl = f; }
            // ab733 mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            // ab73a mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // ab745 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // ab750 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // ab75b mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // ab766 mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // ab770 xorps xmm0,xmm0
            x0 = 0;
            // ab773 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // ab777 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // ab77f lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // ab783 mov rdi,r12
            rdi = r12;
            // ab786 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab78b lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // ab78f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab794 lea rsi,[rip+0x8921e] # 0x1349b9
            rsi = m.env.data_va(299398);
            // ab79b lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // ab7a2 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // ab7a7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab7ac lea rsi,[rip+0x891dd] # 0x134990
            rsi = m.env.data_va(299357);
            // ab7b3 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // ab7b8 mov rdi,rax
            rdi = rax;
            // ab7bb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab7c0 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // ab7c3 and esi,0xf
            { let r = (rsi as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ab7c6 inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // ab7c8 mov rdi,rax
            rdi = rax;
            // ab7cb call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab7d0 mov r12,rax
            r12 = rax;
            // ab7d3 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // ab7d7 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // ab7db add rsi,r12
            { let (r, f) = add::<64>(rsi, r12);
              rsi = r;
              fl = f; }
            // ab7de lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // ab7e2 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab7e7 mov rsi,QWORD PTR [rip+0x949a2] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // ab7ee lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // ab7f2 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab7f7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // ab7fa mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // ab7fe mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // ab803 mov rdi,rax
            rdi = rax;
            // ab806 call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab808 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // ab80a lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // ab80e call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab813 movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // ab816 mov rdi,r12
            rdi = r12;
            // ab819 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab81e mov rdi,r12
            rdi = r12;
            // ab821 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab826 lea rsi,[rip+0x89193] # 0x1349c0
            rsi = m.env.data_va(299405);
            // ab82d mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // ab832 mov rdi,r12
            rdi = r12;
            // ab835 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab83a lea rsi,[rip+0x89188] # 0x1349c9
            rsi = m.env.data_va(299414);
            // ab841 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // ab846 mov rdi,rax
            rdi = rax;
            // ab849 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab84e mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // ab851 shr esi,0x4
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x4u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // ab854 and esi,0xf
            { let r = (rsi as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ab857 inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // ab859 mov rdi,rax
            rdi = rax;
            // ab85c call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab861 mov rbx,rax
            rbx = rax;
            // ab864 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ab867 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // ab86b add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // ab86e lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // ab872 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab877 mov rsi,QWORD PTR [rip+0x94912] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // ab87e lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // ab882 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab887 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // ab88a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // ab88e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // ab893 mov rdi,rax
            rdi = rax;
            // ab896 call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab898 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // ab89b lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // ab89f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab8a4 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // ab8a8 mov rdi,rbx
            rdi = rbx;
            // ab8ab call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab8b0 mov rdi,rbx
            rdi = rbx;
            // ab8b3 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab8b8 lea rsi,[rip+0x89124] # 0x1349e3
            rsi = m.env.data_va(299440);
            // ab8bf mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // ab8c4 mov rdi,rbx
            rdi = rbx;
            // ab8c7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab8cc lea rsi,[rip+0x890f6] # 0x1349c9
            rsi = m.env.data_va(299414);
            // ab8d3 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // ab8d8 mov rdi,rax
            rdi = rax;
            // ab8db call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab8e0 shr r14d,0x8
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x8u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // ab8e4 and r14d,0xf
            { let r = (r14 as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // ab8e8 inc r14d
            { let (r, f) = inc::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // ab8eb mov rdi,rax
            rdi = rax;
            // ab8ee mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // ab8f1 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab8f6 mov rdi,QWORD PTR [rbp-0x160]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // ab8fd lea rsi,[rbp-0x150]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // ab904 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab909 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // ab910 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // ab917 mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // ab91e mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            // ab925 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // ab92c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab931 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // ab938 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab93d mov rsi,QWORD PTR [rip+0x948c4] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // ab944 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // ab948 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // ab94f call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab954 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // ab95b call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab960 mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // ab967 add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // ab96e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ab96f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ab971 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ab973 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ab975 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ab977 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ab978 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xab979 => {
            // block ab979
            #[cfg(feature = "trace")] eprintln!("f_000ab69c pc ab979 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab979 mov rbx,rax
            rbx = rax;
            pc = 0xab97c;
        }
        0xab97c => {
            // block ab97c
            #[cfg(feature = "trace")] eprintln!("f_000ab69c pc ab97c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab97c mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // ab983 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // ab98a mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // ab991 mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            pc = 0xab998;
        }
        0xab998 => {
            // block ab998
            #[cfg(feature = "trace")] eprintln!("f_000ab69c pc ab998 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab998 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // ab99f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab9a4 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // ab9ab call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab9b0;
        }
        0xab9b0 => {
            // block ab9b0
            #[cfg(feature = "trace")] eprintln!("f_000ab69c pc ab9b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab9b0 mov rsi,QWORD PTR [rip+0x94851] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // ab9b7 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // ab9bb lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // ab9c2 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab9c7;
        }
        0xab9c7 => {
            // block ab9c7
            #[cfg(feature = "trace")] eprintln!("f_000ab69c pc ab9c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab9c7 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // ab9ce call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab9d3 mov rdi,rbx
            rdi = rbx;
            // ab9d6 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab9db mov rbx,rax
            rbx = rax;
            // ab9de lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // ab9e2 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab9e7 jmp 0xab97c
            pc = 0xab97c; continue;
        }
        0xab9e9 => {
            // block ab9e9
            #[cfg(feature = "trace")] eprintln!("f_000ab69c pc ab9e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab9e9 mov rbx,rax
            rbx = rax;
            // ab9ec lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // ab9f0 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab9f5 jmp 0xab97c
            pc = 0xab97c; continue;
        }
        0xab9f7 => {
            // block ab9f7
            #[cfg(feature = "trace")] eprintln!("f_000ab69c pc ab9f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab9f7 mov rbx,rax
            rbx = rax;
            // ab9fa jmp 0xab9c7
            pc = 0xab9c7; continue;
        }
        0xab9fc => {
            // block ab9fc
            #[cfg(feature = "trace")] eprintln!("f_000ab69c pc ab9fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab9fc mov rbx,rax
            rbx = rax;
            // ab9ff jmp 0xab9b0
            pc = 0xab9b0; continue;
        }
        0xaba01 => {
            // block aba01
            #[cfg(feature = "trace")] eprintln!("f_000ab69c pc aba01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aba01 mov rbx,rax
            rbx = rax;
            // aba04 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // aba08 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aba0d jmp 0xab998
            pc = 0xab998; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
