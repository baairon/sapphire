//! Mechanically lifted functions (scripts/lift_run.py), module hb002. Each one is checked against the original
//! in the world harness (tests/world_hb002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x794b2 size=19474
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000794b2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x794b2;
    loop {
        match pc {
        0x794b2 => {
            // block 794b2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 794b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 794b2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 794b3 mov rbp,rsp
            rbp = rsp;
            // 794b6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 794b8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 794ba push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 794bc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 794be push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 794bf sub rsp,0x19b8
            { let (r, f) = sub::<64>(rsp, 0x19b8u64);
              rsp = r;
              fl = f; }
            // 794c6 mov r14,rdi
            r14 = rdi;
            // 794c9 lea rsi,[rip+0xbf7f8] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 794d0 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 794d4 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 794d6 mov rdi,rbx
            rdi = rbx;
            // 794d9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 794de mov esi,0x88
            rsi = (0x88u64) as u32 as u64;
            // 794e3 mov rdi,r14
            rdi = r14;
            // 794e6 mov rdx,rbx
            rdx = rbx;
            // 794e9 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 794ee lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 794f2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 794f7 lea rax,[r14+0xf8]
            rax = r14.wrapping_add(0xf8);
            // 794fe mov QWORD PTR [rbp-0x19d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e628), (rax) as u64);
            // 79505 mov DWORD PTR [rbp-0x50],0x88
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x88u64) as u32);
            // 7950c mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 79510 lea rdi,[r14+0x180]
            rdi = r14.wrapping_add(0x180);
            // 79517 mov QWORD PTR [rbp-0x19e0],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e620), (rdi) as u64);
            // 7951e lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 79522 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79527 lea rsi,[rip+0xb0a75] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7952e lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 79532 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 79537 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7953c movzx ebx,BYTE PTR [rbp-0x68]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // 79540 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 79543 jne 0x7954a
            if fl.ne() { pc = 0x7954a; continue; }
            pc = 0x79545;
        }
        0x79545 => {
            // block 79545
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79545 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79545 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 79548 jmp 0x7954e
            pc = 0x7954e; continue;
        }
        0x7954a => {
            // block 7954a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7954a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7954a mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            pc = 0x7954e;
        }
        0x7954e => {
            // block 7954e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7954e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7954e lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 79552 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79557 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7955a je 0x795e3
            if fl.e() { pc = 0x795e3; continue; }
            pc = 0x79560;
        }
        0x79560 => {
            // block 79560
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79560 lea rsi,[rip+0xb0a3c] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 79567 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 7956e mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 79573 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79578 mov rax,QWORD PTR [rbp-0x90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 7957f mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 79583 mov rax,QWORD PTR [rbp-0xa0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 7958a mov rcx,QWORD PTR [rbp-0x98]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 79591 mov QWORD PTR [rbp-0x80],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rcx) as u64);
            // 79595 mov QWORD PTR [rbp-0x88],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rax) as u64);
            // 7959c xorps xmm0,xmm0
            x0 = 0;
            // 7959f movaps XMMWORD PTR [rbp-0xa0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff60), x0);
            // 795a6 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 795b1 mov DWORD PTR [rbp-0x70],0x88
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x88u64) as u32);
            // 795b8 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 795bf lea rsi,[rbp-0x88]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 795c6 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 795cb lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 795d2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 795d7 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 795de call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x795e3;
        }
        0x795e3 => {
            // block 795e3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 795e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 795e3 xorps xmm0,xmm0
            x0 = 0;
            // 795e6 movaps XMMWORD PTR [rbp-0xc0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff40), x0);
            // 795ed mov QWORD PTR [rbp-0xb0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x0u64) as u64);
            // 795f8 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 795ff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79604 xorps xmm0,xmm0
            x0 = 0;
            // 79607 movaps XMMWORD PTR [rbp-0xe0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff20), x0);
            // 7960e mov QWORD PTR [rbp-0xd0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (0x0u64) as u64);
            // 79619 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 79620 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79625 lea rax,[r14+0x1f8]
            rax = r14.wrapping_add(0x1f8);
            // 7962c mov QWORD PTR [rbp-0x19c0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e640), (rax) as u64);
            // 79633 lea r15,[r14+0x200]
            r15 = r14.wrapping_add(0x200);
            // 7963a lea r13,[r14+0x1e0]
            r13 = r14.wrapping_add(0x1e0);
            // 79641 lea r12,[r14+0x1e8]
            r12 = r14.wrapping_add(0x1e8);
            // 79648 mov QWORD PTR [rbp-0x19c8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e638), (r12) as u64);
            // 7964f mov QWORD PTR [rbp-0x19d0],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e630), (r14) as u64);
            // 79656 lea r14,[rip+0x890d3] # 0x102730
            r14 = m.env.data_va(93949);
            // 7965d xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7965f;
        }
        0x7965f => {
            // block 7965f
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7965f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7965f mov DWORD PTR [rbp-0xe8],0x88
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x88u64) as u32);
            // 79669 mov DWORD PTR [rbp-0xe4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c), ((rbx as u32 as u64)) as u32);
            // 7966f mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 79676 lea rsi,[rbp-0xe8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 7967d call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79682 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 79685 jne 0x796b3
            if fl.ne() { pc = 0x796b3; continue; }
            pc = 0x79687;
        }
        0x79687 => {
            // block 79687
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79687 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79687 mov r12,r13
            r12 = r13;
            // 7968a mov r13,r15
            r13 = r15;
            // 7968d mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 79690 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 79697 lea rsi,[rbp-0xe8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 7969e call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 796a3 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 796a6 mov r15,r13
            r15 = r13;
            // 796a9 mov r13,r12
            r13 = r12;
            // 796ac mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x796b3;
        }
        0x796b3 => {
            // block 796b3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 796b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 796b3 mov rdi,r13
            rdi = r13;
            // 796b6 mov rsi,r14
            rsi = r14;
            // 796b9 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 796be cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 796c1 jne 0x796d7
            if fl.ne() { pc = 0x796d7; continue; }
            pc = 0x796c3;
        }
        0x796c3 => {
            // block 796c3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 796c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 796c3 mov rdi,r13
            rdi = r13;
            // 796c6 mov rsi,r14
            rsi = r14;
            // 796c9 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 796ce mov DWORD PTR [rax],0x88
            m.w32(rax, (0x88u64) as u32);
            // 796d4 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x796d7;
        }
        0x796d7 => {
            // block 796d7
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 796d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 796d7 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 796da add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 796de cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 796e2 jne 0x7965f
            if fl.ne() { pc = 0x7965f; continue; }
            pc = 0x796e8;
        }
        0x796e8 => {
            // block 796e8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 796e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 796e8 lea rsi,[rip+0xbf5d9] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 796ef lea rbx,[rbp-0x100]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 796f6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 796f8 mov rdi,rbx
            rdi = rbx;
            // 796fb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79700 mov esi,0x89
            rsi = (0x89u64) as u32 as u64;
            // 79705 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7970c mov rdi,r14
            rdi = r14;
            // 7970f mov rdx,rbx
            rdx = rbx;
            // 79712 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79717 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 7971e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79723 mov DWORD PTR [rbp-0x110],0x89
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x89u64) as u32);
            // 7972d mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 79734 mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 7973b lea rsi,[rbp-0x110]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 79742 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 79749 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7974e lea rsi,[rip+0xb084e] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 79755 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 7975c mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 79761 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79766 movzx ebx,BYTE PTR [rbp-0x128]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64) as u32 as u64;
            // 7976d test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 79770 jne 0x79777
            if fl.ne() { pc = 0x79777; continue; }
            pc = 0x79772;
        }
        0x79772 => {
            // block 79772
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79772 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79772 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 79775 jmp 0x7977e
            pc = 0x7977e; continue;
        }
        0x79777 => {
            // block 79777
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79777 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79777 mov rbx,QWORD PTR [rbp-0x120]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            pc = 0x7977e;
        }
        0x7977e => {
            // block 7977e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7977e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7977e lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 79785 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7978a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7978d je 0x7981f
            if fl.e() { pc = 0x7981f; continue; }
            pc = 0x79793;
        }
        0x79793 => {
            // block 79793
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79793 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79793 lea rsi,[rip+0xb0809] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7979a lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 797a1 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 797a6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 797ab mov rax,QWORD PTR [rbp-0x150]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 797b2 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // 797b9 mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 797c0 mov rcx,QWORD PTR [rbp-0x158]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 797c7 mov QWORD PTR [rbp-0x140],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rcx) as u64);
            // 797ce mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 797d5 xorps xmm0,xmm0
            x0 = 0;
            // 797d8 movaps XMMWORD PTR [rbp-0x160],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fea0), x0);
            // 797df mov QWORD PTR [rbp-0x150],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (0x0u64) as u64);
            // 797ea mov DWORD PTR [rbp-0x130],0x89
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (0x89u64) as u32);
            // 797f4 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 797fb lea rsi,[rbp-0x148]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 79802 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79807 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 7980e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79813 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 7981a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7981f;
        }
        0x7981f => {
            // block 7981f
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7981f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7981f xorps xmm0,xmm0
            x0 = 0;
            // 79822 movaps XMMWORD PTR [rbp-0x180],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe80), x0);
            // 79829 mov QWORD PTR [rbp-0x170],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (0x0u64) as u64);
            // 79834 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 7983b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79840 xorps xmm0,xmm0
            x0 = 0;
            // 79843 movaps XMMWORD PTR [rbp-0x1a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe60), x0);
            // 7984a mov QWORD PTR [rbp-0x190],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (0x0u64) as u64);
            // 79855 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 7985c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79861 lea r14,[rip+0x88ed8] # 0x102740
            r14 = m.env.data_va(93965);
            // 79868 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7986a;
        }
        0x7986a => {
            // block 7986a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7986a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7986a mov DWORD PTR [rbp-0x1a8],0x89
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (0x89u64) as u32);
            // 79874 mov DWORD PTR [rbp-0x1a4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe5c), ((rbx as u32 as u64)) as u32);
            // 7987a mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 79881 lea rsi,[rbp-0x1a8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 79888 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7988d cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 79890 jne 0x798be
            if fl.ne() { pc = 0x798be; continue; }
            pc = 0x79892;
        }
        0x79892 => {
            // block 79892
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79892 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79892 mov r12,r13
            r12 = r13;
            // 79895 mov r13,r15
            r13 = r15;
            // 79898 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7989b mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 798a2 lea rsi,[rbp-0x1a8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 798a9 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 798ae mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 798b1 mov r15,r13
            r15 = r13;
            // 798b4 mov r13,r12
            r13 = r12;
            // 798b7 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x798be;
        }
        0x798be => {
            // block 798be
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 798be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 798be mov rdi,r13
            rdi = r13;
            // 798c1 mov rsi,r14
            rsi = r14;
            // 798c4 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 798c9 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 798cc jne 0x798e2
            if fl.ne() { pc = 0x798e2; continue; }
            pc = 0x798ce;
        }
        0x798ce => {
            // block 798ce
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 798ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 798ce mov rdi,r13
            rdi = r13;
            // 798d1 mov rsi,r14
            rsi = r14;
            // 798d4 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 798d9 mov DWORD PTR [rax],0x89
            m.w32(rax, (0x89u64) as u32);
            // 798df mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x798e2;
        }
        0x798e2 => {
            // block 798e2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 798e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 798e2 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 798e5 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 798e9 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 798ed jne 0x7986a
            if fl.ne() { pc = 0x7986a; continue; }
            pc = 0x798f3;
        }
        0x798f3 => {
            // block 798f3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 798f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 798f3 lea rsi,[rip+0xbf3ce] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 798fa lea rbx,[rbp-0x1c0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 79901 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 79903 mov rdi,rbx
            rdi = rbx;
            // 79906 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7990b mov esi,0x8a
            rsi = (0x8au64) as u32 as u64;
            // 79910 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 79917 mov rdi,r14
            rdi = r14;
            // 7991a mov rdx,rbx
            rdx = rbx;
            // 7991d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79922 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 79929 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7992e mov DWORD PTR [rbp-0x1d0],0x8a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe30), (0x8au64) as u32);
            // 79938 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7993f mov QWORD PTR [rbp-0x1c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe38), (rax) as u64);
            // 79946 lea rsi,[rbp-0x1d0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe30);
            // 7994d mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 79954 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79959 lea rsi,[rip+0xb0643] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 79960 lea rdi,[rbp-0x1e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe18);
            // 79967 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7996c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79971 movzx ebx,BYTE PTR [rbp-0x1e8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe18)) as u64) as u32 as u64;
            // 79978 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7997b jne 0x79982
            if fl.ne() { pc = 0x79982; continue; }
            pc = 0x7997d;
        }
        0x7997d => {
            // block 7997d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7997d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7997d shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 79980 jmp 0x79989
            pc = 0x79989; continue;
        }
        0x79982 => {
            // block 79982
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79982 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79982 mov rbx,QWORD PTR [rbp-0x1e0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            pc = 0x79989;
        }
        0x79989 => {
            // block 79989
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79989 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79989 lea rdi,[rbp-0x1e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe18);
            // 79990 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79995 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 79998 je 0x79a2a
            if fl.e() { pc = 0x79a2a; continue; }
            pc = 0x7999e;
        }
        0x7999e => {
            // block 7999e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7999e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7999e lea rsi,[rip+0xb05fe] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 799a5 lea rdi,[rbp-0x220]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde0);
            // 799ac mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 799b1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 799b6 mov rax,QWORD PTR [rbp-0x210]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdf0)) as u64;
            // 799bd mov QWORD PTR [rbp-0x1f8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe08), (rax) as u64);
            // 799c4 mov rax,QWORD PTR [rbp-0x220]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde0)) as u64;
            // 799cb mov rcx,QWORD PTR [rbp-0x218]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde8)) as u64;
            // 799d2 mov QWORD PTR [rbp-0x200],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe00), (rcx) as u64);
            // 799d9 mov QWORD PTR [rbp-0x208],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdf8), (rax) as u64);
            // 799e0 xorps xmm0,xmm0
            x0 = 0;
            // 799e3 movaps XMMWORD PTR [rbp-0x220],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fde0), x0);
            // 799ea mov QWORD PTR [rbp-0x210],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdf0), (0x0u64) as u64);
            // 799f5 mov DWORD PTR [rbp-0x1f0],0x8a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (0x8au64) as u32);
            // 799ff lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 79a06 lea rsi,[rbp-0x208]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fdf8);
            // 79a0d call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79a12 lea rdi,[rbp-0x208]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf8);
            // 79a19 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79a1e lea rdi,[rbp-0x220]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde0);
            // 79a25 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x79a2a;
        }
        0x79a2a => {
            // block 79a2a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79a2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79a2a xorps xmm0,xmm0
            x0 = 0;
            // 79a2d movaps XMMWORD PTR [rbp-0x240],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fdc0), x0);
            // 79a34 mov QWORD PTR [rbp-0x230],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdd0), (0x0u64) as u64);
            // 79a3f lea rdi,[rbp-0x240]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc0);
            // 79a46 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79a4b xorps xmm0,xmm0
            x0 = 0;
            // 79a4e movaps XMMWORD PTR [rbp-0x260],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fda0), x0);
            // 79a55 mov QWORD PTR [rbp-0x250],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb0), (0x0u64) as u64);
            // 79a60 lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 79a67 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79a6c lea r14,[rip+0x88cdd] # 0x102750
            r14 = m.env.data_va(93981);
            // 79a73 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x79a75;
        }
        0x79a75 => {
            // block 79a75
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79a75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79a75 mov DWORD PTR [rbp-0x268],0x8a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd98), (0x8au64) as u32);
            // 79a7f mov DWORD PTR [rbp-0x264],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd9c), ((rbx as u32 as u64)) as u32);
            // 79a85 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 79a8c lea rsi,[rbp-0x268]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd98);
            // 79a93 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79a98 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 79a9b jne 0x79ac9
            if fl.ne() { pc = 0x79ac9; continue; }
            pc = 0x79a9d;
        }
        0x79a9d => {
            // block 79a9d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79a9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79a9d mov r12,r13
            r12 = r13;
            // 79aa0 mov r13,r15
            r13 = r15;
            // 79aa3 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 79aa6 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 79aad lea rsi,[rbp-0x268]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd98);
            // 79ab4 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79ab9 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 79abc mov r15,r13
            r15 = r13;
            // 79abf mov r13,r12
            r13 = r12;
            // 79ac2 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x79ac9;
        }
        0x79ac9 => {
            // block 79ac9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79ac9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79ac9 mov rdi,r13
            rdi = r13;
            // 79acc mov rsi,r14
            rsi = r14;
            // 79acf call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79ad4 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 79ad7 jne 0x79aed
            if fl.ne() { pc = 0x79aed; continue; }
            pc = 0x79ad9;
        }
        0x79ad9 => {
            // block 79ad9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79ad9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79ad9 mov rdi,r13
            rdi = r13;
            // 79adc mov rsi,r14
            rsi = r14;
            // 79adf call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79ae4 mov DWORD PTR [rax],0x8a
            m.w32(rax, (0x8au64) as u32);
            // 79aea mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x79aed;
        }
        0x79aed => {
            // block 79aed
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79aed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79aed inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 79af0 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 79af4 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 79af8 jne 0x79a75
            if fl.ne() { pc = 0x79a75; continue; }
            pc = 0x79afe;
        }
        0x79afe => {
            // block 79afe
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79afe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79afe lea rsi,[rip+0xbf1c3] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 79b05 lea rbx,[rbp-0x280]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd80);
            // 79b0c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 79b0e mov rdi,rbx
            rdi = rbx;
            // 79b11 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79b16 mov esi,0x8b
            rsi = (0x8bu64) as u32 as u64;
            // 79b1b mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 79b22 mov rdi,r14
            rdi = r14;
            // 79b25 mov rdx,rbx
            rdx = rbx;
            // 79b28 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79b2d lea rdi,[rbp-0x280]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd80);
            // 79b34 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79b39 mov DWORD PTR [rbp-0x290],0x8b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd70), (0x8bu64) as u32);
            // 79b43 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 79b4a mov QWORD PTR [rbp-0x288],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd78), (rax) as u64);
            // 79b51 lea rsi,[rbp-0x290]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // 79b58 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 79b5f call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79b64 lea rsi,[rip+0xb0438] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 79b6b lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 79b72 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 79b77 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79b7c movzx ebx,BYTE PTR [rbp-0x2a8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd58)) as u64) as u32 as u64;
            // 79b83 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 79b86 jne 0x79b8d
            if fl.ne() { pc = 0x79b8d; continue; }
            pc = 0x79b88;
        }
        0x79b88 => {
            // block 79b88
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79b88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79b88 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 79b8b jmp 0x79b94
            pc = 0x79b94; continue;
        }
        0x79b8d => {
            // block 79b8d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79b8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79b8d mov rbx,QWORD PTR [rbp-0x2a0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd60)) as u64;
            pc = 0x79b94;
        }
        0x79b94 => {
            // block 79b94
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79b94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79b94 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 79b9b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79ba0 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 79ba3 je 0x79c35
            if fl.e() { pc = 0x79c35; continue; }
            pc = 0x79ba9;
        }
        0x79ba9 => {
            // block 79ba9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79ba9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79ba9 lea rsi,[rip+0xb03f3] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 79bb0 lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 79bb7 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 79bbc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79bc1 mov rax,QWORD PTR [rbp-0x2d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd30)) as u64;
            // 79bc8 mov QWORD PTR [rbp-0x2b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd48), (rax) as u64);
            // 79bcf mov rax,QWORD PTR [rbp-0x2e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd20)) as u64;
            // 79bd6 mov rcx,QWORD PTR [rbp-0x2d8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd28)) as u64;
            // 79bdd mov QWORD PTR [rbp-0x2c0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd40), (rcx) as u64);
            // 79be4 mov QWORD PTR [rbp-0x2c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd38), (rax) as u64);
            // 79beb xorps xmm0,xmm0
            x0 = 0;
            // 79bee movaps XMMWORD PTR [rbp-0x2e0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fd20), x0);
            // 79bf5 mov QWORD PTR [rbp-0x2d0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd30), (0x0u64) as u64);
            // 79c00 mov DWORD PTR [rbp-0x2b0],0x8b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd50), (0x8bu64) as u32);
            // 79c0a lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 79c11 lea rsi,[rbp-0x2c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 79c18 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79c1d lea rdi,[rbp-0x2c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 79c24 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79c29 lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 79c30 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x79c35;
        }
        0x79c35 => {
            // block 79c35
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79c35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79c35 xorps xmm0,xmm0
            x0 = 0;
            // 79c38 movaps XMMWORD PTR [rbp-0x300],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fd00), x0);
            // 79c3f mov QWORD PTR [rbp-0x2f0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd10), (0x0u64) as u64);
            // 79c4a lea rdi,[rbp-0x300]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd00);
            // 79c51 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79c56 xorps xmm0,xmm0
            x0 = 0;
            // 79c59 movaps XMMWORD PTR [rbp-0x320],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fce0), x0);
            // 79c60 mov QWORD PTR [rbp-0x310],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0), (0x0u64) as u64);
            // 79c6b lea rdi,[rbp-0x320]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fce0);
            // 79c72 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79c77 lea r14,[rip+0x88ae2] # 0x102760
            r14 = m.env.data_va(93997);
            // 79c7e xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x79c80;
        }
        0x79c80 => {
            // block 79c80
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79c80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79c80 mov DWORD PTR [rbp-0x328],0x8b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fcd8), (0x8bu64) as u32);
            // 79c8a mov DWORD PTR [rbp-0x324],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fcdc), ((rbx as u32 as u64)) as u32);
            // 79c90 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 79c97 lea rsi,[rbp-0x328]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 79c9e call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79ca3 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 79ca6 jne 0x79cd4
            if fl.ne() { pc = 0x79cd4; continue; }
            pc = 0x79ca8;
        }
        0x79ca8 => {
            // block 79ca8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79ca8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79ca8 mov r12,r13
            r12 = r13;
            // 79cab mov r13,r15
            r13 = r15;
            // 79cae mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 79cb1 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 79cb8 lea rsi,[rbp-0x328]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 79cbf call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79cc4 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 79cc7 mov r15,r13
            r15 = r13;
            // 79cca mov r13,r12
            r13 = r12;
            // 79ccd mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x79cd4;
        }
        0x79cd4 => {
            // block 79cd4
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79cd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79cd4 mov rdi,r13
            rdi = r13;
            // 79cd7 mov rsi,r14
            rsi = r14;
            // 79cda call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79cdf cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 79ce2 jne 0x79cf8
            if fl.ne() { pc = 0x79cf8; continue; }
            pc = 0x79ce4;
        }
        0x79ce4 => {
            // block 79ce4
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79ce4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79ce4 mov rdi,r13
            rdi = r13;
            // 79ce7 mov rsi,r14
            rsi = r14;
            // 79cea call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79cef mov DWORD PTR [rax],0x8b
            m.w32(rax, (0x8bu64) as u32);
            // 79cf5 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x79cf8;
        }
        0x79cf8 => {
            // block 79cf8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79cf8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79cf8 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 79cfb add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 79cff cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 79d03 jne 0x79c80
            if fl.ne() { pc = 0x79c80; continue; }
            pc = 0x79d09;
        }
        0x79d09 => {
            // block 79d09
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79d09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79d09 lea rsi,[rip+0xbefb8] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 79d10 lea rbx,[rbp-0x340]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 79d17 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 79d19 mov rdi,rbx
            rdi = rbx;
            // 79d1c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79d21 mov esi,0x8c
            rsi = (0x8cu64) as u32 as u64;
            // 79d26 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 79d2d mov rdi,r14
            rdi = r14;
            // 79d30 mov rdx,rbx
            rdx = rbx;
            // 79d33 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79d38 lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 79d3f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79d44 mov DWORD PTR [rbp-0x350],0x8c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fcb0), (0x8cu64) as u32);
            // 79d4e mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 79d55 mov QWORD PTR [rbp-0x348],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcb8), (rax) as u64);
            // 79d5c lea rsi,[rbp-0x350]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fcb0);
            // 79d63 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 79d6a call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79d6f lea rsi,[rip+0xb022d] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 79d76 lea rdi,[rbp-0x368]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc98);
            // 79d7d mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 79d82 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79d87 movzx ebx,BYTE PTR [rbp-0x368]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fc98)) as u64) as u32 as u64;
            // 79d8e test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 79d91 jne 0x79d98
            if fl.ne() { pc = 0x79d98; continue; }
            pc = 0x79d93;
        }
        0x79d93 => {
            // block 79d93
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79d93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79d93 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 79d96 jmp 0x79d9f
            pc = 0x79d9f; continue;
        }
        0x79d98 => {
            // block 79d98
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79d98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79d98 mov rbx,QWORD PTR [rbp-0x360]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fca0)) as u64;
            pc = 0x79d9f;
        }
        0x79d9f => {
            // block 79d9f
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79d9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79d9f lea rdi,[rbp-0x368]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc98);
            // 79da6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79dab test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 79dae je 0x79e40
            if fl.e() { pc = 0x79e40; continue; }
            pc = 0x79db4;
        }
        0x79db4 => {
            // block 79db4
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79db4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79db4 lea rsi,[rip+0xb01e8] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 79dbb lea rdi,[rbp-0x3a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc60);
            // 79dc2 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 79dc7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79dcc mov rax,QWORD PTR [rbp-0x390]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc70)) as u64;
            // 79dd3 mov QWORD PTR [rbp-0x378],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc88), (rax) as u64);
            // 79dda mov rax,QWORD PTR [rbp-0x3a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc60)) as u64;
            // 79de1 mov rcx,QWORD PTR [rbp-0x398]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc68)) as u64;
            // 79de8 mov QWORD PTR [rbp-0x380],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc80), (rcx) as u64);
            // 79def mov QWORD PTR [rbp-0x388],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc78), (rax) as u64);
            // 79df6 xorps xmm0,xmm0
            x0 = 0;
            // 79df9 movaps XMMWORD PTR [rbp-0x3a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fc60), x0);
            // 79e00 mov QWORD PTR [rbp-0x390],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc70), (0x0u64) as u64);
            // 79e0b mov DWORD PTR [rbp-0x370],0x8c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc90), (0x8cu64) as u32);
            // 79e15 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 79e1c lea rsi,[rbp-0x388]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fc78);
            // 79e23 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79e28 lea rdi,[rbp-0x388]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc78);
            // 79e2f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79e34 lea rdi,[rbp-0x3a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc60);
            // 79e3b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x79e40;
        }
        0x79e40 => {
            // block 79e40
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79e40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79e40 xorps xmm0,xmm0
            x0 = 0;
            // 79e43 movaps XMMWORD PTR [rbp-0x3c0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fc40), x0);
            // 79e4a mov QWORD PTR [rbp-0x3b0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc50), (0x0u64) as u64);
            // 79e55 lea rdi,[rbp-0x3c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc40);
            // 79e5c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79e61 xorps xmm0,xmm0
            x0 = 0;
            // 79e64 movaps XMMWORD PTR [rbp-0x3e0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fc20), x0);
            // 79e6b mov QWORD PTR [rbp-0x3d0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc30), (0x0u64) as u64);
            // 79e76 lea rdi,[rbp-0x3e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc20);
            // 79e7d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79e82 lea r14,[rip+0x888e7] # 0x102770
            r14 = m.env.data_va(94013);
            // 79e89 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x79e8b;
        }
        0x79e8b => {
            // block 79e8b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79e8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79e8b mov DWORD PTR [rbp-0x3e8],0x8c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc18), (0x8cu64) as u32);
            // 79e95 mov DWORD PTR [rbp-0x3e4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc1c), ((rbx as u32 as u64)) as u32);
            // 79e9b mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 79ea2 lea rsi,[rbp-0x3e8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // 79ea9 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79eae cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 79eb1 jne 0x79edf
            if fl.ne() { pc = 0x79edf; continue; }
            pc = 0x79eb3;
        }
        0x79eb3 => {
            // block 79eb3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79eb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79eb3 mov r12,r13
            r12 = r13;
            // 79eb6 mov r13,r15
            r13 = r15;
            // 79eb9 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 79ebc mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 79ec3 lea rsi,[rbp-0x3e8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // 79eca call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79ecf mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 79ed2 mov r15,r13
            r15 = r13;
            // 79ed5 mov r13,r12
            r13 = r12;
            // 79ed8 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x79edf;
        }
        0x79edf => {
            // block 79edf
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79edf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79edf mov rdi,r13
            rdi = r13;
            // 79ee2 mov rsi,r14
            rsi = r14;
            // 79ee5 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79eea cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 79eed jne 0x79f03
            if fl.ne() { pc = 0x79f03; continue; }
            pc = 0x79eef;
        }
        0x79eef => {
            // block 79eef
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79eef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79eef mov rdi,r13
            rdi = r13;
            // 79ef2 mov rsi,r14
            rsi = r14;
            // 79ef5 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79efa mov DWORD PTR [rax],0x8c
            m.w32(rax, (0x8cu64) as u32);
            // 79f00 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x79f03;
        }
        0x79f03 => {
            // block 79f03
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79f03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79f03 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 79f06 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 79f0a cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 79f0e jne 0x79e8b
            if fl.ne() { pc = 0x79e8b; continue; }
            pc = 0x79f14;
        }
        0x79f14 => {
            // block 79f14
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79f14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79f14 lea rsi,[rip+0xbedad] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 79f1b lea rbx,[rbp-0x400]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 79f22 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 79f24 mov rdi,rbx
            rdi = rbx;
            // 79f27 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79f2c mov esi,0x8d
            rsi = (0x8du64) as u32 as u64;
            // 79f31 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 79f38 mov rdi,r14
            rdi = r14;
            // 79f3b mov rdx,rbx
            rdx = rbx;
            // 79f3e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79f43 lea rdi,[rbp-0x400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 79f4a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79f4f mov DWORD PTR [rbp-0x410],0x8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fbf0), (0x8du64) as u32);
            // 79f59 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 79f60 mov QWORD PTR [rbp-0x408],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbf8), (rax) as u64);
            // 79f67 lea rsi,[rbp-0x410]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fbf0);
            // 79f6e mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 79f75 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79f7a lea rsi,[rip+0xb0022] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 79f81 lea rdi,[rbp-0x428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd8);
            // 79f88 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 79f8d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79f92 movzx ebx,BYTE PTR [rbp-0x428]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fbd8)) as u64) as u32 as u64;
            // 79f99 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 79f9c jne 0x79fa3
            if fl.ne() { pc = 0x79fa3; continue; }
            pc = 0x79f9e;
        }
        0x79f9e => {
            // block 79f9e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79f9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79f9e shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 79fa1 jmp 0x79faa
            pc = 0x79faa; continue;
        }
        0x79fa3 => {
            // block 79fa3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79fa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79fa3 mov rbx,QWORD PTR [rbp-0x420]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbe0)) as u64;
            pc = 0x79faa;
        }
        0x79faa => {
            // block 79faa
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79faa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79faa lea rdi,[rbp-0x428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd8);
            // 79fb1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79fb6 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 79fb9 je 0x7a04b
            if fl.e() { pc = 0x7a04b; continue; }
            pc = 0x79fbf;
        }
        0x79fbf => {
            // block 79fbf
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 79fbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 79fbf lea rsi,[rip+0xaffdd] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 79fc6 lea rdi,[rbp-0x460]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fba0);
            // 79fcd mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 79fd2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 79fd7 mov rax,QWORD PTR [rbp-0x450]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbb0)) as u64;
            // 79fde mov QWORD PTR [rbp-0x438],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc8), (rax) as u64);
            // 79fe5 mov rax,QWORD PTR [rbp-0x460]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fba0)) as u64;
            // 79fec mov rcx,QWORD PTR [rbp-0x458]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fba8)) as u64;
            // 79ff3 mov QWORD PTR [rbp-0x440],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc0), (rcx) as u64);
            // 79ffa mov QWORD PTR [rbp-0x448],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbb8), (rax) as u64);
            // 7a001 xorps xmm0,xmm0
            x0 = 0;
            // 7a004 movaps XMMWORD PTR [rbp-0x460],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fba0), x0);
            // 7a00b mov QWORD PTR [rbp-0x450],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbb0), (0x0u64) as u64);
            // 7a016 mov DWORD PTR [rbp-0x430],0x8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fbd0), (0x8du64) as u32);
            // 7a020 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7a027 lea rsi,[rbp-0x448]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fbb8);
            // 7a02e call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a033 lea rdi,[rbp-0x448]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb8);
            // 7a03a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a03f lea rdi,[rbp-0x460]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fba0);
            // 7a046 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7a04b;
        }
        0x7a04b => {
            // block 7a04b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a04b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a04b xorps xmm0,xmm0
            x0 = 0;
            // 7a04e movaps XMMWORD PTR [rbp-0x480],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fb80), x0);
            // 7a055 mov QWORD PTR [rbp-0x470],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb90), (0x0u64) as u64);
            // 7a060 lea rdi,[rbp-0x480]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb80);
            // 7a067 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a06c xorps xmm0,xmm0
            x0 = 0;
            // 7a06f movaps XMMWORD PTR [rbp-0x4a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fb60), x0);
            // 7a076 mov QWORD PTR [rbp-0x490],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb70), (0x0u64) as u64);
            // 7a081 lea rdi,[rbp-0x4a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb60);
            // 7a088 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a08d lea r14,[rip+0x886ec] # 0x102780
            r14 = m.env.data_va(94029);
            // 7a094 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7a096;
        }
        0x7a096 => {
            // block 7a096
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a096 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a096 mov DWORD PTR [rbp-0x4a8],0x8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb58), (0x8du64) as u32);
            // 7a0a0 mov DWORD PTR [rbp-0x4a4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb5c), ((rbx as u32 as u64)) as u32);
            // 7a0a6 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7a0ad lea rsi,[rbp-0x4a8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb58);
            // 7a0b4 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a0b9 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7a0bc jne 0x7a0ea
            if fl.ne() { pc = 0x7a0ea; continue; }
            pc = 0x7a0be;
        }
        0x7a0be => {
            // block 7a0be
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a0be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a0be mov r12,r13
            r12 = r13;
            // 7a0c1 mov r13,r15
            r13 = r15;
            // 7a0c4 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7a0c7 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7a0ce lea rsi,[rbp-0x4a8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb58);
            // 7a0d5 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a0da mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7a0dd mov r15,r13
            r15 = r13;
            // 7a0e0 mov r13,r12
            r13 = r12;
            // 7a0e3 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7a0ea;
        }
        0x7a0ea => {
            // block 7a0ea
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a0ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a0ea mov rdi,r13
            rdi = r13;
            // 7a0ed mov rsi,r14
            rsi = r14;
            // 7a0f0 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a0f5 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7a0f8 jne 0x7a10e
            if fl.ne() { pc = 0x7a10e; continue; }
            pc = 0x7a0fa;
        }
        0x7a0fa => {
            // block 7a0fa
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a0fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a0fa mov rdi,r13
            rdi = r13;
            // 7a0fd mov rsi,r14
            rsi = r14;
            // 7a100 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a105 mov DWORD PTR [rax],0x8d
            m.w32(rax, (0x8du64) as u32);
            // 7a10b mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7a10e;
        }
        0x7a10e => {
            // block 7a10e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a10e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a10e inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7a111 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7a115 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7a119 jne 0x7a096
            if fl.ne() { pc = 0x7a096; continue; }
            pc = 0x7a11f;
        }
        0x7a11f => {
            // block 7a11f
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a11f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a11f movabs rbx,0x20000008d
            rbx = 0x0_0002_0000_008du64;
            // 7a129 mov QWORD PTR [rbp-0x4b0],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb50), (rbx) as u64);
            // 7a130 lea rsi,[rbp-0x4b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // 7a137 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7a13e call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a143 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7a146 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7a14d jne 0x7a168
            if fl.ne() { pc = 0x7a168; continue; }
            pc = 0x7a14f;
        }
        0x7a14f => {
            // block 7a14f
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a14f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a14f lea rsi,[rbp-0x4b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // 7a156 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7a15d call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a162 mov DWORD PTR [rax],0x66
            m.w32(rax, (0x66u64) as u32);
            pc = 0x7a168;
        }
        0x7a168 => {
            // block 7a168
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a168 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a168 mov DWORD PTR [rbp-0x4b4],0x66
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb4c), (0x66u64) as u32);
            // 7a172 lea rsi,[rbp-0x4b4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb4c);
            // 7a179 mov rdi,r13
            rdi = r13;
            // 7a17c call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a181 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7a184 jne 0x7a1a2
            if fl.ne() { pc = 0x7a1a2; continue; }
            pc = 0x7a186;
        }
        0x7a186 => {
            // block 7a186
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a186 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a186 mov DWORD PTR [rbp-0x4b8],0x66
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb48), (0x66u64) as u32);
            // 7a190 lea rsi,[rbp-0x4b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb48);
            // 7a197 mov rdi,r13
            rdi = r13;
            // 7a19a call 0xebbfa
            { let r = { let t = m.env.stub_va(0x3c7); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a19f mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            pc = 0x7a1a2;
        }
        0x7a1a2 => {
            // block 7a1a2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a1a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a1a2 lea rsi,[rip+0xbeb1f] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7a1a9 lea rbx,[rbp-0x4d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb30);
            // 7a1b0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7a1b2 mov rdi,rbx
            rdi = rbx;
            // 7a1b5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a1ba mov esi,0xa3
            rsi = (0xa3u64) as u32 as u64;
            // 7a1bf mov rdi,r14
            rdi = r14;
            // 7a1c2 mov rdx,rbx
            rdx = rbx;
            // 7a1c5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a1ca lea rdi,[rbp-0x4d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb30);
            // 7a1d1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a1d6 mov DWORD PTR [rbp-0x4e0],0xa3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb20), (0xa3u64) as u32);
            // 7a1e0 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7a1e7 mov QWORD PTR [rbp-0x4d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb28), (rax) as u64);
            // 7a1ee lea rsi,[rbp-0x4e0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb20);
            // 7a1f5 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7a1fc call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a201 lea rsi,[rip+0xafd9b] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7a208 lea rdi,[rbp-0x4f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb08);
            // 7a20f mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7a214 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a219 movzx ebx,BYTE PTR [rbp-0x4f8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fb08)) as u64) as u32 as u64;
            // 7a220 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7a223 jne 0x7a22a
            if fl.ne() { pc = 0x7a22a; continue; }
            pc = 0x7a225;
        }
        0x7a225 => {
            // block 7a225
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a225 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a225 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7a228 jmp 0x7a231
            pc = 0x7a231; continue;
        }
        0x7a22a => {
            // block 7a22a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a22a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a22a mov rbx,QWORD PTR [rbp-0x4f0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb10)) as u64;
            pc = 0x7a231;
        }
        0x7a231 => {
            // block 7a231
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a231 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a231 lea rdi,[rbp-0x4f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb08);
            // 7a238 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a23d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7a240 je 0x7a2d2
            if fl.e() { pc = 0x7a2d2; continue; }
            pc = 0x7a246;
        }
        0x7a246 => {
            // block 7a246
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a246 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a246 lea rsi,[rip+0xafd56] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7a24d lea rdi,[rbp-0x530]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fad0);
            // 7a254 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7a259 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a25e mov rax,QWORD PTR [rbp-0x520]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fae0)) as u64;
            // 7a265 mov QWORD PTR [rbp-0x508],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_faf8), (rax) as u64);
            // 7a26c mov rax,QWORD PTR [rbp-0x530]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fad0)) as u64;
            // 7a273 mov rcx,QWORD PTR [rbp-0x528]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fad8)) as u64;
            // 7a27a mov QWORD PTR [rbp-0x510],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_faf0), (rcx) as u64);
            // 7a281 mov QWORD PTR [rbp-0x518],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fae8), (rax) as u64);
            // 7a288 xorps xmm0,xmm0
            x0 = 0;
            // 7a28b movaps XMMWORD PTR [rbp-0x530],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fad0), x0);
            // 7a292 mov QWORD PTR [rbp-0x520],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fae0), (0x0u64) as u64);
            // 7a29d mov DWORD PTR [rbp-0x500],0xa3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb00), (0xa3u64) as u32);
            // 7a2a7 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7a2ae lea rsi,[rbp-0x518]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fae8);
            // 7a2b5 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a2ba lea rdi,[rbp-0x518]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fae8);
            // 7a2c1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a2c6 lea rdi,[rbp-0x530]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fad0);
            // 7a2cd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7a2d2;
        }
        0x7a2d2 => {
            // block 7a2d2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a2d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a2d2 xorps xmm0,xmm0
            x0 = 0;
            // 7a2d5 movaps XMMWORD PTR [rbp-0x550],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fab0), x0);
            // 7a2dc mov QWORD PTR [rbp-0x540],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fac0), (0x0u64) as u64);
            // 7a2e7 lea rdi,[rbp-0x550]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fab0);
            // 7a2ee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a2f3 xorps xmm0,xmm0
            x0 = 0;
            // 7a2f6 movaps XMMWORD PTR [rbp-0x570],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fa90), x0);
            // 7a2fd mov QWORD PTR [rbp-0x560],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_faa0), (0x0u64) as u64);
            // 7a308 lea rdi,[rbp-0x570]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa90);
            // 7a30f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a314 lea r14,[rip+0x88475] # 0x102790
            r14 = m.env.data_va(94045);
            // 7a31b xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7a31d;
        }
        0x7a31d => {
            // block 7a31d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a31d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a31d cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 7a320 je 0x7a39a
            if fl.e() { pc = 0x7a39a; continue; }
            pc = 0x7a322;
        }
        0x7a322 => {
            // block 7a322
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a322 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a322 mov DWORD PTR [rbp-0x578],0xa3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa88), (0xa3u64) as u32);
            // 7a32c mov DWORD PTR [rbp-0x574],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa8c), ((rbx as u32 as u64)) as u32);
            // 7a332 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7a339 lea rsi,[rbp-0x578]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa88);
            // 7a340 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a345 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7a348 jne 0x7a376
            if fl.ne() { pc = 0x7a376; continue; }
            pc = 0x7a34a;
        }
        0x7a34a => {
            // block 7a34a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a34a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a34a mov r12,r13
            r12 = r13;
            // 7a34d mov r13,r15
            r13 = r15;
            // 7a350 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7a353 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7a35a lea rsi,[rbp-0x578]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa88);
            // 7a361 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a366 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7a369 mov r15,r13
            r15 = r13;
            // 7a36c mov r13,r12
            r13 = r12;
            // 7a36f mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7a376;
        }
        0x7a376 => {
            // block 7a376
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a376 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a376 mov rdi,r13
            rdi = r13;
            // 7a379 mov rsi,r14
            rsi = r14;
            // 7a37c call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a381 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7a384 jne 0x7a39a
            if fl.ne() { pc = 0x7a39a; continue; }
            pc = 0x7a386;
        }
        0x7a386 => {
            // block 7a386
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a386 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a386 mov rdi,r13
            rdi = r13;
            // 7a389 mov rsi,r14
            rsi = r14;
            // 7a38c call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a391 mov DWORD PTR [rax],0xa3
            m.w32(rax, (0xa3u64) as u32);
            // 7a397 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7a39a;
        }
        0x7a39a => {
            // block 7a39a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a39a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a39a inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7a39d add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7a3a1 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7a3a5 jne 0x7a31d
            if fl.ne() { pc = 0x7a31d; continue; }
            pc = 0x7a3ab;
        }
        0x7a3ab => {
            // block 7a3ab
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a3ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a3ab lea rsi,[rip+0xbe916] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7a3b2 lea rbx,[rbp-0x590]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fa70);
            // 7a3b9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7a3bb mov rdi,rbx
            rdi = rbx;
            // 7a3be call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a3c3 mov esi,0xa4
            rsi = (0xa4u64) as u32 as u64;
            // 7a3c8 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7a3cf mov rdi,r14
            rdi = r14;
            // 7a3d2 mov rdx,rbx
            rdx = rbx;
            // 7a3d5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a3da lea rdi,[rbp-0x590]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa70);
            // 7a3e1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a3e6 mov DWORD PTR [rbp-0x5a0],0xa4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa60), (0xa4u64) as u32);
            // 7a3f0 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7a3f7 mov QWORD PTR [rbp-0x598],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa68), (rax) as u64);
            // 7a3fe lea rsi,[rbp-0x5a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa60);
            // 7a405 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7a40c call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a411 lea rsi,[rip+0xafb8b] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7a418 lea rdi,[rbp-0x5b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa48);
            // 7a41f mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7a424 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a429 movzx ebx,BYTE PTR [rbp-0x5b8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fa48)) as u64) as u32 as u64;
            // 7a430 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7a433 jne 0x7a43a
            if fl.ne() { pc = 0x7a43a; continue; }
            pc = 0x7a435;
        }
        0x7a435 => {
            // block 7a435
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a435 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a435 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7a438 jmp 0x7a441
            pc = 0x7a441; continue;
        }
        0x7a43a => {
            // block 7a43a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a43a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a43a mov rbx,QWORD PTR [rbp-0x5b0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa50)) as u64;
            pc = 0x7a441;
        }
        0x7a441 => {
            // block 7a441
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a441 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a441 lea rdi,[rbp-0x5b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa48);
            // 7a448 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a44d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7a450 je 0x7a4e2
            if fl.e() { pc = 0x7a4e2; continue; }
            pc = 0x7a456;
        }
        0x7a456 => {
            // block 7a456
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a456 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a456 lea rsi,[rip+0xafb46] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7a45d lea rdi,[rbp-0x5f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa10);
            // 7a464 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7a469 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a46e mov rax,QWORD PTR [rbp-0x5e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa20)) as u64;
            // 7a475 mov QWORD PTR [rbp-0x5c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa38), (rax) as u64);
            // 7a47c mov rax,QWORD PTR [rbp-0x5f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa10)) as u64;
            // 7a483 mov rcx,QWORD PTR [rbp-0x5e8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa18)) as u64;
            // 7a48a mov QWORD PTR [rbp-0x5d0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa30), (rcx) as u64);
            // 7a491 mov QWORD PTR [rbp-0x5d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa28), (rax) as u64);
            // 7a498 xorps xmm0,xmm0
            x0 = 0;
            // 7a49b movaps XMMWORD PTR [rbp-0x5f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fa10), x0);
            // 7a4a2 mov QWORD PTR [rbp-0x5e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa20), (0x0u64) as u64);
            // 7a4ad mov DWORD PTR [rbp-0x5c0],0xa4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa40), (0xa4u64) as u32);
            // 7a4b7 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7a4be lea rsi,[rbp-0x5d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa28);
            // 7a4c5 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a4ca lea rdi,[rbp-0x5d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa28);
            // 7a4d1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a4d6 lea rdi,[rbp-0x5f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa10);
            // 7a4dd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7a4e2;
        }
        0x7a4e2 => {
            // block 7a4e2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a4e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a4e2 xorps xmm0,xmm0
            x0 = 0;
            // 7a4e5 movaps XMMWORD PTR [rbp-0x610],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f9f0), x0);
            // 7a4ec mov QWORD PTR [rbp-0x600],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa00), (0x0u64) as u64);
            // 7a4f7 lea rdi,[rbp-0x610]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9f0);
            // 7a4fe call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a503 xorps xmm0,xmm0
            x0 = 0;
            // 7a506 movaps XMMWORD PTR [rbp-0x630],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f9d0), x0);
            // 7a50d mov QWORD PTR [rbp-0x620],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f9e0), (0x0u64) as u64);
            // 7a518 lea rdi,[rbp-0x630]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9d0);
            // 7a51f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a524 lea r14,[rip+0x88275] # 0x1027a0
            r14 = m.env.data_va(94061);
            // 7a52b xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7a52d;
        }
        0x7a52d => {
            // block 7a52d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a52d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a52d cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 7a530 je 0x7a5aa
            if fl.e() { pc = 0x7a5aa; continue; }
            pc = 0x7a532;
        }
        0x7a532 => {
            // block 7a532
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a532 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a532 mov DWORD PTR [rbp-0x638],0xa4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9c8), (0xa4u64) as u32);
            // 7a53c mov DWORD PTR [rbp-0x634],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9cc), ((rbx as u32 as u64)) as u32);
            // 7a542 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7a549 lea rsi,[rbp-0x638]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9c8);
            // 7a550 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a555 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7a558 jne 0x7a586
            if fl.ne() { pc = 0x7a586; continue; }
            pc = 0x7a55a;
        }
        0x7a55a => {
            // block 7a55a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a55a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a55a mov r12,r13
            r12 = r13;
            // 7a55d mov r13,r15
            r13 = r15;
            // 7a560 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7a563 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7a56a lea rsi,[rbp-0x638]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9c8);
            // 7a571 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a576 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7a579 mov r15,r13
            r15 = r13;
            // 7a57c mov r13,r12
            r13 = r12;
            // 7a57f mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7a586;
        }
        0x7a586 => {
            // block 7a586
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a586 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a586 mov rdi,r13
            rdi = r13;
            // 7a589 mov rsi,r14
            rsi = r14;
            // 7a58c call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a591 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7a594 jne 0x7a5aa
            if fl.ne() { pc = 0x7a5aa; continue; }
            pc = 0x7a596;
        }
        0x7a596 => {
            // block 7a596
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a596 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a596 mov rdi,r13
            rdi = r13;
            // 7a599 mov rsi,r14
            rsi = r14;
            // 7a59c call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a5a1 mov DWORD PTR [rax],0xa4
            m.w32(rax, (0xa4u64) as u32);
            // 7a5a7 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7a5aa;
        }
        0x7a5aa => {
            // block 7a5aa
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a5aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a5aa inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7a5ad add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7a5b1 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7a5b5 jne 0x7a52d
            if fl.ne() { pc = 0x7a52d; continue; }
            pc = 0x7a5bb;
        }
        0x7a5bb => {
            // block 7a5bb
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a5bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a5bb lea rsi,[rip+0xbe706] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7a5c2 lea rbx,[rbp-0x650]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f9b0);
            // 7a5c9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7a5cb mov rdi,rbx
            rdi = rbx;
            // 7a5ce call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a5d3 mov esi,0xfa
            rsi = (0xfau64) as u32 as u64;
            // 7a5d8 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7a5df mov rdi,r14
            rdi = r14;
            // 7a5e2 mov rdx,rbx
            rdx = rbx;
            // 7a5e5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a5ea lea rdi,[rbp-0x650]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9b0);
            // 7a5f1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a5f6 mov DWORD PTR [rbp-0x660],0xfa
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9a0), (0xfau64) as u32);
            // 7a600 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7a607 mov QWORD PTR [rbp-0x658],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f9a8), (rax) as u64);
            // 7a60e lea rsi,[rbp-0x660]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9a0);
            // 7a615 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7a61c call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a621 lea rsi,[rip+0xaf97b] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7a628 lea rdi,[rbp-0x678]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f988);
            // 7a62f mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7a634 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a639 movzx ebx,BYTE PTR [rbp-0x678]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f988)) as u64) as u32 as u64;
            // 7a640 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7a643 jne 0x7a64a
            if fl.ne() { pc = 0x7a64a; continue; }
            pc = 0x7a645;
        }
        0x7a645 => {
            // block 7a645
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a645 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a645 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7a648 jmp 0x7a651
            pc = 0x7a651; continue;
        }
        0x7a64a => {
            // block 7a64a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a64a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a64a mov rbx,QWORD PTR [rbp-0x670]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f990)) as u64;
            pc = 0x7a651;
        }
        0x7a651 => {
            // block 7a651
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a651 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a651 lea rdi,[rbp-0x678]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f988);
            // 7a658 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a65d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7a660 je 0x7a6f2
            if fl.e() { pc = 0x7a6f2; continue; }
            pc = 0x7a666;
        }
        0x7a666 => {
            // block 7a666
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a666 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a666 lea rsi,[rip+0xaf936] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7a66d lea rdi,[rbp-0x6b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f950);
            // 7a674 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7a679 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a67e mov rax,QWORD PTR [rbp-0x6a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f960)) as u64;
            // 7a685 mov QWORD PTR [rbp-0x688],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f978), (rax) as u64);
            // 7a68c mov rax,QWORD PTR [rbp-0x6b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f950)) as u64;
            // 7a693 mov rcx,QWORD PTR [rbp-0x6a8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f958)) as u64;
            // 7a69a mov QWORD PTR [rbp-0x690],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f970), (rcx) as u64);
            // 7a6a1 mov QWORD PTR [rbp-0x698],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f968), (rax) as u64);
            // 7a6a8 xorps xmm0,xmm0
            x0 = 0;
            // 7a6ab movaps XMMWORD PTR [rbp-0x6b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f950), x0);
            // 7a6b2 mov QWORD PTR [rbp-0x6a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f960), (0x0u64) as u64);
            // 7a6bd mov DWORD PTR [rbp-0x680],0xfa
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f980), (0xfau64) as u32);
            // 7a6c7 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7a6ce lea rsi,[rbp-0x698]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f968);
            // 7a6d5 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a6da lea rdi,[rbp-0x698]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f968);
            // 7a6e1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a6e6 lea rdi,[rbp-0x6b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f950);
            // 7a6ed call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7a6f2;
        }
        0x7a6f2 => {
            // block 7a6f2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a6f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a6f2 xorps xmm0,xmm0
            x0 = 0;
            // 7a6f5 movaps XMMWORD PTR [rbp-0x6d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f930), x0);
            // 7a6fc mov QWORD PTR [rbp-0x6c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f940), (0x0u64) as u64);
            // 7a707 lea rdi,[rbp-0x6d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f930);
            // 7a70e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a713 xorps xmm0,xmm0
            x0 = 0;
            // 7a716 movaps XMMWORD PTR [rbp-0x6f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f910), x0);
            // 7a71d mov QWORD PTR [rbp-0x6e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f920), (0x0u64) as u64);
            // 7a728 lea rdi,[rbp-0x6f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f910);
            // 7a72f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a734 lea r14,[rip+0x88075] # 0x1027b0
            r14 = m.env.data_va(94077);
            // 7a73b xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7a73d;
        }
        0x7a73d => {
            // block 7a73d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a73d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a73d mov DWORD PTR [rbp-0x6f8],0xfa
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f908), (0xfau64) as u32);
            // 7a747 mov DWORD PTR [rbp-0x6f4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f90c), ((rbx as u32 as u64)) as u32);
            // 7a74d mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7a754 lea rsi,[rbp-0x6f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f908);
            // 7a75b call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a760 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7a763 jne 0x7a791
            if fl.ne() { pc = 0x7a791; continue; }
            pc = 0x7a765;
        }
        0x7a765 => {
            // block 7a765
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a765 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a765 mov r12,r13
            r12 = r13;
            // 7a768 mov r13,r15
            r13 = r15;
            // 7a76b mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7a76e mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7a775 lea rsi,[rbp-0x6f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f908);
            // 7a77c call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a781 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7a784 mov r15,r13
            r15 = r13;
            // 7a787 mov r13,r12
            r13 = r12;
            // 7a78a mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7a791;
        }
        0x7a791 => {
            // block 7a791
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a791 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a791 mov rdi,r13
            rdi = r13;
            // 7a794 mov rsi,r14
            rsi = r14;
            // 7a797 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a79c cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7a79f jne 0x7a7b5
            if fl.ne() { pc = 0x7a7b5; continue; }
            pc = 0x7a7a1;
        }
        0x7a7a1 => {
            // block 7a7a1
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a7a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a7a1 mov rdi,r13
            rdi = r13;
            // 7a7a4 mov rsi,r14
            rsi = r14;
            // 7a7a7 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a7ac mov DWORD PTR [rax],0xfa
            m.w32(rax, (0xfau64) as u32);
            // 7a7b2 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7a7b5;
        }
        0x7a7b5 => {
            // block 7a7b5
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a7b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a7b5 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7a7b8 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7a7bc cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7a7c0 jne 0x7a73d
            if fl.ne() { pc = 0x7a73d; continue; }
            pc = 0x7a7c6;
        }
        0x7a7c6 => {
            // block 7a7c6
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a7c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a7c6 lea rsi,[rip+0xbe4fb] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7a7cd lea rbx,[rbp-0x710]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f8f0);
            // 7a7d4 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7a7d6 mov rdi,rbx
            rdi = rbx;
            // 7a7d9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a7de mov esi,0xfb
            rsi = (0xfbu64) as u32 as u64;
            // 7a7e3 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7a7ea mov rdi,r14
            rdi = r14;
            // 7a7ed mov rdx,rbx
            rdx = rbx;
            // 7a7f0 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a7f5 lea rdi,[rbp-0x710]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8f0);
            // 7a7fc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a801 mov DWORD PTR [rbp-0x720],0xfb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f8e0), (0xfbu64) as u32);
            // 7a80b mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7a812 mov QWORD PTR [rbp-0x718],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8e8), (rax) as u64);
            // 7a819 lea rsi,[rbp-0x720]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f8e0);
            // 7a820 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7a827 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a82c lea rsi,[rip+0xaf770] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7a833 lea rdi,[rbp-0x738]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8c8);
            // 7a83a mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7a83f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a844 movzx ebx,BYTE PTR [rbp-0x738]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f8c8)) as u64) as u32 as u64;
            // 7a84b test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7a84e jne 0x7a855
            if fl.ne() { pc = 0x7a855; continue; }
            pc = 0x7a850;
        }
        0x7a850 => {
            // block 7a850
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a850 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a850 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7a853 jmp 0x7a85c
            pc = 0x7a85c; continue;
        }
        0x7a855 => {
            // block 7a855
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a855 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a855 mov rbx,QWORD PTR [rbp-0x730]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8d0)) as u64;
            pc = 0x7a85c;
        }
        0x7a85c => {
            // block 7a85c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a85c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a85c lea rdi,[rbp-0x738]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8c8);
            // 7a863 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a868 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7a86b je 0x7a8fd
            if fl.e() { pc = 0x7a8fd; continue; }
            pc = 0x7a871;
        }
        0x7a871 => {
            // block 7a871
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a871 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a871 lea rsi,[rip+0xaf72b] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7a878 lea rdi,[rbp-0x770]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f890);
            // 7a87f mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7a884 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a889 mov rax,QWORD PTR [rbp-0x760]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8a0)) as u64;
            // 7a890 mov QWORD PTR [rbp-0x748],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8), (rax) as u64);
            // 7a897 mov rax,QWORD PTR [rbp-0x770]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f890)) as u64;
            // 7a89e mov rcx,QWORD PTR [rbp-0x768]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f898)) as u64;
            // 7a8a5 mov QWORD PTR [rbp-0x750],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8b0), (rcx) as u64);
            // 7a8ac mov QWORD PTR [rbp-0x758],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8a8), (rax) as u64);
            // 7a8b3 xorps xmm0,xmm0
            x0 = 0;
            // 7a8b6 movaps XMMWORD PTR [rbp-0x770],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f890), x0);
            // 7a8bd mov QWORD PTR [rbp-0x760],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8a0), (0x0u64) as u64);
            // 7a8c8 mov DWORD PTR [rbp-0x740],0xfb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f8c0), (0xfbu64) as u32);
            // 7a8d2 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7a8d9 lea rsi,[rbp-0x758]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f8a8);
            // 7a8e0 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a8e5 lea rdi,[rbp-0x758]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8a8);
            // 7a8ec call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a8f1 lea rdi,[rbp-0x770]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f890);
            // 7a8f8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7a8fd;
        }
        0x7a8fd => {
            // block 7a8fd
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a8fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a8fd xorps xmm0,xmm0
            x0 = 0;
            // 7a900 movaps XMMWORD PTR [rbp-0x790],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f870), x0);
            // 7a907 mov QWORD PTR [rbp-0x780],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f880), (0x0u64) as u64);
            // 7a912 lea rdi,[rbp-0x790]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f870);
            // 7a919 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a91e xorps xmm0,xmm0
            x0 = 0;
            // 7a921 movaps XMMWORD PTR [rbp-0x7b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f850), x0);
            // 7a928 mov QWORD PTR [rbp-0x7a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f860), (0x0u64) as u64);
            // 7a933 lea rdi,[rbp-0x7b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f850);
            // 7a93a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a93f lea rbx,[rip+0x87e7a] # 0x1027c0
            rbx = m.env.data_va(94093);
            // 7a946 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x7a949;
        }
        0x7a949 => {
            // block 7a949
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a949 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a949 mov rax,r14
            rax = r14;
            // 7a94c and rax,0xfffffffffffffffe
            { let r = rax & 0xffff_ffff_ffff_fffeu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 7a950 cmp rax,0x2
            fl = sub::<64>(rax, 0x2u64).1;
            // 7a954 je 0x7a9d0
            if fl.e() { pc = 0x7a9d0; continue; }
            pc = 0x7a956;
        }
        0x7a956 => {
            // block 7a956
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a956 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a956 mov DWORD PTR [rbp-0x7b8],0xfb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f848), (0xfbu64) as u32);
            // 7a960 mov DWORD PTR [rbp-0x7b4],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f84c), ((r14 as u32 as u64)) as u32);
            // 7a967 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7a96e lea rsi,[rbp-0x7b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f848);
            // 7a975 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a97a cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7a97d jne 0x7a9ab
            if fl.ne() { pc = 0x7a9ab; continue; }
            pc = 0x7a97f;
        }
        0x7a97f => {
            // block 7a97f
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a97f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a97f mov r12,r13
            r12 = r13;
            // 7a982 mov r13,r15
            r13 = r15;
            // 7a985 mov r15d,DWORD PTR [rbx]
            r15 = (m.r32(rbx) as u64) as u32 as u64;
            // 7a988 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7a98f lea rsi,[rbp-0x7b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f848);
            // 7a996 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a99b mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7a99e mov r15,r13
            r15 = r13;
            // 7a9a1 mov r13,r12
            r13 = r12;
            // 7a9a4 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7a9ab;
        }
        0x7a9ab => {
            // block 7a9ab
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a9ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a9ab mov rdi,r13
            rdi = r13;
            // 7a9ae mov rsi,rbx
            rsi = rbx;
            // 7a9b1 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a9b6 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7a9b9 jne 0x7a9d0
            if fl.ne() { pc = 0x7a9d0; continue; }
            pc = 0x7a9bb;
        }
        0x7a9bb => {
            // block 7a9bb
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a9bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a9bb mov rdi,r13
            rdi = r13;
            // 7a9be mov rsi,rbx
            rsi = rbx;
            // 7a9c1 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a9c6 mov DWORD PTR [rax],0xfb
            m.w32(rax, (0xfbu64) as u32);
            // 7a9cc mov DWORD PTR [rax+0x4],r14d
            m.w32(rax.wrapping_add(0x4), ((r14 as u32 as u64)) as u32);
            pc = 0x7a9d0;
        }
        0x7a9d0 => {
            // block 7a9d0
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a9d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a9d0 inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // 7a9d3 add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // 7a9d7 cmp r14,0x4
            fl = sub::<64>(r14, 0x4u64).1;
            // 7a9db jne 0x7a949
            if fl.ne() { pc = 0x7a949; continue; }
            pc = 0x7a9e1;
        }
        0x7a9e1 => {
            // block 7a9e1
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7a9e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a9e1 lea rsi,[rip+0xbe2e0] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7a9e8 lea rbx,[rbp-0x7d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f830);
            // 7a9ef xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7a9f1 mov rdi,rbx
            rdi = rbx;
            // 7a9f4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a9f9 mov esi,0xbf
            rsi = (0xbfu64) as u32 as u64;
            // 7a9fe mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7aa05 mov rdi,r14
            rdi = r14;
            // 7aa08 mov rdx,rbx
            rdx = rbx;
            // 7aa0b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7aa10 lea rdi,[rbp-0x7d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f830);
            // 7aa17 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7aa1c mov DWORD PTR [rbp-0x7e0],0xbf
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f820), (0xbfu64) as u32);
            // 7aa26 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7aa2d mov QWORD PTR [rbp-0x7d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f828), (rax) as u64);
            // 7aa34 lea rsi,[rbp-0x7e0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f820);
            // 7aa3b mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7aa42 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7aa47 lea rsi,[rip+0xaf555] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7aa4e lea rdi,[rbp-0x7f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f808);
            // 7aa55 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7aa5a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7aa5f movzx ebx,BYTE PTR [rbp-0x7f8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f808)) as u64) as u32 as u64;
            // 7aa66 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7aa69 jne 0x7aa70
            if fl.ne() { pc = 0x7aa70; continue; }
            pc = 0x7aa6b;
        }
        0x7aa6b => {
            // block 7aa6b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7aa6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7aa6b shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7aa6e jmp 0x7aa77
            pc = 0x7aa77; continue;
        }
        0x7aa70 => {
            // block 7aa70
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7aa70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7aa70 mov rbx,QWORD PTR [rbp-0x7f0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f810)) as u64;
            pc = 0x7aa77;
        }
        0x7aa77 => {
            // block 7aa77
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7aa77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7aa77 lea rdi,[rbp-0x7f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f808);
            // 7aa7e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7aa83 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7aa86 je 0x7ab18
            if fl.e() { pc = 0x7ab18; continue; }
            pc = 0x7aa8c;
        }
        0x7aa8c => {
            // block 7aa8c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7aa8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7aa8c lea rsi,[rip+0xaf510] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7aa93 lea rdi,[rbp-0x830]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7d0);
            // 7aa9a mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7aa9f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7aaa4 mov rax,QWORD PTR [rbp-0x820]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f7e0)) as u64;
            // 7aaab mov QWORD PTR [rbp-0x808],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f7f8), (rax) as u64);
            // 7aab2 mov rax,QWORD PTR [rbp-0x830]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f7d0)) as u64;
            // 7aab9 mov rcx,QWORD PTR [rbp-0x828]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f7d8)) as u64;
            // 7aac0 mov QWORD PTR [rbp-0x810],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f7f0), (rcx) as u64);
            // 7aac7 mov QWORD PTR [rbp-0x818],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f7e8), (rax) as u64);
            // 7aace xorps xmm0,xmm0
            x0 = 0;
            // 7aad1 movaps XMMWORD PTR [rbp-0x830],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f7d0), x0);
            // 7aad8 mov QWORD PTR [rbp-0x820],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f7e0), (0x0u64) as u64);
            // 7aae3 mov DWORD PTR [rbp-0x800],0xbf
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f800), (0xbfu64) as u32);
            // 7aaed lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7aaf4 lea rsi,[rbp-0x818]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f7e8);
            // 7aafb call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ab00 lea rdi,[rbp-0x818]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7e8);
            // 7ab07 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ab0c lea rdi,[rbp-0x830]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7d0);
            // 7ab13 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7ab18;
        }
        0x7ab18 => {
            // block 7ab18
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ab18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ab18 xorps xmm0,xmm0
            x0 = 0;
            // 7ab1b movaps XMMWORD PTR [rbp-0x850],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f7b0), x0);
            // 7ab22 mov QWORD PTR [rbp-0x840],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f7c0), (0x0u64) as u64);
            // 7ab2d lea rdi,[rbp-0x850]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7b0);
            // 7ab34 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ab39 xorps xmm0,xmm0
            x0 = 0;
            // 7ab3c movaps XMMWORD PTR [rbp-0x870],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f790), x0);
            // 7ab43 mov QWORD PTR [rbp-0x860],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f7a0), (0x0u64) as u64);
            // 7ab4e lea rdi,[rbp-0x870]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f790);
            // 7ab55 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ab5a lea r14,[rip+0x87c6f] # 0x1027d0
            r14 = m.env.data_va(94109);
            // 7ab61 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7ab63;
        }
        0x7ab63 => {
            // block 7ab63
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ab63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ab63 mov DWORD PTR [rbp-0x878],0xbf
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f788), (0xbfu64) as u32);
            // 7ab6d mov DWORD PTR [rbp-0x874],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f78c), ((rbx as u32 as u64)) as u32);
            // 7ab73 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7ab7a lea rsi,[rbp-0x878]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f788);
            // 7ab81 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ab86 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7ab89 jne 0x7abb7
            if fl.ne() { pc = 0x7abb7; continue; }
            pc = 0x7ab8b;
        }
        0x7ab8b => {
            // block 7ab8b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ab8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ab8b mov r12,r13
            r12 = r13;
            // 7ab8e mov r13,r15
            r13 = r15;
            // 7ab91 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7ab94 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7ab9b lea rsi,[rbp-0x878]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f788);
            // 7aba2 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7aba7 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7abaa mov r15,r13
            r15 = r13;
            // 7abad mov r13,r12
            r13 = r12;
            // 7abb0 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7abb7;
        }
        0x7abb7 => {
            // block 7abb7
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7abb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7abb7 mov rdi,r13
            rdi = r13;
            // 7abba mov rsi,r14
            rsi = r14;
            // 7abbd call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7abc2 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7abc5 jne 0x7abdb
            if fl.ne() { pc = 0x7abdb; continue; }
            pc = 0x7abc7;
        }
        0x7abc7 => {
            // block 7abc7
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7abc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7abc7 mov rdi,r13
            rdi = r13;
            // 7abca mov rsi,r14
            rsi = r14;
            // 7abcd call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7abd2 mov DWORD PTR [rax],0xbf
            m.w32(rax, (0xbfu64) as u32);
            // 7abd8 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7abdb;
        }
        0x7abdb => {
            // block 7abdb
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7abdb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7abdb inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7abde add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7abe2 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7abe6 jne 0x7ab63
            if fl.ne() { pc = 0x7ab63; continue; }
            pc = 0x7abec;
        }
        0x7abec => {
            // block 7abec
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7abec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7abec lea rsi,[rip+0xbe0d5] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7abf3 lea rbx,[rbp-0x890]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f770);
            // 7abfa xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7abfc mov rdi,rbx
            rdi = rbx;
            // 7abff call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ac04 mov esi,0xc1
            rsi = (0xc1u64) as u32 as u64;
            // 7ac09 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7ac10 mov rdi,r14
            rdi = r14;
            // 7ac13 mov rdx,rbx
            rdx = rbx;
            // 7ac16 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ac1b lea rdi,[rbp-0x890]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f770);
            // 7ac22 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ac27 mov DWORD PTR [rbp-0x8a0],0xc1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f760), (0xc1u64) as u32);
            // 7ac31 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7ac38 mov QWORD PTR [rbp-0x898],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f768), (rax) as u64);
            // 7ac3f lea rsi,[rbp-0x8a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f760);
            // 7ac46 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7ac4d call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ac52 lea rsi,[rip+0xaf34a] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7ac59 lea rdi,[rbp-0x8b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f748);
            // 7ac60 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7ac65 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ac6a movzx ebx,BYTE PTR [rbp-0x8b8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f748)) as u64) as u32 as u64;
            // 7ac71 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7ac74 jne 0x7ac7b
            if fl.ne() { pc = 0x7ac7b; continue; }
            pc = 0x7ac76;
        }
        0x7ac76 => {
            // block 7ac76
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ac76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ac76 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7ac79 jmp 0x7ac82
            pc = 0x7ac82; continue;
        }
        0x7ac7b => {
            // block 7ac7b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ac7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ac7b mov rbx,QWORD PTR [rbp-0x8b0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f750)) as u64;
            pc = 0x7ac82;
        }
        0x7ac82 => {
            // block 7ac82
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ac82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ac82 lea rdi,[rbp-0x8b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f748);
            // 7ac89 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ac8e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7ac91 je 0x7ad23
            if fl.e() { pc = 0x7ad23; continue; }
            pc = 0x7ac97;
        }
        0x7ac97 => {
            // block 7ac97
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ac97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ac97 lea rsi,[rip+0xaf305] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7ac9e lea rdi,[rbp-0x8f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f710);
            // 7aca5 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7acaa call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7acaf mov rax,QWORD PTR [rbp-0x8e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f720)) as u64;
            // 7acb6 mov QWORD PTR [rbp-0x8c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f738), (rax) as u64);
            // 7acbd mov rax,QWORD PTR [rbp-0x8f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f710)) as u64;
            // 7acc4 mov rcx,QWORD PTR [rbp-0x8e8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f718)) as u64;
            // 7accb mov QWORD PTR [rbp-0x8d0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f730), (rcx) as u64);
            // 7acd2 mov QWORD PTR [rbp-0x8d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f728), (rax) as u64);
            // 7acd9 xorps xmm0,xmm0
            x0 = 0;
            // 7acdc movaps XMMWORD PTR [rbp-0x8f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f710), x0);
            // 7ace3 mov QWORD PTR [rbp-0x8e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f720), (0x0u64) as u64);
            // 7acee mov DWORD PTR [rbp-0x8c0],0xc1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f740), (0xc1u64) as u32);
            // 7acf8 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7acff lea rsi,[rbp-0x8d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f728);
            // 7ad06 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ad0b lea rdi,[rbp-0x8d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f728);
            // 7ad12 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ad17 lea rdi,[rbp-0x8f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f710);
            // 7ad1e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7ad23;
        }
        0x7ad23 => {
            // block 7ad23
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ad23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ad23 xorps xmm0,xmm0
            x0 = 0;
            // 7ad26 movaps XMMWORD PTR [rbp-0x910],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f6f0), x0);
            // 7ad2d mov QWORD PTR [rbp-0x900],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f700), (0x0u64) as u64);
            // 7ad38 lea rdi,[rbp-0x910]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6f0);
            // 7ad3f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ad44 xorps xmm0,xmm0
            x0 = 0;
            // 7ad47 movaps XMMWORD PTR [rbp-0x930],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f6d0), x0);
            // 7ad4e mov QWORD PTR [rbp-0x920],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f6e0), (0x0u64) as u64);
            // 7ad59 lea rdi,[rbp-0x930]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6d0);
            // 7ad60 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ad65 lea r14,[rip+0x87a74] # 0x1027e0
            r14 = m.env.data_va(94125);
            // 7ad6c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7ad6e;
        }
        0x7ad6e => {
            // block 7ad6e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ad6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ad6e cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 7ad71 je 0x7adeb
            if fl.e() { pc = 0x7adeb; continue; }
            pc = 0x7ad73;
        }
        0x7ad73 => {
            // block 7ad73
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ad73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ad73 mov DWORD PTR [rbp-0x938],0xc1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f6c8), (0xc1u64) as u32);
            // 7ad7d mov DWORD PTR [rbp-0x934],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f6cc), ((rbx as u32 as u64)) as u32);
            // 7ad83 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7ad8a lea rsi,[rbp-0x938]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f6c8);
            // 7ad91 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ad96 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7ad99 jne 0x7adc7
            if fl.ne() { pc = 0x7adc7; continue; }
            pc = 0x7ad9b;
        }
        0x7ad9b => {
            // block 7ad9b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ad9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ad9b mov r12,r13
            r12 = r13;
            // 7ad9e mov r13,r15
            r13 = r15;
            // 7ada1 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7ada4 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7adab lea rsi,[rbp-0x938]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f6c8);
            // 7adb2 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7adb7 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7adba mov r15,r13
            r15 = r13;
            // 7adbd mov r13,r12
            r13 = r12;
            // 7adc0 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7adc7;
        }
        0x7adc7 => {
            // block 7adc7
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7adc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7adc7 mov rdi,r13
            rdi = r13;
            // 7adca mov rsi,r14
            rsi = r14;
            // 7adcd call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7add2 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7add5 jne 0x7adeb
            if fl.ne() { pc = 0x7adeb; continue; }
            pc = 0x7add7;
        }
        0x7add7 => {
            // block 7add7
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7add7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7add7 mov rdi,r13
            rdi = r13;
            // 7adda mov rsi,r14
            rsi = r14;
            // 7addd call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ade2 mov DWORD PTR [rax],0xc1
            m.w32(rax, (0xc1u64) as u32);
            // 7ade8 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7adeb;
        }
        0x7adeb => {
            // block 7adeb
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7adeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7adeb inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7adee add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7adf2 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7adf6 jne 0x7ad6e
            if fl.ne() { pc = 0x7ad6e; continue; }
            pc = 0x7adfc;
        }
        0x7adfc => {
            // block 7adfc
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7adfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7adfc lea rsi,[rip+0xbdec5] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7ae03 lea rbx,[rbp-0x950]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f6b0);
            // 7ae0a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7ae0c mov rdi,rbx
            rdi = rbx;
            // 7ae0f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ae14 mov esi,0x107
            rsi = (0x107u64) as u32 as u64;
            // 7ae19 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7ae20 mov rdi,r14
            rdi = r14;
            // 7ae23 mov rdx,rbx
            rdx = rbx;
            // 7ae26 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ae2b lea rdi,[rbp-0x950]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6b0);
            // 7ae32 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ae37 mov DWORD PTR [rbp-0x960],0x107
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f6a0), (0x107u64) as u32);
            // 7ae41 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7ae48 mov QWORD PTR [rbp-0x958],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f6a8), (rax) as u64);
            // 7ae4f lea rsi,[rbp-0x960]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f6a0);
            // 7ae56 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7ae5d call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ae62 lea rsi,[rip+0xaf13a] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7ae69 lea rdi,[rbp-0x978]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f688);
            // 7ae70 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7ae75 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ae7a movzx ebx,BYTE PTR [rbp-0x978]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f688)) as u64) as u32 as u64;
            // 7ae81 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7ae84 jne 0x7ae8b
            if fl.ne() { pc = 0x7ae8b; continue; }
            pc = 0x7ae86;
        }
        0x7ae86 => {
            // block 7ae86
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ae86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ae86 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7ae89 jmp 0x7ae92
            pc = 0x7ae92; continue;
        }
        0x7ae8b => {
            // block 7ae8b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ae8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ae8b mov rbx,QWORD PTR [rbp-0x970]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f690)) as u64;
            pc = 0x7ae92;
        }
        0x7ae92 => {
            // block 7ae92
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ae92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ae92 lea rdi,[rbp-0x978]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f688);
            // 7ae99 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ae9e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7aea1 je 0x7af33
            if fl.e() { pc = 0x7af33; continue; }
            pc = 0x7aea7;
        }
        0x7aea7 => {
            // block 7aea7
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7aea7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7aea7 lea rsi,[rip+0xaf0f5] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7aeae lea rdi,[rbp-0x9b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f650);
            // 7aeb5 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7aeba call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7aebf mov rax,QWORD PTR [rbp-0x9a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f660)) as u64;
            // 7aec6 mov QWORD PTR [rbp-0x988],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f678), (rax) as u64);
            // 7aecd mov rax,QWORD PTR [rbp-0x9b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f650)) as u64;
            // 7aed4 mov rcx,QWORD PTR [rbp-0x9a8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f658)) as u64;
            // 7aedb mov QWORD PTR [rbp-0x990],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f670), (rcx) as u64);
            // 7aee2 mov QWORD PTR [rbp-0x998],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f668), (rax) as u64);
            // 7aee9 xorps xmm0,xmm0
            x0 = 0;
            // 7aeec movaps XMMWORD PTR [rbp-0x9b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f650), x0);
            // 7aef3 mov QWORD PTR [rbp-0x9a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f660), (0x0u64) as u64);
            // 7aefe mov DWORD PTR [rbp-0x980],0x107
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f680), (0x107u64) as u32);
            // 7af08 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7af0f lea rsi,[rbp-0x998]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f668);
            // 7af16 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7af1b lea rdi,[rbp-0x998]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f668);
            // 7af22 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7af27 lea rdi,[rbp-0x9b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f650);
            // 7af2e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7af33;
        }
        0x7af33 => {
            // block 7af33
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7af33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7af33 xorps xmm0,xmm0
            x0 = 0;
            // 7af36 movaps XMMWORD PTR [rbp-0x9d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f630), x0);
            // 7af3d mov QWORD PTR [rbp-0x9c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f640), (0x0u64) as u64);
            // 7af48 lea rdi,[rbp-0x9d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f630);
            // 7af4f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7af54 xorps xmm0,xmm0
            x0 = 0;
            // 7af57 movaps XMMWORD PTR [rbp-0x9f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f610), x0);
            // 7af5e mov QWORD PTR [rbp-0x9e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f620), (0x0u64) as u64);
            // 7af69 lea rdi,[rbp-0x9f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f610);
            // 7af70 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7af75 lea r14,[rip+0x87874] # 0x1027f0
            r14 = m.env.data_va(94141);
            // 7af7c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7af7e;
        }
        0x7af7e => {
            // block 7af7e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7af7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7af7e mov rax,rbx
            rax = rbx;
            // 7af81 and rax,0xfffffffffffffffd
            { let r = rax & 0xffff_ffff_ffff_fffdu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 7af85 cmp rax,0x1
            fl = sub::<64>(rax, 0x1u64).1;
            // 7af89 je 0x7b003
            if fl.e() { pc = 0x7b003; continue; }
            pc = 0x7af8b;
        }
        0x7af8b => {
            // block 7af8b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7af8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7af8b mov DWORD PTR [rbp-0x9f8],0x107
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f608), (0x107u64) as u32);
            // 7af95 mov DWORD PTR [rbp-0x9f4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f60c), ((rbx as u32 as u64)) as u32);
            // 7af9b mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7afa2 lea rsi,[rbp-0x9f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f608);
            // 7afa9 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7afae cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7afb1 jne 0x7afdf
            if fl.ne() { pc = 0x7afdf; continue; }
            pc = 0x7afb3;
        }
        0x7afb3 => {
            // block 7afb3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7afb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7afb3 mov r12,r13
            r12 = r13;
            // 7afb6 mov r13,r15
            r13 = r15;
            // 7afb9 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7afbc mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7afc3 lea rsi,[rbp-0x9f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f608);
            // 7afca call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7afcf mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7afd2 mov r15,r13
            r15 = r13;
            // 7afd5 mov r13,r12
            r13 = r12;
            // 7afd8 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7afdf;
        }
        0x7afdf => {
            // block 7afdf
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7afdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7afdf mov rdi,r13
            rdi = r13;
            // 7afe2 mov rsi,r14
            rsi = r14;
            // 7afe5 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7afea cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7afed jne 0x7b003
            if fl.ne() { pc = 0x7b003; continue; }
            pc = 0x7afef;
        }
        0x7afef => {
            // block 7afef
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7afef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7afef mov rdi,r13
            rdi = r13;
            // 7aff2 mov rsi,r14
            rsi = r14;
            // 7aff5 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7affa mov DWORD PTR [rax],0x107
            m.w32(rax, (0x107u64) as u32);
            // 7b000 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7b003;
        }
        0x7b003 => {
            // block 7b003
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b003 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b003 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7b006 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7b00a cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7b00e jne 0x7af7e
            if fl.ne() { pc = 0x7af7e; continue; }
            pc = 0x7b014;
        }
        0x7b014 => {
            // block 7b014
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b014 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b014 lea rsi,[rip+0xbdcad] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7b01b lea rbx,[rbp-0xa10]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f5f0);
            // 7b022 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7b024 mov rdi,rbx
            rdi = rbx;
            // 7b027 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b02c mov esi,0x108
            rsi = (0x108u64) as u32 as u64;
            // 7b031 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7b038 mov rdi,r14
            rdi = r14;
            // 7b03b mov rdx,rbx
            rdx = rbx;
            // 7b03e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b043 lea rdi,[rbp-0xa10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5f0);
            // 7b04a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b04f mov DWORD PTR [rbp-0xa20],0x108
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f5e0), (0x108u64) as u32);
            // 7b059 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7b060 mov QWORD PTR [rbp-0xa18],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5e8), (rax) as u64);
            // 7b067 lea rsi,[rbp-0xa20]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f5e0);
            // 7b06e mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7b075 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b07a lea rsi,[rip+0xaef22] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7b081 lea rdi,[rbp-0xa38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5c8);
            // 7b088 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7b08d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b092 movzx ebx,BYTE PTR [rbp-0xa38]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f5c8)) as u64) as u32 as u64;
            // 7b099 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7b09c jne 0x7b0a3
            if fl.ne() { pc = 0x7b0a3; continue; }
            pc = 0x7b09e;
        }
        0x7b09e => {
            // block 7b09e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b09e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b09e shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7b0a1 jmp 0x7b0aa
            pc = 0x7b0aa; continue;
        }
        0x7b0a3 => {
            // block 7b0a3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b0a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b0a3 mov rbx,QWORD PTR [rbp-0xa30]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f5d0)) as u64;
            pc = 0x7b0aa;
        }
        0x7b0aa => {
            // block 7b0aa
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b0aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b0aa lea rdi,[rbp-0xa38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5c8);
            // 7b0b1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b0b6 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7b0b9 je 0x7b14b
            if fl.e() { pc = 0x7b14b; continue; }
            pc = 0x7b0bf;
        }
        0x7b0bf => {
            // block 7b0bf
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b0bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b0bf lea rsi,[rip+0xaeedd] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7b0c6 lea rdi,[rbp-0xa70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f590);
            // 7b0cd mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7b0d2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b0d7 mov rax,QWORD PTR [rbp-0xa60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f5a0)) as u64;
            // 7b0de mov QWORD PTR [rbp-0xa48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5b8), (rax) as u64);
            // 7b0e5 mov rax,QWORD PTR [rbp-0xa70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f590)) as u64;
            // 7b0ec mov rcx,QWORD PTR [rbp-0xa68]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f598)) as u64;
            // 7b0f3 mov QWORD PTR [rbp-0xa50],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5b0), (rcx) as u64);
            // 7b0fa mov QWORD PTR [rbp-0xa58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5a8), (rax) as u64);
            // 7b101 xorps xmm0,xmm0
            x0 = 0;
            // 7b104 movaps XMMWORD PTR [rbp-0xa70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f590), x0);
            // 7b10b mov QWORD PTR [rbp-0xa60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f5a0), (0x0u64) as u64);
            // 7b116 mov DWORD PTR [rbp-0xa40],0x108
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f5c0), (0x108u64) as u32);
            // 7b120 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7b127 lea rsi,[rbp-0xa58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f5a8);
            // 7b12e call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b133 lea rdi,[rbp-0xa58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5a8);
            // 7b13a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b13f lea rdi,[rbp-0xa70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f590);
            // 7b146 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7b14b;
        }
        0x7b14b => {
            // block 7b14b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b14b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b14b xorps xmm0,xmm0
            x0 = 0;
            // 7b14e movaps XMMWORD PTR [rbp-0xa90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f570), x0);
            // 7b155 mov QWORD PTR [rbp-0xa80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f580), (0x0u64) as u64);
            // 7b160 lea rdi,[rbp-0xa90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f570);
            // 7b167 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b16c xorps xmm0,xmm0
            x0 = 0;
            // 7b16f movaps XMMWORD PTR [rbp-0xab0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f550), x0);
            // 7b176 mov QWORD PTR [rbp-0xaa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f560), (0x0u64) as u64);
            // 7b181 lea rdi,[rbp-0xab0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f550);
            // 7b188 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b18d lea r14,[rip+0x8766c] # 0x102800
            r14 = m.env.data_va(94157);
            // 7b194 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7b196;
        }
        0x7b196 => {
            // block 7b196
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b196 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b196 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 7b198 je 0x7b212
            if fl.e() { pc = 0x7b212; continue; }
            pc = 0x7b19a;
        }
        0x7b19a => {
            // block 7b19a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b19a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b19a mov DWORD PTR [rbp-0xab8],0x108
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f548), (0x108u64) as u32);
            // 7b1a4 mov DWORD PTR [rbp-0xab4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f54c), ((rbx as u32 as u64)) as u32);
            // 7b1aa mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7b1b1 lea rsi,[rbp-0xab8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f548);
            // 7b1b8 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b1bd cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7b1c0 jne 0x7b1ee
            if fl.ne() { pc = 0x7b1ee; continue; }
            pc = 0x7b1c2;
        }
        0x7b1c2 => {
            // block 7b1c2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b1c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b1c2 mov r12,r13
            r12 = r13;
            // 7b1c5 mov r13,r15
            r13 = r15;
            // 7b1c8 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7b1cb mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7b1d2 lea rsi,[rbp-0xab8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f548);
            // 7b1d9 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b1de mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7b1e1 mov r15,r13
            r15 = r13;
            // 7b1e4 mov r13,r12
            r13 = r12;
            // 7b1e7 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7b1ee;
        }
        0x7b1ee => {
            // block 7b1ee
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b1ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b1ee mov rdi,r13
            rdi = r13;
            // 7b1f1 mov rsi,r14
            rsi = r14;
            // 7b1f4 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b1f9 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7b1fc jne 0x7b212
            if fl.ne() { pc = 0x7b212; continue; }
            pc = 0x7b1fe;
        }
        0x7b1fe => {
            // block 7b1fe
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b1fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b1fe mov rdi,r13
            rdi = r13;
            // 7b201 mov rsi,r14
            rsi = r14;
            // 7b204 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b209 mov DWORD PTR [rax],0x108
            m.w32(rax, (0x108u64) as u32);
            // 7b20f mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7b212;
        }
        0x7b212 => {
            // block 7b212
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b212 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b212 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7b215 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7b219 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7b21d jne 0x7b196
            if fl.ne() { pc = 0x7b196; continue; }
            pc = 0x7b223;
        }
        0x7b223 => {
            // block 7b223
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b223 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b223 lea rsi,[rip+0xbda9e] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7b22a lea rbx,[rbp-0xad0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f530);
            // 7b231 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7b233 mov rdi,rbx
            rdi = rbx;
            // 7b236 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b23b mov esi,0x12e
            rsi = (0x12eu64) as u32 as u64;
            // 7b240 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7b247 mov rdi,r14
            rdi = r14;
            // 7b24a mov rdx,rbx
            rdx = rbx;
            // 7b24d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b252 lea rdi,[rbp-0xad0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f530);
            // 7b259 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b25e mov DWORD PTR [rbp-0xae0],0x12e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f520), (0x12eu64) as u32);
            // 7b268 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7b26f mov QWORD PTR [rbp-0xad8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f528), (rax) as u64);
            // 7b276 lea rsi,[rbp-0xae0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f520);
            // 7b27d mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7b284 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b289 lea rsi,[rip+0xaed13] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7b290 lea rdi,[rbp-0xaf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f508);
            // 7b297 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7b29c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b2a1 movzx ebx,BYTE PTR [rbp-0xaf8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f508)) as u64) as u32 as u64;
            // 7b2a8 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7b2ab jne 0x7b2b2
            if fl.ne() { pc = 0x7b2b2; continue; }
            pc = 0x7b2ad;
        }
        0x7b2ad => {
            // block 7b2ad
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b2ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b2ad shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7b2b0 jmp 0x7b2b9
            pc = 0x7b2b9; continue;
        }
        0x7b2b2 => {
            // block 7b2b2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b2b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b2b2 mov rbx,QWORD PTR [rbp-0xaf0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f510)) as u64;
            pc = 0x7b2b9;
        }
        0x7b2b9 => {
            // block 7b2b9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b2b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b2b9 lea rdi,[rbp-0xaf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f508);
            // 7b2c0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b2c5 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7b2c8 je 0x7b35a
            if fl.e() { pc = 0x7b35a; continue; }
            pc = 0x7b2ce;
        }
        0x7b2ce => {
            // block 7b2ce
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b2ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b2ce lea rsi,[rip+0xaecce] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7b2d5 lea rdi,[rbp-0xb30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4d0);
            // 7b2dc mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7b2e1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b2e6 mov rax,QWORD PTR [rbp-0xb20]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f4e0)) as u64;
            // 7b2ed mov QWORD PTR [rbp-0xb08],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f4f8), (rax) as u64);
            // 7b2f4 mov rax,QWORD PTR [rbp-0xb30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f4d0)) as u64;
            // 7b2fb mov rcx,QWORD PTR [rbp-0xb28]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f4d8)) as u64;
            // 7b302 mov QWORD PTR [rbp-0xb10],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f4f0), (rcx) as u64);
            // 7b309 mov QWORD PTR [rbp-0xb18],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f4e8), (rax) as u64);
            // 7b310 xorps xmm0,xmm0
            x0 = 0;
            // 7b313 movaps XMMWORD PTR [rbp-0xb30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f4d0), x0);
            // 7b31a mov QWORD PTR [rbp-0xb20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f4e0), (0x0u64) as u64);
            // 7b325 mov DWORD PTR [rbp-0xb00],0x12e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f500), (0x12eu64) as u32);
            // 7b32f lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7b336 lea rsi,[rbp-0xb18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f4e8);
            // 7b33d call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b342 lea rdi,[rbp-0xb18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4e8);
            // 7b349 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b34e lea rdi,[rbp-0xb30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4d0);
            // 7b355 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7b35a;
        }
        0x7b35a => {
            // block 7b35a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b35a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b35a xorps xmm0,xmm0
            x0 = 0;
            // 7b35d movaps XMMWORD PTR [rbp-0xb50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f4b0), x0);
            // 7b364 mov QWORD PTR [rbp-0xb40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f4c0), (0x0u64) as u64);
            // 7b36f lea rdi,[rbp-0xb50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4b0);
            // 7b376 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b37b xorps xmm0,xmm0
            x0 = 0;
            // 7b37e movaps XMMWORD PTR [rbp-0xb70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f490), x0);
            // 7b385 mov QWORD PTR [rbp-0xb60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f4a0), (0x0u64) as u64);
            // 7b390 lea rdi,[rbp-0xb70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f490);
            // 7b397 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b39c lea r14,[rip+0x8746d] # 0x102810
            r14 = m.env.data_va(94173);
            // 7b3a3 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7b3a5;
        }
        0x7b3a5 => {
            // block 7b3a5
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b3a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b3a5 mov DWORD PTR [rbp-0xb78],0x12e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f488), (0x12eu64) as u32);
            // 7b3af mov DWORD PTR [rbp-0xb74],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f48c), ((rbx as u32 as u64)) as u32);
            // 7b3b5 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7b3bc lea rsi,[rbp-0xb78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f488);
            // 7b3c3 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b3c8 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7b3cb jne 0x7b3f9
            if fl.ne() { pc = 0x7b3f9; continue; }
            pc = 0x7b3cd;
        }
        0x7b3cd => {
            // block 7b3cd
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b3cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b3cd mov r12,r13
            r12 = r13;
            // 7b3d0 mov r13,r15
            r13 = r15;
            // 7b3d3 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7b3d6 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7b3dd lea rsi,[rbp-0xb78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f488);
            // 7b3e4 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b3e9 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7b3ec mov r15,r13
            r15 = r13;
            // 7b3ef mov r13,r12
            r13 = r12;
            // 7b3f2 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7b3f9;
        }
        0x7b3f9 => {
            // block 7b3f9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b3f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b3f9 mov rdi,r13
            rdi = r13;
            // 7b3fc mov rsi,r14
            rsi = r14;
            // 7b3ff call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b404 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7b407 jne 0x7b41d
            if fl.ne() { pc = 0x7b41d; continue; }
            pc = 0x7b409;
        }
        0x7b409 => {
            // block 7b409
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b409 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b409 mov rdi,r13
            rdi = r13;
            // 7b40c mov rsi,r14
            rsi = r14;
            // 7b40f call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b414 mov DWORD PTR [rax],0x12e
            m.w32(rax, (0x12eu64) as u32);
            // 7b41a mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7b41d;
        }
        0x7b41d => {
            // block 7b41d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b41d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b41d inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7b420 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7b424 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7b428 jne 0x7b3a5
            if fl.ne() { pc = 0x7b3a5; continue; }
            pc = 0x7b42e;
        }
        0x7b42e => {
            // block 7b42e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b42e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b42e lea rsi,[rip+0xbd893] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7b435 lea rbx,[rbp-0xb90]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f470);
            // 7b43c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7b43e mov rdi,rbx
            rdi = rbx;
            // 7b441 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b446 mov esi,0x12f
            rsi = (0x12fu64) as u32 as u64;
            // 7b44b mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7b452 mov rdi,r14
            rdi = r14;
            // 7b455 mov rdx,rbx
            rdx = rbx;
            // 7b458 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b45d lea rdi,[rbp-0xb90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f470);
            // 7b464 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b469 mov DWORD PTR [rbp-0xba0],0x12f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f460), (0x12fu64) as u32);
            // 7b473 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7b47a mov QWORD PTR [rbp-0xb98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f468), (rax) as u64);
            // 7b481 lea rsi,[rbp-0xba0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f460);
            // 7b488 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7b48f call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b494 lea rsi,[rip+0xaeb08] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7b49b lea rdi,[rbp-0xbb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f448);
            // 7b4a2 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7b4a7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b4ac movzx ebx,BYTE PTR [rbp-0xbb8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f448)) as u64) as u32 as u64;
            // 7b4b3 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7b4b6 jne 0x7b4bd
            if fl.ne() { pc = 0x7b4bd; continue; }
            pc = 0x7b4b8;
        }
        0x7b4b8 => {
            // block 7b4b8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b4b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b4b8 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7b4bb jmp 0x7b4c4
            pc = 0x7b4c4; continue;
        }
        0x7b4bd => {
            // block 7b4bd
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b4bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b4bd mov rbx,QWORD PTR [rbp-0xbb0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f450)) as u64;
            pc = 0x7b4c4;
        }
        0x7b4c4 => {
            // block 7b4c4
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b4c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b4c4 lea rdi,[rbp-0xbb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f448);
            // 7b4cb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b4d0 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7b4d3 je 0x7b565
            if fl.e() { pc = 0x7b565; continue; }
            pc = 0x7b4d9;
        }
        0x7b4d9 => {
            // block 7b4d9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b4d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b4d9 lea rsi,[rip+0xaeac3] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7b4e0 lea rdi,[rbp-0xbf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f410);
            // 7b4e7 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7b4ec call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b4f1 mov rax,QWORD PTR [rbp-0xbe0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f420)) as u64;
            // 7b4f8 mov QWORD PTR [rbp-0xbc8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f438), (rax) as u64);
            // 7b4ff mov rax,QWORD PTR [rbp-0xbf0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f410)) as u64;
            // 7b506 mov rcx,QWORD PTR [rbp-0xbe8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f418)) as u64;
            // 7b50d mov QWORD PTR [rbp-0xbd0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f430), (rcx) as u64);
            // 7b514 mov QWORD PTR [rbp-0xbd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f428), (rax) as u64);
            // 7b51b xorps xmm0,xmm0
            x0 = 0;
            // 7b51e movaps XMMWORD PTR [rbp-0xbf0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f410), x0);
            // 7b525 mov QWORD PTR [rbp-0xbe0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f420), (0x0u64) as u64);
            // 7b530 mov DWORD PTR [rbp-0xbc0],0x12f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f440), (0x12fu64) as u32);
            // 7b53a lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7b541 lea rsi,[rbp-0xbd8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f428);
            // 7b548 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b54d lea rdi,[rbp-0xbd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f428);
            // 7b554 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b559 lea rdi,[rbp-0xbf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f410);
            // 7b560 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7b565;
        }
        0x7b565 => {
            // block 7b565
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b565 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b565 xorps xmm0,xmm0
            x0 = 0;
            // 7b568 movaps XMMWORD PTR [rbp-0xc10],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f3f0), x0);
            // 7b56f mov QWORD PTR [rbp-0xc00],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f400), (0x0u64) as u64);
            // 7b57a lea rdi,[rbp-0xc10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3f0);
            // 7b581 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b586 xorps xmm0,xmm0
            x0 = 0;
            // 7b589 movaps XMMWORD PTR [rbp-0xc30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f3d0), x0);
            // 7b590 mov QWORD PTR [rbp-0xc20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f3e0), (0x0u64) as u64);
            // 7b59b lea rdi,[rbp-0xc30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3d0);
            // 7b5a2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b5a7 lea r14,[rip+0x87272] # 0x102820
            r14 = m.env.data_va(94189);
            // 7b5ae xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7b5b0;
        }
        0x7b5b0 => {
            // block 7b5b0
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b5b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b5b0 cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 7b5b3 je 0x7b62d
            if fl.e() { pc = 0x7b62d; continue; }
            pc = 0x7b5b5;
        }
        0x7b5b5 => {
            // block 7b5b5
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b5b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b5b5 mov DWORD PTR [rbp-0xc38],0x12f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f3c8), (0x12fu64) as u32);
            // 7b5bf mov DWORD PTR [rbp-0xc34],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f3cc), ((rbx as u32 as u64)) as u32);
            // 7b5c5 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7b5cc lea rsi,[rbp-0xc38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f3c8);
            // 7b5d3 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b5d8 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7b5db jne 0x7b609
            if fl.ne() { pc = 0x7b609; continue; }
            pc = 0x7b5dd;
        }
        0x7b5dd => {
            // block 7b5dd
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b5dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b5dd mov r12,r13
            r12 = r13;
            // 7b5e0 mov r13,r15
            r13 = r15;
            // 7b5e3 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7b5e6 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7b5ed lea rsi,[rbp-0xc38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f3c8);
            // 7b5f4 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b5f9 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7b5fc mov r15,r13
            r15 = r13;
            // 7b5ff mov r13,r12
            r13 = r12;
            // 7b602 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7b609;
        }
        0x7b609 => {
            // block 7b609
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b609 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b609 mov rdi,r13
            rdi = r13;
            // 7b60c mov rsi,r14
            rsi = r14;
            // 7b60f call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b614 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7b617 jne 0x7b62d
            if fl.ne() { pc = 0x7b62d; continue; }
            pc = 0x7b619;
        }
        0x7b619 => {
            // block 7b619
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b619 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b619 mov rdi,r13
            rdi = r13;
            // 7b61c mov rsi,r14
            rsi = r14;
            // 7b61f call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b624 mov DWORD PTR [rax],0x12f
            m.w32(rax, (0x12fu64) as u32);
            // 7b62a mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7b62d;
        }
        0x7b62d => {
            // block 7b62d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b62d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b62d inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7b630 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7b634 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7b638 jne 0x7b5b0
            if fl.ne() { pc = 0x7b5b0; continue; }
            pc = 0x7b63e;
        }
        0x7b63e => {
            // block 7b63e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b63e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b63e lea rsi,[rip+0xbd683] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7b645 lea rbx,[rbp-0xc50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f3b0);
            // 7b64c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7b64e mov rdi,rbx
            rdi = rbx;
            // 7b651 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b656 mov esi,0x12d
            rsi = (0x12du64) as u32 as u64;
            // 7b65b mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7b662 mov rdi,r14
            rdi = r14;
            // 7b665 mov rdx,rbx
            rdx = rbx;
            // 7b668 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b66d lea rdi,[rbp-0xc50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3b0);
            // 7b674 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b679 mov DWORD PTR [rbp-0xc60],0x12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f3a0), (0x12du64) as u32);
            // 7b683 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7b68a mov QWORD PTR [rbp-0xc58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f3a8), (rax) as u64);
            // 7b691 lea rsi,[rbp-0xc60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f3a0);
            // 7b698 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7b69f call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b6a4 lea rsi,[rip+0xae8f8] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7b6ab lea rdi,[rbp-0xc78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f388);
            // 7b6b2 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7b6b7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b6bc movzx ebx,BYTE PTR [rbp-0xc78]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f388)) as u64) as u32 as u64;
            // 7b6c3 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7b6c6 jne 0x7b6cd
            if fl.ne() { pc = 0x7b6cd; continue; }
            pc = 0x7b6c8;
        }
        0x7b6c8 => {
            // block 7b6c8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b6c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b6c8 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7b6cb jmp 0x7b6d4
            pc = 0x7b6d4; continue;
        }
        0x7b6cd => {
            // block 7b6cd
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b6cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b6cd mov rbx,QWORD PTR [rbp-0xc70]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f390)) as u64;
            pc = 0x7b6d4;
        }
        0x7b6d4 => {
            // block 7b6d4
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b6d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b6d4 lea rdi,[rbp-0xc78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f388);
            // 7b6db call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b6e0 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7b6e3 je 0x7b775
            if fl.e() { pc = 0x7b775; continue; }
            pc = 0x7b6e9;
        }
        0x7b6e9 => {
            // block 7b6e9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b6e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b6e9 lea rsi,[rip+0xae8b3] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7b6f0 lea rdi,[rbp-0xcb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f350);
            // 7b6f7 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7b6fc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b701 mov rax,QWORD PTR [rbp-0xca0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f360)) as u64;
            // 7b708 mov QWORD PTR [rbp-0xc88],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f378), (rax) as u64);
            // 7b70f mov rax,QWORD PTR [rbp-0xcb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f350)) as u64;
            // 7b716 mov rcx,QWORD PTR [rbp-0xca8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f358)) as u64;
            // 7b71d mov QWORD PTR [rbp-0xc90],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f370), (rcx) as u64);
            // 7b724 mov QWORD PTR [rbp-0xc98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f368), (rax) as u64);
            // 7b72b xorps xmm0,xmm0
            x0 = 0;
            // 7b72e movaps XMMWORD PTR [rbp-0xcb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f350), x0);
            // 7b735 mov QWORD PTR [rbp-0xca0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f360), (0x0u64) as u64);
            // 7b740 mov DWORD PTR [rbp-0xc80],0x12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f380), (0x12du64) as u32);
            // 7b74a lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7b751 lea rsi,[rbp-0xc98]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f368);
            // 7b758 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b75d lea rdi,[rbp-0xc98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f368);
            // 7b764 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b769 lea rdi,[rbp-0xcb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f350);
            // 7b770 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7b775;
        }
        0x7b775 => {
            // block 7b775
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b775 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b775 xorps xmm0,xmm0
            x0 = 0;
            // 7b778 movaps XMMWORD PTR [rbp-0xcd0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f330), x0);
            // 7b77f mov QWORD PTR [rbp-0xcc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f340), (0x0u64) as u64);
            // 7b78a lea rdi,[rbp-0xcd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f330);
            // 7b791 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b796 xorps xmm0,xmm0
            x0 = 0;
            // 7b799 movaps XMMWORD PTR [rbp-0xcf0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f310), x0);
            // 7b7a0 mov QWORD PTR [rbp-0xce0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f320), (0x0u64) as u64);
            // 7b7ab lea rdi,[rbp-0xcf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f310);
            // 7b7b2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b7b7 lea r14,[rip+0x87072] # 0x102830
            r14 = m.env.data_va(94205);
            // 7b7be xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7b7c0;
        }
        0x7b7c0 => {
            // block 7b7c0
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b7c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b7c0 mov DWORD PTR [rbp-0xcf8],0x12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f308), (0x12du64) as u32);
            // 7b7ca mov DWORD PTR [rbp-0xcf4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f30c), ((rbx as u32 as u64)) as u32);
            // 7b7d0 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7b7d7 lea rsi,[rbp-0xcf8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f308);
            // 7b7de call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b7e3 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7b7e6 jne 0x7b814
            if fl.ne() { pc = 0x7b814; continue; }
            pc = 0x7b7e8;
        }
        0x7b7e8 => {
            // block 7b7e8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b7e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b7e8 mov r12,r13
            r12 = r13;
            // 7b7eb mov r13,r15
            r13 = r15;
            // 7b7ee mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7b7f1 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7b7f8 lea rsi,[rbp-0xcf8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f308);
            // 7b7ff call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b804 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7b807 mov r15,r13
            r15 = r13;
            // 7b80a mov r13,r12
            r13 = r12;
            // 7b80d mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7b814;
        }
        0x7b814 => {
            // block 7b814
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b814 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b814 mov rdi,r13
            rdi = r13;
            // 7b817 mov rsi,r14
            rsi = r14;
            // 7b81a call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b81f cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7b822 jne 0x7b838
            if fl.ne() { pc = 0x7b838; continue; }
            pc = 0x7b824;
        }
        0x7b824 => {
            // block 7b824
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b824 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b824 mov rdi,r13
            rdi = r13;
            // 7b827 mov rsi,r14
            rsi = r14;
            // 7b82a call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b82f mov DWORD PTR [rax],0x12d
            m.w32(rax, (0x12du64) as u32);
            // 7b835 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7b838;
        }
        0x7b838 => {
            // block 7b838
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b838 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b838 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7b83b add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7b83f cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7b843 jne 0x7b7c0
            if fl.ne() { pc = 0x7b7c0; continue; }
            pc = 0x7b849;
        }
        0x7b849 => {
            // block 7b849
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b849 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b849 lea rsi,[rip+0xbd478] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7b850 lea rbx,[rbp-0xd10]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f2f0);
            // 7b857 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7b859 mov rdi,rbx
            rdi = rbx;
            // 7b85c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b861 mov esi,0x160
            rsi = (0x160u64) as u32 as u64;
            // 7b866 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7b86d mov rdi,r14
            rdi = r14;
            // 7b870 mov rdx,rbx
            rdx = rbx;
            // 7b873 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b878 lea rdi,[rbp-0xd10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2f0);
            // 7b87f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b884 mov DWORD PTR [rbp-0xd20],0x160
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f2e0), (0x160u64) as u32);
            // 7b88e mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7b895 mov QWORD PTR [rbp-0xd18],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f2e8), (rax) as u64);
            // 7b89c lea rsi,[rbp-0xd20]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f2e0);
            // 7b8a3 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7b8aa call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b8af lea rsi,[rip+0xae6ed] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7b8b6 lea rdi,[rbp-0xd38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2c8);
            // 7b8bd mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7b8c2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b8c7 movzx ebx,BYTE PTR [rbp-0xd38]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f2c8)) as u64) as u32 as u64;
            // 7b8ce test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7b8d1 jne 0x7b8d8
            if fl.ne() { pc = 0x7b8d8; continue; }
            pc = 0x7b8d3;
        }
        0x7b8d3 => {
            // block 7b8d3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b8d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b8d3 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7b8d6 jmp 0x7b8df
            pc = 0x7b8df; continue;
        }
        0x7b8d8 => {
            // block 7b8d8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b8d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b8d8 mov rbx,QWORD PTR [rbp-0xd30]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f2d0)) as u64;
            pc = 0x7b8df;
        }
        0x7b8df => {
            // block 7b8df
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b8df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b8df lea rdi,[rbp-0xd38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2c8);
            // 7b8e6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b8eb test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7b8ee je 0x7b980
            if fl.e() { pc = 0x7b980; continue; }
            pc = 0x7b8f4;
        }
        0x7b8f4 => {
            // block 7b8f4
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b8f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b8f4 lea rsi,[rip+0xae6a8] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7b8fb lea rdi,[rbp-0xd70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f290);
            // 7b902 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7b907 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b90c mov rax,QWORD PTR [rbp-0xd60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f2a0)) as u64;
            // 7b913 mov QWORD PTR [rbp-0xd48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f2b8), (rax) as u64);
            // 7b91a mov rax,QWORD PTR [rbp-0xd70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f290)) as u64;
            // 7b921 mov rcx,QWORD PTR [rbp-0xd68]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f298)) as u64;
            // 7b928 mov QWORD PTR [rbp-0xd50],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f2b0), (rcx) as u64);
            // 7b92f mov QWORD PTR [rbp-0xd58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f2a8), (rax) as u64);
            // 7b936 xorps xmm0,xmm0
            x0 = 0;
            // 7b939 movaps XMMWORD PTR [rbp-0xd70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f290), x0);
            // 7b940 mov QWORD PTR [rbp-0xd60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f2a0), (0x0u64) as u64);
            // 7b94b mov DWORD PTR [rbp-0xd40],0x160
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f2c0), (0x160u64) as u32);
            // 7b955 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7b95c lea rsi,[rbp-0xd58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f2a8);
            // 7b963 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b968 lea rdi,[rbp-0xd58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2a8);
            // 7b96f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b974 lea rdi,[rbp-0xd70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f290);
            // 7b97b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7b980;
        }
        0x7b980 => {
            // block 7b980
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b980 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b980 xorps xmm0,xmm0
            x0 = 0;
            // 7b983 movaps XMMWORD PTR [rbp-0xd90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f270), x0);
            // 7b98a mov QWORD PTR [rbp-0xd80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f280), (0x0u64) as u64);
            // 7b995 lea rdi,[rbp-0xd90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f270);
            // 7b99c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b9a1 xorps xmm0,xmm0
            x0 = 0;
            // 7b9a4 movaps XMMWORD PTR [rbp-0xdb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f250), x0);
            // 7b9ab mov QWORD PTR [rbp-0xda0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f260), (0x0u64) as u64);
            // 7b9b6 lea rdi,[rbp-0xdb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f250);
            // 7b9bd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b9c2 lea r14,[rip+0x86e77] # 0x102840
            r14 = m.env.data_va(94221);
            // 7b9c9 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7b9cb;
        }
        0x7b9cb => {
            // block 7b9cb
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b9cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b9cb mov rax,rbx
            rax = rbx;
            // 7b9ce and rax,0xfffffffffffffffe
            { let r = rax & 0xffff_ffff_ffff_fffeu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 7b9d2 cmp rax,0x2
            fl = sub::<64>(rax, 0x2u64).1;
            // 7b9d6 je 0x7ba50
            if fl.e() { pc = 0x7ba50; continue; }
            pc = 0x7b9d8;
        }
        0x7b9d8 => {
            // block 7b9d8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7b9d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b9d8 mov DWORD PTR [rbp-0xdb8],0x160
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f248), (0x160u64) as u32);
            // 7b9e2 mov DWORD PTR [rbp-0xdb4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f24c), ((rbx as u32 as u64)) as u32);
            // 7b9e8 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7b9ef lea rsi,[rbp-0xdb8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f248);
            // 7b9f6 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7b9fb cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7b9fe jne 0x7ba2c
            if fl.ne() { pc = 0x7ba2c; continue; }
            pc = 0x7ba00;
        }
        0x7ba00 => {
            // block 7ba00
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ba00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ba00 mov r12,r13
            r12 = r13;
            // 7ba03 mov r13,r15
            r13 = r15;
            // 7ba06 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7ba09 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7ba10 lea rsi,[rbp-0xdb8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f248);
            // 7ba17 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ba1c mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7ba1f mov r15,r13
            r15 = r13;
            // 7ba22 mov r13,r12
            r13 = r12;
            // 7ba25 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7ba2c;
        }
        0x7ba2c => {
            // block 7ba2c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ba2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ba2c mov rdi,r13
            rdi = r13;
            // 7ba2f mov rsi,r14
            rsi = r14;
            // 7ba32 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ba37 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7ba3a jne 0x7ba50
            if fl.ne() { pc = 0x7ba50; continue; }
            pc = 0x7ba3c;
        }
        0x7ba3c => {
            // block 7ba3c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ba3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ba3c mov rdi,r13
            rdi = r13;
            // 7ba3f mov rsi,r14
            rsi = r14;
            // 7ba42 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ba47 mov DWORD PTR [rax],0x160
            m.w32(rax, (0x160u64) as u32);
            // 7ba4d mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7ba50;
        }
        0x7ba50 => {
            // block 7ba50
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ba50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ba50 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7ba53 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7ba57 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7ba5b jne 0x7b9cb
            if fl.ne() { pc = 0x7b9cb; continue; }
            pc = 0x7ba61;
        }
        0x7ba61 => {
            // block 7ba61
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ba61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ba61 lea rsi,[rip+0xbd260] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7ba68 lea rbx,[rbp-0xdd0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f230);
            // 7ba6f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7ba71 mov rdi,rbx
            rdi = rbx;
            // 7ba74 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ba79 mov esi,0x164
            rsi = (0x164u64) as u32 as u64;
            // 7ba7e mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7ba85 mov rdi,r14
            rdi = r14;
            // 7ba88 mov rdx,rbx
            rdx = rbx;
            // 7ba8b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ba90 lea rdi,[rbp-0xdd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f230);
            // 7ba97 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ba9c mov DWORD PTR [rbp-0xde0],0x164
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f220), (0x164u64) as u32);
            // 7baa6 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7baad mov QWORD PTR [rbp-0xdd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f228), (rax) as u64);
            // 7bab4 lea rsi,[rbp-0xde0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f220);
            // 7babb mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7bac2 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bac7 lea rsi,[rip+0xae4d5] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7bace lea rdi,[rbp-0xdf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f208);
            // 7bad5 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7bada call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7badf movzx ebx,BYTE PTR [rbp-0xdf8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f208)) as u64) as u32 as u64;
            // 7bae6 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7bae9 jne 0x7baf0
            if fl.ne() { pc = 0x7baf0; continue; }
            pc = 0x7baeb;
        }
        0x7baeb => {
            // block 7baeb
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7baeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7baeb shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7baee jmp 0x7baf7
            pc = 0x7baf7; continue;
        }
        0x7baf0 => {
            // block 7baf0
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7baf0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7baf0 mov rbx,QWORD PTR [rbp-0xdf0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f210)) as u64;
            pc = 0x7baf7;
        }
        0x7baf7 => {
            // block 7baf7
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7baf7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7baf7 lea rdi,[rbp-0xdf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f208);
            // 7bafe call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bb03 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7bb06 je 0x7bb98
            if fl.e() { pc = 0x7bb98; continue; }
            pc = 0x7bb0c;
        }
        0x7bb0c => {
            // block 7bb0c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bb0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bb0c lea rsi,[rip+0xae490] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7bb13 lea rdi,[rbp-0xe30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1d0);
            // 7bb1a mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7bb1f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bb24 mov rax,QWORD PTR [rbp-0xe20]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f1e0)) as u64;
            // 7bb2b mov QWORD PTR [rbp-0xe08],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f1f8), (rax) as u64);
            // 7bb32 mov rax,QWORD PTR [rbp-0xe30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f1d0)) as u64;
            // 7bb39 mov rcx,QWORD PTR [rbp-0xe28]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f1d8)) as u64;
            // 7bb40 mov QWORD PTR [rbp-0xe10],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f1f0), (rcx) as u64);
            // 7bb47 mov QWORD PTR [rbp-0xe18],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f1e8), (rax) as u64);
            // 7bb4e xorps xmm0,xmm0
            x0 = 0;
            // 7bb51 movaps XMMWORD PTR [rbp-0xe30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f1d0), x0);
            // 7bb58 mov QWORD PTR [rbp-0xe20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f1e0), (0x0u64) as u64);
            // 7bb63 mov DWORD PTR [rbp-0xe00],0x164
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f200), (0x164u64) as u32);
            // 7bb6d lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7bb74 lea rsi,[rbp-0xe18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1e8);
            // 7bb7b call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bb80 lea rdi,[rbp-0xe18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1e8);
            // 7bb87 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bb8c lea rdi,[rbp-0xe30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1d0);
            // 7bb93 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7bb98;
        }
        0x7bb98 => {
            // block 7bb98
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bb98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bb98 xorps xmm0,xmm0
            x0 = 0;
            // 7bb9b movaps XMMWORD PTR [rbp-0xe50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f1b0), x0);
            // 7bba2 mov QWORD PTR [rbp-0xe40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f1c0), (0x0u64) as u64);
            // 7bbad lea rdi,[rbp-0xe50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1b0);
            // 7bbb4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bbb9 xorps xmm0,xmm0
            x0 = 0;
            // 7bbbc movaps XMMWORD PTR [rbp-0xe70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f190), x0);
            // 7bbc3 mov QWORD PTR [rbp-0xe60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f1a0), (0x0u64) as u64);
            // 7bbce lea rdi,[rbp-0xe70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f190);
            // 7bbd5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bbda lea r14,[rip+0x86c6f] # 0x102850
            r14 = m.env.data_va(94237);
            // 7bbe1 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7bbe3;
        }
        0x7bbe3 => {
            // block 7bbe3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bbe3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bbe3 mov DWORD PTR [rbp-0xe78],0x164
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f188), (0x164u64) as u32);
            // 7bbed mov DWORD PTR [rbp-0xe74],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f18c), ((rbx as u32 as u64)) as u32);
            // 7bbf3 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7bbfa lea rsi,[rbp-0xe78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f188);
            // 7bc01 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bc06 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7bc09 jne 0x7bc37
            if fl.ne() { pc = 0x7bc37; continue; }
            pc = 0x7bc0b;
        }
        0x7bc0b => {
            // block 7bc0b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bc0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bc0b mov r12,r13
            r12 = r13;
            // 7bc0e mov r13,r15
            r13 = r15;
            // 7bc11 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7bc14 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7bc1b lea rsi,[rbp-0xe78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f188);
            // 7bc22 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bc27 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7bc2a mov r15,r13
            r15 = r13;
            // 7bc2d mov r13,r12
            r13 = r12;
            // 7bc30 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7bc37;
        }
        0x7bc37 => {
            // block 7bc37
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bc37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bc37 mov rdi,r13
            rdi = r13;
            // 7bc3a mov rsi,r14
            rsi = r14;
            // 7bc3d call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bc42 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7bc45 jne 0x7bc5b
            if fl.ne() { pc = 0x7bc5b; continue; }
            pc = 0x7bc47;
        }
        0x7bc47 => {
            // block 7bc47
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bc47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bc47 mov rdi,r13
            rdi = r13;
            // 7bc4a mov rsi,r14
            rsi = r14;
            // 7bc4d call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bc52 mov DWORD PTR [rax],0x164
            m.w32(rax, (0x164u64) as u32);
            // 7bc58 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7bc5b;
        }
        0x7bc5b => {
            // block 7bc5b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bc5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bc5b inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7bc5e add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7bc62 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7bc66 jne 0x7bbe3
            if fl.ne() { pc = 0x7bbe3; continue; }
            pc = 0x7bc6c;
        }
        0x7bc6c => {
            // block 7bc6c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bc6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bc6c lea rsi,[rip+0xbd055] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7bc73 lea rbx,[rbp-0xe90]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f170);
            // 7bc7a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7bc7c mov rdi,rbx
            rdi = rbx;
            // 7bc7f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bc84 mov esi,0x165
            rsi = (0x165u64) as u32 as u64;
            // 7bc89 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7bc90 mov rdi,r14
            rdi = r14;
            // 7bc93 mov rdx,rbx
            rdx = rbx;
            // 7bc96 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bc9b lea rdi,[rbp-0xe90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f170);
            // 7bca2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bca7 mov DWORD PTR [rbp-0xea0],0x165
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f160), (0x165u64) as u32);
            // 7bcb1 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7bcb8 mov QWORD PTR [rbp-0xe98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f168), (rax) as u64);
            // 7bcbf lea rsi,[rbp-0xea0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f160);
            // 7bcc6 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7bccd call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bcd2 lea rsi,[rip+0xae2ca] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7bcd9 lea rdi,[rbp-0xeb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f148);
            // 7bce0 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7bce5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bcea movzx ebx,BYTE PTR [rbp-0xeb8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f148)) as u64) as u32 as u64;
            // 7bcf1 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7bcf4 jne 0x7bcfb
            if fl.ne() { pc = 0x7bcfb; continue; }
            pc = 0x7bcf6;
        }
        0x7bcf6 => {
            // block 7bcf6
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bcf6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bcf6 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7bcf9 jmp 0x7bd02
            pc = 0x7bd02; continue;
        }
        0x7bcfb => {
            // block 7bcfb
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bcfb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bcfb mov rbx,QWORD PTR [rbp-0xeb0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f150)) as u64;
            pc = 0x7bd02;
        }
        0x7bd02 => {
            // block 7bd02
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bd02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bd02 lea rdi,[rbp-0xeb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f148);
            // 7bd09 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bd0e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7bd11 je 0x7bda3
            if fl.e() { pc = 0x7bda3; continue; }
            pc = 0x7bd17;
        }
        0x7bd17 => {
            // block 7bd17
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bd17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bd17 lea rsi,[rip+0xae285] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7bd1e lea rdi,[rbp-0xef0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f110);
            // 7bd25 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7bd2a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bd2f mov rax,QWORD PTR [rbp-0xee0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f120)) as u64;
            // 7bd36 mov QWORD PTR [rbp-0xec8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f138), (rax) as u64);
            // 7bd3d mov rax,QWORD PTR [rbp-0xef0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f110)) as u64;
            // 7bd44 mov rcx,QWORD PTR [rbp-0xee8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f118)) as u64;
            // 7bd4b mov QWORD PTR [rbp-0xed0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f130), (rcx) as u64);
            // 7bd52 mov QWORD PTR [rbp-0xed8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f128), (rax) as u64);
            // 7bd59 xorps xmm0,xmm0
            x0 = 0;
            // 7bd5c movaps XMMWORD PTR [rbp-0xef0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f110), x0);
            // 7bd63 mov QWORD PTR [rbp-0xee0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f120), (0x0u64) as u64);
            // 7bd6e mov DWORD PTR [rbp-0xec0],0x165
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f140), (0x165u64) as u32);
            // 7bd78 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7bd7f lea rsi,[rbp-0xed8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f128);
            // 7bd86 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bd8b lea rdi,[rbp-0xed8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f128);
            // 7bd92 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bd97 lea rdi,[rbp-0xef0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f110);
            // 7bd9e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7bda3;
        }
        0x7bda3 => {
            // block 7bda3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bda3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bda3 xorps xmm0,xmm0
            x0 = 0;
            // 7bda6 movaps XMMWORD PTR [rbp-0xf10],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f0f0), x0);
            // 7bdad mov QWORD PTR [rbp-0xf00],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f100), (0x0u64) as u64);
            // 7bdb8 lea rdi,[rbp-0xf10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0f0);
            // 7bdbf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bdc4 xorps xmm0,xmm0
            x0 = 0;
            // 7bdc7 movaps XMMWORD PTR [rbp-0xf30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f0d0), x0);
            // 7bdce mov QWORD PTR [rbp-0xf20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f0e0), (0x0u64) as u64);
            // 7bdd9 lea rdi,[rbp-0xf30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0d0);
            // 7bde0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bde5 lea r14,[rip+0x86a74] # 0x102860
            r14 = m.env.data_va(94253);
            // 7bdec xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7bdee;
        }
        0x7bdee => {
            // block 7bdee
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bdee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bdee test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 7bdf0 je 0x7be6a
            if fl.e() { pc = 0x7be6a; continue; }
            pc = 0x7bdf2;
        }
        0x7bdf2 => {
            // block 7bdf2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bdf2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bdf2 mov DWORD PTR [rbp-0xf38],0x165
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0c8), (0x165u64) as u32);
            // 7bdfc mov DWORD PTR [rbp-0xf34],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0cc), ((rbx as u32 as u64)) as u32);
            // 7be02 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7be09 lea rsi,[rbp-0xf38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0c8);
            // 7be10 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7be15 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7be18 jne 0x7be46
            if fl.ne() { pc = 0x7be46; continue; }
            pc = 0x7be1a;
        }
        0x7be1a => {
            // block 7be1a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7be1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7be1a mov r12,r13
            r12 = r13;
            // 7be1d mov r13,r15
            r13 = r15;
            // 7be20 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7be23 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7be2a lea rsi,[rbp-0xf38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0c8);
            // 7be31 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7be36 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7be39 mov r15,r13
            r15 = r13;
            // 7be3c mov r13,r12
            r13 = r12;
            // 7be3f mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7be46;
        }
        0x7be46 => {
            // block 7be46
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7be46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7be46 mov rdi,r13
            rdi = r13;
            // 7be49 mov rsi,r14
            rsi = r14;
            // 7be4c call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7be51 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7be54 jne 0x7be6a
            if fl.ne() { pc = 0x7be6a; continue; }
            pc = 0x7be56;
        }
        0x7be56 => {
            // block 7be56
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7be56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7be56 mov rdi,r13
            rdi = r13;
            // 7be59 mov rsi,r14
            rsi = r14;
            // 7be5c call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7be61 mov DWORD PTR [rax],0x165
            m.w32(rax, (0x165u64) as u32);
            // 7be67 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7be6a;
        }
        0x7be6a => {
            // block 7be6a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7be6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7be6a inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7be6d add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7be71 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7be75 jne 0x7bdee
            if fl.ne() { pc = 0x7bdee; continue; }
            pc = 0x7be7b;
        }
        0x7be7b => {
            // block 7be7b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7be7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7be7b lea rsi,[rip+0xbce46] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7be82 lea rbx,[rbp-0xf50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f0b0);
            // 7be89 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7be8b mov rdi,rbx
            rdi = rbx;
            // 7be8e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7be93 mov esi,0x190
            rsi = (0x190u64) as u32 as u64;
            // 7be98 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7be9f mov rdi,r14
            rdi = r14;
            // 7bea2 mov rdx,rbx
            rdx = rbx;
            // 7bea5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7beaa lea rdi,[rbp-0xf50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0b0);
            // 7beb1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7beb6 mov DWORD PTR [rbp-0xf60],0x190
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0a0), (0x190u64) as u32);
            // 7bec0 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7bec7 mov QWORD PTR [rbp-0xf58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f0a8), (rax) as u64);
            // 7bece lea rsi,[rbp-0xf60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0a0);
            // 7bed5 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7bedc call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bee1 lea rsi,[rip+0xae0bb] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7bee8 lea rdi,[rbp-0xf78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f088);
            // 7beef mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7bef4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bef9 movzx ebx,BYTE PTR [rbp-0xf78]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f088)) as u64) as u32 as u64;
            // 7bf00 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7bf03 jne 0x7bf0a
            if fl.ne() { pc = 0x7bf0a; continue; }
            pc = 0x7bf05;
        }
        0x7bf05 => {
            // block 7bf05
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bf05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bf05 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7bf08 jmp 0x7bf11
            pc = 0x7bf11; continue;
        }
        0x7bf0a => {
            // block 7bf0a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bf0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bf0a mov rbx,QWORD PTR [rbp-0xf70]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f090)) as u64;
            pc = 0x7bf11;
        }
        0x7bf11 => {
            // block 7bf11
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bf11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bf11 lea rdi,[rbp-0xf78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f088);
            // 7bf18 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bf1d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7bf20 je 0x7bfb2
            if fl.e() { pc = 0x7bfb2; continue; }
            pc = 0x7bf26;
        }
        0x7bf26 => {
            // block 7bf26
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bf26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bf26 lea rsi,[rip+0xae076] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7bf2d lea rdi,[rbp-0xfb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f050);
            // 7bf34 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7bf39 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bf3e mov rax,QWORD PTR [rbp-0xfa0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f060)) as u64;
            // 7bf45 mov QWORD PTR [rbp-0xf88],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f078), (rax) as u64);
            // 7bf4c mov rax,QWORD PTR [rbp-0xfb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f050)) as u64;
            // 7bf53 mov rcx,QWORD PTR [rbp-0xfa8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f058)) as u64;
            // 7bf5a mov QWORD PTR [rbp-0xf90],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f070), (rcx) as u64);
            // 7bf61 mov QWORD PTR [rbp-0xf98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f068), (rax) as u64);
            // 7bf68 xorps xmm0,xmm0
            x0 = 0;
            // 7bf6b movaps XMMWORD PTR [rbp-0xfb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f050), x0);
            // 7bf72 mov QWORD PTR [rbp-0xfa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f060), (0x0u64) as u64);
            // 7bf7d mov DWORD PTR [rbp-0xf80],0x190
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f080), (0x190u64) as u32);
            // 7bf87 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7bf8e lea rsi,[rbp-0xf98]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f068);
            // 7bf95 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bf9a lea rdi,[rbp-0xf98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f068);
            // 7bfa1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bfa6 lea rdi,[rbp-0xfb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f050);
            // 7bfad call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7bfb2;
        }
        0x7bfb2 => {
            // block 7bfb2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bfb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bfb2 xorps xmm0,xmm0
            x0 = 0;
            // 7bfb5 movaps XMMWORD PTR [rbp-0xfd0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f030), x0);
            // 7bfbc mov QWORD PTR [rbp-0xfc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f040), (0x0u64) as u64);
            // 7bfc7 lea rdi,[rbp-0xfd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f030);
            // 7bfce call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bfd3 xorps xmm0,xmm0
            x0 = 0;
            // 7bfd6 movaps XMMWORD PTR [rbp-0xff0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f010), x0);
            // 7bfdd mov QWORD PTR [rbp-0xfe0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f020), (0x0u64) as u64);
            // 7bfe8 lea rdi,[rbp-0xff0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f010);
            // 7bfef call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bff4 lea r14,[rip+0x86875] # 0x102870
            r14 = m.env.data_va(94269);
            // 7bffb xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7bffd;
        }
        0x7bffd => {
            // block 7bffd
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7bffd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bffd mov DWORD PTR [rbp-0xff8],0x190
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f008), (0x190u64) as u32);
            // 7c007 mov DWORD PTR [rbp-0xff4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f00c), ((rbx as u32 as u64)) as u32);
            // 7c00d mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7c014 lea rsi,[rbp-0xff8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f008);
            // 7c01b call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c020 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7c023 jne 0x7c051
            if fl.ne() { pc = 0x7c051; continue; }
            pc = 0x7c025;
        }
        0x7c025 => {
            // block 7c025
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c025 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c025 mov r12,r13
            r12 = r13;
            // 7c028 mov r13,r15
            r13 = r15;
            // 7c02b mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7c02e mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7c035 lea rsi,[rbp-0xff8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f008);
            // 7c03c call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c041 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7c044 mov r15,r13
            r15 = r13;
            // 7c047 mov r13,r12
            r13 = r12;
            // 7c04a mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7c051;
        }
        0x7c051 => {
            // block 7c051
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c051 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c051 mov rdi,r13
            rdi = r13;
            // 7c054 mov rsi,r14
            rsi = r14;
            // 7c057 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c05c cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7c05f jne 0x7c075
            if fl.ne() { pc = 0x7c075; continue; }
            pc = 0x7c061;
        }
        0x7c061 => {
            // block 7c061
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c061 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c061 mov rdi,r13
            rdi = r13;
            // 7c064 mov rsi,r14
            rsi = r14;
            // 7c067 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c06c mov DWORD PTR [rax],0x190
            m.w32(rax, (0x190u64) as u32);
            // 7c072 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7c075;
        }
        0x7c075 => {
            // block 7c075
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c075 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c075 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7c078 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7c07c cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7c080 jne 0x7bffd
            if fl.ne() { pc = 0x7bffd; continue; }
            pc = 0x7c086;
        }
        0x7c086 => {
            // block 7c086
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c086 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c086 lea rsi,[rip+0xbcc3b] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7c08d lea rbx,[rbp-0x1010]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eff0);
            // 7c094 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7c096 mov rdi,rbx
            rdi = rbx;
            // 7c099 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c09e mov esi,0x191
            rsi = (0x191u64) as u32 as u64;
            // 7c0a3 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7c0aa mov rdi,r14
            rdi = r14;
            // 7c0ad mov rdx,rbx
            rdx = rbx;
            // 7c0b0 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c0b5 lea rdi,[rbp-0x1010]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eff0);
            // 7c0bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c0c1 mov DWORD PTR [rbp-0x1020],0x191
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_efe0), (0x191u64) as u32);
            // 7c0cb mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7c0d2 mov QWORD PTR [rbp-0x1018],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efe8), (rax) as u64);
            // 7c0d9 lea rsi,[rbp-0x1020]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efe0);
            // 7c0e0 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7c0e7 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c0ec lea rsi,[rip+0xadeb0] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7c0f3 lea rdi,[rbp-0x1038]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efc8);
            // 7c0fa mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7c0ff call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c104 movzx ebx,BYTE PTR [rbp-0x1038]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_efc8)) as u64) as u32 as u64;
            // 7c10b test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7c10e jne 0x7c115
            if fl.ne() { pc = 0x7c115; continue; }
            pc = 0x7c110;
        }
        0x7c110 => {
            // block 7c110
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c110 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c110 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7c113 jmp 0x7c11c
            pc = 0x7c11c; continue;
        }
        0x7c115 => {
            // block 7c115
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c115 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c115 mov rbx,QWORD PTR [rbp-0x1030]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_efd0)) as u64;
            pc = 0x7c11c;
        }
        0x7c11c => {
            // block 7c11c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c11c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c11c lea rdi,[rbp-0x1038]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efc8);
            // 7c123 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c128 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7c12b je 0x7c1bd
            if fl.e() { pc = 0x7c1bd; continue; }
            pc = 0x7c131;
        }
        0x7c131 => {
            // block 7c131
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c131 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c131 lea rsi,[rip+0xade6b] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7c138 lea rdi,[rbp-0x1070]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef90);
            // 7c13f mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7c144 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c149 mov rax,QWORD PTR [rbp-0x1060]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_efa0)) as u64;
            // 7c150 mov QWORD PTR [rbp-0x1048],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efb8), (rax) as u64);
            // 7c157 mov rax,QWORD PTR [rbp-0x1070]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ef90)) as u64;
            // 7c15e mov rcx,QWORD PTR [rbp-0x1068]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ef98)) as u64;
            // 7c165 mov QWORD PTR [rbp-0x1050],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efb0), (rcx) as u64);
            // 7c16c mov QWORD PTR [rbp-0x1058],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efa8), (rax) as u64);
            // 7c173 xorps xmm0,xmm0
            x0 = 0;
            // 7c176 movaps XMMWORD PTR [rbp-0x1070],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ef90), x0);
            // 7c17d mov QWORD PTR [rbp-0x1060],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efa0), (0x0u64) as u64);
            // 7c188 mov DWORD PTR [rbp-0x1040],0x191
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_efc0), (0x191u64) as u32);
            // 7c192 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7c199 lea rsi,[rbp-0x1058]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efa8);
            // 7c1a0 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c1a5 lea rdi,[rbp-0x1058]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efa8);
            // 7c1ac call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c1b1 lea rdi,[rbp-0x1070]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef90);
            // 7c1b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7c1bd;
        }
        0x7c1bd => {
            // block 7c1bd
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c1bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c1bd xorps xmm0,xmm0
            x0 = 0;
            // 7c1c0 movaps XMMWORD PTR [rbp-0x1090],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ef70), x0);
            // 7c1c7 mov QWORD PTR [rbp-0x1080],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef80), (0x0u64) as u64);
            // 7c1d2 lea rdi,[rbp-0x1090]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef70);
            // 7c1d9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c1de xorps xmm0,xmm0
            x0 = 0;
            // 7c1e1 movaps XMMWORD PTR [rbp-0x10b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ef50), x0);
            // 7c1e8 mov QWORD PTR [rbp-0x10a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef60), (0x0u64) as u64);
            // 7c1f3 lea rdi,[rbp-0x10b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef50);
            // 7c1fa call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c1ff lea r14,[rip+0x8667a] # 0x102880
            r14 = m.env.data_va(94285);
            // 7c206 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7c208;
        }
        0x7c208 => {
            // block 7c208
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c208 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c208 mov DWORD PTR [rbp-0x10b8],0x191
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef48), (0x191u64) as u32);
            // 7c212 mov DWORD PTR [rbp-0x10b4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef4c), ((rbx as u32 as u64)) as u32);
            // 7c218 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7c21f lea rsi,[rbp-0x10b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef48);
            // 7c226 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c22b cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7c22e jne 0x7c25c
            if fl.ne() { pc = 0x7c25c; continue; }
            pc = 0x7c230;
        }
        0x7c230 => {
            // block 7c230
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c230 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c230 mov r12,r13
            r12 = r13;
            // 7c233 mov r13,r15
            r13 = r15;
            // 7c236 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7c239 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7c240 lea rsi,[rbp-0x10b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef48);
            // 7c247 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c24c mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7c24f mov r15,r13
            r15 = r13;
            // 7c252 mov r13,r12
            r13 = r12;
            // 7c255 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7c25c;
        }
        0x7c25c => {
            // block 7c25c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c25c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c25c mov rdi,r13
            rdi = r13;
            // 7c25f mov rsi,r14
            rsi = r14;
            // 7c262 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c267 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7c26a jne 0x7c280
            if fl.ne() { pc = 0x7c280; continue; }
            pc = 0x7c26c;
        }
        0x7c26c => {
            // block 7c26c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c26c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c26c mov rdi,r13
            rdi = r13;
            // 7c26f mov rsi,r14
            rsi = r14;
            // 7c272 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c277 mov DWORD PTR [rax],0x191
            m.w32(rax, (0x191u64) as u32);
            // 7c27d mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7c280;
        }
        0x7c280 => {
            // block 7c280
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c280 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c280 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7c283 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7c287 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7c28b jne 0x7c208
            if fl.ne() { pc = 0x7c208; continue; }
            pc = 0x7c291;
        }
        0x7c291 => {
            // block 7c291
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c291 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c291 lea rsi,[rip+0xbca30] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7c298 lea rbx,[rbp-0x10d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ef30);
            // 7c29f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7c2a1 mov rdi,rbx
            rdi = rbx;
            // 7c2a4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c2a9 mov esi,0x193
            rsi = (0x193u64) as u32 as u64;
            // 7c2ae mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7c2b5 mov rdi,r14
            rdi = r14;
            // 7c2b8 mov rdx,rbx
            rdx = rbx;
            // 7c2bb call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c2c0 lea rdi,[rbp-0x10d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef30);
            // 7c2c7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c2cc mov DWORD PTR [rbp-0x10e0],0x193
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef20), (0x193u64) as u32);
            // 7c2d6 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7c2dd mov QWORD PTR [rbp-0x10d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef28), (rax) as u64);
            // 7c2e4 lea rsi,[rbp-0x10e0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef20);
            // 7c2eb mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7c2f2 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c2f7 lea rsi,[rip+0xadca5] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7c2fe lea rdi,[rbp-0x10f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef08);
            // 7c305 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7c30a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c30f movzx ebx,BYTE PTR [rbp-0x10f8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ef08)) as u64) as u32 as u64;
            // 7c316 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7c319 jne 0x7c320
            if fl.ne() { pc = 0x7c320; continue; }
            pc = 0x7c31b;
        }
        0x7c31b => {
            // block 7c31b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c31b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c31b shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7c31e jmp 0x7c327
            pc = 0x7c327; continue;
        }
        0x7c320 => {
            // block 7c320
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c320 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c320 mov rbx,QWORD PTR [rbp-0x10f0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ef10)) as u64;
            pc = 0x7c327;
        }
        0x7c327 => {
            // block 7c327
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c327 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c327 lea rdi,[rbp-0x10f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef08);
            // 7c32e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c333 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7c336 je 0x7c3c8
            if fl.e() { pc = 0x7c3c8; continue; }
            pc = 0x7c33c;
        }
        0x7c33c => {
            // block 7c33c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c33c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c33c lea rsi,[rip+0xadc60] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7c343 lea rdi,[rbp-0x1130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eed0);
            // 7c34a mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7c34f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c354 mov rax,QWORD PTR [rbp-0x1120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eee0)) as u64;
            // 7c35b mov QWORD PTR [rbp-0x1108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eef8), (rax) as u64);
            // 7c362 mov rax,QWORD PTR [rbp-0x1130]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eed0)) as u64;
            // 7c369 mov rcx,QWORD PTR [rbp-0x1128]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eed8)) as u64;
            // 7c370 mov QWORD PTR [rbp-0x1110],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eef0), (rcx) as u64);
            // 7c377 mov QWORD PTR [rbp-0x1118],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eee8), (rax) as u64);
            // 7c37e xorps xmm0,xmm0
            x0 = 0;
            // 7c381 movaps XMMWORD PTR [rbp-0x1130],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_eed0), x0);
            // 7c388 mov QWORD PTR [rbp-0x1120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eee0), (0x0u64) as u64);
            // 7c393 mov DWORD PTR [rbp-0x1100],0x193
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef00), (0x193u64) as u32);
            // 7c39d lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7c3a4 lea rsi,[rbp-0x1118]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eee8);
            // 7c3ab call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c3b0 lea rdi,[rbp-0x1118]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eee8);
            // 7c3b7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c3bc lea rdi,[rbp-0x1130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eed0);
            // 7c3c3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7c3c8;
        }
        0x7c3c8 => {
            // block 7c3c8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c3c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c3c8 xorps xmm0,xmm0
            x0 = 0;
            // 7c3cb movaps XMMWORD PTR [rbp-0x1150],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_eeb0), x0);
            // 7c3d2 mov QWORD PTR [rbp-0x1140],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eec0), (0x0u64) as u64);
            // 7c3dd lea rdi,[rbp-0x1150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eeb0);
            // 7c3e4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c3e9 xorps xmm0,xmm0
            x0 = 0;
            // 7c3ec movaps XMMWORD PTR [rbp-0x1170],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ee90), x0);
            // 7c3f3 mov QWORD PTR [rbp-0x1160],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eea0), (0x0u64) as u64);
            // 7c3fe lea rdi,[rbp-0x1170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee90);
            // 7c405 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c40a lea r14,[rip+0x8647f] # 0x102890
            r14 = m.env.data_va(94301);
            // 7c411 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7c413;
        }
        0x7c413 => {
            // block 7c413
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c413 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c413 mov DWORD PTR [rbp-0x1178],0x193
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee88), (0x193u64) as u32);
            // 7c41d mov DWORD PTR [rbp-0x1174],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee8c), ((rbx as u32 as u64)) as u32);
            // 7c423 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7c42a lea rsi,[rbp-0x1178]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee88);
            // 7c431 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c436 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7c439 jne 0x7c467
            if fl.ne() { pc = 0x7c467; continue; }
            pc = 0x7c43b;
        }
        0x7c43b => {
            // block 7c43b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c43b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c43b mov r12,r13
            r12 = r13;
            // 7c43e mov r13,r15
            r13 = r15;
            // 7c441 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7c444 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7c44b lea rsi,[rbp-0x1178]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee88);
            // 7c452 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c457 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7c45a mov r15,r13
            r15 = r13;
            // 7c45d mov r13,r12
            r13 = r12;
            // 7c460 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7c467;
        }
        0x7c467 => {
            // block 7c467
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c467 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c467 mov rdi,r13
            rdi = r13;
            // 7c46a mov rsi,r14
            rsi = r14;
            // 7c46d call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c472 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7c475 jne 0x7c48b
            if fl.ne() { pc = 0x7c48b; continue; }
            pc = 0x7c477;
        }
        0x7c477 => {
            // block 7c477
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c477 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c477 mov rdi,r13
            rdi = r13;
            // 7c47a mov rsi,r14
            rsi = r14;
            // 7c47d call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c482 mov DWORD PTR [rax],0x193
            m.w32(rax, (0x193u64) as u32);
            // 7c488 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7c48b;
        }
        0x7c48b => {
            // block 7c48b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c48b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c48b inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7c48e add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7c492 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7c496 jne 0x7c413
            if fl.ne() { pc = 0x7c413; continue; }
            pc = 0x7c49c;
        }
        0x7c49c => {
            // block 7c49c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c49c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c49c lea rsi,[rip+0xbc825] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7c4a3 lea rbx,[rbp-0x1190]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ee70);
            // 7c4aa xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7c4ac mov rdi,rbx
            rdi = rbx;
            // 7c4af call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c4b4 mov esi,0x194
            rsi = (0x194u64) as u32 as u64;
            // 7c4b9 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7c4c0 mov rdi,r14
            rdi = r14;
            // 7c4c3 mov rdx,rbx
            rdx = rbx;
            // 7c4c6 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c4cb lea rdi,[rbp-0x1190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee70);
            // 7c4d2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c4d7 mov DWORD PTR [rbp-0x11a0],0x194
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee60), (0x194u64) as u32);
            // 7c4e1 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7c4e8 mov QWORD PTR [rbp-0x1198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee68), (rax) as u64);
            // 7c4ef lea rsi,[rbp-0x11a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee60);
            // 7c4f6 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7c4fd call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c502 lea rsi,[rip+0xada9a] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7c509 lea rdi,[rbp-0x11b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee48);
            // 7c510 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7c515 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c51a movzx ebx,BYTE PTR [rbp-0x11b8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ee48)) as u64) as u32 as u64;
            // 7c521 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7c524 jne 0x7c52b
            if fl.ne() { pc = 0x7c52b; continue; }
            pc = 0x7c526;
        }
        0x7c526 => {
            // block 7c526
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c526 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c526 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7c529 jmp 0x7c532
            pc = 0x7c532; continue;
        }
        0x7c52b => {
            // block 7c52b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c52b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c52b mov rbx,QWORD PTR [rbp-0x11b0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ee50)) as u64;
            pc = 0x7c532;
        }
        0x7c532 => {
            // block 7c532
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c532 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c532 lea rdi,[rbp-0x11b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee48);
            // 7c539 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c53e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7c541 je 0x7c5d3
            if fl.e() { pc = 0x7c5d3; continue; }
            pc = 0x7c547;
        }
        0x7c547 => {
            // block 7c547
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c547 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c547 lea rsi,[rip+0xada55] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7c54e lea rdi,[rbp-0x11f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee10);
            // 7c555 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7c55a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c55f mov rax,QWORD PTR [rbp-0x11e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ee20)) as u64;
            // 7c566 mov QWORD PTR [rbp-0x11c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee38), (rax) as u64);
            // 7c56d mov rax,QWORD PTR [rbp-0x11f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ee10)) as u64;
            // 7c574 mov rcx,QWORD PTR [rbp-0x11e8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ee18)) as u64;
            // 7c57b mov QWORD PTR [rbp-0x11d0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee30), (rcx) as u64);
            // 7c582 mov QWORD PTR [rbp-0x11d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee28), (rax) as u64);
            // 7c589 xorps xmm0,xmm0
            x0 = 0;
            // 7c58c movaps XMMWORD PTR [rbp-0x11f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ee10), x0);
            // 7c593 mov QWORD PTR [rbp-0x11e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee20), (0x0u64) as u64);
            // 7c59e mov DWORD PTR [rbp-0x11c0],0x194
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee40), (0x194u64) as u32);
            // 7c5a8 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7c5af lea rsi,[rbp-0x11d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee28);
            // 7c5b6 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c5bb lea rdi,[rbp-0x11d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee28);
            // 7c5c2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c5c7 lea rdi,[rbp-0x11f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee10);
            // 7c5ce call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7c5d3;
        }
        0x7c5d3 => {
            // block 7c5d3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c5d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c5d3 xorps xmm0,xmm0
            x0 = 0;
            // 7c5d6 movaps XMMWORD PTR [rbp-0x1210],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_edf0), x0);
            // 7c5dd mov QWORD PTR [rbp-0x1200],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ee00), (0x0u64) as u64);
            // 7c5e8 lea rdi,[rbp-0x1210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edf0);
            // 7c5ef call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c5f4 xorps xmm0,xmm0
            x0 = 0;
            // 7c5f7 movaps XMMWORD PTR [rbp-0x1230],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_edd0), x0);
            // 7c5fe mov QWORD PTR [rbp-0x1220],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ede0), (0x0u64) as u64);
            // 7c609 lea rdi,[rbp-0x1230]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edd0);
            // 7c610 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c615 lea r14,[rip+0x86284] # 0x1028a0
            r14 = m.env.data_va(94317);
            // 7c61c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7c61e;
        }
        0x7c61e => {
            // block 7c61e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c61e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c61e cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 7c621 je 0x7c69b
            if fl.e() { pc = 0x7c69b; continue; }
            pc = 0x7c623;
        }
        0x7c623 => {
            // block 7c623
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c623 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c623 mov DWORD PTR [rbp-0x1238],0x194
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edc8), (0x194u64) as u32);
            // 7c62d mov DWORD PTR [rbp-0x1234],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edcc), ((rbx as u32 as u64)) as u32);
            // 7c633 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7c63a lea rsi,[rbp-0x1238]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edc8);
            // 7c641 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c646 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7c649 jne 0x7c677
            if fl.ne() { pc = 0x7c677; continue; }
            pc = 0x7c64b;
        }
        0x7c64b => {
            // block 7c64b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c64b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c64b mov r12,r13
            r12 = r13;
            // 7c64e mov r13,r15
            r13 = r15;
            // 7c651 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7c654 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7c65b lea rsi,[rbp-0x1238]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edc8);
            // 7c662 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c667 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7c66a mov r15,r13
            r15 = r13;
            // 7c66d mov r13,r12
            r13 = r12;
            // 7c670 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7c677;
        }
        0x7c677 => {
            // block 7c677
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c677 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c677 mov rdi,r13
            rdi = r13;
            // 7c67a mov rsi,r14
            rsi = r14;
            // 7c67d call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c682 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7c685 jne 0x7c69b
            if fl.ne() { pc = 0x7c69b; continue; }
            pc = 0x7c687;
        }
        0x7c687 => {
            // block 7c687
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c687 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c687 mov rdi,r13
            rdi = r13;
            // 7c68a mov rsi,r14
            rsi = r14;
            // 7c68d call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c692 mov DWORD PTR [rax],0x194
            m.w32(rax, (0x194u64) as u32);
            // 7c698 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7c69b;
        }
        0x7c69b => {
            // block 7c69b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c69b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c69b inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7c69e add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7c6a2 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7c6a6 jne 0x7c61e
            if fl.ne() { pc = 0x7c61e; continue; }
            pc = 0x7c6ac;
        }
        0x7c6ac => {
            // block 7c6ac
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c6ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c6ac lea rsi,[rip+0xbc615] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7c6b3 lea rbx,[rbp-0x1250]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_edb0);
            // 7c6ba xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7c6bc mov rdi,rbx
            rdi = rbx;
            // 7c6bf call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c6c4 mov esi,0x195
            rsi = (0x195u64) as u32 as u64;
            // 7c6c9 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7c6d0 mov rdi,r14
            rdi = r14;
            // 7c6d3 mov rdx,rbx
            rdx = rbx;
            // 7c6d6 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c6db lea rdi,[rbp-0x1250]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edb0);
            // 7c6e2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c6e7 mov DWORD PTR [rbp-0x1260],0x195
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eda0), (0x195u64) as u32);
            // 7c6f1 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7c6f8 mov QWORD PTR [rbp-0x1258],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eda8), (rax) as u64);
            // 7c6ff lea rsi,[rbp-0x1260]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eda0);
            // 7c706 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7c70d call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c712 lea rsi,[rip+0xad88a] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7c719 lea rdi,[rbp-0x1278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed88);
            // 7c720 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7c725 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c72a movzx ebx,BYTE PTR [rbp-0x1278]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ed88)) as u64) as u32 as u64;
            // 7c731 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7c734 jne 0x7c73b
            if fl.ne() { pc = 0x7c73b; continue; }
            pc = 0x7c736;
        }
        0x7c736 => {
            // block 7c736
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c736 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c736 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7c739 jmp 0x7c742
            pc = 0x7c742; continue;
        }
        0x7c73b => {
            // block 7c73b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c73b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c73b mov rbx,QWORD PTR [rbp-0x1270]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ed90)) as u64;
            pc = 0x7c742;
        }
        0x7c742 => {
            // block 7c742
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c742 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c742 lea rdi,[rbp-0x1278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed88);
            // 7c749 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c74e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7c751 je 0x7c7e3
            if fl.e() { pc = 0x7c7e3; continue; }
            pc = 0x7c757;
        }
        0x7c757 => {
            // block 7c757
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c757 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c757 lea rsi,[rip+0xad845] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7c75e lea rdi,[rbp-0x12b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed50);
            // 7c765 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7c76a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c76f mov rax,QWORD PTR [rbp-0x12a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ed60)) as u64;
            // 7c776 mov QWORD PTR [rbp-0x1288],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed78), (rax) as u64);
            // 7c77d mov rax,QWORD PTR [rbp-0x12b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ed50)) as u64;
            // 7c784 mov rcx,QWORD PTR [rbp-0x12a8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ed58)) as u64;
            // 7c78b mov QWORD PTR [rbp-0x1290],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed70), (rcx) as u64);
            // 7c792 mov QWORD PTR [rbp-0x1298],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed68), (rax) as u64);
            // 7c799 xorps xmm0,xmm0
            x0 = 0;
            // 7c79c movaps XMMWORD PTR [rbp-0x12b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ed50), x0);
            // 7c7a3 mov QWORD PTR [rbp-0x12a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed60), (0x0u64) as u64);
            // 7c7ae mov DWORD PTR [rbp-0x1280],0x195
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed80), (0x195u64) as u32);
            // 7c7b8 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7c7bf lea rsi,[rbp-0x1298]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed68);
            // 7c7c6 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c7cb lea rdi,[rbp-0x1298]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed68);
            // 7c7d2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c7d7 lea rdi,[rbp-0x12b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed50);
            // 7c7de call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7c7e3;
        }
        0x7c7e3 => {
            // block 7c7e3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c7e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c7e3 xorps xmm0,xmm0
            x0 = 0;
            // 7c7e6 movaps XMMWORD PTR [rbp-0x12d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ed30), x0);
            // 7c7ed mov QWORD PTR [rbp-0x12c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed40), (0x0u64) as u64);
            // 7c7f8 lea rdi,[rbp-0x12d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed30);
            // 7c7ff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c804 xorps xmm0,xmm0
            x0 = 0;
            // 7c807 movaps XMMWORD PTR [rbp-0x12f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ed10), x0);
            // 7c80e mov QWORD PTR [rbp-0x12e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ed20), (0x0u64) as u64);
            // 7c819 lea rdi,[rbp-0x12f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed10);
            // 7c820 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c825 lea r14,[rip+0x86084] # 0x1028b0
            r14 = m.env.data_va(94333);
            // 7c82c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7c82e;
        }
        0x7c82e => {
            // block 7c82e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c82e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c82e mov DWORD PTR [rbp-0x12f8],0x195
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed08), (0x195u64) as u32);
            // 7c838 mov DWORD PTR [rbp-0x12f4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed0c), ((rbx as u32 as u64)) as u32);
            // 7c83e mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7c845 lea rsi,[rbp-0x12f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed08);
            // 7c84c call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c851 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7c854 jne 0x7c882
            if fl.ne() { pc = 0x7c882; continue; }
            pc = 0x7c856;
        }
        0x7c856 => {
            // block 7c856
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c856 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c856 mov r12,r13
            r12 = r13;
            // 7c859 mov r13,r15
            r13 = r15;
            // 7c85c mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7c85f mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7c866 lea rsi,[rbp-0x12f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed08);
            // 7c86d call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c872 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7c875 mov r15,r13
            r15 = r13;
            // 7c878 mov r13,r12
            r13 = r12;
            // 7c87b mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7c882;
        }
        0x7c882 => {
            // block 7c882
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c882 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c882 mov rdi,r13
            rdi = r13;
            // 7c885 mov rsi,r14
            rsi = r14;
            // 7c888 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c88d cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7c890 jne 0x7c8a6
            if fl.ne() { pc = 0x7c8a6; continue; }
            pc = 0x7c892;
        }
        0x7c892 => {
            // block 7c892
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c892 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c892 mov rdi,r13
            rdi = r13;
            // 7c895 mov rsi,r14
            rsi = r14;
            // 7c898 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c89d mov DWORD PTR [rax],0x195
            m.w32(rax, (0x195u64) as u32);
            // 7c8a3 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7c8a6;
        }
        0x7c8a6 => {
            // block 7c8a6
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c8a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c8a6 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7c8a9 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7c8ad cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7c8b1 jne 0x7c82e
            if fl.ne() { pc = 0x7c82e; continue; }
            pc = 0x7c8b7;
        }
        0x7c8b7 => {
            // block 7c8b7
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c8b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c8b7 lea rsi,[rip+0xbc40a] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7c8be lea rbx,[rbp-0x1310]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ecf0);
            // 7c8c5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7c8c7 mov rdi,rbx
            rdi = rbx;
            // 7c8ca call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c8cf mov esi,0x196
            rsi = (0x196u64) as u32 as u64;
            // 7c8d4 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7c8db mov rdi,r14
            rdi = r14;
            // 7c8de mov rdx,rbx
            rdx = rbx;
            // 7c8e1 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c8e6 lea rdi,[rbp-0x1310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecf0);
            // 7c8ed call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c8f2 mov DWORD PTR [rbp-0x1320],0x196
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ece0), (0x196u64) as u32);
            // 7c8fc mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7c903 mov QWORD PTR [rbp-0x1318],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ece8), (rax) as u64);
            // 7c90a lea rsi,[rbp-0x1320]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ece0);
            // 7c911 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7c918 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c91d lea rsi,[rip+0xad67f] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7c924 lea rdi,[rbp-0x1338]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecc8);
            // 7c92b mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7c930 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c935 movzx ebx,BYTE PTR [rbp-0x1338]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ecc8)) as u64) as u32 as u64;
            // 7c93c test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7c93f jne 0x7c946
            if fl.ne() { pc = 0x7c946; continue; }
            pc = 0x7c941;
        }
        0x7c941 => {
            // block 7c941
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c941 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c941 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7c944 jmp 0x7c94d
            pc = 0x7c94d; continue;
        }
        0x7c946 => {
            // block 7c946
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c946 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c946 mov rbx,QWORD PTR [rbp-0x1330]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ecd0)) as u64;
            pc = 0x7c94d;
        }
        0x7c94d => {
            // block 7c94d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c94d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c94d lea rdi,[rbp-0x1338]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecc8);
            // 7c954 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c959 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7c95c je 0x7c9ee
            if fl.e() { pc = 0x7c9ee; continue; }
            pc = 0x7c962;
        }
        0x7c962 => {
            // block 7c962
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c962 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c962 lea rsi,[rip+0xad63a] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7c969 lea rdi,[rbp-0x1370]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec90);
            // 7c970 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7c975 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c97a mov rax,QWORD PTR [rbp-0x1360]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eca0)) as u64;
            // 7c981 mov QWORD PTR [rbp-0x1348],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ecb8), (rax) as u64);
            // 7c988 mov rax,QWORD PTR [rbp-0x1370]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ec90)) as u64;
            // 7c98f mov rcx,QWORD PTR [rbp-0x1368]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ec98)) as u64;
            // 7c996 mov QWORD PTR [rbp-0x1350],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ecb0), (rcx) as u64);
            // 7c99d mov QWORD PTR [rbp-0x1358],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eca8), (rax) as u64);
            // 7c9a4 xorps xmm0,xmm0
            x0 = 0;
            // 7c9a7 movaps XMMWORD PTR [rbp-0x1370],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ec90), x0);
            // 7c9ae mov QWORD PTR [rbp-0x1360],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eca0), (0x0u64) as u64);
            // 7c9b9 mov DWORD PTR [rbp-0x1340],0x196
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ecc0), (0x196u64) as u32);
            // 7c9c3 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7c9ca lea rsi,[rbp-0x1358]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eca8);
            // 7c9d1 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c9d6 lea rdi,[rbp-0x1358]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eca8);
            // 7c9dd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7c9e2 lea rdi,[rbp-0x1370]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec90);
            // 7c9e9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7c9ee;
        }
        0x7c9ee => {
            // block 7c9ee
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7c9ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c9ee xorps xmm0,xmm0
            x0 = 0;
            // 7c9f1 movaps XMMWORD PTR [rbp-0x1390],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ec70), x0);
            // 7c9f8 mov QWORD PTR [rbp-0x1380],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ec80), (0x0u64) as u64);
            // 7ca03 lea rdi,[rbp-0x1390]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec70);
            // 7ca0a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ca0f xorps xmm0,xmm0
            x0 = 0;
            // 7ca12 movaps XMMWORD PTR [rbp-0x13b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ec50), x0);
            // 7ca19 mov QWORD PTR [rbp-0x13a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ec60), (0x0u64) as u64);
            // 7ca24 lea rdi,[rbp-0x13b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec50);
            // 7ca2b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ca30 lea r14,[rip+0x85e89] # 0x1028c0
            r14 = m.env.data_va(94349);
            // 7ca37 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7ca39;
        }
        0x7ca39 => {
            // block 7ca39
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ca39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ca39 mov DWORD PTR [rbp-0x13b8],0x196
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ec48), (0x196u64) as u32);
            // 7ca43 mov DWORD PTR [rbp-0x13b4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ec4c), ((rbx as u32 as u64)) as u32);
            // 7ca49 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7ca50 lea rsi,[rbp-0x13b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ec48);
            // 7ca57 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ca5c cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7ca5f jne 0x7ca8d
            if fl.ne() { pc = 0x7ca8d; continue; }
            pc = 0x7ca61;
        }
        0x7ca61 => {
            // block 7ca61
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ca61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ca61 mov r12,r13
            r12 = r13;
            // 7ca64 mov r13,r15
            r13 = r15;
            // 7ca67 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7ca6a mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7ca71 lea rsi,[rbp-0x13b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ec48);
            // 7ca78 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ca7d mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7ca80 mov r15,r13
            r15 = r13;
            // 7ca83 mov r13,r12
            r13 = r12;
            // 7ca86 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7ca8d;
        }
        0x7ca8d => {
            // block 7ca8d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ca8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ca8d mov rdi,r13
            rdi = r13;
            // 7ca90 mov rsi,r14
            rsi = r14;
            // 7ca93 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ca98 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7ca9b jne 0x7cab1
            if fl.ne() { pc = 0x7cab1; continue; }
            pc = 0x7ca9d;
        }
        0x7ca9d => {
            // block 7ca9d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ca9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ca9d mov rdi,r13
            rdi = r13;
            // 7caa0 mov rsi,r14
            rsi = r14;
            // 7caa3 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7caa8 mov DWORD PTR [rax],0x196
            m.w32(rax, (0x196u64) as u32);
            // 7caae mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7cab1;
        }
        0x7cab1 => {
            // block 7cab1
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cab1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cab1 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7cab4 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7cab8 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7cabc jne 0x7ca39
            if fl.ne() { pc = 0x7ca39; continue; }
            pc = 0x7cac2;
        }
        0x7cac2 => {
            // block 7cac2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cac2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cac2 lea rsi,[rip+0xbc1ff] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7cac9 lea rbx,[rbp-0x13d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ec30);
            // 7cad0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7cad2 mov rdi,rbx
            rdi = rbx;
            // 7cad5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cada mov esi,0x197
            rsi = (0x197u64) as u32 as u64;
            // 7cadf mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7cae6 mov rdi,r14
            rdi = r14;
            // 7cae9 mov rdx,rbx
            rdx = rbx;
            // 7caec call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7caf1 lea rdi,[rbp-0x13d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec30);
            // 7caf8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cafd mov DWORD PTR [rbp-0x13e0],0x197
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ec20), (0x197u64) as u32);
            // 7cb07 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7cb0e mov QWORD PTR [rbp-0x13d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ec28), (rax) as u64);
            // 7cb15 lea rsi,[rbp-0x13e0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ec20);
            // 7cb1c mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7cb23 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cb28 lea rsi,[rip+0xad474] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7cb2f lea rdi,[rbp-0x13f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec08);
            // 7cb36 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7cb3b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cb40 movzx ebx,BYTE PTR [rbp-0x13f8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ec08)) as u64) as u32 as u64;
            // 7cb47 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7cb4a jne 0x7cb51
            if fl.ne() { pc = 0x7cb51; continue; }
            pc = 0x7cb4c;
        }
        0x7cb4c => {
            // block 7cb4c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cb4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cb4c shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7cb4f jmp 0x7cb58
            pc = 0x7cb58; continue;
        }
        0x7cb51 => {
            // block 7cb51
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cb51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cb51 mov rbx,QWORD PTR [rbp-0x13f0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ec10)) as u64;
            pc = 0x7cb58;
        }
        0x7cb58 => {
            // block 7cb58
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cb58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cb58 lea rdi,[rbp-0x13f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec08);
            // 7cb5f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cb64 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7cb67 je 0x7cbf9
            if fl.e() { pc = 0x7cbf9; continue; }
            pc = 0x7cb6d;
        }
        0x7cb6d => {
            // block 7cb6d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cb6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cb6d lea rsi,[rip+0xad42f] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7cb74 lea rdi,[rbp-0x1430]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebd0);
            // 7cb7b mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7cb80 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cb85 mov rax,QWORD PTR [rbp-0x1420]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ebe0)) as u64;
            // 7cb8c mov QWORD PTR [rbp-0x1408],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ebf8), (rax) as u64);
            // 7cb93 mov rax,QWORD PTR [rbp-0x1430]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ebd0)) as u64;
            // 7cb9a mov rcx,QWORD PTR [rbp-0x1428]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ebd8)) as u64;
            // 7cba1 mov QWORD PTR [rbp-0x1410],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ebf0), (rcx) as u64);
            // 7cba8 mov QWORD PTR [rbp-0x1418],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ebe8), (rax) as u64);
            // 7cbaf xorps xmm0,xmm0
            x0 = 0;
            // 7cbb2 movaps XMMWORD PTR [rbp-0x1430],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ebd0), x0);
            // 7cbb9 mov QWORD PTR [rbp-0x1420],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ebe0), (0x0u64) as u64);
            // 7cbc4 mov DWORD PTR [rbp-0x1400],0x197
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ec00), (0x197u64) as u32);
            // 7cbce lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7cbd5 lea rsi,[rbp-0x1418]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ebe8);
            // 7cbdc call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cbe1 lea rdi,[rbp-0x1418]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebe8);
            // 7cbe8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cbed lea rdi,[rbp-0x1430]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebd0);
            // 7cbf4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7cbf9;
        }
        0x7cbf9 => {
            // block 7cbf9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cbf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cbf9 xorps xmm0,xmm0
            x0 = 0;
            // 7cbfc movaps XMMWORD PTR [rbp-0x1450],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ebb0), x0);
            // 7cc03 mov QWORD PTR [rbp-0x1440],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ebc0), (0x0u64) as u64);
            // 7cc0e lea rdi,[rbp-0x1450]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebb0);
            // 7cc15 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cc1a xorps xmm0,xmm0
            x0 = 0;
            // 7cc1d movaps XMMWORD PTR [rbp-0x1470],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_eb90), x0);
            // 7cc24 mov QWORD PTR [rbp-0x1460],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eba0), (0x0u64) as u64);
            // 7cc2f lea rdi,[rbp-0x1470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb90);
            // 7cc36 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cc3b lea r14,[rip+0x85c8e] # 0x1028d0
            r14 = m.env.data_va(94365);
            // 7cc42 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7cc44;
        }
        0x7cc44 => {
            // block 7cc44
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cc44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cc44 cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 7cc47 je 0x7ccc1
            if fl.e() { pc = 0x7ccc1; continue; }
            pc = 0x7cc49;
        }
        0x7cc49 => {
            // block 7cc49
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cc49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cc49 mov DWORD PTR [rbp-0x1478],0x197
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eb88), (0x197u64) as u32);
            // 7cc53 mov DWORD PTR [rbp-0x1474],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eb8c), ((rbx as u32 as u64)) as u32);
            // 7cc59 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7cc60 lea rsi,[rbp-0x1478]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eb88);
            // 7cc67 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cc6c cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7cc6f jne 0x7cc9d
            if fl.ne() { pc = 0x7cc9d; continue; }
            pc = 0x7cc71;
        }
        0x7cc71 => {
            // block 7cc71
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cc71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cc71 mov r12,r13
            r12 = r13;
            // 7cc74 mov r13,r15
            r13 = r15;
            // 7cc77 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7cc7a mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7cc81 lea rsi,[rbp-0x1478]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eb88);
            // 7cc88 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cc8d mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7cc90 mov r15,r13
            r15 = r13;
            // 7cc93 mov r13,r12
            r13 = r12;
            // 7cc96 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7cc9d;
        }
        0x7cc9d => {
            // block 7cc9d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cc9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cc9d mov rdi,r13
            rdi = r13;
            // 7cca0 mov rsi,r14
            rsi = r14;
            // 7cca3 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cca8 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7ccab jne 0x7ccc1
            if fl.ne() { pc = 0x7ccc1; continue; }
            pc = 0x7ccad;
        }
        0x7ccad => {
            // block 7ccad
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ccad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ccad mov rdi,r13
            rdi = r13;
            // 7ccb0 mov rsi,r14
            rsi = r14;
            // 7ccb3 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ccb8 mov DWORD PTR [rax],0x197
            m.w32(rax, (0x197u64) as u32);
            // 7ccbe mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7ccc1;
        }
        0x7ccc1 => {
            // block 7ccc1
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ccc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ccc1 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7ccc4 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7ccc8 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7cccc jne 0x7cc44
            if fl.ne() { pc = 0x7cc44; continue; }
            pc = 0x7ccd2;
        }
        0x7ccd2 => {
            // block 7ccd2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ccd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ccd2 lea rsi,[rip+0xbbfef] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7ccd9 lea rbx,[rbp-0x1490]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eb70);
            // 7cce0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7cce2 mov rdi,rbx
            rdi = rbx;
            // 7cce5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ccea mov esi,0x198
            rsi = (0x198u64) as u32 as u64;
            // 7ccef mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7ccf6 mov rdi,r14
            rdi = r14;
            // 7ccf9 mov rdx,rbx
            rdx = rbx;
            // 7ccfc call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cd01 lea rdi,[rbp-0x1490]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb70);
            // 7cd08 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cd0d mov DWORD PTR [rbp-0x14a0],0x198
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eb60), (0x198u64) as u32);
            // 7cd17 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7cd1e mov QWORD PTR [rbp-0x1498],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eb68), (rax) as u64);
            // 7cd25 lea rsi,[rbp-0x14a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eb60);
            // 7cd2c mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7cd33 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cd38 lea rsi,[rip+0xad264] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7cd3f lea rdi,[rbp-0x14b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb48);
            // 7cd46 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7cd4b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cd50 movzx ebx,BYTE PTR [rbp-0x14b8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_eb48)) as u64) as u32 as u64;
            // 7cd57 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7cd5a jne 0x7cd61
            if fl.ne() { pc = 0x7cd61; continue; }
            pc = 0x7cd5c;
        }
        0x7cd5c => {
            // block 7cd5c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cd5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cd5c shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7cd5f jmp 0x7cd68
            pc = 0x7cd68; continue;
        }
        0x7cd61 => {
            // block 7cd61
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cd61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cd61 mov rbx,QWORD PTR [rbp-0x14b0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eb50)) as u64;
            pc = 0x7cd68;
        }
        0x7cd68 => {
            // block 7cd68
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cd68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cd68 lea rdi,[rbp-0x14b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb48);
            // 7cd6f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cd74 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7cd77 je 0x7ce09
            if fl.e() { pc = 0x7ce09; continue; }
            pc = 0x7cd7d;
        }
        0x7cd7d => {
            // block 7cd7d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cd7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cd7d lea rsi,[rip+0xad21f] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7cd84 lea rdi,[rbp-0x14f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb10);
            // 7cd8b mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7cd90 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cd95 mov rax,QWORD PTR [rbp-0x14e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eb20)) as u64;
            // 7cd9c mov QWORD PTR [rbp-0x14c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eb38), (rax) as u64);
            // 7cda3 mov rax,QWORD PTR [rbp-0x14f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eb10)) as u64;
            // 7cdaa mov rcx,QWORD PTR [rbp-0x14e8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_eb18)) as u64;
            // 7cdb1 mov QWORD PTR [rbp-0x14d0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eb30), (rcx) as u64);
            // 7cdb8 mov QWORD PTR [rbp-0x14d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eb28), (rax) as u64);
            // 7cdbf xorps xmm0,xmm0
            x0 = 0;
            // 7cdc2 movaps XMMWORD PTR [rbp-0x14f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_eb10), x0);
            // 7cdc9 mov QWORD PTR [rbp-0x14e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eb20), (0x0u64) as u64);
            // 7cdd4 mov DWORD PTR [rbp-0x14c0],0x198
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eb40), (0x198u64) as u32);
            // 7cdde lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7cde5 lea rsi,[rbp-0x14d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eb28);
            // 7cdec call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cdf1 lea rdi,[rbp-0x14d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb28);
            // 7cdf8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cdfd lea rdi,[rbp-0x14f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb10);
            // 7ce04 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7ce09;
        }
        0x7ce09 => {
            // block 7ce09
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ce09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ce09 xorps xmm0,xmm0
            x0 = 0;
            // 7ce0c movaps XMMWORD PTR [rbp-0x1510],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_eaf0), x0);
            // 7ce13 mov QWORD PTR [rbp-0x1500],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eb00), (0x0u64) as u64);
            // 7ce1e lea rdi,[rbp-0x1510]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eaf0);
            // 7ce25 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ce2a xorps xmm0,xmm0
            x0 = 0;
            // 7ce2d movaps XMMWORD PTR [rbp-0x1530],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ead0), x0);
            // 7ce34 mov QWORD PTR [rbp-0x1520],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eae0), (0x0u64) as u64);
            // 7ce3f lea rdi,[rbp-0x1530]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ead0);
            // 7ce46 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ce4b lea r14,[rip+0x85a8e] # 0x1028e0
            r14 = m.env.data_va(94381);
            // 7ce52 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7ce54;
        }
        0x7ce54 => {
            // block 7ce54
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ce54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ce54 mov DWORD PTR [rbp-0x1538],0x198
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eac8), (0x198u64) as u32);
            // 7ce5e mov DWORD PTR [rbp-0x1534],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eacc), ((rbx as u32 as u64)) as u32);
            // 7ce64 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7ce6b lea rsi,[rbp-0x1538]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eac8);
            // 7ce72 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ce77 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7ce7a jne 0x7cea8
            if fl.ne() { pc = 0x7cea8; continue; }
            pc = 0x7ce7c;
        }
        0x7ce7c => {
            // block 7ce7c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ce7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ce7c mov r12,r13
            r12 = r13;
            // 7ce7f mov r13,r15
            r13 = r15;
            // 7ce82 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7ce85 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7ce8c lea rsi,[rbp-0x1538]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eac8);
            // 7ce93 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ce98 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7ce9b mov r15,r13
            r15 = r13;
            // 7ce9e mov r13,r12
            r13 = r12;
            // 7cea1 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7cea8;
        }
        0x7cea8 => {
            // block 7cea8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cea8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cea8 mov rdi,r13
            rdi = r13;
            // 7ceab mov rsi,r14
            rsi = r14;
            // 7ceae call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ceb3 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7ceb6 jne 0x7cecc
            if fl.ne() { pc = 0x7cecc; continue; }
            pc = 0x7ceb8;
        }
        0x7ceb8 => {
            // block 7ceb8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ceb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ceb8 mov rdi,r13
            rdi = r13;
            // 7cebb mov rsi,r14
            rsi = r14;
            // 7cebe call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cec3 mov DWORD PTR [rax],0x198
            m.w32(rax, (0x198u64) as u32);
            // 7cec9 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7cecc;
        }
        0x7cecc => {
            // block 7cecc
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cecc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cecc inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7cecf add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7ced3 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7ced7 jne 0x7ce54
            if fl.ne() { pc = 0x7ce54; continue; }
            pc = 0x7cedd;
        }
        0x7cedd => {
            // block 7cedd
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cedd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cedd lea rsi,[rip+0xbbde4] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7cee4 lea rbx,[rbp-0x1550]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eab0);
            // 7ceeb xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7ceed mov rdi,rbx
            rdi = rbx;
            // 7cef0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cef5 mov esi,0x199
            rsi = (0x199u64) as u32 as u64;
            // 7cefa mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7cf01 mov rdi,r14
            rdi = r14;
            // 7cf04 mov rdx,rbx
            rdx = rbx;
            // 7cf07 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cf0c lea rdi,[rbp-0x1550]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eab0);
            // 7cf13 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cf18 mov DWORD PTR [rbp-0x1560],0x199
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eaa0), (0x199u64) as u32);
            // 7cf22 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7cf29 mov QWORD PTR [rbp-0x1558],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eaa8), (rax) as u64);
            // 7cf30 lea rsi,[rbp-0x1560]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eaa0);
            // 7cf37 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7cf3e call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cf43 lea rsi,[rip+0xad059] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7cf4a lea rdi,[rbp-0x1578]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea88);
            // 7cf51 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7cf56 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cf5b movzx ebx,BYTE PTR [rbp-0x1578]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ea88)) as u64) as u32 as u64;
            // 7cf62 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7cf65 jne 0x7cf6c
            if fl.ne() { pc = 0x7cf6c; continue; }
            pc = 0x7cf67;
        }
        0x7cf67 => {
            // block 7cf67
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cf67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cf67 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7cf6a jmp 0x7cf73
            pc = 0x7cf73; continue;
        }
        0x7cf6c => {
            // block 7cf6c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cf6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cf6c mov rbx,QWORD PTR [rbp-0x1570]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ea90)) as u64;
            pc = 0x7cf73;
        }
        0x7cf73 => {
            // block 7cf73
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cf73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cf73 lea rdi,[rbp-0x1578]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea88);
            // 7cf7a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cf7f test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7cf82 je 0x7d014
            if fl.e() { pc = 0x7d014; continue; }
            pc = 0x7cf88;
        }
        0x7cf88 => {
            // block 7cf88
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7cf88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7cf88 lea rsi,[rip+0xad014] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7cf8f lea rdi,[rbp-0x15b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea50);
            // 7cf96 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7cf9b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cfa0 mov rax,QWORD PTR [rbp-0x15a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ea60)) as u64;
            // 7cfa7 mov QWORD PTR [rbp-0x1588],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ea78), (rax) as u64);
            // 7cfae mov rax,QWORD PTR [rbp-0x15b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ea50)) as u64;
            // 7cfb5 mov rcx,QWORD PTR [rbp-0x15a8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ea58)) as u64;
            // 7cfbc mov QWORD PTR [rbp-0x1590],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ea70), (rcx) as u64);
            // 7cfc3 mov QWORD PTR [rbp-0x1598],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ea68), (rax) as u64);
            // 7cfca xorps xmm0,xmm0
            x0 = 0;
            // 7cfcd movaps XMMWORD PTR [rbp-0x15b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ea50), x0);
            // 7cfd4 mov QWORD PTR [rbp-0x15a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ea60), (0x0u64) as u64);
            // 7cfdf mov DWORD PTR [rbp-0x1580],0x199
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ea80), (0x199u64) as u32);
            // 7cfe9 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7cff0 lea rsi,[rbp-0x1598]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ea68);
            // 7cff7 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7cffc lea rdi,[rbp-0x1598]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea68);
            // 7d003 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d008 lea rdi,[rbp-0x15b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea50);
            // 7d00f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7d014;
        }
        0x7d014 => {
            // block 7d014
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d014 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d014 xorps xmm0,xmm0
            x0 = 0;
            // 7d017 movaps XMMWORD PTR [rbp-0x15d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ea30), x0);
            // 7d01e mov QWORD PTR [rbp-0x15c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ea40), (0x0u64) as u64);
            // 7d029 lea rdi,[rbp-0x15d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea30);
            // 7d030 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d035 xorps xmm0,xmm0
            x0 = 0;
            // 7d038 movaps XMMWORD PTR [rbp-0x15f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ea10), x0);
            // 7d03f mov QWORD PTR [rbp-0x15e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ea20), (0x0u64) as u64);
            // 7d04a lea rdi,[rbp-0x15f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea10);
            // 7d051 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d056 lea r14,[rip+0x85893] # 0x1028f0
            r14 = m.env.data_va(94397);
            // 7d05d xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7d05f;
        }
        0x7d05f => {
            // block 7d05f
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d05f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d05f mov DWORD PTR [rbp-0x15f8],0x199
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ea08), (0x199u64) as u32);
            // 7d069 mov DWORD PTR [rbp-0x15f4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ea0c), ((rbx as u32 as u64)) as u32);
            // 7d06f mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7d076 lea rsi,[rbp-0x15f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ea08);
            // 7d07d call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d082 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7d085 jne 0x7d0b3
            if fl.ne() { pc = 0x7d0b3; continue; }
            pc = 0x7d087;
        }
        0x7d087 => {
            // block 7d087
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d087 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d087 mov r12,r13
            r12 = r13;
            // 7d08a mov r13,r15
            r13 = r15;
            // 7d08d mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7d090 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7d097 lea rsi,[rbp-0x15f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ea08);
            // 7d09e call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d0a3 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7d0a6 mov r15,r13
            r15 = r13;
            // 7d0a9 mov r13,r12
            r13 = r12;
            // 7d0ac mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7d0b3;
        }
        0x7d0b3 => {
            // block 7d0b3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d0b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d0b3 mov rdi,r13
            rdi = r13;
            // 7d0b6 mov rsi,r14
            rsi = r14;
            // 7d0b9 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d0be cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7d0c1 jne 0x7d0d7
            if fl.ne() { pc = 0x7d0d7; continue; }
            pc = 0x7d0c3;
        }
        0x7d0c3 => {
            // block 7d0c3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d0c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d0c3 mov rdi,r13
            rdi = r13;
            // 7d0c6 mov rsi,r14
            rsi = r14;
            // 7d0c9 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d0ce mov DWORD PTR [rax],0x199
            m.w32(rax, (0x199u64) as u32);
            // 7d0d4 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7d0d7;
        }
        0x7d0d7 => {
            // block 7d0d7
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d0d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d0d7 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7d0da add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7d0de cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7d0e2 jne 0x7d05f
            if fl.ne() { pc = 0x7d05f; continue; }
            pc = 0x7d0e8;
        }
        0x7d0e8 => {
            // block 7d0e8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d0e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d0e8 lea rsi,[rip+0xbbbd9] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7d0ef lea rbx,[rbp-0x1610]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e9f0);
            // 7d0f6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7d0f8 mov rdi,rbx
            rdi = rbx;
            // 7d0fb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d100 mov esi,0x192
            rsi = (0x192u64) as u32 as u64;
            // 7d105 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7d10c mov rdi,r14
            rdi = r14;
            // 7d10f mov rdx,rbx
            rdx = rbx;
            // 7d112 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d117 lea rdi,[rbp-0x1610]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9f0);
            // 7d11e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d123 mov DWORD PTR [rbp-0x1620],0x192
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e9e0), (0x192u64) as u32);
            // 7d12d mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7d134 mov QWORD PTR [rbp-0x1618],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e9e8), (rax) as u64);
            // 7d13b lea rsi,[rbp-0x1620]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e9e0);
            // 7d142 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7d149 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d14e lea rsi,[rip+0xace4e] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7d155 lea rdi,[rbp-0x1638]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9c8);
            // 7d15c mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7d161 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d166 movzx ebx,BYTE PTR [rbp-0x1638]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e9c8)) as u64) as u32 as u64;
            // 7d16d test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7d170 jne 0x7d177
            if fl.ne() { pc = 0x7d177; continue; }
            pc = 0x7d172;
        }
        0x7d172 => {
            // block 7d172
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d172 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d172 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7d175 jmp 0x7d17e
            pc = 0x7d17e; continue;
        }
        0x7d177 => {
            // block 7d177
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d177 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d177 mov rbx,QWORD PTR [rbp-0x1630]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e9d0)) as u64;
            pc = 0x7d17e;
        }
        0x7d17e => {
            // block 7d17e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d17e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d17e lea rdi,[rbp-0x1638]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9c8);
            // 7d185 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d18a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7d18d je 0x7d21f
            if fl.e() { pc = 0x7d21f; continue; }
            pc = 0x7d193;
        }
        0x7d193 => {
            // block 7d193
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d193 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d193 lea rsi,[rip+0xace09] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7d19a lea rdi,[rbp-0x1670]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e990);
            // 7d1a1 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7d1a6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d1ab mov rax,QWORD PTR [rbp-0x1660]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e9a0)) as u64;
            // 7d1b2 mov QWORD PTR [rbp-0x1648],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e9b8), (rax) as u64);
            // 7d1b9 mov rax,QWORD PTR [rbp-0x1670]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e990)) as u64;
            // 7d1c0 mov rcx,QWORD PTR [rbp-0x1668]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e998)) as u64;
            // 7d1c7 mov QWORD PTR [rbp-0x1650],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e9b0), (rcx) as u64);
            // 7d1ce mov QWORD PTR [rbp-0x1658],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e9a8), (rax) as u64);
            // 7d1d5 xorps xmm0,xmm0
            x0 = 0;
            // 7d1d8 movaps XMMWORD PTR [rbp-0x1670],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e990), x0);
            // 7d1df mov QWORD PTR [rbp-0x1660],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e9a0), (0x0u64) as u64);
            // 7d1ea mov DWORD PTR [rbp-0x1640],0x192
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e9c0), (0x192u64) as u32);
            // 7d1f4 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7d1fb lea rsi,[rbp-0x1658]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e9a8);
            // 7d202 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d207 lea rdi,[rbp-0x1658]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9a8);
            // 7d20e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d213 lea rdi,[rbp-0x1670]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e990);
            // 7d21a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7d21f;
        }
        0x7d21f => {
            // block 7d21f
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d21f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d21f xorps xmm0,xmm0
            x0 = 0;
            // 7d222 movaps XMMWORD PTR [rbp-0x1690],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e970), x0);
            // 7d229 mov QWORD PTR [rbp-0x1680],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e980), (0x0u64) as u64);
            // 7d234 lea rdi,[rbp-0x1690]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e970);
            // 7d23b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d240 xorps xmm0,xmm0
            x0 = 0;
            // 7d243 movaps XMMWORD PTR [rbp-0x16b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e950), x0);
            // 7d24a mov QWORD PTR [rbp-0x16a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e960), (0x0u64) as u64);
            // 7d255 lea rdi,[rbp-0x16b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e950);
            // 7d25c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d261 lea r14,[rip+0x85698] # 0x102900
            r14 = m.env.data_va(94413);
            // 7d268 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7d26a;
        }
        0x7d26a => {
            // block 7d26a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d26a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d26a mov DWORD PTR [rbp-0x16b8],0x192
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e948), (0x192u64) as u32);
            // 7d274 mov DWORD PTR [rbp-0x16b4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e94c), ((rbx as u32 as u64)) as u32);
            // 7d27a mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7d281 lea rsi,[rbp-0x16b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e948);
            // 7d288 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d28d cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7d290 jne 0x7d2be
            if fl.ne() { pc = 0x7d2be; continue; }
            pc = 0x7d292;
        }
        0x7d292 => {
            // block 7d292
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d292 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d292 mov r12,r13
            r12 = r13;
            // 7d295 mov r13,r15
            r13 = r15;
            // 7d298 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7d29b mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7d2a2 lea rsi,[rbp-0x16b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e948);
            // 7d2a9 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d2ae mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7d2b1 mov r15,r13
            r15 = r13;
            // 7d2b4 mov r13,r12
            r13 = r12;
            // 7d2b7 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7d2be;
        }
        0x7d2be => {
            // block 7d2be
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d2be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d2be mov rdi,r13
            rdi = r13;
            // 7d2c1 mov rsi,r14
            rsi = r14;
            // 7d2c4 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d2c9 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7d2cc jne 0x7d2e2
            if fl.ne() { pc = 0x7d2e2; continue; }
            pc = 0x7d2ce;
        }
        0x7d2ce => {
            // block 7d2ce
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d2ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d2ce mov rdi,r13
            rdi = r13;
            // 7d2d1 mov rsi,r14
            rsi = r14;
            // 7d2d4 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d2d9 mov DWORD PTR [rax],0x192
            m.w32(rax, (0x192u64) as u32);
            // 7d2df mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7d2e2;
        }
        0x7d2e2 => {
            // block 7d2e2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d2e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d2e2 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7d2e5 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7d2e9 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7d2ed jne 0x7d26a
            if fl.ne() { pc = 0x7d26a; continue; }
            pc = 0x7d2f3;
        }
        0x7d2f3 => {
            // block 7d2f3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d2f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d2f3 lea rsi,[rip+0xbb9ce] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7d2fa lea rbx,[rbp-0x16d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e930);
            // 7d301 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7d303 mov rdi,rbx
            rdi = rbx;
            // 7d306 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d30b mov esi,0x1fa
            rsi = (0x1fau64) as u32 as u64;
            // 7d310 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7d317 mov rdi,r14
            rdi = r14;
            // 7d31a mov rdx,rbx
            rdx = rbx;
            // 7d31d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d322 lea rdi,[rbp-0x16d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e930);
            // 7d329 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d32e mov DWORD PTR [rbp-0x16e0],0x1fa
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e920), (0x1fau64) as u32);
            // 7d338 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7d33f mov QWORD PTR [rbp-0x16d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e928), (rax) as u64);
            // 7d346 lea rsi,[rbp-0x16e0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e920);
            // 7d34d mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7d354 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d359 lea rsi,[rip+0xacc43] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7d360 lea rdi,[rbp-0x16f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e908);
            // 7d367 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7d36c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d371 movzx ebx,BYTE PTR [rbp-0x16f8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e908)) as u64) as u32 as u64;
            // 7d378 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7d37b jne 0x7d382
            if fl.ne() { pc = 0x7d382; continue; }
            pc = 0x7d37d;
        }
        0x7d37d => {
            // block 7d37d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d37d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d37d shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7d380 jmp 0x7d389
            pc = 0x7d389; continue;
        }
        0x7d382 => {
            // block 7d382
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d382 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d382 mov rbx,QWORD PTR [rbp-0x16f0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e910)) as u64;
            pc = 0x7d389;
        }
        0x7d389 => {
            // block 7d389
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d389 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d389 lea rdi,[rbp-0x16f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e908);
            // 7d390 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d395 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7d398 je 0x7d42a
            if fl.e() { pc = 0x7d42a; continue; }
            pc = 0x7d39e;
        }
        0x7d39e => {
            // block 7d39e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d39e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d39e lea rsi,[rip+0xacbfe] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7d3a5 lea rdi,[rbp-0x1730]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8d0);
            // 7d3ac mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7d3b1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d3b6 mov rax,QWORD PTR [rbp-0x1720]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e8e0)) as u64;
            // 7d3bd mov QWORD PTR [rbp-0x1708],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e8f8), (rax) as u64);
            // 7d3c4 mov rax,QWORD PTR [rbp-0x1730]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e8d0)) as u64;
            // 7d3cb mov rcx,QWORD PTR [rbp-0x1728]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e8d8)) as u64;
            // 7d3d2 mov QWORD PTR [rbp-0x1710],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e8f0), (rcx) as u64);
            // 7d3d9 mov QWORD PTR [rbp-0x1718],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e8e8), (rax) as u64);
            // 7d3e0 xorps xmm0,xmm0
            x0 = 0;
            // 7d3e3 movaps XMMWORD PTR [rbp-0x1730],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e8d0), x0);
            // 7d3ea mov QWORD PTR [rbp-0x1720],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e8e0), (0x0u64) as u64);
            // 7d3f5 mov DWORD PTR [rbp-0x1700],0x1fa
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e900), (0x1fau64) as u32);
            // 7d3ff lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7d406 lea rsi,[rbp-0x1718]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e8e8);
            // 7d40d call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d412 lea rdi,[rbp-0x1718]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8e8);
            // 7d419 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d41e lea rdi,[rbp-0x1730]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8d0);
            // 7d425 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7d42a;
        }
        0x7d42a => {
            // block 7d42a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d42a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d42a xorps xmm0,xmm0
            x0 = 0;
            // 7d42d movaps XMMWORD PTR [rbp-0x1750],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e8b0), x0);
            // 7d434 mov QWORD PTR [rbp-0x1740],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e8c0), (0x0u64) as u64);
            // 7d43f lea rdi,[rbp-0x1750]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8b0);
            // 7d446 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d44b xorps xmm0,xmm0
            x0 = 0;
            // 7d44e movaps XMMWORD PTR [rbp-0x1770],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e890), x0);
            // 7d455 mov QWORD PTR [rbp-0x1760],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e8a0), (0x0u64) as u64);
            // 7d460 lea rdi,[rbp-0x1770]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e890);
            // 7d467 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d46c lea r14,[rip+0x8549d] # 0x102910
            r14 = m.env.data_va(94429);
            // 7d473 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7d475;
        }
        0x7d475 => {
            // block 7d475
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d475 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d475 mov DWORD PTR [rbp-0x1778],0x1fa
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e888), (0x1fau64) as u32);
            // 7d47f mov DWORD PTR [rbp-0x1774],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e88c), ((rbx as u32 as u64)) as u32);
            // 7d485 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7d48c lea rsi,[rbp-0x1778]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e888);
            // 7d493 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d498 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7d49b jne 0x7d4c9
            if fl.ne() { pc = 0x7d4c9; continue; }
            pc = 0x7d49d;
        }
        0x7d49d => {
            // block 7d49d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d49d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d49d mov r12,r13
            r12 = r13;
            // 7d4a0 mov r13,r15
            r13 = r15;
            // 7d4a3 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7d4a6 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7d4ad lea rsi,[rbp-0x1778]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e888);
            // 7d4b4 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d4b9 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7d4bc mov r15,r13
            r15 = r13;
            // 7d4bf mov r13,r12
            r13 = r12;
            // 7d4c2 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7d4c9;
        }
        0x7d4c9 => {
            // block 7d4c9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d4c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d4c9 mov rdi,r13
            rdi = r13;
            // 7d4cc mov rsi,r14
            rsi = r14;
            // 7d4cf call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d4d4 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7d4d7 jne 0x7d4ed
            if fl.ne() { pc = 0x7d4ed; continue; }
            pc = 0x7d4d9;
        }
        0x7d4d9 => {
            // block 7d4d9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d4d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d4d9 mov rdi,r13
            rdi = r13;
            // 7d4dc mov rsi,r14
            rsi = r14;
            // 7d4df call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d4e4 mov DWORD PTR [rax],0x1fa
            m.w32(rax, (0x1fau64) as u32);
            // 7d4ea mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7d4ed;
        }
        0x7d4ed => {
            // block 7d4ed
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d4ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d4ed inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7d4f0 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7d4f4 cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7d4f8 jne 0x7d475
            if fl.ne() { pc = 0x7d475; continue; }
            pc = 0x7d4fe;
        }
        0x7d4fe => {
            // block 7d4fe
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d4fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d4fe lea rsi,[rip+0xbb7c3] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7d505 lea rbx,[rbp-0x1790]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e870);
            // 7d50c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7d50e mov rdi,rbx
            rdi = rbx;
            // 7d511 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d516 mov esi,0x1fb
            rsi = (0x1fbu64) as u32 as u64;
            // 7d51b mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7d522 mov rdi,r14
            rdi = r14;
            // 7d525 mov rdx,rbx
            rdx = rbx;
            // 7d528 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d52d lea rdi,[rbp-0x1790]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e870);
            // 7d534 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d539 mov DWORD PTR [rbp-0x17a0],0x1fb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e860), (0x1fbu64) as u32);
            // 7d543 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7d54a mov QWORD PTR [rbp-0x1798],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e868), (rax) as u64);
            // 7d551 lea rsi,[rbp-0x17a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e860);
            // 7d558 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7d55f call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d564 lea rsi,[rip+0xaca38] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7d56b lea rdi,[rbp-0x17b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e848);
            // 7d572 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7d577 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d57c movzx ebx,BYTE PTR [rbp-0x17b8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e848)) as u64) as u32 as u64;
            // 7d583 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7d586 jne 0x7d58d
            if fl.ne() { pc = 0x7d58d; continue; }
            pc = 0x7d588;
        }
        0x7d588 => {
            // block 7d588
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d588 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d588 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7d58b jmp 0x7d594
            pc = 0x7d594; continue;
        }
        0x7d58d => {
            // block 7d58d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d58d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d58d mov rbx,QWORD PTR [rbp-0x17b0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e850)) as u64;
            pc = 0x7d594;
        }
        0x7d594 => {
            // block 7d594
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d594 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d594 lea rdi,[rbp-0x17b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e848);
            // 7d59b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d5a0 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7d5a3 je 0x7d635
            if fl.e() { pc = 0x7d635; continue; }
            pc = 0x7d5a9;
        }
        0x7d5a9 => {
            // block 7d5a9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d5a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d5a9 lea rsi,[rip+0xac9f3] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7d5b0 lea rdi,[rbp-0x17f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e810);
            // 7d5b7 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7d5bc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d5c1 mov rax,QWORD PTR [rbp-0x17e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e820)) as u64;
            // 7d5c8 mov QWORD PTR [rbp-0x17c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e838), (rax) as u64);
            // 7d5cf mov rax,QWORD PTR [rbp-0x17f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e810)) as u64;
            // 7d5d6 mov rcx,QWORD PTR [rbp-0x17e8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e818)) as u64;
            // 7d5dd mov QWORD PTR [rbp-0x17d0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e830), (rcx) as u64);
            // 7d5e4 mov QWORD PTR [rbp-0x17d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e828), (rax) as u64);
            // 7d5eb xorps xmm0,xmm0
            x0 = 0;
            // 7d5ee movaps XMMWORD PTR [rbp-0x17f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e810), x0);
            // 7d5f5 mov QWORD PTR [rbp-0x17e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e820), (0x0u64) as u64);
            // 7d600 mov DWORD PTR [rbp-0x17c0],0x1fb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e840), (0x1fbu64) as u32);
            // 7d60a lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7d611 lea rsi,[rbp-0x17d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e828);
            // 7d618 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d61d lea rdi,[rbp-0x17d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e828);
            // 7d624 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d629 lea rdi,[rbp-0x17f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e810);
            // 7d630 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7d635;
        }
        0x7d635 => {
            // block 7d635
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d635 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d635 xorps xmm0,xmm0
            x0 = 0;
            // 7d638 movaps XMMWORD PTR [rbp-0x1810],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e7f0), x0);
            // 7d63f mov QWORD PTR [rbp-0x1800],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e800), (0x0u64) as u64);
            // 7d64a lea rdi,[rbp-0x1810]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7f0);
            // 7d651 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d656 xorps xmm0,xmm0
            x0 = 0;
            // 7d659 movaps XMMWORD PTR [rbp-0x1830],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e7d0), x0);
            // 7d660 mov QWORD PTR [rbp-0x1820],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e7e0), (0x0u64) as u64);
            // 7d66b lea rdi,[rbp-0x1830]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7d0);
            // 7d672 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d677 lea r14,[rip+0x852a2] # 0x102920
            r14 = m.env.data_va(94445);
            // 7d67e xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7d680;
        }
        0x7d680 => {
            // block 7d680
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d680 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d680 mov DWORD PTR [rbp-0x1838],0x1fb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e7c8), (0x1fbu64) as u32);
            // 7d68a mov DWORD PTR [rbp-0x1834],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e7cc), ((rbx as u32 as u64)) as u32);
            // 7d690 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7d697 lea rsi,[rbp-0x1838]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e7c8);
            // 7d69e call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d6a3 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7d6a6 jne 0x7d6d4
            if fl.ne() { pc = 0x7d6d4; continue; }
            pc = 0x7d6a8;
        }
        0x7d6a8 => {
            // block 7d6a8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d6a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d6a8 mov r12,r13
            r12 = r13;
            // 7d6ab mov r13,r15
            r13 = r15;
            // 7d6ae mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7d6b1 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7d6b8 lea rsi,[rbp-0x1838]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e7c8);
            // 7d6bf call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d6c4 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7d6c7 mov r15,r13
            r15 = r13;
            // 7d6ca mov r13,r12
            r13 = r12;
            // 7d6cd mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7d6d4;
        }
        0x7d6d4 => {
            // block 7d6d4
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d6d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d6d4 mov rdi,r13
            rdi = r13;
            // 7d6d7 mov rsi,r14
            rsi = r14;
            // 7d6da call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d6df cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7d6e2 jne 0x7d6f8
            if fl.ne() { pc = 0x7d6f8; continue; }
            pc = 0x7d6e4;
        }
        0x7d6e4 => {
            // block 7d6e4
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d6e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d6e4 mov rdi,r13
            rdi = r13;
            // 7d6e7 mov rsi,r14
            rsi = r14;
            // 7d6ea call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d6ef mov DWORD PTR [rax],0x1fb
            m.w32(rax, (0x1fbu64) as u32);
            // 7d6f5 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7d6f8;
        }
        0x7d6f8 => {
            // block 7d6f8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d6f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d6f8 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7d6fb add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7d6ff cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7d703 jne 0x7d680
            if fl.ne() { pc = 0x7d680; continue; }
            pc = 0x7d709;
        }
        0x7d709 => {
            // block 7d709
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d709 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d709 lea rsi,[rip+0xbb5b8] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7d710 lea rbx,[rbp-0x1850]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e7b0);
            // 7d717 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7d719 mov rdi,rbx
            rdi = rbx;
            // 7d71c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d721 mov esi,0x1fc
            rsi = (0x1fcu64) as u32 as u64;
            // 7d726 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7d72d mov rdi,r14
            rdi = r14;
            // 7d730 mov rdx,rbx
            rdx = rbx;
            // 7d733 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d738 lea rdi,[rbp-0x1850]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7b0);
            // 7d73f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d744 mov DWORD PTR [rbp-0x1860],0x1fc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e7a0), (0x1fcu64) as u32);
            // 7d74e mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7d755 mov QWORD PTR [rbp-0x1858],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e7a8), (rax) as u64);
            // 7d75c lea rsi,[rbp-0x1860]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e7a0);
            // 7d763 mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7d76a call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d76f lea rsi,[rip+0xac82d] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7d776 lea rdi,[rbp-0x1878]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e788);
            // 7d77d mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7d782 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d787 movzx ebx,BYTE PTR [rbp-0x1878]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e788)) as u64) as u32 as u64;
            // 7d78e test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7d791 jne 0x7d798
            if fl.ne() { pc = 0x7d798; continue; }
            pc = 0x7d793;
        }
        0x7d793 => {
            // block 7d793
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d793 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d793 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7d796 jmp 0x7d79f
            pc = 0x7d79f; continue;
        }
        0x7d798 => {
            // block 7d798
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d798 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d798 mov rbx,QWORD PTR [rbp-0x1870]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e790)) as u64;
            pc = 0x7d79f;
        }
        0x7d79f => {
            // block 7d79f
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d79f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d79f lea rdi,[rbp-0x1878]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e788);
            // 7d7a6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d7ab test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7d7ae je 0x7d840
            if fl.e() { pc = 0x7d840; continue; }
            pc = 0x7d7b4;
        }
        0x7d7b4 => {
            // block 7d7b4
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d7b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d7b4 lea rsi,[rip+0xac7e8] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7d7bb lea rdi,[rbp-0x18b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e750);
            // 7d7c2 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7d7c7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d7cc mov rax,QWORD PTR [rbp-0x18a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e760)) as u64;
            // 7d7d3 mov QWORD PTR [rbp-0x1888],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e778), (rax) as u64);
            // 7d7da mov rax,QWORD PTR [rbp-0x18b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e750)) as u64;
            // 7d7e1 mov rcx,QWORD PTR [rbp-0x18a8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e758)) as u64;
            // 7d7e8 mov QWORD PTR [rbp-0x1890],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e770), (rcx) as u64);
            // 7d7ef mov QWORD PTR [rbp-0x1898],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e768), (rax) as u64);
            // 7d7f6 xorps xmm0,xmm0
            x0 = 0;
            // 7d7f9 movaps XMMWORD PTR [rbp-0x18b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e750), x0);
            // 7d800 mov QWORD PTR [rbp-0x18a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e760), (0x0u64) as u64);
            // 7d80b mov DWORD PTR [rbp-0x1880],0x1fc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e780), (0x1fcu64) as u32);
            // 7d815 lea rdi,[r14+0x198]
            rdi = r14.wrapping_add(0x198);
            // 7d81c lea rsi,[rbp-0x1898]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e768);
            // 7d823 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d828 lea rdi,[rbp-0x1898]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e768);
            // 7d82f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d834 lea rdi,[rbp-0x18b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e750);
            // 7d83b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7d840;
        }
        0x7d840 => {
            // block 7d840
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d840 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d840 xorps xmm0,xmm0
            x0 = 0;
            // 7d843 movaps XMMWORD PTR [rbp-0x18d0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e730), x0);
            // 7d84a mov QWORD PTR [rbp-0x18c0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e740), (0x0u64) as u64);
            // 7d855 lea rdi,[rbp-0x18d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e730);
            // 7d85c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d861 xorps xmm0,xmm0
            x0 = 0;
            // 7d864 movaps XMMWORD PTR [rbp-0x18f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e710), x0);
            // 7d86b mov QWORD PTR [rbp-0x18e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e720), (0x0u64) as u64);
            // 7d876 lea rdi,[rbp-0x18f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e710);
            // 7d87d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d882 lea r14,[rip+0x850a7] # 0x102930
            r14 = m.env.data_va(94461);
            // 7d889 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7d88b;
        }
        0x7d88b => {
            // block 7d88b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d88b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d88b mov DWORD PTR [rbp-0x18f8],0x1fc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e708), (0x1fcu64) as u32);
            // 7d895 mov DWORD PTR [rbp-0x18f4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e70c), ((rbx as u32 as u64)) as u32);
            // 7d89b mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7d8a2 lea rsi,[rbp-0x18f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e708);
            // 7d8a9 call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d8ae cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7d8b1 jne 0x7d8df
            if fl.ne() { pc = 0x7d8df; continue; }
            pc = 0x7d8b3;
        }
        0x7d8b3 => {
            // block 7d8b3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d8b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d8b3 mov r12,r13
            r12 = r13;
            // 7d8b6 mov r13,r15
            r13 = r15;
            // 7d8b9 mov r15d,DWORD PTR [r14]
            r15 = (m.r32(r14) as u64) as u32 as u64;
            // 7d8bc mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7d8c3 lea rsi,[rbp-0x18f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e708);
            // 7d8ca call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d8cf mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7d8d2 mov r15,r13
            r15 = r13;
            // 7d8d5 mov r13,r12
            r13 = r12;
            // 7d8d8 mov r12,QWORD PTR [rbp-0x19c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64;
            pc = 0x7d8df;
        }
        0x7d8df => {
            // block 7d8df
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d8df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d8df mov rdi,r13
            rdi = r13;
            // 7d8e2 mov rsi,r14
            rsi = r14;
            // 7d8e5 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d8ea cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 7d8ed jne 0x7d903
            if fl.ne() { pc = 0x7d903; continue; }
            pc = 0x7d8ef;
        }
        0x7d8ef => {
            // block 7d8ef
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d8ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d8ef mov rdi,r13
            rdi = r13;
            // 7d8f2 mov rsi,r14
            rsi = r14;
            // 7d8f5 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d8fa mov DWORD PTR [rax],0x1fc
            m.w32(rax, (0x1fcu64) as u32);
            // 7d900 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7d903;
        }
        0x7d903 => {
            // block 7d903
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d903 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d903 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7d906 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 7d90a cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7d90e jne 0x7d88b
            if fl.ne() { pc = 0x7d88b; continue; }
            pc = 0x7d914;
        }
        0x7d914 => {
            // block 7d914
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d914 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d914 lea rsi,[rip+0xbb3ad] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 7d91b lea rbx,[rbp-0x1910]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e6f0);
            // 7d922 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7d924 mov rdi,rbx
            rdi = rbx;
            // 7d927 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d92c mov esi,0x1fd
            rsi = (0x1fdu64) as u32 as u64;
            // 7d931 mov r14,QWORD PTR [rbp-0x19d0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e630)) as u64;
            // 7d938 mov rdi,r14
            rdi = r14;
            // 7d93b mov rdx,rbx
            rdx = rbx;
            // 7d93e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d943 lea rdi,[rbp-0x1910]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6f0);
            // 7d94a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d94f mov DWORD PTR [rbp-0x1920],0x1fd
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e6e0), (0x1fdu64) as u32);
            // 7d959 mov rax,QWORD PTR [rbp-0x19d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e628)) as u64;
            // 7d960 mov QWORD PTR [rbp-0x1918],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e6e8), (rax) as u64);
            // 7d967 lea rsi,[rbp-0x1920]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e6e0);
            // 7d96e mov rdi,QWORD PTR [rbp-0x19e0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e620)) as u64;
            // 7d975 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d97a lea rsi,[rip+0xac622] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7d981 lea rdi,[rbp-0x1938]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6c8);
            // 7d988 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7d98d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d992 movzx ebx,BYTE PTR [rbp-0x1938]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_e6c8)) as u64) as u32 as u64;
            // 7d999 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 7d99c jne 0x7d9a3
            if fl.ne() { pc = 0x7d9a3; continue; }
            pc = 0x7d99e;
        }
        0x7d99e => {
            // block 7d99e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d99e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d99e shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 7d9a1 jmp 0x7d9aa
            pc = 0x7d9aa; continue;
        }
        0x7d9a3 => {
            // block 7d9a3
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d9a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d9a3 mov rbx,QWORD PTR [rbp-0x1930]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e6d0)) as u64;
            pc = 0x7d9aa;
        }
        0x7d9aa => {
            // block 7d9aa
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d9aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d9aa mov QWORD PTR [rbp-0x19c8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e638), (r12) as u64);
            // 7d9b1 lea rdi,[rbp-0x1938]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6c8);
            // 7d9b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d9bd test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 7d9c0 je 0x7da55
            if fl.e() { pc = 0x7da55; continue; }
            pc = 0x7d9c6;
        }
        0x7d9c6 => {
            // block 7d9c6
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7d9c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d9c6 lea rsi,[rip+0xac5d6] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 7d9cd lea rdi,[rbp-0x1970]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e690);
            // 7d9d4 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 7d9d9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7d9de mov rax,QWORD PTR [rbp-0x1960]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e6a0)) as u64;
            // 7d9e5 mov QWORD PTR [rbp-0x1948],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e6b8), (rax) as u64);
            // 7d9ec mov rax,QWORD PTR [rbp-0x1970]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e690)) as u64;
            // 7d9f3 mov rcx,QWORD PTR [rbp-0x1968]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e698)) as u64;
            // 7d9fa mov QWORD PTR [rbp-0x1950],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e6b0), (rcx) as u64);
            // 7da01 mov QWORD PTR [rbp-0x1958],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e6a8), (rax) as u64);
            // 7da08 xorps xmm0,xmm0
            x0 = 0;
            // 7da0b movaps XMMWORD PTR [rbp-0x1970],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e690), x0);
            // 7da12 mov QWORD PTR [rbp-0x1960],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e6a0), (0x0u64) as u64);
            // 7da1d mov DWORD PTR [rbp-0x1940],0x1fd
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e6c0), (0x1fdu64) as u32);
            // 7da27 add r14,0x198
            { let (r, f) = add::<64>(r14, 0x198u64);
              r14 = r;
              fl = f; }
            // 7da2e lea rsi,[rbp-0x1958]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e6a8);
            // 7da35 mov rdi,r14
            rdi = r14;
            // 7da38 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7da3d lea rdi,[rbp-0x1958]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6a8);
            // 7da44 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7da49 lea rdi,[rbp-0x1970]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e690);
            // 7da50 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7da55;
        }
        0x7da55 => {
            // block 7da55
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7da55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7da55 xorps xmm0,xmm0
            x0 = 0;
            // 7da58 movaps XMMWORD PTR [rbp-0x1990],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e670), x0);
            // 7da5f mov QWORD PTR [rbp-0x1980],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e680), (0x0u64) as u64);
            // 7da6a lea rdi,[rbp-0x1990]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e670);
            // 7da71 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7da76 xorps xmm0,xmm0
            x0 = 0;
            // 7da79 movaps XMMWORD PTR [rbp-0x19b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_e650), x0);
            // 7da80 mov QWORD PTR [rbp-0x19a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_e660), (0x0u64) as u64);
            // 7da8b lea rdi,[rbp-0x19b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e650);
            // 7da92 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7da97 lea r12,[rip+0x84ea2] # 0x102940
            r12 = m.env.data_va(94477);
            // 7da9e xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x7daa0;
        }
        0x7daa0 => {
            // block 7daa0
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7daa0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7daa0 mov DWORD PTR [rbp-0x19b8],0x1fd
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e648), (0x1fdu64) as u32);
            // 7daaa mov DWORD PTR [rbp-0x19b4],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_e64c), ((rbx as u32 as u64)) as u32);
            // 7dab0 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7dab7 lea rsi,[rbp-0x19b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e648);
            // 7dabe call 0xebc42
            { let r = { let t = m.env.stub_va(0x40f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7dac3 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 7dac6 jne 0x7daee
            if fl.ne() { pc = 0x7daee; continue; }
            pc = 0x7dac8;
        }
        0x7dac8 => {
            // block 7dac8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dac8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dac8 mov r14,r13
            r14 = r13;
            // 7dacb mov r13,r15
            r13 = r15;
            // 7dace mov r15d,DWORD PTR [r12]
            r15 = (m.r32(r12) as u64) as u32 as u64;
            // 7dad2 mov rdi,QWORD PTR [rbp-0x19c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e640)) as u64;
            // 7dad9 lea rsi,[rbp-0x19b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_e648);
            // 7dae0 call 0xebc0c
            { let r = { let t = m.env.stub_va(0x3d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7dae5 mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 7dae8 mov r15,r13
            r15 = r13;
            // 7daeb mov r13,r14
            r13 = r14;
            pc = 0x7daee;
        }
        0x7daee => {
            // block 7daee
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7daee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7daee mov rdi,r13
            rdi = r13;
            // 7daf1 mov rsi,r12
            rsi = r12;
            // 7daf4 call 0xebc18
            { let r = { let t = m.env.stub_va(0x3e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7daf9 cmp rax,QWORD PTR [rbp-0x19c8]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_e638)) as u64).1;
            // 7db00 jne 0x7db16
            if fl.ne() { pc = 0x7db16; continue; }
            pc = 0x7db02;
        }
        0x7db02 => {
            // block 7db02
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7db02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7db02 mov rdi,r13
            rdi = r13;
            // 7db05 mov rsi,r12
            rsi = r12;
            // 7db08 call 0xebc00
            { let r = { let t = m.env.stub_va(0x3cd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7db0d mov DWORD PTR [rax],0x1fd
            m.w32(rax, (0x1fdu64) as u32);
            // 7db13 mov DWORD PTR [rax+0x4],ebx
            m.w32(rax.wrapping_add(0x4), ((rbx as u32 as u64)) as u32);
            pc = 0x7db16;
        }
        0x7db16 => {
            // block 7db16
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7db16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7db16 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7db19 add r12,0x4
            { let (r, f) = add::<64>(r12, 0x4u64);
              r12 = r;
              fl = f; }
            // 7db1d cmp rbx,0x4
            fl = sub::<64>(rbx, 0x4u64).1;
            // 7db21 jne 0x7daa0
            if fl.ne() { pc = 0x7daa0; continue; }
            pc = 0x7db27;
        }
        0x7db27 => {
            // block 7db27
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7db27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7db27 add rsp,0x19b8
            { let (r, f) = add::<64>(rsp, 0x19b8u64);
              rsp = r;
              fl = f; }
            // 7db2e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7db2f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 7db31 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 7db33 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 7db35 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 7db37 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7db38 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x7db39 => {
            // block 7db39
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7db39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7db39 mov rbx,rax
            rbx = rax;
            // 7db3c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 7db40 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7db45 => {
            // block 7db45
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7db45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7db45 mov rbx,rax
            rbx = rax;
            // 7db48 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 7db4f jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7db54 => {
            // block 7db54
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7db54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7db54 mov rbx,rax
            rbx = rax;
            // 7db57 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 7db5e jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7db63 => {
            // block 7db63
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7db63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7db63 mov rbx,rax
            rbx = rax;
            // 7db66 lea rdi,[rbp-0x280]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd80);
            // 7db6d jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7db72 => {
            // block 7db72
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7db72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7db72 mov rbx,rax
            rbx = rax;
            // 7db75 lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 7db7c jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7db81 => {
            // block 7db81
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7db81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7db81 mov rbx,rax
            rbx = rax;
            // 7db84 lea rdi,[rbp-0x400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 7db8b jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7db90 => {
            // block 7db90
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7db90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7db90 mov rbx,rax
            rbx = rax;
            // 7db93 lea rdi,[rbp-0x4d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb30);
            // 7db9a jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7db9f => {
            // block 7db9f
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7db9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7db9f mov rbx,rax
            rbx = rax;
            // 7dba2 lea rdi,[rbp-0x590]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa70);
            // 7dba9 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dbae => {
            // block 7dbae
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dbae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dbae mov rbx,rax
            rbx = rax;
            // 7dbb1 lea rdi,[rbp-0x650]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9b0);
            // 7dbb8 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dbbd => {
            // block 7dbbd
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dbbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dbbd mov rbx,rax
            rbx = rax;
            // 7dbc0 lea rdi,[rbp-0x710]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8f0);
            // 7dbc7 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dbcc => {
            // block 7dbcc
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dbcc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dbcc mov rbx,rax
            rbx = rax;
            // 7dbcf lea rdi,[rbp-0x7d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f830);
            // 7dbd6 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dbdb => {
            // block 7dbdb
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dbdb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dbdb mov rbx,rax
            rbx = rax;
            // 7dbde lea rdi,[rbp-0x890]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f770);
            // 7dbe5 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dbea => {
            // block 7dbea
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dbea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dbea mov rbx,rax
            rbx = rax;
            // 7dbed lea rdi,[rbp-0x950]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6b0);
            // 7dbf4 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dbf9 => {
            // block 7dbf9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dbf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dbf9 mov rbx,rax
            rbx = rax;
            // 7dbfc lea rdi,[rbp-0xa10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5f0);
            // 7dc03 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dc08 => {
            // block 7dc08
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dc08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc08 mov rbx,rax
            rbx = rax;
            // 7dc0b lea rdi,[rbp-0xad0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f530);
            // 7dc12 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dc17 => {
            // block 7dc17
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dc17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc17 mov rbx,rax
            rbx = rax;
            // 7dc1a lea rdi,[rbp-0xb90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f470);
            // 7dc21 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dc26 => {
            // block 7dc26
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dc26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc26 mov rbx,rax
            rbx = rax;
            // 7dc29 lea rdi,[rbp-0xc50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3b0);
            // 7dc30 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dc35 => {
            // block 7dc35
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dc35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc35 mov rbx,rax
            rbx = rax;
            // 7dc38 lea rdi,[rbp-0xd10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2f0);
            // 7dc3f jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dc44 => {
            // block 7dc44
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dc44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc44 mov rbx,rax
            rbx = rax;
            // 7dc47 lea rdi,[rbp-0xdd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f230);
            // 7dc4e jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dc53 => {
            // block 7dc53
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dc53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc53 mov rbx,rax
            rbx = rax;
            // 7dc56 lea rdi,[rbp-0xe90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f170);
            // 7dc5d jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dc62 => {
            // block 7dc62
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dc62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc62 mov rbx,rax
            rbx = rax;
            // 7dc65 lea rdi,[rbp-0xf50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0b0);
            // 7dc6c jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dc71 => {
            // block 7dc71
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dc71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc71 mov rbx,rax
            rbx = rax;
            // 7dc74 lea rdi,[rbp-0x1010]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eff0);
            // 7dc7b jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dc80 => {
            // block 7dc80
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dc80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc80 mov rbx,rax
            rbx = rax;
            // 7dc83 lea rdi,[rbp-0x10d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef30);
            // 7dc8a jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dc8f => {
            // block 7dc8f
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dc8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc8f mov rbx,rax
            rbx = rax;
            // 7dc92 lea rdi,[rbp-0x1190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee70);
            // 7dc99 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dc9e => {
            // block 7dc9e
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dc9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc9e mov rbx,rax
            rbx = rax;
            // 7dca1 lea rdi,[rbp-0x1250]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edb0);
            // 7dca8 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dcad => {
            // block 7dcad
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dcad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dcad mov rbx,rax
            rbx = rax;
            // 7dcb0 lea rdi,[rbp-0x1310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecf0);
            // 7dcb7 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dcbc => {
            // block 7dcbc
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dcbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dcbc mov rbx,rax
            rbx = rax;
            // 7dcbf lea rdi,[rbp-0x13d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec30);
            // 7dcc6 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dccb => {
            // block 7dccb
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dccb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dccb mov rbx,rax
            rbx = rax;
            // 7dcce lea rdi,[rbp-0x1490]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb70);
            // 7dcd5 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dcda => {
            // block 7dcda
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dcda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dcda mov rbx,rax
            rbx = rax;
            // 7dcdd lea rdi,[rbp-0x1550]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eab0);
            // 7dce4 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dce9 => {
            // block 7dce9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dce9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dce9 mov rbx,rax
            rbx = rax;
            // 7dcec lea rdi,[rbp-0x1610]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9f0);
            // 7dcf3 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dcf8 => {
            // block 7dcf8
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dcf8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dcf8 mov rbx,rax
            rbx = rax;
            // 7dcfb lea rdi,[rbp-0x16d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e930);
            // 7dd02 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dd07 => {
            // block 7dd07
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dd07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dd07 mov rbx,rax
            rbx = rax;
            // 7dd0a lea rdi,[rbp-0x1790]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e870);
            // 7dd11 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dd16 => {
            // block 7dd16
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dd16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dd16 mov rbx,rax
            rbx = rax;
            // 7dd19 lea rdi,[rbp-0x1850]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7b0);
            // 7dd20 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dd25 => {
            // block 7dd25
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dd25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dd25 mov rbx,rax
            rbx = rax;
            // 7dd28 lea rdi,[rbp-0x1910]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6f0);
            // 7dd2f jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dd34 => {
            // block 7dd34
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dd34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dd34 mov rbx,rax
            rbx = rax;
            // 7dd37 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 7dd3e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7dd43 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 7dd4a jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dd4f => {
            // block 7dd4f
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dd4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dd4f mov rbx,rax
            rbx = rax;
            // 7dd52 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 7dd59 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7dd5e lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 7dd65 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dd6a => {
            // block 7dd6a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dd6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dd6a mov rbx,rax
            rbx = rax;
            // 7dd6d lea rdi,[rbp-0x208]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf8);
            // 7dd74 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7dd79 lea rdi,[rbp-0x220]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde0);
            // 7dd80 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dd85 => {
            // block 7dd85
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dd85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dd85 mov rbx,rax
            rbx = rax;
            // 7dd88 lea rdi,[rbp-0x2c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 7dd8f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7dd94 lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 7dd9b jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dda0 => {
            // block 7dda0
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dda0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dda0 mov rbx,rax
            rbx = rax;
            // 7dda3 lea rdi,[rbp-0x388]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc78);
            // 7ddaa call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ddaf lea rdi,[rbp-0x3a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc60);
            // 7ddb6 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7ddbb => {
            // block 7ddbb
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ddbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ddbb mov rbx,rax
            rbx = rax;
            // 7ddbe lea rdi,[rbp-0x448]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb8);
            // 7ddc5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ddca lea rdi,[rbp-0x460]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fba0);
            // 7ddd1 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7ddd6 => {
            // block 7ddd6
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ddd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ddd6 mov rbx,rax
            rbx = rax;
            // 7ddd9 lea rdi,[rbp-0x518]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fae8);
            // 7dde0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7dde5 lea rdi,[rbp-0x530]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fad0);
            // 7ddec jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7ddf1 => {
            // block 7ddf1
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7ddf1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ddf1 mov rbx,rax
            rbx = rax;
            // 7ddf4 lea rdi,[rbp-0x5d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa28);
            // 7ddfb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7de00 lea rdi,[rbp-0x5f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa10);
            // 7de07 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7de0c => {
            // block 7de0c
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7de0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7de0c mov rbx,rax
            rbx = rax;
            // 7de0f lea rdi,[rbp-0x698]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f968);
            // 7de16 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7de1b lea rdi,[rbp-0x6b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f950);
            // 7de22 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7de27 => {
            // block 7de27
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7de27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7de27 mov rbx,rax
            rbx = rax;
            // 7de2a lea rdi,[rbp-0x758]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8a8);
            // 7de31 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7de36 lea rdi,[rbp-0x770]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f890);
            // 7de3d jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7de42 => {
            // block 7de42
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7de42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7de42 mov rbx,rax
            rbx = rax;
            // 7de45 lea rdi,[rbp-0x818]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7e8);
            // 7de4c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7de51 lea rdi,[rbp-0x830]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7d0);
            // 7de58 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7de5d => {
            // block 7de5d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7de5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7de5d mov rbx,rax
            rbx = rax;
            // 7de60 lea rdi,[rbp-0x8d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f728);
            // 7de67 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7de6c lea rdi,[rbp-0x8f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f710);
            // 7de73 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7de78 => {
            // block 7de78
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7de78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7de78 mov rbx,rax
            rbx = rax;
            // 7de7b lea rdi,[rbp-0x998]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f668);
            // 7de82 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7de87 lea rdi,[rbp-0x9b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f650);
            // 7de8e jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7de93 => {
            // block 7de93
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7de93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7de93 mov rbx,rax
            rbx = rax;
            // 7de96 lea rdi,[rbp-0xa58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5a8);
            // 7de9d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7dea2 lea rdi,[rbp-0xa70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f590);
            // 7dea9 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7deae => {
            // block 7deae
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7deae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7deae mov rbx,rax
            rbx = rax;
            // 7deb1 lea rdi,[rbp-0xb18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4e8);
            // 7deb8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7debd lea rdi,[rbp-0xb30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4d0);
            // 7dec4 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dec9 => {
            // block 7dec9
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dec9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dec9 mov rbx,rax
            rbx = rax;
            // 7decc lea rdi,[rbp-0xbd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f428);
            // 7ded3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ded8 lea rdi,[rbp-0xbf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f410);
            // 7dedf jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dee4 => {
            // block 7dee4
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dee4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dee4 mov rbx,rax
            rbx = rax;
            // 7dee7 lea rdi,[rbp-0xc98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f368);
            // 7deee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7def3 lea rdi,[rbp-0xcb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f350);
            // 7defa jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7deff => {
            // block 7deff
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7deff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7deff mov rbx,rax
            rbx = rax;
            // 7df02 lea rdi,[rbp-0xd58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2a8);
            // 7df09 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7df0e lea rdi,[rbp-0xd70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f290);
            // 7df15 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7df1a => {
            // block 7df1a
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7df1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7df1a mov rbx,rax
            rbx = rax;
            // 7df1d lea rdi,[rbp-0xe18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1e8);
            // 7df24 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7df29 lea rdi,[rbp-0xe30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1d0);
            // 7df30 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7df35 => {
            // block 7df35
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7df35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7df35 mov rbx,rax
            rbx = rax;
            // 7df38 lea rdi,[rbp-0xed8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f128);
            // 7df3f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7df44 lea rdi,[rbp-0xef0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f110);
            // 7df4b jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7df50 => {
            // block 7df50
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7df50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7df50 mov rbx,rax
            rbx = rax;
            // 7df53 lea rdi,[rbp-0xf98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f068);
            // 7df5a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7df5f lea rdi,[rbp-0xfb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f050);
            // 7df66 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7df6b => {
            // block 7df6b
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7df6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7df6b mov rbx,rax
            rbx = rax;
            // 7df6e lea rdi,[rbp-0x1058]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efa8);
            // 7df75 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7df7a lea rdi,[rbp-0x1070]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef90);
            // 7df81 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7df86 => {
            // block 7df86
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7df86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7df86 mov rbx,rax
            rbx = rax;
            // 7df89 lea rdi,[rbp-0x1118]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eee8);
            // 7df90 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7df95 lea rdi,[rbp-0x1130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eed0);
            // 7df9c jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dfa1 => {
            // block 7dfa1
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dfa1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dfa1 mov rbx,rax
            rbx = rax;
            // 7dfa4 lea rdi,[rbp-0x11d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee28);
            // 7dfab call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7dfb0 lea rdi,[rbp-0x11f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee10);
            // 7dfb7 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dfbc => {
            // block 7dfbc
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dfbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dfbc mov rbx,rax
            rbx = rax;
            // 7dfbf lea rdi,[rbp-0x1298]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed68);
            // 7dfc6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7dfcb lea rdi,[rbp-0x12b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed50);
            // 7dfd2 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dfd7 => {
            // block 7dfd7
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dfd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dfd7 mov rbx,rax
            rbx = rax;
            // 7dfda lea rdi,[rbp-0x1358]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eca8);
            // 7dfe1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7dfe6 lea rdi,[rbp-0x1370]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec90);
            // 7dfed jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7dff2 => {
            // block 7dff2
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7dff2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dff2 mov rbx,rax
            rbx = rax;
            // 7dff5 lea rdi,[rbp-0x1418]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebe8);
            // 7dffc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e001 lea rdi,[rbp-0x1430]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebd0);
            // 7e008 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7e00d => {
            // block 7e00d
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7e00d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e00d mov rbx,rax
            rbx = rax;
            // 7e010 lea rdi,[rbp-0x14d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb28);
            // 7e017 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e01c lea rdi,[rbp-0x14f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb10);
            // 7e023 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7e028 => {
            // block 7e028
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7e028 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e028 mov rbx,rax
            rbx = rax;
            // 7e02b lea rdi,[rbp-0x1598]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea68);
            // 7e032 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e037 lea rdi,[rbp-0x15b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea50);
            // 7e03e jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7e040 => {
            // block 7e040
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7e040 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e040 mov rbx,rax
            rbx = rax;
            // 7e043 lea rdi,[rbp-0x1658]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9a8);
            // 7e04a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e04f lea rdi,[rbp-0x1670]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e990);
            // 7e056 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7e058 => {
            // block 7e058
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7e058 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e058 mov rbx,rax
            rbx = rax;
            // 7e05b lea rdi,[rbp-0x1718]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8e8);
            // 7e062 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e067 lea rdi,[rbp-0x1730]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8d0);
            // 7e06e jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7e070 => {
            // block 7e070
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7e070 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e070 mov rbx,rax
            rbx = rax;
            // 7e073 lea rdi,[rbp-0x17d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e828);
            // 7e07a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e07f lea rdi,[rbp-0x17f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e810);
            // 7e086 jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7e088 => {
            // block 7e088
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7e088 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e088 mov rbx,rax
            rbx = rax;
            // 7e08b lea rdi,[rbp-0x1898]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e768);
            // 7e092 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e097 lea rdi,[rbp-0x18b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e750);
            // 7e09e jmp 0x7e0b6
            pc = 0x7e0b6; continue;
        }
        0x7e0a0 => {
            // block 7e0a0
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7e0a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e0a0 mov rbx,rax
            rbx = rax;
            // 7e0a3 lea rdi,[rbp-0x1958]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6a8);
            // 7e0aa call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e0af lea rdi,[rbp-0x1970]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e690);
            pc = 0x7e0b6;
        }
        0x7e0b6 => {
            // block 7e0b6
            #[cfg(feature = "trace")] eprintln!("f_000794b2 pc 7e0b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e0b6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7e0bb mov rdi,rbx
            rdi = rbx;
            // 7e0be call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
