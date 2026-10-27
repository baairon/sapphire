//! Mechanically lifted functions (scripts/lift_run.py), module hb003. Each one is checked against the original
//! in the world harness (tests/world_hb003.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x7e0c4 size=10416
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0007e0c4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x7e0c4;
    loop {
        match pc {
        0x7e0c4 => {
            // block 7e0c4
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e0c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e0c4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7e0c5 mov rbp,rsp
            rbp = rsp;
            // 7e0c8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7e0ca push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7e0cc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7e0cd sub rsp,0x1378
            { let (r, f) = sub::<64>(rsp, 0x1378u64);
              rsp = r;
              fl = f; }
            // 7e0d4 mov r15,rdi
            r15 = rdi;
            // 7e0d7 lea rsi,[rip+0xbabea] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7e0de lea rbx,[rbp-0x30]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 7e0e2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7e0e4 mov rdi,rbx
            rdi = rbx;
            // 7e0e7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e0ec mov esi,0x20
            rsi = (0x20u64) as u32 as u64;
            // 7e0f1 mov rdi,r15
            rdi = r15;
            // 7e0f4 mov rdx,rbx
            rdx = rbx;
            // 7e0f7 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e0fc lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 7e100 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e105 mov DWORD PTR [rbp-0x40],0x20
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x20u64) as u32);
            // 7e10c mov QWORD PTR [rbp-0x38],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r15) as u64);
            // 7e110 lea r14,[r15+0x180]
            r14 = r15.wrapping_add(0x180);
            // 7e117 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 7e11b mov rdi,r14
            rdi = r14;
            // 7e11e call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e123 lea rsi,[rip+0xb58d3] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e12a lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 7e12e mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e133 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e138 movzx ebx,BYTE PTR [rbp-0x58]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // 7e13c test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7e13f jne 0x7e146
            if fl.ne() { pc = 0x7e146; continue; }
            pc = 0x7e141;
        }
        0x7e141 => {
            // block 7e141
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e141 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e141 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7e144 jmp 0x7e14a
            pc = 0x7e14a; continue;
        }
        0x7e146 => {
            // block 7e146
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e146 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e146 mov rbx,QWORD PTR [rbp-0x50]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0x7e14a;
        }
        0x7e14a => {
            // block 7e14a
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e14a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e14a lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 7e14e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e153 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7e156 je 0x7e1cc
            if fl.e() { pc = 0x7e1cc; continue; }
            pc = 0x7e158;
        }
        0x7e158 => {
            // block 7e158
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e158 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e158 lea rsi,[rip+0xb589e] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e15f lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 7e166 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e16b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e170 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 7e174 mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 7e178 mov rax,QWORD PTR [rbp-0x90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 7e17f mov rcx,QWORD PTR [rbp-0x88]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 7e186 mov QWORD PTR [rbp-0x70],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rcx) as u64);
            // 7e18a mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 7e18e xorps xmm0,xmm0
            x0 = 0;
            // 7e191 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // 7e198 mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // 7e1a0 mov DWORD PTR [rbp-0x60],0x20
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x20u64) as u32);
            // 7e1a7 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7e1ae lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 7e1b2 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e1b7 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 7e1bb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e1c0 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 7e1c7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7e1cc;
        }
        0x7e1cc => {
            // block 7e1cc
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e1cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e1cc xorps xmm0,xmm0
            x0 = 0;
            // 7e1cf movaps XMMWORD PTR [rbp-0xb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff50), x0);
            // 7e1d6 mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // 7e1e1 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 7e1e8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e1ed xorps xmm0,xmm0
            x0 = 0;
            // 7e1f0 movaps XMMWORD PTR [rbp-0xd0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff30), x0);
            // 7e1f7 mov QWORD PTR [rbp-0xc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u64);
            // 7e202 lea rdi,[rbp-0xd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // 7e209 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e20e lea rsi,[rip+0xbaab3] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7e215 lea rbx,[rbp-0xe8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 7e21c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7e21e mov rdi,rbx
            rdi = rbx;
            // 7e221 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e226 mov esi,0x64
            rsi = (0x64u64) as u32 as u64;
            // 7e22b mov rdi,r15
            rdi = r15;
            // 7e22e mov rdx,rbx
            rdx = rbx;
            // 7e231 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e236 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 7e23d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e242 mov DWORD PTR [rbp-0xf8],0x64
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x64u64) as u32);
            // 7e24c mov QWORD PTR [rbp-0xf0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (r15) as u64);
            // 7e253 lea rsi,[rbp-0xf8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 7e25a mov rdi,r14
            rdi = r14;
            // 7e25d call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e262 lea rsi,[rip+0xb5794] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e269 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 7e270 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e275 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e27a movzx ebx,BYTE PTR [rbp-0x110]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64) as u32 as u64;
            // 7e281 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7e284 jne 0x7e28b
            if fl.ne() { pc = 0x7e28b; continue; }
            pc = 0x7e286;
        }
        0x7e286 => {
            // block 7e286
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e286 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e286 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7e289 jmp 0x7e292
            pc = 0x7e292; continue;
        }
        0x7e28b => {
            // block 7e28b
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e28b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e28b mov rbx,QWORD PTR [rbp-0x108]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef8)) as u64;
            pc = 0x7e292;
        }
        0x7e292 => {
            // block 7e292
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e292 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e292 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 7e299 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e29e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7e2a1 je 0x7e333
            if fl.e() { pc = 0x7e333; continue; }
            pc = 0x7e2a7;
        }
        0x7e2a7 => {
            // block 7e2a7
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e2a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e2a7 lea rsi,[rip+0xb574f] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e2ae lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 7e2b5 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e2ba call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e2bf mov rax,QWORD PTR [rbp-0x140]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 7e2c6 mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 7e2cd mov rax,QWORD PTR [rbp-0x150]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 7e2d4 mov rcx,QWORD PTR [rbp-0x148]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 7e2db mov QWORD PTR [rbp-0x128],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rcx) as u64);
            // 7e2e2 mov QWORD PTR [rbp-0x130],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (rax) as u64);
            // 7e2e9 xorps xmm0,xmm0
            x0 = 0;
            // 7e2ec movaps XMMWORD PTR [rbp-0x150],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_feb0), x0);
            // 7e2f3 mov QWORD PTR [rbp-0x140],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (0x0u64) as u64);
            // 7e2fe mov DWORD PTR [rbp-0x118],0x64
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x64u64) as u32);
            // 7e308 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7e30f lea rsi,[rbp-0x130]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 7e316 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e31b lea rdi,[rbp-0x130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 7e322 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e327 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 7e32e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7e333;
        }
        0x7e333 => {
            // block 7e333
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e333 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e333 xorps xmm0,xmm0
            x0 = 0;
            // 7e336 movaps XMMWORD PTR [rbp-0x170],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe90), x0);
            // 7e33d mov QWORD PTR [rbp-0x160],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (0x0u64) as u64);
            // 7e348 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 7e34f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e354 xorps xmm0,xmm0
            x0 = 0;
            // 7e357 movaps XMMWORD PTR [rbp-0x190],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe70), x0);
            // 7e35e mov QWORD PTR [rbp-0x180],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (0x0u64) as u64);
            // 7e369 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 7e370 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e375 lea rsi,[rip+0xba94c] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7e37c lea rbx,[rbp-0x1a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 7e383 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7e385 mov rdi,rbx
            rdi = rbx;
            // 7e388 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e38d mov esi,0x21
            rsi = (0x21u64) as u32 as u64;
            // 7e392 mov rdi,r15
            rdi = r15;
            // 7e395 mov rdx,rbx
            rdx = rbx;
            // 7e398 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e39d lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 7e3a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e3a9 mov DWORD PTR [rbp-0x1b8],0x21
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe48), (0x21u64) as u32);
            // 7e3b3 mov QWORD PTR [rbp-0x1b0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (r15) as u64);
            // 7e3ba lea rsi,[rbp-0x1b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe48);
            // 7e3c1 mov rdi,r14
            rdi = r14;
            // 7e3c4 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e3c9 lea rsi,[rip+0xb562d] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e3d0 lea rdi,[rbp-0x1d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe30);
            // 7e3d7 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e3dc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e3e1 movzx ebx,BYTE PTR [rbp-0x1d0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64) as u32 as u64;
            // 7e3e8 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7e3eb jne 0x7e3f2
            if fl.ne() { pc = 0x7e3f2; continue; }
            pc = 0x7e3ed;
        }
        0x7e3ed => {
            // block 7e3ed
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e3ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e3ed shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7e3f0 jmp 0x7e3f9
            pc = 0x7e3f9; continue;
        }
        0x7e3f2 => {
            // block 7e3f2
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e3f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e3f2 mov rbx,QWORD PTR [rbp-0x1c8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64;
            pc = 0x7e3f9;
        }
        0x7e3f9 => {
            // block 7e3f9
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e3f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e3f9 lea rdi,[rbp-0x1d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe30);
            // 7e400 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e405 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7e408 je 0x7e49a
            if fl.e() { pc = 0x7e49a; continue; }
            pc = 0x7e40e;
        }
        0x7e40e => {
            // block 7e40e
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e40e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e40e lea rsi,[rip+0xb55e8] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e415 lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 7e41c mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e421 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e426 mov rax,QWORD PTR [rbp-0x200]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe00)) as u64;
            // 7e42d mov QWORD PTR [rbp-0x1e0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe20), (rax) as u64);
            // 7e434 mov rax,QWORD PTR [rbp-0x210]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdf0)) as u64;
            // 7e43b mov rcx,QWORD PTR [rbp-0x208]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdf8)) as u64;
            // 7e442 mov QWORD PTR [rbp-0x1e8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe18), (rcx) as u64);
            // 7e449 mov QWORD PTR [rbp-0x1f0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (rax) as u64);
            // 7e450 xorps xmm0,xmm0
            x0 = 0;
            // 7e453 movaps XMMWORD PTR [rbp-0x210],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fdf0), x0);
            // 7e45a mov QWORD PTR [rbp-0x200],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe00), (0x0u64) as u64);
            // 7e465 mov DWORD PTR [rbp-0x1d8],0x21
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe28), (0x21u64) as u32);
            // 7e46f lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7e476 lea rsi,[rbp-0x1f0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 7e47d call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e482 lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 7e489 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e48e lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 7e495 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7e49a;
        }
        0x7e49a => {
            // block 7e49a
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e49a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e49a xorps xmm0,xmm0
            x0 = 0;
            // 7e49d movaps XMMWORD PTR [rbp-0x230],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fdd0), x0);
            // 7e4a4 mov QWORD PTR [rbp-0x220],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fde0), (0x0u64) as u64);
            // 7e4af lea rdi,[rbp-0x230]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdd0);
            // 7e4b6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e4bb xorps xmm0,xmm0
            x0 = 0;
            // 7e4be movaps XMMWORD PTR [rbp-0x250],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fdb0), x0);
            // 7e4c5 mov QWORD PTR [rbp-0x240],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc0), (0x0u64) as u64);
            // 7e4d0 lea rdi,[rbp-0x250]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdb0);
            // 7e4d7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e4dc lea rsi,[rip+0xba7e5] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7e4e3 lea rbx,[rbp-0x268]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd98);
            // 7e4ea xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7e4ec mov rdi,rbx
            rdi = rbx;
            // 7e4ef call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e4f4 mov esi,0x22
            rsi = (0x22u64) as u32 as u64;
            // 7e4f9 mov rdi,r15
            rdi = r15;
            // 7e4fc mov rdx,rbx
            rdx = rbx;
            // 7e4ff call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e504 lea rdi,[rbp-0x268]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd98);
            // 7e50b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e510 mov DWORD PTR [rbp-0x278],0x22
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd88), (0x22u64) as u32);
            // 7e51a mov QWORD PTR [rbp-0x270],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd90), (r15) as u64);
            // 7e521 lea rsi,[rbp-0x278]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd88);
            // 7e528 mov rdi,r14
            rdi = r14;
            // 7e52b call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e530 lea rsi,[rip+0xb54c6] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e537 lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // 7e53e mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e543 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e548 movzx ebx,BYTE PTR [rbp-0x290]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd70)) as u64) as u32 as u64;
            // 7e54f test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7e552 jne 0x7e559
            if fl.ne() { pc = 0x7e559; continue; }
            pc = 0x7e554;
        }
        0x7e554 => {
            // block 7e554
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e554 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e554 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7e557 jmp 0x7e560
            pc = 0x7e560; continue;
        }
        0x7e559 => {
            // block 7e559
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e559 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e559 mov rbx,QWORD PTR [rbp-0x288]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd78)) as u64;
            pc = 0x7e560;
        }
        0x7e560 => {
            // block 7e560
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e560 lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // 7e567 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e56c test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7e56f je 0x7e601
            if fl.e() { pc = 0x7e601; continue; }
            pc = 0x7e575;
        }
        0x7e575 => {
            // block 7e575
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e575 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e575 lea rsi,[rip+0xb5481] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e57c lea rdi,[rbp-0x2d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd30);
            // 7e583 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e588 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e58d mov rax,QWORD PTR [rbp-0x2c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd40)) as u64;
            // 7e594 mov QWORD PTR [rbp-0x2a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd60), (rax) as u64);
            // 7e59b mov rax,QWORD PTR [rbp-0x2d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd30)) as u64;
            // 7e5a2 mov rcx,QWORD PTR [rbp-0x2c8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 7e5a9 mov QWORD PTR [rbp-0x2a8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (rcx) as u64);
            // 7e5b0 mov QWORD PTR [rbp-0x2b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd50), (rax) as u64);
            // 7e5b7 xorps xmm0,xmm0
            x0 = 0;
            // 7e5ba movaps XMMWORD PTR [rbp-0x2d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fd30), x0);
            // 7e5c1 mov QWORD PTR [rbp-0x2c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd40), (0x0u64) as u64);
            // 7e5cc mov DWORD PTR [rbp-0x298],0x22
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd68), (0x22u64) as u32);
            // 7e5d6 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7e5dd lea rsi,[rbp-0x2b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // 7e5e4 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e5e9 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // 7e5f0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e5f5 lea rdi,[rbp-0x2d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd30);
            // 7e5fc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7e601;
        }
        0x7e601 => {
            // block 7e601
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e601 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e601 xorps xmm0,xmm0
            x0 = 0;
            // 7e604 movaps XMMWORD PTR [rbp-0x2f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fd10), x0);
            // 7e60b mov QWORD PTR [rbp-0x2e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd20), (0x0u64) as u64);
            // 7e616 lea rdi,[rbp-0x2f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd10);
            // 7e61d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e622 xorps xmm0,xmm0
            x0 = 0;
            // 7e625 movaps XMMWORD PTR [rbp-0x310],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fcf0), x0);
            // 7e62c mov QWORD PTR [rbp-0x300],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd00), (0x0u64) as u64);
            // 7e637 lea rdi,[rbp-0x310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf0);
            // 7e63e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e643 lea rsi,[rip+0xba67e] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7e64a lea rbx,[rbp-0x328]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 7e651 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7e653 mov rdi,rbx
            rdi = rbx;
            // 7e656 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e65b mov esi,0x23
            rsi = (0x23u64) as u32 as u64;
            // 7e660 mov rdi,r15
            rdi = r15;
            // 7e663 mov rdx,rbx
            rdx = rbx;
            // 7e666 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e66b lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 7e672 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e677 mov DWORD PTR [rbp-0x338],0x23
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fcc8), (0x23u64) as u32);
            // 7e681 mov QWORD PTR [rbp-0x330],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0), (r15) as u64);
            // 7e688 lea rsi,[rbp-0x338]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fcc8);
            // 7e68f mov rdi,r14
            rdi = r14;
            // 7e692 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e697 lea rsi,[rip+0xb535f] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e69e lea rdi,[rbp-0x350]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb0);
            // 7e6a5 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e6aa call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e6af movzx ebx,BYTE PTR [rbp-0x350]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fcb0)) as u64) as u32 as u64;
            // 7e6b6 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7e6b9 jne 0x7e6c0
            if fl.ne() { pc = 0x7e6c0; continue; }
            pc = 0x7e6bb;
        }
        0x7e6bb => {
            // block 7e6bb
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e6bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e6bb shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7e6be jmp 0x7e6c7
            pc = 0x7e6c7; continue;
        }
        0x7e6c0 => {
            // block 7e6c0
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e6c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e6c0 mov rbx,QWORD PTR [rbp-0x348]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcb8)) as u64;
            pc = 0x7e6c7;
        }
        0x7e6c7 => {
            // block 7e6c7
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e6c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e6c7 lea rdi,[rbp-0x350]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb0);
            // 7e6ce call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e6d3 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7e6d6 je 0x7e768
            if fl.e() { pc = 0x7e768; continue; }
            pc = 0x7e6dc;
        }
        0x7e6dc => {
            // block 7e6dc
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e6dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e6dc lea rsi,[rip+0xb531a] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e6e3 lea rdi,[rbp-0x390]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc70);
            // 7e6ea mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e6ef call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e6f4 mov rax,QWORD PTR [rbp-0x380]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc80)) as u64;
            // 7e6fb mov QWORD PTR [rbp-0x360],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fca0), (rax) as u64);
            // 7e702 mov rax,QWORD PTR [rbp-0x390]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc70)) as u64;
            // 7e709 mov rcx,QWORD PTR [rbp-0x388]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc78)) as u64;
            // 7e710 mov QWORD PTR [rbp-0x368],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc98), (rcx) as u64);
            // 7e717 mov QWORD PTR [rbp-0x370],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc90), (rax) as u64);
            // 7e71e xorps xmm0,xmm0
            x0 = 0;
            // 7e721 movaps XMMWORD PTR [rbp-0x390],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fc70), x0);
            // 7e728 mov QWORD PTR [rbp-0x380],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc80), (0x0u64) as u64);
            // 7e733 mov DWORD PTR [rbp-0x358],0x23
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fca8), (0x23u64) as u32);
            // 7e73d lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7e744 lea rsi,[rbp-0x370]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fc90);
            // 7e74b call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e750 lea rdi,[rbp-0x370]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc90);
            // 7e757 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e75c lea rdi,[rbp-0x390]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc70);
            // 7e763 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7e768;
        }
        0x7e768 => {
            // block 7e768
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e768 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e768 xorps xmm0,xmm0
            x0 = 0;
            // 7e76b movaps XMMWORD PTR [rbp-0x3b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fc50), x0);
            // 7e772 mov QWORD PTR [rbp-0x3a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc60), (0x0u64) as u64);
            // 7e77d lea rdi,[rbp-0x3b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc50);
            // 7e784 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e789 xorps xmm0,xmm0
            x0 = 0;
            // 7e78c movaps XMMWORD PTR [rbp-0x3d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fc30), x0);
            // 7e793 mov QWORD PTR [rbp-0x3c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc40), (0x0u64) as u64);
            // 7e79e lea rdi,[rbp-0x3d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc30);
            // 7e7a5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e7aa lea rsi,[rip+0xba517] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7e7b1 lea rbx,[rbp-0x3e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // 7e7b8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7e7ba mov rdi,rbx
            rdi = rbx;
            // 7e7bd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e7c2 mov esi,0x65
            rsi = (0x65u64) as u32 as u64;
            // 7e7c7 mov rdi,r15
            rdi = r15;
            // 7e7ca mov rdx,rbx
            rdx = rbx;
            // 7e7cd call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e7d2 lea rdi,[rbp-0x3e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // 7e7d9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e7de mov DWORD PTR [rbp-0x3f8],0x65
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc08), (0x65u64) as u32);
            // 7e7e8 mov QWORD PTR [rbp-0x3f0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc10), (r15) as u64);
            // 7e7ef lea rsi,[rbp-0x3f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fc08);
            // 7e7f6 mov rdi,r14
            rdi = r14;
            // 7e7f9 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e7fe lea rsi,[rip+0xb51f8] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e805 lea rdi,[rbp-0x410]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbf0);
            // 7e80c mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e811 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e816 movzx ebx,BYTE PTR [rbp-0x410]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fbf0)) as u64) as u32 as u64;
            // 7e81d test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7e820 jne 0x7e827
            if fl.ne() { pc = 0x7e827; continue; }
            pc = 0x7e822;
        }
        0x7e822 => {
            // block 7e822
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e822 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e822 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7e825 jmp 0x7e82e
            pc = 0x7e82e; continue;
        }
        0x7e827 => {
            // block 7e827
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e827 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e827 mov rbx,QWORD PTR [rbp-0x408]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbf8)) as u64;
            pc = 0x7e82e;
        }
        0x7e82e => {
            // block 7e82e
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e82e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e82e lea rdi,[rbp-0x410]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbf0);
            // 7e835 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e83a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7e83d je 0x7e8cf
            if fl.e() { pc = 0x7e8cf; continue; }
            pc = 0x7e843;
        }
        0x7e843 => {
            // block 7e843
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e843 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e843 lea rsi,[rip+0xb51b3] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e84a lea rdi,[rbp-0x450]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb0);
            // 7e851 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e856 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e85b mov rax,QWORD PTR [rbp-0x440]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbc0)) as u64;
            // 7e862 mov QWORD PTR [rbp-0x420],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbe0), (rax) as u64);
            // 7e869 mov rax,QWORD PTR [rbp-0x450]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbb0)) as u64;
            // 7e870 mov rcx,QWORD PTR [rbp-0x448]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbb8)) as u64;
            // 7e877 mov QWORD PTR [rbp-0x428],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbd8), (rcx) as u64);
            // 7e87e mov QWORD PTR [rbp-0x430],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbd0), (rax) as u64);
            // 7e885 xorps xmm0,xmm0
            x0 = 0;
            // 7e888 movaps XMMWORD PTR [rbp-0x450],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fbb0), x0);
            // 7e88f mov QWORD PTR [rbp-0x440],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc0), (0x0u64) as u64);
            // 7e89a mov DWORD PTR [rbp-0x418],0x65
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fbe8), (0x65u64) as u32);
            // 7e8a4 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7e8ab lea rsi,[rbp-0x430]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fbd0);
            // 7e8b2 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e8b7 lea rdi,[rbp-0x430]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd0);
            // 7e8be call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e8c3 lea rdi,[rbp-0x450]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb0);
            // 7e8ca call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7e8cf;
        }
        0x7e8cf => {
            // block 7e8cf
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e8cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e8cf xorps xmm0,xmm0
            x0 = 0;
            // 7e8d2 movaps XMMWORD PTR [rbp-0x470],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fb90), x0);
            // 7e8d9 mov QWORD PTR [rbp-0x460],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fba0), (0x0u64) as u64);
            // 7e8e4 lea rdi,[rbp-0x470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // 7e8eb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e8f0 xorps xmm0,xmm0
            x0 = 0;
            // 7e8f3 movaps XMMWORD PTR [rbp-0x490],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fb70), x0);
            // 7e8fa mov QWORD PTR [rbp-0x480],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb80), (0x0u64) as u64);
            // 7e905 lea rdi,[rbp-0x490]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb70);
            // 7e90c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e911 lea rsi,[rip+0xba3b0] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7e918 lea rbx,[rbp-0x4a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb58);
            // 7e91f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7e921 mov rdi,rbx
            rdi = rbx;
            // 7e924 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e929 mov esi,0x24
            rsi = (0x24u64) as u32 as u64;
            // 7e92e mov rdi,r15
            rdi = r15;
            // 7e931 mov rdx,rbx
            rdx = rbx;
            // 7e934 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e939 lea rdi,[rbp-0x4a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb58);
            // 7e940 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e945 mov DWORD PTR [rbp-0x4b8],0x24
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb48), (0x24u64) as u32);
            // 7e94f mov QWORD PTR [rbp-0x4b0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb50), (r15) as u64);
            // 7e956 lea rsi,[rbp-0x4b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb48);
            // 7e95d mov rdi,r14
            rdi = r14;
            // 7e960 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e965 lea rsi,[rip+0xb5091] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e96c lea rdi,[rbp-0x4d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb30);
            // 7e973 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e978 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e97d movzx ebx,BYTE PTR [rbp-0x4d0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fb30)) as u64) as u32 as u64;
            // 7e984 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7e987 jne 0x7e98e
            if fl.ne() { pc = 0x7e98e; continue; }
            pc = 0x7e989;
        }
        0x7e989 => {
            // block 7e989
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e989 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e989 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7e98c jmp 0x7e995
            pc = 0x7e995; continue;
        }
        0x7e98e => {
            // block 7e98e
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e98e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e98e mov rbx,QWORD PTR [rbp-0x4c8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb38)) as u64;
            pc = 0x7e995;
        }
        0x7e995 => {
            // block 7e995
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e995 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e995 lea rdi,[rbp-0x4d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb30);
            // 7e99c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e9a1 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7e9a4 je 0x7ea36
            if fl.e() { pc = 0x7ea36; continue; }
            pc = 0x7e9aa;
        }
        0x7e9aa => {
            // block 7e9aa
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7e9aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e9aa lea rsi,[rip+0xb504c] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7e9b1 lea rdi,[rbp-0x510]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_faf0);
            // 7e9b8 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7e9bd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e9c2 mov rax,QWORD PTR [rbp-0x500]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb00)) as u64;
            // 7e9c9 mov QWORD PTR [rbp-0x4e0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb20), (rax) as u64);
            // 7e9d0 mov rax,QWORD PTR [rbp-0x510]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_faf0)) as u64;
            // 7e9d7 mov rcx,QWORD PTR [rbp-0x508]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_faf8)) as u64;
            // 7e9de mov QWORD PTR [rbp-0x4e8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb18), (rcx) as u64);
            // 7e9e5 mov QWORD PTR [rbp-0x4f0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb10), (rax) as u64);
            // 7e9ec xorps xmm0,xmm0
            x0 = 0;
            // 7e9ef movaps XMMWORD PTR [rbp-0x510],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_faf0), x0);
            // 7e9f6 mov QWORD PTR [rbp-0x500],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb00), (0x0u64) as u64);
            // 7ea01 mov DWORD PTR [rbp-0x4d8],0x24
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb28), (0x24u64) as u32);
            // 7ea0b lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7ea12 lea rsi,[rbp-0x4f0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb10);
            // 7ea19 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ea1e lea rdi,[rbp-0x4f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb10);
            // 7ea25 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ea2a lea rdi,[rbp-0x510]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_faf0);
            // 7ea31 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7ea36;
        }
        0x7ea36 => {
            // block 7ea36
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7ea36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ea36 xorps xmm0,xmm0
            x0 = 0;
            // 7ea39 movaps XMMWORD PTR [rbp-0x530],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fad0), x0);
            // 7ea40 mov QWORD PTR [rbp-0x520],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fae0), (0x0u64) as u64);
            // 7ea4b lea rdi,[rbp-0x530]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fad0);
            // 7ea52 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ea57 xorps xmm0,xmm0
            x0 = 0;
            // 7ea5a movaps XMMWORD PTR [rbp-0x550],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fab0), x0);
            // 7ea61 mov QWORD PTR [rbp-0x540],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fac0), (0x0u64) as u64);
            // 7ea6c lea rdi,[rbp-0x550]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fab0);
            // 7ea73 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ea78 lea rsi,[rip+0xba249] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7ea7f lea rbx,[rbp-0x568]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fa98);
            // 7ea86 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7ea88 mov rdi,rbx
            rdi = rbx;
            // 7ea8b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ea90 mov esi,0x66
            rsi = (0x66u64) as u32 as u64;
            // 7ea95 mov rdi,r15
            rdi = r15;
            // 7ea98 mov rdx,rbx
            rdx = rbx;
            // 7ea9b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7eaa0 lea rdi,[rbp-0x568]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa98);
            // 7eaa7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7eaac mov DWORD PTR [rbp-0x578],0x66
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa88), (0x66u64) as u32);
            // 7eab6 mov QWORD PTR [rbp-0x570],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa90), (r15) as u64);
            // 7eabd lea rsi,[rbp-0x578]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa88);
            // 7eac4 mov rdi,r14
            rdi = r14;
            // 7eac7 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7eacc lea rsi,[rip+0xb4f2a] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7ead3 lea rdi,[rbp-0x590]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa70);
            // 7eada mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7eadf call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7eae4 movzx ebx,BYTE PTR [rbp-0x590]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fa70)) as u64) as u32 as u64;
            // 7eaeb test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7eaee jne 0x7eaf5
            if fl.ne() { pc = 0x7eaf5; continue; }
            pc = 0x7eaf0;
        }
        0x7eaf0 => {
            // block 7eaf0
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7eaf0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7eaf0 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7eaf3 jmp 0x7eafc
            pc = 0x7eafc; continue;
        }
        0x7eaf5 => {
            // block 7eaf5
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7eaf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7eaf5 mov rbx,QWORD PTR [rbp-0x588]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa78)) as u64;
            pc = 0x7eafc;
        }
        0x7eafc => {
            // block 7eafc
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7eafc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7eafc lea rdi,[rbp-0x590]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa70);
            // 7eb03 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7eb08 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7eb0b je 0x7eb9d
            if fl.e() { pc = 0x7eb9d; continue; }
            pc = 0x7eb11;
        }
        0x7eb11 => {
            // block 7eb11
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7eb11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7eb11 lea rsi,[rip+0xb4ee5] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7eb18 lea rdi,[rbp-0x5d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa30);
            // 7eb1f mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7eb24 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7eb29 mov rax,QWORD PTR [rbp-0x5c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa40)) as u64;
            // 7eb30 mov QWORD PTR [rbp-0x5a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa60), (rax) as u64);
            // 7eb37 mov rax,QWORD PTR [rbp-0x5d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa30)) as u64;
            // 7eb3e mov rcx,QWORD PTR [rbp-0x5c8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa38)) as u64;
            // 7eb45 mov QWORD PTR [rbp-0x5a8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa58), (rcx) as u64);
            // 7eb4c mov QWORD PTR [rbp-0x5b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa50), (rax) as u64);
            // 7eb53 xorps xmm0,xmm0
            x0 = 0;
            // 7eb56 movaps XMMWORD PTR [rbp-0x5d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fa30), x0);
            // 7eb5d mov QWORD PTR [rbp-0x5c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa40), (0x0u64) as u64);
            // 7eb68 mov DWORD PTR [rbp-0x598],0x66
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa68), (0x66u64) as u32);
            // 7eb72 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7eb79 lea rsi,[rbp-0x5b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa50);
            // 7eb80 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7eb85 lea rdi,[rbp-0x5b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa50);
            // 7eb8c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7eb91 lea rdi,[rbp-0x5d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa30);
            // 7eb98 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7eb9d;
        }
        0x7eb9d => {
            // block 7eb9d
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7eb9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7eb9d xorps xmm0,xmm0
            x0 = 0;
            // 7eba0 movaps XMMWORD PTR [rbp-0x5f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fa10), x0);
            // 7eba7 mov QWORD PTR [rbp-0x5e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa20), (0x0u64) as u64);
            // 7ebb2 lea rdi,[rbp-0x5f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa10);
            // 7ebb9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ebbe xorps xmm0,xmm0
            x0 = 0;
            // 7ebc1 movaps XMMWORD PTR [rbp-0x610],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f9f0), x0);
            // 7ebc8 mov QWORD PTR [rbp-0x600],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa00), (0x0u64) as u64);
            // 7ebd3 lea rdi,[rbp-0x610]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9f0);
            // 7ebda call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ebdf lea rsi,[rip+0xba0e2] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7ebe6 lea rbx,[rbp-0x628]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f9d8);
            // 7ebed xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7ebef mov rdi,rbx
            rdi = rbx;
            // 7ebf2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ebf7 mov esi,0x25
            rsi = (0x25u64) as u32 as u64;
            // 7ebfc mov rdi,r15
            rdi = r15;
            // 7ebff mov rdx,rbx
            rdx = rbx;
            // 7ec02 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ec07 lea rdi,[rbp-0x628]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9d8);
            // 7ec0e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ec13 mov DWORD PTR [rbp-0x638],0x25
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9c8), (0x25u64) as u32);
            // 7ec1d mov QWORD PTR [rbp-0x630],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f9d0), (r15) as u64);
            // 7ec24 lea rsi,[rbp-0x638]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9c8);
            // 7ec2b mov rdi,r14
            rdi = r14;
            // 7ec2e call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ec33 lea rsi,[rip+0xb4dc3] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7ec3a lea rdi,[rbp-0x650]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9b0);
            // 7ec41 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7ec46 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ec4b movzx ebx,BYTE PTR [rbp-0x650]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f9b0)) as u64) as u32 as u64;
            // 7ec52 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7ec55 jne 0x7ec5c
            if fl.ne() { pc = 0x7ec5c; continue; }
            pc = 0x7ec57;
        }
        0x7ec57 => {
            // block 7ec57
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7ec57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ec57 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7ec5a jmp 0x7ec63
            pc = 0x7ec63; continue;
        }
        0x7ec5c => {
            // block 7ec5c
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7ec5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ec5c mov rbx,QWORD PTR [rbp-0x648]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f9b8)) as u64;
            pc = 0x7ec63;
        }
        0x7ec63 => {
            // block 7ec63
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7ec63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ec63 lea rdi,[rbp-0x650]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9b0);
            // 7ec6a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ec6f test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7ec72 je 0x7ed04
            if fl.e() { pc = 0x7ed04; continue; }
            pc = 0x7ec78;
        }
        0x7ec78 => {
            // block 7ec78
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7ec78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ec78 lea rsi,[rip+0xb4d7e] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7ec7f lea rdi,[rbp-0x690]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f970);
            // 7ec86 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7ec8b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ec90 mov rax,QWORD PTR [rbp-0x680]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f980)) as u64;
            // 7ec97 mov QWORD PTR [rbp-0x660],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f9a0), (rax) as u64);
            // 7ec9e mov rax,QWORD PTR [rbp-0x690]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f970)) as u64;
            // 7eca5 mov rcx,QWORD PTR [rbp-0x688]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f978)) as u64;
            // 7ecac mov QWORD PTR [rbp-0x668],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f998), (rcx) as u64);
            // 7ecb3 mov QWORD PTR [rbp-0x670],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f990), (rax) as u64);
            // 7ecba xorps xmm0,xmm0
            x0 = 0;
            // 7ecbd movaps XMMWORD PTR [rbp-0x690],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f970), x0);
            // 7ecc4 mov QWORD PTR [rbp-0x680],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f980), (0x0u64) as u64);
            // 7eccf mov DWORD PTR [rbp-0x658],0x25
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9a8), (0x25u64) as u32);
            // 7ecd9 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7ece0 lea rsi,[rbp-0x670]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f990);
            // 7ece7 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ecec lea rdi,[rbp-0x670]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f990);
            // 7ecf3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ecf8 lea rdi,[rbp-0x690]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f970);
            // 7ecff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7ed04;
        }
        0x7ed04 => {
            // block 7ed04
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7ed04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ed04 xorps xmm0,xmm0
            x0 = 0;
            // 7ed07 movaps XMMWORD PTR [rbp-0x6b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f950), x0);
            // 7ed0e mov QWORD PTR [rbp-0x6a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f960), (0x0u64) as u64);
            // 7ed19 lea rdi,[rbp-0x6b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f950);
            // 7ed20 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ed25 xorps xmm0,xmm0
            x0 = 0;
            // 7ed28 movaps XMMWORD PTR [rbp-0x6d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f930), x0);
            // 7ed2f mov QWORD PTR [rbp-0x6c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f940), (0x0u64) as u64);
            // 7ed3a lea rdi,[rbp-0x6d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f930);
            // 7ed41 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ed46 lea rsi,[rip+0xb9f7b] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7ed4d lea rbx,[rbp-0x6e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f918);
            // 7ed54 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7ed56 mov rdi,rbx
            rdi = rbx;
            // 7ed59 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ed5e mov esi,0x26
            rsi = (0x26u64) as u32 as u64;
            // 7ed63 mov rdi,r15
            rdi = r15;
            // 7ed66 mov rdx,rbx
            rdx = rbx;
            // 7ed69 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ed6e lea rdi,[rbp-0x6e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f918);
            // 7ed75 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ed7a mov DWORD PTR [rbp-0x6f8],0x26
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f908), (0x26u64) as u32);
            // 7ed84 mov QWORD PTR [rbp-0x6f0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f910), (r15) as u64);
            // 7ed8b lea rsi,[rbp-0x6f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f908);
            // 7ed92 mov rdi,r14
            rdi = r14;
            // 7ed95 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ed9a lea rsi,[rip+0xb4c5c] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7eda1 lea rdi,[rbp-0x710]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8f0);
            // 7eda8 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7edad call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7edb2 movzx ebx,BYTE PTR [rbp-0x710]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f8f0)) as u64) as u32 as u64;
            // 7edb9 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7edbc jne 0x7edc3
            if fl.ne() { pc = 0x7edc3; continue; }
            pc = 0x7edbe;
        }
        0x7edbe => {
            // block 7edbe
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7edbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7edbe shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7edc1 jmp 0x7edca
            pc = 0x7edca; continue;
        }
        0x7edc3 => {
            // block 7edc3
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7edc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7edc3 mov rbx,QWORD PTR [rbp-0x708]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8f8)) as u64;
            pc = 0x7edca;
        }
        0x7edca => {
            // block 7edca
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7edca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7edca lea rdi,[rbp-0x710]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8f0);
            // 7edd1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7edd6 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7edd9 je 0x7ee6b
            if fl.e() { pc = 0x7ee6b; continue; }
            pc = 0x7eddf;
        }
        0x7eddf => {
            // block 7eddf
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7eddf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7eddf lea rsi,[rip+0xb4c17] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7ede6 lea rdi,[rbp-0x750]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8b0);
            // 7eded mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7edf2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7edf7 mov rax,QWORD PTR [rbp-0x740]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8c0)) as u64;
            // 7edfe mov QWORD PTR [rbp-0x720],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8e0), (rax) as u64);
            // 7ee05 mov rax,QWORD PTR [rbp-0x750]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b0)) as u64;
            // 7ee0c mov rcx,QWORD PTR [rbp-0x748]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 7ee13 mov QWORD PTR [rbp-0x728],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8d8), (rcx) as u64);
            // 7ee1a mov QWORD PTR [rbp-0x730],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8d0), (rax) as u64);
            // 7ee21 xorps xmm0,xmm0
            x0 = 0;
            // 7ee24 movaps XMMWORD PTR [rbp-0x750],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f8b0), x0);
            // 7ee2b mov QWORD PTR [rbp-0x740],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8c0), (0x0u64) as u64);
            // 7ee36 mov DWORD PTR [rbp-0x718],0x26
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f8e8), (0x26u64) as u32);
            // 7ee40 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7ee47 lea rsi,[rbp-0x730]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f8d0);
            // 7ee4e call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ee53 lea rdi,[rbp-0x730]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8d0);
            // 7ee5a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ee5f lea rdi,[rbp-0x750]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8b0);
            // 7ee66 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7ee6b;
        }
        0x7ee6b => {
            // block 7ee6b
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7ee6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ee6b xorps xmm0,xmm0
            x0 = 0;
            // 7ee6e movaps XMMWORD PTR [rbp-0x770],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f890), x0);
            // 7ee75 mov QWORD PTR [rbp-0x760],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8a0), (0x0u64) as u64);
            // 7ee80 lea rdi,[rbp-0x770]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f890);
            // 7ee87 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ee8c xorps xmm0,xmm0
            x0 = 0;
            // 7ee8f movaps XMMWORD PTR [rbp-0x790],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f870), x0);
            // 7ee96 mov QWORD PTR [rbp-0x780],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f880), (0x0u64) as u64);
            // 7eea1 lea rdi,[rbp-0x790]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f870);
            // 7eea8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7eead lea rsi,[rip+0xb9e14] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7eeb4 lea rbx,[rbp-0x7a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f858);
            // 7eebb xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7eebd mov rdi,rbx
            rdi = rbx;
            // 7eec0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7eec5 mov esi,0x27
            rsi = (0x27u64) as u32 as u64;
            // 7eeca mov rdi,r15
            rdi = r15;
            // 7eecd mov rdx,rbx
            rdx = rbx;
            // 7eed0 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7eed5 lea rdi,[rbp-0x7a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f858);
            // 7eedc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7eee1 mov DWORD PTR [rbp-0x7b8],0x27
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f848), (0x27u64) as u32);
            // 7eeeb mov QWORD PTR [rbp-0x7b0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f850), (r15) as u64);
            // 7eef2 lea rsi,[rbp-0x7b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f848);
            // 7eef9 mov rdi,r14
            rdi = r14;
            // 7eefc call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ef01 lea rsi,[rip+0xb4af5] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7ef08 lea rdi,[rbp-0x7d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f830);
            // 7ef0f mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7ef14 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ef19 movzx ebx,BYTE PTR [rbp-0x7d0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f830)) as u64) as u32 as u64;
            // 7ef20 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7ef23 jne 0x7ef2a
            if fl.ne() { pc = 0x7ef2a; continue; }
            pc = 0x7ef25;
        }
        0x7ef25 => {
            // block 7ef25
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7ef25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ef25 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7ef28 jmp 0x7ef31
            pc = 0x7ef31; continue;
        }
        0x7ef2a => {
            // block 7ef2a
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7ef2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ef2a mov rbx,QWORD PTR [rbp-0x7c8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f838)) as u64;
            pc = 0x7ef31;
        }
        0x7ef31 => {
            // block 7ef31
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7ef31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ef31 lea rdi,[rbp-0x7d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f830);
            // 7ef38 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ef3d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7ef40 je 0x7efd2
            if fl.e() { pc = 0x7efd2; continue; }
            pc = 0x7ef46;
        }
        0x7ef46 => {
            // block 7ef46
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7ef46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ef46 lea rsi,[rip+0xb4ab0] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7ef4d lea rdi,[rbp-0x810]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7f0);
            // 7ef54 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7ef59 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ef5e mov rax,QWORD PTR [rbp-0x800]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f800)) as u64;
            // 7ef65 mov QWORD PTR [rbp-0x7e0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f820), (rax) as u64);
            // 7ef6c mov rax,QWORD PTR [rbp-0x810]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f7f0)) as u64;
            // 7ef73 mov rcx,QWORD PTR [rbp-0x808]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f7f8)) as u64;
            // 7ef7a mov QWORD PTR [rbp-0x7e8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f818), (rcx) as u64);
            // 7ef81 mov QWORD PTR [rbp-0x7f0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f810), (rax) as u64);
            // 7ef88 xorps xmm0,xmm0
            x0 = 0;
            // 7ef8b movaps XMMWORD PTR [rbp-0x810],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f7f0), x0);
            // 7ef92 mov QWORD PTR [rbp-0x800],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f800), (0x0u64) as u64);
            // 7ef9d mov DWORD PTR [rbp-0x7d8],0x27
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f828), (0x27u64) as u32);
            // 7efa7 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7efae lea rsi,[rbp-0x7f0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f810);
            // 7efb5 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7efba lea rdi,[rbp-0x7f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f810);
            // 7efc1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7efc6 lea rdi,[rbp-0x810]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7f0);
            // 7efcd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7efd2;
        }
        0x7efd2 => {
            // block 7efd2
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7efd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7efd2 xorps xmm0,xmm0
            x0 = 0;
            // 7efd5 movaps XMMWORD PTR [rbp-0x830],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f7d0), x0);
            // 7efdc mov QWORD PTR [rbp-0x820],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f7e0), (0x0u64) as u64);
            // 7efe7 lea rdi,[rbp-0x830]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7d0);
            // 7efee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7eff3 xorps xmm0,xmm0
            x0 = 0;
            // 7eff6 movaps XMMWORD PTR [rbp-0x850],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f7b0), x0);
            // 7effd mov QWORD PTR [rbp-0x840],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f7c0), (0x0u64) as u64);
            // 7f008 lea rdi,[rbp-0x850]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7b0);
            // 7f00f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f014 lea rsi,[rip+0xb9cad] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7f01b lea rbx,[rbp-0x868]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f798);
            // 7f022 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7f024 mov rdi,rbx
            rdi = rbx;
            // 7f027 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f02c mov esi,0x67
            rsi = (0x67u64) as u32 as u64;
            // 7f031 mov rdi,r15
            rdi = r15;
            // 7f034 mov rdx,rbx
            rdx = rbx;
            // 7f037 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f03c lea rdi,[rbp-0x868]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f798);
            // 7f043 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f048 mov DWORD PTR [rbp-0x878],0x67
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f788), (0x67u64) as u32);
            // 7f052 mov QWORD PTR [rbp-0x870],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f790), (r15) as u64);
            // 7f059 lea rsi,[rbp-0x878]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f788);
            // 7f060 mov rdi,r14
            rdi = r14;
            // 7f063 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f068 lea rsi,[rip+0xb498e] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f06f lea rdi,[rbp-0x890]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f770);
            // 7f076 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f07b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f080 movzx ebx,BYTE PTR [rbp-0x890]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f770)) as u64) as u32 as u64;
            // 7f087 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7f08a jne 0x7f091
            if fl.ne() { pc = 0x7f091; continue; }
            pc = 0x7f08c;
        }
        0x7f08c => {
            // block 7f08c
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f08c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f08c shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7f08f jmp 0x7f098
            pc = 0x7f098; continue;
        }
        0x7f091 => {
            // block 7f091
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f091 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f091 mov rbx,QWORD PTR [rbp-0x888]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f778)) as u64;
            pc = 0x7f098;
        }
        0x7f098 => {
            // block 7f098
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f098 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f098 lea rdi,[rbp-0x890]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f770);
            // 7f09f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f0a4 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7f0a7 je 0x7f139
            if fl.e() { pc = 0x7f139; continue; }
            pc = 0x7f0ad;
        }
        0x7f0ad => {
            // block 7f0ad
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f0ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f0ad lea rsi,[rip+0xb4949] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f0b4 lea rdi,[rbp-0x8d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f730);
            // 7f0bb mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f0c0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f0c5 mov rax,QWORD PTR [rbp-0x8c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f740)) as u64;
            // 7f0cc mov QWORD PTR [rbp-0x8a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f760), (rax) as u64);
            // 7f0d3 mov rax,QWORD PTR [rbp-0x8d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f730)) as u64;
            // 7f0da mov rcx,QWORD PTR [rbp-0x8c8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f738)) as u64;
            // 7f0e1 mov QWORD PTR [rbp-0x8a8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f758), (rcx) as u64);
            // 7f0e8 mov QWORD PTR [rbp-0x8b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f750), (rax) as u64);
            // 7f0ef xorps xmm0,xmm0
            x0 = 0;
            // 7f0f2 movaps XMMWORD PTR [rbp-0x8d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f730), x0);
            // 7f0f9 mov QWORD PTR [rbp-0x8c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f740), (0x0u64) as u64);
            // 7f104 mov DWORD PTR [rbp-0x898],0x67
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f768), (0x67u64) as u32);
            // 7f10e lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7f115 lea rsi,[rbp-0x8b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f750);
            // 7f11c call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f121 lea rdi,[rbp-0x8b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f750);
            // 7f128 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f12d lea rdi,[rbp-0x8d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f730);
            // 7f134 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7f139;
        }
        0x7f139 => {
            // block 7f139
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f139 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f139 xorps xmm0,xmm0
            x0 = 0;
            // 7f13c movaps XMMWORD PTR [rbp-0x8f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f710), x0);
            // 7f143 mov QWORD PTR [rbp-0x8e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f720), (0x0u64) as u64);
            // 7f14e lea rdi,[rbp-0x8f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f710);
            // 7f155 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f15a xorps xmm0,xmm0
            x0 = 0;
            // 7f15d movaps XMMWORD PTR [rbp-0x910],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f6f0), x0);
            // 7f164 mov QWORD PTR [rbp-0x900],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f700), (0x0u64) as u64);
            // 7f16f lea rdi,[rbp-0x910]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6f0);
            // 7f176 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f17b lea rsi,[rip+0xb9b46] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7f182 lea rbx,[rbp-0x928]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f6d8);
            // 7f189 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7f18b mov rdi,rbx
            rdi = rbx;
            // 7f18e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f193 mov esi,0x28
            rsi = (0x28u64) as u32 as u64;
            // 7f198 mov rdi,r15
            rdi = r15;
            // 7f19b mov rdx,rbx
            rdx = rbx;
            // 7f19e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f1a3 lea rdi,[rbp-0x928]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6d8);
            // 7f1aa call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f1af mov DWORD PTR [rbp-0x938],0x28
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f6c8), (0x28u64) as u32);
            // 7f1b9 mov QWORD PTR [rbp-0x930],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f6d0), (r15) as u64);
            // 7f1c0 lea rsi,[rbp-0x938]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f6c8);
            // 7f1c7 mov rdi,r14
            rdi = r14;
            // 7f1ca call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f1cf lea rsi,[rip+0xb4827] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f1d6 lea rdi,[rbp-0x950]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6b0);
            // 7f1dd mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f1e2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f1e7 movzx ebx,BYTE PTR [rbp-0x950]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f6b0)) as u64) as u32 as u64;
            // 7f1ee test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7f1f1 jne 0x7f1f8
            if fl.ne() { pc = 0x7f1f8; continue; }
            pc = 0x7f1f3;
        }
        0x7f1f3 => {
            // block 7f1f3
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f1f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f1f3 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7f1f6 jmp 0x7f1ff
            pc = 0x7f1ff; continue;
        }
        0x7f1f8 => {
            // block 7f1f8
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f1f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f1f8 mov rbx,QWORD PTR [rbp-0x948]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f6b8)) as u64;
            pc = 0x7f1ff;
        }
        0x7f1ff => {
            // block 7f1ff
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f1ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f1ff lea rdi,[rbp-0x950]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6b0);
            // 7f206 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f20b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7f20e je 0x7f2a0
            if fl.e() { pc = 0x7f2a0; continue; }
            pc = 0x7f214;
        }
        0x7f214 => {
            // block 7f214
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f214 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f214 lea rsi,[rip+0xb47e2] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f21b lea rdi,[rbp-0x990]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f670);
            // 7f222 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f227 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f22c mov rax,QWORD PTR [rbp-0x980]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f680)) as u64;
            // 7f233 mov QWORD PTR [rbp-0x960],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f6a0), (rax) as u64);
            // 7f23a mov rax,QWORD PTR [rbp-0x990]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f670)) as u64;
            // 7f241 mov rcx,QWORD PTR [rbp-0x988]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f678)) as u64;
            // 7f248 mov QWORD PTR [rbp-0x968],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f698), (rcx) as u64);
            // 7f24f mov QWORD PTR [rbp-0x970],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f690), (rax) as u64);
            // 7f256 xorps xmm0,xmm0
            x0 = 0;
            // 7f259 movaps XMMWORD PTR [rbp-0x990],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f670), x0);
            // 7f260 mov QWORD PTR [rbp-0x980],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f680), (0x0u64) as u64);
            // 7f26b mov DWORD PTR [rbp-0x958],0x28
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f6a8), (0x28u64) as u32);
            // 7f275 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7f27c lea rsi,[rbp-0x970]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f690);
            // 7f283 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f288 lea rdi,[rbp-0x970]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f690);
            // 7f28f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f294 lea rdi,[rbp-0x990]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f670);
            // 7f29b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7f2a0;
        }
        0x7f2a0 => {
            // block 7f2a0
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f2a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f2a0 xorps xmm0,xmm0
            x0 = 0;
            // 7f2a3 movaps XMMWORD PTR [rbp-0x9b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f650), x0);
            // 7f2aa mov QWORD PTR [rbp-0x9a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f660), (0x0u64) as u64);
            // 7f2b5 lea rdi,[rbp-0x9b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f650);
            // 7f2bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f2c1 xorps xmm0,xmm0
            x0 = 0;
            // 7f2c4 movaps XMMWORD PTR [rbp-0x9d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f630), x0);
            // 7f2cb mov QWORD PTR [rbp-0x9c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f640), (0x0u64) as u64);
            // 7f2d6 lea rdi,[rbp-0x9d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f630);
            // 7f2dd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f2e2 lea rsi,[rip+0xb99df] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7f2e9 lea rbx,[rbp-0x9e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f618);
            // 7f2f0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7f2f2 mov rdi,rbx
            rdi = rbx;
            // 7f2f5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f2fa mov esi,0x68
            rsi = (0x68u64) as u32 as u64;
            // 7f2ff mov rdi,r15
            rdi = r15;
            // 7f302 mov rdx,rbx
            rdx = rbx;
            // 7f305 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f30a lea rdi,[rbp-0x9e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f618);
            // 7f311 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f316 mov DWORD PTR [rbp-0x9f8],0x68
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f608), (0x68u64) as u32);
            // 7f320 mov QWORD PTR [rbp-0x9f0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f610), (r15) as u64);
            // 7f327 lea rsi,[rbp-0x9f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f608);
            // 7f32e mov rdi,r14
            rdi = r14;
            // 7f331 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f336 lea rsi,[rip+0xb46c0] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f33d lea rdi,[rbp-0xa10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5f0);
            // 7f344 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f349 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f34e movzx ebx,BYTE PTR [rbp-0xa10]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f5f0)) as u64) as u32 as u64;
            // 7f355 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7f358 jne 0x7f35f
            if fl.ne() { pc = 0x7f35f; continue; }
            pc = 0x7f35a;
        }
        0x7f35a => {
            // block 7f35a
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f35a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f35a shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7f35d jmp 0x7f366
            pc = 0x7f366; continue;
        }
        0x7f35f => {
            // block 7f35f
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f35f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f35f mov rbx,QWORD PTR [rbp-0xa08]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f5f8)) as u64;
            pc = 0x7f366;
        }
        0x7f366 => {
            // block 7f366
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f366 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f366 lea rdi,[rbp-0xa10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5f0);
            // 7f36d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f372 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7f375 je 0x7f407
            if fl.e() { pc = 0x7f407; continue; }
            pc = 0x7f37b;
        }
        0x7f37b => {
            // block 7f37b
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f37b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f37b lea rsi,[rip+0xb467b] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f382 lea rdi,[rbp-0xa50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5b0);
            // 7f389 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f38e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f393 mov rax,QWORD PTR [rbp-0xa40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f5c0)) as u64;
            // 7f39a mov QWORD PTR [rbp-0xa20],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5e0), (rax) as u64);
            // 7f3a1 mov rax,QWORD PTR [rbp-0xa50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f5b0)) as u64;
            // 7f3a8 mov rcx,QWORD PTR [rbp-0xa48]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f5b8)) as u64;
            // 7f3af mov QWORD PTR [rbp-0xa28],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5d8), (rcx) as u64);
            // 7f3b6 mov QWORD PTR [rbp-0xa30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5d0), (rax) as u64);
            // 7f3bd xorps xmm0,xmm0
            x0 = 0;
            // 7f3c0 movaps XMMWORD PTR [rbp-0xa50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f5b0), x0);
            // 7f3c7 mov QWORD PTR [rbp-0xa40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5c0), (0x0u64) as u64);
            // 7f3d2 mov DWORD PTR [rbp-0xa18],0x68
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f5e8), (0x68u64) as u32);
            // 7f3dc lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7f3e3 lea rsi,[rbp-0xa30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f5d0);
            // 7f3ea call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f3ef lea rdi,[rbp-0xa30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5d0);
            // 7f3f6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f3fb lea rdi,[rbp-0xa50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5b0);
            // 7f402 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7f407;
        }
        0x7f407 => {
            // block 7f407
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f407 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f407 xorps xmm0,xmm0
            x0 = 0;
            // 7f40a movaps XMMWORD PTR [rbp-0xa70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f590), x0);
            // 7f411 mov QWORD PTR [rbp-0xa60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5a0), (0x0u64) as u64);
            // 7f41c lea rdi,[rbp-0xa70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f590);
            // 7f423 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f428 xorps xmm0,xmm0
            x0 = 0;
            // 7f42b movaps XMMWORD PTR [rbp-0xa90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f570), x0);
            // 7f432 mov QWORD PTR [rbp-0xa80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f580), (0x0u64) as u64);
            // 7f43d lea rdi,[rbp-0xa90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f570);
            // 7f444 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f449 lea rsi,[rip+0xb9878] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7f450 lea rbx,[rbp-0xaa8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f558);
            // 7f457 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7f459 mov rdi,rbx
            rdi = rbx;
            // 7f45c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f461 mov esi,0x29
            rsi = (0x29u64) as u32 as u64;
            // 7f466 mov rdi,r15
            rdi = r15;
            // 7f469 mov rdx,rbx
            rdx = rbx;
            // 7f46c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f471 lea rdi,[rbp-0xaa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f558);
            // 7f478 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f47d mov DWORD PTR [rbp-0xab8],0x29
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f548), (0x29u64) as u32);
            // 7f487 mov QWORD PTR [rbp-0xab0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f550), (r15) as u64);
            // 7f48e lea rsi,[rbp-0xab8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f548);
            // 7f495 mov rdi,r14
            rdi = r14;
            // 7f498 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f49d lea rsi,[rip+0xb4559] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f4a4 lea rdi,[rbp-0xad0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f530);
            // 7f4ab mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f4b0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f4b5 movzx ebx,BYTE PTR [rbp-0xad0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f530)) as u64) as u32 as u64;
            // 7f4bc test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7f4bf jne 0x7f4c6
            if fl.ne() { pc = 0x7f4c6; continue; }
            pc = 0x7f4c1;
        }
        0x7f4c1 => {
            // block 7f4c1
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f4c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f4c1 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7f4c4 jmp 0x7f4cd
            pc = 0x7f4cd; continue;
        }
        0x7f4c6 => {
            // block 7f4c6
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f4c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f4c6 mov rbx,QWORD PTR [rbp-0xac8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f538)) as u64;
            pc = 0x7f4cd;
        }
        0x7f4cd => {
            // block 7f4cd
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f4cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f4cd lea rdi,[rbp-0xad0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f530);
            // 7f4d4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f4d9 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7f4dc je 0x7f56e
            if fl.e() { pc = 0x7f56e; continue; }
            pc = 0x7f4e2;
        }
        0x7f4e2 => {
            // block 7f4e2
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f4e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f4e2 lea rsi,[rip+0xb4514] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f4e9 lea rdi,[rbp-0xb10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4f0);
            // 7f4f0 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f4f5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f4fa mov rax,QWORD PTR [rbp-0xb00]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f500)) as u64;
            // 7f501 mov QWORD PTR [rbp-0xae0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f520), (rax) as u64);
            // 7f508 mov rax,QWORD PTR [rbp-0xb10]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f4f0)) as u64;
            // 7f50f mov rcx,QWORD PTR [rbp-0xb08]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f4f8)) as u64;
            // 7f516 mov QWORD PTR [rbp-0xae8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f518), (rcx) as u64);
            // 7f51d mov QWORD PTR [rbp-0xaf0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f510), (rax) as u64);
            // 7f524 xorps xmm0,xmm0
            x0 = 0;
            // 7f527 movaps XMMWORD PTR [rbp-0xb10],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f4f0), x0);
            // 7f52e mov QWORD PTR [rbp-0xb00],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f500), (0x0u64) as u64);
            // 7f539 mov DWORD PTR [rbp-0xad8],0x29
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f528), (0x29u64) as u32);
            // 7f543 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7f54a lea rsi,[rbp-0xaf0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f510);
            // 7f551 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f556 lea rdi,[rbp-0xaf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f510);
            // 7f55d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f562 lea rdi,[rbp-0xb10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4f0);
            // 7f569 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7f56e;
        }
        0x7f56e => {
            // block 7f56e
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f56e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f56e xorps xmm0,xmm0
            x0 = 0;
            // 7f571 movaps XMMWORD PTR [rbp-0xb30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f4d0), x0);
            // 7f578 mov QWORD PTR [rbp-0xb20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f4e0), (0x0u64) as u64);
            // 7f583 lea rdi,[rbp-0xb30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4d0);
            // 7f58a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f58f xorps xmm0,xmm0
            x0 = 0;
            // 7f592 movaps XMMWORD PTR [rbp-0xb50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f4b0), x0);
            // 7f599 mov QWORD PTR [rbp-0xb40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f4c0), (0x0u64) as u64);
            // 7f5a4 lea rdi,[rbp-0xb50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4b0);
            // 7f5ab call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f5b0 lea rsi,[rip+0xb9711] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7f5b7 lea rbx,[rbp-0xb68]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f498);
            // 7f5be xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7f5c0 mov rdi,rbx
            rdi = rbx;
            // 7f5c3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f5c8 mov esi,0x2a
            rsi = (0x2au64) as u32 as u64;
            // 7f5cd mov rdi,r15
            rdi = r15;
            // 7f5d0 mov rdx,rbx
            rdx = rbx;
            // 7f5d3 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f5d8 lea rdi,[rbp-0xb68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f498);
            // 7f5df call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f5e4 mov DWORD PTR [rbp-0xb78],0x2a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f488), (0x2au64) as u32);
            // 7f5ee mov QWORD PTR [rbp-0xb70],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f490), (r15) as u64);
            // 7f5f5 lea rsi,[rbp-0xb78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f488);
            // 7f5fc mov rdi,r14
            rdi = r14;
            // 7f5ff call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f604 lea rsi,[rip+0xb43f2] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f60b lea rdi,[rbp-0xb90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f470);
            // 7f612 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f617 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f61c movzx ebx,BYTE PTR [rbp-0xb90]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f470)) as u64) as u32 as u64;
            // 7f623 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7f626 jne 0x7f62d
            if fl.ne() { pc = 0x7f62d; continue; }
            pc = 0x7f628;
        }
        0x7f628 => {
            // block 7f628
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f628 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f628 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7f62b jmp 0x7f634
            pc = 0x7f634; continue;
        }
        0x7f62d => {
            // block 7f62d
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f62d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f62d mov rbx,QWORD PTR [rbp-0xb88]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f478)) as u64;
            pc = 0x7f634;
        }
        0x7f634 => {
            // block 7f634
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f634 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f634 lea rdi,[rbp-0xb90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f470);
            // 7f63b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f640 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7f643 je 0x7f6d5
            if fl.e() { pc = 0x7f6d5; continue; }
            pc = 0x7f649;
        }
        0x7f649 => {
            // block 7f649
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f649 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f649 lea rsi,[rip+0xb43ad] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f650 lea rdi,[rbp-0xbd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f430);
            // 7f657 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f65c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f661 mov rax,QWORD PTR [rbp-0xbc0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f440)) as u64;
            // 7f668 mov QWORD PTR [rbp-0xba0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f460), (rax) as u64);
            // 7f66f mov rax,QWORD PTR [rbp-0xbd0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f430)) as u64;
            // 7f676 mov rcx,QWORD PTR [rbp-0xbc8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f438)) as u64;
            // 7f67d mov QWORD PTR [rbp-0xba8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f458), (rcx) as u64);
            // 7f684 mov QWORD PTR [rbp-0xbb0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f450), (rax) as u64);
            // 7f68b xorps xmm0,xmm0
            x0 = 0;
            // 7f68e movaps XMMWORD PTR [rbp-0xbd0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f430), x0);
            // 7f695 mov QWORD PTR [rbp-0xbc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f440), (0x0u64) as u64);
            // 7f6a0 mov DWORD PTR [rbp-0xb98],0x2a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f468), (0x2au64) as u32);
            // 7f6aa lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7f6b1 lea rsi,[rbp-0xbb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f450);
            // 7f6b8 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f6bd lea rdi,[rbp-0xbb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f450);
            // 7f6c4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f6c9 lea rdi,[rbp-0xbd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f430);
            // 7f6d0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7f6d5;
        }
        0x7f6d5 => {
            // block 7f6d5
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f6d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f6d5 xorps xmm0,xmm0
            x0 = 0;
            // 7f6d8 movaps XMMWORD PTR [rbp-0xbf0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f410), x0);
            // 7f6df mov QWORD PTR [rbp-0xbe0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f420), (0x0u64) as u64);
            // 7f6ea lea rdi,[rbp-0xbf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f410);
            // 7f6f1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f6f6 xorps xmm0,xmm0
            x0 = 0;
            // 7f6f9 movaps XMMWORD PTR [rbp-0xc10],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f3f0), x0);
            // 7f700 mov QWORD PTR [rbp-0xc00],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f400), (0x0u64) as u64);
            // 7f70b lea rdi,[rbp-0xc10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3f0);
            // 7f712 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f717 lea rsi,[rip+0xb95aa] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7f71e lea rbx,[rbp-0xc28]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f3d8);
            // 7f725 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7f727 mov rdi,rbx
            rdi = rbx;
            // 7f72a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f72f mov esi,0x2b
            rsi = (0x2bu64) as u32 as u64;
            // 7f734 mov rdi,r15
            rdi = r15;
            // 7f737 mov rdx,rbx
            rdx = rbx;
            // 7f73a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f73f lea rdi,[rbp-0xc28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3d8);
            // 7f746 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f74b mov DWORD PTR [rbp-0xc38],0x2b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f3c8), (0x2bu64) as u32);
            // 7f755 mov QWORD PTR [rbp-0xc30],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f3d0), (r15) as u64);
            // 7f75c lea rsi,[rbp-0xc38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f3c8);
            // 7f763 mov rdi,r14
            rdi = r14;
            // 7f766 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f76b lea rsi,[rip+0xb428b] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f772 lea rdi,[rbp-0xc50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3b0);
            // 7f779 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f77e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f783 movzx ebx,BYTE PTR [rbp-0xc50]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f3b0)) as u64) as u32 as u64;
            // 7f78a test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7f78d jne 0x7f794
            if fl.ne() { pc = 0x7f794; continue; }
            pc = 0x7f78f;
        }
        0x7f78f => {
            // block 7f78f
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f78f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f78f shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7f792 jmp 0x7f79b
            pc = 0x7f79b; continue;
        }
        0x7f794 => {
            // block 7f794
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f794 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f794 mov rbx,QWORD PTR [rbp-0xc48]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f3b8)) as u64;
            pc = 0x7f79b;
        }
        0x7f79b => {
            // block 7f79b
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f79b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f79b lea rdi,[rbp-0xc50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3b0);
            // 7f7a2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f7a7 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7f7aa je 0x7f83c
            if fl.e() { pc = 0x7f83c; continue; }
            pc = 0x7f7b0;
        }
        0x7f7b0 => {
            // block 7f7b0
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f7b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f7b0 lea rsi,[rip+0xb4246] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f7b7 lea rdi,[rbp-0xc90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f370);
            // 7f7be mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f7c3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f7c8 mov rax,QWORD PTR [rbp-0xc80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f380)) as u64;
            // 7f7cf mov QWORD PTR [rbp-0xc60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f3a0), (rax) as u64);
            // 7f7d6 mov rax,QWORD PTR [rbp-0xc90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f370)) as u64;
            // 7f7dd mov rcx,QWORD PTR [rbp-0xc88]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f378)) as u64;
            // 7f7e4 mov QWORD PTR [rbp-0xc68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f398), (rcx) as u64);
            // 7f7eb mov QWORD PTR [rbp-0xc70],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f390), (rax) as u64);
            // 7f7f2 xorps xmm0,xmm0
            x0 = 0;
            // 7f7f5 movaps XMMWORD PTR [rbp-0xc90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f370), x0);
            // 7f7fc mov QWORD PTR [rbp-0xc80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f380), (0x0u64) as u64);
            // 7f807 mov DWORD PTR [rbp-0xc58],0x2b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f3a8), (0x2bu64) as u32);
            // 7f811 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7f818 lea rsi,[rbp-0xc70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f390);
            // 7f81f call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f824 lea rdi,[rbp-0xc70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f390);
            // 7f82b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f830 lea rdi,[rbp-0xc90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f370);
            // 7f837 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7f83c;
        }
        0x7f83c => {
            // block 7f83c
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f83c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f83c xorps xmm0,xmm0
            x0 = 0;
            // 7f83f movaps XMMWORD PTR [rbp-0xcb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f350), x0);
            // 7f846 mov QWORD PTR [rbp-0xca0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f360), (0x0u64) as u64);
            // 7f851 lea rdi,[rbp-0xcb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f350);
            // 7f858 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f85d xorps xmm0,xmm0
            x0 = 0;
            // 7f860 movaps XMMWORD PTR [rbp-0xcd0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f330), x0);
            // 7f867 mov QWORD PTR [rbp-0xcc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f340), (0x0u64) as u64);
            // 7f872 lea rdi,[rbp-0xcd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f330);
            // 7f879 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f87e lea rsi,[rip+0xb9443] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7f885 lea rbx,[rbp-0xce8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f318);
            // 7f88c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7f88e mov rdi,rbx
            rdi = rbx;
            // 7f891 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f896 mov esi,0x69
            rsi = (0x69u64) as u32 as u64;
            // 7f89b mov rdi,r15
            rdi = r15;
            // 7f89e mov rdx,rbx
            rdx = rbx;
            // 7f8a1 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f8a6 lea rdi,[rbp-0xce8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f318);
            // 7f8ad call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f8b2 mov DWORD PTR [rbp-0xcf8],0x69
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f308), (0x69u64) as u32);
            // 7f8bc mov QWORD PTR [rbp-0xcf0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f310), (r15) as u64);
            // 7f8c3 lea rsi,[rbp-0xcf8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f308);
            // 7f8ca mov rdi,r14
            rdi = r14;
            // 7f8cd call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f8d2 lea rsi,[rip+0xb4124] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f8d9 lea rdi,[rbp-0xd10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2f0);
            // 7f8e0 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f8e5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f8ea movzx ebx,BYTE PTR [rbp-0xd10]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f2f0)) as u64) as u32 as u64;
            // 7f8f1 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7f8f4 jne 0x7f8fb
            if fl.ne() { pc = 0x7f8fb; continue; }
            pc = 0x7f8f6;
        }
        0x7f8f6 => {
            // block 7f8f6
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f8f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f8f6 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7f8f9 jmp 0x7f902
            pc = 0x7f902; continue;
        }
        0x7f8fb => {
            // block 7f8fb
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f8fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f8fb mov rbx,QWORD PTR [rbp-0xd08]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f2f8)) as u64;
            pc = 0x7f902;
        }
        0x7f902 => {
            // block 7f902
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f902 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f902 lea rdi,[rbp-0xd10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2f0);
            // 7f909 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f90e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7f911 je 0x7f9a3
            if fl.e() { pc = 0x7f9a3; continue; }
            pc = 0x7f917;
        }
        0x7f917 => {
            // block 7f917
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f917 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f917 lea rsi,[rip+0xb40df] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7f91e lea rdi,[rbp-0xd50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2b0);
            // 7f925 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7f92a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f92f mov rax,QWORD PTR [rbp-0xd40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f2c0)) as u64;
            // 7f936 mov QWORD PTR [rbp-0xd20],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f2e0), (rax) as u64);
            // 7f93d mov rax,QWORD PTR [rbp-0xd50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f2b0)) as u64;
            // 7f944 mov rcx,QWORD PTR [rbp-0xd48]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f2b8)) as u64;
            // 7f94b mov QWORD PTR [rbp-0xd28],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f2d8), (rcx) as u64);
            // 7f952 mov QWORD PTR [rbp-0xd30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f2d0), (rax) as u64);
            // 7f959 xorps xmm0,xmm0
            x0 = 0;
            // 7f95c movaps XMMWORD PTR [rbp-0xd50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f2b0), x0);
            // 7f963 mov QWORD PTR [rbp-0xd40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f2c0), (0x0u64) as u64);
            // 7f96e mov DWORD PTR [rbp-0xd18],0x69
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f2e8), (0x69u64) as u32);
            // 7f978 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7f97f lea rsi,[rbp-0xd30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f2d0);
            // 7f986 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f98b lea rdi,[rbp-0xd30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2d0);
            // 7f992 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f997 lea rdi,[rbp-0xd50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2b0);
            // 7f99e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7f9a3;
        }
        0x7f9a3 => {
            // block 7f9a3
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7f9a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f9a3 xorps xmm0,xmm0
            x0 = 0;
            // 7f9a6 movaps XMMWORD PTR [rbp-0xd70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f290), x0);
            // 7f9ad mov QWORD PTR [rbp-0xd60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f2a0), (0x0u64) as u64);
            // 7f9b8 lea rdi,[rbp-0xd70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f290);
            // 7f9bf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f9c4 xorps xmm0,xmm0
            x0 = 0;
            // 7f9c7 movaps XMMWORD PTR [rbp-0xd90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f270), x0);
            // 7f9ce mov QWORD PTR [rbp-0xd80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f280), (0x0u64) as u64);
            // 7f9d9 lea rdi,[rbp-0xd90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f270);
            // 7f9e0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f9e5 lea rsi,[rip+0xb92dc] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7f9ec lea rbx,[rbp-0xda8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f258);
            // 7f9f3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7f9f5 mov rdi,rbx
            rdi = rbx;
            // 7f9f8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7f9fd mov esi,0x2c
            rsi = (0x2cu64) as u32 as u64;
            // 7fa02 mov rdi,r15
            rdi = r15;
            // 7fa05 mov rdx,rbx
            rdx = rbx;
            // 7fa08 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fa0d lea rdi,[rbp-0xda8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f258);
            // 7fa14 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fa19 mov DWORD PTR [rbp-0xdb8],0x2c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f248), (0x2cu64) as u32);
            // 7fa23 mov QWORD PTR [rbp-0xdb0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f250), (r15) as u64);
            // 7fa2a lea rsi,[rbp-0xdb8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f248);
            // 7fa31 mov rdi,r14
            rdi = r14;
            // 7fa34 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fa39 lea rsi,[rip+0xb3fbd] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7fa40 lea rdi,[rbp-0xdd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f230);
            // 7fa47 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7fa4c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fa51 movzx ebx,BYTE PTR [rbp-0xdd0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f230)) as u64) as u32 as u64;
            // 7fa58 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7fa5b jne 0x7fa62
            if fl.ne() { pc = 0x7fa62; continue; }
            pc = 0x7fa5d;
        }
        0x7fa5d => {
            // block 7fa5d
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fa5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fa5d shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7fa60 jmp 0x7fa69
            pc = 0x7fa69; continue;
        }
        0x7fa62 => {
            // block 7fa62
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fa62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fa62 mov rbx,QWORD PTR [rbp-0xdc8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f238)) as u64;
            pc = 0x7fa69;
        }
        0x7fa69 => {
            // block 7fa69
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fa69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fa69 lea rdi,[rbp-0xdd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f230);
            // 7fa70 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fa75 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7fa78 je 0x7fb0a
            if fl.e() { pc = 0x7fb0a; continue; }
            pc = 0x7fa7e;
        }
        0x7fa7e => {
            // block 7fa7e
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fa7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fa7e lea rsi,[rip+0xb3f78] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7fa85 lea rdi,[rbp-0xe10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1f0);
            // 7fa8c mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7fa91 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fa96 mov rax,QWORD PTR [rbp-0xe00]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f200)) as u64;
            // 7fa9d mov QWORD PTR [rbp-0xde0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f220), (rax) as u64);
            // 7faa4 mov rax,QWORD PTR [rbp-0xe10]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f1f0)) as u64;
            // 7faab mov rcx,QWORD PTR [rbp-0xe08]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f1f8)) as u64;
            // 7fab2 mov QWORD PTR [rbp-0xde8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f218), (rcx) as u64);
            // 7fab9 mov QWORD PTR [rbp-0xdf0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f210), (rax) as u64);
            // 7fac0 xorps xmm0,xmm0
            x0 = 0;
            // 7fac3 movaps XMMWORD PTR [rbp-0xe10],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f1f0), x0);
            // 7faca mov QWORD PTR [rbp-0xe00],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f200), (0x0u64) as u64);
            // 7fad5 mov DWORD PTR [rbp-0xdd8],0x2c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f228), (0x2cu64) as u32);
            // 7fadf lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7fae6 lea rsi,[rbp-0xdf0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f210);
            // 7faed call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7faf2 lea rdi,[rbp-0xdf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f210);
            // 7faf9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fafe lea rdi,[rbp-0xe10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1f0);
            // 7fb05 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7fb0a;
        }
        0x7fb0a => {
            // block 7fb0a
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fb0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fb0a xorps xmm0,xmm0
            x0 = 0;
            // 7fb0d movaps XMMWORD PTR [rbp-0xe30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f1d0), x0);
            // 7fb14 mov QWORD PTR [rbp-0xe20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f1e0), (0x0u64) as u64);
            // 7fb1f lea rdi,[rbp-0xe30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1d0);
            // 7fb26 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fb2b xorps xmm0,xmm0
            x0 = 0;
            // 7fb2e movaps XMMWORD PTR [rbp-0xe50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f1b0), x0);
            // 7fb35 mov QWORD PTR [rbp-0xe40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f1c0), (0x0u64) as u64);
            // 7fb40 lea rdi,[rbp-0xe50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1b0);
            // 7fb47 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fb4c lea rsi,[rip+0xb9175] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7fb53 lea rbx,[rbp-0xe68]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f198);
            // 7fb5a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7fb5c mov rdi,rbx
            rdi = rbx;
            // 7fb5f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fb64 mov esi,0x6a
            rsi = (0x6au64) as u32 as u64;
            // 7fb69 mov rdi,r15
            rdi = r15;
            // 7fb6c mov rdx,rbx
            rdx = rbx;
            // 7fb6f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fb74 lea rdi,[rbp-0xe68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f198);
            // 7fb7b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fb80 mov DWORD PTR [rbp-0xe78],0x6a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f188), (0x6au64) as u32);
            // 7fb8a mov QWORD PTR [rbp-0xe70],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f190), (r15) as u64);
            // 7fb91 lea rsi,[rbp-0xe78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f188);
            // 7fb98 mov rdi,r14
            rdi = r14;
            // 7fb9b call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fba0 lea rsi,[rip+0xb3e56] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7fba7 lea rdi,[rbp-0xe90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f170);
            // 7fbae mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7fbb3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fbb8 movzx ebx,BYTE PTR [rbp-0xe90]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f170)) as u64) as u32 as u64;
            // 7fbbf test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7fbc2 jne 0x7fbc9
            if fl.ne() { pc = 0x7fbc9; continue; }
            pc = 0x7fbc4;
        }
        0x7fbc4 => {
            // block 7fbc4
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fbc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fbc4 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7fbc7 jmp 0x7fbd0
            pc = 0x7fbd0; continue;
        }
        0x7fbc9 => {
            // block 7fbc9
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fbc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fbc9 mov rbx,QWORD PTR [rbp-0xe88]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f178)) as u64;
            pc = 0x7fbd0;
        }
        0x7fbd0 => {
            // block 7fbd0
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fbd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fbd0 lea rdi,[rbp-0xe90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f170);
            // 7fbd7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fbdc test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7fbdf je 0x7fc71
            if fl.e() { pc = 0x7fc71; continue; }
            pc = 0x7fbe5;
        }
        0x7fbe5 => {
            // block 7fbe5
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fbe5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fbe5 lea rsi,[rip+0xb3e11] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7fbec lea rdi,[rbp-0xed0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f130);
            // 7fbf3 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7fbf8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fbfd mov rax,QWORD PTR [rbp-0xec0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f140)) as u64;
            // 7fc04 mov QWORD PTR [rbp-0xea0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f160), (rax) as u64);
            // 7fc0b mov rax,QWORD PTR [rbp-0xed0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f130)) as u64;
            // 7fc12 mov rcx,QWORD PTR [rbp-0xec8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f138)) as u64;
            // 7fc19 mov QWORD PTR [rbp-0xea8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f158), (rcx) as u64);
            // 7fc20 mov QWORD PTR [rbp-0xeb0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f150), (rax) as u64);
            // 7fc27 xorps xmm0,xmm0
            x0 = 0;
            // 7fc2a movaps XMMWORD PTR [rbp-0xed0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f130), x0);
            // 7fc31 mov QWORD PTR [rbp-0xec0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f140), (0x0u64) as u64);
            // 7fc3c mov DWORD PTR [rbp-0xe98],0x6a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f168), (0x6au64) as u32);
            // 7fc46 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7fc4d lea rsi,[rbp-0xeb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f150);
            // 7fc54 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fc59 lea rdi,[rbp-0xeb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f150);
            // 7fc60 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fc65 lea rdi,[rbp-0xed0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f130);
            // 7fc6c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7fc71;
        }
        0x7fc71 => {
            // block 7fc71
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fc71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fc71 xorps xmm0,xmm0
            x0 = 0;
            // 7fc74 movaps XMMWORD PTR [rbp-0xef0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f110), x0);
            // 7fc7b mov QWORD PTR [rbp-0xee0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f120), (0x0u64) as u64);
            // 7fc86 lea rdi,[rbp-0xef0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f110);
            // 7fc8d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fc92 xorps xmm0,xmm0
            x0 = 0;
            // 7fc95 movaps XMMWORD PTR [rbp-0xf10],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f0f0), x0);
            // 7fc9c mov QWORD PTR [rbp-0xf00],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f100), (0x0u64) as u64);
            // 7fca7 lea rdi,[rbp-0xf10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0f0);
            // 7fcae call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fcb3 lea rsi,[rip+0xb900e] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7fcba lea rbx,[rbp-0xf28]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f0d8);
            // 7fcc1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7fcc3 mov rdi,rbx
            rdi = rbx;
            // 7fcc6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fccb mov esi,0x2d
            rsi = (0x2du64) as u32 as u64;
            // 7fcd0 mov rdi,r15
            rdi = r15;
            // 7fcd3 mov rdx,rbx
            rdx = rbx;
            // 7fcd6 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fcdb lea rdi,[rbp-0xf28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0d8);
            // 7fce2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fce7 mov DWORD PTR [rbp-0xf38],0x2d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0c8), (0x2du64) as u32);
            // 7fcf1 mov QWORD PTR [rbp-0xf30],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f0d0), (r15) as u64);
            // 7fcf8 lea rsi,[rbp-0xf38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0c8);
            // 7fcff mov rdi,r14
            rdi = r14;
            // 7fd02 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fd07 lea rsi,[rip+0xb3cef] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7fd0e lea rdi,[rbp-0xf50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0b0);
            // 7fd15 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7fd1a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fd1f movzx ebx,BYTE PTR [rbp-0xf50]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f0b0)) as u64) as u32 as u64;
            // 7fd26 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7fd29 jne 0x7fd30
            if fl.ne() { pc = 0x7fd30; continue; }
            pc = 0x7fd2b;
        }
        0x7fd2b => {
            // block 7fd2b
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fd2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fd2b shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7fd2e jmp 0x7fd37
            pc = 0x7fd37; continue;
        }
        0x7fd30 => {
            // block 7fd30
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fd30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fd30 mov rbx,QWORD PTR [rbp-0xf48]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f0b8)) as u64;
            pc = 0x7fd37;
        }
        0x7fd37 => {
            // block 7fd37
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fd37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fd37 lea rdi,[rbp-0xf50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0b0);
            // 7fd3e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fd43 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7fd46 je 0x7fdd8
            if fl.e() { pc = 0x7fdd8; continue; }
            pc = 0x7fd4c;
        }
        0x7fd4c => {
            // block 7fd4c
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fd4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fd4c lea rsi,[rip+0xb3caa] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7fd53 lea rdi,[rbp-0xf90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f070);
            // 7fd5a mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7fd5f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fd64 mov rax,QWORD PTR [rbp-0xf80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f080)) as u64;
            // 7fd6b mov QWORD PTR [rbp-0xf60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f0a0), (rax) as u64);
            // 7fd72 mov rax,QWORD PTR [rbp-0xf90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f070)) as u64;
            // 7fd79 mov rcx,QWORD PTR [rbp-0xf88]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f078)) as u64;
            // 7fd80 mov QWORD PTR [rbp-0xf68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f098), (rcx) as u64);
            // 7fd87 mov QWORD PTR [rbp-0xf70],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f090), (rax) as u64);
            // 7fd8e xorps xmm0,xmm0
            x0 = 0;
            // 7fd91 movaps XMMWORD PTR [rbp-0xf90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f070), x0);
            // 7fd98 mov QWORD PTR [rbp-0xf80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f080), (0x0u64) as u64);
            // 7fda3 mov DWORD PTR [rbp-0xf58],0x2d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0a8), (0x2du64) as u32);
            // 7fdad lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7fdb4 lea rsi,[rbp-0xf70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f090);
            // 7fdbb call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fdc0 lea rdi,[rbp-0xf70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f090);
            // 7fdc7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fdcc lea rdi,[rbp-0xf90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f070);
            // 7fdd3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7fdd8;
        }
        0x7fdd8 => {
            // block 7fdd8
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fdd8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fdd8 xorps xmm0,xmm0
            x0 = 0;
            // 7fddb movaps XMMWORD PTR [rbp-0xfb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f050), x0);
            // 7fde2 mov QWORD PTR [rbp-0xfa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f060), (0x0u64) as u64);
            // 7fded lea rdi,[rbp-0xfb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f050);
            // 7fdf4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fdf9 xorps xmm0,xmm0
            x0 = 0;
            // 7fdfc movaps XMMWORD PTR [rbp-0xfd0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f030), x0);
            // 7fe03 mov QWORD PTR [rbp-0xfc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f040), (0x0u64) as u64);
            // 7fe0e lea rdi,[rbp-0xfd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f030);
            // 7fe15 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fe1a lea rsi,[rip+0xb8ea7] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7fe21 lea rbx,[rbp-0xfe8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f018);
            // 7fe28 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7fe2a mov rdi,rbx
            rdi = rbx;
            // 7fe2d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fe32 mov esi,0x2e
            rsi = (0x2eu64) as u32 as u64;
            // 7fe37 mov rdi,r15
            rdi = r15;
            // 7fe3a mov rdx,rbx
            rdx = rbx;
            // 7fe3d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fe42 lea rdi,[rbp-0xfe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f018);
            // 7fe49 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fe4e mov DWORD PTR [rbp-0xff8],0x2e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f008), (0x2eu64) as u32);
            // 7fe58 mov QWORD PTR [rbp-0xff0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f010), (r15) as u64);
            // 7fe5f lea rsi,[rbp-0xff8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f008);
            // 7fe66 mov rdi,r14
            rdi = r14;
            // 7fe69 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fe6e lea rsi,[rip+0xb3b88] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7fe75 lea rdi,[rbp-0x1010]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eff0);
            // 7fe7c mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7fe81 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fe86 movzx ebx,BYTE PTR [rbp-0x1010]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_eff0)) as u64) as u32 as u64;
            // 7fe8d test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7fe90 jne 0x7fe97
            if fl.ne() { pc = 0x7fe97; continue; }
            pc = 0x7fe92;
        }
        0x7fe92 => {
            // block 7fe92
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fe92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fe92 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7fe95 jmp 0x7fe9e
            pc = 0x7fe9e; continue;
        }
        0x7fe97 => {
            // block 7fe97
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fe97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fe97 mov rbx,QWORD PTR [rbp-0x1008]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eff8)) as u64;
            pc = 0x7fe9e;
        }
        0x7fe9e => {
            // block 7fe9e
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fe9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fe9e lea rdi,[rbp-0x1010]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eff0);
            // 7fea5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7feaa test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7fead je 0x7ff3f
            if fl.e() { pc = 0x7ff3f; continue; }
            pc = 0x7feb3;
        }
        0x7feb3 => {
            // block 7feb3
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7feb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7feb3 lea rsi,[rip+0xb3b43] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7feba lea rdi,[rbp-0x1050]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efb0);
            // 7fec1 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7fec6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7fecb mov rax,QWORD PTR [rbp-0x1040]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_efc0)) as u64;
            // 7fed2 mov QWORD PTR [rbp-0x1020],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efe0), (rax) as u64);
            // 7fed9 mov rax,QWORD PTR [rbp-0x1050]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_efb0)) as u64;
            // 7fee0 mov rcx,QWORD PTR [rbp-0x1048]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_efb8)) as u64;
            // 7fee7 mov QWORD PTR [rbp-0x1028],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efd8), (rcx) as u64);
            // 7feee mov QWORD PTR [rbp-0x1030],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efd0), (rax) as u64);
            // 7fef5 xorps xmm0,xmm0
            x0 = 0;
            // 7fef8 movaps XMMWORD PTR [rbp-0x1050],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_efb0), x0);
            // 7feff mov QWORD PTR [rbp-0x1040],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efc0), (0x0u64) as u64);
            // 7ff0a mov DWORD PTR [rbp-0x1018],0x2e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_efe8), (0x2eu64) as u32);
            // 7ff14 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 7ff1b lea rsi,[rbp-0x1030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efd0);
            // 7ff22 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ff27 lea rdi,[rbp-0x1030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efd0);
            // 7ff2e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ff33 lea rdi,[rbp-0x1050]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efb0);
            // 7ff3a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7ff3f;
        }
        0x7ff3f => {
            // block 7ff3f
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7ff3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ff3f xorps xmm0,xmm0
            x0 = 0;
            // 7ff42 movaps XMMWORD PTR [rbp-0x1070],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ef90), x0);
            // 7ff49 mov QWORD PTR [rbp-0x1060],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efa0), (0x0u64) as u64);
            // 7ff54 lea rdi,[rbp-0x1070]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef90);
            // 7ff5b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ff60 xorps xmm0,xmm0
            x0 = 0;
            // 7ff63 movaps XMMWORD PTR [rbp-0x1090],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ef70), x0);
            // 7ff6a mov QWORD PTR [rbp-0x1080],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef80), (0x0u64) as u64);
            // 7ff75 lea rdi,[rbp-0x1090]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef70);
            // 7ff7c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ff81 lea rsi,[rip+0xb8d40] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7ff88 lea rbx,[rbp-0x10a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ef58);
            // 7ff8f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7ff91 mov rdi,rbx
            rdi = rbx;
            // 7ff94 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ff99 mov esi,0x2f
            rsi = (0x2fu64) as u32 as u64;
            // 7ff9e mov rdi,r15
            rdi = r15;
            // 7ffa1 mov rdx,rbx
            rdx = rbx;
            // 7ffa4 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ffa9 lea rdi,[rbp-0x10a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef58);
            // 7ffb0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ffb5 mov DWORD PTR [rbp-0x10b8],0x2f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef48), (0x2fu64) as u32);
            // 7ffbf mov QWORD PTR [rbp-0x10b0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef50), (r15) as u64);
            // 7ffc6 lea rsi,[rbp-0x10b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef48);
            // 7ffcd mov rdi,r14
            rdi = r14;
            // 7ffd0 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ffd5 lea rsi,[rip+0xb3a21] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 7ffdc lea rdi,[rbp-0x10d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef30);
            // 7ffe3 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 7ffe8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ffed movzx ebx,BYTE PTR [rbp-0x10d0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ef30)) as u64) as u32 as u64;
            // 7fff4 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7fff7 jne 0x7fffe
            if fl.ne() { pc = 0x7fffe; continue; }
            pc = 0x7fff9;
        }
        0x7fff9 => {
            // block 7fff9
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fff9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fff9 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7fffc jmp 0x80005
            pc = 0x80005; continue;
        }
        0x7fffe => {
            // block 7fffe
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 7fffe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fffe mov rbx,QWORD PTR [rbp-0x10c8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ef38)) as u64;
            pc = 0x80005;
        }
        0x80005 => {
            // block 80005
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80005 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80005 lea rdi,[rbp-0x10d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef30);
            // 8000c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80011 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 80014 je 0x800a6
            if fl.e() { pc = 0x800a6; continue; }
            pc = 0x8001a;
        }
        0x8001a => {
            // block 8001a
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 8001a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8001a lea rsi,[rip+0xb39dc] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 80021 lea rdi,[rbp-0x1110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eef0);
            // 80028 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 8002d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80032 mov rax,QWORD PTR [rbp-0x1100]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ef00)) as u64;
            // 80039 mov QWORD PTR [rbp-0x10e0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef20), (rax) as u64);
            // 80040 mov rax,QWORD PTR [rbp-0x1110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eef0)) as u64;
            // 80047 mov rcx,QWORD PTR [rbp-0x1108]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eef8)) as u64;
            // 8004e mov QWORD PTR [rbp-0x10e8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef18), (rcx) as u64);
            // 80055 mov QWORD PTR [rbp-0x10f0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef10), (rax) as u64);
            // 8005c xorps xmm0,xmm0
            x0 = 0;
            // 8005f movaps XMMWORD PTR [rbp-0x1110],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_eef0), x0);
            // 80066 mov QWORD PTR [rbp-0x1100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef00), (0x0u64) as u64);
            // 80071 mov DWORD PTR [rbp-0x10d8],0x2f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef28), (0x2fu64) as u32);
            // 8007b lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 80082 lea rsi,[rbp-0x10f0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef10);
            // 80089 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8008e lea rdi,[rbp-0x10f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef10);
            // 80095 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8009a lea rdi,[rbp-0x1110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eef0);
            // 800a1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x800a6;
        }
        0x800a6 => {
            // block 800a6
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 800a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 800a6 xorps xmm0,xmm0
            x0 = 0;
            // 800a9 movaps XMMWORD PTR [rbp-0x1130],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_eed0), x0);
            // 800b0 mov QWORD PTR [rbp-0x1120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eee0), (0x0u64) as u64);
            // 800bb lea rdi,[rbp-0x1130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eed0);
            // 800c2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 800c7 xorps xmm0,xmm0
            x0 = 0;
            // 800ca movaps XMMWORD PTR [rbp-0x1150],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_eeb0), x0);
            // 800d1 mov QWORD PTR [rbp-0x1140],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eec0), (0x0u64) as u64);
            // 800dc lea rdi,[rbp-0x1150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eeb0);
            // 800e3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 800e8 lea rsi,[rip+0xb8bd9] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 800ef lea rbx,[rbp-0x1168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ee98);
            // 800f6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 800f8 mov rdi,rbx
            rdi = rbx;
            // 800fb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80100 mov esi,0x6b
            rsi = (0x6bu64) as u32 as u64;
            // 80105 mov rdi,r15
            rdi = r15;
            // 80108 mov rdx,rbx
            rdx = rbx;
            // 8010b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80110 lea rdi,[rbp-0x1168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee98);
            // 80117 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8011c mov DWORD PTR [rbp-0x1178],0x6b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee88), (0x6bu64) as u32);
            // 80126 mov QWORD PTR [rbp-0x1170],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee90), (r15) as u64);
            // 8012d lea rsi,[rbp-0x1178]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee88);
            // 80134 mov rdi,r14
            rdi = r14;
            // 80137 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8013c lea rsi,[rip+0xb38ba] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 80143 lea rdi,[rbp-0x1190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee70);
            // 8014a mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 8014f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80154 movzx ebx,BYTE PTR [rbp-0x1190]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ee70)) as u64) as u32 as u64;
            // 8015b test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 8015e jne 0x80165
            if fl.ne() { pc = 0x80165; continue; }
            pc = 0x80160;
        }
        0x80160 => {
            // block 80160
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80160 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80160 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 80163 jmp 0x8016c
            pc = 0x8016c; continue;
        }
        0x80165 => {
            // block 80165
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80165 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80165 mov rbx,QWORD PTR [rbp-0x1188]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ee78)) as u64;
            pc = 0x8016c;
        }
        0x8016c => {
            // block 8016c
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 8016c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8016c lea rdi,[rbp-0x1190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee70);
            // 80173 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80178 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 8017b je 0x8020d
            if fl.e() { pc = 0x8020d; continue; }
            pc = 0x80181;
        }
        0x80181 => {
            // block 80181
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80181 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80181 lea rsi,[rip+0xb3875] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 80188 lea rdi,[rbp-0x11d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee30);
            // 8018f mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 80194 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80199 mov rax,QWORD PTR [rbp-0x11c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ee40)) as u64;
            // 801a0 mov QWORD PTR [rbp-0x11a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee60), (rax) as u64);
            // 801a7 mov rax,QWORD PTR [rbp-0x11d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ee30)) as u64;
            // 801ae mov rcx,QWORD PTR [rbp-0x11c8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ee38)) as u64;
            // 801b5 mov QWORD PTR [rbp-0x11a8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee58), (rcx) as u64);
            // 801bc mov QWORD PTR [rbp-0x11b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee50), (rax) as u64);
            // 801c3 xorps xmm0,xmm0
            x0 = 0;
            // 801c6 movaps XMMWORD PTR [rbp-0x11d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ee30), x0);
            // 801cd mov QWORD PTR [rbp-0x11c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee40), (0x0u64) as u64);
            // 801d8 mov DWORD PTR [rbp-0x1198],0x6b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee68), (0x6bu64) as u32);
            // 801e2 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 801e9 lea rsi,[rbp-0x11b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee50);
            // 801f0 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 801f5 lea rdi,[rbp-0x11b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee50);
            // 801fc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80201 lea rdi,[rbp-0x11d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee30);
            // 80208 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x8020d;
        }
        0x8020d => {
            // block 8020d
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 8020d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8020d xorps xmm0,xmm0
            x0 = 0;
            // 80210 movaps XMMWORD PTR [rbp-0x11f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ee10), x0);
            // 80217 mov QWORD PTR [rbp-0x11e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee20), (0x0u64) as u64);
            // 80222 lea rdi,[rbp-0x11f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee10);
            // 80229 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8022e xorps xmm0,xmm0
            x0 = 0;
            // 80231 movaps XMMWORD PTR [rbp-0x1210],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_edf0), x0);
            // 80238 mov QWORD PTR [rbp-0x1200],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee00), (0x0u64) as u64);
            // 80243 lea rdi,[rbp-0x1210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edf0);
            // 8024a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8024f lea rsi,[rip+0xb8a72] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 80256 lea rbx,[rbp-0x1228]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_edd8);
            // 8025d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 8025f mov rdi,rbx
            rdi = rbx;
            // 80262 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80267 mov esi,0x30
            rsi = (0x30u64) as u32 as u64;
            // 8026c mov rdi,r15
            rdi = r15;
            // 8026f mov rdx,rbx
            rdx = rbx;
            // 80272 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80277 lea rdi,[rbp-0x1228]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edd8);
            // 8027e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80283 lea rax,[r15+0x128]
            rax = r15.wrapping_add(0x128);
            // 8028a mov DWORD PTR [rbp-0x1238],0x30
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edc8), (0x30u64) as u32);
            // 80294 mov QWORD PTR [rbp-0x1230],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_edd0), (rax) as u64);
            // 8029b lea rsi,[rbp-0x1238]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edc8);
            // 802a2 mov rdi,r14
            rdi = r14;
            // 802a5 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 802aa lea rsi,[rip+0xb374c] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 802b1 lea rdi,[rbp-0x1250]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edb0);
            // 802b8 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 802bd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 802c2 movzx ebx,BYTE PTR [rbp-0x1250]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_edb0)) as u64) as u32 as u64;
            // 802c9 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 802cc jne 0x802d3
            if fl.ne() { pc = 0x802d3; continue; }
            pc = 0x802ce;
        }
        0x802ce => {
            // block 802ce
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 802ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 802ce shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 802d1 jmp 0x802da
            pc = 0x802da; continue;
        }
        0x802d3 => {
            // block 802d3
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 802d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 802d3 mov rbx,QWORD PTR [rbp-0x1248]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_edb8)) as u64;
            pc = 0x802da;
        }
        0x802da => {
            // block 802da
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 802da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 802da lea rdi,[rbp-0x1250]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edb0);
            // 802e1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 802e6 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 802e9 je 0x8037b
            if fl.e() { pc = 0x8037b; continue; }
            pc = 0x802ef;
        }
        0x802ef => {
            // block 802ef
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 802ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 802ef lea rsi,[rip+0xb3707] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 802f6 lea rdi,[rbp-0x1290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed70);
            // 802fd mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 80302 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80307 mov rax,QWORD PTR [rbp-0x1280]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ed80)) as u64;
            // 8030e mov QWORD PTR [rbp-0x1260],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eda0), (rax) as u64);
            // 80315 mov rax,QWORD PTR [rbp-0x1290]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ed70)) as u64;
            // 8031c mov rcx,QWORD PTR [rbp-0x1288]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ed78)) as u64;
            // 80323 mov QWORD PTR [rbp-0x1268],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed98), (rcx) as u64);
            // 8032a mov QWORD PTR [rbp-0x1270],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed90), (rax) as u64);
            // 80331 xorps xmm0,xmm0
            x0 = 0;
            // 80334 movaps XMMWORD PTR [rbp-0x1290],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ed70), x0);
            // 8033b mov QWORD PTR [rbp-0x1280],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed80), (0x0u64) as u64);
            // 80346 mov DWORD PTR [rbp-0x1258],0x30
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eda8), (0x30u64) as u32);
            // 80350 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 80357 lea rsi,[rbp-0x1270]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed90);
            // 8035e call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80363 lea rdi,[rbp-0x1270]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed90);
            // 8036a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8036f lea rdi,[rbp-0x1290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed70);
            // 80376 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x8037b;
        }
        0x8037b => {
            // block 8037b
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 8037b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8037b xorps xmm0,xmm0
            x0 = 0;
            // 8037e movaps XMMWORD PTR [rbp-0x12b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ed50), x0);
            // 80385 mov QWORD PTR [rbp-0x12a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed60), (0x0u64) as u64);
            // 80390 lea rdi,[rbp-0x12b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed50);
            // 80397 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8039c xorps xmm0,xmm0
            x0 = 0;
            // 8039f movaps XMMWORD PTR [rbp-0x12d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ed30), x0);
            // 803a6 mov QWORD PTR [rbp-0x12c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed40), (0x0u64) as u64);
            // 803b1 lea rdi,[rbp-0x12d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed30);
            // 803b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 803bd lea rsi,[rip+0xb8904] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 803c4 lea rbx,[rbp-0x12e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ed18);
            // 803cb xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 803cd mov rdi,rbx
            rdi = rbx;
            // 803d0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 803d5 mov esi,0x31
            rsi = (0x31u64) as u32 as u64;
            // 803da mov rdi,r15
            rdi = r15;
            // 803dd mov rdx,rbx
            rdx = rbx;
            // 803e0 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 803e5 lea rdi,[rbp-0x12e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed18);
            // 803ec call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 803f1 lea rax,[r15+0x130]
            rax = r15.wrapping_add(0x130);
            // 803f8 mov DWORD PTR [rbp-0x12f8],0x31
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed08), (0x31u64) as u32);
            // 80402 mov QWORD PTR [rbp-0x12f0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed10), (rax) as u64);
            // 80409 lea rsi,[rbp-0x12f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed08);
            // 80410 mov rdi,r14
            rdi = r14;
            // 80413 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80418 lea rsi,[rip+0xb35de] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 8041f lea rdi,[rbp-0x1310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecf0);
            // 80426 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 8042b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80430 movzx ebx,BYTE PTR [rbp-0x1310]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ecf0)) as u64) as u32 as u64;
            // 80437 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 8043a jne 0x80441
            if fl.ne() { pc = 0x80441; continue; }
            pc = 0x8043c;
        }
        0x8043c => {
            // block 8043c
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 8043c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8043c shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 8043f jmp 0x80448
            pc = 0x80448; continue;
        }
        0x80441 => {
            // block 80441
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80441 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80441 mov rbx,QWORD PTR [rbp-0x1308]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ecf8)) as u64;
            pc = 0x80448;
        }
        0x80448 => {
            // block 80448
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80448 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80448 lea rdi,[rbp-0x1310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecf0);
            // 8044f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80454 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 80457 je 0x804ec
            if fl.e() { pc = 0x804ec; continue; }
            pc = 0x8045d;
        }
        0x8045d => {
            // block 8045d
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 8045d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8045d lea rsi,[rip+0xb3599] # 0x1339fd
            rsi = m.env.data_va(295370);
            // 80464 lea rdi,[rbp-0x1350]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecb0);
            // 8046b mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 80470 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80475 mov rax,QWORD PTR [rbp-0x1340]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ecc0)) as u64;
            // 8047c mov QWORD PTR [rbp-0x1320],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ece0), (rax) as u64);
            // 80483 mov rax,QWORD PTR [rbp-0x1350]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ecb0)) as u64;
            // 8048a mov rcx,QWORD PTR [rbp-0x1348]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ecb8)) as u64;
            // 80491 mov QWORD PTR [rbp-0x1328],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ecd8), (rcx) as u64);
            // 80498 mov QWORD PTR [rbp-0x1330],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ecd0), (rax) as u64);
            // 8049f xorps xmm0,xmm0
            x0 = 0;
            // 804a2 movaps XMMWORD PTR [rbp-0x1350],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ecb0), x0);
            // 804a9 mov QWORD PTR [rbp-0x1340],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ecc0), (0x0u64) as u64);
            // 804b4 mov DWORD PTR [rbp-0x1318],0x31
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ece8), (0x31u64) as u32);
            // 804be add r15,0x198
            { let (r, f) = add::<64>(r15, 0x198u64);
              r15 = r;
              fl = f; }
            // 804c5 lea rsi,[rbp-0x1330]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ecd0);
            // 804cc mov rdi,r15
            rdi = r15;
            // 804cf call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 804d4 lea rdi,[rbp-0x1330]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecd0);
            // 804db call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 804e0 lea rdi,[rbp-0x1350]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecb0);
            // 804e7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x804ec;
        }
        0x804ec => {
            // block 804ec
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 804ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 804ec xorps xmm0,xmm0
            x0 = 0;
            // 804ef movaps XMMWORD PTR [rbp-0x1370],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ec90), x0);
            // 804f6 mov QWORD PTR [rbp-0x1360],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eca0), (0x0u64) as u64);
            // 80501 lea rdi,[rbp-0x1370]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec90);
            // 80508 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8050d xorps xmm0,xmm0
            x0 = 0;
            // 80510 movaps XMMWORD PTR [rbp-0x1390],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ec70), x0);
            // 80517 mov QWORD PTR [rbp-0x1380],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ec80), (0x0u64) as u64);
            // 80522 lea rdi,[rbp-0x1390]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec70);
            // 80529 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8052e add rsp,0x1378
            { let (r, f) = add::<64>(rsp, 0x1378u64);
              rsp = r;
              fl = f; }
            // 80535 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 80536 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 80538 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 8053a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8053b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x8053c => {
            // block 8053c
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 8053c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8053c mov rbx,rax
            rbx = rax;
            // 8053f lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 80543 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80548 => {
            // block 80548
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80548 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80548 mov rbx,rax
            rbx = rax;
            // 8054b lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 80552 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80557 => {
            // block 80557
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80557 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80557 mov rbx,rax
            rbx = rax;
            // 8055a lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 80561 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80566 => {
            // block 80566
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80566 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80566 mov rbx,rax
            rbx = rax;
            // 80569 lea rdi,[rbp-0x268]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd98);
            // 80570 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80575 => {
            // block 80575
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80575 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80575 mov rbx,rax
            rbx = rax;
            // 80578 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 8057f jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80584 => {
            // block 80584
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80584 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80584 mov rbx,rax
            rbx = rax;
            // 80587 lea rdi,[rbp-0x3e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // 8058e jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80593 => {
            // block 80593
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80593 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80593 mov rbx,rax
            rbx = rax;
            // 80596 lea rdi,[rbp-0x4a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb58);
            // 8059d jmp 0x80966
            pc = 0x80966; continue;
        }
        0x805a2 => {
            // block 805a2
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 805a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 805a2 mov rbx,rax
            rbx = rax;
            // 805a5 lea rdi,[rbp-0x568]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa98);
            // 805ac jmp 0x80966
            pc = 0x80966; continue;
        }
        0x805b1 => {
            // block 805b1
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 805b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 805b1 mov rbx,rax
            rbx = rax;
            // 805b4 lea rdi,[rbp-0x628]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9d8);
            // 805bb jmp 0x80966
            pc = 0x80966; continue;
        }
        0x805c0 => {
            // block 805c0
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 805c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 805c0 mov rbx,rax
            rbx = rax;
            // 805c3 lea rdi,[rbp-0x6e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f918);
            // 805ca jmp 0x80966
            pc = 0x80966; continue;
        }
        0x805cf => {
            // block 805cf
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 805cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 805cf mov rbx,rax
            rbx = rax;
            // 805d2 lea rdi,[rbp-0x7a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f858);
            // 805d9 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x805de => {
            // block 805de
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 805de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 805de mov rbx,rax
            rbx = rax;
            // 805e1 lea rdi,[rbp-0x868]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f798);
            // 805e8 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x805ed => {
            // block 805ed
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 805ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 805ed mov rbx,rax
            rbx = rax;
            // 805f0 lea rdi,[rbp-0x928]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6d8);
            // 805f7 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x805fc => {
            // block 805fc
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 805fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 805fc mov rbx,rax
            rbx = rax;
            // 805ff lea rdi,[rbp-0x9e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f618);
            // 80606 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x8060b => {
            // block 8060b
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 8060b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8060b mov rbx,rax
            rbx = rax;
            // 8060e lea rdi,[rbp-0xaa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f558);
            // 80615 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x8061a => {
            // block 8061a
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 8061a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8061a mov rbx,rax
            rbx = rax;
            // 8061d lea rdi,[rbp-0xb68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f498);
            // 80624 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80629 => {
            // block 80629
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80629 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80629 mov rbx,rax
            rbx = rax;
            // 8062c lea rdi,[rbp-0xc28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3d8);
            // 80633 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80638 => {
            // block 80638
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80638 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80638 mov rbx,rax
            rbx = rax;
            // 8063b lea rdi,[rbp-0xce8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f318);
            // 80642 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80647 => {
            // block 80647
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80647 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80647 mov rbx,rax
            rbx = rax;
            // 8064a lea rdi,[rbp-0xda8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f258);
            // 80651 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80656 => {
            // block 80656
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80656 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80656 mov rbx,rax
            rbx = rax;
            // 80659 lea rdi,[rbp-0xe68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f198);
            // 80660 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80665 => {
            // block 80665
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80665 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80665 mov rbx,rax
            rbx = rax;
            // 80668 lea rdi,[rbp-0xf28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0d8);
            // 8066f jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80674 => {
            // block 80674
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80674 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80674 mov rbx,rax
            rbx = rax;
            // 80677 lea rdi,[rbp-0xfe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f018);
            // 8067e jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80683 => {
            // block 80683
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80683 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80683 mov rbx,rax
            rbx = rax;
            // 80686 lea rdi,[rbp-0x10a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef58);
            // 8068d jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80692 => {
            // block 80692
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80692 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80692 mov rbx,rax
            rbx = rax;
            // 80695 lea rdi,[rbp-0x1168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee98);
            // 8069c jmp 0x80966
            pc = 0x80966; continue;
        }
        0x806a1 => {
            // block 806a1
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 806a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 806a1 mov rbx,rax
            rbx = rax;
            // 806a4 lea rdi,[rbp-0x1228]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edd8);
            // 806ab jmp 0x80966
            pc = 0x80966; continue;
        }
        0x806b0 => {
            // block 806b0
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 806b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 806b0 mov rbx,rax
            rbx = rax;
            // 806b3 lea rdi,[rbp-0x12e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed18);
            // 806ba jmp 0x80966
            pc = 0x80966; continue;
        }
        0x806bf => {
            // block 806bf
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 806bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 806bf mov rbx,rax
            rbx = rax;
            // 806c2 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 806c6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 806cb lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 806d2 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x806d7 => {
            // block 806d7
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 806d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 806d7 mov rbx,rax
            rbx = rax;
            // 806da lea rdi,[rbp-0x130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 806e1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 806e6 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 806ed jmp 0x80966
            pc = 0x80966; continue;
        }
        0x806f2 => {
            // block 806f2
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 806f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 806f2 mov rbx,rax
            rbx = rax;
            // 806f5 lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 806fc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80701 lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 80708 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x8070d => {
            // block 8070d
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 8070d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8070d mov rbx,rax
            rbx = rax;
            // 80710 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // 80717 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8071c lea rdi,[rbp-0x2d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd30);
            // 80723 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80728 => {
            // block 80728
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80728 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80728 mov rbx,rax
            rbx = rax;
            // 8072b lea rdi,[rbp-0x370]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc90);
            // 80732 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80737 lea rdi,[rbp-0x390]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc70);
            // 8073e jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80743 => {
            // block 80743
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80743 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80743 mov rbx,rax
            rbx = rax;
            // 80746 lea rdi,[rbp-0x430]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd0);
            // 8074d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80752 lea rdi,[rbp-0x450]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb0);
            // 80759 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x8075e => {
            // block 8075e
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 8075e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8075e mov rbx,rax
            rbx = rax;
            // 80761 lea rdi,[rbp-0x4f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb10);
            // 80768 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8076d lea rdi,[rbp-0x510]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_faf0);
            // 80774 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80779 => {
            // block 80779
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80779 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80779 mov rbx,rax
            rbx = rax;
            // 8077c lea rdi,[rbp-0x5b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa50);
            // 80783 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80788 lea rdi,[rbp-0x5d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa30);
            // 8078f jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80794 => {
            // block 80794
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80794 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80794 mov rbx,rax
            rbx = rax;
            // 80797 lea rdi,[rbp-0x670]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f990);
            // 8079e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 807a3 lea rdi,[rbp-0x690]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f970);
            // 807aa jmp 0x80966
            pc = 0x80966; continue;
        }
        0x807af => {
            // block 807af
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 807af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 807af mov rbx,rax
            rbx = rax;
            // 807b2 lea rdi,[rbp-0x730]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8d0);
            // 807b9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 807be lea rdi,[rbp-0x750]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8b0);
            // 807c5 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x807ca => {
            // block 807ca
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 807ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 807ca mov rbx,rax
            rbx = rax;
            // 807cd lea rdi,[rbp-0x7f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f810);
            // 807d4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 807d9 lea rdi,[rbp-0x810]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7f0);
            // 807e0 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x807e5 => {
            // block 807e5
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 807e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 807e5 mov rbx,rax
            rbx = rax;
            // 807e8 lea rdi,[rbp-0x8b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f750);
            // 807ef call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 807f4 lea rdi,[rbp-0x8d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f730);
            // 807fb jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80800 => {
            // block 80800
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80800 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80800 mov rbx,rax
            rbx = rax;
            // 80803 lea rdi,[rbp-0x970]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f690);
            // 8080a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8080f lea rdi,[rbp-0x990]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f670);
            // 80816 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x8081b => {
            // block 8081b
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 8081b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8081b mov rbx,rax
            rbx = rax;
            // 8081e lea rdi,[rbp-0xa30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5d0);
            // 80825 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8082a lea rdi,[rbp-0xa50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5b0);
            // 80831 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80836 => {
            // block 80836
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80836 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80836 mov rbx,rax
            rbx = rax;
            // 80839 lea rdi,[rbp-0xaf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f510);
            // 80840 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80845 lea rdi,[rbp-0xb10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4f0);
            // 8084c jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80851 => {
            // block 80851
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80851 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80851 mov rbx,rax
            rbx = rax;
            // 80854 lea rdi,[rbp-0xbb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f450);
            // 8085b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80860 lea rdi,[rbp-0xbd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f430);
            // 80867 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x8086c => {
            // block 8086c
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 8086c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8086c mov rbx,rax
            rbx = rax;
            // 8086f lea rdi,[rbp-0xc70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f390);
            // 80876 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8087b lea rdi,[rbp-0xc90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f370);
            // 80882 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80887 => {
            // block 80887
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80887 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80887 mov rbx,rax
            rbx = rax;
            // 8088a lea rdi,[rbp-0xd30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2d0);
            // 80891 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80896 lea rdi,[rbp-0xd50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2b0);
            // 8089d jmp 0x80966
            pc = 0x80966; continue;
        }
        0x808a2 => {
            // block 808a2
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 808a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 808a2 mov rbx,rax
            rbx = rax;
            // 808a5 lea rdi,[rbp-0xdf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f210);
            // 808ac call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 808b1 lea rdi,[rbp-0xe10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1f0);
            // 808b8 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x808bd => {
            // block 808bd
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 808bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 808bd mov rbx,rax
            rbx = rax;
            // 808c0 lea rdi,[rbp-0xeb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f150);
            // 808c7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 808cc lea rdi,[rbp-0xed0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f130);
            // 808d3 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x808d8 => {
            // block 808d8
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 808d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 808d8 mov rbx,rax
            rbx = rax;
            // 808db lea rdi,[rbp-0xf70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f090);
            // 808e2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 808e7 lea rdi,[rbp-0xf90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f070);
            // 808ee jmp 0x80966
            pc = 0x80966; continue;
        }
        0x808f0 => {
            // block 808f0
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 808f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 808f0 mov rbx,rax
            rbx = rax;
            // 808f3 lea rdi,[rbp-0x1030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efd0);
            // 808fa call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 808ff lea rdi,[rbp-0x1050]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efb0);
            // 80906 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80908 => {
            // block 80908
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80908 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80908 mov rbx,rax
            rbx = rax;
            // 8090b lea rdi,[rbp-0x10f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef10);
            // 80912 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80917 lea rdi,[rbp-0x1110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eef0);
            // 8091e jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80920 => {
            // block 80920
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80920 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80920 mov rbx,rax
            rbx = rax;
            // 80923 lea rdi,[rbp-0x11b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee50);
            // 8092a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8092f lea rdi,[rbp-0x11d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee30);
            // 80936 jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80938 => {
            // block 80938
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80938 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80938 mov rbx,rax
            rbx = rax;
            // 8093b lea rdi,[rbp-0x1270]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed90);
            // 80942 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80947 lea rdi,[rbp-0x1290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed70);
            // 8094e jmp 0x80966
            pc = 0x80966; continue;
        }
        0x80950 => {
            // block 80950
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80950 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80950 mov rbx,rax
            rbx = rax;
            // 80953 lea rdi,[rbp-0x1330]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecd0);
            // 8095a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8095f lea rdi,[rbp-0x1350]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecb0);
            pc = 0x80966;
        }
        0x80966 => {
            // block 80966
            #[cfg(feature = "trace")] eprintln!("f_0007e0c4 pc 80966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80966 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8096b mov rdi,rbx
            rdi = rbx;
            // 8096e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
