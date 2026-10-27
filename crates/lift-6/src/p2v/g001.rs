//! Mechanically lifted functions (scripts/lift_run.py), module g001. Each one is checked against the original
//! in the world harness (tests/world_g001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x951bc size=17544
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000951bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x951bc;
    loop {
        match pc {
        0x951bc => {
            // block 951bc
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 951bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 951bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 951bd mov rbp,rsp
            rbp = rsp;
            // 951c0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 951c2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 951c4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 951c6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 951c7 sub rsp,0x1a10
            { let (r, f) = sub::<64>(rsp, 0x1a10u64);
              rsp = r;
              fl = f; }
            // 951ce mov r15,rdi
            r15 = rdi;
            // 951d1 lea rsi,[rip+0xa3af0] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 951d8 lea rbx,[rbp-0x38]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 951dc xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 951de mov rdi,rbx
            rdi = rbx;
            // 951e1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 951e6 mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // 951eb mov rdi,r15
            rdi = r15;
            // 951ee mov rdx,rbx
            rdx = rbx;
            // 951f1 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 951f6 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 951fa call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 951ff lea rax,[r15+0x100]
            rax = r15.wrapping_add(0x100);
            // 95206 mov DWORD PTR [rbp-0x48],0x7d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0x7du64) as u32);
            // 9520d mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 95211 lea r14,[r15+0x180]
            r14 = r15.wrapping_add(0x180);
            // 95218 lea rsi,[rbp-0x48]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 9521c mov rdi,r14
            rdi = r14;
            // 9521f call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95224 lea rsi,[rip+0xa0380] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 9522b lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 9522f mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 95234 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95239 movzx ebx,BYTE PTR [rbp-0x60]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // 9523d test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 95240 jne 0x95247
            if crate::cov::brk(0x95240, fl.ne()) { pc = 0x95247; continue; }
            pc = 0x95242;
        }
        0x95242 => {
            // block 95242
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95242 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95242 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 95245 jmp 0x9524b
            pc = 0x9524b; continue;
        }
        0x95247 => {
            // block 95247
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95247 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95247 mov rbx,QWORD PTR [rbp-0x58]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x9524b;
        }
        0x9524b => {
            // block 9524b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9524b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9524b lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 9524f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95254 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 95257 je 0x952d3
            if crate::cov::brk(0x95257, fl.e()) { pc = 0x952d3; continue; }
            pc = 0x95259;
        }
        0x95259 => {
            // block 95259
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95259 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95259 lea rsi,[rip+0xa034b] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 95260 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 95267 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 9526c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95271 mov rax,QWORD PTR [rbp-0x90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 95278 mov QWORD PTR [rbp-0x70],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rax) as u64);
            // 9527c mov rax,QWORD PTR [rbp-0xa0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 95283 mov rcx,QWORD PTR [rbp-0x98]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 9528a mov QWORD PTR [rbp-0x78],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rcx) as u64);
            // 9528e mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 95292 xorps xmm0,xmm0
            x0 = 0;
            // 95295 movaps XMMWORD PTR [rbp-0xa0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff60), x0);
            // 9529c mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 952a7 mov DWORD PTR [rbp-0x68],0x7d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0x7du64) as u32);
            // 952ae lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 952b5 lea rsi,[rbp-0x80]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 952b9 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 952be lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 952c2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 952c7 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 952ce call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x952d3;
        }
        0x952d3 => {
            // block 952d3
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 952d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 952d3 lea rsi,[rip+0x9e6ff] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 952da lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 952e1 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 952e6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 952eb movzx ebx,BYTE PTR [rbp-0xb8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64) as u32 as u64;
            // 952f2 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 952f5 jne 0x952fc
            if crate::cov::brk(0x952f5, fl.ne()) { pc = 0x952fc; continue; }
            pc = 0x952f7;
        }
        0x952f7 => {
            // block 952f7
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 952f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 952f7 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 952fa jmp 0x95303
            pc = 0x95303; continue;
        }
        0x952fc => {
            // block 952fc
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 952fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 952fc mov rbx,QWORD PTR [rbp-0xb0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            pc = 0x95303;
        }
        0x95303 => {
            // block 95303
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95303 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95303 lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 9530a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9530f test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 95312 je 0x953a4
            if crate::cov::brk(0x95312, fl.e()) { pc = 0x953a4; continue; }
            pc = 0x95318;
        }
        0x95318 => {
            // block 95318
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95318 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95318 lea rsi,[rip+0x9e6ba] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 9531f lea rdi,[rbp-0xf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 95326 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 9532b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95330 mov rax,QWORD PTR [rbp-0xe0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff20)) as u64;
            // 95337 mov QWORD PTR [rbp-0xc8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (rax) as u64);
            // 9533e mov rax,QWORD PTR [rbp-0xf0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff10)) as u64;
            // 95345 mov rcx,QWORD PTR [rbp-0xe8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff18)) as u64;
            // 9534c mov QWORD PTR [rbp-0xd0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (rcx) as u64);
            // 95353 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 9535a xorps xmm0,xmm0
            x0 = 0;
            // 9535d movaps XMMWORD PTR [rbp-0xf0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff10), x0);
            // 95364 mov QWORD PTR [rbp-0xe0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x0u64) as u64);
            // 9536f mov DWORD PTR [rbp-0xc0],0x7d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x7du64) as u32);
            // 95379 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 95380 lea rsi,[rbp-0xd8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 95387 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9538c lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 95393 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95398 lea rdi,[rbp-0xf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 9539f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x953a4;
        }
        0x953a4 => {
            // block 953a4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 953a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 953a4 lea rsi,[rip+0x9e596] # 0x133941
            rsi = m.env.data_va(295182);
            // 953ab lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 953b2 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 953b7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 953bc movzx ebx,BYTE PTR [rbp-0x108]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fef8)) as u64) as u32 as u64;
            // 953c3 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 953c6 jne 0x953cd
            if crate::cov::brk(0x953c6, fl.ne()) { pc = 0x953cd; continue; }
            pc = 0x953c8;
        }
        0x953c8 => {
            // block 953c8
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 953c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 953c8 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 953cb jmp 0x953d4
            pc = 0x953d4; continue;
        }
        0x953cd => {
            // block 953cd
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 953cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 953cd mov rbx,QWORD PTR [rbp-0x100]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff00)) as u64;
            pc = 0x953d4;
        }
        0x953d4 => {
            // block 953d4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 953d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 953d4 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 953db call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 953e0 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 953e3 je 0x95475
            if crate::cov::brk(0x953e3, fl.e()) { pc = 0x95475; continue; }
            pc = 0x953e9;
        }
        0x953e9 => {
            // block 953e9
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 953e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 953e9 lea rsi,[rip+0x9e551] # 0x133941
            rsi = m.env.data_va(295182);
            // 953f0 lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 953f7 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 953fc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95401 mov rax,QWORD PTR [rbp-0x130]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 95408 mov QWORD PTR [rbp-0x118],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rax) as u64);
            // 9540f mov rax,QWORD PTR [rbp-0x140]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 95416 mov rcx,QWORD PTR [rbp-0x138]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 9541d mov QWORD PTR [rbp-0x120],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rcx) as u64);
            // 95424 mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // 9542b xorps xmm0,xmm0
            x0 = 0;
            // 9542e movaps XMMWORD PTR [rbp-0x140],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fec0), x0);
            // 95435 mov QWORD PTR [rbp-0x130],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (0x0u64) as u64);
            // 95440 mov DWORD PTR [rbp-0x110],0x7d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x7du64) as u32);
            // 9544a lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 95451 lea rsi,[rbp-0x128]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 95458 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9545d lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 95464 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95469 lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 95470 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x95475;
        }
        0x95475 => {
            // block 95475
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95475 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95475 lea rsi,[rip+0xa384c] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 9547c lea rbx,[rbp-0x158]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 95483 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 95485 mov rdi,rbx
            rdi = rbx;
            // 95488 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9548d mov esi,0x7e
            rsi = (0x7eu64) as u32 as u64;
            // 95492 mov rdi,r15
            rdi = r15;
            // 95495 mov rdx,rbx
            rdx = rbx;
            // 95498 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9549d lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 954a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 954a9 lea rax,[r15+0x108]
            rax = r15.wrapping_add(0x108);
            // 954b0 mov DWORD PTR [rbp-0x168],0x7e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (0x7eu64) as u32);
            // 954ba mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // 954c1 lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 954c8 mov rdi,r14
            rdi = r14;
            // 954cb call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 954d0 lea rsi,[rip+0xa00d4] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 954d7 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 954de mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 954e3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 954e8 movzx ebx,BYTE PTR [rbp-0x180]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64) as u32 as u64;
            // 954ef test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 954f2 jne 0x954f9
            if crate::cov::brk(0x954f2, fl.ne()) { pc = 0x954f9; continue; }
            pc = 0x954f4;
        }
        0x954f4 => {
            // block 954f4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 954f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 954f4 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 954f7 jmp 0x95500
            pc = 0x95500; continue;
        }
        0x954f9 => {
            // block 954f9
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 954f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 954f9 mov rbx,QWORD PTR [rbp-0x178]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x95500;
        }
        0x95500 => {
            // block 95500
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95500 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95500 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 95507 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9550c test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9550f je 0x955a1
            if crate::cov::brk(0x9550f, fl.e()) { pc = 0x955a1; continue; }
            pc = 0x95515;
        }
        0x95515 => {
            // block 95515
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95515 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95515 lea rsi,[rip+0xa008f] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 9551c lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 95523 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 95528 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9552d mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 95534 mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // 9553b mov rax,QWORD PTR [rbp-0x1c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // 95542 mov rcx,QWORD PTR [rbp-0x1b8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            // 95549 mov QWORD PTR [rbp-0x198],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rcx) as u64);
            // 95550 mov QWORD PTR [rbp-0x1a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe60), (rax) as u64);
            // 95557 xorps xmm0,xmm0
            x0 = 0;
            // 9555a movaps XMMWORD PTR [rbp-0x1c0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe40), x0);
            // 95561 mov QWORD PTR [rbp-0x1b0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (0x0u64) as u64);
            // 9556c mov DWORD PTR [rbp-0x188],0x7e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (0x7eu64) as u32);
            // 95576 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 9557d lea rsi,[rbp-0x1a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 95584 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95589 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 95590 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95595 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 9559c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x955a1;
        }
        0x955a1 => {
            // block 955a1
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 955a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 955a1 lea rsi,[rip+0x9e463] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 955a8 lea rdi,[rbp-0x1d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe28);
            // 955af mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 955b4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 955b9 movzx ebx,BYTE PTR [rbp-0x1d8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64) as u32 as u64;
            // 955c0 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 955c3 jne 0x955ca
            if crate::cov::brk(0x955c3, fl.ne()) { pc = 0x955ca; continue; }
            pc = 0x955c5;
        }
        0x955c5 => {
            // block 955c5
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 955c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 955c5 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 955c8 jmp 0x955d1
            pc = 0x955d1; continue;
        }
        0x955ca => {
            // block 955ca
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 955ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 955ca mov rbx,QWORD PTR [rbp-0x1d0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            pc = 0x955d1;
        }
        0x955d1 => {
            // block 955d1
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 955d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 955d1 lea rdi,[rbp-0x1d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe28);
            // 955d8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 955dd test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 955e0 je 0x95672
            if crate::cov::brk(0x955e0, fl.e()) { pc = 0x95672; continue; }
            pc = 0x955e6;
        }
        0x955e6 => {
            // block 955e6
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 955e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 955e6 lea rsi,[rip+0x9e41e] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 955ed lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 955f4 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 955f9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 955fe mov rax,QWORD PTR [rbp-0x200]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe00)) as u64;
            // 95605 mov QWORD PTR [rbp-0x1e8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe18), (rax) as u64);
            // 9560c mov rax,QWORD PTR [rbp-0x210]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdf0)) as u64;
            // 95613 mov rcx,QWORD PTR [rbp-0x208]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdf8)) as u64;
            // 9561a mov QWORD PTR [rbp-0x1f0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (rcx) as u64);
            // 95621 mov QWORD PTR [rbp-0x1f8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe08), (rax) as u64);
            // 95628 xorps xmm0,xmm0
            x0 = 0;
            // 9562b movaps XMMWORD PTR [rbp-0x210],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fdf0), x0);
            // 95632 mov QWORD PTR [rbp-0x200],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe00), (0x0u64) as u64);
            // 9563d mov DWORD PTR [rbp-0x1e0],0x7e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe20), (0x7eu64) as u32);
            // 95647 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 9564e lea rsi,[rbp-0x1f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe08);
            // 95655 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9565a lea rdi,[rbp-0x1f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe08);
            // 95661 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95666 lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 9566d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x95672;
        }
        0x95672 => {
            // block 95672
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95672 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95672 lea rsi,[rip+0x9e2c8] # 0x133941
            rsi = m.env.data_va(295182);
            // 95679 lea rdi,[rbp-0x228]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdd8);
            // 95680 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 95685 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9568a movzx ebx,BYTE PTR [rbp-0x228]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fdd8)) as u64) as u32 as u64;
            // 95691 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 95694 jne 0x9569b
            if crate::cov::brk(0x95694, fl.ne()) { pc = 0x9569b; continue; }
            pc = 0x95696;
        }
        0x95696 => {
            // block 95696
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95696 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95696 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 95699 jmp 0x956a2
            pc = 0x956a2; continue;
        }
        0x9569b => {
            // block 9569b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9569b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9569b mov rbx,QWORD PTR [rbp-0x220]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde0)) as u64;
            pc = 0x956a2;
        }
        0x956a2 => {
            // block 956a2
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 956a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 956a2 lea rdi,[rbp-0x228]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdd8);
            // 956a9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 956ae test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 956b1 je 0x95743
            if crate::cov::brk(0x956b1, fl.e()) { pc = 0x95743; continue; }
            pc = 0x956b7;
        }
        0x956b7 => {
            // block 956b7
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 956b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 956b7 lea rsi,[rip+0x9e283] # 0x133941
            rsi = m.env.data_va(295182);
            // 956be lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 956c5 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 956ca call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 956cf mov rax,QWORD PTR [rbp-0x250]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdb0)) as u64;
            // 956d6 mov QWORD PTR [rbp-0x238],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (rax) as u64);
            // 956dd mov rax,QWORD PTR [rbp-0x260]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fda0)) as u64;
            // 956e4 mov rcx,QWORD PTR [rbp-0x258]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fda8)) as u64;
            // 956eb mov QWORD PTR [rbp-0x240],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc0), (rcx) as u64);
            // 956f2 mov QWORD PTR [rbp-0x248],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb8), (rax) as u64);
            // 956f9 xorps xmm0,xmm0
            x0 = 0;
            // 956fc movaps XMMWORD PTR [rbp-0x260],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fda0), x0);
            // 95703 mov QWORD PTR [rbp-0x250],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb0), (0x0u64) as u64);
            // 9570e mov DWORD PTR [rbp-0x230],0x7e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fdd0), (0x7eu64) as u32);
            // 95718 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 9571f lea rsi,[rbp-0x248]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fdb8);
            // 95726 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9572b lea rdi,[rbp-0x248]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdb8);
            // 95732 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95737 lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 9573e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x95743;
        }
        0x95743 => {
            // block 95743
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95743 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95743 lea rsi,[rip+0xa357e] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 9574a lea rbx,[rbp-0x278]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd88);
            // 95751 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 95753 mov rdi,rbx
            rdi = rbx;
            // 95756 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9575b mov esi,0x14a
            rsi = (0x14au64) as u32 as u64;
            // 95760 mov rdi,r15
            rdi = r15;
            // 95763 mov rdx,rbx
            rdx = rbx;
            // 95766 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9576b lea rdi,[rbp-0x278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd88);
            // 95772 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95777 lea r12,[r15+0x110]
            r12 = r15.wrapping_add(0x110);
            // 9577e mov DWORD PTR [rbp-0x288],0x14a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd78), (0x14au64) as u32);
            // 95788 mov QWORD PTR [rbp-0x280],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd80), (r12) as u64);
            // 9578f lea rsi,[rbp-0x288]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd78);
            // 95796 mov rdi,r14
            rdi = r14;
            // 95799 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9579e lea rsi,[rip+0x9fe06] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 957a5 lea rdi,[rbp-0x2a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // 957ac mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 957b1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 957b6 movzx ebx,BYTE PTR [rbp-0x2a0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd60)) as u64) as u32 as u64;
            // 957bd test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 957c0 jne 0x957c7
            if crate::cov::brk(0x957c0, fl.ne()) { pc = 0x957c7; continue; }
            pc = 0x957c2;
        }
        0x957c2 => {
            // block 957c2
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 957c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 957c2 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 957c5 jmp 0x957ce
            pc = 0x957ce; continue;
        }
        0x957c7 => {
            // block 957c7
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 957c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 957c7 mov rbx,QWORD PTR [rbp-0x298]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd68)) as u64;
            pc = 0x957ce;
        }
        0x957ce => {
            // block 957ce
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 957ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 957ce lea rdi,[rbp-0x2a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // 957d5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 957da test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 957dd je 0x9586f
            if crate::cov::brk(0x957dd, fl.e()) { pc = 0x9586f; continue; }
            pc = 0x957e3;
        }
        0x957e3 => {
            // block 957e3
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 957e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 957e3 lea rsi,[rip+0x9fdc1] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 957ea lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 957f1 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 957f6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 957fb mov rax,QWORD PTR [rbp-0x2d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd30)) as u64;
            // 95802 mov QWORD PTR [rbp-0x2b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd50), (rax) as u64);
            // 95809 mov rax,QWORD PTR [rbp-0x2e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd20)) as u64;
            // 95810 mov rcx,QWORD PTR [rbp-0x2d8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd28)) as u64;
            // 95817 mov QWORD PTR [rbp-0x2b8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd48), (rcx) as u64);
            // 9581e mov QWORD PTR [rbp-0x2c0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd40), (rax) as u64);
            // 95825 xorps xmm0,xmm0
            x0 = 0;
            // 95828 movaps XMMWORD PTR [rbp-0x2e0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fd20), x0);
            // 9582f mov QWORD PTR [rbp-0x2d0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd30), (0x0u64) as u64);
            // 9583a mov DWORD PTR [rbp-0x2a8],0x14a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (0x14au64) as u32);
            // 95844 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 9584b lea rsi,[rbp-0x2c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 95852 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95857 lea rdi,[rbp-0x2c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 9585e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95863 lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 9586a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x9586f;
        }
        0x9586f => {
            // block 9586f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9586f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9586f lea rsi,[rip+0x9e163] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 95876 lea rdi,[rbp-0x2f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd08);
            // 9587d mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 95882 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95887 movzx ebx,BYTE PTR [rbp-0x2f8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd08)) as u64) as u32 as u64;
            // 9588e test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 95891 jne 0x95898
            if crate::cov::brk(0x95891, fl.ne()) { pc = 0x95898; continue; }
            pc = 0x95893;
        }
        0x95893 => {
            // block 95893
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95893 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95893 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 95896 jmp 0x9589f
            pc = 0x9589f; continue;
        }
        0x95898 => {
            // block 95898
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95898 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95898 mov rbx,QWORD PTR [rbp-0x2f0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd10)) as u64;
            pc = 0x9589f;
        }
        0x9589f => {
            // block 9589f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9589f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9589f lea rdi,[rbp-0x2f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd08);
            // 958a6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 958ab test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 958ae je 0x95940
            if crate::cov::brk(0x958ae, fl.e()) { pc = 0x95940; continue; }
            pc = 0x958b4;
        }
        0x958b4 => {
            // block 958b4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 958b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 958b4 lea rsi,[rip+0x9e11e] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 958bb lea rdi,[rbp-0x330]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd0);
            // 958c2 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 958c7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 958cc mov rax,QWORD PTR [rbp-0x320]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce0)) as u64;
            // 958d3 mov QWORD PTR [rbp-0x308],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcf8), (rax) as u64);
            // 958da mov rax,QWORD PTR [rbp-0x330]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64;
            // 958e1 mov rcx,QWORD PTR [rbp-0x328]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd8)) as u64;
            // 958e8 mov QWORD PTR [rbp-0x310],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0), (rcx) as u64);
            // 958ef mov QWORD PTR [rbp-0x318],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce8), (rax) as u64);
            // 958f6 xorps xmm0,xmm0
            x0 = 0;
            // 958f9 movaps XMMWORD PTR [rbp-0x330],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fcd0), x0);
            // 95900 mov QWORD PTR [rbp-0x320],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce0), (0x0u64) as u64);
            // 9590b mov DWORD PTR [rbp-0x300],0x14a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd00), (0x14au64) as u32);
            // 95915 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 9591c lea rsi,[rbp-0x318]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fce8);
            // 95923 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95928 lea rdi,[rbp-0x318]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fce8);
            // 9592f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95934 lea rdi,[rbp-0x330]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd0);
            // 9593b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x95940;
        }
        0x95940 => {
            // block 95940
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95940 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95940 lea rsi,[rip+0x9feb8] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 95947 lea rdi,[rbp-0x348]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb8);
            // 9594e mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 95953 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95958 movzx ebx,BYTE PTR [rbp-0x348]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fcb8)) as u64) as u32 as u64;
            // 9595f test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 95962 jne 0x95969
            if crate::cov::brk(0x95962, fl.ne()) { pc = 0x95969; continue; }
            pc = 0x95964;
        }
        0x95964 => {
            // block 95964
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95964 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95964 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 95967 jmp 0x95970
            pc = 0x95970; continue;
        }
        0x95969 => {
            // block 95969
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95969 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95969 mov rbx,QWORD PTR [rbp-0x340]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcc0)) as u64;
            pc = 0x95970;
        }
        0x95970 => {
            // block 95970
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95970 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95970 lea rdi,[rbp-0x348]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb8);
            // 95977 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9597c test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9597f je 0x95a11
            if crate::cov::brk(0x9597f, fl.e()) { pc = 0x95a11; continue; }
            pc = 0x95985;
        }
        0x95985 => {
            // block 95985
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95985 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95985 lea rsi,[rip+0x9fe73] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 9598c lea rdi,[rbp-0x380]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc80);
            // 95993 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 95998 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9599d mov rax,QWORD PTR [rbp-0x370]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc90)) as u64;
            // 959a4 mov QWORD PTR [rbp-0x358],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fca8), (rax) as u64);
            // 959ab mov rax,QWORD PTR [rbp-0x380]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc80)) as u64;
            // 959b2 mov rcx,QWORD PTR [rbp-0x378]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc88)) as u64;
            // 959b9 mov QWORD PTR [rbp-0x360],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fca0), (rcx) as u64);
            // 959c0 mov QWORD PTR [rbp-0x368],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc98), (rax) as u64);
            // 959c7 xorps xmm0,xmm0
            x0 = 0;
            // 959ca movaps XMMWORD PTR [rbp-0x380],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fc80), x0);
            // 959d1 mov QWORD PTR [rbp-0x370],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc90), (0x0u64) as u64);
            // 959dc mov DWORD PTR [rbp-0x350],0x14a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fcb0), (0x14au64) as u32);
            // 959e6 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 959ed lea rsi,[rbp-0x368]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fc98);
            // 959f4 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 959f9 lea rdi,[rbp-0x368]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc98);
            // 95a00 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95a05 lea rdi,[rbp-0x380]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc80);
            // 95a0c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x95a11;
        }
        0x95a11 => {
            // block 95a11
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95a11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95a11 lea rsi,[rip+0xa32b0] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 95a18 lea rbx,[rbp-0x398]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc68);
            // 95a1f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 95a21 mov rdi,rbx
            rdi = rbx;
            // 95a24 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95a29 mov esi,0x14b
            rsi = (0x14bu64) as u32 as u64;
            // 95a2e mov rdi,r15
            rdi = r15;
            // 95a31 mov rdx,rbx
            rdx = rbx;
            // 95a34 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95a39 lea rdi,[rbp-0x398]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc68);
            // 95a40 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95a45 mov DWORD PTR [rbp-0x3a8],0x14b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc58), (0x14bu64) as u32);
            // 95a4f mov QWORD PTR [rbp-0x3a0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc60), (r12) as u64);
            // 95a56 lea rsi,[rbp-0x3a8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fc58);
            // 95a5d mov rdi,r14
            rdi = r14;
            // 95a60 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95a65 lea rsi,[rip+0x9fb3f] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 95a6c lea rdi,[rbp-0x3c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc40);
            // 95a73 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 95a78 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95a7d movzx ebx,BYTE PTR [rbp-0x3c0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fc40)) as u64) as u32 as u64;
            // 95a84 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 95a87 jne 0x95a8e
            if crate::cov::brk(0x95a87, fl.ne()) { pc = 0x95a8e; continue; }
            pc = 0x95a89;
        }
        0x95a89 => {
            // block 95a89
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95a89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95a89 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 95a8c jmp 0x95a95
            pc = 0x95a95; continue;
        }
        0x95a8e => {
            // block 95a8e
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95a8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95a8e mov rbx,QWORD PTR [rbp-0x3b8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc48)) as u64;
            pc = 0x95a95;
        }
        0x95a95 => {
            // block 95a95
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95a95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95a95 lea rdi,[rbp-0x3c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc40);
            // 95a9c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95aa1 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 95aa4 je 0x95b36
            if crate::cov::brk(0x95aa4, fl.e()) { pc = 0x95b36; continue; }
            pc = 0x95aaa;
        }
        0x95aaa => {
            // block 95aaa
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95aaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95aaa lea rsi,[rip+0x9fafa] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 95ab1 lea rdi,[rbp-0x400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 95ab8 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 95abd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95ac2 mov rax,QWORD PTR [rbp-0x3f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc10)) as u64;
            // 95ac9 mov QWORD PTR [rbp-0x3d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc30), (rax) as u64);
            // 95ad0 mov rax,QWORD PTR [rbp-0x400]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc00)) as u64;
            // 95ad7 mov rcx,QWORD PTR [rbp-0x3f8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc08)) as u64;
            // 95ade mov QWORD PTR [rbp-0x3d8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc28), (rcx) as u64);
            // 95ae5 mov QWORD PTR [rbp-0x3e0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc20), (rax) as u64);
            // 95aec xorps xmm0,xmm0
            x0 = 0;
            // 95aef movaps XMMWORD PTR [rbp-0x400],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fc00), x0);
            // 95af6 mov QWORD PTR [rbp-0x3f0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc10), (0x0u64) as u64);
            // 95b01 mov DWORD PTR [rbp-0x3c8],0x14b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc38), (0x14bu64) as u32);
            // 95b0b lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 95b12 lea rsi,[rbp-0x3e0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fc20);
            // 95b19 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95b1e lea rdi,[rbp-0x3e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc20);
            // 95b25 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95b2a lea rdi,[rbp-0x400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 95b31 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x95b36;
        }
        0x95b36 => {
            // block 95b36
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95b36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95b36 lea rsi,[rip+0x9de9c] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 95b3d lea rdi,[rbp-0x418]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbe8);
            // 95b44 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 95b49 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95b4e movzx ebx,BYTE PTR [rbp-0x418]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fbe8)) as u64) as u32 as u64;
            // 95b55 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 95b58 jne 0x95b5f
            if crate::cov::brk(0x95b58, fl.ne()) { pc = 0x95b5f; continue; }
            pc = 0x95b5a;
        }
        0x95b5a => {
            // block 95b5a
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95b5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95b5a shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 95b5d jmp 0x95b66
            pc = 0x95b66; continue;
        }
        0x95b5f => {
            // block 95b5f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95b5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95b5f mov rbx,QWORD PTR [rbp-0x410]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbf0)) as u64;
            pc = 0x95b66;
        }
        0x95b66 => {
            // block 95b66
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95b66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95b66 lea rdi,[rbp-0x418]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbe8);
            // 95b6d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95b72 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 95b75 je 0x95c07
            if crate::cov::brk(0x95b75, fl.e()) { pc = 0x95c07; continue; }
            pc = 0x95b7b;
        }
        0x95b7b => {
            // block 95b7b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95b7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95b7b lea rsi,[rip+0x9de57] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 95b82 lea rdi,[rbp-0x450]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb0);
            // 95b89 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 95b8e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95b93 mov rax,QWORD PTR [rbp-0x440]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbc0)) as u64;
            // 95b9a mov QWORD PTR [rbp-0x428],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbd8), (rax) as u64);
            // 95ba1 mov rax,QWORD PTR [rbp-0x450]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbb0)) as u64;
            // 95ba8 mov rcx,QWORD PTR [rbp-0x448]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbb8)) as u64;
            // 95baf mov QWORD PTR [rbp-0x430],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbd0), (rcx) as u64);
            // 95bb6 mov QWORD PTR [rbp-0x438],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc8), (rax) as u64);
            // 95bbd xorps xmm0,xmm0
            x0 = 0;
            // 95bc0 movaps XMMWORD PTR [rbp-0x450],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fbb0), x0);
            // 95bc7 mov QWORD PTR [rbp-0x440],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc0), (0x0u64) as u64);
            // 95bd2 mov DWORD PTR [rbp-0x420],0x14b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fbe0), (0x14bu64) as u32);
            // 95bdc lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 95be3 lea rsi,[rbp-0x438]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fbc8);
            // 95bea call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95bef lea rdi,[rbp-0x438]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbc8);
            // 95bf6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95bfb lea rdi,[rbp-0x450]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb0);
            // 95c02 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x95c07;
        }
        0x95c07 => {
            // block 95c07
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95c07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95c07 lea rsi,[rip+0x9fbf1] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 95c0e lea rdi,[rbp-0x468]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb98);
            // 95c15 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 95c1a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95c1f movzx ebx,BYTE PTR [rbp-0x468]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fb98)) as u64) as u32 as u64;
            // 95c26 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 95c29 jne 0x95c30
            if crate::cov::brk(0x95c29, fl.ne()) { pc = 0x95c30; continue; }
            pc = 0x95c2b;
        }
        0x95c2b => {
            // block 95c2b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95c2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95c2b shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 95c2e jmp 0x95c37
            pc = 0x95c37; continue;
        }
        0x95c30 => {
            // block 95c30
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95c30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95c30 mov rbx,QWORD PTR [rbp-0x460]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fba0)) as u64;
            pc = 0x95c37;
        }
        0x95c37 => {
            // block 95c37
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95c37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95c37 lea rdi,[rbp-0x468]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb98);
            // 95c3e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95c43 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 95c46 je 0x95cd8
            if crate::cov::brk(0x95c46, fl.e()) { pc = 0x95cd8; continue; }
            pc = 0x95c4c;
        }
        0x95c4c => {
            // block 95c4c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95c4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95c4c lea rsi,[rip+0x9fbac] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 95c53 lea rdi,[rbp-0x4a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb60);
            // 95c5a mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 95c5f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95c64 mov rax,QWORD PTR [rbp-0x490]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb70)) as u64;
            // 95c6b mov QWORD PTR [rbp-0x478],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb88), (rax) as u64);
            // 95c72 mov rax,QWORD PTR [rbp-0x4a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb60)) as u64;
            // 95c79 mov rcx,QWORD PTR [rbp-0x498]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb68)) as u64;
            // 95c80 mov QWORD PTR [rbp-0x480],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb80), (rcx) as u64);
            // 95c87 mov QWORD PTR [rbp-0x488],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb78), (rax) as u64);
            // 95c8e xorps xmm0,xmm0
            x0 = 0;
            // 95c91 movaps XMMWORD PTR [rbp-0x4a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fb60), x0);
            // 95c98 mov QWORD PTR [rbp-0x490],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb70), (0x0u64) as u64);
            // 95ca3 mov DWORD PTR [rbp-0x470],0x14b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb90), (0x14bu64) as u32);
            // 95cad lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 95cb4 lea rsi,[rbp-0x488]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb78);
            // 95cbb call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95cc0 lea rdi,[rbp-0x488]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb78);
            // 95cc7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95ccc lea rdi,[rbp-0x4a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb60);
            // 95cd3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x95cd8;
        }
        0x95cd8 => {
            // block 95cd8
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95cd8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95cd8 lea rsi,[rip+0xa2fe9] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 95cdf lea rbx,[rbp-0x4b8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb48);
            // 95ce6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 95ce8 mov rdi,rbx
            rdi = rbx;
            // 95ceb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95cf0 mov esi,0x14c
            rsi = (0x14cu64) as u32 as u64;
            // 95cf5 mov rdi,r15
            rdi = r15;
            // 95cf8 mov rdx,rbx
            rdx = rbx;
            // 95cfb call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95d00 lea rdi,[rbp-0x4b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb48);
            // 95d07 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95d0c mov DWORD PTR [rbp-0x4c8],0x14c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb38), (0x14cu64) as u32);
            // 95d16 mov QWORD PTR [rbp-0x4c0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb40), (r12) as u64);
            // 95d1d lea rsi,[rbp-0x4c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb38);
            // 95d24 mov rdi,r14
            rdi = r14;
            // 95d27 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95d2c lea rsi,[rip+0x9f878] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 95d33 lea rdi,[rbp-0x4e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb20);
            // 95d3a mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 95d3f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95d44 movzx ebx,BYTE PTR [rbp-0x4e0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fb20)) as u64) as u32 as u64;
            // 95d4b test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 95d4e jne 0x95d55
            if crate::cov::brk(0x95d4e, fl.ne()) { pc = 0x95d55; continue; }
            pc = 0x95d50;
        }
        0x95d50 => {
            // block 95d50
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95d50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95d50 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 95d53 jmp 0x95d5c
            pc = 0x95d5c; continue;
        }
        0x95d55 => {
            // block 95d55
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95d55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95d55 mov rbx,QWORD PTR [rbp-0x4d8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb28)) as u64;
            pc = 0x95d5c;
        }
        0x95d5c => {
            // block 95d5c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95d5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95d5c lea rdi,[rbp-0x4e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb20);
            // 95d63 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95d68 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 95d6b je 0x95dfd
            if crate::cov::brk(0x95d6b, fl.e()) { pc = 0x95dfd; continue; }
            pc = 0x95d71;
        }
        0x95d71 => {
            // block 95d71
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95d71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95d71 lea rsi,[rip+0x9f833] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 95d78 lea rdi,[rbp-0x520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fae0);
            // 95d7f mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 95d84 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95d89 mov rax,QWORD PTR [rbp-0x510]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_faf0)) as u64;
            // 95d90 mov QWORD PTR [rbp-0x4f0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb10), (rax) as u64);
            // 95d97 mov rax,QWORD PTR [rbp-0x520]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fae0)) as u64;
            // 95d9e mov rcx,QWORD PTR [rbp-0x518]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fae8)) as u64;
            // 95da5 mov QWORD PTR [rbp-0x4f8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb08), (rcx) as u64);
            // 95dac mov QWORD PTR [rbp-0x500],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb00), (rax) as u64);
            // 95db3 xorps xmm0,xmm0
            x0 = 0;
            // 95db6 movaps XMMWORD PTR [rbp-0x520],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fae0), x0);
            // 95dbd mov QWORD PTR [rbp-0x510],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_faf0), (0x0u64) as u64);
            // 95dc8 mov DWORD PTR [rbp-0x4e8],0x14c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb18), (0x14cu64) as u32);
            // 95dd2 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 95dd9 lea rsi,[rbp-0x500]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb00);
            // 95de0 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95de5 lea rdi,[rbp-0x500]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb00);
            // 95dec call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95df1 lea rdi,[rbp-0x520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fae0);
            // 95df8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x95dfd;
        }
        0x95dfd => {
            // block 95dfd
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95dfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95dfd lea rsi,[rip+0x9dbd5] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 95e04 lea rdi,[rbp-0x538]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fac8);
            // 95e0b mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 95e10 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95e15 movzx ebx,BYTE PTR [rbp-0x538]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fac8)) as u64) as u32 as u64;
            // 95e1c test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 95e1f jne 0x95e26
            if crate::cov::brk(0x95e1f, fl.ne()) { pc = 0x95e26; continue; }
            pc = 0x95e21;
        }
        0x95e21 => {
            // block 95e21
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95e21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95e21 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 95e24 jmp 0x95e2d
            pc = 0x95e2d; continue;
        }
        0x95e26 => {
            // block 95e26
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95e26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95e26 mov rbx,QWORD PTR [rbp-0x530]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fad0)) as u64;
            pc = 0x95e2d;
        }
        0x95e2d => {
            // block 95e2d
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95e2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95e2d lea rdi,[rbp-0x538]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fac8);
            // 95e34 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95e39 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 95e3c je 0x95ece
            if crate::cov::brk(0x95e3c, fl.e()) { pc = 0x95ece; continue; }
            pc = 0x95e42;
        }
        0x95e42 => {
            // block 95e42
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95e42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95e42 lea rsi,[rip+0x9db90] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 95e49 lea rdi,[rbp-0x570]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa90);
            // 95e50 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 95e55 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95e5a mov rax,QWORD PTR [rbp-0x560]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_faa0)) as u64;
            // 95e61 mov QWORD PTR [rbp-0x548],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fab8), (rax) as u64);
            // 95e68 mov rax,QWORD PTR [rbp-0x570]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa90)) as u64;
            // 95e6f mov rcx,QWORD PTR [rbp-0x568]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa98)) as u64;
            // 95e76 mov QWORD PTR [rbp-0x550],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fab0), (rcx) as u64);
            // 95e7d mov QWORD PTR [rbp-0x558],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_faa8), (rax) as u64);
            // 95e84 xorps xmm0,xmm0
            x0 = 0;
            // 95e87 movaps XMMWORD PTR [rbp-0x570],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fa90), x0);
            // 95e8e mov QWORD PTR [rbp-0x560],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_faa0), (0x0u64) as u64);
            // 95e99 mov DWORD PTR [rbp-0x540],0x14c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fac0), (0x14cu64) as u32);
            // 95ea3 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 95eaa lea rsi,[rbp-0x558]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_faa8);
            // 95eb1 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95eb6 lea rdi,[rbp-0x558]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_faa8);
            // 95ebd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95ec2 lea rdi,[rbp-0x570]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa90);
            // 95ec9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x95ece;
        }
        0x95ece => {
            // block 95ece
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95ece rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95ece lea rsi,[rip+0x9f92a] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 95ed5 lea rdi,[rbp-0x588]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa78);
            // 95edc mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 95ee1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95ee6 movzx ebx,BYTE PTR [rbp-0x588]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fa78)) as u64) as u32 as u64;
            // 95eed test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 95ef0 jne 0x95ef7
            if crate::cov::brk(0x95ef0, fl.ne()) { pc = 0x95ef7; continue; }
            pc = 0x95ef2;
        }
        0x95ef2 => {
            // block 95ef2
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95ef2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95ef2 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 95ef5 jmp 0x95efe
            pc = 0x95efe; continue;
        }
        0x95ef7 => {
            // block 95ef7
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95ef7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95ef7 mov rbx,QWORD PTR [rbp-0x580]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa80)) as u64;
            pc = 0x95efe;
        }
        0x95efe => {
            // block 95efe
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95efe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95efe lea rdi,[rbp-0x588]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa78);
            // 95f05 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95f0a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 95f0d je 0x95f9f
            if crate::cov::brk(0x95f0d, fl.e()) { pc = 0x95f9f; continue; }
            pc = 0x95f13;
        }
        0x95f13 => {
            // block 95f13
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95f13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95f13 lea rsi,[rip+0x9f8e5] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 95f1a lea rdi,[rbp-0x5c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa40);
            // 95f21 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 95f26 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95f2b mov rax,QWORD PTR [rbp-0x5b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa50)) as u64;
            // 95f32 mov QWORD PTR [rbp-0x598],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa68), (rax) as u64);
            // 95f39 mov rax,QWORD PTR [rbp-0x5c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa40)) as u64;
            // 95f40 mov rcx,QWORD PTR [rbp-0x5b8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa48)) as u64;
            // 95f47 mov QWORD PTR [rbp-0x5a0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa60), (rcx) as u64);
            // 95f4e mov QWORD PTR [rbp-0x5a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa58), (rax) as u64);
            // 95f55 xorps xmm0,xmm0
            x0 = 0;
            // 95f58 movaps XMMWORD PTR [rbp-0x5c0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fa40), x0);
            // 95f5f mov QWORD PTR [rbp-0x5b0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa50), (0x0u64) as u64);
            // 95f6a mov DWORD PTR [rbp-0x590],0x14c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa70), (0x14cu64) as u32);
            // 95f74 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 95f7b lea rsi,[rbp-0x5a8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa58);
            // 95f82 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95f87 lea rdi,[rbp-0x5a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa58);
            // 95f8e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95f93 lea rdi,[rbp-0x5c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa40);
            // 95f9a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x95f9f;
        }
        0x95f9f => {
            // block 95f9f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 95f9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95f9f lea rsi,[rip+0xa2d22] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 95fa6 lea rbx,[rbp-0x5d8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fa28);
            // 95fad xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 95faf mov rdi,rbx
            rdi = rbx;
            // 95fb2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95fb7 mov esi,0x14d
            rsi = (0x14du64) as u32 as u64;
            // 95fbc mov rdi,r15
            rdi = r15;
            // 95fbf mov rdx,rbx
            rdx = rbx;
            // 95fc2 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95fc7 lea rdi,[rbp-0x5d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa28);
            // 95fce call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95fd3 mov DWORD PTR [rbp-0x5e8],0x14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa18), (0x14du64) as u32);
            // 95fdd mov QWORD PTR [rbp-0x5e0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa20), (r12) as u64);
            // 95fe4 lea rsi,[rbp-0x5e8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa18);
            // 95feb mov rdi,r14
            rdi = r14;
            // 95fee call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 95ff3 lea rsi,[rip+0x9f5b1] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 95ffa lea rdi,[rbp-0x600]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa00);
            // 96001 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 96006 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9600b movzx ebx,BYTE PTR [rbp-0x600]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fa00)) as u64) as u32 as u64;
            // 96012 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 96015 jne 0x9601c
            if crate::cov::brk(0x96015, fl.ne()) { pc = 0x9601c; continue; }
            pc = 0x96017;
        }
        0x96017 => {
            // block 96017
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96017 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96017 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9601a jmp 0x96023
            pc = 0x96023; continue;
        }
        0x9601c => {
            // block 9601c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9601c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9601c mov rbx,QWORD PTR [rbp-0x5f8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa08)) as u64;
            pc = 0x96023;
        }
        0x96023 => {
            // block 96023
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96023 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96023 lea rdi,[rbp-0x600]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa00);
            // 9602a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9602f test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 96032 je 0x960c4
            if crate::cov::brk(0x96032, fl.e()) { pc = 0x960c4; continue; }
            pc = 0x96038;
        }
        0x96038 => {
            // block 96038
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96038 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96038 lea rsi,[rip+0x9f56c] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 9603f lea rdi,[rbp-0x640]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9c0);
            // 96046 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 9604b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96050 mov rax,QWORD PTR [rbp-0x630]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f9d0)) as u64;
            // 96057 mov QWORD PTR [rbp-0x610],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f9f0), (rax) as u64);
            // 9605e mov rax,QWORD PTR [rbp-0x640]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f9c0)) as u64;
            // 96065 mov rcx,QWORD PTR [rbp-0x638]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f9c8)) as u64;
            // 9606c mov QWORD PTR [rbp-0x618],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f9e8), (rcx) as u64);
            // 96073 mov QWORD PTR [rbp-0x620],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f9e0), (rax) as u64);
            // 9607a xorps xmm0,xmm0
            x0 = 0;
            // 9607d movaps XMMWORD PTR [rbp-0x640],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f9c0), x0);
            // 96084 mov QWORD PTR [rbp-0x630],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f9d0), (0x0u64) as u64);
            // 9608f mov DWORD PTR [rbp-0x608],0x14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9f8), (0x14du64) as u32);
            // 96099 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 960a0 lea rsi,[rbp-0x620]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9e0);
            // 960a7 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 960ac lea rdi,[rbp-0x620]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9e0);
            // 960b3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 960b8 lea rdi,[rbp-0x640]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9c0);
            // 960bf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x960c4;
        }
        0x960c4 => {
            // block 960c4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 960c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 960c4 lea rsi,[rip+0x9d90e] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 960cb lea rdi,[rbp-0x658]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9a8);
            // 960d2 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 960d7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 960dc movzx ebx,BYTE PTR [rbp-0x658]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f9a8)) as u64) as u32 as u64;
            // 960e3 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 960e6 jne 0x960ed
            if crate::cov::brk(0x960e6, fl.ne()) { pc = 0x960ed; continue; }
            pc = 0x960e8;
        }
        0x960e8 => {
            // block 960e8
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 960e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 960e8 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 960eb jmp 0x960f4
            pc = 0x960f4; continue;
        }
        0x960ed => {
            // block 960ed
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 960ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 960ed mov rbx,QWORD PTR [rbp-0x650]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f9b0)) as u64;
            pc = 0x960f4;
        }
        0x960f4 => {
            // block 960f4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 960f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 960f4 lea rdi,[rbp-0x658]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9a8);
            // 960fb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96100 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 96103 je 0x96195
            if crate::cov::brk(0x96103, fl.e()) { pc = 0x96195; continue; }
            pc = 0x96109;
        }
        0x96109 => {
            // block 96109
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96109 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96109 lea rsi,[rip+0x9d8c9] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 96110 lea rdi,[rbp-0x690]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f970);
            // 96117 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 9611c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96121 mov rax,QWORD PTR [rbp-0x680]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f980)) as u64;
            // 96128 mov QWORD PTR [rbp-0x668],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f998), (rax) as u64);
            // 9612f mov rax,QWORD PTR [rbp-0x690]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f970)) as u64;
            // 96136 mov rcx,QWORD PTR [rbp-0x688]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f978)) as u64;
            // 9613d mov QWORD PTR [rbp-0x670],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f990), (rcx) as u64);
            // 96144 mov QWORD PTR [rbp-0x678],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f988), (rax) as u64);
            // 9614b xorps xmm0,xmm0
            x0 = 0;
            // 9614e movaps XMMWORD PTR [rbp-0x690],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f970), x0);
            // 96155 mov QWORD PTR [rbp-0x680],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f980), (0x0u64) as u64);
            // 96160 mov DWORD PTR [rbp-0x660],0x14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9a0), (0x14du64) as u32);
            // 9616a lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 96171 lea rsi,[rbp-0x678]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f988);
            // 96178 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9617d lea rdi,[rbp-0x678]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f988);
            // 96184 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96189 lea rdi,[rbp-0x690]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f970);
            // 96190 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x96195;
        }
        0x96195 => {
            // block 96195
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96195 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96195 lea rsi,[rip+0x9f663] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 9619c lea rdi,[rbp-0x6a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f958);
            // 961a3 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 961a8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 961ad movzx ebx,BYTE PTR [rbp-0x6a8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f958)) as u64) as u32 as u64;
            // 961b4 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 961b7 jne 0x961be
            if crate::cov::brk(0x961b7, fl.ne()) { pc = 0x961be; continue; }
            pc = 0x961b9;
        }
        0x961b9 => {
            // block 961b9
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 961b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 961b9 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 961bc jmp 0x961c5
            pc = 0x961c5; continue;
        }
        0x961be => {
            // block 961be
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 961be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 961be mov rbx,QWORD PTR [rbp-0x6a0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f960)) as u64;
            pc = 0x961c5;
        }
        0x961c5 => {
            // block 961c5
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 961c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 961c5 lea rdi,[rbp-0x6a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f958);
            // 961cc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 961d1 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 961d4 je 0x96266
            if crate::cov::brk(0x961d4, fl.e()) { pc = 0x96266; continue; }
            pc = 0x961da;
        }
        0x961da => {
            // block 961da
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 961da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 961da lea rsi,[rip+0x9f61e] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 961e1 lea rdi,[rbp-0x6e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f920);
            // 961e8 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 961ed call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 961f2 mov rax,QWORD PTR [rbp-0x6d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f930)) as u64;
            // 961f9 mov QWORD PTR [rbp-0x6b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f948), (rax) as u64);
            // 96200 mov rax,QWORD PTR [rbp-0x6e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f920)) as u64;
            // 96207 mov rcx,QWORD PTR [rbp-0x6d8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f928)) as u64;
            // 9620e mov QWORD PTR [rbp-0x6c0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f940), (rcx) as u64);
            // 96215 mov QWORD PTR [rbp-0x6c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f938), (rax) as u64);
            // 9621c xorps xmm0,xmm0
            x0 = 0;
            // 9621f movaps XMMWORD PTR [rbp-0x6e0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f920), x0);
            // 96226 mov QWORD PTR [rbp-0x6d0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f930), (0x0u64) as u64);
            // 96231 mov DWORD PTR [rbp-0x6b0],0x14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f950), (0x14du64) as u32);
            // 9623b lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 96242 lea rsi,[rbp-0x6c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f938);
            // 96249 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9624e lea rdi,[rbp-0x6c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f938);
            // 96255 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9625a lea rdi,[rbp-0x6e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f920);
            // 96261 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x96266;
        }
        0x96266 => {
            // block 96266
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96266 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96266 lea rsi,[rip+0xa2a5b] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 9626d lea rbx,[rbp-0x6f8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f908);
            // 96274 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 96276 mov rdi,rbx
            rdi = rbx;
            // 96279 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9627e mov esi,0x14e
            rsi = (0x14eu64) as u32 as u64;
            // 96283 mov rdi,r15
            rdi = r15;
            // 96286 mov rdx,rbx
            rdx = rbx;
            // 96289 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9628e lea rdi,[rbp-0x6f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f908);
            // 96295 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9629a mov DWORD PTR [rbp-0x708],0x14e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f8f8), (0x14eu64) as u32);
            // 962a4 mov QWORD PTR [rbp-0x700],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f900), (r12) as u64);
            // 962ab lea rsi,[rbp-0x708]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f8f8);
            // 962b2 mov rdi,r14
            rdi = r14;
            // 962b5 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 962ba lea rsi,[rip+0x9f2ea] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 962c1 lea rdi,[rbp-0x720]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8e0);
            // 962c8 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 962cd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 962d2 movzx ebx,BYTE PTR [rbp-0x720]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f8e0)) as u64) as u32 as u64;
            // 962d9 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 962dc jne 0x962e3
            if crate::cov::brk(0x962dc, fl.ne()) { pc = 0x962e3; continue; }
            pc = 0x962de;
        }
        0x962de => {
            // block 962de
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 962de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 962de shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 962e1 jmp 0x962ea
            pc = 0x962ea; continue;
        }
        0x962e3 => {
            // block 962e3
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 962e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 962e3 mov rbx,QWORD PTR [rbp-0x718]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8e8)) as u64;
            pc = 0x962ea;
        }
        0x962ea => {
            // block 962ea
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 962ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 962ea lea rdi,[rbp-0x720]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8e0);
            // 962f1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 962f6 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 962f9 je 0x9638b
            if crate::cov::brk(0x962f9, fl.e()) { pc = 0x9638b; continue; }
            pc = 0x962ff;
        }
        0x962ff => {
            // block 962ff
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 962ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 962ff lea rsi,[rip+0x9f2a5] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 96306 lea rdi,[rbp-0x760]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8a0);
            // 9630d mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 96312 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96317 mov rax,QWORD PTR [rbp-0x750]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b0)) as u64;
            // 9631e mov QWORD PTR [rbp-0x730],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8d0), (rax) as u64);
            // 96325 mov rax,QWORD PTR [rbp-0x760]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8a0)) as u64;
            // 9632c mov rcx,QWORD PTR [rbp-0x758]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8a8)) as u64;
            // 96333 mov QWORD PTR [rbp-0x738],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8c8), (rcx) as u64);
            // 9633a mov QWORD PTR [rbp-0x740],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8c0), (rax) as u64);
            // 96341 xorps xmm0,xmm0
            x0 = 0;
            // 96344 movaps XMMWORD PTR [rbp-0x760],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f8a0), x0);
            // 9634b mov QWORD PTR [rbp-0x750],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8b0), (0x0u64) as u64);
            // 96356 mov DWORD PTR [rbp-0x728],0x14e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f8d8), (0x14eu64) as u32);
            // 96360 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 96367 lea rsi,[rbp-0x740]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f8c0);
            // 9636e call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96373 lea rdi,[rbp-0x740]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8c0);
            // 9637a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9637f lea rdi,[rbp-0x760]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8a0);
            // 96386 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x9638b;
        }
        0x9638b => {
            // block 9638b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9638b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9638b lea rsi,[rip+0x9d647] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 96392 lea rdi,[rbp-0x778]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f888);
            // 96399 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 9639e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 963a3 movzx ebx,BYTE PTR [rbp-0x778]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f888)) as u64) as u32 as u64;
            // 963aa test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 963ad jne 0x963b4
            if crate::cov::brk(0x963ad, fl.ne()) { pc = 0x963b4; continue; }
            pc = 0x963af;
        }
        0x963af => {
            // block 963af
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 963af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 963af shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 963b2 jmp 0x963bb
            pc = 0x963bb; continue;
        }
        0x963b4 => {
            // block 963b4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 963b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 963b4 mov rbx,QWORD PTR [rbp-0x770]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f890)) as u64;
            pc = 0x963bb;
        }
        0x963bb => {
            // block 963bb
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 963bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 963bb lea rdi,[rbp-0x778]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f888);
            // 963c2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 963c7 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 963ca je 0x9645c
            if crate::cov::brk(0x963ca, fl.e()) { pc = 0x9645c; continue; }
            pc = 0x963d0;
        }
        0x963d0 => {
            // block 963d0
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 963d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 963d0 lea rsi,[rip+0x9d602] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 963d7 lea rdi,[rbp-0x7b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f850);
            // 963de mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 963e3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 963e8 mov rax,QWORD PTR [rbp-0x7a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f860)) as u64;
            // 963ef mov QWORD PTR [rbp-0x788],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f878), (rax) as u64);
            // 963f6 mov rax,QWORD PTR [rbp-0x7b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f850)) as u64;
            // 963fd mov rcx,QWORD PTR [rbp-0x7a8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f858)) as u64;
            // 96404 mov QWORD PTR [rbp-0x790],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f870), (rcx) as u64);
            // 9640b mov QWORD PTR [rbp-0x798],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f868), (rax) as u64);
            // 96412 xorps xmm0,xmm0
            x0 = 0;
            // 96415 movaps XMMWORD PTR [rbp-0x7b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f850), x0);
            // 9641c mov QWORD PTR [rbp-0x7a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f860), (0x0u64) as u64);
            // 96427 mov DWORD PTR [rbp-0x780],0x14e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f880), (0x14eu64) as u32);
            // 96431 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 96438 lea rsi,[rbp-0x798]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f868);
            // 9643f call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96444 lea rdi,[rbp-0x798]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f868);
            // 9644b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96450 lea rdi,[rbp-0x7b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f850);
            // 96457 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x9645c;
        }
        0x9645c => {
            // block 9645c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9645c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9645c lea rsi,[rip+0x9f39c] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 96463 lea rdi,[rbp-0x7c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f838);
            // 9646a mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 9646f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96474 movzx ebx,BYTE PTR [rbp-0x7c8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f838)) as u64) as u32 as u64;
            // 9647b test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9647e jne 0x96485
            if crate::cov::brk(0x9647e, fl.ne()) { pc = 0x96485; continue; }
            pc = 0x96480;
        }
        0x96480 => {
            // block 96480
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96480 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96480 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 96483 jmp 0x9648c
            pc = 0x9648c; continue;
        }
        0x96485 => {
            // block 96485
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96485 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96485 mov rbx,QWORD PTR [rbp-0x7c0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f840)) as u64;
            pc = 0x9648c;
        }
        0x9648c => {
            // block 9648c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9648c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9648c lea rdi,[rbp-0x7c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f838);
            // 96493 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96498 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9649b je 0x9652d
            if crate::cov::brk(0x9649b, fl.e()) { pc = 0x9652d; continue; }
            pc = 0x964a1;
        }
        0x964a1 => {
            // block 964a1
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 964a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 964a1 lea rsi,[rip+0x9f357] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 964a8 lea rdi,[rbp-0x800]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f800);
            // 964af mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 964b4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 964b9 mov rax,QWORD PTR [rbp-0x7f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f810)) as u64;
            // 964c0 mov QWORD PTR [rbp-0x7d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f828), (rax) as u64);
            // 964c7 mov rax,QWORD PTR [rbp-0x800]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f800)) as u64;
            // 964ce mov rcx,QWORD PTR [rbp-0x7f8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f808)) as u64;
            // 964d5 mov QWORD PTR [rbp-0x7e0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f820), (rcx) as u64);
            // 964dc mov QWORD PTR [rbp-0x7e8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f818), (rax) as u64);
            // 964e3 xorps xmm0,xmm0
            x0 = 0;
            // 964e6 movaps XMMWORD PTR [rbp-0x800],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f800), x0);
            // 964ed mov QWORD PTR [rbp-0x7f0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f810), (0x0u64) as u64);
            // 964f8 mov DWORD PTR [rbp-0x7d0],0x14e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f830), (0x14eu64) as u32);
            // 96502 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 96509 lea rsi,[rbp-0x7e8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f818);
            // 96510 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96515 lea rdi,[rbp-0x7e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f818);
            // 9651c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96521 lea rdi,[rbp-0x800]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f800);
            // 96528 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x9652d;
        }
        0x9652d => {
            // block 9652d
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9652d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9652d lea rsi,[rip+0xa2794] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 96534 lea rbx,[rbp-0x818]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f7e8);
            // 9653b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 9653d mov rdi,rbx
            rdi = rbx;
            // 96540 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96545 mov esi,0x14f
            rsi = (0x14fu64) as u32 as u64;
            // 9654a mov rdi,r15
            rdi = r15;
            // 9654d mov rdx,rbx
            rdx = rbx;
            // 96550 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96555 lea rdi,[rbp-0x818]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7e8);
            // 9655c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96561 mov DWORD PTR [rbp-0x828],0x14f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f7d8), (0x14fu64) as u32);
            // 9656b mov QWORD PTR [rbp-0x820],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f7e0), (r12) as u64);
            // 96572 lea rsi,[rbp-0x828]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f7d8);
            // 96579 mov rdi,r14
            rdi = r14;
            // 9657c call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96581 lea rsi,[rip+0x9f023] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 96588 lea rdi,[rbp-0x840]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7c0);
            // 9658f mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 96594 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96599 movzx ebx,BYTE PTR [rbp-0x840]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f7c0)) as u64) as u32 as u64;
            // 965a0 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 965a3 jne 0x965aa
            if crate::cov::brk(0x965a3, fl.ne()) { pc = 0x965aa; continue; }
            pc = 0x965a5;
        }
        0x965a5 => {
            // block 965a5
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 965a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 965a5 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 965a8 jmp 0x965b1
            pc = 0x965b1; continue;
        }
        0x965aa => {
            // block 965aa
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 965aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 965aa mov rbx,QWORD PTR [rbp-0x838]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f7c8)) as u64;
            pc = 0x965b1;
        }
        0x965b1 => {
            // block 965b1
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 965b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 965b1 lea rdi,[rbp-0x840]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7c0);
            // 965b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 965bd test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 965c0 je 0x96652
            if crate::cov::brk(0x965c0, fl.e()) { pc = 0x96652; continue; }
            pc = 0x965c6;
        }
        0x965c6 => {
            // block 965c6
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 965c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 965c6 lea rsi,[rip+0x9efde] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 965cd lea rdi,[rbp-0x880]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f780);
            // 965d4 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 965d9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 965de mov rax,QWORD PTR [rbp-0x870]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f790)) as u64;
            // 965e5 mov QWORD PTR [rbp-0x850],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f7b0), (rax) as u64);
            // 965ec mov rax,QWORD PTR [rbp-0x880]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f780)) as u64;
            // 965f3 mov rcx,QWORD PTR [rbp-0x878]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f788)) as u64;
            // 965fa mov QWORD PTR [rbp-0x858],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f7a8), (rcx) as u64);
            // 96601 mov QWORD PTR [rbp-0x860],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f7a0), (rax) as u64);
            // 96608 xorps xmm0,xmm0
            x0 = 0;
            // 9660b movaps XMMWORD PTR [rbp-0x880],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f780), x0);
            // 96612 mov QWORD PTR [rbp-0x870],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f790), (0x0u64) as u64);
            // 9661d mov DWORD PTR [rbp-0x848],0x14f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f7b8), (0x14fu64) as u32);
            // 96627 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 9662e lea rsi,[rbp-0x860]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f7a0);
            // 96635 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9663a lea rdi,[rbp-0x860]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7a0);
            // 96641 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96646 lea rdi,[rbp-0x880]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f780);
            // 9664d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x96652;
        }
        0x96652 => {
            // block 96652
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96652 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96652 lea rsi,[rip+0x9d380] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 96659 lea rdi,[rbp-0x898]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f768);
            // 96660 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 96665 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9666a movzx ebx,BYTE PTR [rbp-0x898]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f768)) as u64) as u32 as u64;
            // 96671 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 96674 jne 0x9667b
            if crate::cov::brk(0x96674, fl.ne()) { pc = 0x9667b; continue; }
            pc = 0x96676;
        }
        0x96676 => {
            // block 96676
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96676 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96676 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 96679 jmp 0x96682
            pc = 0x96682; continue;
        }
        0x9667b => {
            // block 9667b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9667b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9667b mov rbx,QWORD PTR [rbp-0x890]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f770)) as u64;
            pc = 0x96682;
        }
        0x96682 => {
            // block 96682
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96682 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96682 lea rdi,[rbp-0x898]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f768);
            // 96689 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9668e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 96691 je 0x96723
            if crate::cov::brk(0x96691, fl.e()) { pc = 0x96723; continue; }
            pc = 0x96697;
        }
        0x96697 => {
            // block 96697
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96697 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96697 lea rsi,[rip+0x9d33b] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 9669e lea rdi,[rbp-0x8d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f730);
            // 966a5 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 966aa call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 966af mov rax,QWORD PTR [rbp-0x8c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f740)) as u64;
            // 966b6 mov QWORD PTR [rbp-0x8a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f758), (rax) as u64);
            // 966bd mov rax,QWORD PTR [rbp-0x8d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f730)) as u64;
            // 966c4 mov rcx,QWORD PTR [rbp-0x8c8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f738)) as u64;
            // 966cb mov QWORD PTR [rbp-0x8b0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f750), (rcx) as u64);
            // 966d2 mov QWORD PTR [rbp-0x8b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f748), (rax) as u64);
            // 966d9 xorps xmm0,xmm0
            x0 = 0;
            // 966dc movaps XMMWORD PTR [rbp-0x8d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f730), x0);
            // 966e3 mov QWORD PTR [rbp-0x8c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f740), (0x0u64) as u64);
            // 966ee mov DWORD PTR [rbp-0x8a0],0x14f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f760), (0x14fu64) as u32);
            // 966f8 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 966ff lea rsi,[rbp-0x8b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f748);
            // 96706 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9670b lea rdi,[rbp-0x8b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f748);
            // 96712 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96717 lea rdi,[rbp-0x8d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f730);
            // 9671e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x96723;
        }
        0x96723 => {
            // block 96723
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96723 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96723 lea rsi,[rip+0x9f0d5] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 9672a lea rdi,[rbp-0x8e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f718);
            // 96731 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 96736 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9673b movzx ebx,BYTE PTR [rbp-0x8e8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f718)) as u64) as u32 as u64;
            // 96742 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 96745 jne 0x9674c
            if crate::cov::brk(0x96745, fl.ne()) { pc = 0x9674c; continue; }
            pc = 0x96747;
        }
        0x96747 => {
            // block 96747
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96747 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96747 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9674a jmp 0x96753
            pc = 0x96753; continue;
        }
        0x9674c => {
            // block 9674c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9674c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9674c mov rbx,QWORD PTR [rbp-0x8e0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f720)) as u64;
            pc = 0x96753;
        }
        0x96753 => {
            // block 96753
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96753 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96753 lea rdi,[rbp-0x8e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f718);
            // 9675a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9675f test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 96762 je 0x967f4
            if crate::cov::brk(0x96762, fl.e()) { pc = 0x967f4; continue; }
            pc = 0x96768;
        }
        0x96768 => {
            // block 96768
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96768 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96768 lea rsi,[rip+0x9f090] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 9676f lea rdi,[rbp-0x920]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6e0);
            // 96776 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 9677b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96780 mov rax,QWORD PTR [rbp-0x910]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f6f0)) as u64;
            // 96787 mov QWORD PTR [rbp-0x8f8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f708), (rax) as u64);
            // 9678e mov rax,QWORD PTR [rbp-0x920]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f6e0)) as u64;
            // 96795 mov rcx,QWORD PTR [rbp-0x918]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f6e8)) as u64;
            // 9679c mov QWORD PTR [rbp-0x900],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f700), (rcx) as u64);
            // 967a3 mov QWORD PTR [rbp-0x908],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f6f8), (rax) as u64);
            // 967aa xorps xmm0,xmm0
            x0 = 0;
            // 967ad movaps XMMWORD PTR [rbp-0x920],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f6e0), x0);
            // 967b4 mov QWORD PTR [rbp-0x910],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f6f0), (0x0u64) as u64);
            // 967bf mov DWORD PTR [rbp-0x8f0],0x14f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f710), (0x14fu64) as u32);
            // 967c9 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 967d0 lea rsi,[rbp-0x908]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f6f8);
            // 967d7 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 967dc lea rdi,[rbp-0x908]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6f8);
            // 967e3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 967e8 lea rdi,[rbp-0x920]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6e0);
            // 967ef call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x967f4;
        }
        0x967f4 => {
            // block 967f4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 967f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 967f4 lea rsi,[rip+0xa24cd] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 967fb lea rbx,[rbp-0x938]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f6c8);
            // 96802 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 96804 mov rdi,rbx
            rdi = rbx;
            // 96807 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9680c mov esi,0x150
            rsi = (0x150u64) as u32 as u64;
            // 96811 mov rdi,r15
            rdi = r15;
            // 96814 mov rdx,rbx
            rdx = rbx;
            // 96817 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9681c lea rdi,[rbp-0x938]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6c8);
            // 96823 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96828 lea rax,[r15+0x118]
            rax = r15.wrapping_add(0x118);
            // 9682f mov DWORD PTR [rbp-0x948],0x150
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f6b8), (0x150u64) as u32);
            // 96839 mov QWORD PTR [rbp-0x940],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f6c0), (rax) as u64);
            // 96840 lea rsi,[rbp-0x948]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f6b8);
            // 96847 mov rdi,r14
            rdi = r14;
            // 9684a call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9684f lea rsi,[rip+0x9ed55] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 96856 lea rdi,[rbp-0x960]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6a0);
            // 9685d mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 96862 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96867 movzx ebx,BYTE PTR [rbp-0x960]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f6a0)) as u64) as u32 as u64;
            // 9686e test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 96871 jne 0x96878
            if crate::cov::brk(0x96871, fl.ne()) { pc = 0x96878; continue; }
            pc = 0x96873;
        }
        0x96873 => {
            // block 96873
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96873 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96873 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 96876 jmp 0x9687f
            pc = 0x9687f; continue;
        }
        0x96878 => {
            // block 96878
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96878 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96878 mov rbx,QWORD PTR [rbp-0x958]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f6a8)) as u64;
            pc = 0x9687f;
        }
        0x9687f => {
            // block 9687f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9687f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9687f lea rdi,[rbp-0x960]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6a0);
            // 96886 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9688b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9688e je 0x96920
            if crate::cov::brk(0x9688e, fl.e()) { pc = 0x96920; continue; }
            pc = 0x96894;
        }
        0x96894 => {
            // block 96894
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96894 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96894 lea rsi,[rip+0x9ed10] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 9689b lea rdi,[rbp-0x9a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f660);
            // 968a2 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 968a7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 968ac mov rax,QWORD PTR [rbp-0x990]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f670)) as u64;
            // 968b3 mov QWORD PTR [rbp-0x970],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f690), (rax) as u64);
            // 968ba mov rax,QWORD PTR [rbp-0x9a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f660)) as u64;
            // 968c1 mov rcx,QWORD PTR [rbp-0x998]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f668)) as u64;
            // 968c8 mov QWORD PTR [rbp-0x978],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f688), (rcx) as u64);
            // 968cf mov QWORD PTR [rbp-0x980],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f680), (rax) as u64);
            // 968d6 xorps xmm0,xmm0
            x0 = 0;
            // 968d9 movaps XMMWORD PTR [rbp-0x9a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f660), x0);
            // 968e0 mov QWORD PTR [rbp-0x990],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f670), (0x0u64) as u64);
            // 968eb mov DWORD PTR [rbp-0x968],0x150
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f698), (0x150u64) as u32);
            // 968f5 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 968fc lea rsi,[rbp-0x980]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f680);
            // 96903 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96908 lea rdi,[rbp-0x980]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f680);
            // 9690f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96914 lea rdi,[rbp-0x9a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f660);
            // 9691b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x96920;
        }
        0x96920 => {
            // block 96920
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96920 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96920 lea rsi,[rip+0x9d0b2] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 96927 lea rdi,[rbp-0x9b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f648);
            // 9692e mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 96933 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96938 movzx ebx,BYTE PTR [rbp-0x9b8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f648)) as u64) as u32 as u64;
            // 9693f test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 96942 jne 0x96949
            if crate::cov::brk(0x96942, fl.ne()) { pc = 0x96949; continue; }
            pc = 0x96944;
        }
        0x96944 => {
            // block 96944
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96944 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96944 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 96947 jmp 0x96950
            pc = 0x96950; continue;
        }
        0x96949 => {
            // block 96949
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96949 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96949 mov rbx,QWORD PTR [rbp-0x9b0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f650)) as u64;
            pc = 0x96950;
        }
        0x96950 => {
            // block 96950
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96950 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96950 lea rdi,[rbp-0x9b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f648);
            // 96957 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9695c test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9695f je 0x969f1
            if crate::cov::brk(0x9695f, fl.e()) { pc = 0x969f1; continue; }
            pc = 0x96965;
        }
        0x96965 => {
            // block 96965
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96965 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96965 lea rsi,[rip+0x9d06d] # 0x1339d9
            rsi = m.env.data_va(295334);
            // 9696c lea rdi,[rbp-0x9f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f610);
            // 96973 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 96978 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9697d mov rax,QWORD PTR [rbp-0x9e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f620)) as u64;
            // 96984 mov QWORD PTR [rbp-0x9c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f638), (rax) as u64);
            // 9698b mov rax,QWORD PTR [rbp-0x9f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f610)) as u64;
            // 96992 mov rcx,QWORD PTR [rbp-0x9e8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f618)) as u64;
            // 96999 mov QWORD PTR [rbp-0x9d0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f630), (rcx) as u64);
            // 969a0 mov QWORD PTR [rbp-0x9d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f628), (rax) as u64);
            // 969a7 xorps xmm0,xmm0
            x0 = 0;
            // 969aa movaps XMMWORD PTR [rbp-0x9f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f610), x0);
            // 969b1 mov QWORD PTR [rbp-0x9e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f620), (0x0u64) as u64);
            // 969bc mov DWORD PTR [rbp-0x9c0],0x150
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f640), (0x150u64) as u32);
            // 969c6 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 969cd lea rsi,[rbp-0x9d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f628);
            // 969d4 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 969d9 lea rdi,[rbp-0x9d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f628);
            // 969e0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 969e5 lea rdi,[rbp-0x9f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f610);
            // 969ec call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x969f1;
        }
        0x969f1 => {
            // block 969f1
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 969f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 969f1 lea rsi,[rip+0x9ee07] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 969f8 lea rdi,[rbp-0xa08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5f8);
            // 969ff mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 96a04 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96a09 movzx ebx,BYTE PTR [rbp-0xa08]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f5f8)) as u64) as u32 as u64;
            // 96a10 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 96a13 jne 0x96a1a
            if crate::cov::brk(0x96a13, fl.ne()) { pc = 0x96a1a; continue; }
            pc = 0x96a15;
        }
        0x96a15 => {
            // block 96a15
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96a15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96a15 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 96a18 jmp 0x96a21
            pc = 0x96a21; continue;
        }
        0x96a1a => {
            // block 96a1a
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96a1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96a1a mov rbx,QWORD PTR [rbp-0xa00]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f600)) as u64;
            pc = 0x96a21;
        }
        0x96a21 => {
            // block 96a21
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96a21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96a21 lea rdi,[rbp-0xa08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5f8);
            // 96a28 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96a2d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 96a30 je 0x96ac2
            if crate::cov::brk(0x96a30, fl.e()) { pc = 0x96ac2; continue; }
            pc = 0x96a36;
        }
        0x96a36 => {
            // block 96a36
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96a36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96a36 lea rsi,[rip+0x9edc2] # 0x1357ff
            rsi = m.env.data_va(303052);
            // 96a3d lea rdi,[rbp-0xa40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5c0);
            // 96a44 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 96a49 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96a4e mov rax,QWORD PTR [rbp-0xa30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f5d0)) as u64;
            // 96a55 mov QWORD PTR [rbp-0xa18],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5e8), (rax) as u64);
            // 96a5c mov rax,QWORD PTR [rbp-0xa40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f5c0)) as u64;
            // 96a63 mov rcx,QWORD PTR [rbp-0xa38]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f5c8)) as u64;
            // 96a6a mov QWORD PTR [rbp-0xa20],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5e0), (rcx) as u64);
            // 96a71 mov QWORD PTR [rbp-0xa28],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5d8), (rax) as u64);
            // 96a78 xorps xmm0,xmm0
            x0 = 0;
            // 96a7b movaps XMMWORD PTR [rbp-0xa40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f5c0), x0);
            // 96a82 mov QWORD PTR [rbp-0xa30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5d0), (0x0u64) as u64);
            // 96a8d mov DWORD PTR [rbp-0xa10],0x150
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f5f0), (0x150u64) as u32);
            // 96a97 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 96a9e lea rsi,[rbp-0xa28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f5d8);
            // 96aa5 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96aaa lea rdi,[rbp-0xa28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5d8);
            // 96ab1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96ab6 lea rdi,[rbp-0xa40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5c0);
            // 96abd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x96ac2;
        }
        0x96ac2 => {
            // block 96ac2
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96ac2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96ac2 lea rsi,[rip+0xa21ff] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 96ac9 lea rbx,[rbp-0xa58]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f5a8);
            // 96ad0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 96ad2 mov rdi,rbx
            rdi = rbx;
            // 96ad5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96ada mov esi,0x167
            rsi = (0x167u64) as u32 as u64;
            // 96adf mov rdi,r15
            rdi = r15;
            // 96ae2 mov rdx,rbx
            rdx = rbx;
            // 96ae5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96aea lea rdi,[rbp-0xa58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5a8);
            // 96af1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96af6 mov DWORD PTR [rbp-0xa68],0x167
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f598), (0x167u64) as u32);
            // 96b00 mov QWORD PTR [rbp-0xa60],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5a0), (r15) as u64);
            // 96b07 lea rsi,[rbp-0xa68]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f598);
            // 96b0e mov rdi,r14
            rdi = r14;
            // 96b11 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96b16 lea rsi,[rip+0x9ea8e] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 96b1d lea rdi,[rbp-0xa80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f580);
            // 96b24 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 96b29 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96b2e movzx ebx,BYTE PTR [rbp-0xa80]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f580)) as u64) as u32 as u64;
            // 96b35 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 96b38 jne 0x96b3f
            if crate::cov::brk(0x96b38, fl.ne()) { pc = 0x96b3f; continue; }
            pc = 0x96b3a;
        }
        0x96b3a => {
            // block 96b3a
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96b3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96b3a shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 96b3d jmp 0x96b46
            pc = 0x96b46; continue;
        }
        0x96b3f => {
            // block 96b3f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96b3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96b3f mov rbx,QWORD PTR [rbp-0xa78]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f588)) as u64;
            pc = 0x96b46;
        }
        0x96b46 => {
            // block 96b46
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96b46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96b46 lea rdi,[rbp-0xa80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f580);
            // 96b4d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96b52 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 96b55 je 0x96be7
            if crate::cov::brk(0x96b55, fl.e()) { pc = 0x96be7; continue; }
            pc = 0x96b5b;
        }
        0x96b5b => {
            // block 96b5b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96b5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96b5b lea rsi,[rip+0x9ea49] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 96b62 lea rdi,[rbp-0xac0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f540);
            // 96b69 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 96b6e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96b73 mov rax,QWORD PTR [rbp-0xab0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f550)) as u64;
            // 96b7a mov QWORD PTR [rbp-0xa90],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f570), (rax) as u64);
            // 96b81 mov rax,QWORD PTR [rbp-0xac0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f540)) as u64;
            // 96b88 mov rcx,QWORD PTR [rbp-0xab8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f548)) as u64;
            // 96b8f mov QWORD PTR [rbp-0xa98],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f568), (rcx) as u64);
            // 96b96 mov QWORD PTR [rbp-0xaa0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f560), (rax) as u64);
            // 96b9d xorps xmm0,xmm0
            x0 = 0;
            // 96ba0 movaps XMMWORD PTR [rbp-0xac0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f540), x0);
            // 96ba7 mov QWORD PTR [rbp-0xab0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f550), (0x0u64) as u64);
            // 96bb2 mov DWORD PTR [rbp-0xa88],0x167
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f578), (0x167u64) as u32);
            // 96bbc lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 96bc3 lea rsi,[rbp-0xaa0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f560);
            // 96bca call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96bcf lea rdi,[rbp-0xaa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f560);
            // 96bd6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96bdb lea rdi,[rbp-0xac0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f540);
            // 96be2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x96be7;
        }
        0x96be7 => {
            // block 96be7
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96be7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96be7 lea rsi,[rip+0x9ce1d] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 96bee lea rdi,[rbp-0xad8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f528);
            // 96bf5 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 96bfa call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96bff movzx ebx,BYTE PTR [rbp-0xad8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f528)) as u64) as u32 as u64;
            // 96c06 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 96c09 jne 0x96c10
            if crate::cov::brk(0x96c09, fl.ne()) { pc = 0x96c10; continue; }
            pc = 0x96c0b;
        }
        0x96c0b => {
            // block 96c0b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96c0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96c0b shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 96c0e jmp 0x96c17
            pc = 0x96c17; continue;
        }
        0x96c10 => {
            // block 96c10
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96c10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96c10 mov rbx,QWORD PTR [rbp-0xad0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f530)) as u64;
            pc = 0x96c17;
        }
        0x96c17 => {
            // block 96c17
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96c17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96c17 lea rdi,[rbp-0xad8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f528);
            // 96c1e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96c23 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 96c26 je 0x96cb8
            if crate::cov::brk(0x96c26, fl.e()) { pc = 0x96cb8; continue; }
            pc = 0x96c2c;
        }
        0x96c2c => {
            // block 96c2c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96c2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96c2c lea rsi,[rip+0x9cdd8] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 96c33 lea rdi,[rbp-0xb10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4f0);
            // 96c3a mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 96c3f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96c44 mov rax,QWORD PTR [rbp-0xb00]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f500)) as u64;
            // 96c4b mov QWORD PTR [rbp-0xae8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f518), (rax) as u64);
            // 96c52 mov rax,QWORD PTR [rbp-0xb10]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f4f0)) as u64;
            // 96c59 mov rcx,QWORD PTR [rbp-0xb08]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f4f8)) as u64;
            // 96c60 mov QWORD PTR [rbp-0xaf0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f510), (rcx) as u64);
            // 96c67 mov QWORD PTR [rbp-0xaf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f508), (rax) as u64);
            // 96c6e xorps xmm0,xmm0
            x0 = 0;
            // 96c71 movaps XMMWORD PTR [rbp-0xb10],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f4f0), x0);
            // 96c78 mov QWORD PTR [rbp-0xb00],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f500), (0x0u64) as u64);
            // 96c83 mov DWORD PTR [rbp-0xae0],0x167
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f520), (0x167u64) as u32);
            // 96c8d lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 96c94 lea rsi,[rbp-0xaf8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f508);
            // 96c9b call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96ca0 lea rdi,[rbp-0xaf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f508);
            // 96ca7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96cac lea rdi,[rbp-0xb10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4f0);
            // 96cb3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x96cb8;
        }
        0x96cb8 => {
            // block 96cb8
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96cb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96cb8 xorps xmm0,xmm0
            x0 = 0;
            // 96cbb movaps XMMWORD PTR [rbp-0xb30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f4d0), x0);
            // 96cc2 mov QWORD PTR [rbp-0xb20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f4e0), (0x0u64) as u64);
            // 96ccd lea rdi,[rbp-0xb30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4d0);
            // 96cd4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96cd9 lea rsi,[rip+0xa1fe8] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 96ce0 lea rbx,[rbp-0xb48]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f4b8);
            // 96ce7 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 96ce9 mov rdi,rbx
            rdi = rbx;
            // 96cec call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96cf1 mov esi,0x168
            rsi = (0x168u64) as u32 as u64;
            // 96cf6 mov rdi,r15
            rdi = r15;
            // 96cf9 mov rdx,rbx
            rdx = rbx;
            // 96cfc call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96d01 lea rdi,[rbp-0xb48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4b8);
            // 96d08 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96d0d mov DWORD PTR [rbp-0xb58],0x168
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f4a8), (0x168u64) as u32);
            // 96d17 mov QWORD PTR [rbp-0xb50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f4b0), (r15) as u64);
            // 96d1e lea rsi,[rbp-0xb58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f4a8);
            // 96d25 mov rdi,r14
            rdi = r14;
            // 96d28 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96d2d lea rsi,[rip+0x9e877] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 96d34 lea rdi,[rbp-0xb70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f490);
            // 96d3b mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 96d40 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96d45 movzx ebx,BYTE PTR [rbp-0xb70]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f490)) as u64) as u32 as u64;
            // 96d4c test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 96d4f jne 0x96d56
            if crate::cov::brk(0x96d4f, fl.ne()) { pc = 0x96d56; continue; }
            pc = 0x96d51;
        }
        0x96d51 => {
            // block 96d51
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96d51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96d51 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 96d54 jmp 0x96d5d
            pc = 0x96d5d; continue;
        }
        0x96d56 => {
            // block 96d56
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96d56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96d56 mov rbx,QWORD PTR [rbp-0xb68]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f498)) as u64;
            pc = 0x96d5d;
        }
        0x96d5d => {
            // block 96d5d
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96d5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96d5d lea rdi,[rbp-0xb70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f490);
            // 96d64 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96d69 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 96d6c je 0x96dfe
            if crate::cov::brk(0x96d6c, fl.e()) { pc = 0x96dfe; continue; }
            pc = 0x96d72;
        }
        0x96d72 => {
            // block 96d72
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96d72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96d72 lea rsi,[rip+0x9e832] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 96d79 lea rdi,[rbp-0xbb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f450);
            // 96d80 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 96d85 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96d8a mov rax,QWORD PTR [rbp-0xba0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f460)) as u64;
            // 96d91 mov QWORD PTR [rbp-0xb80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f480), (rax) as u64);
            // 96d98 mov rax,QWORD PTR [rbp-0xbb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f450)) as u64;
            // 96d9f mov rcx,QWORD PTR [rbp-0xba8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f458)) as u64;
            // 96da6 mov QWORD PTR [rbp-0xb88],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f478), (rcx) as u64);
            // 96dad mov QWORD PTR [rbp-0xb90],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f470), (rax) as u64);
            // 96db4 xorps xmm0,xmm0
            x0 = 0;
            // 96db7 movaps XMMWORD PTR [rbp-0xbb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f450), x0);
            // 96dbe mov QWORD PTR [rbp-0xba0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f460), (0x0u64) as u64);
            // 96dc9 mov DWORD PTR [rbp-0xb78],0x168
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f488), (0x168u64) as u32);
            // 96dd3 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 96dda lea rsi,[rbp-0xb90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f470);
            // 96de1 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96de6 lea rdi,[rbp-0xb90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f470);
            // 96ded call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96df2 lea rdi,[rbp-0xbb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f450);
            // 96df9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x96dfe;
        }
        0x96dfe => {
            // block 96dfe
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96dfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96dfe lea rsi,[rip+0x9cc06] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 96e05 lea rdi,[rbp-0xbc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f438);
            // 96e0c mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 96e11 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96e16 movzx ebx,BYTE PTR [rbp-0xbc8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f438)) as u64) as u32 as u64;
            // 96e1d test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 96e20 jne 0x96e27
            if crate::cov::brk(0x96e20, fl.ne()) { pc = 0x96e27; continue; }
            pc = 0x96e22;
        }
        0x96e22 => {
            // block 96e22
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96e22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96e22 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 96e25 jmp 0x96e2e
            pc = 0x96e2e; continue;
        }
        0x96e27 => {
            // block 96e27
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96e27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96e27 mov rbx,QWORD PTR [rbp-0xbc0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f440)) as u64;
            pc = 0x96e2e;
        }
        0x96e2e => {
            // block 96e2e
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96e2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96e2e lea rdi,[rbp-0xbc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f438);
            // 96e35 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96e3a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 96e3d je 0x96ecf
            if crate::cov::brk(0x96e3d, fl.e()) { pc = 0x96ecf; continue; }
            pc = 0x96e43;
        }
        0x96e43 => {
            // block 96e43
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96e43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96e43 lea rsi,[rip+0x9cbc1] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 96e4a lea rdi,[rbp-0xc00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f400);
            // 96e51 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 96e56 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96e5b mov rax,QWORD PTR [rbp-0xbf0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f410)) as u64;
            // 96e62 mov QWORD PTR [rbp-0xbd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f428), (rax) as u64);
            // 96e69 mov rax,QWORD PTR [rbp-0xc00]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f400)) as u64;
            // 96e70 mov rcx,QWORD PTR [rbp-0xbf8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f408)) as u64;
            // 96e77 mov QWORD PTR [rbp-0xbe0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f420), (rcx) as u64);
            // 96e7e mov QWORD PTR [rbp-0xbe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f418), (rax) as u64);
            // 96e85 xorps xmm0,xmm0
            x0 = 0;
            // 96e88 movaps XMMWORD PTR [rbp-0xc00],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f400), x0);
            // 96e8f mov QWORD PTR [rbp-0xbf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f410), (0x0u64) as u64);
            // 96e9a mov DWORD PTR [rbp-0xbd0],0x168
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f430), (0x168u64) as u32);
            // 96ea4 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 96eab lea rsi,[rbp-0xbe8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f418);
            // 96eb2 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96eb7 lea rdi,[rbp-0xbe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f418);
            // 96ebe call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96ec3 lea rdi,[rbp-0xc00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f400);
            // 96eca call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x96ecf;
        }
        0x96ecf => {
            // block 96ecf
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96ecf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96ecf xorps xmm0,xmm0
            x0 = 0;
            // 96ed2 movaps XMMWORD PTR [rbp-0xc20],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f3e0), x0);
            // 96ed9 mov QWORD PTR [rbp-0xc10],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f3f0), (0x0u64) as u64);
            // 96ee4 lea rdi,[rbp-0xc20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3e0);
            // 96eeb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96ef0 lea rsi,[rip+0xa1dd1] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 96ef7 lea rbx,[rbp-0xc38]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f3c8);
            // 96efe xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 96f00 mov rdi,rbx
            rdi = rbx;
            // 96f03 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96f08 mov esi,0x169
            rsi = (0x169u64) as u32 as u64;
            // 96f0d mov rdi,r15
            rdi = r15;
            // 96f10 mov rdx,rbx
            rdx = rbx;
            // 96f13 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96f18 lea rdi,[rbp-0xc38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3c8);
            // 96f1f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96f24 mov DWORD PTR [rbp-0xc48],0x169
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f3b8), (0x169u64) as u32);
            // 96f2e mov QWORD PTR [rbp-0xc40],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f3c0), (r15) as u64);
            // 96f35 lea rsi,[rbp-0xc48]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f3b8);
            // 96f3c mov rdi,r14
            rdi = r14;
            // 96f3f call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96f44 lea rsi,[rip+0x9e660] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 96f4b lea rdi,[rbp-0xc60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3a0);
            // 96f52 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 96f57 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96f5c movzx ebx,BYTE PTR [rbp-0xc60]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f3a0)) as u64) as u32 as u64;
            // 96f63 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 96f66 jne 0x96f6d
            if crate::cov::brk(0x96f66, fl.ne()) { pc = 0x96f6d; continue; }
            pc = 0x96f68;
        }
        0x96f68 => {
            // block 96f68
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96f68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96f68 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 96f6b jmp 0x96f74
            pc = 0x96f74; continue;
        }
        0x96f6d => {
            // block 96f6d
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96f6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96f6d mov rbx,QWORD PTR [rbp-0xc58]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f3a8)) as u64;
            pc = 0x96f74;
        }
        0x96f74 => {
            // block 96f74
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96f74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96f74 lea rdi,[rbp-0xc60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3a0);
            // 96f7b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96f80 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 96f83 je 0x97015
            if crate::cov::brk(0x96f83, fl.e()) { pc = 0x97015; continue; }
            pc = 0x96f89;
        }
        0x96f89 => {
            // block 96f89
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 96f89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96f89 lea rsi,[rip+0x9e61b] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 96f90 lea rdi,[rbp-0xca0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f360);
            // 96f97 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 96f9c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96fa1 mov rax,QWORD PTR [rbp-0xc90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f370)) as u64;
            // 96fa8 mov QWORD PTR [rbp-0xc70],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f390), (rax) as u64);
            // 96faf mov rax,QWORD PTR [rbp-0xca0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f360)) as u64;
            // 96fb6 mov rcx,QWORD PTR [rbp-0xc98]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f368)) as u64;
            // 96fbd mov QWORD PTR [rbp-0xc78],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f388), (rcx) as u64);
            // 96fc4 mov QWORD PTR [rbp-0xc80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f380), (rax) as u64);
            // 96fcb xorps xmm0,xmm0
            x0 = 0;
            // 96fce movaps XMMWORD PTR [rbp-0xca0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f360), x0);
            // 96fd5 mov QWORD PTR [rbp-0xc90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f370), (0x0u64) as u64);
            // 96fe0 mov DWORD PTR [rbp-0xc68],0x169
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f398), (0x169u64) as u32);
            // 96fea lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 96ff1 lea rsi,[rbp-0xc80]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f380);
            // 96ff8 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 96ffd lea rdi,[rbp-0xc80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f380);
            // 97004 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97009 lea rdi,[rbp-0xca0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f360);
            // 97010 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x97015;
        }
        0x97015 => {
            // block 97015
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97015 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97015 lea rsi,[rip+0x9c9ef] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 9701c lea rdi,[rbp-0xcb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f348);
            // 97023 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 97028 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9702d movzx ebx,BYTE PTR [rbp-0xcb8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f348)) as u64) as u32 as u64;
            // 97034 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 97037 jne 0x9703e
            if crate::cov::brk(0x97037, fl.ne()) { pc = 0x9703e; continue; }
            pc = 0x97039;
        }
        0x97039 => {
            // block 97039
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97039 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97039 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9703c jmp 0x97045
            pc = 0x97045; continue;
        }
        0x9703e => {
            // block 9703e
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9703e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9703e mov rbx,QWORD PTR [rbp-0xcb0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f350)) as u64;
            pc = 0x97045;
        }
        0x97045 => {
            // block 97045
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97045 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97045 lea rdi,[rbp-0xcb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f348);
            // 9704c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97051 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 97054 je 0x970e6
            if crate::cov::brk(0x97054, fl.e()) { pc = 0x970e6; continue; }
            pc = 0x9705a;
        }
        0x9705a => {
            // block 9705a
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9705a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9705a lea rsi,[rip+0x9c9aa] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 97061 lea rdi,[rbp-0xcf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f310);
            // 97068 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9706d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97072 mov rax,QWORD PTR [rbp-0xce0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f320)) as u64;
            // 97079 mov QWORD PTR [rbp-0xcc8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f338), (rax) as u64);
            // 97080 mov rax,QWORD PTR [rbp-0xcf0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f310)) as u64;
            // 97087 mov rcx,QWORD PTR [rbp-0xce8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f318)) as u64;
            // 9708e mov QWORD PTR [rbp-0xcd0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f330), (rcx) as u64);
            // 97095 mov QWORD PTR [rbp-0xcd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f328), (rax) as u64);
            // 9709c xorps xmm0,xmm0
            x0 = 0;
            // 9709f movaps XMMWORD PTR [rbp-0xcf0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f310), x0);
            // 970a6 mov QWORD PTR [rbp-0xce0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f320), (0x0u64) as u64);
            // 970b1 mov DWORD PTR [rbp-0xcc0],0x169
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f340), (0x169u64) as u32);
            // 970bb lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 970c2 lea rsi,[rbp-0xcd8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f328);
            // 970c9 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 970ce lea rdi,[rbp-0xcd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f328);
            // 970d5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 970da lea rdi,[rbp-0xcf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f310);
            // 970e1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x970e6;
        }
        0x970e6 => {
            // block 970e6
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 970e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 970e6 xorps xmm0,xmm0
            x0 = 0;
            // 970e9 movaps XMMWORD PTR [rbp-0xd10],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f2f0), x0);
            // 970f0 mov QWORD PTR [rbp-0xd00],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f300), (0x0u64) as u64);
            // 970fb lea rdi,[rbp-0xd10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2f0);
            // 97102 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97107 lea rsi,[rip+0xa1bba] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 9710e lea rbx,[rbp-0xd28]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f2d8);
            // 97115 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 97117 mov rdi,rbx
            rdi = rbx;
            // 9711a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9711f mov esi,0x16a
            rsi = (0x16au64) as u32 as u64;
            // 97124 mov rdi,r15
            rdi = r15;
            // 97127 mov rdx,rbx
            rdx = rbx;
            // 9712a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9712f lea rdi,[rbp-0xd28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2d8);
            // 97136 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9713b mov DWORD PTR [rbp-0xd38],0x16a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f2c8), (0x16au64) as u32);
            // 97145 mov QWORD PTR [rbp-0xd30],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f2d0), (r15) as u64);
            // 9714c lea rsi,[rbp-0xd38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f2c8);
            // 97153 mov rdi,r14
            rdi = r14;
            // 97156 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9715b lea rsi,[rip+0x9e449] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 97162 lea rdi,[rbp-0xd50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2b0);
            // 97169 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 9716e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97173 movzx ebx,BYTE PTR [rbp-0xd50]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f2b0)) as u64) as u32 as u64;
            // 9717a test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9717d jne 0x97184
            if crate::cov::brk(0x9717d, fl.ne()) { pc = 0x97184; continue; }
            pc = 0x9717f;
        }
        0x9717f => {
            // block 9717f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9717f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9717f shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 97182 jmp 0x9718b
            pc = 0x9718b; continue;
        }
        0x97184 => {
            // block 97184
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97184 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97184 mov rbx,QWORD PTR [rbp-0xd48]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f2b8)) as u64;
            pc = 0x9718b;
        }
        0x9718b => {
            // block 9718b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9718b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9718b lea rdi,[rbp-0xd50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2b0);
            // 97192 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97197 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9719a je 0x9722c
            if crate::cov::brk(0x9719a, fl.e()) { pc = 0x9722c; continue; }
            pc = 0x971a0;
        }
        0x971a0 => {
            // block 971a0
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 971a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 971a0 lea rsi,[rip+0x9e404] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 971a7 lea rdi,[rbp-0xd90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f270);
            // 971ae mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 971b3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 971b8 mov rax,QWORD PTR [rbp-0xd80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f280)) as u64;
            // 971bf mov QWORD PTR [rbp-0xd60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f2a0), (rax) as u64);
            // 971c6 mov rax,QWORD PTR [rbp-0xd90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f270)) as u64;
            // 971cd mov rcx,QWORD PTR [rbp-0xd88]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f278)) as u64;
            // 971d4 mov QWORD PTR [rbp-0xd68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f298), (rcx) as u64);
            // 971db mov QWORD PTR [rbp-0xd70],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f290), (rax) as u64);
            // 971e2 xorps xmm0,xmm0
            x0 = 0;
            // 971e5 movaps XMMWORD PTR [rbp-0xd90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f270), x0);
            // 971ec mov QWORD PTR [rbp-0xd80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f280), (0x0u64) as u64);
            // 971f7 mov DWORD PTR [rbp-0xd58],0x16a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f2a8), (0x16au64) as u32);
            // 97201 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 97208 lea rsi,[rbp-0xd70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f290);
            // 9720f call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97214 lea rdi,[rbp-0xd70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f290);
            // 9721b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97220 lea rdi,[rbp-0xd90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f270);
            // 97227 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x9722c;
        }
        0x9722c => {
            // block 9722c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9722c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9722c lea rsi,[rip+0x9c7d8] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 97233 lea rdi,[rbp-0xda8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f258);
            // 9723a mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9723f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97244 movzx ebx,BYTE PTR [rbp-0xda8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f258)) as u64) as u32 as u64;
            // 9724b test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9724e jne 0x97255
            if crate::cov::brk(0x9724e, fl.ne()) { pc = 0x97255; continue; }
            pc = 0x97250;
        }
        0x97250 => {
            // block 97250
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97250 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97250 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 97253 jmp 0x9725c
            pc = 0x9725c; continue;
        }
        0x97255 => {
            // block 97255
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97255 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97255 mov rbx,QWORD PTR [rbp-0xda0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f260)) as u64;
            pc = 0x9725c;
        }
        0x9725c => {
            // block 9725c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9725c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9725c lea rdi,[rbp-0xda8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f258);
            // 97263 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97268 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9726b je 0x972fd
            if crate::cov::brk(0x9726b, fl.e()) { pc = 0x972fd; continue; }
            pc = 0x97271;
        }
        0x97271 => {
            // block 97271
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97271 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97271 lea rsi,[rip+0x9c793] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 97278 lea rdi,[rbp-0xde0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f220);
            // 9727f mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 97284 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97289 mov rax,QWORD PTR [rbp-0xdd0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f230)) as u64;
            // 97290 mov QWORD PTR [rbp-0xdb8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f248), (rax) as u64);
            // 97297 mov rax,QWORD PTR [rbp-0xde0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f220)) as u64;
            // 9729e mov rcx,QWORD PTR [rbp-0xdd8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f228)) as u64;
            // 972a5 mov QWORD PTR [rbp-0xdc0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f240), (rcx) as u64);
            // 972ac mov QWORD PTR [rbp-0xdc8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f238), (rax) as u64);
            // 972b3 xorps xmm0,xmm0
            x0 = 0;
            // 972b6 movaps XMMWORD PTR [rbp-0xde0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f220), x0);
            // 972bd mov QWORD PTR [rbp-0xdd0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f230), (0x0u64) as u64);
            // 972c8 mov DWORD PTR [rbp-0xdb0],0x16a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f250), (0x16au64) as u32);
            // 972d2 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 972d9 lea rsi,[rbp-0xdc8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f238);
            // 972e0 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 972e5 lea rdi,[rbp-0xdc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f238);
            // 972ec call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 972f1 lea rdi,[rbp-0xde0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f220);
            // 972f8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x972fd;
        }
        0x972fd => {
            // block 972fd
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 972fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 972fd xorps xmm0,xmm0
            x0 = 0;
            // 97300 movaps XMMWORD PTR [rbp-0xe00],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f200), x0);
            // 97307 mov QWORD PTR [rbp-0xdf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f210), (0x0u64) as u64);
            // 97312 lea rdi,[rbp-0xe00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f200);
            // 97319 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9731e lea rsi,[rip+0xa19a3] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 97325 lea rbx,[rbp-0xe18]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f1e8);
            // 9732c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 9732e mov rdi,rbx
            rdi = rbx;
            // 97331 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97336 mov esi,0x16b
            rsi = (0x16bu64) as u32 as u64;
            // 9733b mov rdi,r15
            rdi = r15;
            // 9733e mov rdx,rbx
            rdx = rbx;
            // 97341 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97346 lea rdi,[rbp-0xe18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1e8);
            // 9734d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97352 mov DWORD PTR [rbp-0xe28],0x16b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1d8), (0x16bu64) as u32);
            // 9735c mov QWORD PTR [rbp-0xe20],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f1e0), (r15) as u64);
            // 97363 lea rsi,[rbp-0xe28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1d8);
            // 9736a mov rdi,r14
            rdi = r14;
            // 9736d call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97372 lea rsi,[rip+0x9e232] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 97379 lea rdi,[rbp-0xe40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1c0);
            // 97380 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 97385 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9738a movzx ebx,BYTE PTR [rbp-0xe40]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f1c0)) as u64) as u32 as u64;
            // 97391 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 97394 jne 0x9739b
            if crate::cov::brk(0x97394, fl.ne()) { pc = 0x9739b; continue; }
            pc = 0x97396;
        }
        0x97396 => {
            // block 97396
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97396 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97396 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 97399 jmp 0x973a2
            pc = 0x973a2; continue;
        }
        0x9739b => {
            // block 9739b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9739b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9739b mov rbx,QWORD PTR [rbp-0xe38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f1c8)) as u64;
            pc = 0x973a2;
        }
        0x973a2 => {
            // block 973a2
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 973a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 973a2 lea rdi,[rbp-0xe40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1c0);
            // 973a9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 973ae test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 973b1 je 0x97443
            if crate::cov::brk(0x973b1, fl.e()) { pc = 0x97443; continue; }
            pc = 0x973b7;
        }
        0x973b7 => {
            // block 973b7
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 973b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 973b7 lea rsi,[rip+0x9e1ed] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 973be lea rdi,[rbp-0xe80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f180);
            // 973c5 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 973ca call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 973cf mov rax,QWORD PTR [rbp-0xe70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f190)) as u64;
            // 973d6 mov QWORD PTR [rbp-0xe50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f1b0), (rax) as u64);
            // 973dd mov rax,QWORD PTR [rbp-0xe80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f180)) as u64;
            // 973e4 mov rcx,QWORD PTR [rbp-0xe78]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f188)) as u64;
            // 973eb mov QWORD PTR [rbp-0xe58],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f1a8), (rcx) as u64);
            // 973f2 mov QWORD PTR [rbp-0xe60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f1a0), (rax) as u64);
            // 973f9 xorps xmm0,xmm0
            x0 = 0;
            // 973fc movaps XMMWORD PTR [rbp-0xe80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f180), x0);
            // 97403 mov QWORD PTR [rbp-0xe70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f190), (0x0u64) as u64);
            // 9740e mov DWORD PTR [rbp-0xe48],0x16b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1b8), (0x16bu64) as u32);
            // 97418 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 9741f lea rsi,[rbp-0xe60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1a0);
            // 97426 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9742b lea rdi,[rbp-0xe60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1a0);
            // 97432 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97437 lea rdi,[rbp-0xe80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f180);
            // 9743e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x97443;
        }
        0x97443 => {
            // block 97443
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97443 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97443 lea rsi,[rip+0x9c5c1] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 9744a lea rdi,[rbp-0xe98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f168);
            // 97451 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 97456 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9745b movzx ebx,BYTE PTR [rbp-0xe98]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f168)) as u64) as u32 as u64;
            // 97462 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 97465 jne 0x9746c
            if crate::cov::brk(0x97465, fl.ne()) { pc = 0x9746c; continue; }
            pc = 0x97467;
        }
        0x97467 => {
            // block 97467
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97467 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97467 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9746a jmp 0x97473
            pc = 0x97473; continue;
        }
        0x9746c => {
            // block 9746c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9746c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9746c mov rbx,QWORD PTR [rbp-0xe90]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f170)) as u64;
            pc = 0x97473;
        }
        0x97473 => {
            // block 97473
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97473 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97473 lea rdi,[rbp-0xe98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f168);
            // 9747a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9747f test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 97482 je 0x97514
            if crate::cov::brk(0x97482, fl.e()) { pc = 0x97514; continue; }
            pc = 0x97488;
        }
        0x97488 => {
            // block 97488
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97488 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97488 lea rsi,[rip+0x9c57c] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 9748f lea rdi,[rbp-0xed0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f130);
            // 97496 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9749b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 974a0 mov rax,QWORD PTR [rbp-0xec0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f140)) as u64;
            // 974a7 mov QWORD PTR [rbp-0xea8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f158), (rax) as u64);
            // 974ae mov rax,QWORD PTR [rbp-0xed0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f130)) as u64;
            // 974b5 mov rcx,QWORD PTR [rbp-0xec8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f138)) as u64;
            // 974bc mov QWORD PTR [rbp-0xeb0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f150), (rcx) as u64);
            // 974c3 mov QWORD PTR [rbp-0xeb8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f148), (rax) as u64);
            // 974ca xorps xmm0,xmm0
            x0 = 0;
            // 974cd movaps XMMWORD PTR [rbp-0xed0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f130), x0);
            // 974d4 mov QWORD PTR [rbp-0xec0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f140), (0x0u64) as u64);
            // 974df mov DWORD PTR [rbp-0xea0],0x16b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f160), (0x16bu64) as u32);
            // 974e9 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 974f0 lea rsi,[rbp-0xeb8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f148);
            // 974f7 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 974fc lea rdi,[rbp-0xeb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f148);
            // 97503 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97508 lea rdi,[rbp-0xed0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f130);
            // 9750f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x97514;
        }
        0x97514 => {
            // block 97514
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97514 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97514 xorps xmm0,xmm0
            x0 = 0;
            // 97517 movaps XMMWORD PTR [rbp-0xef0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f110), x0);
            // 9751e mov QWORD PTR [rbp-0xee0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f120), (0x0u64) as u64);
            // 97529 lea rdi,[rbp-0xef0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f110);
            // 97530 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97535 lea rsi,[rip+0xa178c] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 9753c lea rbx,[rbp-0xf08]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f0f8);
            // 97543 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 97545 mov rdi,rbx
            rdi = rbx;
            // 97548 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9754d mov esi,0x16c
            rsi = (0x16cu64) as u32 as u64;
            // 97552 mov rdi,r15
            rdi = r15;
            // 97555 mov rdx,rbx
            rdx = rbx;
            // 97558 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9755d lea rdi,[rbp-0xf08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0f8);
            // 97564 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97569 mov DWORD PTR [rbp-0xf18],0x16c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0e8), (0x16cu64) as u32);
            // 97573 mov QWORD PTR [rbp-0xf10],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f0f0), (r15) as u64);
            // 9757a lea rsi,[rbp-0xf18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0e8);
            // 97581 mov rdi,r14
            rdi = r14;
            // 97584 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97589 lea rsi,[rip+0x9e01b] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 97590 lea rdi,[rbp-0xf30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0d0);
            // 97597 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 9759c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 975a1 movzx ebx,BYTE PTR [rbp-0xf30]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f0d0)) as u64) as u32 as u64;
            // 975a8 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 975ab jne 0x975b2
            if crate::cov::brk(0x975ab, fl.ne()) { pc = 0x975b2; continue; }
            pc = 0x975ad;
        }
        0x975ad => {
            // block 975ad
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 975ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 975ad shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 975b0 jmp 0x975b9
            pc = 0x975b9; continue;
        }
        0x975b2 => {
            // block 975b2
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 975b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 975b2 mov rbx,QWORD PTR [rbp-0xf28]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f0d8)) as u64;
            pc = 0x975b9;
        }
        0x975b9 => {
            // block 975b9
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 975b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 975b9 lea rdi,[rbp-0xf30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0d0);
            // 975c0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 975c5 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 975c8 je 0x9765a
            if crate::cov::brk(0x975c8, fl.e()) { pc = 0x9765a; continue; }
            pc = 0x975ce;
        }
        0x975ce => {
            // block 975ce
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 975ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 975ce lea rsi,[rip+0x9dfd6] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 975d5 lea rdi,[rbp-0xf70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f090);
            // 975dc mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 975e1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 975e6 mov rax,QWORD PTR [rbp-0xf60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f0a0)) as u64;
            // 975ed mov QWORD PTR [rbp-0xf40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f0c0), (rax) as u64);
            // 975f4 mov rax,QWORD PTR [rbp-0xf70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f090)) as u64;
            // 975fb mov rcx,QWORD PTR [rbp-0xf68]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f098)) as u64;
            // 97602 mov QWORD PTR [rbp-0xf48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f0b8), (rcx) as u64);
            // 97609 mov QWORD PTR [rbp-0xf50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f0b0), (rax) as u64);
            // 97610 xorps xmm0,xmm0
            x0 = 0;
            // 97613 movaps XMMWORD PTR [rbp-0xf70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f090), x0);
            // 9761a mov QWORD PTR [rbp-0xf60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f0a0), (0x0u64) as u64);
            // 97625 mov DWORD PTR [rbp-0xf38],0x16c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0c8), (0x16cu64) as u32);
            // 9762f lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 97636 lea rsi,[rbp-0xf50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0b0);
            // 9763d call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97642 lea rdi,[rbp-0xf50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0b0);
            // 97649 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9764e lea rdi,[rbp-0xf70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f090);
            // 97655 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x9765a;
        }
        0x9765a => {
            // block 9765a
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9765a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9765a lea rsi,[rip+0x9c3aa] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 97661 lea rdi,[rbp-0xf88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f078);
            // 97668 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9766d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97672 movzx ebx,BYTE PTR [rbp-0xf88]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f078)) as u64) as u32 as u64;
            // 97679 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9767c jne 0x97683
            if crate::cov::brk(0x9767c, fl.ne()) { pc = 0x97683; continue; }
            pc = 0x9767e;
        }
        0x9767e => {
            // block 9767e
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9767e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9767e shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 97681 jmp 0x9768a
            pc = 0x9768a; continue;
        }
        0x97683 => {
            // block 97683
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97683 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97683 mov rbx,QWORD PTR [rbp-0xf80]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f080)) as u64;
            pc = 0x9768a;
        }
        0x9768a => {
            // block 9768a
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9768a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9768a lea rdi,[rbp-0xf88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f078);
            // 97691 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97696 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 97699 je 0x9772b
            if crate::cov::brk(0x97699, fl.e()) { pc = 0x9772b; continue; }
            pc = 0x9769f;
        }
        0x9769f => {
            // block 9769f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9769f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9769f lea rsi,[rip+0x9c365] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 976a6 lea rdi,[rbp-0xfc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f040);
            // 976ad mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 976b2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 976b7 mov rax,QWORD PTR [rbp-0xfb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f050)) as u64;
            // 976be mov QWORD PTR [rbp-0xf98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f068), (rax) as u64);
            // 976c5 mov rax,QWORD PTR [rbp-0xfc0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f040)) as u64;
            // 976cc mov rcx,QWORD PTR [rbp-0xfb8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f048)) as u64;
            // 976d3 mov QWORD PTR [rbp-0xfa0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f060), (rcx) as u64);
            // 976da mov QWORD PTR [rbp-0xfa8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f058), (rax) as u64);
            // 976e1 xorps xmm0,xmm0
            x0 = 0;
            // 976e4 movaps XMMWORD PTR [rbp-0xfc0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f040), x0);
            // 976eb mov QWORD PTR [rbp-0xfb0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f050), (0x0u64) as u64);
            // 976f6 mov DWORD PTR [rbp-0xf90],0x16c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f070), (0x16cu64) as u32);
            // 97700 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 97707 lea rsi,[rbp-0xfa8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f058);
            // 9770e call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97713 lea rdi,[rbp-0xfa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f058);
            // 9771a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9771f lea rdi,[rbp-0xfc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f040);
            // 97726 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x9772b;
        }
        0x9772b => {
            // block 9772b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9772b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9772b xorps xmm0,xmm0
            x0 = 0;
            // 9772e movaps XMMWORD PTR [rbp-0xfe0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f020), x0);
            // 97735 mov QWORD PTR [rbp-0xfd0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f030), (0x0u64) as u64);
            // 97740 lea rdi,[rbp-0xfe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f020);
            // 97747 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9774c lea rsi,[rip+0xa1575] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 97753 lea rbx,[rbp-0xff8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f008);
            // 9775a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 9775c mov rdi,rbx
            rdi = rbx;
            // 9775f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97764 mov esi,0x16d
            rsi = (0x16du64) as u32 as u64;
            // 97769 mov rdi,r15
            rdi = r15;
            // 9776c mov rdx,rbx
            rdx = rbx;
            // 9776f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97774 lea rdi,[rbp-0xff8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f008);
            // 9777b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97780 mov DWORD PTR [rbp-0x1008],0x16d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eff8), (0x16du64) as u32);
            // 9778a mov QWORD PTR [rbp-0x1000],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f000), (r15) as u64);
            // 97791 lea rsi,[rbp-0x1008]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eff8);
            // 97798 mov rdi,r14
            rdi = r14;
            // 9779b call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 977a0 lea rsi,[rip+0x9de04] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 977a7 lea rdi,[rbp-0x1020]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efe0);
            // 977ae mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 977b3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 977b8 movzx ebx,BYTE PTR [rbp-0x1020]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_efe0)) as u64) as u32 as u64;
            // 977bf test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 977c2 jne 0x977c9
            if crate::cov::brk(0x977c2, fl.ne()) { pc = 0x977c9; continue; }
            pc = 0x977c4;
        }
        0x977c4 => {
            // block 977c4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 977c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 977c4 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 977c7 jmp 0x977d0
            pc = 0x977d0; continue;
        }
        0x977c9 => {
            // block 977c9
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 977c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 977c9 mov rbx,QWORD PTR [rbp-0x1018]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_efe8)) as u64;
            pc = 0x977d0;
        }
        0x977d0 => {
            // block 977d0
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 977d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 977d0 lea rdi,[rbp-0x1020]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efe0);
            // 977d7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 977dc test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 977df je 0x97871
            if crate::cov::brk(0x977df, fl.e()) { pc = 0x97871; continue; }
            pc = 0x977e5;
        }
        0x977e5 => {
            // block 977e5
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 977e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 977e5 lea rsi,[rip+0x9ddbf] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 977ec lea rdi,[rbp-0x1060]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efa0);
            // 977f3 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 977f8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 977fd mov rax,QWORD PTR [rbp-0x1050]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_efb0)) as u64;
            // 97804 mov QWORD PTR [rbp-0x1030],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efd0), (rax) as u64);
            // 9780b mov rax,QWORD PTR [rbp-0x1060]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_efa0)) as u64;
            // 97812 mov rcx,QWORD PTR [rbp-0x1058]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_efa8)) as u64;
            // 97819 mov QWORD PTR [rbp-0x1038],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efc8), (rcx) as u64);
            // 97820 mov QWORD PTR [rbp-0x1040],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efc0), (rax) as u64);
            // 97827 xorps xmm0,xmm0
            x0 = 0;
            // 9782a movaps XMMWORD PTR [rbp-0x1060],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_efa0), x0);
            // 97831 mov QWORD PTR [rbp-0x1050],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efb0), (0x0u64) as u64);
            // 9783c mov DWORD PTR [rbp-0x1028],0x16d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_efd8), (0x16du64) as u32);
            // 97846 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 9784d lea rsi,[rbp-0x1040]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efc0);
            // 97854 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97859 lea rdi,[rbp-0x1040]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efc0);
            // 97860 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97865 lea rdi,[rbp-0x1060]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efa0);
            // 9786c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x97871;
        }
        0x97871 => {
            // block 97871
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97871 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97871 lea rsi,[rip+0x9c193] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 97878 lea rdi,[rbp-0x1078]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef88);
            // 9787f mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 97884 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97889 movzx ebx,BYTE PTR [rbp-0x1078]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ef88)) as u64) as u32 as u64;
            // 97890 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 97893 jne 0x9789a
            if crate::cov::brk(0x97893, fl.ne()) { pc = 0x9789a; continue; }
            pc = 0x97895;
        }
        0x97895 => {
            // block 97895
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97895 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97895 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 97898 jmp 0x978a1
            pc = 0x978a1; continue;
        }
        0x9789a => {
            // block 9789a
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9789a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9789a mov rbx,QWORD PTR [rbp-0x1070]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ef90)) as u64;
            pc = 0x978a1;
        }
        0x978a1 => {
            // block 978a1
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 978a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 978a1 lea rdi,[rbp-0x1078]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef88);
            // 978a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 978ad test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 978b0 je 0x97942
            if crate::cov::brk(0x978b0, fl.e()) { pc = 0x97942; continue; }
            pc = 0x978b6;
        }
        0x978b6 => {
            // block 978b6
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 978b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 978b6 lea rsi,[rip+0x9c14e] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 978bd lea rdi,[rbp-0x10b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef50);
            // 978c4 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 978c9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 978ce mov rax,QWORD PTR [rbp-0x10a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ef60)) as u64;
            // 978d5 mov QWORD PTR [rbp-0x1088],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef78), (rax) as u64);
            // 978dc mov rax,QWORD PTR [rbp-0x10b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ef50)) as u64;
            // 978e3 mov rcx,QWORD PTR [rbp-0x10a8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ef58)) as u64;
            // 978ea mov QWORD PTR [rbp-0x1090],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef70), (rcx) as u64);
            // 978f1 mov QWORD PTR [rbp-0x1098],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef68), (rax) as u64);
            // 978f8 xorps xmm0,xmm0
            x0 = 0;
            // 978fb movaps XMMWORD PTR [rbp-0x10b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ef50), x0);
            // 97902 mov QWORD PTR [rbp-0x10a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef60), (0x0u64) as u64);
            // 9790d mov DWORD PTR [rbp-0x1080],0x16d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef80), (0x16du64) as u32);
            // 97917 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 9791e lea rsi,[rbp-0x1098]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef68);
            // 97925 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9792a lea rdi,[rbp-0x1098]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef68);
            // 97931 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97936 lea rdi,[rbp-0x10b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef50);
            // 9793d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x97942;
        }
        0x97942 => {
            // block 97942
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97942 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97942 xorps xmm0,xmm0
            x0 = 0;
            // 97945 movaps XMMWORD PTR [rbp-0x10d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ef30), x0);
            // 9794c mov QWORD PTR [rbp-0x10c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef40), (0x0u64) as u64);
            // 97957 lea rdi,[rbp-0x10d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef30);
            // 9795e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97963 lea rsi,[rip+0xa135e] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 9796a lea rbx,[rbp-0x10e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ef18);
            // 97971 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 97973 mov rdi,rbx
            rdi = rbx;
            // 97976 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9797b mov esi,0x16e
            rsi = (0x16eu64) as u32 as u64;
            // 97980 mov rdi,r15
            rdi = r15;
            // 97983 mov rdx,rbx
            rdx = rbx;
            // 97986 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9798b lea rdi,[rbp-0x10e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef18);
            // 97992 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97997 mov DWORD PTR [rbp-0x10f8],0x16e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef08), (0x16eu64) as u32);
            // 979a1 mov QWORD PTR [rbp-0x10f0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef10), (r15) as u64);
            // 979a8 lea rsi,[rbp-0x10f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef08);
            // 979af mov rdi,r14
            rdi = r14;
            // 979b2 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 979b7 lea rsi,[rip+0x9dbed] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 979be lea rdi,[rbp-0x1110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eef0);
            // 979c5 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 979ca call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 979cf movzx ebx,BYTE PTR [rbp-0x1110]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_eef0)) as u64) as u32 as u64;
            // 979d6 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 979d9 jne 0x979e0
            if crate::cov::brk(0x979d9, fl.ne()) { pc = 0x979e0; continue; }
            pc = 0x979db;
        }
        0x979db => {
            // block 979db
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 979db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 979db shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 979de jmp 0x979e7
            pc = 0x979e7; continue;
        }
        0x979e0 => {
            // block 979e0
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 979e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 979e0 mov rbx,QWORD PTR [rbp-0x1108]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eef8)) as u64;
            pc = 0x979e7;
        }
        0x979e7 => {
            // block 979e7
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 979e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 979e7 lea rdi,[rbp-0x1110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eef0);
            // 979ee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 979f3 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 979f6 je 0x97a88
            if crate::cov::brk(0x979f6, fl.e()) { pc = 0x97a88; continue; }
            pc = 0x979fc;
        }
        0x979fc => {
            // block 979fc
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 979fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 979fc lea rsi,[rip+0x9dba8] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 97a03 lea rdi,[rbp-0x1150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eeb0);
            // 97a0a mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 97a0f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97a14 mov rax,QWORD PTR [rbp-0x1140]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eec0)) as u64;
            // 97a1b mov QWORD PTR [rbp-0x1120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eee0), (rax) as u64);
            // 97a22 mov rax,QWORD PTR [rbp-0x1150]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eeb0)) as u64;
            // 97a29 mov rcx,QWORD PTR [rbp-0x1148]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eeb8)) as u64;
            // 97a30 mov QWORD PTR [rbp-0x1128],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eed8), (rcx) as u64);
            // 97a37 mov QWORD PTR [rbp-0x1130],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eed0), (rax) as u64);
            // 97a3e xorps xmm0,xmm0
            x0 = 0;
            // 97a41 movaps XMMWORD PTR [rbp-0x1150],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_eeb0), x0);
            // 97a48 mov QWORD PTR [rbp-0x1140],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eec0), (0x0u64) as u64);
            // 97a53 mov DWORD PTR [rbp-0x1118],0x16e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eee8), (0x16eu64) as u32);
            // 97a5d lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 97a64 lea rsi,[rbp-0x1130]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eed0);
            // 97a6b call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97a70 lea rdi,[rbp-0x1130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eed0);
            // 97a77 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97a7c lea rdi,[rbp-0x1150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eeb0);
            // 97a83 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x97a88;
        }
        0x97a88 => {
            // block 97a88
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97a88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97a88 lea rsi,[rip+0x9bf7c] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 97a8f lea rdi,[rbp-0x1168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee98);
            // 97a96 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 97a9b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97aa0 movzx ebx,BYTE PTR [rbp-0x1168]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ee98)) as u64) as u32 as u64;
            // 97aa7 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 97aaa jne 0x97ab1
            if crate::cov::brk(0x97aaa, fl.ne()) { pc = 0x97ab1; continue; }
            pc = 0x97aac;
        }
        0x97aac => {
            // block 97aac
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97aac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97aac shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 97aaf jmp 0x97ab8
            pc = 0x97ab8; continue;
        }
        0x97ab1 => {
            // block 97ab1
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97ab1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97ab1 mov rbx,QWORD PTR [rbp-0x1160]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eea0)) as u64;
            pc = 0x97ab8;
        }
        0x97ab8 => {
            // block 97ab8
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97ab8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97ab8 lea rdi,[rbp-0x1168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee98);
            // 97abf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97ac4 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 97ac7 je 0x97b59
            if crate::cov::brk(0x97ac7, fl.e()) { pc = 0x97b59; continue; }
            pc = 0x97acd;
        }
        0x97acd => {
            // block 97acd
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97acd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97acd lea rsi,[rip+0x9bf37] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 97ad4 lea rdi,[rbp-0x11a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee60);
            // 97adb mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 97ae0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97ae5 mov rax,QWORD PTR [rbp-0x1190]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ee70)) as u64;
            // 97aec mov QWORD PTR [rbp-0x1178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee88), (rax) as u64);
            // 97af3 mov rax,QWORD PTR [rbp-0x11a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ee60)) as u64;
            // 97afa mov rcx,QWORD PTR [rbp-0x1198]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ee68)) as u64;
            // 97b01 mov QWORD PTR [rbp-0x1180],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee80), (rcx) as u64);
            // 97b08 mov QWORD PTR [rbp-0x1188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee78), (rax) as u64);
            // 97b0f xorps xmm0,xmm0
            x0 = 0;
            // 97b12 movaps XMMWORD PTR [rbp-0x11a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ee60), x0);
            // 97b19 mov QWORD PTR [rbp-0x1190],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee70), (0x0u64) as u64);
            // 97b24 mov DWORD PTR [rbp-0x1170],0x16e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee90), (0x16eu64) as u32);
            // 97b2e lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 97b35 lea rsi,[rbp-0x1188]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee78);
            // 97b3c call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97b41 lea rdi,[rbp-0x1188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee78);
            // 97b48 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97b4d lea rdi,[rbp-0x11a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee60);
            // 97b54 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x97b59;
        }
        0x97b59 => {
            // block 97b59
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97b59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97b59 xorps xmm0,xmm0
            x0 = 0;
            // 97b5c movaps XMMWORD PTR [rbp-0x11c0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ee40), x0);
            // 97b63 mov QWORD PTR [rbp-0x11b0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee50), (0x0u64) as u64);
            // 97b6e lea rdi,[rbp-0x11c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee40);
            // 97b75 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97b7a lea rsi,[rip+0xa1147] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 97b81 lea rbx,[rbp-0x11d8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ee28);
            // 97b88 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 97b8a mov rdi,rbx
            rdi = rbx;
            // 97b8d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97b92 mov esi,0x16f
            rsi = (0x16fu64) as u32 as u64;
            // 97b97 mov rdi,r15
            rdi = r15;
            // 97b9a mov rdx,rbx
            rdx = rbx;
            // 97b9d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97ba2 lea rdi,[rbp-0x11d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee28);
            // 97ba9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97bae mov DWORD PTR [rbp-0x11e8],0x16f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee18), (0x16fu64) as u32);
            // 97bb8 mov QWORD PTR [rbp-0x11e0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee20), (r15) as u64);
            // 97bbf lea rsi,[rbp-0x11e8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee18);
            // 97bc6 mov rdi,r14
            rdi = r14;
            // 97bc9 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97bce lea rsi,[rip+0x9d9d6] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 97bd5 lea rdi,[rbp-0x1200]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee00);
            // 97bdc mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 97be1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97be6 movzx ebx,BYTE PTR [rbp-0x1200]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ee00)) as u64) as u32 as u64;
            // 97bed test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 97bf0 jne 0x97bf7
            if crate::cov::brk(0x97bf0, fl.ne()) { pc = 0x97bf7; continue; }
            pc = 0x97bf2;
        }
        0x97bf2 => {
            // block 97bf2
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97bf2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97bf2 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 97bf5 jmp 0x97bfe
            pc = 0x97bfe; continue;
        }
        0x97bf7 => {
            // block 97bf7
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97bf7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97bf7 mov rbx,QWORD PTR [rbp-0x11f8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ee08)) as u64;
            pc = 0x97bfe;
        }
        0x97bfe => {
            // block 97bfe
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97bfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97bfe lea rdi,[rbp-0x1200]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee00);
            // 97c05 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97c0a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 97c0d je 0x97c9f
            if crate::cov::brk(0x97c0d, fl.e()) { pc = 0x97c9f; continue; }
            pc = 0x97c13;
        }
        0x97c13 => {
            // block 97c13
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97c13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97c13 lea rsi,[rip+0x9d991] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 97c1a lea rdi,[rbp-0x1240]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edc0);
            // 97c21 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 97c26 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97c2b mov rax,QWORD PTR [rbp-0x1230]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_edd0)) as u64;
            // 97c32 mov QWORD PTR [rbp-0x1210],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_edf0), (rax) as u64);
            // 97c39 mov rax,QWORD PTR [rbp-0x1240]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_edc0)) as u64;
            // 97c40 mov rcx,QWORD PTR [rbp-0x1238]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_edc8)) as u64;
            // 97c47 mov QWORD PTR [rbp-0x1218],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ede8), (rcx) as u64);
            // 97c4e mov QWORD PTR [rbp-0x1220],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ede0), (rax) as u64);
            // 97c55 xorps xmm0,xmm0
            x0 = 0;
            // 97c58 movaps XMMWORD PTR [rbp-0x1240],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_edc0), x0);
            // 97c5f mov QWORD PTR [rbp-0x1230],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_edd0), (0x0u64) as u64);
            // 97c6a mov DWORD PTR [rbp-0x1208],0x16f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edf8), (0x16fu64) as u32);
            // 97c74 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 97c7b lea rsi,[rbp-0x1220]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ede0);
            // 97c82 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97c87 lea rdi,[rbp-0x1220]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ede0);
            // 97c8e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97c93 lea rdi,[rbp-0x1240]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edc0);
            // 97c9a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x97c9f;
        }
        0x97c9f => {
            // block 97c9f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97c9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97c9f lea rsi,[rip+0x9bd65] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 97ca6 lea rdi,[rbp-0x1258]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eda8);
            // 97cad mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 97cb2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97cb7 movzx ebx,BYTE PTR [rbp-0x1258]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_eda8)) as u64) as u32 as u64;
            // 97cbe test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 97cc1 jne 0x97cc8
            if crate::cov::brk(0x97cc1, fl.ne()) { pc = 0x97cc8; continue; }
            pc = 0x97cc3;
        }
        0x97cc3 => {
            // block 97cc3
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97cc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97cc3 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 97cc6 jmp 0x97ccf
            pc = 0x97ccf; continue;
        }
        0x97cc8 => {
            // block 97cc8
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97cc8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97cc8 mov rbx,QWORD PTR [rbp-0x1250]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_edb0)) as u64;
            pc = 0x97ccf;
        }
        0x97ccf => {
            // block 97ccf
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97ccf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97ccf lea rdi,[rbp-0x1258]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eda8);
            // 97cd6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97cdb test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 97cde je 0x97d70
            if crate::cov::brk(0x97cde, fl.e()) { pc = 0x97d70; continue; }
            pc = 0x97ce4;
        }
        0x97ce4 => {
            // block 97ce4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97ce4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97ce4 lea rsi,[rip+0x9bd20] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 97ceb lea rdi,[rbp-0x1290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed70);
            // 97cf2 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 97cf7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97cfc mov rax,QWORD PTR [rbp-0x1280]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ed80)) as u64;
            // 97d03 mov QWORD PTR [rbp-0x1268],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed98), (rax) as u64);
            // 97d0a mov rax,QWORD PTR [rbp-0x1290]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ed70)) as u64;
            // 97d11 mov rcx,QWORD PTR [rbp-0x1288]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ed78)) as u64;
            // 97d18 mov QWORD PTR [rbp-0x1270],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed90), (rcx) as u64);
            // 97d1f mov QWORD PTR [rbp-0x1278],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed88), (rax) as u64);
            // 97d26 xorps xmm0,xmm0
            x0 = 0;
            // 97d29 movaps XMMWORD PTR [rbp-0x1290],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ed70), x0);
            // 97d30 mov QWORD PTR [rbp-0x1280],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed80), (0x0u64) as u64);
            // 97d3b mov DWORD PTR [rbp-0x1260],0x16f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eda0), (0x16fu64) as u32);
            // 97d45 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 97d4c lea rsi,[rbp-0x1278]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed88);
            // 97d53 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97d58 lea rdi,[rbp-0x1278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed88);
            // 97d5f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97d64 lea rdi,[rbp-0x1290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed70);
            // 97d6b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x97d70;
        }
        0x97d70 => {
            // block 97d70
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97d70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97d70 xorps xmm0,xmm0
            x0 = 0;
            // 97d73 movaps XMMWORD PTR [rbp-0x12b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ed50), x0);
            // 97d7a mov QWORD PTR [rbp-0x12a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed60), (0x0u64) as u64);
            // 97d85 lea rdi,[rbp-0x12b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed50);
            // 97d8c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97d91 lea rsi,[rip+0xa0f30] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 97d98 lea rbx,[rbp-0x12c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ed38);
            // 97d9f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 97da1 mov rdi,rbx
            rdi = rbx;
            // 97da4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97da9 mov esi,0x170
            rsi = (0x170u64) as u32 as u64;
            // 97dae mov rdi,r15
            rdi = r15;
            // 97db1 mov rdx,rbx
            rdx = rbx;
            // 97db4 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97db9 lea rdi,[rbp-0x12c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed38);
            // 97dc0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97dc5 mov DWORD PTR [rbp-0x12d8],0x170
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed28), (0x170u64) as u32);
            // 97dcf mov QWORD PTR [rbp-0x12d0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed30), (r15) as u64);
            // 97dd6 lea rsi,[rbp-0x12d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed28);
            // 97ddd mov rdi,r14
            rdi = r14;
            // 97de0 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97de5 lea rsi,[rip+0x9d7bf] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 97dec lea rdi,[rbp-0x12f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed10);
            // 97df3 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 97df8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97dfd movzx ebx,BYTE PTR [rbp-0x12f0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ed10)) as u64) as u32 as u64;
            // 97e04 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 97e07 jne 0x97e0e
            if crate::cov::brk(0x97e07, fl.ne()) { pc = 0x97e0e; continue; }
            pc = 0x97e09;
        }
        0x97e09 => {
            // block 97e09
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97e09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97e09 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 97e0c jmp 0x97e15
            pc = 0x97e15; continue;
        }
        0x97e0e => {
            // block 97e0e
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97e0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97e0e mov rbx,QWORD PTR [rbp-0x12e8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ed18)) as u64;
            pc = 0x97e15;
        }
        0x97e15 => {
            // block 97e15
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97e15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97e15 lea rdi,[rbp-0x12f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed10);
            // 97e1c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97e21 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 97e24 je 0x97eb6
            if crate::cov::brk(0x97e24, fl.e()) { pc = 0x97eb6; continue; }
            pc = 0x97e2a;
        }
        0x97e2a => {
            // block 97e2a
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97e2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97e2a lea rsi,[rip+0x9d77a] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 97e31 lea rdi,[rbp-0x1330]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecd0);
            // 97e38 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 97e3d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97e42 mov rax,QWORD PTR [rbp-0x1320]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ece0)) as u64;
            // 97e49 mov QWORD PTR [rbp-0x1300],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed00), (rax) as u64);
            // 97e50 mov rax,QWORD PTR [rbp-0x1330]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ecd0)) as u64;
            // 97e57 mov rcx,QWORD PTR [rbp-0x1328]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ecd8)) as u64;
            // 97e5e mov QWORD PTR [rbp-0x1308],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ecf8), (rcx) as u64);
            // 97e65 mov QWORD PTR [rbp-0x1310],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ecf0), (rax) as u64);
            // 97e6c xorps xmm0,xmm0
            x0 = 0;
            // 97e6f movaps XMMWORD PTR [rbp-0x1330],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ecd0), x0);
            // 97e76 mov QWORD PTR [rbp-0x1320],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ece0), (0x0u64) as u64);
            // 97e81 mov DWORD PTR [rbp-0x12f8],0x170
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed08), (0x170u64) as u32);
            // 97e8b lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 97e92 lea rsi,[rbp-0x1310]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ecf0);
            // 97e99 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97e9e lea rdi,[rbp-0x1310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecf0);
            // 97ea5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97eaa lea rdi,[rbp-0x1330]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecd0);
            // 97eb1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x97eb6;
        }
        0x97eb6 => {
            // block 97eb6
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97eb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97eb6 lea rsi,[rip+0x9bb4e] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 97ebd lea rdi,[rbp-0x1348]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecb8);
            // 97ec4 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 97ec9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97ece movzx ebx,BYTE PTR [rbp-0x1348]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ecb8)) as u64) as u32 as u64;
            // 97ed5 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 97ed8 jne 0x97edf
            if crate::cov::brk(0x97ed8, fl.ne()) { pc = 0x97edf; continue; }
            pc = 0x97eda;
        }
        0x97eda => {
            // block 97eda
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97eda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97eda shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 97edd jmp 0x97ee6
            pc = 0x97ee6; continue;
        }
        0x97edf => {
            // block 97edf
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97edf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97edf mov rbx,QWORD PTR [rbp-0x1340]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ecc0)) as u64;
            pc = 0x97ee6;
        }
        0x97ee6 => {
            // block 97ee6
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97ee6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97ee6 lea rdi,[rbp-0x1348]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecb8);
            // 97eed call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97ef2 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 97ef5 je 0x97f87
            if crate::cov::brk(0x97ef5, fl.e()) { pc = 0x97f87; continue; }
            pc = 0x97efb;
        }
        0x97efb => {
            // block 97efb
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97efb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97efb lea rsi,[rip+0x9bb09] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 97f02 lea rdi,[rbp-0x1380]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec80);
            // 97f09 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 97f0e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97f13 mov rax,QWORD PTR [rbp-0x1370]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ec90)) as u64;
            // 97f1a mov QWORD PTR [rbp-0x1358],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eca8), (rax) as u64);
            // 97f21 mov rax,QWORD PTR [rbp-0x1380]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ec80)) as u64;
            // 97f28 mov rcx,QWORD PTR [rbp-0x1378]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ec88)) as u64;
            // 97f2f mov QWORD PTR [rbp-0x1360],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eca0), (rcx) as u64);
            // 97f36 mov QWORD PTR [rbp-0x1368],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ec98), (rax) as u64);
            // 97f3d xorps xmm0,xmm0
            x0 = 0;
            // 97f40 movaps XMMWORD PTR [rbp-0x1380],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ec80), x0);
            // 97f47 mov QWORD PTR [rbp-0x1370],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ec90), (0x0u64) as u64);
            // 97f52 mov DWORD PTR [rbp-0x1350],0x170
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ecb0), (0x170u64) as u32);
            // 97f5c lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 97f63 lea rsi,[rbp-0x1368]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ec98);
            // 97f6a call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97f6f lea rdi,[rbp-0x1368]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec98);
            // 97f76 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97f7b lea rdi,[rbp-0x1380]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec80);
            // 97f82 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x97f87;
        }
        0x97f87 => {
            // block 97f87
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 97f87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 97f87 xorps xmm0,xmm0
            x0 = 0;
            // 97f8a movaps XMMWORD PTR [rbp-0x13a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ec60), x0);
            // 97f91 mov QWORD PTR [rbp-0x1390],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ec70), (0x0u64) as u64);
            // 97f9c lea rdi,[rbp-0x13a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec60);
            // 97fa3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97fa8 lea rsi,[rip+0xa0d19] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 97faf lea rbx,[rbp-0x13b8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ec48);
            // 97fb6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 97fb8 mov rdi,rbx
            rdi = rbx;
            // 97fbb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97fc0 mov esi,0x171
            rsi = (0x171u64) as u32 as u64;
            // 97fc5 mov rdi,r15
            rdi = r15;
            // 97fc8 mov rdx,rbx
            rdx = rbx;
            // 97fcb call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97fd0 lea rdi,[rbp-0x13b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec48);
            // 97fd7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97fdc mov DWORD PTR [rbp-0x13c8],0x171
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ec38), (0x171u64) as u32);
            // 97fe6 mov QWORD PTR [rbp-0x13c0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ec40), (r15) as u64);
            // 97fed lea rsi,[rbp-0x13c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ec38);
            // 97ff4 mov rdi,r14
            rdi = r14;
            // 97ff7 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 97ffc lea rsi,[rip+0x9d5a8] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 98003 lea rdi,[rbp-0x13e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec20);
            // 9800a mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 9800f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98014 movzx ebx,BYTE PTR [rbp-0x13e0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ec20)) as u64) as u32 as u64;
            // 9801b test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9801e jne 0x98025
            if crate::cov::brk(0x9801e, fl.ne()) { pc = 0x98025; continue; }
            pc = 0x98020;
        }
        0x98020 => {
            // block 98020
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98020 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98020 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 98023 jmp 0x9802c
            pc = 0x9802c; continue;
        }
        0x98025 => {
            // block 98025
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98025 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98025 mov rbx,QWORD PTR [rbp-0x13d8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ec28)) as u64;
            pc = 0x9802c;
        }
        0x9802c => {
            // block 9802c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9802c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9802c lea rdi,[rbp-0x13e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec20);
            // 98033 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98038 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9803b je 0x980cd
            if crate::cov::brk(0x9803b, fl.e()) { pc = 0x980cd; continue; }
            pc = 0x98041;
        }
        0x98041 => {
            // block 98041
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98041 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98041 lea rsi,[rip+0x9d563] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 98048 lea rdi,[rbp-0x1420]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebe0);
            // 9804f mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 98054 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98059 mov rax,QWORD PTR [rbp-0x1410]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ebf0)) as u64;
            // 98060 mov QWORD PTR [rbp-0x13f0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ec10), (rax) as u64);
            // 98067 mov rax,QWORD PTR [rbp-0x1420]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ebe0)) as u64;
            // 9806e mov rcx,QWORD PTR [rbp-0x1418]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ebe8)) as u64;
            // 98075 mov QWORD PTR [rbp-0x13f8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ec08), (rcx) as u64);
            // 9807c mov QWORD PTR [rbp-0x1400],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ec00), (rax) as u64);
            // 98083 xorps xmm0,xmm0
            x0 = 0;
            // 98086 movaps XMMWORD PTR [rbp-0x1420],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ebe0), x0);
            // 9808d mov QWORD PTR [rbp-0x1410],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ebf0), (0x0u64) as u64);
            // 98098 mov DWORD PTR [rbp-0x13e8],0x171
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ec18), (0x171u64) as u32);
            // 980a2 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 980a9 lea rsi,[rbp-0x1400]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ec00);
            // 980b0 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 980b5 lea rdi,[rbp-0x1400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec00);
            // 980bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 980c1 lea rdi,[rbp-0x1420]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebe0);
            // 980c8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x980cd;
        }
        0x980cd => {
            // block 980cd
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 980cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 980cd lea rsi,[rip+0x9b937] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 980d4 lea rdi,[rbp-0x1438]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebc8);
            // 980db mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 980e0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 980e5 movzx ebx,BYTE PTR [rbp-0x1438]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ebc8)) as u64) as u32 as u64;
            // 980ec test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 980ef jne 0x980f6
            if crate::cov::brk(0x980ef, fl.ne()) { pc = 0x980f6; continue; }
            pc = 0x980f1;
        }
        0x980f1 => {
            // block 980f1
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 980f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 980f1 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 980f4 jmp 0x980fd
            pc = 0x980fd; continue;
        }
        0x980f6 => {
            // block 980f6
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 980f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 980f6 mov rbx,QWORD PTR [rbp-0x1430]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ebd0)) as u64;
            pc = 0x980fd;
        }
        0x980fd => {
            // block 980fd
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 980fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 980fd lea rdi,[rbp-0x1438]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebc8);
            // 98104 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98109 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9810c je 0x9819e
            if crate::cov::brk(0x9810c, fl.e()) { pc = 0x9819e; continue; }
            pc = 0x98112;
        }
        0x98112 => {
            // block 98112
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98112 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98112 lea rsi,[rip+0x9b8f2] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 98119 lea rdi,[rbp-0x1470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb90);
            // 98120 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 98125 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9812a mov rax,QWORD PTR [rbp-0x1460]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eba0)) as u64;
            // 98131 mov QWORD PTR [rbp-0x1448],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ebb8), (rax) as u64);
            // 98138 mov rax,QWORD PTR [rbp-0x1470]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eb90)) as u64;
            // 9813f mov rcx,QWORD PTR [rbp-0x1468]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eb98)) as u64;
            // 98146 mov QWORD PTR [rbp-0x1450],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ebb0), (rcx) as u64);
            // 9814d mov QWORD PTR [rbp-0x1458],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eba8), (rax) as u64);
            // 98154 xorps xmm0,xmm0
            x0 = 0;
            // 98157 movaps XMMWORD PTR [rbp-0x1470],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_eb90), x0);
            // 9815e mov QWORD PTR [rbp-0x1460],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eba0), (0x0u64) as u64);
            // 98169 mov DWORD PTR [rbp-0x1440],0x171
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ebc0), (0x171u64) as u32);
            // 98173 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 9817a lea rsi,[rbp-0x1458]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eba8);
            // 98181 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98186 lea rdi,[rbp-0x1458]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eba8);
            // 9818d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98192 lea rdi,[rbp-0x1470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb90);
            // 98199 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x9819e;
        }
        0x9819e => {
            // block 9819e
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9819e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9819e xorps xmm0,xmm0
            x0 = 0;
            // 981a1 movaps XMMWORD PTR [rbp-0x1490],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_eb70), x0);
            // 981a8 mov QWORD PTR [rbp-0x1480],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eb80), (0x0u64) as u64);
            // 981b3 lea rdi,[rbp-0x1490]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb70);
            // 981ba call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 981bf lea rsi,[rip+0xa0b02] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 981c6 lea rbx,[rbp-0x14a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eb58);
            // 981cd xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 981cf mov rdi,rbx
            rdi = rbx;
            // 981d2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 981d7 mov esi,0x172
            rsi = (0x172u64) as u32 as u64;
            // 981dc mov rdi,r15
            rdi = r15;
            // 981df mov rdx,rbx
            rdx = rbx;
            // 981e2 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 981e7 lea rdi,[rbp-0x14a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb58);
            // 981ee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 981f3 mov DWORD PTR [rbp-0x14b8],0x172
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eb48), (0x172u64) as u32);
            // 981fd mov QWORD PTR [rbp-0x14b0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eb50), (r15) as u64);
            // 98204 lea rsi,[rbp-0x14b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eb48);
            // 9820b mov rdi,r14
            rdi = r14;
            // 9820e call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98213 lea rsi,[rip+0x9d391] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 9821a lea rdi,[rbp-0x14d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb30);
            // 98221 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 98226 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9822b movzx ebx,BYTE PTR [rbp-0x14d0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_eb30)) as u64) as u32 as u64;
            // 98232 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 98235 jne 0x9823c
            if crate::cov::brk(0x98235, fl.ne()) { pc = 0x9823c; continue; }
            pc = 0x98237;
        }
        0x98237 => {
            // block 98237
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98237 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98237 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9823a jmp 0x98243
            pc = 0x98243; continue;
        }
        0x9823c => {
            // block 9823c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9823c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9823c mov rbx,QWORD PTR [rbp-0x14c8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eb38)) as u64;
            pc = 0x98243;
        }
        0x98243 => {
            // block 98243
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98243 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98243 lea rdi,[rbp-0x14d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb30);
            // 9824a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9824f test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 98252 je 0x982e4
            if crate::cov::brk(0x98252, fl.e()) { pc = 0x982e4; continue; }
            pc = 0x98258;
        }
        0x98258 => {
            // block 98258
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98258 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98258 lea rsi,[rip+0x9d34c] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 9825f lea rdi,[rbp-0x1510]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eaf0);
            // 98266 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 9826b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98270 mov rax,QWORD PTR [rbp-0x1500]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eb00)) as u64;
            // 98277 mov QWORD PTR [rbp-0x14e0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eb20), (rax) as u64);
            // 9827e mov rax,QWORD PTR [rbp-0x1510]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eaf0)) as u64;
            // 98285 mov rcx,QWORD PTR [rbp-0x1508]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eaf8)) as u64;
            // 9828c mov QWORD PTR [rbp-0x14e8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eb18), (rcx) as u64);
            // 98293 mov QWORD PTR [rbp-0x14f0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eb10), (rax) as u64);
            // 9829a xorps xmm0,xmm0
            x0 = 0;
            // 9829d movaps XMMWORD PTR [rbp-0x1510],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_eaf0), x0);
            // 982a4 mov QWORD PTR [rbp-0x1500],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eb00), (0x0u64) as u64);
            // 982af mov DWORD PTR [rbp-0x14d8],0x172
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eb28), (0x172u64) as u32);
            // 982b9 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 982c0 lea rsi,[rbp-0x14f0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eb10);
            // 982c7 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 982cc lea rdi,[rbp-0x14f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb10);
            // 982d3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 982d8 lea rdi,[rbp-0x1510]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eaf0);
            // 982df call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x982e4;
        }
        0x982e4 => {
            // block 982e4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 982e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 982e4 lea rsi,[rip+0x9b720] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 982eb lea rdi,[rbp-0x1528]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ead8);
            // 982f2 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 982f7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 982fc movzx ebx,BYTE PTR [rbp-0x1528]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ead8)) as u64) as u32 as u64;
            // 98303 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 98306 jne 0x9830d
            if crate::cov::brk(0x98306, fl.ne()) { pc = 0x9830d; continue; }
            pc = 0x98308;
        }
        0x98308 => {
            // block 98308
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98308 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98308 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9830b jmp 0x98314
            pc = 0x98314; continue;
        }
        0x9830d => {
            // block 9830d
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9830d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9830d mov rbx,QWORD PTR [rbp-0x1520]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eae0)) as u64;
            pc = 0x98314;
        }
        0x98314 => {
            // block 98314
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98314 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98314 lea rdi,[rbp-0x1528]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ead8);
            // 9831b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98320 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 98323 je 0x983b5
            if crate::cov::brk(0x98323, fl.e()) { pc = 0x983b5; continue; }
            pc = 0x98329;
        }
        0x98329 => {
            // block 98329
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98329 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98329 lea rsi,[rip+0x9b6db] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 98330 lea rdi,[rbp-0x1560]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eaa0);
            // 98337 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9833c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98341 mov rax,QWORD PTR [rbp-0x1550]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eab0)) as u64;
            // 98348 mov QWORD PTR [rbp-0x1538],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eac8), (rax) as u64);
            // 9834f mov rax,QWORD PTR [rbp-0x1560]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eaa0)) as u64;
            // 98356 mov rcx,QWORD PTR [rbp-0x1558]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eaa8)) as u64;
            // 9835d mov QWORD PTR [rbp-0x1540],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eac0), (rcx) as u64);
            // 98364 mov QWORD PTR [rbp-0x1548],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eab8), (rax) as u64);
            // 9836b xorps xmm0,xmm0
            x0 = 0;
            // 9836e movaps XMMWORD PTR [rbp-0x1560],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_eaa0), x0);
            // 98375 mov QWORD PTR [rbp-0x1550],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eab0), (0x0u64) as u64);
            // 98380 mov DWORD PTR [rbp-0x1530],0x172
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ead0), (0x172u64) as u32);
            // 9838a lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 98391 lea rsi,[rbp-0x1548]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eab8);
            // 98398 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9839d lea rdi,[rbp-0x1548]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eab8);
            // 983a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 983a9 lea rdi,[rbp-0x1560]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eaa0);
            // 983b0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x983b5;
        }
        0x983b5 => {
            // block 983b5
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 983b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 983b5 xorps xmm0,xmm0
            x0 = 0;
            // 983b8 movaps XMMWORD PTR [rbp-0x1580],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ea80), x0);
            // 983bf mov QWORD PTR [rbp-0x1570],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ea90), (0x0u64) as u64);
            // 983ca lea rdi,[rbp-0x1580]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea80);
            // 983d1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 983d6 lea rsi,[rip+0xa08eb] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 983dd lea rbx,[rbp-0x1598]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ea68);
            // 983e4 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 983e6 mov rdi,rbx
            rdi = rbx;
            // 983e9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 983ee mov esi,0x173
            rsi = (0x173u64) as u32 as u64;
            // 983f3 mov rdi,r15
            rdi = r15;
            // 983f6 mov rdx,rbx
            rdx = rbx;
            // 983f9 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 983fe lea rdi,[rbp-0x1598]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea68);
            // 98405 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9840a mov DWORD PTR [rbp-0x15a8],0x173
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ea58), (0x173u64) as u32);
            // 98414 mov QWORD PTR [rbp-0x15a0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ea60), (r15) as u64);
            // 9841b lea rsi,[rbp-0x15a8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ea58);
            // 98422 mov rdi,r14
            rdi = r14;
            // 98425 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9842a lea rsi,[rip+0x9d17a] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 98431 lea rdi,[rbp-0x15c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea40);
            // 98438 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 9843d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98442 movzx ebx,BYTE PTR [rbp-0x15c0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ea40)) as u64) as u32 as u64;
            // 98449 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9844c jne 0x98453
            if crate::cov::brk(0x9844c, fl.ne()) { pc = 0x98453; continue; }
            pc = 0x9844e;
        }
        0x9844e => {
            // block 9844e
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9844e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9844e shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 98451 jmp 0x9845a
            pc = 0x9845a; continue;
        }
        0x98453 => {
            // block 98453
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98453 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98453 mov rbx,QWORD PTR [rbp-0x15b8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ea48)) as u64;
            pc = 0x9845a;
        }
        0x9845a => {
            // block 9845a
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9845a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9845a lea rdi,[rbp-0x15c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea40);
            // 98461 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98466 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 98469 je 0x984fb
            if crate::cov::brk(0x98469, fl.e()) { pc = 0x984fb; continue; }
            pc = 0x9846f;
        }
        0x9846f => {
            // block 9846f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9846f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9846f lea rsi,[rip+0x9d135] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 98476 lea rdi,[rbp-0x1600]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea00);
            // 9847d mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 98482 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98487 mov rax,QWORD PTR [rbp-0x15f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ea10)) as u64;
            // 9848e mov QWORD PTR [rbp-0x15d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ea30), (rax) as u64);
            // 98495 mov rax,QWORD PTR [rbp-0x1600]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ea00)) as u64;
            // 9849c mov rcx,QWORD PTR [rbp-0x15f8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ea08)) as u64;
            // 984a3 mov QWORD PTR [rbp-0x15d8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ea28), (rcx) as u64);
            // 984aa mov QWORD PTR [rbp-0x15e0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ea20), (rax) as u64);
            // 984b1 xorps xmm0,xmm0
            x0 = 0;
            // 984b4 movaps XMMWORD PTR [rbp-0x1600],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ea00), x0);
            // 984bb mov QWORD PTR [rbp-0x15f0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ea10), (0x0u64) as u64);
            // 984c6 mov DWORD PTR [rbp-0x15c8],0x173
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ea38), (0x173u64) as u32);
            // 984d0 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 984d7 lea rsi,[rbp-0x15e0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ea20);
            // 984de call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 984e3 lea rdi,[rbp-0x15e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea20);
            // 984ea call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 984ef lea rdi,[rbp-0x1600]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea00);
            // 984f6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x984fb;
        }
        0x984fb => {
            // block 984fb
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 984fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 984fb lea rsi,[rip+0x9b509] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 98502 lea rdi,[rbp-0x1618]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9e8);
            // 98509 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9850e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98513 movzx ebx,BYTE PTR [rbp-0x1618]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e9e8)) as u64) as u32 as u64;
            // 9851a test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9851d jne 0x98524
            if crate::cov::brk(0x9851d, fl.ne()) { pc = 0x98524; continue; }
            pc = 0x9851f;
        }
        0x9851f => {
            // block 9851f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9851f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9851f shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 98522 jmp 0x9852b
            pc = 0x9852b; continue;
        }
        0x98524 => {
            // block 98524
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98524 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98524 mov rbx,QWORD PTR [rbp-0x1610]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e9f0)) as u64;
            pc = 0x9852b;
        }
        0x9852b => {
            // block 9852b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9852b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9852b lea rdi,[rbp-0x1618]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9e8);
            // 98532 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98537 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9853a je 0x985cc
            if crate::cov::brk(0x9853a, fl.e()) { pc = 0x985cc; continue; }
            pc = 0x98540;
        }
        0x98540 => {
            // block 98540
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98540 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98540 lea rsi,[rip+0x9b4c4] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 98547 lea rdi,[rbp-0x1650]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9b0);
            // 9854e mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 98553 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98558 mov rax,QWORD PTR [rbp-0x1640]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e9c0)) as u64;
            // 9855f mov QWORD PTR [rbp-0x1628],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e9d8), (rax) as u64);
            // 98566 mov rax,QWORD PTR [rbp-0x1650]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e9b0)) as u64;
            // 9856d mov rcx,QWORD PTR [rbp-0x1648]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e9b8)) as u64;
            // 98574 mov QWORD PTR [rbp-0x1630],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e9d0), (rcx) as u64);
            // 9857b mov QWORD PTR [rbp-0x1638],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e9c8), (rax) as u64);
            // 98582 xorps xmm0,xmm0
            x0 = 0;
            // 98585 movaps XMMWORD PTR [rbp-0x1650],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e9b0), x0);
            // 9858c mov QWORD PTR [rbp-0x1640],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e9c0), (0x0u64) as u64);
            // 98597 mov DWORD PTR [rbp-0x1620],0x173
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e9e0), (0x173u64) as u32);
            // 985a1 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 985a8 lea rsi,[rbp-0x1638]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e9c8);
            // 985af call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 985b4 lea rdi,[rbp-0x1638]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9c8);
            // 985bb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 985c0 lea rdi,[rbp-0x1650]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9b0);
            // 985c7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x985cc;
        }
        0x985cc => {
            // block 985cc
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 985cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 985cc xorps xmm0,xmm0
            x0 = 0;
            // 985cf movaps XMMWORD PTR [rbp-0x1670],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e990), x0);
            // 985d6 mov QWORD PTR [rbp-0x1660],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e9a0), (0x0u64) as u64);
            // 985e1 lea rdi,[rbp-0x1670]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e990);
            // 985e8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 985ed lea rsi,[rip+0xa06d4] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 985f4 lea rbx,[rbp-0x1688]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e978);
            // 985fb xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 985fd mov rdi,rbx
            rdi = rbx;
            // 98600 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98605 mov esi,0x174
            rsi = (0x174u64) as u32 as u64;
            // 9860a mov rdi,r15
            rdi = r15;
            // 9860d mov rdx,rbx
            rdx = rbx;
            // 98610 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98615 lea rdi,[rbp-0x1688]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e978);
            // 9861c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98621 mov DWORD PTR [rbp-0x1698],0x174
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e968), (0x174u64) as u32);
            // 9862b mov QWORD PTR [rbp-0x1690],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e970), (r15) as u64);
            // 98632 lea rsi,[rbp-0x1698]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e968);
            // 98639 mov rdi,r14
            rdi = r14;
            // 9863c call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98641 lea rsi,[rip+0x9cf63] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 98648 lea rdi,[rbp-0x16b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e950);
            // 9864f mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 98654 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98659 movzx ebx,BYTE PTR [rbp-0x16b0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e950)) as u64) as u32 as u64;
            // 98660 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 98663 jne 0x9866a
            if crate::cov::brk(0x98663, fl.ne()) { pc = 0x9866a; continue; }
            pc = 0x98665;
        }
        0x98665 => {
            // block 98665
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98665 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98665 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 98668 jmp 0x98671
            pc = 0x98671; continue;
        }
        0x9866a => {
            // block 9866a
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9866a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9866a mov rbx,QWORD PTR [rbp-0x16a8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e958)) as u64;
            pc = 0x98671;
        }
        0x98671 => {
            // block 98671
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98671 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98671 lea rdi,[rbp-0x16b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e950);
            // 98678 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9867d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 98680 je 0x98712
            if crate::cov::brk(0x98680, fl.e()) { pc = 0x98712; continue; }
            pc = 0x98686;
        }
        0x98686 => {
            // block 98686
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98686 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98686 lea rsi,[rip+0x9cf1e] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 9868d lea rdi,[rbp-0x16f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e910);
            // 98694 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 98699 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9869e mov rax,QWORD PTR [rbp-0x16e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e920)) as u64;
            // 986a5 mov QWORD PTR [rbp-0x16c0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e940), (rax) as u64);
            // 986ac mov rax,QWORD PTR [rbp-0x16f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e910)) as u64;
            // 986b3 mov rcx,QWORD PTR [rbp-0x16e8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e918)) as u64;
            // 986ba mov QWORD PTR [rbp-0x16c8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e938), (rcx) as u64);
            // 986c1 mov QWORD PTR [rbp-0x16d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e930), (rax) as u64);
            // 986c8 xorps xmm0,xmm0
            x0 = 0;
            // 986cb movaps XMMWORD PTR [rbp-0x16f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e910), x0);
            // 986d2 mov QWORD PTR [rbp-0x16e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e920), (0x0u64) as u64);
            // 986dd mov DWORD PTR [rbp-0x16b8],0x174
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e948), (0x174u64) as u32);
            // 986e7 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 986ee lea rsi,[rbp-0x16d0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e930);
            // 986f5 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 986fa lea rdi,[rbp-0x16d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e930);
            // 98701 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98706 lea rdi,[rbp-0x16f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e910);
            // 9870d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x98712;
        }
        0x98712 => {
            // block 98712
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98712 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98712 lea rsi,[rip+0x9b2f2] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 98719 lea rdi,[rbp-0x1708]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8f8);
            // 98720 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 98725 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9872a movzx ebx,BYTE PTR [rbp-0x1708]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e8f8)) as u64) as u32 as u64;
            // 98731 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 98734 jne 0x9873b
            if crate::cov::brk(0x98734, fl.ne()) { pc = 0x9873b; continue; }
            pc = 0x98736;
        }
        0x98736 => {
            // block 98736
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98736 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98736 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 98739 jmp 0x98742
            pc = 0x98742; continue;
        }
        0x9873b => {
            // block 9873b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9873b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9873b mov rbx,QWORD PTR [rbp-0x1700]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e900)) as u64;
            pc = 0x98742;
        }
        0x98742 => {
            // block 98742
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98742 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98742 lea rdi,[rbp-0x1708]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8f8);
            // 98749 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9874e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 98751 je 0x987e3
            if crate::cov::brk(0x98751, fl.e()) { pc = 0x987e3; continue; }
            pc = 0x98757;
        }
        0x98757 => {
            // block 98757
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98757 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98757 lea rsi,[rip+0x9b2ad] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 9875e lea rdi,[rbp-0x1740]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8c0);
            // 98765 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9876a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9876f mov rax,QWORD PTR [rbp-0x1730]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e8d0)) as u64;
            // 98776 mov QWORD PTR [rbp-0x1718],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e8e8), (rax) as u64);
            // 9877d mov rax,QWORD PTR [rbp-0x1740]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e8c0)) as u64;
            // 98784 mov rcx,QWORD PTR [rbp-0x1738]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e8c8)) as u64;
            // 9878b mov QWORD PTR [rbp-0x1720],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e8e0), (rcx) as u64);
            // 98792 mov QWORD PTR [rbp-0x1728],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e8d8), (rax) as u64);
            // 98799 xorps xmm0,xmm0
            x0 = 0;
            // 9879c movaps XMMWORD PTR [rbp-0x1740],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e8c0), x0);
            // 987a3 mov QWORD PTR [rbp-0x1730],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e8d0), (0x0u64) as u64);
            // 987ae mov DWORD PTR [rbp-0x1710],0x174
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e8f0), (0x174u64) as u32);
            // 987b8 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 987bf lea rsi,[rbp-0x1728]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e8d8);
            // 987c6 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 987cb lea rdi,[rbp-0x1728]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8d8);
            // 987d2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 987d7 lea rdi,[rbp-0x1740]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8c0);
            // 987de call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x987e3;
        }
        0x987e3 => {
            // block 987e3
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 987e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 987e3 xorps xmm0,xmm0
            x0 = 0;
            // 987e6 movaps XMMWORD PTR [rbp-0x1760],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e8a0), x0);
            // 987ed mov QWORD PTR [rbp-0x1750],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e8b0), (0x0u64) as u64);
            // 987f8 lea rdi,[rbp-0x1760]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8a0);
            // 987ff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98804 lea rsi,[rip+0xa04bd] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 9880b lea rbx,[rbp-0x1778]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e888);
            // 98812 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 98814 mov rdi,rbx
            rdi = rbx;
            // 98817 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9881c mov esi,0x175
            rsi = (0x175u64) as u32 as u64;
            // 98821 mov rdi,r15
            rdi = r15;
            // 98824 mov rdx,rbx
            rdx = rbx;
            // 98827 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9882c lea rdi,[rbp-0x1778]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e888);
            // 98833 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98838 mov DWORD PTR [rbp-0x1788],0x175
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e878), (0x175u64) as u32);
            // 98842 mov QWORD PTR [rbp-0x1780],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e880), (r15) as u64);
            // 98849 lea rsi,[rbp-0x1788]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e878);
            // 98850 mov rdi,r14
            rdi = r14;
            // 98853 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98858 lea rsi,[rip+0x9cd4c] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 9885f lea rdi,[rbp-0x17a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e860);
            // 98866 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 9886b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98870 movzx ebx,BYTE PTR [rbp-0x17a0]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e860)) as u64) as u32 as u64;
            // 98877 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9887a jne 0x98881
            if crate::cov::brk(0x9887a, fl.ne()) { pc = 0x98881; continue; }
            pc = 0x9887c;
        }
        0x9887c => {
            // block 9887c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9887c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9887c shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9887f jmp 0x98888
            pc = 0x98888; continue;
        }
        0x98881 => {
            // block 98881
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98881 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98881 mov rbx,QWORD PTR [rbp-0x1798]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e868)) as u64;
            pc = 0x98888;
        }
        0x98888 => {
            // block 98888
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98888 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98888 lea rdi,[rbp-0x17a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e860);
            // 9888f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98894 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 98897 je 0x98929
            if crate::cov::brk(0x98897, fl.e()) { pc = 0x98929; continue; }
            pc = 0x9889d;
        }
        0x9889d => {
            // block 9889d
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9889d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9889d lea rsi,[rip+0x9cd07] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 988a4 lea rdi,[rbp-0x17e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e820);
            // 988ab mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 988b0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 988b5 mov rax,QWORD PTR [rbp-0x17d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e830)) as u64;
            // 988bc mov QWORD PTR [rbp-0x17b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e850), (rax) as u64);
            // 988c3 mov rax,QWORD PTR [rbp-0x17e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e820)) as u64;
            // 988ca mov rcx,QWORD PTR [rbp-0x17d8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e828)) as u64;
            // 988d1 mov QWORD PTR [rbp-0x17b8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e848), (rcx) as u64);
            // 988d8 mov QWORD PTR [rbp-0x17c0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e840), (rax) as u64);
            // 988df xorps xmm0,xmm0
            x0 = 0;
            // 988e2 movaps XMMWORD PTR [rbp-0x17e0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e820), x0);
            // 988e9 mov QWORD PTR [rbp-0x17d0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e830), (0x0u64) as u64);
            // 988f4 mov DWORD PTR [rbp-0x17a8],0x175
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e858), (0x175u64) as u32);
            // 988fe lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 98905 lea rsi,[rbp-0x17c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e840);
            // 9890c call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98911 lea rdi,[rbp-0x17c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e840);
            // 98918 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9891d lea rdi,[rbp-0x17e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e820);
            // 98924 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x98929;
        }
        0x98929 => {
            // block 98929
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98929 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98929 lea rsi,[rip+0x9b0db] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 98930 lea rdi,[rbp-0x17f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e808);
            // 98937 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9893c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98941 movzx ebx,BYTE PTR [rbp-0x17f8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e808)) as u64) as u32 as u64;
            // 98948 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9894b jne 0x98952
            if crate::cov::brk(0x9894b, fl.ne()) { pc = 0x98952; continue; }
            pc = 0x9894d;
        }
        0x9894d => {
            // block 9894d
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9894d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9894d shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 98950 jmp 0x98959
            pc = 0x98959; continue;
        }
        0x98952 => {
            // block 98952
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98952 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98952 mov rbx,QWORD PTR [rbp-0x17f0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e810)) as u64;
            pc = 0x98959;
        }
        0x98959 => {
            // block 98959
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98959 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98959 lea rdi,[rbp-0x17f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e808);
            // 98960 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98965 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 98968 je 0x989fa
            if crate::cov::brk(0x98968, fl.e()) { pc = 0x989fa; continue; }
            pc = 0x9896e;
        }
        0x9896e => {
            // block 9896e
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9896e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9896e lea rsi,[rip+0x9b096] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 98975 lea rdi,[rbp-0x1830]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7d0);
            // 9897c mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 98981 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98986 mov rax,QWORD PTR [rbp-0x1820]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e7e0)) as u64;
            // 9898d mov QWORD PTR [rbp-0x1808],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e7f8), (rax) as u64);
            // 98994 mov rax,QWORD PTR [rbp-0x1830]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e7d0)) as u64;
            // 9899b mov rcx,QWORD PTR [rbp-0x1828]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e7d8)) as u64;
            // 989a2 mov QWORD PTR [rbp-0x1810],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e7f0), (rcx) as u64);
            // 989a9 mov QWORD PTR [rbp-0x1818],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e7e8), (rax) as u64);
            // 989b0 xorps xmm0,xmm0
            x0 = 0;
            // 989b3 movaps XMMWORD PTR [rbp-0x1830],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e7d0), x0);
            // 989ba mov QWORD PTR [rbp-0x1820],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e7e0), (0x0u64) as u64);
            // 989c5 mov DWORD PTR [rbp-0x1800],0x175
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e800), (0x175u64) as u32);
            // 989cf lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 989d6 lea rsi,[rbp-0x1818]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e7e8);
            // 989dd call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 989e2 lea rdi,[rbp-0x1818]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7e8);
            // 989e9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 989ee lea rdi,[rbp-0x1830]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7d0);
            // 989f5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x989fa;
        }
        0x989fa => {
            // block 989fa
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 989fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 989fa xorps xmm0,xmm0
            x0 = 0;
            // 989fd movaps XMMWORD PTR [rbp-0x1850],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e7b0), x0);
            // 98a04 mov QWORD PTR [rbp-0x1840],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e7c0), (0x0u64) as u64);
            // 98a0f lea rdi,[rbp-0x1850]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7b0);
            // 98a16 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98a1b lea rsi,[rip+0xa02a6] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 98a22 lea rbx,[rbp-0x1868]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e798);
            // 98a29 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 98a2b mov rdi,rbx
            rdi = rbx;
            // 98a2e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98a33 mov esi,0x176
            rsi = (0x176u64) as u32 as u64;
            // 98a38 mov rdi,r15
            rdi = r15;
            // 98a3b mov rdx,rbx
            rdx = rbx;
            // 98a3e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98a43 lea rdi,[rbp-0x1868]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e798);
            // 98a4a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98a4f mov DWORD PTR [rbp-0x1878],0x176
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e788), (0x176u64) as u32);
            // 98a59 mov QWORD PTR [rbp-0x1870],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e790), (r15) as u64);
            // 98a60 lea rsi,[rbp-0x1878]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e788);
            // 98a67 mov rdi,r14
            rdi = r14;
            // 98a6a call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98a6f lea rsi,[rip+0x9cb35] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 98a76 lea rdi,[rbp-0x1890]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e770);
            // 98a7d mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 98a82 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98a87 movzx ebx,BYTE PTR [rbp-0x1890]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e770)) as u64) as u32 as u64;
            // 98a8e test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 98a91 jne 0x98a98
            if crate::cov::brk(0x98a91, fl.ne()) { pc = 0x98a98; continue; }
            pc = 0x98a93;
        }
        0x98a93 => {
            // block 98a93
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98a93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98a93 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 98a96 jmp 0x98a9f
            pc = 0x98a9f; continue;
        }
        0x98a98 => {
            // block 98a98
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98a98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98a98 mov rbx,QWORD PTR [rbp-0x1888]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e778)) as u64;
            pc = 0x98a9f;
        }
        0x98a9f => {
            // block 98a9f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98a9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98a9f lea rdi,[rbp-0x1890]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e770);
            // 98aa6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98aab test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 98aae je 0x98b40
            if crate::cov::brk(0x98aae, fl.e()) { pc = 0x98b40; continue; }
            pc = 0x98ab4;
        }
        0x98ab4 => {
            // block 98ab4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98ab4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98ab4 lea rsi,[rip+0x9caf0] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 98abb lea rdi,[rbp-0x18d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e730);
            // 98ac2 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 98ac7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98acc mov rax,QWORD PTR [rbp-0x18c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e740)) as u64;
            // 98ad3 mov QWORD PTR [rbp-0x18a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e760), (rax) as u64);
            // 98ada mov rax,QWORD PTR [rbp-0x18d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e730)) as u64;
            // 98ae1 mov rcx,QWORD PTR [rbp-0x18c8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e738)) as u64;
            // 98ae8 mov QWORD PTR [rbp-0x18a8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e758), (rcx) as u64);
            // 98aef mov QWORD PTR [rbp-0x18b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e750), (rax) as u64);
            // 98af6 xorps xmm0,xmm0
            x0 = 0;
            // 98af9 movaps XMMWORD PTR [rbp-0x18d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e730), x0);
            // 98b00 mov QWORD PTR [rbp-0x18c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e740), (0x0u64) as u64);
            // 98b0b mov DWORD PTR [rbp-0x1898],0x176
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e768), (0x176u64) as u32);
            // 98b15 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 98b1c lea rsi,[rbp-0x18b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e750);
            // 98b23 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98b28 lea rdi,[rbp-0x18b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e750);
            // 98b2f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98b34 lea rdi,[rbp-0x18d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e730);
            // 98b3b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x98b40;
        }
        0x98b40 => {
            // block 98b40
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98b40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98b40 lea rsi,[rip+0x9aec4] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 98b47 lea rdi,[rbp-0x18e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e718);
            // 98b4e mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 98b53 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98b58 movzx ebx,BYTE PTR [rbp-0x18e8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e718)) as u64) as u32 as u64;
            // 98b5f test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 98b62 jne 0x98b69
            if crate::cov::brk(0x98b62, fl.ne()) { pc = 0x98b69; continue; }
            pc = 0x98b64;
        }
        0x98b64 => {
            // block 98b64
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98b64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98b64 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 98b67 jmp 0x98b70
            pc = 0x98b70; continue;
        }
        0x98b69 => {
            // block 98b69
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98b69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98b69 mov rbx,QWORD PTR [rbp-0x18e0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e720)) as u64;
            pc = 0x98b70;
        }
        0x98b70 => {
            // block 98b70
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98b70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98b70 lea rdi,[rbp-0x18e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e718);
            // 98b77 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98b7c test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 98b7f je 0x98c11
            if crate::cov::brk(0x98b7f, fl.e()) { pc = 0x98c11; continue; }
            pc = 0x98b85;
        }
        0x98b85 => {
            // block 98b85
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98b85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98b85 lea rsi,[rip+0x9ae7f] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 98b8c lea rdi,[rbp-0x1920]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6e0);
            // 98b93 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 98b98 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98b9d mov rax,QWORD PTR [rbp-0x1910]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e6f0)) as u64;
            // 98ba4 mov QWORD PTR [rbp-0x18f8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e708), (rax) as u64);
            // 98bab mov rax,QWORD PTR [rbp-0x1920]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e6e0)) as u64;
            // 98bb2 mov rcx,QWORD PTR [rbp-0x1918]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e6e8)) as u64;
            // 98bb9 mov QWORD PTR [rbp-0x1900],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e700), (rcx) as u64);
            // 98bc0 mov QWORD PTR [rbp-0x1908],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e6f8), (rax) as u64);
            // 98bc7 xorps xmm0,xmm0
            x0 = 0;
            // 98bca movaps XMMWORD PTR [rbp-0x1920],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e6e0), x0);
            // 98bd1 mov QWORD PTR [rbp-0x1910],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e6f0), (0x0u64) as u64);
            // 98bdc mov DWORD PTR [rbp-0x18f0],0x176
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e710), (0x176u64) as u32);
            // 98be6 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 98bed lea rsi,[rbp-0x1908]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e6f8);
            // 98bf4 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98bf9 lea rdi,[rbp-0x1908]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6f8);
            // 98c00 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98c05 lea rdi,[rbp-0x1920]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6e0);
            // 98c0c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x98c11;
        }
        0x98c11 => {
            // block 98c11
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98c11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98c11 xorps xmm0,xmm0
            x0 = 0;
            // 98c14 movaps XMMWORD PTR [rbp-0x1940],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e6c0), x0);
            // 98c1b mov QWORD PTR [rbp-0x1930],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e6d0), (0x0u64) as u64);
            // 98c26 lea rdi,[rbp-0x1940]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6c0);
            // 98c2d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98c32 lea rsi,[rip+0xa008f] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 98c39 lea rbx,[rbp-0x1958]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e6a8);
            // 98c40 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 98c42 mov rdi,rbx
            rdi = rbx;
            // 98c45 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98c4a mov esi,0x177
            rsi = (0x177u64) as u32 as u64;
            // 98c4f mov rdi,r15
            rdi = r15;
            // 98c52 mov rdx,rbx
            rdx = rbx;
            // 98c55 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98c5a lea rdi,[rbp-0x1958]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6a8);
            // 98c61 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98c66 mov DWORD PTR [rbp-0x1968],0x177
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e698), (0x177u64) as u32);
            // 98c70 mov QWORD PTR [rbp-0x1960],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e6a0), (r15) as u64);
            // 98c77 lea rsi,[rbp-0x1968]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e698);
            // 98c7e mov rdi,r14
            rdi = r14;
            // 98c81 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98c86 lea rsi,[rip+0x9c91e] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 98c8d lea rdi,[rbp-0x1980]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e680);
            // 98c94 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 98c99 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98c9e movzx ebx,BYTE PTR [rbp-0x1980]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e680)) as u64) as u32 as u64;
            // 98ca5 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 98ca8 jne 0x98caf
            if crate::cov::brk(0x98ca8, fl.ne()) { pc = 0x98caf; continue; }
            pc = 0x98caa;
        }
        0x98caa => {
            // block 98caa
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98caa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98caa shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 98cad jmp 0x98cb6
            pc = 0x98cb6; continue;
        }
        0x98caf => {
            // block 98caf
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98caf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98caf mov rbx,QWORD PTR [rbp-0x1978]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e688)) as u64;
            pc = 0x98cb6;
        }
        0x98cb6 => {
            // block 98cb6
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98cb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98cb6 lea rdi,[rbp-0x1980]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e680);
            // 98cbd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98cc2 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 98cc5 je 0x98d57
            if crate::cov::brk(0x98cc5, fl.e()) { pc = 0x98d57; continue; }
            pc = 0x98ccb;
        }
        0x98ccb => {
            // block 98ccb
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98ccb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98ccb lea rsi,[rip+0x9c8d9] # 0x1355ab
            rsi = m.env.data_va(302456);
            // 98cd2 lea rdi,[rbp-0x19c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e640);
            // 98cd9 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 98cde call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98ce3 mov rax,QWORD PTR [rbp-0x19b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e650)) as u64;
            // 98cea mov QWORD PTR [rbp-0x1990],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e670), (rax) as u64);
            // 98cf1 mov rax,QWORD PTR [rbp-0x19c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 98cf8 mov rcx,QWORD PTR [rbp-0x19b8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e648)) as u64;
            // 98cff mov QWORD PTR [rbp-0x1998],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e668), (rcx) as u64);
            // 98d06 mov QWORD PTR [rbp-0x19a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e660), (rax) as u64);
            // 98d0d xorps xmm0,xmm0
            x0 = 0;
            // 98d10 movaps XMMWORD PTR [rbp-0x19c0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e640), x0);
            // 98d17 mov QWORD PTR [rbp-0x19b0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e650), (0x0u64) as u64);
            // 98d22 mov DWORD PTR [rbp-0x1988],0x177
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e678), (0x177u64) as u32);
            // 98d2c lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 98d33 lea rsi,[rbp-0x19a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e660);
            // 98d3a call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98d3f lea rdi,[rbp-0x19a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e660);
            // 98d46 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98d4b lea rdi,[rbp-0x19c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e640);
            // 98d52 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x98d57;
        }
        0x98d57 => {
            // block 98d57
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98d57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98d57 lea rsi,[rip+0x9acad] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 98d5e lea rdi,[rbp-0x19d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e628);
            // 98d65 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 98d6a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98d6f movzx ebx,BYTE PTR [rbp-0x19d8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64) as u32 as u64;
            // 98d76 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 98d79 jne 0x98d80
            if crate::cov::brk(0x98d79, fl.ne()) { pc = 0x98d80; continue; }
            pc = 0x98d7b;
        }
        0x98d7b => {
            // block 98d7b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98d7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98d7b shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 98d7e jmp 0x98d87
            pc = 0x98d87; continue;
        }
        0x98d80 => {
            // block 98d80
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98d80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98d80 mov rbx,QWORD PTR [rbp-0x19d0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            pc = 0x98d87;
        }
        0x98d87 => {
            // block 98d87
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98d87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98d87 lea rdi,[rbp-0x19d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e628);
            // 98d8e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98d93 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 98d96 je 0x98e2b
            if crate::cov::brk(0x98d96, fl.e()) { pc = 0x98e2b; continue; }
            pc = 0x98d9c;
        }
        0x98d9c => {
            // block 98d9c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98d9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98d9c lea rsi,[rip+0x9ac68] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 98da3 lea rdi,[rbp-0x1a10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e5f0);
            // 98daa mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 98daf call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98db4 mov rax,QWORD PTR [rbp-0x1a00]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e600)) as u64;
            // 98dbb mov QWORD PTR [rbp-0x19e8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e618), (rax) as u64);
            // 98dc2 mov rax,QWORD PTR [rbp-0x1a10]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e5f0)) as u64;
            // 98dc9 mov rcx,QWORD PTR [rbp-0x1a08]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e5f8)) as u64;
            // 98dd0 mov QWORD PTR [rbp-0x19f0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e610), (rcx) as u64);
            // 98dd7 mov QWORD PTR [rbp-0x19f8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e608), (rax) as u64);
            // 98dde xorps xmm0,xmm0
            x0 = 0;
            // 98de1 movaps XMMWORD PTR [rbp-0x1a10],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e5f0), x0);
            // 98de8 mov QWORD PTR [rbp-0x1a00],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e600), (0x0u64) as u64);
            // 98df3 mov DWORD PTR [rbp-0x19e0],0x177
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e620), (0x177u64) as u32);
            // 98dfd add r15,0x198
            { let (r, f) = add::<64>(r15, 0x198u64);
              r15 = r;
              fl = f; }
            // 98e04 lea rsi,[rbp-0x19f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e608);
            // 98e0b mov rdi,r15
            rdi = r15;
            // 98e0e call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98e13 lea rdi,[rbp-0x19f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e608);
            // 98e1a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98e1f lea rdi,[rbp-0x1a10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e5f0);
            // 98e26 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x98e2b;
        }
        0x98e2b => {
            // block 98e2b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98e2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98e2b xorps xmm0,xmm0
            x0 = 0;
            // 98e2e movaps XMMWORD PTR [rbp-0x1a30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e5d0), x0);
            // 98e35 mov QWORD PTR [rbp-0x1a20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e5e0), (0x0u64) as u64);
            // 98e40 lea rdi,[rbp-0x1a30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e5d0);
            // 98e47 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98e4c add rsp,0x1a10
            { let (r, f) = add::<64>(rsp, 0x1a10u64);
              rsp = r;
              fl = f; }
            // 98e53 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 98e54 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 98e56 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 98e58 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 98e5a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 98e5b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x98e5c => {
            // block 98e5c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98e5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98e5c mov rbx,rax
            rbx = rax;
            // 98e5f lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 98e63 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98e68 => {
            // block 98e68
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98e68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98e68 mov rbx,rax
            rbx = rax;
            // 98e6b lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 98e72 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98e77 => {
            // block 98e77
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98e77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98e77 mov rbx,rax
            rbx = rax;
            // 98e7a lea rdi,[rbp-0x278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd88);
            // 98e81 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98e86 => {
            // block 98e86
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98e86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98e86 mov rbx,rax
            rbx = rax;
            // 98e89 lea rdi,[rbp-0x398]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc68);
            // 98e90 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98e95 => {
            // block 98e95
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98e95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98e95 mov rbx,rax
            rbx = rax;
            // 98e98 lea rdi,[rbp-0x4b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb48);
            // 98e9f jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98ea4 => {
            // block 98ea4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98ea4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98ea4 mov rbx,rax
            rbx = rax;
            // 98ea7 lea rdi,[rbp-0x5d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa28);
            // 98eae jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98eb3 => {
            // block 98eb3
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98eb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98eb3 mov rbx,rax
            rbx = rax;
            // 98eb6 lea rdi,[rbp-0x6f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f908);
            // 98ebd jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98ec2 => {
            // block 98ec2
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98ec2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98ec2 mov rbx,rax
            rbx = rax;
            // 98ec5 lea rdi,[rbp-0x818]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7e8);
            // 98ecc jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98ed1 => {
            // block 98ed1
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98ed1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98ed1 mov rbx,rax
            rbx = rax;
            // 98ed4 lea rdi,[rbp-0x938]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6c8);
            // 98edb jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98ee0 => {
            // block 98ee0
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98ee0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98ee0 mov rbx,rax
            rbx = rax;
            // 98ee3 lea rdi,[rbp-0xa58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5a8);
            // 98eea jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98eef => {
            // block 98eef
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98eef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98eef mov rbx,rax
            rbx = rax;
            // 98ef2 lea rdi,[rbp-0xb48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4b8);
            // 98ef9 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98efe => {
            // block 98efe
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98efe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98efe mov rbx,rax
            rbx = rax;
            // 98f01 lea rdi,[rbp-0xc38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3c8);
            // 98f08 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98f0d => {
            // block 98f0d
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98f0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98f0d mov rbx,rax
            rbx = rax;
            // 98f10 lea rdi,[rbp-0xd28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2d8);
            // 98f17 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98f1c => {
            // block 98f1c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98f1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98f1c mov rbx,rax
            rbx = rax;
            // 98f1f lea rdi,[rbp-0xe18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1e8);
            // 98f26 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98f2b => {
            // block 98f2b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98f2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98f2b mov rbx,rax
            rbx = rax;
            // 98f2e lea rdi,[rbp-0xf08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0f8);
            // 98f35 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98f3a => {
            // block 98f3a
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98f3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98f3a mov rbx,rax
            rbx = rax;
            // 98f3d lea rdi,[rbp-0xff8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f008);
            // 98f44 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98f49 => {
            // block 98f49
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98f49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98f49 mov rbx,rax
            rbx = rax;
            // 98f4c lea rdi,[rbp-0x10e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef18);
            // 98f53 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98f58 => {
            // block 98f58
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98f58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98f58 mov rbx,rax
            rbx = rax;
            // 98f5b lea rdi,[rbp-0x11d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee28);
            // 98f62 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98f67 => {
            // block 98f67
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98f67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98f67 mov rbx,rax
            rbx = rax;
            // 98f6a lea rdi,[rbp-0x12c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed38);
            // 98f71 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98f76 => {
            // block 98f76
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98f76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98f76 mov rbx,rax
            rbx = rax;
            // 98f79 lea rdi,[rbp-0x13b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec48);
            // 98f80 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98f85 => {
            // block 98f85
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98f85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98f85 mov rbx,rax
            rbx = rax;
            // 98f88 lea rdi,[rbp-0x14a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb58);
            // 98f8f jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98f94 => {
            // block 98f94
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98f94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98f94 mov rbx,rax
            rbx = rax;
            // 98f97 lea rdi,[rbp-0x1598]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea68);
            // 98f9e jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98fa3 => {
            // block 98fa3
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98fa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98fa3 mov rbx,rax
            rbx = rax;
            // 98fa6 lea rdi,[rbp-0x1688]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e978);
            // 98fad jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98fb2 => {
            // block 98fb2
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98fb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98fb2 mov rbx,rax
            rbx = rax;
            // 98fb5 lea rdi,[rbp-0x1778]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e888);
            // 98fbc jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98fc1 => {
            // block 98fc1
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98fc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98fc1 mov rbx,rax
            rbx = rax;
            // 98fc4 lea rdi,[rbp-0x1868]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e798);
            // 98fcb jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98fd0 => {
            // block 98fd0
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98fd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98fd0 mov rbx,rax
            rbx = rax;
            // 98fd3 lea rdi,[rbp-0x1958]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6a8);
            // 98fda jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98fdf => {
            // block 98fdf
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98fdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98fdf mov rbx,rax
            rbx = rax;
            // 98fe2 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 98fe6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 98feb lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 98ff2 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x98ff7 => {
            // block 98ff7
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 98ff7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98ff7 mov rbx,rax
            rbx = rax;
            // 98ffa lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 99001 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99006 lea rdi,[rbp-0xf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 9900d jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99012 => {
            // block 99012
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99012 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99012 mov rbx,rax
            rbx = rax;
            // 99015 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 9901c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99021 lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 99028 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9902d => {
            // block 9902d
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9902d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9902d mov rbx,rax
            rbx = rax;
            // 99030 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 99037 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9903c lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 99043 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99048 => {
            // block 99048
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99048 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99048 mov rbx,rax
            rbx = rax;
            // 9904b lea rdi,[rbp-0x1f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe08);
            // 99052 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99057 lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 9905e jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99063 => {
            // block 99063
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99063 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99063 mov rbx,rax
            rbx = rax;
            // 99066 lea rdi,[rbp-0x248]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdb8);
            // 9906d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99072 lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 99079 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9907e => {
            // block 9907e
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9907e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9907e mov rbx,rax
            rbx = rax;
            // 99081 lea rdi,[rbp-0x2c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 99088 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9908d lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 99094 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99099 => {
            // block 99099
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99099 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99099 mov rbx,rax
            rbx = rax;
            // 9909c lea rdi,[rbp-0x318]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fce8);
            // 990a3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 990a8 lea rdi,[rbp-0x330]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd0);
            // 990af jmp 0x99637
            pc = 0x99637; continue;
        }
        0x990b4 => {
            // block 990b4
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 990b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 990b4 mov rbx,rax
            rbx = rax;
            // 990b7 lea rdi,[rbp-0x368]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc98);
            // 990be call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 990c3 lea rdi,[rbp-0x380]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc80);
            // 990ca jmp 0x99637
            pc = 0x99637; continue;
        }
        0x990cf => {
            // block 990cf
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 990cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 990cf mov rbx,rax
            rbx = rax;
            // 990d2 lea rdi,[rbp-0x3e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc20);
            // 990d9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 990de lea rdi,[rbp-0x400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 990e5 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x990ea => {
            // block 990ea
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 990ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 990ea mov rbx,rax
            rbx = rax;
            // 990ed lea rdi,[rbp-0x438]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbc8);
            // 990f4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 990f9 lea rdi,[rbp-0x450]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb0);
            // 99100 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99105 => {
            // block 99105
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99105 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99105 mov rbx,rax
            rbx = rax;
            // 99108 lea rdi,[rbp-0x488]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb78);
            // 9910f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99114 lea rdi,[rbp-0x4a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb60);
            // 9911b jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99120 => {
            // block 99120
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99120 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99120 mov rbx,rax
            rbx = rax;
            // 99123 lea rdi,[rbp-0x500]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb00);
            // 9912a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9912f lea rdi,[rbp-0x520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fae0);
            // 99136 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9913b => {
            // block 9913b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9913b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9913b mov rbx,rax
            rbx = rax;
            // 9913e lea rdi,[rbp-0x558]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_faa8);
            // 99145 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9914a lea rdi,[rbp-0x570]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa90);
            // 99151 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99156 => {
            // block 99156
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99156 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99156 mov rbx,rax
            rbx = rax;
            // 99159 lea rdi,[rbp-0x5a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa58);
            // 99160 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99165 lea rdi,[rbp-0x5c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa40);
            // 9916c jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99171 => {
            // block 99171
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99171 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99171 mov rbx,rax
            rbx = rax;
            // 99174 lea rdi,[rbp-0x620]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9e0);
            // 9917b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99180 lea rdi,[rbp-0x640]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9c0);
            // 99187 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9918c => {
            // block 9918c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9918c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9918c mov rbx,rax
            rbx = rax;
            // 9918f lea rdi,[rbp-0x678]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f988);
            // 99196 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9919b lea rdi,[rbp-0x690]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f970);
            // 991a2 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x991a7 => {
            // block 991a7
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 991a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 991a7 mov rbx,rax
            rbx = rax;
            // 991aa lea rdi,[rbp-0x6c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f938);
            // 991b1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 991b6 lea rdi,[rbp-0x6e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f920);
            // 991bd jmp 0x99637
            pc = 0x99637; continue;
        }
        0x991c2 => {
            // block 991c2
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 991c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 991c2 mov rbx,rax
            rbx = rax;
            // 991c5 lea rdi,[rbp-0x740]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8c0);
            // 991cc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 991d1 lea rdi,[rbp-0x760]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8a0);
            // 991d8 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x991dd => {
            // block 991dd
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 991dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 991dd mov rbx,rax
            rbx = rax;
            // 991e0 lea rdi,[rbp-0x798]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f868);
            // 991e7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 991ec lea rdi,[rbp-0x7b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f850);
            // 991f3 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x991f8 => {
            // block 991f8
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 991f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 991f8 mov rbx,rax
            rbx = rax;
            // 991fb lea rdi,[rbp-0x7e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f818);
            // 99202 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99207 lea rdi,[rbp-0x800]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f800);
            // 9920e jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99213 => {
            // block 99213
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99213 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99213 mov rbx,rax
            rbx = rax;
            // 99216 lea rdi,[rbp-0x860]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7a0);
            // 9921d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99222 lea rdi,[rbp-0x880]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f780);
            // 99229 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9922e => {
            // block 9922e
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9922e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9922e mov rbx,rax
            rbx = rax;
            // 99231 lea rdi,[rbp-0x8b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f748);
            // 99238 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9923d lea rdi,[rbp-0x8d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f730);
            // 99244 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99249 => {
            // block 99249
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99249 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99249 mov rbx,rax
            rbx = rax;
            // 9924c lea rdi,[rbp-0x908]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6f8);
            // 99253 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99258 lea rdi,[rbp-0x920]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6e0);
            // 9925f jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99264 => {
            // block 99264
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99264 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99264 mov rbx,rax
            rbx = rax;
            // 99267 lea rdi,[rbp-0x980]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f680);
            // 9926e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99273 lea rdi,[rbp-0x9a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f660);
            // 9927a jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9927f => {
            // block 9927f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9927f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9927f mov rbx,rax
            rbx = rax;
            // 99282 lea rdi,[rbp-0x9d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f628);
            // 99289 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9928e lea rdi,[rbp-0x9f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f610);
            // 99295 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9929a => {
            // block 9929a
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9929a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9929a mov rbx,rax
            rbx = rax;
            // 9929d lea rdi,[rbp-0xa28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5d8);
            // 992a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 992a9 lea rdi,[rbp-0xa40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5c0);
            // 992b0 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x992b5 => {
            // block 992b5
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 992b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 992b5 mov rbx,rax
            rbx = rax;
            // 992b8 lea rdi,[rbp-0xaa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f560);
            // 992bf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 992c4 lea rdi,[rbp-0xac0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f540);
            // 992cb jmp 0x99637
            pc = 0x99637; continue;
        }
        0x992d0 => {
            // block 992d0
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 992d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 992d0 mov rbx,rax
            rbx = rax;
            // 992d3 lea rdi,[rbp-0xaf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f508);
            // 992da call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 992df lea rdi,[rbp-0xb10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4f0);
            // 992e6 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x992eb => {
            // block 992eb
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 992eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 992eb mov rbx,rax
            rbx = rax;
            // 992ee lea rdi,[rbp-0xb90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f470);
            // 992f5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 992fa lea rdi,[rbp-0xbb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f450);
            // 99301 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99306 => {
            // block 99306
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99306 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99306 mov rbx,rax
            rbx = rax;
            // 99309 lea rdi,[rbp-0xbe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f418);
            // 99310 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99315 lea rdi,[rbp-0xc00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f400);
            // 9931c jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99321 => {
            // block 99321
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99321 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99321 mov rbx,rax
            rbx = rax;
            // 99324 lea rdi,[rbp-0xc80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f380);
            // 9932b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99330 lea rdi,[rbp-0xca0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f360);
            // 99337 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9933c => {
            // block 9933c
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9933c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9933c mov rbx,rax
            rbx = rax;
            // 9933f lea rdi,[rbp-0xcd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f328);
            // 99346 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9934b lea rdi,[rbp-0xcf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f310);
            // 99352 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99357 => {
            // block 99357
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99357 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99357 mov rbx,rax
            rbx = rax;
            // 9935a lea rdi,[rbp-0xd70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f290);
            // 99361 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99366 lea rdi,[rbp-0xd90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f270);
            // 9936d jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99372 => {
            // block 99372
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99372 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99372 mov rbx,rax
            rbx = rax;
            // 99375 lea rdi,[rbp-0xdc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f238);
            // 9937c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99381 lea rdi,[rbp-0xde0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f220);
            // 99388 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9938d => {
            // block 9938d
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9938d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9938d mov rbx,rax
            rbx = rax;
            // 99390 lea rdi,[rbp-0xe60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1a0);
            // 99397 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9939c lea rdi,[rbp-0xe80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f180);
            // 993a3 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x993a8 => {
            // block 993a8
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 993a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 993a8 mov rbx,rax
            rbx = rax;
            // 993ab lea rdi,[rbp-0xeb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f148);
            // 993b2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 993b7 lea rdi,[rbp-0xed0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f130);
            // 993be jmp 0x99637
            pc = 0x99637; continue;
        }
        0x993c3 => {
            // block 993c3
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 993c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 993c3 mov rbx,rax
            rbx = rax;
            // 993c6 lea rdi,[rbp-0xf50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0b0);
            // 993cd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 993d2 lea rdi,[rbp-0xf70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f090);
            // 993d9 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x993de => {
            // block 993de
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 993de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 993de mov rbx,rax
            rbx = rax;
            // 993e1 lea rdi,[rbp-0xfa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f058);
            // 993e8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 993ed lea rdi,[rbp-0xfc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f040);
            // 993f4 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x993f9 => {
            // block 993f9
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 993f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 993f9 mov rbx,rax
            rbx = rax;
            // 993fc lea rdi,[rbp-0x1040]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efc0);
            // 99403 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99408 lea rdi,[rbp-0x1060]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efa0);
            // 9940f jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99414 => {
            // block 99414
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99414 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99414 mov rbx,rax
            rbx = rax;
            // 99417 lea rdi,[rbp-0x1098]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef68);
            // 9941e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99423 lea rdi,[rbp-0x10b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef50);
            // 9942a jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9942f => {
            // block 9942f
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9942f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9942f mov rbx,rax
            rbx = rax;
            // 99432 lea rdi,[rbp-0x1130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eed0);
            // 99439 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9943e lea rdi,[rbp-0x1150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eeb0);
            // 99445 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9944a => {
            // block 9944a
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9944a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9944a mov rbx,rax
            rbx = rax;
            // 9944d lea rdi,[rbp-0x1188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee78);
            // 99454 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99459 lea rdi,[rbp-0x11a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee60);
            // 99460 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99465 => {
            // block 99465
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99465 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99465 mov rbx,rax
            rbx = rax;
            // 99468 lea rdi,[rbp-0x1220]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ede0);
            // 9946f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99474 lea rdi,[rbp-0x1240]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edc0);
            // 9947b jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99480 => {
            // block 99480
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99480 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99480 mov rbx,rax
            rbx = rax;
            // 99483 lea rdi,[rbp-0x1278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed88);
            // 9948a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9948f lea rdi,[rbp-0x1290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed70);
            // 99496 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9949b => {
            // block 9949b
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9949b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9949b mov rbx,rax
            rbx = rax;
            // 9949e lea rdi,[rbp-0x1310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecf0);
            // 994a5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 994aa lea rdi,[rbp-0x1330]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecd0);
            // 994b1 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x994b6 => {
            // block 994b6
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 994b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 994b6 mov rbx,rax
            rbx = rax;
            // 994b9 lea rdi,[rbp-0x1368]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec98);
            // 994c0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 994c5 lea rdi,[rbp-0x1380]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec80);
            // 994cc jmp 0x99637
            pc = 0x99637; continue;
        }
        0x994d1 => {
            // block 994d1
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 994d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 994d1 mov rbx,rax
            rbx = rax;
            // 994d4 lea rdi,[rbp-0x1400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec00);
            // 994db call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 994e0 lea rdi,[rbp-0x1420]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebe0);
            // 994e7 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x994ec => {
            // block 994ec
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 994ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 994ec mov rbx,rax
            rbx = rax;
            // 994ef lea rdi,[rbp-0x1458]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eba8);
            // 994f6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 994fb lea rdi,[rbp-0x1470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb90);
            // 99502 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99507 => {
            // block 99507
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99507 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99507 mov rbx,rax
            rbx = rax;
            // 9950a lea rdi,[rbp-0x14f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb10);
            // 99511 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99516 lea rdi,[rbp-0x1510]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eaf0);
            // 9951d jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99522 => {
            // block 99522
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99522 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99522 mov rbx,rax
            rbx = rax;
            // 99525 lea rdi,[rbp-0x1548]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eab8);
            // 9952c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99531 lea rdi,[rbp-0x1560]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eaa0);
            // 99538 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9953d => {
            // block 9953d
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9953d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9953d mov rbx,rax
            rbx = rax;
            // 99540 lea rdi,[rbp-0x15e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea20);
            // 99547 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9954c lea rdi,[rbp-0x1600]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea00);
            // 99553 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99558 => {
            // block 99558
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99558 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99558 mov rbx,rax
            rbx = rax;
            // 9955b lea rdi,[rbp-0x1638]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9c8);
            // 99562 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99567 lea rdi,[rbp-0x1650]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9b0);
            // 9956e jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99573 => {
            // block 99573
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99573 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99573 mov rbx,rax
            rbx = rax;
            // 99576 lea rdi,[rbp-0x16d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e930);
            // 9957d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99582 lea rdi,[rbp-0x16f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e910);
            // 99589 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x9958e => {
            // block 9958e
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 9958e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9958e mov rbx,rax
            rbx = rax;
            // 99591 lea rdi,[rbp-0x1728]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8d8);
            // 99598 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9959d lea rdi,[rbp-0x1740]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8c0);
            // 995a4 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x995a9 => {
            // block 995a9
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 995a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 995a9 mov rbx,rax
            rbx = rax;
            // 995ac lea rdi,[rbp-0x17c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e840);
            // 995b3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 995b8 lea rdi,[rbp-0x17e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e820);
            // 995bf jmp 0x99637
            pc = 0x99637; continue;
        }
        0x995c1 => {
            // block 995c1
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 995c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 995c1 mov rbx,rax
            rbx = rax;
            // 995c4 lea rdi,[rbp-0x1818]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7e8);
            // 995cb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 995d0 lea rdi,[rbp-0x1830]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7d0);
            // 995d7 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x995d9 => {
            // block 995d9
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 995d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 995d9 mov rbx,rax
            rbx = rax;
            // 995dc lea rdi,[rbp-0x18b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e750);
            // 995e3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 995e8 lea rdi,[rbp-0x18d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e730);
            // 995ef jmp 0x99637
            pc = 0x99637; continue;
        }
        0x995f1 => {
            // block 995f1
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 995f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 995f1 mov rbx,rax
            rbx = rax;
            // 995f4 lea rdi,[rbp-0x1908]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6f8);
            // 995fb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99600 lea rdi,[rbp-0x1920]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6e0);
            // 99607 jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99609 => {
            // block 99609
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99609 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99609 mov rbx,rax
            rbx = rax;
            // 9960c lea rdi,[rbp-0x19a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e660);
            // 99613 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99618 lea rdi,[rbp-0x19c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e640);
            // 9961f jmp 0x99637
            pc = 0x99637; continue;
        }
        0x99621 => {
            // block 99621
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99621 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99621 mov rbx,rax
            rbx = rax;
            // 99624 lea rdi,[rbp-0x19f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e608);
            // 9962b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 99630 lea rdi,[rbp-0x1a10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e5f0);
            pc = 0x99637;
        }
        0x99637 => {
            // block 99637
            #[cfg(feature = "trace")] eprintln!("f_000951bc pc 99637 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99637 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9963c mov rdi,rbx
            rdi = rbx;
            // 9963f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
