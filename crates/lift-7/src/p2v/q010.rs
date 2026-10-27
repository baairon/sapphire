//! Mechanically lifted functions (scripts/lift_run.py), module q010. Each one is checked against the original
//! in the world harness (tests/world_q010.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x506e4 size=1600
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000506e4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x506e4;
    loop {
        match pc {
        0x506e4 => {
            // block 506e4
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 506e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 506e4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 506e5 mov rbp,rsp
            rbp = rsp;
            // 506e8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 506ea push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 506ec push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 506ee push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 506f0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 506f1 sub rsp,0x288
            { let (r, f) = sub::<64>(rsp, 0x288u64);
              rsp = r;
              fl = f; }
            // 506f8 mov r15,rdx
            r15 = rdx;
            // 506fb mov r12,rsi
            r12 = rsi;
            // 506fe mov r14,rdi
            r14 = rdi;
            // 50701 mov rbx,QWORD PTR [rip+0xefcd0] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 50708 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 5070b mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 5070f mov QWORD PTR [rbp-0x158],0x100
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (0x100u64) as u64);
            // 5071a mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 5071d xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5071f call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50725 mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 5072b test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 5072d je 0x50767
            if fl.e() { pc = 0x50767; continue; }
            pc = 0x5072f;
        }
        0x5072f => {
            // block 5072f
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 5072f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5072f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 50732 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 50734 mov rdi,r14
            rdi = r14;
            // 50737 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5073d lea r8,[rbp-0x130]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 50744 lea r9,[rbp-0x158]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 5074b mov esi,0x14
            rsi = (0x14u64) as u32 as u64;
            // 50750 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50752 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 50754 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 50756 call 0xebf6c
            { let r = { let t = m.env.stub_va(0x739); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5075b mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 5075e test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 50761 je 0x50965
            if fl.e() { pc = 0x50965; continue; }
            pc = 0x50767;
        }
        0x50767 => {
            // block 50767
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50767 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50767 test BYTE PTR [rip+0x177b24],0x10 # 0x1c8292
            fl = logic::<8>(m.r8(m.env.data_va(903775)) as u64 & 0x10u64);
            // 5076e jne 0x50777
            if fl.ne() { pc = 0x50777; continue; }
            pc = 0x50770;
        }
        0x50770 => {
            // block 50770
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50770 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50770 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 50772 jmp 0x50c5e
            pc = 0x50c5e; continue;
        }
        0x50777 => {
            // block 50777
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50777 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50777 lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 5077e lea rbx,[rbp-0x258]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fda8);
            // 50785 mov rax,QWORD PTR [rip+0xefa44] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 5078c lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 50790 mov QWORD PTR [rbp-0x260],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda0), (rcx) as u64);
            // 50797 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 5079b mov QWORD PTR [rbp-0x1f0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (rax) as u64);
            // 507a2 mov rsi,rbx
            rsi = rbx;
            // 507a5 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 507aa mov QWORD PTR [rbp-0x168],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (0x0u64) as u64);
            // 507b5 mov DWORD PTR [rbp-0x160],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (0xffffffffu64) as u32);
            // 507bf mov r12,QWORD PTR [rip+0xefbfa] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 507c6 lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 507cb mov QWORD PTR [rbp-0x298],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd68), (rax) as u64);
            // 507d2 mov QWORD PTR [rbp-0x260],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda0), (rax) as u64);
            // 507d9 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 507dd mov QWORD PTR [rbp-0x1f0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (r12) as u64);
            // 507e4 mov rdi,rbx
            rdi = rbx;
            // 507e7 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 507ec mov rax,QWORD PTR [rip+0xefbb5] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 507f3 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 507f7 mov QWORD PTR [rbp-0x2a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd60), (rax) as u64);
            // 507fe mov QWORD PTR [rbp-0x258],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda8), (rax) as u64);
            // 50805 mov QWORD PTR [rbp-0x200],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe00), (0x0u64) as u64);
            // 50810 mov QWORD PTR [rbp-0x208],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdf8), (0x0u64) as u64);
            // 5081b mov QWORD PTR [rbp-0x210],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdf0), (0x0u64) as u64);
            // 50826 mov QWORD PTR [rbp-0x218],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fde8), (0x0u64) as u64);
            // 50831 mov DWORD PTR [rbp-0x1f8],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe08), (0x10u64) as u32);
            // 5083b xorps xmm0,xmm0
            x0 = 0;
            // 5083e movaps XMMWORD PTR [rbp-0x150],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_feb0), x0);
            // 50845 mov QWORD PTR [rbp-0x140],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (0x0u64) as u64);
            // 50850 lea rsi,[rbp-0x150]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 50857 mov rdi,rbx
            rdi = rbx;
            // 5085a call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5085f lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 50866 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5086b call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50870 mov r15,rax
            r15 = rax;
            // 50873 lea rsi,[rip+0xda447] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 5087a lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 50881 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 50886 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5088b lea rsi,[rip+0xda433] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 50892 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 50897 mov rdi,rax
            rdi = rax;
            // 5089a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5089f lea rsi,[rip+0xda0da] # 0x12a980
            rsi = m.env.data_va(258381);
            // 508a6 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 508ab mov rdi,rax
            rdi = rax;
            // 508ae call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 508b3 lea rsi,[rip+0xd9bbb] # 0x12a475
            rsi = m.env.data_va(257090);
            // 508ba mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 508bf mov rdi,rax
            rdi = rax;
            // 508c2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 508c7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 508ca mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 508ce mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 508d2 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 508d5 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 508d8 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 508dc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 508df mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 508e3 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 508ec mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 508ef mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 508f3 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 508fe mov rdi,rax
            rdi = rax;
            // 50901 mov rsi,r15
            rsi = r15;
            // 50904 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50909 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 5090c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50910 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 50914 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50917 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 5091a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 5091e lea rsi,[rip+0xda05b] # 0x12a980
            rsi = m.env.data_va(258381);
            // 50925 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 5092a mov rdi,rax
            rdi = rax;
            // 5092d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50932 mov rbx,rax
            rbx = rax;
            // 50935 lea rsi,[rip+0xda660] # 0x12af9c
            rsi = m.env.data_va(259945);
            // 5093c lea rdi,[rbp-0x278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd88);
            // 50943 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 50948 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5094d movzx edx,BYTE PTR [rbp-0x278]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd88)) as u64) as u32 as u64;
            // 50954 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 50957 jne 0x50994
            if fl.ne() { pc = 0x50994; continue; }
            pc = 0x50959;
        }
        0x50959 => {
            // block 50959
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50959 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50959 lea rsi,[rbp-0x277]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd89);
            // 50960 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 50963 jmp 0x509a2
            pc = 0x509a2; continue;
        }
        0x50965 => {
            // block 50965
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50965 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50965 lea rsi,[rbp-0x130]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 5096c mov edx,0x80
            rdx = (0x80u64) as u32 as u64;
            // 50971 mov rdi,r12
            rdi = r12;
            // 50974 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50979 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 50980 mov edx,0x80
            rdx = (0x80u64) as u32 as u64;
            // 50985 mov rdi,r15
            rdi = r15;
            // 50988 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5098d mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 5098f jmp 0x50c5e
            pc = 0x50c5e; continue;
        }
        0x50994 => {
            // block 50994
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50994 mov rdx,QWORD PTR [rbp-0x270]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd90)) as u64;
            // 5099b mov rsi,QWORD PTR [rbp-0x268]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd98)) as u64;
            pc = 0x509a2;
        }
        0x509a2 => {
            // block 509a2
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 509a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 509a2 mov rdi,rbx
            rdi = rbx;
            // 509a5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 509aa lea rsi,[rip+0xd9fcf] # 0x12a980
            rsi = m.env.data_va(258381);
            // 509b1 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 509b6 mov rdi,rax
            rdi = rax;
            // 509b9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 509be lea rsi,[rip+0xda3ab] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 509c5 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 509ca mov rdi,rax
            rdi = rax;
            // 509cd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 509d2 lea rsi,[rip+0xd9a9c] # 0x12a475
            rsi = m.env.data_va(257090);
            // 509d9 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 509de mov rdi,rax
            rdi = rax;
            // 509e1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 509e6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 509e9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 509ed mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 509f1 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 509f4 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 509f7 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 509fb mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 509fe mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50a02 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 50a0b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 50a0e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50a12 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 50a1d mov rdi,rax
            rdi = rax;
            // 50a20 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 50a23 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50a28 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 50a2b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50a2f mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 50a33 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50a36 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50a39 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 50a3d lea rsi,[rip+0xd9f42] # 0x12a986
            rsi = m.env.data_va(258387);
            // 50a44 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 50a49 mov rdi,rax
            rdi = rax;
            // 50a4c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50a51 mov r15,rax
            r15 = rax;
            // 50a54 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 50a57 call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50a5c mov rbx,rax
            rbx = rax;
            // 50a5f mov rdi,rbx
            rdi = rbx;
            // 50a62 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50a67 mov rdi,r15
            rdi = r15;
            // 50a6a mov rsi,rbx
            rsi = rbx;
            // 50a6d mov rdx,rax
            rdx = rax;
            // 50a70 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50a75 lea rsi,[rip+0xd9c06] # 0x12a682
            rsi = m.env.data_va(257615);
            // 50a7c mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 50a81 mov rdi,rax
            rdi = rax;
            // 50a84 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50a89 lea rsi,[rip+0xda26a] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 50a90 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 50a95 mov rdi,rax
            rdi = rax;
            // 50a98 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50a9d lea rsi,[rip+0xd99d1] # 0x12a475
            rsi = m.env.data_va(257090);
            // 50aa4 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 50aa9 mov rdi,rax
            rdi = rax;
            // 50aac call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50ab1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 50ab4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50ab8 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 50abc and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50abf or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50ac2 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 50ac6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 50ac9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50acd mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 50ad6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 50ad9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50add mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 50ae8 mov rdi,rax
            rdi = rax;
            // 50aeb mov rsi,r14
            rsi = r14;
            // 50aee call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50af3 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 50af6 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50afa mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 50afe and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50b01 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50b04 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 50b08 lea rsi,[rip+0xda1fd] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 50b0f mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 50b14 mov rdi,rax
            rdi = rax;
            // 50b17 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50b1c lea rsi,[rip+0xd9952] # 0x12a475
            rsi = m.env.data_va(257090);
            // 50b23 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 50b28 mov rdi,rax
            rdi = rax;
            // 50b2b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50b30 mov rbx,rax
            rbx = rax;
            // 50b33 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 50b36 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50b3a mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 50b3e and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 50b41 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 50b44 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 50b48 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 50b4b mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50b4f mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 50b58 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 50b5b mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50b5f mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 50b6a mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 50b6d mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 50b74 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 50b76 mov rdi,r14
            rdi = r14;
            // 50b79 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50b7b mov rdi,rbx
            rdi = rbx;
            // 50b7e mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 50b80 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50b85 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 50b88 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 50b8c mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 50b90 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50b93 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 50b96 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 50b9a lea rdi,[rbp-0x278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd88);
            // 50ba1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50ba6 lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // 50bad lea rsi,[rbp-0x258]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fda8);
            // 50bb4 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50bb9 test BYTE PTR [rbp-0x290],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd70)) as u64 & 0x1u64);
            // 50bc0 jne 0x50bcb
            if fl.ne() { pc = 0x50bcb; continue; }
            pc = 0x50bc2;
        }
        0x50bc2 => {
            // block 50bc2
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50bc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50bc2 lea rdx,[rbp-0x28f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fd71);
            // 50bc9 jmp 0x50bd2
            pc = 0x50bd2; continue;
        }
        0x50bcb => {
            // block 50bcb
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50bcb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50bcb mov rdx,QWORD PTR [rbp-0x280]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd80)) as u64;
            pc = 0x50bd2;
        }
        0x50bd2 => {
            // block 50bd2
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50bd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50bd2 lea rsi,[rip+0xda140] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 50bd9 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 50bde xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 50be0 lea rbx,[rbp-0x258]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fda8);
            // 50be7 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50bec lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // 50bf3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50bf8 mov rax,QWORD PTR [rbp-0x298]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd68)) as u64;
            // 50bff mov QWORD PTR [rbp-0x260],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda0), (rax) as u64);
            // 50c06 mov QWORD PTR [rbp-0x1f0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (r12) as u64);
            // 50c0d mov rax,QWORD PTR [rbp-0x2a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd60)) as u64;
            // 50c14 mov QWORD PTR [rbp-0x258],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda8), (rax) as u64);
            // 50c1b lea rdi,[rbp-0x218]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde8);
            // 50c22 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50c27 mov rdi,rbx
            rdi = rbx;
            // 50c2a call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50c2f mov rsi,QWORD PTR [rip+0xef5d2] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 50c36 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 50c3a lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 50c41 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50c46 lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 50c4d call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50c52 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 50c54 mov rbx,QWORD PTR [rip+0xef77d] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 50c5b mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            pc = 0x50c5e;
        }
        0x50c5e => {
            // block 50c5e
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50c5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50c5e cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 50c62 jne 0x50cf4
            if fl.ne() { pc = 0x50cf4; continue; }
            pc = 0x50c68;
        }
        0x50c68 => {
            // block 50c68
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50c68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50c68 add rsp,0x288
            { let (r, f) = add::<64>(rsp, 0x288u64);
              rsp = r;
              fl = f; }
            // 50c6f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 50c70 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 50c72 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 50c74 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 50c76 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 50c78 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 50c79 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x50c7a => {
            // block 50c7a
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50c7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50c7a mov r14,rax
            r14 = rax;
            // 50c7d lea rdi,[rbp-0x278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd88);
            pc = 0x50c84;
        }
        0x50c84 => {
            // block 50c84
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50c84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50c84 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50c89 jmp 0x50c8e
            pc = 0x50c8e; continue;
        }
        0x50c8b => {
            // block 50c8b
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50c8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50c8b mov r14,rax
            r14 = rax;
            pc = 0x50c8e;
        }
        0x50c8e => {
            // block 50c8e
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50c8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50c8e mov rax,QWORD PTR [rbp-0x298]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd68)) as u64;
            // 50c95 mov QWORD PTR [rbp-0x260],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda0), (rax) as u64);
            // 50c9c mov QWORD PTR [rbp-0x1f0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (r12) as u64);
            // 50ca3 mov rax,QWORD PTR [rbp-0x2a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd60)) as u64;
            // 50caa mov QWORD PTR [rbp-0x258],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda8), (rax) as u64);
            pc = 0x50cb1;
        }
        0x50cb1 => {
            // block 50cb1
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50cb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50cb1 lea rdi,[rbp-0x218]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde8);
            // 50cb8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50cbd lea rdi,[rbp-0x258]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda8);
            // 50cc4 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x50cc9;
        }
        0x50cc9 => {
            // block 50cc9
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50cc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50cc9 mov rsi,QWORD PTR [rip+0xef538] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 50cd0 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 50cd4 lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 50cdb call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x50ce0;
        }
        0x50ce0 => {
            // block 50ce0
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50ce0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50ce0 lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 50ce7 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50cec mov rdi,r14
            rdi = r14;
            // 50cef call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x50cf4;
        }
        0x50cf4 => {
            // block 50cf4
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50cf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50cf4 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x50cfe => {
            // block 50cfe
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50cfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50cfe mov r14,rax
            r14 = rax;
            // 50d01 jmp 0x50cc9
            pc = 0x50cc9; continue;
        }
        0x50d03 => {
            // block 50d03
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50d03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50d03 mov r14,rax
            r14 = rax;
            // 50d06 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 50d0d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50d12 jmp 0x50cb1
            pc = 0x50cb1; continue;
        }
        0x50d14 => {
            // block 50d14
            #[cfg(feature = "trace")] eprintln!("f_000506e4 pc 50d14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50d14 mov r14,rax
            r14 = rax;
            // 50d17 lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // 50d1e jmp 0x50c84
            pc = 0x50c84; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4e8a2 size=1554
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004e8a2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4e8a2;
    loop {
        match pc {
        0x4e8a2 => {
            // block 4e8a2
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4e8a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e8a2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4e8a3 mov rbp,rsp
            rbp = rsp;
            // 4e8a6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4e8a8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4e8aa push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4e8ac push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4e8ae push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4e8af sub rsp,0x1a8
            { let (r, f) = sub::<64>(rsp, 0x1a8u64);
              rsp = r;
              fl = f; }
            // 4e8b6 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 4e8b8 mov r14,rdi
            r14 = rdi;
            // 4e8bb mov r10d,DWORD PTR [rbp+0x28]
            r10 = (m.r32(rbp.wrapping_add(0x28)) as u64) as u32 as u64;
            // 4e8bf mov r11d,DWORD PTR [rbp+0x20]
            r11 = (m.r32(rbp.wrapping_add(0x20)) as u64) as u32 as u64;
            // 4e8c3 mov edi,DWORD PTR [rbp+0x18]
            rdi = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // 4e8c6 mov eax,DWORD PTR [rbp+0x10]
            rax = (m.r32(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // 4e8c9 mov QWORD PTR [rbp-0x48],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0x0u64) as u64);
            // 4e8d1 mov DWORD PTR [rbp-0x78],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff88), ((rsi as u32 as u64)) as u32);
            // 4e8d4 mov DWORD PTR [rbp-0x74],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c), (0x0u64) as u32);
            // 4e8db mov QWORD PTR [rbp-0x70],r8
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r8) as u64);
            // 4e8df mov DWORD PTR [rbp-0x68],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), ((rax as u32 as u64)) as u32);
            // 4e8e2 mov DWORD PTR [rbp-0x64],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), ((rcx as u32 as u64)) as u32);
            // 4e8e5 mov DWORD PTR [rbp-0x60],r9d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), ((r9 as u32 as u64)) as u32);
            // 4e8e9 mov DWORD PTR [rbp-0x5c],edi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rdi as u32 as u64)) as u32);
            // 4e8ec xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4e8ee div edi
            { let (q, r) = div::<32>(rdx, rax, (rdi as u32 as u64)); rax = q; rdx = r; }
            // 4e8f0 mov DWORD PTR [rbp-0x58],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((rax as u32 as u64)) as u32);
            // 4e8f3 mov DWORD PTR [rbp-0x54],r11d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((r11 as u32 as u64)) as u32);
            // 4e8f7 mov DWORD PTR [rbp-0x50],r10d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), ((r10 as u32 as u64)) as u32);
            // 4e8fb movzx eax,bl
            rax = ((rbx as u8 as u64)) as u32 as u64;
            // 4e8fe xor eax,0x1
            { let r = (rax as u32 as u64) ^ 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4e901 mov DWORD PTR [rbp-0x4c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), ((rax as u32 as u64)) as u32);
            // 4e904 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4e907 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4e909 mov rdi,r14
            rdi = r14;
            // 4e90c call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e912 mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 4e918 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4e91a je 0x4e954
            if fl.e() { pc = 0x4e954; continue; }
            pc = 0x4e91c;
        }
        0x4e91c => {
            // block 4e91c
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4e91c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e91c mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4e91f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4e921 mov rdi,r14
            rdi = r14;
            // 4e924 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e92a lea rdx,[rbp-0x78]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 4e92e lea r9,[rbp-0x48]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 4e932 mov esi,0x1d
            rsi = (0x1du64) as u32 as u64;
            // 4e937 mov ecx,0x30
            rcx = (0x30u64) as u32 as u64;
            // 4e93c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4e93f mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4e941 call 0xebf6c
            { let r = { let t = m.env.stub_va(0x739); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e946 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 4e949 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4e94b test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 4e94e je 0x4ee00
            if fl.e() { pc = 0x4ee00; continue; }
            pc = 0x4e954;
        }
        0x4e954 => {
            // block 4e954
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4e954 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e954 test BYTE PTR [rip+0x179938],0x20 # 0x1c8293
            fl = logic::<8>(m.r8(m.env.data_va(903776)) as u64 & 0x20u64);
            // 4e95b je 0x4edfe
            if fl.e() { pc = 0x4edfe; continue; }
            pc = 0x4e961;
        }
        0x4e961 => {
            // block 4e961
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4e961 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e961 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 4e968 lea rbx,[rbp-0x178]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 4e96f mov rax,QWORD PTR [rip+0xf185a] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 4e976 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4e97a mov QWORD PTR [rbp-0x180],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rcx) as u64);
            // 4e981 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4e985 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 4e98c mov rsi,rbx
            rsi = rbx;
            // 4e98f call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e994 mov QWORD PTR [rbp-0x88],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (0x0u64) as u64);
            // 4e99f mov DWORD PTR [rbp-0x80],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0xffffffffu64) as u32);
            // 4e9a6 mov r12,QWORD PTR [rip+0xf1a13] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 4e9ad lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 4e9b2 mov QWORD PTR [rbp-0x1b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe48), (rax) as u64);
            // 4e9b9 mov QWORD PTR [rbp-0x180],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rax) as u64);
            // 4e9c0 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 4e9c4 mov QWORD PTR [rbp-0x110],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (r12) as u64);
            // 4e9cb mov rdi,rbx
            rdi = rbx;
            // 4e9ce call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e9d3 mov rax,QWORD PTR [rip+0xf19ce] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4e9da add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 4e9de mov QWORD PTR [rbp-0x1c0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe40), (rax) as u64);
            // 4e9e5 mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            // 4e9ec mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 4e9f7 mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 4ea02 mov QWORD PTR [rbp-0x130],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (0x0u64) as u64);
            // 4ea0d mov QWORD PTR [rbp-0x138],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (0x0u64) as u64);
            // 4ea18 mov DWORD PTR [rbp-0x118],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x10u64) as u32);
            // 4ea22 xorps xmm0,xmm0
            x0 = 0;
            // 4ea25 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 4ea29 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 4ea31 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4ea35 mov rdi,rbx
            rdi = rbx;
            // 4ea38 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ea3d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4ea41 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ea46 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ea4b mov r15,rax
            r15 = rax;
            // 4ea4e lea rsi,[rip+0xdc26c] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4ea55 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 4ea5c mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4ea61 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ea66 lea rsi,[rip+0xdc258] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 4ea6d mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 4ea72 mov rdi,rax
            rdi = rax;
            // 4ea75 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ea7a lea rsi,[rip+0xdbeff] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4ea81 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4ea86 mov rdi,rax
            rdi = rax;
            // 4ea89 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ea8e lea rsi,[rip+0xdb9e0] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4ea95 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4ea9a mov rdi,rax
            rdi = rax;
            // 4ea9d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4eaa2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4eaa5 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4eaa9 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4eaad and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4eab0 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4eab3 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4eab7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4eaba mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4eabe mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4eac7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4eaca mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4eace mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4ead9 mov rdi,rax
            rdi = rax;
            // 4eadc mov rsi,r15
            rsi = r15;
            // 4eadf call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4eae4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4eae7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4eaeb mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4eaef and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4eaf2 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4eaf5 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4eaf9 lea rsi,[rip+0xdbe80] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4eb00 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4eb05 mov rdi,rax
            rdi = rax;
            // 4eb08 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4eb0d mov rbx,rax
            rbx = rax;
            // 4eb10 lea rsi,[rip+0xdc40c] # 0x12af23
            rsi = m.env.data_va(259824);
            // 4eb17 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // 4eb1e mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4eb23 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4eb28 movzx edx,BYTE PTR [rbp-0x198]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64) as u32 as u64;
            // 4eb2f test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 4eb32 jne 0x4eb40
            if fl.ne() { pc = 0x4eb40; continue; }
            pc = 0x4eb34;
        }
        0x4eb34 => {
            // block 4eb34
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4eb34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4eb34 lea rsi,[rbp-0x197]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe69);
            // 4eb3b shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 4eb3e jmp 0x4eb4e
            pc = 0x4eb4e; continue;
        }
        0x4eb40 => {
            // block 4eb40
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4eb40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4eb40 mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // 4eb47 mov rsi,QWORD PTR [rbp-0x188]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            pc = 0x4eb4e;
        }
        0x4eb4e => {
            // block 4eb4e
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4eb4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4eb4e mov rdi,rbx
            rdi = rbx;
            // 4eb51 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4eb56 lea rsi,[rip+0xdbe23] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4eb5d mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4eb62 mov rdi,rax
            rdi = rax;
            // 4eb65 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4eb6a lea rsi,[rip+0xdc1ff] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 4eb71 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4eb76 mov rdi,rax
            rdi = rax;
            // 4eb79 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4eb7e lea rsi,[rip+0xdb8f0] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4eb85 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4eb8a mov rdi,rax
            rdi = rax;
            // 4eb8d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4eb92 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4eb95 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4eb99 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4eb9d and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4eba0 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4eba3 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4eba7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ebaa mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ebae mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4ebb7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ebba mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ebbe mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4ebc9 mov rdi,rax
            rdi = rax;
            // 4ebcc mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4ebcf call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ebd4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ebd7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ebdb mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4ebdf and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ebe2 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ebe5 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4ebe9 lea rsi,[rip+0xdbd96] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4ebf0 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4ebf5 mov rdi,rax
            rdi = rax;
            // 4ebf8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ebfd mov r15,rax
            r15 = rax;
            // 4ec00 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 4ec03 call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ec08 mov rbx,rax
            rbx = rax;
            // 4ec0b mov rdi,rbx
            rdi = rbx;
            // 4ec0e call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ec13 mov rdi,r15
            rdi = r15;
            // 4ec16 mov rsi,rbx
            rsi = rbx;
            // 4ec19 mov rdx,rax
            rdx = rax;
            // 4ec1c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ec21 lea rsi,[rip+0xdba5a] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4ec28 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4ec2d mov rdi,rax
            rdi = rax;
            // 4ec30 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ec35 lea rsi,[rip+0xdc0be] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 4ec3c mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4ec41 mov rdi,rax
            rdi = rax;
            // 4ec44 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ec49 lea rsi,[rip+0xdb825] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4ec50 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4ec55 mov rdi,rax
            rdi = rax;
            // 4ec58 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ec5d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ec60 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ec64 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4ec68 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ec6b or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ec6e mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4ec72 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ec75 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ec79 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4ec82 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ec85 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ec89 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4ec94 mov rdi,rax
            rdi = rax;
            // 4ec97 mov rsi,r14
            rsi = r14;
            // 4ec9a call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ec9f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4eca2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4eca6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4ecaa and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ecad or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ecb0 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4ecb4 lea rsi,[rip+0xdc051] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 4ecbb mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4ecc0 mov rdi,rax
            rdi = rax;
            // 4ecc3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ecc8 lea rsi,[rip+0xdb7a6] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4eccf mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4ecd4 mov rdi,rax
            rdi = rax;
            // 4ecd7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ecdc mov rbx,rax
            rbx = rax;
            // 4ecdf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4ece2 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ece6 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4ecea and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4eced or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4ecf0 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 4ecf4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4ecf7 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ecfb mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 4ed04 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4ed07 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ed0b mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 4ed16 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4ed19 mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 4ed20 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4ed22 mov rdi,r14
            rdi = r14;
            // 4ed25 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ed27 mov rdi,rbx
            rdi = rbx;
            // 4ed2a mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 4ed2c call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ed31 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ed34 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ed38 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4ed3c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ed3f or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ed42 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4ed46 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // 4ed4d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ed52 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // 4ed59 lea rsi,[rbp-0x178]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 4ed60 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ed65 test BYTE PTR [rbp-0x1b0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64 & 0x1u64);
            // 4ed6c jne 0x4ed77
            if fl.ne() { pc = 0x4ed77; continue; }
            pc = 0x4ed6e;
        }
        0x4ed6e => {
            // block 4ed6e
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4ed6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ed6e lea rdx,[rbp-0x1af]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe51);
            // 4ed75 jmp 0x4ed7e
            pc = 0x4ed7e; continue;
        }
        0x4ed77 => {
            // block 4ed77
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4ed77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ed77 mov rdx,QWORD PTR [rbp-0x1a0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            pc = 0x4ed7e;
        }
        0x4ed7e => {
            // block 4ed7e
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4ed7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ed7e lea rsi,[rip+0xdbf94] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 4ed85 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 4ed8a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4ed8c lea rbx,[rbp-0x178]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 4ed93 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ed98 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // 4ed9f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4eda4 mov rax,QWORD PTR [rbp-0x1b8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            // 4edab mov QWORD PTR [rbp-0x180],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rax) as u64);
            // 4edb2 mov QWORD PTR [rbp-0x110],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (r12) as u64);
            // 4edb9 mov rax,QWORD PTR [rbp-0x1c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // 4edc0 mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            // 4edc7 lea rdi,[rbp-0x138]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // 4edce call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4edd3 mov rdi,rbx
            rdi = rbx;
            // 4edd6 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4eddb mov rsi,QWORD PTR [rip+0xf1426] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4ede2 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4ede6 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 4eded call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4edf2 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 4edf9 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4edfe;
        }
        0x4edfe => {
            // block 4edfe
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4edfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4edfe xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4ee00;
        }
        0x4ee00 => {
            // block 4ee00
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4ee00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ee00 add rsp,0x1a8
            { let (r, f) = add::<64>(rsp, 0x1a8u64);
              rsp = r;
              fl = f; }
            // 4ee07 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4ee08 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4ee0a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4ee0c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4ee0e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4ee10 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4ee11 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4ee12 => {
            // block 4ee12
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4ee12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ee12 mov rbx,rax
            rbx = rax;
            // 4ee15 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            pc = 0x4ee1c;
        }
        0x4ee1c => {
            // block 4ee1c
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4ee1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ee1c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ee21 jmp 0x4ee26
            pc = 0x4ee26; continue;
        }
        0x4ee23 => {
            // block 4ee23
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4ee23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ee23 mov rbx,rax
            rbx = rax;
            pc = 0x4ee26;
        }
        0x4ee26 => {
            // block 4ee26
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4ee26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ee26 mov rax,QWORD PTR [rbp-0x1b8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            // 4ee2d mov QWORD PTR [rbp-0x180],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rax) as u64);
            // 4ee34 mov QWORD PTR [rbp-0x110],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (r12) as u64);
            // 4ee3b mov rax,QWORD PTR [rbp-0x1c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // 4ee42 mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            pc = 0x4ee49;
        }
        0x4ee49 => {
            // block 4ee49
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4ee49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ee49 lea rdi,[rbp-0x138]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // 4ee50 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ee55 lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 4ee5c call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4ee61;
        }
        0x4ee61 => {
            // block 4ee61
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4ee61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ee61 mov rsi,QWORD PTR [rip+0xf13a0] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4ee68 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4ee6c lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 4ee73 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4ee78;
        }
        0x4ee78 => {
            // block 4ee78
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4ee78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ee78 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 4ee7f call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ee84 mov rdi,rbx
            rdi = rbx;
            // 4ee87 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ee8c mov rbx,rax
            rbx = rax;
            // 4ee8f jmp 0x4ee78
            pc = 0x4ee78; continue;
        }
        0x4ee91 => {
            // block 4ee91
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4ee91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ee91 mov rbx,rax
            rbx = rax;
            // 4ee94 jmp 0x4ee61
            pc = 0x4ee61; continue;
        }
        0x4ee96 => {
            // block 4ee96
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4ee96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ee96 mov rbx,rax
            rbx = rax;
            // 4ee99 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4ee9d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4eea2 jmp 0x4ee49
            pc = 0x4ee49; continue;
        }
        0x4eea4 => {
            // block 4eea4
            #[cfg(feature = "trace")] eprintln!("f_0004e8a2 pc 4eea4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4eea4 mov rbx,rax
            rbx = rax;
            // 4eea7 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // 4eeae jmp 0x4ee1c
            pc = 0x4ee1c; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x46556 size=1536
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_00046556<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x46556;
    loop {
        match pc {
        0x46556 => {
            // block 46556
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46556 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46556 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 46557 mov rbp,rsp
            rbp = rsp;
            // 4655a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4655c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4655e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 46560 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 46562 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 46563 sub rsp,0x198
            { let (r, f) = sub::<64>(rsp, 0x198u64);
              rsp = r;
              fl = f; }
            // 4656a mov r15,rdx
            r15 = rdx;
            // 4656d mov r12,rsi
            r12 = rsi;
            // 46570 mov r14,rdi
            r14 = rdi;
            // 46573 mov rbx,QWORD PTR [rip+0xf9e5e] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4657a mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4657d mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 46581 mov DWORD PTR [rbp-0x64],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x2u64) as u32);
            // 46588 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4658b xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4658d call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46593 mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 46599 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4659b je 0x465de
            if fl.e() { pc = 0x465de; continue; }
            pc = 0x4659d;
        }
        0x4659d => {
            // block 4659d
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 4659d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4659d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 465a0 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 465a2 mov rdi,r14
            rdi = r14;
            // 465a5 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 465ab lea r8,[rbp-0x40]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 465af lea r9,[rbp-0x64]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 465b3 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 465b8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 465ba xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 465bc mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 465be call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 465c3 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 465c6 mov eax,DWORD PTR [rbp-0x40]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 465c9 mov ecx,DWORD PTR [rbp-0x38]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // 465cc mov DWORD PTR [r12],eax
            m.w32(r12, ((rax as u32 as u64)) as u32);
            // 465d0 mov DWORD PTR [r15],ecx
            m.w32(r15, ((rcx as u32 as u64)) as u32);
            // 465d3 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 465d5 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 465d8 je 0x46a94
            if fl.e() { pc = 0x46a94; continue; }
            pc = 0x465de;
        }
        0x465de => {
            // block 465de
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 465de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 465de test BYTE PTR [rip+0x181cab],0x80 # 0x1c8290
            fl = logic::<8>(m.r8(m.env.data_va(903773)) as u64 & 0x80u64);
            // 465e5 jne 0x465ee
            if fl.ne() { pc = 0x465ee; continue; }
            pc = 0x465e7;
        }
        0x465e7 => {
            // block 465e7
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 465e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 465e7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 465e9 jmp 0x46a94
            pc = 0x46a94; continue;
        }
        0x465ee => {
            // block 465ee
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 465ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 465ee lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 465f5 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 465fc mov rax,QWORD PTR [rip+0xf9bcd] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 46603 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 46607 mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 4660e add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 46612 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 46619 mov rsi,rbx
            rsi = rbx;
            // 4661c call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46621 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 46629 mov DWORD PTR [rbp-0x70],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0xffffffffu64) as u32);
            // 46630 mov r12,QWORD PTR [rip+0xf9d89] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 46637 lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 4663c mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 46643 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4664a add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 4664e mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 46655 mov rdi,rbx
            rdi = rbx;
            // 46658 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4665d mov rax,QWORD PTR [rip+0xf9d44] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 46664 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 46668 mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            // 4666f mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 46676 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 46681 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 4668c mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 46697 mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 466a2 mov DWORD PTR [rbp-0x108],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x10u64) as u32);
            // 466ac xorps xmm0,xmm0
            x0 = 0;
            // 466af movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 466b3 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 466bb lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 466bf mov rdi,rbx
            rdi = rbx;
            // 466c2 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 466c7 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 466cb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 466d0 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 466d5 mov r15,rax
            r15 = rax;
            // 466d8 lea rsi,[rip+0xe45e2] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 466df lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 466e6 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 466eb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 466f0 lea rsi,[rip+0xe45ce] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 466f7 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 466fc mov rdi,rax
            rdi = rax;
            // 466ff call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46704 lea rsi,[rip+0xe4275] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4670b mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 46710 mov rdi,rax
            rdi = rax;
            // 46713 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46718 lea rsi,[rip+0xe3d56] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4671f mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 46724 mov rdi,rax
            rdi = rax;
            // 46727 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4672c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4672f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46733 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 46737 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4673a or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4673d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 46741 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 46744 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46748 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 46751 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 46754 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46758 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 46763 mov rdi,rax
            rdi = rax;
            // 46766 mov rsi,r15
            rsi = r15;
            // 46769 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4676e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 46771 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46775 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 46779 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4677c or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4677f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 46783 lea rsi,[rip+0xe41f6] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4678a mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4678f mov rdi,rax
            rdi = rax;
            // 46792 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46797 mov rbx,rax
            rbx = rax;
            // 4679a lea rsi,[rip+0xe46eb] # 0x12ae8c
            rsi = m.env.data_va(259673);
            // 467a1 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 467a8 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 467ad call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 467b2 movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 467b9 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 467bc jne 0x467ca
            if fl.ne() { pc = 0x467ca; continue; }
            pc = 0x467be;
        }
        0x467be => {
            // block 467be
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 467be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 467be lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // 467c5 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 467c8 jmp 0x467d8
            pc = 0x467d8; continue;
        }
        0x467ca => {
            // block 467ca
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 467ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 467ca mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 467d1 mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x467d8;
        }
        0x467d8 => {
            // block 467d8
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 467d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 467d8 mov rdi,rbx
            rdi = rbx;
            // 467db call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 467e0 lea rsi,[rip+0xe4199] # 0x12a980
            rsi = m.env.data_va(258381);
            // 467e7 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 467ec mov rdi,rax
            rdi = rax;
            // 467ef call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 467f4 lea rsi,[rip+0xe4575] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 467fb mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 46800 mov rdi,rax
            rdi = rax;
            // 46803 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46808 lea rsi,[rip+0xe3c66] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4680f mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 46814 mov rdi,rax
            rdi = rax;
            // 46817 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4681c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4681f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46823 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 46827 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4682a or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4682d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 46831 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 46834 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46838 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 46841 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 46844 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46848 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 46853 mov rdi,rax
            rdi = rax;
            // 46856 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 46859 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4685e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 46861 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46865 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 46869 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4686c or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4686f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 46873 lea rsi,[rip+0xe410c] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4687a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4687f mov rdi,rax
            rdi = rax;
            // 46882 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46887 mov r15,rax
            r15 = rax;
            // 4688a mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 4688d call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46892 mov rbx,rax
            rbx = rax;
            // 46895 mov rdi,rbx
            rdi = rbx;
            // 46898 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4689d mov rdi,r15
            rdi = r15;
            // 468a0 mov rsi,rbx
            rsi = rbx;
            // 468a3 mov rdx,rax
            rdx = rax;
            // 468a6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 468ab lea rsi,[rip+0xe3dd0] # 0x12a682
            rsi = m.env.data_va(257615);
            // 468b2 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 468b7 mov rdi,rax
            rdi = rax;
            // 468ba call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 468bf lea rsi,[rip+0xe4434] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 468c6 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 468cb mov rdi,rax
            rdi = rax;
            // 468ce call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 468d3 lea rsi,[rip+0xe3b9b] # 0x12a475
            rsi = m.env.data_va(257090);
            // 468da mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 468df mov rdi,rax
            rdi = rax;
            // 468e2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 468e7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 468ea mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 468ee mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 468f2 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 468f5 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 468f8 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 468fc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 468ff mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46903 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4690c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4690f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46913 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4691e mov rdi,rax
            rdi = rax;
            // 46921 mov rsi,r14
            rsi = r14;
            // 46924 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46929 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4692c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46930 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 46934 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 46937 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4693a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4693e lea rsi,[rip+0xe43c7] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 46945 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4694a mov rdi,rax
            rdi = rax;
            // 4694d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46952 lea rsi,[rip+0xe3b1c] # 0x12a475
            rsi = m.env.data_va(257090);
            // 46959 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4695e mov rdi,rax
            rdi = rax;
            // 46961 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46966 mov rbx,rax
            rbx = rax;
            // 46969 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4696c mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46970 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 46974 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 46977 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4697a mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 4697e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 46981 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46985 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 4698e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 46991 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 46995 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 469a0 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 469a3 mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 469aa xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 469ac mov rdi,r14
            rdi = r14;
            // 469af call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 469b1 mov rdi,rbx
            rdi = rbx;
            // 469b4 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 469b6 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 469bb mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 469be mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 469c2 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 469c6 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 469c9 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 469cc mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 469d0 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 469d7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 469dc lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 469e3 lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 469ea call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 469ef test BYTE PTR [rbp-0x1a0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64 & 0x1u64);
            // 469f6 jne 0x46a01
            if fl.ne() { pc = 0x46a01; continue; }
            pc = 0x469f8;
        }
        0x469f8 => {
            // block 469f8
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 469f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 469f8 lea rdx,[rbp-0x19f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe61);
            // 469ff jmp 0x46a08
            pc = 0x46a08; continue;
        }
        0x46a01 => {
            // block 46a01
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46a01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46a01 mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            pc = 0x46a08;
        }
        0x46a08 => {
            // block 46a08
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46a08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46a08 lea rsi,[rip+0xe430a] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 46a0f mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 46a14 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 46a16 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 46a1d call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46a22 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 46a29 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46a2e mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 46a35 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 46a3c mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 46a43 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 46a4a mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 46a51 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 46a58 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46a5d mov rdi,rbx
            rdi = rbx;
            // 46a60 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46a65 mov rsi,QWORD PTR [rip+0xf979c] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 46a6c add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 46a70 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 46a77 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46a7c lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 46a83 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46a88 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 46a8a mov rbx,QWORD PTR [rip+0xf9947] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 46a91 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            pc = 0x46a94;
        }
        0x46a94 => {
            // block 46a94
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46a94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46a94 cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 46a98 jne 0x46b2a
            if fl.ne() { pc = 0x46b2a; continue; }
            pc = 0x46a9e;
        }
        0x46a9e => {
            // block 46a9e
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46a9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46a9e add rsp,0x198
            { let (r, f) = add::<64>(rsp, 0x198u64);
              rsp = r;
              fl = f; }
            // 46aa5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 46aa6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 46aa8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 46aaa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 46aac pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 46aae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 46aaf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x46ab0 => {
            // block 46ab0
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46ab0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46ab0 mov r14,rax
            r14 = rax;
            // 46ab3 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            pc = 0x46aba;
        }
        0x46aba => {
            // block 46aba
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46aba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46aba call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46abf jmp 0x46ac4
            pc = 0x46ac4; continue;
        }
        0x46ac1 => {
            // block 46ac1
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46ac1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46ac1 mov r14,rax
            r14 = rax;
            pc = 0x46ac4;
        }
        0x46ac4 => {
            // block 46ac4
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46ac4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46ac4 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 46acb mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 46ad2 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 46ad9 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 46ae0 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            pc = 0x46ae7;
        }
        0x46ae7 => {
            // block 46ae7
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46ae7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46ae7 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 46aee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46af3 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 46afa call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x46aff;
        }
        0x46aff => {
            // block 46aff
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46aff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46aff mov rsi,QWORD PTR [rip+0xf9702] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 46b06 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 46b0a lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 46b11 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x46b16;
        }
        0x46b16 => {
            // block 46b16
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46b16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46b16 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 46b1d call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46b22 mov rdi,r14
            rdi = r14;
            // 46b25 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x46b2a;
        }
        0x46b2a => {
            // block 46b2a
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46b2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46b2a call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x46b34 => {
            // block 46b34
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46b34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46b34 mov r14,rax
            r14 = rax;
            // 46b37 jmp 0x46aff
            pc = 0x46aff; continue;
        }
        0x46b39 => {
            // block 46b39
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46b39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46b39 mov r14,rax
            r14 = rax;
            // 46b3c lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 46b40 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46b45 jmp 0x46ae7
            pc = 0x46ae7; continue;
        }
        0x46b47 => {
            // block 46b47
            #[cfg(feature = "trace")] eprintln!("f_00046556 pc 46b47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46b47 mov r14,rax
            r14 = rax;
            // 46b4a lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 46b51 jmp 0x46aba
            pc = 0x46aba; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4b772 size=1474
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004b772<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4b772;
    loop {
        match pc {
        0x4b772 => {
            // block 4b772
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4b772 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b772 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4b773 mov rbp,rsp
            rbp = rsp;
            // 4b776 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4b778 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4b77a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4b77c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4b77e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4b77f sub rsp,0x178
            { let (r, f) = sub::<64>(rsp, 0x178u64);
              rsp = r;
              fl = f; }
            // 4b786 mov r14,rdi
            r14 = rdi;
            // 4b789 mov DWORD PTR [rbp-0x44],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), (0x0u64) as u32);
            // 4b790 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4b793 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4b795 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b79b mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 4b7a1 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4b7a3 je 0x4b7d8
            if fl.e() { pc = 0x4b7d8; continue; }
            pc = 0x4b7a5;
        }
        0x4b7a5 => {
            // block 4b7a5
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4b7a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b7a5 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4b7a8 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4b7aa mov rdi,r14
            rdi = r14;
            // 4b7ad call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b7b3 lea r9,[rbp-0x44]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // 4b7b7 mov esi,0x8
            rsi = (0x8u64) as u32 as u64;
            // 4b7bc xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b7be xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4b7c0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4b7c3 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4b7c5 call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b7ca mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 4b7cd mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4b7cf test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 4b7d2 je 0x4bc81
            if fl.e() { pc = 0x4bc81; continue; }
            pc = 0x4b7d8;
        }
        0x4b7d8 => {
            // block 4b7d8
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4b7d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b7d8 test BYTE PTR [rip+0x17cab2],0x1 # 0x1c8291
            fl = logic::<8>(m.r8(m.env.data_va(903774)) as u64 & 0x1u64);
            // 4b7df je 0x4bc7f
            if fl.e() { pc = 0x4bc7f; continue; }
            pc = 0x4b7e5;
        }
        0x4b7e5 => {
            // block 4b7e5
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4b7e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b7e5 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 4b7ec lea rbx,[rbp-0x148]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 4b7f3 mov rax,QWORD PTR [rip+0xf49d6] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 4b7fa lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4b7fe mov QWORD PTR [rbp-0x150],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rcx) as u64);
            // 4b805 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4b809 mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 4b810 mov rsi,rbx
            rsi = rbx;
            // 4b813 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b818 mov QWORD PTR [rbp-0x58],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0x0u64) as u64);
            // 4b820 mov DWORD PTR [rbp-0x50],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0xffffffffu64) as u32);
            // 4b827 mov r12,QWORD PTR [rip+0xf4b92] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 4b82e lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 4b833 mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            // 4b83a mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 4b841 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 4b845 mov QWORD PTR [rbp-0xe0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (r12) as u64);
            // 4b84c mov rdi,rbx
            rdi = rbx;
            // 4b84f call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b854 mov rax,QWORD PTR [rip+0xf4b4d] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4b85b add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 4b85f mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // 4b866 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 4b86d mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // 4b878 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // 4b883 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // 4b88e mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // 4b899 mov DWORD PTR [rbp-0xe8],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x10u64) as u32);
            // 4b8a3 xorps xmm0,xmm0
            x0 = 0;
            // 4b8a6 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 4b8aa mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 4b8b2 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4b8b6 mov rdi,rbx
            rdi = rbx;
            // 4b8b9 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b8be lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4b8c2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b8c7 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b8cc mov r15,rax
            r15 = rax;
            // 4b8cf lea rsi,[rip+0xdf3eb] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4b8d6 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 4b8dd mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4b8e2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b8e7 lea rsi,[rip+0xdf3d7] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 4b8ee mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 4b8f3 mov rdi,rax
            rdi = rax;
            // 4b8f6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b8fb lea rsi,[rip+0xdf07e] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4b902 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4b907 mov rdi,rax
            rdi = rax;
            // 4b90a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b90f lea rsi,[rip+0xdeb5f] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4b916 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4b91b mov rdi,rax
            rdi = rax;
            // 4b91e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b923 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b926 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b92a mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4b92e and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b931 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b934 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4b938 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b93b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b93f mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4b948 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b94b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b94f mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4b95a mov rdi,rax
            rdi = rax;
            // 4b95d mov rsi,r15
            rsi = r15;
            // 4b960 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b965 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4b968 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4b96c mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4b970 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b973 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b976 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4b97a lea rsi,[rip+0xdefff] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4b981 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4b986 mov rdi,rax
            rdi = rax;
            // 4b989 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b98e mov rbx,rax
            rbx = rax;
            // 4b991 lea rsi,[rip+0xdf523] # 0x12aebb
            rsi = m.env.data_va(259720);
            // 4b998 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4b99f mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4b9a4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b9a9 movzx edx,BYTE PTR [rbp-0x168]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64) as u32 as u64;
            // 4b9b0 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 4b9b3 jne 0x4b9c1
            if fl.ne() { pc = 0x4b9c1; continue; }
            pc = 0x4b9b5;
        }
        0x4b9b5 => {
            // block 4b9b5
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4b9b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b9b5 lea rsi,[rbp-0x167]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe99);
            // 4b9bc shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 4b9bf jmp 0x4b9cf
            pc = 0x4b9cf; continue;
        }
        0x4b9c1 => {
            // block 4b9c1
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4b9c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b9c1 mov rdx,QWORD PTR [rbp-0x160]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 4b9c8 mov rsi,QWORD PTR [rbp-0x158]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            pc = 0x4b9cf;
        }
        0x4b9cf => {
            // block 4b9cf
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4b9cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b9cf mov rdi,rbx
            rdi = rbx;
            // 4b9d2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b9d7 lea rsi,[rip+0xdefa2] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4b9de mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4b9e3 mov rdi,rax
            rdi = rax;
            // 4b9e6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b9eb lea rsi,[rip+0xdf37e] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 4b9f2 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4b9f7 mov rdi,rax
            rdi = rax;
            // 4b9fa call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b9ff lea rsi,[rip+0xdea6f] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4ba06 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4ba0b mov rdi,rax
            rdi = rax;
            // 4ba0e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ba13 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ba16 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ba1a mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4ba1e and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ba21 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ba24 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4ba28 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ba2b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ba2f mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4ba38 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ba3b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ba3f mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4ba4a mov rdi,rax
            rdi = rax;
            // 4ba4d mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4ba50 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ba55 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ba58 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ba5c mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4ba60 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ba63 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ba66 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4ba6a lea rsi,[rip+0xdef15] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4ba71 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4ba76 mov rdi,rax
            rdi = rax;
            // 4ba79 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ba7e mov r15,rax
            r15 = rax;
            // 4ba81 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 4ba84 call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ba89 mov rbx,rax
            rbx = rax;
            // 4ba8c mov rdi,rbx
            rdi = rbx;
            // 4ba8f call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ba94 mov rdi,r15
            rdi = r15;
            // 4ba97 mov rsi,rbx
            rsi = rbx;
            // 4ba9a mov rdx,rax
            rdx = rax;
            // 4ba9d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4baa2 lea rsi,[rip+0xdebd9] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4baa9 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4baae mov rdi,rax
            rdi = rax;
            // 4bab1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bab6 lea rsi,[rip+0xdf23d] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 4babd mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4bac2 mov rdi,rax
            rdi = rax;
            // 4bac5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4baca lea rsi,[rip+0xde9a4] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4bad1 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4bad6 mov rdi,rax
            rdi = rax;
            // 4bad9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bade mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4bae1 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4bae5 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4bae9 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4baec or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4baef mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4baf3 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4baf6 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4bafa mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4bb03 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4bb06 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4bb0a mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4bb15 mov rdi,rax
            rdi = rax;
            // 4bb18 mov rsi,r14
            rsi = r14;
            // 4bb1b call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bb20 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4bb23 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4bb27 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4bb2b and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4bb2e or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4bb31 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4bb35 lea rsi,[rip+0xdf1d0] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 4bb3c mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4bb41 mov rdi,rax
            rdi = rax;
            // 4bb44 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bb49 lea rsi,[rip+0xde925] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4bb50 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4bb55 mov rdi,rax
            rdi = rax;
            // 4bb58 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bb5d mov rbx,rax
            rbx = rax;
            // 4bb60 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4bb63 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4bb67 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4bb6b and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4bb6e or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4bb71 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 4bb75 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4bb78 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4bb7c mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 4bb85 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4bb88 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4bb8c mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 4bb97 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4bb9a mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 4bba1 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4bba3 mov rdi,r14
            rdi = r14;
            // 4bba6 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bba8 mov rdi,rbx
            rdi = rbx;
            // 4bbab mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 4bbad call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bbb2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4bbb5 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4bbb9 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4bbbd and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4bbc0 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4bbc3 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4bbc7 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4bbce call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bbd3 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 4bbda lea rsi,[rbp-0x148]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 4bbe1 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bbe6 test BYTE PTR [rbp-0x180],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64 & 0x1u64);
            // 4bbed jne 0x4bbf8
            if fl.ne() { pc = 0x4bbf8; continue; }
            pc = 0x4bbef;
        }
        0x4bbef => {
            // block 4bbef
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bbef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bbef lea rdx,[rbp-0x17f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe81);
            // 4bbf6 jmp 0x4bbff
            pc = 0x4bbff; continue;
        }
        0x4bbf8 => {
            // block 4bbf8
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bbf8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bbf8 mov rdx,QWORD PTR [rbp-0x170]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            pc = 0x4bbff;
        }
        0x4bbff => {
            // block 4bbff
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bbff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bbff lea rsi,[rip+0xdf113] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 4bc06 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 4bc0b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4bc0d lea rbx,[rbp-0x148]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 4bc14 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bc19 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 4bc20 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bc25 mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 4bc2c mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 4bc33 mov QWORD PTR [rbp-0xe0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (r12) as u64);
            // 4bc3a mov rax,QWORD PTR [rbp-0x190]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // 4bc41 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 4bc48 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 4bc4f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bc54 mov rdi,rbx
            rdi = rbx;
            // 4bc57 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bc5c mov rsi,QWORD PTR [rip+0xf45a5] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4bc63 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4bc67 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 4bc6e call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bc73 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 4bc7a call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4bc7f;
        }
        0x4bc7f => {
            // block 4bc7f
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bc7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bc7f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4bc81;
        }
        0x4bc81 => {
            // block 4bc81
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bc81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bc81 add rsp,0x178
            { let (r, f) = add::<64>(rsp, 0x178u64);
              rsp = r;
              fl = f; }
            // 4bc88 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4bc89 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4bc8b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4bc8d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4bc8f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4bc91 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4bc92 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4bc93 => {
            // block 4bc93
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bc93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bc93 mov rbx,rax
            rbx = rax;
            // 4bc96 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            pc = 0x4bc9d;
        }
        0x4bc9d => {
            // block 4bc9d
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bc9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bc9d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bca2 jmp 0x4bca7
            pc = 0x4bca7; continue;
        }
        0x4bca4 => {
            // block 4bca4
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bca4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bca4 mov rbx,rax
            rbx = rax;
            pc = 0x4bca7;
        }
        0x4bca7 => {
            // block 4bca7
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bca7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bca7 mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 4bcae mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 4bcb5 mov QWORD PTR [rbp-0xe0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (r12) as u64);
            // 4bcbc mov rax,QWORD PTR [rbp-0x190]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // 4bcc3 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            pc = 0x4bcca;
        }
        0x4bcca => {
            // block 4bcca
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bcca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bcca lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 4bcd1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bcd6 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 4bcdd call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4bce2;
        }
        0x4bce2 => {
            // block 4bce2
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bce2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bce2 mov rsi,QWORD PTR [rip+0xf451f] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4bce9 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4bced lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 4bcf4 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4bcf9;
        }
        0x4bcf9 => {
            // block 4bcf9
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bcf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bcf9 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 4bd00 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bd05 mov rdi,rbx
            rdi = rbx;
            // 4bd08 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bd0d mov rbx,rax
            rbx = rax;
            // 4bd10 jmp 0x4bcf9
            pc = 0x4bcf9; continue;
        }
        0x4bd12 => {
            // block 4bd12
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bd12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bd12 mov rbx,rax
            rbx = rax;
            // 4bd15 jmp 0x4bce2
            pc = 0x4bce2; continue;
        }
        0x4bd17 => {
            // block 4bd17
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bd17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bd17 mov rbx,rax
            rbx = rax;
            // 4bd1a lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4bd1e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bd23 jmp 0x4bcca
            pc = 0x4bcca; continue;
        }
        0x4bd25 => {
            // block 4bd25
            #[cfg(feature = "trace")] eprintln!("f_0004b772 pc 4bd25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bd25 mov rbx,rax
            rbx = rax;
            // 4bd28 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 4bd2f jmp 0x4bc9d
            pc = 0x4bc9d; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4eebc size=1474
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004eebc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4eebc;
    loop {
        match pc {
        0x4eebc => {
            // block 4eebc
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4eebc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4eebc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4eebd mov rbp,rsp
            rbp = rsp;
            // 4eec0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4eec2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4eec4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4eec6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4eec8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4eec9 sub rsp,0x178
            { let (r, f) = sub::<64>(rsp, 0x178u64);
              rsp = r;
              fl = f; }
            // 4eed0 mov r14,rdi
            r14 = rdi;
            // 4eed3 mov DWORD PTR [rbp-0x44],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), (0x0u64) as u32);
            // 4eeda mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4eedd xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4eedf call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4eee5 mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 4eeeb test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4eeed je 0x4ef22
            if fl.e() { pc = 0x4ef22; continue; }
            pc = 0x4eeef;
        }
        0x4eeef => {
            // block 4eeef
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4eeef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4eeef mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4eef2 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4eef4 mov rdi,r14
            rdi = r14;
            // 4eef7 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4eefd lea r9,[rbp-0x44]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // 4ef01 mov esi,0x10
            rsi = (0x10u64) as u32 as u64;
            // 4ef06 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ef08 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4ef0a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4ef0d mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4ef0f call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ef14 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 4ef17 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4ef19 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 4ef1c je 0x4f3cb
            if fl.e() { pc = 0x4f3cb; continue; }
            pc = 0x4ef22;
        }
        0x4ef22 => {
            // block 4ef22
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4ef22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ef22 test BYTE PTR [rip+0x179369],0x1 # 0x1c8292
            fl = logic::<8>(m.r8(m.env.data_va(903775)) as u64 & 0x1u64);
            // 4ef29 je 0x4f3c9
            if fl.e() { pc = 0x4f3c9; continue; }
            pc = 0x4ef2f;
        }
        0x4ef2f => {
            // block 4ef2f
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4ef2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ef2f lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 4ef36 lea rbx,[rbp-0x148]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 4ef3d mov rax,QWORD PTR [rip+0xf128c] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 4ef44 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4ef48 mov QWORD PTR [rbp-0x150],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rcx) as u64);
            // 4ef4f add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4ef53 mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 4ef5a mov rsi,rbx
            rsi = rbx;
            // 4ef5d call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ef62 mov QWORD PTR [rbp-0x58],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0x0u64) as u64);
            // 4ef6a mov DWORD PTR [rbp-0x50],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0xffffffffu64) as u32);
            // 4ef71 mov r12,QWORD PTR [rip+0xf1448] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 4ef78 lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 4ef7d mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            // 4ef84 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 4ef8b add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 4ef8f mov QWORD PTR [rbp-0xe0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (r12) as u64);
            // 4ef96 mov rdi,rbx
            rdi = rbx;
            // 4ef99 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ef9e mov rax,QWORD PTR [rip+0xf1403] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4efa5 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 4efa9 mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // 4efb0 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 4efb7 mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // 4efc2 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // 4efcd mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // 4efd8 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // 4efe3 mov DWORD PTR [rbp-0xe8],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x10u64) as u32);
            // 4efed xorps xmm0,xmm0
            x0 = 0;
            // 4eff0 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 4eff4 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 4effc lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4f000 mov rdi,rbx
            rdi = rbx;
            // 4f003 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f008 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4f00c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f011 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f016 mov r15,rax
            r15 = rax;
            // 4f019 lea rsi,[rip+0xdbca1] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4f020 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 4f027 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4f02c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f031 lea rsi,[rip+0xdbc8d] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 4f038 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 4f03d mov rdi,rax
            rdi = rax;
            // 4f040 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f045 lea rsi,[rip+0xdb934] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4f04c mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4f051 mov rdi,rax
            rdi = rax;
            // 4f054 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f059 lea rsi,[rip+0xdb415] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4f060 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4f065 mov rdi,rax
            rdi = rax;
            // 4f068 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f06d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f070 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f074 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f078 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f07b or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f07e mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f082 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f085 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f089 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4f092 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f095 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f099 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4f0a4 mov rdi,rax
            rdi = rax;
            // 4f0a7 mov rsi,r15
            rsi = r15;
            // 4f0aa call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f0af mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f0b2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f0b6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f0ba and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f0bd or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f0c0 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f0c4 lea rsi,[rip+0xdb8b5] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4f0cb mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4f0d0 mov rdi,rax
            rdi = rax;
            // 4f0d3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f0d8 mov rbx,rax
            rbx = rax;
            // 4f0db lea rsi,[rip+0xdbe69] # 0x12af4b
            rsi = m.env.data_va(259864);
            // 4f0e2 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4f0e9 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 4f0ee call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f0f3 movzx edx,BYTE PTR [rbp-0x168]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64) as u32 as u64;
            // 4f0fa test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 4f0fd jne 0x4f10b
            if fl.ne() { pc = 0x4f10b; continue; }
            pc = 0x4f0ff;
        }
        0x4f0ff => {
            // block 4f0ff
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f0ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f0ff lea rsi,[rbp-0x167]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe99);
            // 4f106 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 4f109 jmp 0x4f119
            pc = 0x4f119; continue;
        }
        0x4f10b => {
            // block 4f10b
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f10b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f10b mov rdx,QWORD PTR [rbp-0x160]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 4f112 mov rsi,QWORD PTR [rbp-0x158]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            pc = 0x4f119;
        }
        0x4f119 => {
            // block 4f119
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f119 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f119 mov rdi,rbx
            rdi = rbx;
            // 4f11c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f121 lea rsi,[rip+0xdb858] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4f128 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4f12d mov rdi,rax
            rdi = rax;
            // 4f130 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f135 lea rsi,[rip+0xdbc34] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 4f13c mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4f141 mov rdi,rax
            rdi = rax;
            // 4f144 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f149 lea rsi,[rip+0xdb325] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4f150 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4f155 mov rdi,rax
            rdi = rax;
            // 4f158 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f15d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f160 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f164 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f168 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f16b or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f16e mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f172 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f175 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f179 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4f182 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f185 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f189 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4f194 mov rdi,rax
            rdi = rax;
            // 4f197 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4f19a call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f19f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f1a2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f1a6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f1aa and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f1ad or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f1b0 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f1b4 lea rsi,[rip+0xdb7cb] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4f1bb mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4f1c0 mov rdi,rax
            rdi = rax;
            // 4f1c3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f1c8 mov r15,rax
            r15 = rax;
            // 4f1cb mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 4f1ce call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f1d3 mov rbx,rax
            rbx = rax;
            // 4f1d6 mov rdi,rbx
            rdi = rbx;
            // 4f1d9 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f1de mov rdi,r15
            rdi = r15;
            // 4f1e1 mov rsi,rbx
            rsi = rbx;
            // 4f1e4 mov rdx,rax
            rdx = rax;
            // 4f1e7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f1ec lea rsi,[rip+0xdb48f] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4f1f3 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4f1f8 mov rdi,rax
            rdi = rax;
            // 4f1fb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f200 lea rsi,[rip+0xdbaf3] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 4f207 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4f20c mov rdi,rax
            rdi = rax;
            // 4f20f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f214 lea rsi,[rip+0xdb25a] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4f21b mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4f220 mov rdi,rax
            rdi = rax;
            // 4f223 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f228 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f22b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f22f mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f233 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f236 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f239 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f23d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f240 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f244 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4f24d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f250 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f254 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4f25f mov rdi,rax
            rdi = rax;
            // 4f262 mov rsi,r14
            rsi = r14;
            // 4f265 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f26a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f26d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f271 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f275 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f278 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f27b mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f27f lea rsi,[rip+0xdba86] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 4f286 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4f28b mov rdi,rax
            rdi = rax;
            // 4f28e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f293 lea rsi,[rip+0xdb1db] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4f29a mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4f29f mov rdi,rax
            rdi = rax;
            // 4f2a2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f2a7 mov rbx,rax
            rbx = rax;
            // 4f2aa mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4f2ad mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f2b1 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f2b5 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4f2b8 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4f2bb mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 4f2bf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4f2c2 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f2c6 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 4f2cf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4f2d2 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f2d6 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 4f2e1 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4f2e4 mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 4f2eb xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4f2ed mov rdi,r14
            rdi = r14;
            // 4f2f0 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f2f2 mov rdi,rbx
            rdi = rbx;
            // 4f2f5 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 4f2f7 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f2fc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f2ff mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f303 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f307 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f30a or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f30d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f311 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4f318 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f31d lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 4f324 lea rsi,[rbp-0x148]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 4f32b call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f330 test BYTE PTR [rbp-0x180],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64 & 0x1u64);
            // 4f337 jne 0x4f342
            if fl.ne() { pc = 0x4f342; continue; }
            pc = 0x4f339;
        }
        0x4f339 => {
            // block 4f339
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f339 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f339 lea rdx,[rbp-0x17f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe81);
            // 4f340 jmp 0x4f349
            pc = 0x4f349; continue;
        }
        0x4f342 => {
            // block 4f342
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f342 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f342 mov rdx,QWORD PTR [rbp-0x170]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            pc = 0x4f349;
        }
        0x4f349 => {
            // block 4f349
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f349 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f349 lea rsi,[rip+0xdb9c9] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 4f350 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 4f355 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4f357 lea rbx,[rbp-0x148]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 4f35e call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f363 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 4f36a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f36f mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 4f376 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 4f37d mov QWORD PTR [rbp-0xe0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (r12) as u64);
            // 4f384 mov rax,QWORD PTR [rbp-0x190]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // 4f38b mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 4f392 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 4f399 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f39e mov rdi,rbx
            rdi = rbx;
            // 4f3a1 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f3a6 mov rsi,QWORD PTR [rip+0xf0e5b] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4f3ad add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4f3b1 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 4f3b8 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f3bd lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 4f3c4 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4f3c9;
        }
        0x4f3c9 => {
            // block 4f3c9
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f3c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f3c9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4f3cb;
        }
        0x4f3cb => {
            // block 4f3cb
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f3cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f3cb add rsp,0x178
            { let (r, f) = add::<64>(rsp, 0x178u64);
              rsp = r;
              fl = f; }
            // 4f3d2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4f3d3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4f3d5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4f3d7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4f3d9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4f3db pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4f3dc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4f3dd => {
            // block 4f3dd
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f3dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f3dd mov rbx,rax
            rbx = rax;
            // 4f3e0 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            pc = 0x4f3e7;
        }
        0x4f3e7 => {
            // block 4f3e7
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f3e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f3e7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f3ec jmp 0x4f3f1
            pc = 0x4f3f1; continue;
        }
        0x4f3ee => {
            // block 4f3ee
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f3ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f3ee mov rbx,rax
            rbx = rax;
            pc = 0x4f3f1;
        }
        0x4f3f1 => {
            // block 4f3f1
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f3f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f3f1 mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 4f3f8 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 4f3ff mov QWORD PTR [rbp-0xe0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (r12) as u64);
            // 4f406 mov rax,QWORD PTR [rbp-0x190]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // 4f40d mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            pc = 0x4f414;
        }
        0x4f414 => {
            // block 4f414
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f414 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f414 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 4f41b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f420 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 4f427 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4f42c;
        }
        0x4f42c => {
            // block 4f42c
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f42c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f42c mov rsi,QWORD PTR [rip+0xf0dd5] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4f433 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4f437 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 4f43e call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4f443;
        }
        0x4f443 => {
            // block 4f443
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f443 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f443 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 4f44a call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f44f mov rdi,rbx
            rdi = rbx;
            // 4f452 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f457 mov rbx,rax
            rbx = rax;
            // 4f45a jmp 0x4f443
            pc = 0x4f443; continue;
        }
        0x4f45c => {
            // block 4f45c
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f45c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f45c mov rbx,rax
            rbx = rax;
            // 4f45f jmp 0x4f42c
            pc = 0x4f42c; continue;
        }
        0x4f461 => {
            // block 4f461
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f461 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f461 mov rbx,rax
            rbx = rax;
            // 4f464 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4f468 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f46d jmp 0x4f414
            pc = 0x4f414; continue;
        }
        0x4f46f => {
            // block 4f46f
            #[cfg(feature = "trace")] eprintln!("f_0004eebc pc 4f46f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f46f mov rbx,rax
            rbx = rax;
            // 4f472 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 4f479 jmp 0x4f3e7
            pc = 0x4f3e7; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4db64 size=1390
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_0004db64<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4db64;
    loop {
        match pc {
        0x4db64 => {
            // block 4db64
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4db64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4db64 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4db65 mov rbp,rsp
            rbp = rsp;
            // 4db68 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4db6a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4db6c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4db6e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4db70 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4db71 sub rsp,0x198
            { let (r, f) = sub::<64>(rsp, 0x198u64);
              rsp = r;
              fl = f; }
            // 4db78 mov r15,rdx
            r15 = rdx;
            // 4db7b mov r12,rsi
            r12 = rsi;
            // 4db7e mov r14,rdi
            r14 = rdi;
            // 4db81 mov rbx,QWORD PTR [rip+0xf2850] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4db88 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4db8b mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 4db8f mov DWORD PTR [rbp-0x64],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x2u64) as u32);
            // 4db96 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4db99 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4db9b call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dba1 mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 4dba7 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4dba9 je 0x4dbec
            if fl.e() { pc = 0x4dbec; continue; }
            pc = 0x4dbab;
        }
        0x4dbab => {
            // block 4dbab
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4dbab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4dbab mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4dbae xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4dbb0 mov rdi,r14
            rdi = r14;
            // 4dbb3 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dbb9 lea r8,[rbp-0x40]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4dbbd lea r9,[rbp-0x64]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 4dbc1 mov esi,0xe
            rsi = (0xeu64) as u32 as u64;
            // 4dbc6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4dbc8 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4dbca mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4dbcc call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dbd1 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 4dbd4 mov eax,DWORD PTR [rbp-0x40]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 4dbd7 mov ecx,DWORD PTR [rbp-0x38]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // 4dbda mov DWORD PTR [r12],eax
            m.w32(r12, ((rax as u32 as u64)) as u32);
            // 4dbde mov DWORD PTR [r15],ecx
            m.w32(r15, ((rcx as u32 as u64)) as u32);
            // 4dbe1 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4dbe3 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 4dbe6 je 0x4e010
            if fl.e() { pc = 0x4e010; continue; }
            pc = 0x4dbec;
        }
        0x4dbec => {
            // block 4dbec
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4dbec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4dbec test BYTE PTR [rip+0x17a69e],0x40 # 0x1c8291
            fl = logic::<8>(m.r8(m.env.data_va(903774)) as u64 & 0x40u64);
            // 4dbf3 jne 0x4dbfc
            if fl.ne() { pc = 0x4dbfc; continue; }
            pc = 0x4dbf5;
        }
        0x4dbf5 => {
            // block 4dbf5
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4dbf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4dbf5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4dbf7 jmp 0x4e010
            pc = 0x4e010; continue;
        }
        0x4dbfc => {
            // block 4dbfc
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4dbfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4dbfc lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4dc03 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4dc0a mov rax,QWORD PTR [rip+0xf25bf] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 4dc11 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4dc15 mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 4dc1c add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4dc20 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 4dc27 mov rsi,rbx
            rsi = rbx;
            // 4dc2a call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dc2f mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 4dc37 mov DWORD PTR [rbp-0x70],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0xffffffffu64) as u32);
            // 4dc3e mov r12,QWORD PTR [rip+0xf277b] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 4dc45 lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 4dc4a mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 4dc51 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4dc58 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 4dc5c mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4dc63 mov rdi,rbx
            rdi = rbx;
            // 4dc66 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dc6b mov rax,QWORD PTR [rip+0xf2736] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4dc72 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 4dc76 mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            // 4dc7d mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4dc84 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 4dc8f mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 4dc9a mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 4dca5 mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 4dcb0 mov DWORD PTR [rbp-0x108],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x10u64) as u32);
            // 4dcba xorps xmm0,xmm0
            x0 = 0;
            // 4dcbd movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 4dcc1 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 4dcc9 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4dccd mov rdi,rbx
            rdi = rbx;
            // 4dcd0 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dcd5 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4dcd9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dcde call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dce3 mov r15,rax
            r15 = rax;
            // 4dce6 lea rsi,[rip+0xdcfd4] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4dced lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4dcf4 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4dcf9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dcfe lea rsi,[rip+0xdcfc0] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 4dd05 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 4dd0a mov rdi,rax
            rdi = rax;
            // 4dd0d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dd12 lea rsi,[rip+0xdcc67] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4dd19 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4dd1e mov rdi,rax
            rdi = rax;
            // 4dd21 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dd26 lea rsi,[rip+0xdc748] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4dd2d mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4dd32 mov rdi,rax
            rdi = rax;
            // 4dd35 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dd3a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4dd3d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4dd41 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4dd45 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4dd48 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4dd4b mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4dd4f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4dd52 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4dd56 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4dd5f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4dd62 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4dd66 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4dd71 mov rdi,rax
            rdi = rax;
            // 4dd74 mov rsi,r15
            rsi = r15;
            // 4dd77 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dd7c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4dd7f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4dd83 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4dd87 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4dd8a or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4dd8d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4dd91 lea rsi,[rip+0xdcbe8] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4dd98 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4dd9d mov rdi,rax
            rdi = rax;
            // 4dda0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dda5 mov rbx,rax
            rbx = rax;
            // 4dda8 lea rsi,[rip+0xdd16c] # 0x12af1b
            rsi = m.env.data_va(259816);
            // 4ddaf lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4ddb6 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 4ddbb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ddc0 movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 4ddc7 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 4ddca jne 0x4ddd8
            if fl.ne() { pc = 0x4ddd8; continue; }
            pc = 0x4ddcc;
        }
        0x4ddcc => {
            // block 4ddcc
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4ddcc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ddcc lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // 4ddd3 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 4ddd6 jmp 0x4dde6
            pc = 0x4dde6; continue;
        }
        0x4ddd8 => {
            // block 4ddd8
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4ddd8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ddd8 mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 4dddf mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x4dde6;
        }
        0x4dde6 => {
            // block 4dde6
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4dde6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4dde6 mov rdi,rbx
            rdi = rbx;
            // 4dde9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ddee lea rsi,[rip+0xdcb8b] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4ddf5 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4ddfa mov rdi,rax
            rdi = rax;
            // 4ddfd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4de02 lea rsi,[rip+0xdcf67] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 4de09 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4de0e mov rdi,rax
            rdi = rax;
            // 4de11 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4de16 lea rsi,[rip+0xdc658] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4de1d mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4de22 mov rdi,rax
            rdi = rax;
            // 4de25 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4de2a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4de2d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4de31 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4de35 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4de38 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4de3b mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4de3f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4de42 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4de46 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4de4f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4de52 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4de56 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4de61 mov rdi,rax
            rdi = rax;
            // 4de64 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4de67 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4de6c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4de6f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4de73 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4de77 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4de7a or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4de7d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4de81 lea rsi,[rip+0xdcafe] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4de88 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4de8d mov rdi,rax
            rdi = rax;
            // 4de90 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4de95 mov r15,rax
            r15 = rax;
            // 4de98 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 4de9b call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dea0 mov rbx,rax
            rbx = rax;
            // 4dea3 mov rdi,rbx
            rdi = rbx;
            // 4dea6 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4deab mov rdi,r15
            rdi = r15;
            // 4deae mov rsi,rbx
            rsi = rbx;
            // 4deb1 mov rdx,rax
            rdx = rax;
            // 4deb4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4deb9 lea rsi,[rip+0xdc7c2] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4dec0 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4dec5 mov rdi,rax
            rdi = rax;
            // 4dec8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4decd lea rsi,[rip+0xdce26] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 4ded4 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4ded9 mov rdi,rax
            rdi = rax;
            // 4dedc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dee1 lea rsi,[rip+0xdc58d] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4dee8 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4deed mov rdi,rax
            rdi = rax;
            // 4def0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4def5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4def8 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4defc mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4df00 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4df03 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4df06 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4df0a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4df0d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4df11 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4df1a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4df1d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4df21 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4df2c mov rdi,rax
            rdi = rax;
            // 4df2f mov rsi,r14
            rsi = r14;
            // 4df32 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4df37 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4df3a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4df3e mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4df42 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4df45 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4df48 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4df4c lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4df53 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4df58 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4df5f lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4df66 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4df6b test BYTE PTR [rbp-0x1a0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64 & 0x1u64);
            // 4df72 jne 0x4df7d
            if fl.ne() { pc = 0x4df7d; continue; }
            pc = 0x4df74;
        }
        0x4df74 => {
            // block 4df74
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4df74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4df74 lea rdx,[rbp-0x19f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe61);
            // 4df7b jmp 0x4df84
            pc = 0x4df84; continue;
        }
        0x4df7d => {
            // block 4df7d
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4df7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4df7d mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            pc = 0x4df84;
        }
        0x4df84 => {
            // block 4df84
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4df84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4df84 lea rsi,[rip+0xdcd8e] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 4df8b mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 4df90 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4df92 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4df99 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4df9e lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4dfa5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dfaa mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4dfb1 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4dfb8 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4dfbf mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 4dfc6 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4dfcd lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4dfd4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dfd9 mov rdi,rbx
            rdi = rbx;
            // 4dfdc call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dfe1 mov rsi,QWORD PTR [rip+0xf2220] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4dfe8 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4dfec lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4dff3 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dff8 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4dfff call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e004 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4e006 mov rbx,QWORD PTR [rip+0xf23cb] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4e00d mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            pc = 0x4e010;
        }
        0x4e010 => {
            // block 4e010
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4e010 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e010 cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 4e014 jne 0x4e0a6
            if fl.ne() { pc = 0x4e0a6; continue; }
            pc = 0x4e01a;
        }
        0x4e01a => {
            // block 4e01a
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4e01a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e01a add rsp,0x198
            { let (r, f) = add::<64>(rsp, 0x198u64);
              rsp = r;
              fl = f; }
            // 4e021 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4e022 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4e024 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4e026 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4e028 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4e02a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4e02b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4e02c => {
            // block 4e02c
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4e02c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e02c mov r14,rax
            r14 = rax;
            // 4e02f lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            pc = 0x4e036;
        }
        0x4e036 => {
            // block 4e036
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4e036 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e036 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e03b jmp 0x4e040
            pc = 0x4e040; continue;
        }
        0x4e03d => {
            // block 4e03d
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4e03d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e03d mov r14,rax
            r14 = rax;
            pc = 0x4e040;
        }
        0x4e040 => {
            // block 4e040
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4e040 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e040 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4e047 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4e04e mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4e055 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 4e05c mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            pc = 0x4e063;
        }
        0x4e063 => {
            // block 4e063
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4e063 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e063 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4e06a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e06f lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4e076 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4e07b;
        }
        0x4e07b => {
            // block 4e07b
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4e07b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e07b mov rsi,QWORD PTR [rip+0xf2186] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4e082 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4e086 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4e08d call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4e092;
        }
        0x4e092 => {
            // block 4e092
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4e092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e092 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4e099 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e09e mov rdi,r14
            rdi = r14;
            // 4e0a1 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4e0a6;
        }
        0x4e0a6 => {
            // block 4e0a6
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4e0a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e0a6 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x4e0b0 => {
            // block 4e0b0
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4e0b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e0b0 mov r14,rax
            r14 = rax;
            // 4e0b3 jmp 0x4e07b
            pc = 0x4e07b; continue;
        }
        0x4e0b5 => {
            // block 4e0b5
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4e0b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e0b5 mov r14,rax
            r14 = rax;
            // 4e0b8 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4e0bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e0c1 jmp 0x4e063
            pc = 0x4e063; continue;
        }
        0x4e0c3 => {
            // block 4e0c3
            #[cfg(feature = "trace")] eprintln!("f_0004db64 pc 4e0c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e0c3 mov r14,rax
            r14 = rax;
            // 4e0c6 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4e0cd jmp 0x4e036
            pc = 0x4e036; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd898e size=812
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000d898e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd898e;
    loop {
        match pc {
        0xd898e => {
            // block d898e
            #[cfg(feature = "trace")] eprintln!("f_000d898e pc d898e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d898e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d898f mov rbp,rsp
            rbp = rsp;
            // d8992 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d8994 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d8996 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d8998 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d899a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d899b sub rsp,0xb8
            { let (r, f) = sub::<64>(rsp, 0xb8u64);
              rsp = r;
              fl = f; }
            // d89a2 mov DWORD PTR [rbp-0xd4],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff2c), ((rsi as u32 as u64)) as u32);
            // d89a8 mov rbx,rdi
            rbx = rdi;
            // d89ab mov QWORD PTR [rbp-0xc8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (rbx) as u64);
            // d89b2 mov rax,QWORD PTR [rip+0x67a1f] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // d89b9 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // d89bc mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // d89c0 lea rdi,[rip+0x604d1] # 0x138e98
            rdi = m.env.data_va(317029);
            // d89c7 call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d89cc cmp DWORD PTR [rbx+0x338],0x0
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x338)) as u64, 0x0u64).1;
            // d89d3 je 0xd8c7a
            if fl.e() { pc = 0xd8c7a; continue; }
            pc = 0xd89d9;
        }
        0xd89d9 => {
            // block d89d9
            #[cfg(feature = "trace")] eprintln!("f_000d898e pc d89d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d89d9 lea r14,[rip+0x604c3] # 0x138ea3
            r14 = m.env.data_va(317040);
            // d89e0 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d89e3 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd89e6;
        }
        0xd89e6 => {
            // block d89e6
            #[cfg(feature = "trace")] eprintln!("f_000d898e pc d89e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d89e6 mov rdi,rbx
            rdi = rbx;
            // d89e9 call 0xd8956
            { let r = { let t = m.env.code_va(0xd8956); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d89ee test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d89f0 je 0xd8a00
            if fl.e() { pc = 0xd8a00; continue; }
            pc = 0xd89f2;
        }
        0xd89f2 => {
            // block d89f2
            #[cfg(feature = "trace")] eprintln!("f_000d898e pc d89f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d89f2 mov rdi,rbx
            rdi = rbx;
            // d89f5 mov esi,DWORD PTR [rbp-0xd4]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff2c)) as u64) as u32 as u64;
            // d89fb call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd8a00;
        }
        0xd8a00 => {
            // block d8a00
            #[cfg(feature = "trace")] eprintln!("f_000d898e pc d8a00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8a00 mov ecx,DWORD PTR [rbx+0x334]
            rcx = (m.r32(rbx.wrapping_add(0x334)) as u64) as u32 as u64;
            // d8a06 sub ecx,r12d
            { let (r, f) = sub::<32>((rcx as u32 as u64), (r12 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // d8a09 mov DWORD PTR [rbp-0xcc],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff34), ((r12 as u32 as u64)) as u32);
            // d8a10 cmp ecx,0x20
            fl = sub::<32>((rcx as u32 as u64), 0x20u64).1;
            // d8a13 mov eax,0x20
            rax = (0x20u64) as u32 as u64;
            // d8a18 cmovae ecx,eax
            if fl.ae() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // d8a1b mov QWORD PTR [rbp-0xc0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rcx) as u64);
            // d8a22 mov BYTE PTR [rbp-0xa0],0x53
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x53u64) as u8);
            // d8a29 mov BYTE PTR [rbp-0x9f],0x33
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff61), (0x33u64) as u8);
            // d8a30 lea eax,[rcx+0x5]
            rax = (rcx.wrapping_add(0x5)) as u32 as u64;
            // d8a33 movzx edx,ax
            rdx = ((rax as u16 as u64)) as u32 as u64;
            // d8a36 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d8a39 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8a3b lea rdi,[rbp-0x9e]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff62);
            // d8a42 mov rsi,r14
            rsi = r14;
            // d8a45 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8a4a mov r12,r14
            r12 = r14;
            // d8a4d mov r14d,r13d
            r14 = ((r13 as u32 as u64)) as u32 as u64;
            // d8a50 shr r14d,0x18
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x18u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // d8a54 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8a56 lea rdi,[rbp-0x9c]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff64);
            // d8a5d mov rsi,r12
            rsi = r12;
            // d8a60 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d8a63 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8a68 mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // d8a6b shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // d8a6e mov DWORD PTR [rbp-0xb0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff50), ((rax as u32 as u64)) as u32);
            // d8a74 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // d8a77 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8a79 lea rdi,[rbp-0x9a]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff66);
            // d8a80 mov rsi,r12
            rsi = r12;
            // d8a83 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8a88 mov QWORD PTR [rbp-0xb8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (r13) as u64);
            // d8a8f mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // d8a92 shr eax,0x8
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // d8a95 mov DWORD PTR [rbp-0xd0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff30), ((rax as u32 as u64)) as u32);
            // d8a9b movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // d8a9e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8aa0 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // d8aa7 mov rsi,r12
            rsi = r12;
            // d8aaa call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8aaf movzx edx,r13b
            rdx = ((r13 as u8 as u64)) as u32 as u64;
            // d8ab3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8ab5 lea rdi,[rbp-0x96]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff6a);
            // d8abc mov rsi,r12
            rsi = r12;
            // d8abf call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8ac4 add r14d,r13d
            { let (r, f) = add::<32>((r14 as u32 as u64), (r13 as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // d8ac7 add r14d,DWORD PTR [rbp-0xb0]
            { let (r, f) = add::<32>((r14 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d8ace add r14d,DWORD PTR [rbp-0xd0]
            { let (r, f) = add::<32>((r14 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff30)) as u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d8ad5 mov rax,QWORD PTR [rbp-0xc0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // d8adc lea r13d,[rax+r14*1+0x5]
            r13 = (rax.wrapping_add(r14).wrapping_add(0x5)) as u32 as u64;
            // d8ae1 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d8ae3 mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // d8ae8 je 0xd8c2a
            if fl.e() { pc = 0xd8c2a; continue; }
            pc = 0xd8aee;
        }
        0xd8aee => {
            // block d8aee
            #[cfg(feature = "trace")] eprintln!("f_000d898e pc d8aee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8aee mov QWORD PTR [rbp-0xb0],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (rdx) as u64);
            // d8af5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8af8 mov rcx,QWORD PTR [rbp-0xb8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // d8aff mov rdx,QWORD PTR [rbp-0xb0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // d8b06 lea edx,[rcx+rdx*1]
            rdx = (rcx.wrapping_add(rdx)) as u32 as u64;
            // d8b09 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d8b0e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8b13 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8b16 mov rdi,rbx
            rdi = rbx;
            // d8b19 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8b1c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8b1f mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d8b24 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // d8b29 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8b2e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8b31 mov rdi,rbx
            rdi = rbx;
            // d8b34 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8b37 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8b3a mov rdi,rbx
            rdi = rbx;
            // d8b3d call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8b43 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8b46 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d8b4b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8b50 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8b53 mov rdi,rbx
            rdi = rbx;
            // d8b56 lea rdx,[rbp-0xa4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff5c);
            // d8b5d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8b60 mov ebx,DWORD PTR [rbp-0xa4]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff5c)) as u64) as u32 as u64;
            // d8b66 movzx edx,bl
            rdx = ((rbx as u8 as u64)) as u32 as u64;
            // d8b69 mov r14,r15
            r14 = r15;
            // d8b6c lea rdi,[rbp+r14*1-0x94]
            rdi = rbp.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_ff6c);
            // d8b74 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8b76 mov rsi,r12
            rsi = r12;
            // d8b79 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8b7e add r13d,ebx
            { let (r, f) = add::<32>((r13 as u32 as u64), (rbx as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // d8b81 mov r15d,DWORD PTR [rbp-0xa4]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff5c)) as u64) as u32 as u64;
            // d8b88 shr r15d,0x8
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x8u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // d8b8c movzx edx,r15b
            rdx = ((r15 as u8 as u64)) as u32 as u64;
            // d8b90 lea rdi,[rbp+r14*1-0x92]
            rdi = rbp.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_ff6e);
            // d8b98 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8b9a mov rsi,r12
            rsi = r12;
            // d8b9d call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8ba2 add r15d,r13d
            { let (r, f) = add::<32>((r15 as u32 as u64), (r13 as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // d8ba5 mov ebx,DWORD PTR [rbp-0xa4]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff5c)) as u64) as u32 as u64;
            // d8bab shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // d8bae movzx edx,bl
            rdx = ((rbx as u8 as u64)) as u32 as u64;
            // d8bb1 lea rdi,[rbp+r14*1-0x90]
            rdi = rbp.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_ff70);
            // d8bb9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8bbb mov rsi,r12
            rsi = r12;
            // d8bbe call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8bc3 add ebx,r15d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r15 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // d8bc6 mov r15d,DWORD PTR [rbp-0xa4]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff5c)) as u64) as u32 as u64;
            // d8bcd shr r15d,0x18
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x18u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // d8bd1 lea rdi,[rbp+r14*1-0x8e]
            rdi = rbp.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_ff72);
            // d8bd9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8bdb mov rsi,r12
            rsi = r12;
            // d8bde mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // d8be1 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8be6 mov rdx,QWORD PTR [rbp-0xb0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // d8bed movzx r13d,bl
            r13 = ((rbx as u8 as u64)) as u32 as u64;
            // d8bf1 add r13d,r15d
            { let (r, f) = add::<32>((r13 as u32 as u64), (r15 as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // d8bf4 mov rbx,QWORD PTR [rbp-0xc8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff38)) as u64;
            // d8bfb add edx,0x4
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x4u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d8bfe add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // d8c02 mov r15,r14
            r15 = r14;
            // d8c05 mov rax,QWORD PTR [rbp-0xc0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // d8c0c cmp edx,eax
            fl = sub::<32>((rdx as u32 as u64), (rax as u32 as u64)).1;
            // d8c0e jb 0xd8aee
            if fl.b() { pc = 0xd8aee; continue; }
            pc = 0xd8c14;
        }
        0xd8c14 => {
            // block d8c14
            #[cfg(feature = "trace")] eprintln!("f_000d898e pc d8c14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8c14 add r15,0xc
            { let (r, f) = add::<64>(r15, 0xcu64);
              r15 = r;
              fl = f; }
            // d8c18 mov rax,QWORD PTR [rbp-0xb8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // d8c1f add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // d8c21 mov QWORD PTR [rbp-0xb8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rax) as u64);
            // d8c28 jmp 0xd8c30
            pc = 0xd8c30; continue;
        }
        0xd8c2a => {
            // block d8c2a
            #[cfg(feature = "trace")] eprintln!("f_000d898e pc d8c2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8c2a mov r15d,0xc
            r15 = (0xcu64) as u32 as u64;
            pc = 0xd8c30;
        }
        0xd8c30 => {
            // block d8c30
            #[cfg(feature = "trace")] eprintln!("f_000d898e pc d8c30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8c30 movsxd rax,r15d
            rax = sx::<32>((r15 as u32 as u64)) as u64;
            // d8c33 lea rdi,[rbp+rax*1-0xa0]
            rdi = rbp.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_ff60);
            // d8c3b not r13d
            r13 = (!(r13 as u32 as u64)) as u32 as u64;
            // d8c3e movzx edx,r13b
            rdx = ((r13 as u8 as u64)) as u32 as u64;
            // d8c42 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8c44 mov r14,r12
            r14 = r12;
            // d8c47 mov rsi,r14
            rsi = r14;
            // d8c4a call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8c4f lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d8c56 call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8c5b mov r12d,DWORD PTR [rbp-0xcc]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff34)) as u64) as u32 as u64;
            // d8c62 add r12d,0x20
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x20u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d8c66 cmp r12d,DWORD PTR [rbx+0x338]
            fl = sub::<32>((r12 as u32 as u64), m.r32(rbx.wrapping_add(0x338)) as u64).1;
            // d8c6d mov r13,QWORD PTR [rbp-0xb8]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // d8c74 jb 0xd89e6
            if fl.b() { pc = 0xd89e6; continue; }
            pc = 0xd8c7a;
        }
        0xd8c7a => {
            // block d8c7a
            #[cfg(feature = "trace")] eprintln!("f_000d898e pc d8c7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8c7a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d8c7c mov rdi,rbx
            rdi = rbx;
            // d8c7f call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8c84 lea rdi,[rip+0x6021d] # 0x138ea8
            rdi = m.env.data_va(317045);
            // d8c8b call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8c90 mov rax,QWORD PTR [rip+0x67741] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // d8c97 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // d8c9a cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // d8c9e jne 0xd8cb4
            if fl.ne() { pc = 0xd8cb4; continue; }
            pc = 0xd8ca0;
        }
        0xd8ca0 => {
            // block d8ca0
            #[cfg(feature = "trace")] eprintln!("f_000d898e pc d8ca0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8ca0 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d8ca2 add rsp,0xb8
            { let (r, f) = add::<64>(rsp, 0xb8u64);
              rsp = r;
              fl = f; }
            // d8ca9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d8caa pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d8cac pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d8cae pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d8cb0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d8cb2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d8cb3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd8cb4 => {
            // block d8cb4
            #[cfg(feature = "trace")] eprintln!("f_000d898e pc d8cb4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8cb4 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdbee6 size=452
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000dbee6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdbee6;
    loop {
        match pc {
        0xdbee6 => {
            // block dbee6
            #[cfg(feature = "trace")] eprintln!("f_000dbee6 pc dbee6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbee6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dbee7 mov rbp,rsp
            rbp = rsp;
            // dbeea push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dbeec push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dbeee push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dbef0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dbef2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dbef3 sub rsp,0x418
            { let (r, f) = sub::<64>(rsp, 0x418u64);
              rsp = r;
              fl = f; }
            // dbefa mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // dbefd mov r15,rdi
            r15 = rdi;
            // dbf00 mov r12,QWORD PTR [rip+0x644d1] # 0x1403d8
            r12 = m.r64(m.env.data_va(347045)) as u64;
            // dbf07 mov r12,QWORD PTR [r12]
            r12 = m.r64(r12) as u64;
            // dbf0b mov QWORD PTR [rbp-0x30],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r12) as u64);
            // dbf0f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbf11 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // dbf13 div DWORD PTR [r15+0x338]
            { let (q, r) = div::<32>(rdx, rax, m.r32(r15.wrapping_add(0x338)) as u64); rax = q; rdx = r; }
            // dbf1a mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // dbf1c mov QWORD PTR [rbp-0x440],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc0), (rbx) as u64);
            // dbf23 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // dbf26 mov DWORD PTR [rbp-0x438],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fbc8), ((r13 as u32 as u64)) as u32);
            // dbf2d mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // dbf30 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbf35 lea rdi,[rbp-0x430]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd0);
            // dbf3c mov esi,0x400
            rsi = (0x400u64) as u32 as u64;
            // dbf41 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbf46 mov edx,DWORD PTR [r15+0x338]
            rdx = (m.r32(r15.wrapping_add(0x338)) as u64) as u32 as u64;
            // dbf4d imul edx,r13d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r13 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dbf51 add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dbf53 lea rsi,[rip+0x5d0d1] # 0x13902b
            rsi = m.env.data_va(317432);
            // dbf5a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // dbf5c lea rdi,[rbp-0x430]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd0);
            // dbf63 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbf68 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // dbf6b je 0xdc08d
            if fl.e() { pc = 0xdc08d; continue; }
            pc = 0xdbf71;
        }
        0xdbf71 => {
            // block dbf71
            #[cfg(feature = "trace")] eprintln!("f_000dbee6 pc dbf71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbf71 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // dbf73 lea rbx,[rbp+rax*1-0x430]
            rbx = rbp.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_fbd0);
            // dbf7b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // dbf7e mov r12,QWORD PTR [rbp-0x440]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbc0)) as u64;
            pc = 0xdbf85;
        }
        0xdbf85 => {
            // block dbf85
            #[cfg(feature = "trace")] eprintln!("f_000dbee6 pc dbf85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbf85 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbf88 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // dbf8d mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dbf92 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dbf95 mov rdi,r15
            rdi = r15;
            // dbf98 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // dbf9b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbf9e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbfa1 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // dbfa6 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // dbfab mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dbfb0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dbfb3 mov rdi,r15
            rdi = r15;
            // dbfb6 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbfb9 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbfbc mov rdi,r15
            rdi = r15;
            // dbfbf call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbfc5 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dbfc8 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // dbfcd mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dbfd2 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dbfd5 mov rdi,r15
            rdi = r15;
            // dbfd8 lea rdx,[rbp-0x434]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fbcc);
            // dbfdf call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbfe2 mov edx,DWORD PTR [rbp-0x434]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fbcc)) as u64) as u32 as u64;
            // dbfe8 bswap edx
            rdx = (rdx as u32).swap_bytes() as u64;
            // dbfea mov DWORD PTR [rbp-0x434],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fbcc), ((rdx as u32 as u64)) as u32);
            // dbff0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // dbff2 mov rdi,rbx
            rdi = rbx;
            // dbff5 lea rsi,[rip+0x5d036] # 0x139032
            rsi = m.env.data_va(317439);
            // dbffc call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc001 inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // dc004 cmp r13d,0x4
            fl = sub::<32>((r13 as u32 as u64), 0x4u64).1;
            // dc008 jne 0xdc05f
            if fl.ne() { pc = 0xdc05f; continue; }
            pc = 0xdc00a;
        }
        0xdc00a => {
            // block dc00a
            #[cfg(feature = "trace")] eprintln!("f_000dbee6 pc dc00a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc00a lea rbx,[rbp-0x430]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fbd0);
            // dc011 mov rdi,rbx
            rdi = rbx;
            // dc014 call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc019 mov esi,0x400
            rsi = (0x400u64) as u32 as u64;
            // dc01e mov rdi,rbx
            rdi = rbx;
            // dc021 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc026 mov eax,DWORD PTR [r15+0x338]
            rax = (m.r32(r15.wrapping_add(0x338)) as u64) as u32 as u64;
            // dc02d imul eax,DWORD PTR [rbp-0x438]
            { let (r, f) = imul::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fbc8)) as u64); rax = (r) as u32 as u64; fl = f; }
            // dc034 mov rcx,r12
            rcx = r12;
            // dc037 lea r12d,[rcx+0x4]
            r12 = (rcx.wrapping_add(0x4)) as u32 as u64;
            // dc03b lea edx,[rcx+rax*1+0x4]
            rdx = (rcx.wrapping_add(rax).wrapping_add(0x4)) as u32 as u64;
            // dc03f xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // dc042 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // dc044 mov rdi,rbx
            rdi = rbx;
            // dc047 lea rsi,[rip+0x5cfdd] # 0x13902b
            rsi = m.env.data_va(317432);
            // dc04e call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc053 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // dc055 lea rbx,[rbp+rax*1-0x430]
            rbx = rbp.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_fbd0);
            // dc05d jmp 0xdc068
            pc = 0xdc068; continue;
        }
        0xdc05f => {
            // block dc05f
            #[cfg(feature = "trace")] eprintln!("f_000dbee6 pc dc05f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc05f cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // dc061 add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // dc064 add r12d,0x4
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x4u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xdc068;
        }
        0xdc068 => {
            // block dc068
            #[cfg(feature = "trace")] eprintln!("f_000dbee6 pc dc068 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc068 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // dc06b jne 0xdbf85
            if fl.ne() { pc = 0xdbf85; continue; }
            pc = 0xdc071;
        }
        0xdc071 => {
            // block dc071
            #[cfg(feature = "trace")] eprintln!("f_000dbee6 pc dc071 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc071 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // dc074 mov r12,QWORD PTR [rip+0x6435d] # 0x1403d8
            r12 = m.r64(m.env.data_va(347045)) as u64;
            // dc07b mov r12,QWORD PTR [r12]
            r12 = m.r64(r12) as u64;
            // dc07f je 0xdc08d
            if fl.e() { pc = 0xdc08d; continue; }
            pc = 0xdc081;
        }
        0xdc081 => {
            // block dc081
            #[cfg(feature = "trace")] eprintln!("f_000dbee6 pc dc081 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc081 lea rdi,[rbp-0x430]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd0);
            // dc088 call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdc08d;
        }
        0xdc08d => {
            // block dc08d
            #[cfg(feature = "trace")] eprintln!("f_000dbee6 pc dc08d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc08d cmp r12,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // dc091 jne 0xdc0a5
            if fl.ne() { pc = 0xdc0a5; continue; }
            pc = 0xdc093;
        }
        0xdc093 => {
            // block dc093
            #[cfg(feature = "trace")] eprintln!("f_000dbee6 pc dc093 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc093 add rsp,0x418
            { let (r, f) = add::<64>(rsp, 0x418u64);
              rsp = r;
              fl = f; }
            // dc09a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dc09b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dc09d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dc09f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dc0a1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dc0a3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc0a4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xdc0a5 => {
            // block dc0a5
            #[cfg(feature = "trace")] eprintln!("f_000dbee6 pc dc0a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0a5 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7c4a size=264
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000b7c4a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7c4a;
    loop {
        match pc {
        0xb7c4a => {
            // block b7c4a
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7c4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c4a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7c4b mov rbp,rsp
            rbp = rsp;
            // b7c4e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b7c50 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7c52 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b7c54 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b7c56 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7c57 sub rsp,0x468
            { let (r, f) = sub::<64>(rsp, 0x468u64);
              rsp = r;
              fl = f; }
            // b7c5e mov r15,rcx
            r15 = rcx;
            // b7c61 mov r12,rdx
            r12 = rdx;
            // b7c64 mov rbx,rdi
            rbx = rdi;
            // b7c67 mov rax,QWORD PTR [rip+0x8876a] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // b7c6e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // b7c71 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // b7c75 test BYTE PTR [r12],0x1
            fl = logic::<8>(m.r8(r12) as u64 & 0x1u64);
            // b7c7a jne 0xb7c85
            if fl.ne() { pc = 0xb7c85; continue; }
            pc = 0xb7c7c;
        }
        0xb7c7c => {
            // block b7c7c
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7c7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c7c mov WORD PTR [r12],0x0
            m.w16(r12, (0x0u64) as u16);
            // b7c83 jmp 0xb7c96
            pc = 0xb7c96; continue;
        }
        0xb7c85 => {
            // block b7c85
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7c85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c85 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // b7c8a mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7c8d mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7c96;
        }
        0xb7c96 => {
            // block b7c96
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7c96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c96 test BYTE PTR [r15],0x1
            fl = logic::<8>(m.r8(r15) as u64 & 0x1u64);
            // b7c9a jne 0xb7ca4
            if fl.ne() { pc = 0xb7ca4; continue; }
            pc = 0xb7c9c;
        }
        0xb7c9c => {
            // block b7c9c
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7c9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c9c mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // b7ca2 jmp 0xb7cb3
            pc = 0xb7cb3; continue;
        }
        0xb7ca4 => {
            // block b7ca4
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7ca4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ca4 mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // b7ca8 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7cab mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7cb3;
        }
        0xb7cb3 => {
            // block b7cb3
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7cb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7cb3 mov DWORD PTR [rsi],0x0
            m.w32(rsi, (0x0u64) as u32);
            // b7cb9 cmp BYTE PTR [rbx+0xa],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0xa)) as u64, 0x0u64).1;
            // b7cbd je 0xb7d24
            if fl.e() { pc = 0xb7d24; continue; }
            pc = 0xb7cbf;
        }
        0xb7cbf => {
            // block b7cbf
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7cbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7cbf mov QWORD PTR [rbp-0x490],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb70), (rsi) as u64);
            // b7cc6 lea r14,[rbp-0x488]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fb78);
            // b7ccd mov esi,0x454
            rsi = (0x454u64) as u32 as u64;
            // b7cd2 mov rdi,r14
            rdi = r14;
            // b7cd5 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7cda mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b7cdd xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // b7ce0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b7ce2 mov rdi,rbx
            rdi = rbx;
            // b7ce5 mov rsi,r14
            rsi = r14;
            // b7ce8 call QWORD PTR [rax+0x140]
            { let r = { let t = m.r64(rax.wrapping_add(0x140)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7cee test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7cf0 je 0xb7d27
            if fl.e() { pc = 0xb7d27; continue; }
            pc = 0xb7cf2;
        }
        0xb7cf2 => {
            // block b7cf2
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7cf2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7cf2 lea rsi,[rbp-0x478]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb88);
            // b7cf9 mov rdi,r12
            rdi = r12;
            // b7cfc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7d01 lea rsi,[rbp-0x468]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb98);
            // b7d08 mov rdi,r15
            rdi = r15;
            // b7d0b call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7d10 mov eax,DWORD PTR [rbp-0x47c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fb84)) as u64) as u32 as u64;
            // b7d16 mov rcx,QWORD PTR [rbp-0x490]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb70)) as u64;
            // b7d1d mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            // b7d1f mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // b7d22 jmp 0xb7d27
            pc = 0xb7d27; continue;
        }
        0xb7d24 => {
            // block b7d24
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7d24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7d24 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xb7d27;
        }
        0xb7d27 => {
            // block b7d27
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7d27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7d27 mov rax,QWORD PTR [rip+0x886aa] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // b7d2e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // b7d31 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // b7d35 jne 0xb7d4c
            if fl.ne() { pc = 0xb7d4c; continue; }
            pc = 0xb7d37;
        }
        0xb7d37 => {
            // block b7d37
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7d37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7d37 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // b7d3a add rsp,0x468
            { let (r, f) = add::<64>(rsp, 0x468u64);
              rsp = r;
              fl = f; }
            // b7d41 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7d42 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b7d44 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b7d46 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7d48 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b7d4a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7d4b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb7d4c => {
            // block b7d4c
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7d4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7d4c call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6914 size=512
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6914<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb6914;
    loop {
        match pc {
        0xb6914 => {
            // block b6914
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6914 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6914 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6915 mov rbp,rsp
            rbp = rsp;
            // b6918 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b691a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b691c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b691d sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // b6921 mov rbx,rdi
            rbx = rdi;
            // b6924 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // b692b mov r14d,0xffffffff
            r14 = (0xffffffffu64) as u32 as u64;
            // b6931 cmp BYTE PTR [rbx+0xa],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0xa)) as u64, 0x0u64).1;
            // b6935 je 0xb6ad6
            if fl.e() { pc = 0xb6ad6; continue; }
            pc = 0xb693b;
        }
        0xb693b => {
            // block b693b
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b693b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b693b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b693e lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // b6942 mov r14d,0xffffffff
            r14 = (0xffffffffu64) as u32 as u64;
            // b6948 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // b694d mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b6952 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b6955 mov rdi,rbx
            rdi = rbx;
            // b6958 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b695b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b695d je 0xb6ad6
            if fl.e() { pc = 0xb6ad6; continue; }
            pc = 0xb6963;
        }
        0xb6963 => {
            // block b6963
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6963 mov r14d,DWORD PTR [rbp-0x24]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // b6967 cmp r14d,DWORD PTR [rbx+0x10]
            fl = sub::<32>((r14 as u32 as u64), m.r32(rbx.wrapping_add(0x10)) as u64).1;
            // b696b je 0xb6ad6
            if fl.e() { pc = 0xb6ad6; continue; }
            pc = 0xb6971;
        }
        0xb6971 => {
            // block b6971
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6971 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6971 mov rdi,QWORD PTR [rip+0x89808] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // b6978 lea rsi,[rip+0x813f8] # 0x137d77
            rsi = m.env.data_va(312644);
            // b697f mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // b6984 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6989 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b698c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b6990 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b6994 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b6997 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b699a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b699e mov rdi,rax
            rdi = rax;
            // b69a1 mov rsi,rbx
            rsi = rbx;
            // b69a4 call 0xec086
            { let r = { let t = m.env.stub_va(0x853); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b69a9 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b69ac mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b69b0 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b69b4 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b69b7 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b69ba mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b69be lea rsi,[rip+0x813d0] # 0x137d95
            rsi = m.env.data_va(312674);
            // b69c5 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // b69ca mov rdi,rax
            rdi = rax;
            // b69cd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b69d2 mov r15,rax
            r15 = rax;
            // b69d5 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b69d9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b69db mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // b69de call 0x5c0a9
            { let r = { let t = m.env.code_va(0x5c0a9); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b69e3 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // b69e7 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b69ea jne 0xb69f5
            if fl.ne() { pc = 0xb69f5; continue; }
            pc = 0xb69ec;
        }
        0xb69ec => {
            // block b69ec
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b69ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b69ec lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // b69f0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b69f3 jmp 0xb69fd
            pc = 0xb69fd; continue;
        }
        0xb69f5 => {
            // block b69f5
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b69f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b69f5 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b69f9 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xb69fd;
        }
        0xb69fd => {
            // block b69fd
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b69fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b69fd mov rdi,r15
            rdi = r15;
            // b6a00 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a05 lea rsi,[rip+0x8139a] # 0x137da6
            rsi = m.env.data_va(312691);
            // b6a0c mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // b6a11 mov rdi,rax
            rdi = rax;
            // b6a14 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a19 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // b6a1e mov rdi,rax
            rdi = rax;
            // b6a21 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a26 lea rsi,[rip+0x8138e] # 0x137dbb
            rsi = m.env.data_va(312712);
            // b6a2d mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // b6a32 mov rdi,rax
            rdi = rax;
            // b6a35 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a3a mov r15,rax
            r15 = rax;
            // b6a3d mov esi,DWORD PTR [rbx+0x10]
            rsi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b6a40 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b6a44 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b6a46 call 0x5c0a9
            { let r = { let t = m.env.code_va(0x5c0a9); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a4b movzx edx,BYTE PTR [rbp-0x58]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // b6a4f test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b6a52 jne 0xb6a5d
            if fl.ne() { pc = 0xb6a5d; continue; }
            pc = 0xb6a54;
        }
        0xb6a54 => {
            // block b6a54
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6a54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6a54 lea rsi,[rbp-0x57]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa9);
            // b6a58 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b6a5b jmp 0xb6a65
            pc = 0xb6a65; continue;
        }
        0xb6a5d => {
            // block b6a5d
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6a5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6a5d mov rdx,QWORD PTR [rbp-0x50]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // b6a61 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0xb6a65;
        }
        0xb6a65 => {
            // block b6a65
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6a65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6a65 mov rdi,r15
            rdi = r15;
            // b6a68 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a6d mov rbx,rax
            rbx = rax;
            // b6a70 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b6a73 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b6a77 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b6a7a lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // b6a7e call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a83 mov rsi,QWORD PTR [rip+0x89706] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b6a8a lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // b6a8e call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a93 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b6a96 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b6a9a mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b6a9f mov rdi,rax
            rdi = rax;
            // b6aa2 call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6aa4 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b6aa7 lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // b6aab call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6ab0 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b6ab4 mov rdi,rbx
            rdi = rbx;
            // b6ab7 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6abc mov rdi,rbx
            rdi = rbx;
            // b6abf call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6ac4 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b6ac8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6acd lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b6ad1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb6ad6;
        }
        0xb6ad6 => {
            // block b6ad6
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6ad6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6ad6 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // b6ad9 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // b6add pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b6ade pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b6ae0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b6ae2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6ae3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb6ae4 => {
            // block b6ae4
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6ae4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6ae4 mov rbx,rax
            rbx = rax;
            // b6ae7 jmp 0xb6b03
            pc = 0xb6b03; continue;
        }
        0xb6ae9 => {
            // block b6ae9
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6ae9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6ae9 mov rbx,rax
            rbx = rax;
            // b6aec jmp 0xb6afa
            pc = 0xb6afa; continue;
        }
        0xb6aee => {
            // block b6aee
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6aee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6aee mov rbx,rax
            rbx = rax;
            // b6af1 lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // b6af5 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb6afa;
        }
        0xb6afa => {
            // block b6afa
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6afa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6afa lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b6afe call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb6b03;
        }
        0xb6b03 => {
            // block b6b03
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6b03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6b03 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b6b07 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6b0c mov rdi,rbx
            rdi = rbx;
            // b6b0f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
