//! Mechanically lifted functions (scripts/lift_run.py), module t001. Each one is checked against the original
//! in the world harness (tests/world_t001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x29a94 size=754
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00029a94<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x29a94;
    loop {
        match pc {
        0x29a94 => {
            // block 29a94
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29a94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29a94 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 29a95 mov rbp,rsp
            rbp = rsp;
            // 29a98 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 29a9a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 29a9c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 29a9e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 29aa0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 29aa1 sub rsp,0x1f8
            { let (r, f) = sub::<64>(rsp, 0x1f8u64);
              rsp = r;
              fl = f; }
            // 29aa8 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 29aab mov r14,rsi
            r14 = rsi;
            // 29aae mov QWORD PTR [rbp-0x218],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fde8), (rdi) as u64);
            // 29ab5 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 29abc lea rbx,[rbp-0x140]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 29ac3 mov rax,QWORD PTR [rip+0x116706] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 29aca lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 29ace mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // 29ad5 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 29ad9 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 29ae0 mov rsi,rbx
            rsi = rbx;
            // 29ae3 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29ae8 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 29af0 mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // 29af7 mov r12,QWORD PTR [rip+0x1168c2] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 29afe lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 29b03 mov QWORD PTR [rbp-0x220],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fde0), (rax) as u64);
            // 29b0a mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 29b11 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 29b15 mov QWORD PTR [rbp-0xd8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r12) as u64);
            // 29b1c mov rdi,rbx
            rdi = rbx;
            // 29b1f call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29b24 mov rdi,rbx
            rdi = rbx;
            // 29b27 mov r13,QWORD PTR [rip+0x11687a] # 0x1403a8
            r13 = m.r64(m.env.data_va(346997)) as u64;
            // 29b2e add r13,0x10
            { let (r, f) = add::<64>(r13, 0x10u64);
              r13 = r;
              fl = f; }
            // 29b32 mov QWORD PTR [rbp-0x140],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r13) as u64);
            // 29b39 lea rbx,[rbp-0x100]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 29b40 mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // 29b4b mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // 29b56 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // 29b61 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // 29b6c mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // 29b76 xorps xmm0,xmm0
            x0 = 0;
            // 29b79 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 29b7d mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 29b85 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 29b89 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29b8e lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 29b92 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29b97 mov rax,QWORD PTR [r14+0x50]
            rax = m.r64(r14.wrapping_add(0x50)) as u64;
            // 29b9b mov DWORD PTR [rbp-0x158],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (0xffffffffu64) as u32);
            // 29ba5 mov DWORD PTR [rbp-0x154],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feac), (0xffffffffu64) as u32);
            // 29baf mov DWORD PTR [rbp-0x150],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (0xffffffffu64) as u32);
            // 29bb9 cmp r15d,0x1a
            fl = sub::<32>((r15 as u32 as u64), 0x1au64).1;
            // 29bbd ja 0x29c1b
            if fl.a() { pc = 0x29c1b; continue; }
            pc = 0x29bbf;
        }
        0x29bbf => {
            // block 29bbf
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29bbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29bbf test rax,rax
            fl = logic::<64>(rax & rax);
            // 29bc2 je 0x29c1b
            if fl.e() { pc = 0x29c1b; continue; }
            pc = 0x29bc4;
        }
        0x29bc4 => {
            // block 29bc4
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29bc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29bc4 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 29bc7 lea rcx,[rcx+rcx*2]
            rcx = rcx.wrapping_add(rcx.wrapping_mul(2));
            // 29bcb mov edx,DWORD PTR [rax+rcx*4+0x8]
            rdx = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x8)) as u64) as u32 as u64;
            // 29bcf mov DWORD PTR [rbp-0x150],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feb0), ((rdx as u32 as u64)) as u32);
            // 29bd5 mov rax,QWORD PTR [rax+rcx*4]
            rax = m.r64(rax.wrapping_add(rcx.wrapping_mul(4))) as u64;
            // 29bd9 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // 29be0 mov eax,DWORD PTR [rbp-0x154]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_feac)) as u64) as u32 as u64;
            // 29be6 and eax,DWORD PTR [rbp-0x158]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 29bec cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // 29bef mov rbx,QWORD PTR [rbp-0x218]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde8)) as u64;
            // 29bf6 jne 0x29c42
            if fl.ne() { pc = 0x29c42; continue; }
            pc = 0x29bf8;
        }
        0x29bf8 => {
            // block 29bf8
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29bf8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29bf8 cmp DWORD PTR [rbp-0x150],0xffffffff
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64, 0xffffffffu64).1;
            // 29bff jne 0x29c42
            if fl.ne() { pc = 0x29c42; continue; }
            pc = 0x29c01;
        }
        0x29c01 => {
            // block 29c01
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29c01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29c01 lea rsi,[rip+0x100cf1] # 0x12a8f9
            rsi = m.env.data_va(258246);
            // 29c08 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 29c0f mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 29c14 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29c19 jmp 0x29c79
            pc = 0x29c79; continue;
        }
        0x29c1b => {
            // block 29c1b
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29c1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29c1b cmp r15d,0x1a
            fl = sub::<32>((r15 as u32 as u64), 0x1au64).1;
            // 29c1f mov rbx,QWORD PTR [rbp-0x218]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde8)) as u64;
            // 29c26 ja 0x29c79
            if fl.a() { pc = 0x29c79; continue; }
            pc = 0x29c28;
        }
        0x29c28 => {
            // block 29c28
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29c28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29c28 lea rsi,[rip+0x100cca] # 0x12a8f9
            rsi = m.env.data_va(258246);
            // 29c2f lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 29c36 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 29c3b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29c40 jmp 0x29c79
            pc = 0x29c79; continue;
        }
        0x29c42 => {
            // block 29c42
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29c42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29c42 lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 29c49 lea rsi,[rbp-0x158]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 29c50 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 29c55 call 0xe5324
            { let r = { let t = m.env.code_va(0xe5324); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29c5a lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 29c61 lea rsi,[rbp-0x210]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 29c68 call 0xe7554
            { let r = { let t = m.env.code_va(0xe7554); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29c6d lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 29c74 call 0xe554e
            { let r = { let t = m.env.code_va(0xe554e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x29c79;
        }
        0x29c79 => {
            // block 29c79
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29c79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29c79 mov rdi,rbx
            rdi = rbx;
            // 29c7c lea rsi,[rbp-0x140]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 29c83 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29c88 mov rax,QWORD PTR [rbp-0x220]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde0)) as u64;
            // 29c8f mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 29c96 mov QWORD PTR [rbp-0xd8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r12) as u64);
            // 29c9d mov QWORD PTR [rbp-0x140],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r13) as u64);
            // 29ca4 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 29cab call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29cb0 lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 29cb7 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29cbc mov rsi,QWORD PTR [rip+0x116545] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 29cc3 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 29cc7 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 29cce call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29cd3 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 29cda call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29cdf mov rax,rbx
            rax = rbx;
            // 29ce2 add rsp,0x1f8
            { let (r, f) = add::<64>(rsp, 0x1f8u64);
              rsp = r;
              fl = f; }
            // 29ce9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 29cea pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 29cec pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 29cee pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 29cf0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 29cf2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29cf3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x29cf4 => {
            // block 29cf4
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29cf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29cf4 mov r14,rax
            r14 = rax;
            pc = 0x29cf7;
        }
        0x29cf7 => {
            // block 29cf7
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29cf7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29cf7 mov rax,QWORD PTR [rbp-0x220]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde0)) as u64;
            // 29cfe mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 29d05 mov QWORD PTR [rbp-0xd8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r12) as u64);
            // 29d0c mov QWORD PTR [rbp-0x140],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r13) as u64);
            // 29d13 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 29d1a jmp 0x29d35
            pc = 0x29d35; continue;
        }
        0x29d1c => {
            // block 29d1c
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29d1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d1c mov r14,rax
            r14 = rax;
            // 29d1f jmp 0x29d5d
            pc = 0x29d5d; continue;
        }
        0x29d21 => {
            // block 29d21
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d21 mov r14,rax
            r14 = rax;
            // 29d24 jmp 0x29d46
            pc = 0x29d46; continue;
        }
        0x29d26 => {
            // block 29d26
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29d26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d26 mov r14,rax
            r14 = rax;
            // 29d29 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 29d2d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29d32 mov rdi,rbx
            rdi = rbx;
            pc = 0x29d35;
        }
        0x29d35 => {
            // block 29d35
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29d35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d35 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29d3a lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 29d41 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x29d46;
        }
        0x29d46 => {
            // block 29d46
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29d46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d46 mov rsi,QWORD PTR [rip+0x1164bb] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 29d4d add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 29d51 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 29d58 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x29d5d;
        }
        0x29d5d => {
            // block 29d5d
            #[cfg(feature = "trace")] eprintln!("f_00029a94 pc 29d5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d5d lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // 29d64 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29d69 mov rdi,r14
            rdi = r14;
            // 29d6c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29d71 mov r14,rax
            r14 = rax;
            // 29d74 lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // 29d7b call 0xe554e
            { let r = { let t = m.env.code_va(0xe554e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29d80 jmp 0x29cf7
            pc = 0x29cf7; continue;
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
            if fl.ne() { pc = 0xe5aed; continue; }
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
            { let r = { let t = m.env.stub_va(0xa69); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xe5b2d; continue; }
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
            { let r = { let t = m.env.stub_va(0xa69); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xe5b6a; continue; }
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
            { let r = { let t = m.env.stub_va(0xa69); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xe5ba7; continue; }
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
            { let r = { let t = m.env.stub_va(0xa69); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xe5be1; continue; }
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
            { let r = { let t = m.env.stub_va(0xa69); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xe5c1e; continue; }
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
            { let r = { let t = m.env.stub_va(0xa69); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xe5c62; continue; }
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
            if fl.ae() { pc = 0xe5c92; continue; }
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
            { let r = { let t = m.env.stub_va(0xa69); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e5c87 test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // e5c8b jne 0xe5cc0
            if fl.ne() { pc = 0xe5cc0; continue; }
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
            { let r = { let t = m.env.stub_va(0xa69); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xe5d6d; continue; }
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
            { let r = { let t = m.env.stub_va(0xa69); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.a() { pc = 0xe5d09; continue; }
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
            if fl.b() { pc = 0xe5d2f; continue; }
            pc = 0xe5d1c;
        }
        0xe5d1c => {
            // block e5d1c
            #[cfg(feature = "trace")] eprintln!("f_000e5ab6 pc e5d1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5d1c test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // e5d1f jne 0xe5d2f
            if fl.ne() { pc = 0xe5d2f; continue; }
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
            { let r = { let t = m.env.code_va(0xe5ea8); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0xa69); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0xc7c7f size=789
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c7c7f<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc7c7f;
    loop {
        match pc {
        0xc7c7f => {
            // block c7c7f
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7c7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7c7f push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c7c80 mov rbp,rsp
            rbp = rsp;
            // c7c83 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c7c85 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c7c86 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // c7c89 mov rbx,rsi
            rbx = rsi;
            // c7c8c dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // c7c8e cmp ecx,0x1c
            fl = sub::<32>((rcx as u32 as u64), 0x1cu64).1;
            // c7c91 ja 0xc7f19
            if fl.a() { pc = 0xc7f19; continue; }
            pc = 0xc7c97;
        }
        0xc7c97 => {
            // block c7c97
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7c97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7c97 lea rax,[rip+0x282] # 0xc7f20
            rax = m.env.code_va(0xc7f20);
            // c7c9e movsxd rcx,DWORD PTR [rax+rcx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // c7ca2 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // c7ca5 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc7ca7 => {
            // block c7ca7
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7ca7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7ca7 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7cae sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7cb0 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7cb3 mov rsi,rbx
            rsi = rbx;
            // c7cb6 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7cb9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7cba pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7cbc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7cbd jmp 0xc70f4
            { let r = { let t = m.env.code_va(0xc70f4); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc7cc2 => {
            // block c7cc2
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7cc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7cc2 mov rsi,rbx
            rsi = rbx;
            // c7cc5 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7cc8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7cc9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7ccb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7ccc jmp 0xc5d09
            { let r = { let t = m.env.code_va(0xc5d09); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc7cd1 => {
            // block c7cd1
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7cd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7cd1 test r14d,0x7fffffff
            fl = logic::<32>((r14 as u32 as u64) & 0x7fffffffu64);
            // c7cd8 je 0xc7f19
            if fl.e() { pc = 0xc7f19; continue; }
            pc = 0xc7cde;
        }
        0xc7cde => {
            // block c7cde
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7cde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7cde add r14d,r14d
            { let (r, f) = add::<32>((r14 as u32 as u64), (r14 as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc7ce1;
        }
        0xc7ce1 => {
            // block c7ce1
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7ce1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7ce1 movzx eax,WORD PTR [rdi]
            rax = (m.r16(rdi) as u64) as u32 as u64;
            // c7ce4 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c7ce7 mov BYTE PTR [rbx],al
            m.w8(rbx, ((rax as u8 as u64)) as u8);
            // c7ce9 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // c7cec add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // c7cf0 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // c7cf3 jne 0xc7ce1
            if fl.ne() { pc = 0xc7ce1; continue; }
            pc = 0xc7cf5;
        }
        0xc7cf5 => {
            // block c7cf5
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7cf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7cf5 jmp 0xc7f19
            pc = 0xc7f19; continue;
        }
        0xc7cfa => {
            // block c7cfa
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7cfa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7cfa cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7d01 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d03 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d06 mov rsi,rbx
            rsi = rbx;
            // c7d09 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7d0c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7d0d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7d0f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7d10 jmp 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc7d15 => {
            // block c7d15
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d15 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7d1c sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d1e and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d21 mov rsi,rbx
            rsi = rbx;
            // c7d24 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7d27 call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7d2c mov rdi,rbx
            rdi = rbx;
            // c7d2f mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7d32 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7d33 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7d35 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7d36 jmp 0xc7a37
            { let r = { let t = m.env.code_va(0xc7a37); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc7d3b => {
            // block c7d3b
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d3b cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7d42 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d44 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d47 mov rsi,rbx
            rsi = rbx;
            // c7d4a mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7d4d call 0xc6487
            { let r = { let t = m.env.code_va(0xc6487); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7d52 mov rdi,rbx
            rdi = rbx;
            // c7d55 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7d58 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7d59 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7d5b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7d5c jmp 0xc601b
            { let r = { let t = m.env.code_va(0xc601b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc7d61 => {
            // block c7d61
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d61 test r14d,0x7fffffff
            fl = logic::<32>((r14 as u32 as u64) & 0x7fffffffu64);
            // c7d68 je 0xc7d84
            if fl.e() { pc = 0xc7d84; continue; }
            pc = 0xc7d6a;
        }
        0xc7d6a => {
            // block c7d6a
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d6a lea eax,[r14+r14*1]
            rax = (r14.wrapping_add(r14)) as u32 as u64;
            // c7d6e mov rcx,rbx
            rcx = rbx;
            pc = 0xc7d71;
        }
        0xc7d71 => {
            // block c7d71
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d71 movzx edx,WORD PTR [rdi]
            rdx = (m.r16(rdi) as u64) as u32 as u64;
            // c7d74 shr edx,0x2
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // c7d77 mov BYTE PTR [rcx],dl
            m.w8(rcx, ((rdx as u8 as u64)) as u8);
            // c7d79 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // c7d7c add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // c7d80 dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // c7d82 jne 0xc7d71
            if fl.ne() { pc = 0xc7d71; continue; }
            pc = 0xc7d84;
        }
        0xc7d84 => {
            // block c7d84
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d84 mov rdi,rbx
            rdi = rbx;
            // c7d87 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7d8a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7d8b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7d8d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7d8e jmp 0xc7bec
            { let r = { let t = m.env.code_va(0xc7bec); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc7d93 => {
            // block c7d93
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7d93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7d93 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7d9a sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d9c and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d9f mov rsi,rbx
            rsi = rbx;
            // c7da2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7da5 call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7daa shl r14d,0x2
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x2u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c7dae test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c7db1 je 0xc7f19
            if fl.e() { pc = 0xc7f19; continue; }
            pc = 0xc7db7;
        }
        0xc7db7 => {
            // block c7db7
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7db7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7db7 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7dba xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xc7dbc;
        }
        0xc7dbc => {
            // block c7dbc
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7dbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7dbc mov dl,BYTE PTR [rbx+rcx*1]
            rdx = (rdx & !0xff) | ((m.r8(rbx.wrapping_add(rcx)) as u64) as u8 as u64);
            // c7dbf mov al,BYTE PTR [rbx+rcx*1+0x2]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(rcx).wrapping_add(0x2)) as u64) as u8 as u64);
            // c7dc3 mov BYTE PTR [rbx+rcx*1],al
            m.w8(rbx.wrapping_add(rcx), ((rax as u8 as u64)) as u8);
            // c7dc6 mov BYTE PTR [rbx+rcx*1+0x2],dl
            m.w8(rbx.wrapping_add(rcx).wrapping_add(0x2), ((rdx as u8 as u64)) as u8);
            // c7dca add rcx,0x4
            { let (r, f) = add::<64>(rcx, 0x4u64);
              rcx = r;
              fl = f; }
            // c7dce cmp rcx,rsi
            fl = sub::<64>(rcx, rsi).1;
            // c7dd1 jb 0xc7dbc
            if fl.b() { pc = 0xc7dbc; continue; }
            pc = 0xc7dd3;
        }
        0xc7dd3 => {
            // block c7dd3
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7dd3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7dd3 jmp 0xc7f19
            pc = 0xc7f19; continue;
        }
        0xc7dd8 => {
            // block c7dd8
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7dd8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7dd8 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7ddf sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7de1 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7de4 mov rsi,rbx
            rsi = rbx;
            // c7de7 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7dea call 0xc6487
            { let r = { let t = m.env.code_va(0xc6487); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7def mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // c7df4 jmp 0xc7e75
            pc = 0xc7e75; continue;
        }
        0xc7df6 => {
            // block c7df6
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7df6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7df6 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // c7dfb mov rsi,rbx
            rsi = rbx;
            // c7dfe mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e01 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7e02 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7e04 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7e05 jmp 0xc5ddf
            { let r = { let t = m.env.code_va(0xc5ddf); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc7e0a => {
            // block c7e0a
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7e0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7e0a cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7e11 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e13 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e16 mov rsi,rbx
            rsi = rbx;
            // c7e19 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e1c call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7e21 mov rdi,rbx
            rdi = rbx;
            // c7e24 mov rsi,rbx
            rsi = rbx;
            // c7e27 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e2a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7e2b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7e2d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7e2e jmp 0xc7a9b
            { let r = { let t = m.env.code_va(0xc7a9b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc7e33 => {
            // block c7e33
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7e33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7e33 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7e3a sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e3c and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e3f mov rsi,rbx
            rsi = rbx;
            // c7e42 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e45 call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7e4a mov rdi,rbx
            rdi = rbx;
            // c7e4d mov rsi,rbx
            rsi = rbx;
            // c7e50 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e53 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7e54 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7e56 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7e57 jmp 0xc7acf
            { let r = { let t = m.env.code_va(0xc7acf); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc7e5c => {
            // block c7e5c
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7e5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7e5c cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7e63 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e65 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e68 mov rsi,rbx
            rsi = rbx;
            // c7e6b mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e6e call 0xc6487
            { let r = { let t = m.env.code_va(0xc6487); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7e73 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xc7e75;
        }
        0xc7e75 => {
            // block c7e75
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7e75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7e75 mov rdi,rbx
            rdi = rbx;
            // c7e78 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7e7b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7e7c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7e7e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7e7f jmp 0xc60bc
            { let r = { let t = m.env.code_va(0xc60bc); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc7e84 => {
            // block c7e84
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7e84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7e84 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7e8b sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e8d and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e90 mov rsi,rbx
            rsi = rbx;
            // c7e93 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e96 call 0xc70f4
            { let r = { let t = m.env.code_va(0xc70f4); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7e9b mov rdi,rbx
            rdi = rbx;
            // c7e9e mov rsi,rbx
            rsi = rbx;
            // c7ea1 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7ea4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7ea5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7ea7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7ea8 jmp 0xc7b35
            { let r = { let t = m.env.code_va(0xc7b35); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc7ead => {
            // block c7ead
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7ead rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7ead cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7eb4 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7eb6 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7eb9 mov rsi,rbx
            rsi = rbx;
            // c7ebc mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7ebf call 0xc6487
            { let r = { let t = m.env.code_va(0xc6487); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7ec4 mov rdi,rbx
            rdi = rbx;
            // c7ec7 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7eca pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7ecb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7ecd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7ece jmp 0xc604d
            { let r = { let t = m.env.code_va(0xc604d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc7ed3 => {
            // block c7ed3
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7ed3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7ed3 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7eda sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7edc and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7edf mov rsi,rbx
            rsi = rbx;
            // c7ee2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7ee5 call 0xc70f4
            { let r = { let t = m.env.code_va(0xc70f4); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7eea test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c7eed je 0xc7f19
            if fl.e() { pc = 0xc7f19; continue; }
            pc = 0xc7eef;
        }
        0xc7eef => {
            // block c7eef
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7eef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7eef mov rax,rbx
            rax = rbx;
            // c7ef2 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            pc = 0xc7ef6;
        }
        0xc7ef6 => {
            // block c7ef6
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7ef6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7ef6 mov cx,WORD PTR [rax-0x4]
            rcx = (rcx & !0xffff) | ((m.r16(rax.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u16 as u64);
            // c7efa mov dx,WORD PTR [rax-0x2]
            rdx = (rdx & !0xffff) | ((m.r16(rax.wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u16 as u64);
            // c7efe mov si,WORD PTR [rax]
            rsi = (rsi & !0xffff) | ((m.r16(rax) as u64) as u16 as u64);
            // c7f01 mov WORD PTR [rbx],si
            m.w16(rbx, ((rsi as u16 as u64)) as u16);
            // c7f04 mov WORD PTR [rbx+0x2],dx
            m.w16(rbx.wrapping_add(0x2), ((rdx as u16 as u64)) as u16);
            // c7f08 mov WORD PTR [rbx+0x4],cx
            m.w16(rbx.wrapping_add(0x4), ((rcx as u16 as u64)) as u16);
            // c7f0c add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c7f10 add rbx,0x6
            { let (r, f) = add::<64>(rbx, 0x6u64);
              rbx = r;
              fl = f; }
            // c7f14 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // c7f17 jne 0xc7ef6
            if fl.ne() { pc = 0xc7ef6; continue; }
            pc = 0xc7f19;
        }
        0xc7f19 => {
            // block c7f19
            #[cfg(feature = "trace")] eprintln!("f_000c7c7f pc c7f19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7f19 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7f1a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7f1c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7f1d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x39c0 size=796
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
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
            { let r = { let t = m.env.code_va(0x12b8c); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 39f4 mov rdi,rbx
            rdi = rbx;
            // 39f7 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 39fc lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 3a03 mov rsi,rbx
            rsi = rbx;
            // 3a06 mov rdx,rax
            rdx = rax;
            // 3a09 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3a0e lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3a15 lea rsi,[rbp-0x340]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 3a1c call 0x12c80
            { let r = { let t = m.env.code_va(0x12c80); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3a21 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 3a23 lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 3a2a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3a2f test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // 3a31 je 0x3c1e
            if fl.e() { pc = 0x3c1e; continue; }
            pc = 0x3a37;
        }
        0x3a37 => {
            // block 3a37
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3a37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3a37 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3a3e call 0x136b6
            { let r = { let t = m.env.code_va(0x136b6); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3a43 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // 3a46 lea r12d,[r14+0x200]
            r12 = (r14.wrapping_add(0x200)) as u32 as u64;
            // 3a4d mov rdi,r12
            rdi = r12;
            // 3a50 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x9f7); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3a6f lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3a76 mov rsi,rbx
            rsi = rbx;
            // 3a79 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 3a7c call 0x1378a
            { let r = { let t = m.env.code_va(0x1378a); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3a81 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 3a83 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 3a8a call 0x12b96
            { let r = { let t = m.env.code_va(0x12b96); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3a8f cmp ebx,r14d
            fl = sub::<32>((rbx as u32 as u64), (r14 as u32 as u64)).1;
            // 3a92 jne 0x3c12
            if fl.ne() { pc = 0x3c12; continue; }
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
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3ab7 add r14d,0x4
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x4u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 3abb shr r14d,0x2
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x2u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 3abf je 0x3c22
            if fl.e() { pc = 0x3c22; continue; }
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
            { let r = { let t = m.env.code_va(0xce3be); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0x3b43; continue; }
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
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3b7e mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3b82 mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 3b89 mov rdi,r13
            rdi = r13;
            // 3b8c call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3baf mov eax,DWORD PTR [rbp-0x344]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcbc)) as u64) as u32 as u64;
            // 3bb5 mov rcx,QWORD PTR [rbp-0x350]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcb0)) as u64;
            // 3bbc cmp eax,DWORD PTR [rcx+r15*4]
            fl = sub::<32>((rax as u32 as u64), m.r32(rcx.wrapping_add(r15.wrapping_mul(4))) as u64).1;
            // 3bc0 je 0x3bcb
            if fl.e() { pc = 0x3bcb; continue; }
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
            if fl.a() { pc = 0x3c06; continue; }
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
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.b() { pc = 0x3ad8; continue; }
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
            { let r = { let t = m.env.stub_va(0x901); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xce3be); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3c31 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 3c33 je 0x3c80
            if fl.e() { pc = 0x3c80; continue; }
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
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3c70 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 3c74 mov rax,QWORD PTR [rax+0x9e0]
            rax = m.r64(rax.wrapping_add(0x9e0)) as u64;
            // 3c7b mov rdi,r13
            rdi = r13;
            // 3c7e call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x14c0c); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3c92 cmp r15,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 3c96 jne 0x3cd6
            if fl.ne() { pc = 0x3cd6; continue; }
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
            { let r = { let t = m.env.code_va(0x14c0c); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3cbb mov rdi,rbx
            rdi = rbx;
            // 3cbe call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3cd4 jmp 0x3caf
            pc = 0x3caf; continue;
        }
        0x3cd6 => {
            // block 3cd6
            #[cfg(feature = "trace")] eprintln!("f_000039c0 pc 3cd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3cd6 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
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
            if fl.e() { pc = 0x29181; continue; }
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
            if fl.e() { pc = 0x29181; continue; }
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
            if fl.e() { pc = 0x29181; continue; }
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
            if fl.e() { pc = 0x29184; continue; }
            pc = 0x29170;
        }
        0x29170 => {
            // block 29170
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 29170 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29170 cmp esi,edx
            fl = sub::<32>((rsi as u32 as u64), (rdx as u32 as u64)).1;
            // 29172 jne 0x29184
            if fl.ne() { pc = 0x29184; continue; }
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
            if fl.e() { pc = 0x29184; continue; }
            pc = 0x2917c;
        }
        0x2917c => {
            // block 2917c
            #[cfg(feature = "trace")] eprintln!("f_0002912e pc 2917c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2917c cmp edx,0x3
            fl = sub::<32>((rdx as u32 as u64), 0x3u64).1;
            // 2917f jae 0x29196
            if fl.ae() { pc = 0x29196; continue; }
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
            if fl.e() { pc = 0x29184; continue; }
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
            if fl.ne() { pc = 0x291c5; continue; }
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
            if fl.b() { pc = 0x291a7; continue; }
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
            if fl.ne() { pc = 0x29210; continue; }
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
            if fl.e() { pc = 0x291ff; continue; }
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
            if fl.e() { pc = 0x291fb; continue; }
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
            if fl.b() { pc = 0x291dc; continue; }
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
            if fl.ae() { pc = 0x2920e; continue; }
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
            if fl.e() { pc = 0x29184; continue; }
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
            if fl.e() { pc = 0x29268; continue; }
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
            if fl.ne() { pc = 0x29273; continue; }
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
            if fl.ne() { pc = 0x29238; continue; }
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
            if fl.ne() { pc = 0x29184; continue; }
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
            if fl.e() { pc = 0x292bc; continue; }
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
            if fl.e() { pc = 0x292b9; continue; }
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
            if fl.ne() { pc = 0x2929d; continue; }
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
            if fl.ae() { pc = 0x292c6; continue; }
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
            if fl.a() { pc = 0x29388; continue; }
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
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 292ea mov esi,DWORD PTR [r15]
            rsi = (m.r32(r15) as u64) as u32 as u64;
            // 292ed mov rdi,rax
            rdi = rax;
            // 292f0 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 292f5 lea rsi,[rip+0x1015d4] # 0x12a8d0
            rsi = m.env.data_va(258205);
            // 292fc mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 29301 mov rdi,rax
            rdi = rax;
            // 29304 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29309 mov esi,DWORD PTR [r13+0x0]
            rsi = (m.r32(r13) as u64) as u32 as u64;
            // 2930d mov rdi,rax
            rdi = rax;
            // 29310 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29315 lea rsi,[rip+0x1015bf] # 0x12a8db
            rsi = m.env.data_va(258216);
            // 2931c mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 29321 mov rdi,rax
            rdi = rax;
            // 29324 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29343 mov rsi,QWORD PTR [rip+0x116e46] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 2934a mov rdi,rbx
            rdi = rbx;
            // 2934d call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29352 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 29355 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 29359 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 2935e mov rdi,rax
            rdi = rax;
            // 29361 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29363 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 29365 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 29369 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2936e movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // 29371 mov rdi,r12
            rdi = r12;
            // 29374 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29379 mov rdi,r12
            rdi = r12;
            // 2937c call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.b() { pc = 0x29184; continue; }
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
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 293b0 mov esi,DWORD PTR [r15]
            rsi = (m.r32(r15) as u64) as u32 as u64;
            // 293b3 mov rdi,rax
            rdi = rax;
            // 293b6 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 293bb lea rsi,[rip+0x10152c] # 0x12a8ee
            rsi = m.env.data_va(258235);
            // 293c2 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 293c7 mov rdi,rax
            rdi = rax;
            // 293ca call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 293cf mov esi,DWORD PTR [r13+0x0]
            rsi = (m.r32(r13) as u64) as u32 as u64;
            // 293d3 mov rdi,rax
            rdi = rax;
            // 293d6 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 293db lea rsi,[rip+0x1014f9] # 0x12a8db
            rsi = m.env.data_va(258216);
            // 293e2 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 293e7 mov rdi,rax
            rdi = rax;
            // 293ea call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29408 mov rsi,QWORD PTR [rip+0x116d81] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 2940f mov rdi,r15
            rdi = r15;
            // 29412 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29417 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2941a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 2941e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 29423 mov rdi,rax
            rdi = rax;
            // 29426 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29428 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2942b lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 2942f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29434 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // 29438 mov rdi,rbx
            rdi = rbx;
            // 2943b call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29440 mov rdi,rbx
            rdi = rbx;
            // 29443 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29462 mov rdi,rbx
            rdi = rbx;
            // 29465 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
