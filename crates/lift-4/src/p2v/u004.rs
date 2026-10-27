//! Mechanically lifted functions (scripts/lift_run.py), module u004. Each one is checked against the original
//! in the world harness (tests/world_u004.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x167fc size=2078
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000167fc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x167fc;
    loop {
        match pc {
        0x167fc => {
            // block 167fc
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 167fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 167fc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 167fd mov rbp,rsp
            rbp = rsp;
            // 16800 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 16802 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 16804 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 16806 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 16808 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 16809 sub rsp,0x548
            { let (r, f) = sub::<64>(rsp, 0x548u64);
              rsp = r;
              fl = f; }
            // 16810 mov QWORD PTR [rbp-0x530],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fad0), (rdi) as u64);
            // 16817 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 1681a call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1681d mov r14,rax
            r14 = rax;
            // 16820 mov r15,QWORD PTR [r14]
            r15 = m.r64(r14) as u64;
            // 16823 jmp 0x1686f
            pc = 0x1686f; continue;
        }
        0x16825 => {
            // block 16825
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16825 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16825 lea rax,[rbx-0xf8]
            rax = rbx.wrapping_add(0xffff_ffff_ffff_ff08);
            // 1682c mov QWORD PTR [r14+0x8],rax
            m.w64(r14.wrapping_add(0x8), (rax) as u64);
            // 16830 lea rdi,[rbx-0x38]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 16834 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16839 lea rdi,[rbx-0x50]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1683d call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16842 lea rdi,[rbx-0x68]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff98);
            // 16846 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1684b lea rdi,[rbx-0x80]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff80);
            // 1684f call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16854 lea rdi,[rbx-0x98]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff68);
            // 1685b call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16860 add rbx,0xffffffffffffff20
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ff20u64);
              rbx = r;
              fl = f; }
            // 16867 mov rdi,rbx
            rdi = rbx;
            // 1686a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1686f;
        }
        0x1686f => {
            // block 1686f
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 1686f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1686f mov rbx,QWORD PTR [r14+0x8]
            rbx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 16873 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 16876 jne 0x16825
            if fl.ne() { pc = 0x16825; continue; }
            pc = 0x16878;
        }
        0x16878 => {
            // block 16878
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16878 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16878 lea r12,[rbp-0x388]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fc78);
            // 1687f mov rax,QWORD PTR [rip+0x129b22] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 16886 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 1688a mov QWORD PTR [rbp-0x510],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_faf0), (rax) as u64);
            // 16891 mov rax,QWORD PTR [rip+0x129970] # 0x140208
            rax = m.r64(m.env.data_va(346581)) as u64;
            // 16898 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 1689c mov QWORD PTR [rbp-0x528],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fad8), (rax) as u64);
            // 168a3 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 168a5 lea r15,[rbp-0x2c8]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 168ac jmp 0x168bf
            pc = 0x168bf; continue;
        }
        0x168ae => {
            // block 168ae
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 168ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 168ae mov rdi,r15
            rdi = r15;
            // 168b1 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 168b6 mov rbx,QWORD PTR [rbp-0x508]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_faf8)) as u64;
            // 168bd inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            pc = 0x168bf;
        }
        0x168bf => {
            // block 168bf
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 168bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 168bf mov QWORD PTR [rbp-0x508],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_faf8), (rbx) as u64);
            // 168c6 mov rdi,r15
            rdi = r15;
            // 168c9 call 0xb6722
            { let r = { let t = m.env.code_va(0xb6722); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 168ce mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 168d5 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 168d9 movzx r13d,bx
            r13 = ((rbx as u16 as u64)) as u32 as u64;
            // 168dd xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 168df xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 168e1 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 168e4 mov rdi,r15
            rdi = r15;
            // 168e7 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 168ea call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 168ec test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 168ee je 0x16e6c
            if fl.e() { pc = 0x16e6c; continue; }
            pc = 0x168f4;
        }
        0x168f4 => {
            // block 168f4
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 168f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 168f4 mov rdi,r15
            rdi = r15;
            // 168f7 call 0xb6914
            { let r = { let t = m.env.code_va(0xb6914); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 168fc mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // 168ff cmp r14d,0xffffffff
            fl = sub::<32>((r14 as u32 as u64), 0xffffffffu64).1;
            // 16903 je 0x16e4f
            if fl.e() { pc = 0x16e4f; continue; }
            pc = 0x16909;
        }
        0x16909 => {
            // block 16909
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16909 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16909 mov rax,QWORD PTR [rip+0x1298c0] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 16910 mov rcx,rax
            rcx = rax;
            // 16913 lea rax,[rcx+0x18]
            rax = rcx.wrapping_add(0x18);
            // 16917 mov QWORD PTR [rbp-0x3d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc30), (rax) as u64);
            // 1691e lea rax,[rcx+0x40]
            rax = rcx.wrapping_add(0x40);
            // 16922 mov QWORD PTR [rbp-0x360],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fca0), (rax) as u64);
            // 16929 lea rdi,[rbp-0x360]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fca0);
            // 16930 lea rbx,[rbp-0x3c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc38);
            // 16937 mov rsi,rbx
            rsi = rbx;
            // 1693a call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1693f mov QWORD PTR [rbp-0x2d8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd28), (0x0u64) as u64);
            // 1694a mov DWORD PTR [rbp-0x2d0],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd30), (0xffffffffu64) as u32);
            // 16954 mov rax,QWORD PTR [rip+0x129a65] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 1695b mov rcx,rax
            rcx = rax;
            // 1695e lea rax,[rcx+0x18]
            rax = rcx.wrapping_add(0x18);
            // 16962 mov QWORD PTR [rbp-0x518],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fae8), (rax) as u64);
            // 16969 mov QWORD PTR [rbp-0x3d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc30), (rax) as u64);
            // 16970 lea rax,[rcx+0x40]
            rax = rcx.wrapping_add(0x40);
            // 16974 mov QWORD PTR [rbp-0x520],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fae0), (rax) as u64);
            // 1697b mov QWORD PTR [rbp-0x360],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fca0), (rax) as u64);
            // 16982 mov rdi,rbx
            rdi = rbx;
            // 16985 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1698a mov rax,QWORD PTR [rbp-0x510]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_faf0)) as u64;
            // 16991 mov QWORD PTR [rbp-0x3c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc38), (rax) as u64);
            // 16998 mov QWORD PTR [r12+0x18],0x0
            m.w64(r12.wrapping_add(0x18), (0x0u64) as u64);
            // 169a1 mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 169aa mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 169b3 mov QWORD PTR [r12],0x0
            m.w64(r12, (0x0u64) as u64);
            // 169bb mov DWORD PTR [rbp-0x368],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc98), (0x10u64) as u32);
            // 169c5 xorps xmm0,xmm0
            x0 = 0;
            // 169c8 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 169cc mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 169d4 mov rdi,rbx
            rdi = rbx;
            // 169d7 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 169db mov rsi,rbx
            rsi = rbx;
            // 169de call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 169e3 mov rdi,rbx
            rdi = rbx;
            // 169e6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 169eb lea rax,[rbp-0x4b0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // 169f2 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 169fa mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 16a02 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 16a09 lea rax,[rbp-0x468]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_fb98);
            // 16a10 mov QWORD PTR [rax+0x70],0x0
            m.w64(rax.wrapping_add(0x70), (0x0u64) as u64);
            // 16a18 mov QWORD PTR [rax+0x68],0x0
            m.w64(rax.wrapping_add(0x68), (0x0u64) as u64);
            // 16a20 mov QWORD PTR [rax+0x60],0x0
            m.w64(rax.wrapping_add(0x60), (0x0u64) as u64);
            // 16a28 mov QWORD PTR [rax+0x58],0x0
            m.w64(rax.wrapping_add(0x58), (0x0u64) as u64);
            // 16a30 mov QWORD PTR [rax+0x50],0x0
            m.w64(rax.wrapping_add(0x50), (0x0u64) as u64);
            // 16a38 mov QWORD PTR [rax+0x48],0x0
            m.w64(rax.wrapping_add(0x48), (0x0u64) as u64);
            // 16a40 mov QWORD PTR [rax+0x40],0x0
            m.w64(rax.wrapping_add(0x40), (0x0u64) as u64);
            // 16a48 mov QWORD PTR [rax+0x38],0x0
            m.w64(rax.wrapping_add(0x38), (0x0u64) as u64);
            // 16a50 mov QWORD PTR [rax+0x30],0x0
            m.w64(rax.wrapping_add(0x30), (0x0u64) as u64);
            // 16a58 mov QWORD PTR [rax+0x28],0x0
            m.w64(rax.wrapping_add(0x28), (0x0u64) as u64);
            // 16a60 mov QWORD PTR [rax+0x20],0x0
            m.w64(rax.wrapping_add(0x20), (0x0u64) as u64);
            // 16a68 mov QWORD PTR [rax+0x18],0x0
            m.w64(rax.wrapping_add(0x18), (0x0u64) as u64);
            // 16a70 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 16a78 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 16a80 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 16a87 mov rdi,r15
            rdi = r15;
            // 16a8a call 0xb818a
            { let r = { let t = m.env.code_va(0xb818a); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16a8f mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 16a91 mov DWORD PTR [rbp-0x4c4],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb3c), ((r13 as u32 as u64)) as u32);
            // 16a98 mov DWORD PTR [rbp-0x4c8],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb38), ((r14 as u32 as u64)) as u32);
            // 16a9f mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 16aa6 mov rax,QWORD PTR [rax+0x118]
            rax = m.r64(rax.wrapping_add(0x118)) as u64;
            // 16aad lea rdi,[rbp-0x2c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 16ab4 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16ab6 mov DWORD PTR [rbp-0x4c0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb40), ((rax as u32 as u64)) as u32);
            // 16abc lea rdi,[rbp-0x2c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 16ac3 call 0xb796e
            { let r = { let t = m.env.code_va(0xb796e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16ac8 mov QWORD PTR [rbp-0x4b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb48), (rax) as u64);
            // 16acf movzx edx,bl
            rdx = ((rbx as u8 as u64)) as u32 as u64;
            // 16ad2 lea rdi,[rbp-0x4e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb20);
            // 16ad9 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 16adc call 0x5c0a9
            { let r = { let t = m.env.code_va(0x5c0a9); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16ae1 movzx edx,BYTE PTR [rbp-0x4e0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fb20)) as u64) as u32 as u64;
            // 16ae8 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 16aeb jne 0x16af9
            if fl.ne() { pc = 0x16af9; continue; }
            pc = 0x16aed;
        }
        0x16aed => {
            // block 16aed
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16aed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16aed shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 16af0 lea rsi,[rbp-0x4df]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb21);
            // 16af7 jmp 0x16b07
            pc = 0x16b07; continue;
        }
        0x16af9 => {
            // block 16af9
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16af9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16af9 mov rdx,QWORD PTR [rbp-0x4d8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb28)) as u64;
            // 16b00 mov rsi,QWORD PTR [rbp-0x4d0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb30)) as u64;
            pc = 0x16b07;
        }
        0x16b07 => {
            // block 16b07
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16b07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16b07 lea rdi,[rbp-0x3d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc30);
            // 16b0e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16b13 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 16b18 mov rdi,rax
            rdi = rax;
            // 16b1b lea rsi,[rip+0x113766] # 0x12a288
            rsi = m.env.data_va(256597);
            // 16b22 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16b27 mov rdi,rax
            rdi = rax;
            // 16b2a mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 16b2d call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16b32 lea rdi,[rbp-0x4e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb20);
            // 16b39 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16b3e cmp DWORD PTR [rbp-0x4c0],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fb40)) as u64, 0x0u64).1;
            // 16b45 je 0x16b6d
            if fl.e() { pc = 0x16b6d; continue; }
            pc = 0x16b47;
        }
        0x16b47 => {
            // block 16b47
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16b47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16b47 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 16b4c lea rdi,[rbp-0x3d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc30);
            // 16b53 lea rsi,[rip+0x113732] # 0x12a28c
            rsi = m.env.data_va(256601);
            // 16b5a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16b5f mov esi,DWORD PTR [rbp-0x4c0]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fb40)) as u64) as u32 as u64;
            // 16b65 mov rdi,rax
            rdi = rax;
            // 16b68 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16b6d;
        }
        0x16b6d => {
            // block 16b6d
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16b6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16b6d lea rdi,[rbp-0x500]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb00);
            // 16b74 lea rsi,[rbp-0x3c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fc38);
            // 16b7b call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16b80 test BYTE PTR [rbp-0x4b0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fb50)) as u64 & 0x1u64);
            // 16b87 jne 0x16b99
            if fl.ne() { pc = 0x16b99; continue; }
            pc = 0x16b89;
        }
        0x16b89 => {
            // block 16b89
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16b89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16b89 mov BYTE PTR [rbp-0x4af],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fb51), (0x0u64) as u8);
            // 16b90 mov BYTE PTR [rbp-0x4b0],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fb50), (0x0u64) as u8);
            // 16b97 jmp 0x16bae
            pc = 0x16bae; continue;
        }
        0x16b99 => {
            // block 16b99
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16b99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16b99 mov rax,QWORD PTR [rbp-0x4a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb60)) as u64;
            // 16ba0 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 16ba3 mov QWORD PTR [rbp-0x4a8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb58), (0x0u64) as u64);
            pc = 0x16bae;
        }
        0x16bae => {
            // block 16bae
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16bae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16bae lea rbx,[rbp-0x4b0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // 16bb5 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 16bb7 mov rdi,rbx
            rdi = rbx;
            // 16bba call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16bbf mov rax,QWORD PTR [rbp-0x4f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb10)) as u64;
            // 16bc6 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 16bca mov rax,QWORD PTR [rbp-0x500]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb00)) as u64;
            // 16bd1 mov rcx,QWORD PTR [rbp-0x4f8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb08)) as u64;
            // 16bd8 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 16bdc mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 16bdf xorps xmm0,xmm0
            x0 = 0;
            // 16be2 movaps XMMWORD PTR [rbp-0x500],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fb00), x0);
            // 16be9 mov QWORD PTR [rbp-0x4f0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb10), (0x0u64) as u64);
            // 16bf4 lea rdi,[rbp-0x500]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb00);
            // 16bfb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16c00 mov rdi,QWORD PTR [rbp-0x530]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fad0)) as u64;
            // 16c07 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 16c0a mov rax,QWORD PTR [rax+0x60]
            rax = m.r64(rax.wrapping_add(0x60)) as u64;
            // 16c0e mov rbx,rdi
            rbx = rdi;
            // 16c11 lea rsi,[rbp-0x4c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb38);
            // 16c18 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16c1a mov rdi,rbx
            rdi = rbx;
            // 16c1d mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 16c20 mov rax,QWORD PTR [rax+0x58]
            rax = m.r64(rax.wrapping_add(0x58)) as u64;
            // 16c24 lea rsi,[rbp-0x4c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb38);
            // 16c2b lea rdx,[rbp-0x2c8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 16c32 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16c34 mov rdi,rbx
            rdi = rbx;
            // 16c37 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 16c3a mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 16c3e call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16c40 mov r15,rax
            r15 = rax;
            // 16c43 mov r12,QWORD PTR [r15+0x8]
            r12 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 16c47 cmp r12,QWORD PTR [r15+0x10]
            fl = sub::<64>(r12, m.r64(r15.wrapping_add(0x10)) as u64).1;
            // 16c4b je 0x16d8a
            if fl.e() { pc = 0x16d8a; continue; }
            pc = 0x16c51;
        }
        0x16c51 => {
            // block 16c51
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16c51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16c51 mov rax,QWORD PTR [rbp-0x4b8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb48)) as u64;
            // 16c58 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 16c5d mov rax,QWORD PTR [rbp-0x4c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb38)) as u64;
            // 16c64 mov rcx,QWORD PTR [rbp-0x4c0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb40)) as u64;
            // 16c6b mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 16c70 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 16c74 lea r14,[r12+0x18]
            r14 = r12.wrapping_add(0x18);
            // 16c79 mov rdi,r14
            rdi = r14;
            // 16c7c lea rsi,[rbp-0x4b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // 16c83 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16c88 lea rax,[rbp-0x498]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_fb68);
            // 16c8f mov rcx,rax
            rcx = rax;
            // 16c92 mov ax,WORD PTR [rcx+0x2c]
            rax = (rax & !0xffff) | ((m.r16(rcx.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 16c96 mov WORD PTR [r12+0x5c],ax
            m.w16(r12.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 16c9c mov eax,DWORD PTR [rcx+0x28]
            rax = (m.r32(rcx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 16c9f mov DWORD PTR [r12+0x58],eax
            m.w32(r12.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 16ca4 mov rax,QWORD PTR [rcx+0x20]
            rax = m.r64(rcx.wrapping_add(0x20)) as u64;
            // 16ca8 mov QWORD PTR [r12+0x50],rax
            m.w64(r12.wrapping_add(0x50), (rax) as u64);
            // 16cad mov rax,QWORD PTR [rcx+0x18]
            rax = m.r64(rcx.wrapping_add(0x18)) as u64;
            // 16cb1 mov QWORD PTR [r12+0x48],rax
            m.w64(r12.wrapping_add(0x48), (rax) as u64);
            // 16cb6 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 16cba mov QWORD PTR [r12+0x40],rax
            m.w64(r12.wrapping_add(0x40), (rax) as u64);
            // 16cbf mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 16cc2 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 16cc6 mov QWORD PTR [r12+0x38],rcx
            m.w64(r12.wrapping_add(0x38), (rcx) as u64);
            // 16ccb mov QWORD PTR [r12+0x30],rax
            m.w64(r12.wrapping_add(0x30), (rax) as u64);
            // 16cd0 lea rbx,[r12+0x60]
            rbx = r12.wrapping_add(0x60);
            // 16cd5 mov rdi,rbx
            rdi = rbx;
            // 16cd8 lea rsi,[rbp-0x468]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb98);
            // 16cdf call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16ce4 mov QWORD PTR [rbp-0x540],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fac0), (rbx) as u64);
            // 16ceb mov QWORD PTR [rbp-0x538],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fac8), (r14) as u64);
            // 16cf2 lea rbx,[r12+0x78]
            rbx = r12.wrapping_add(0x78);
            // 16cf7 mov rdi,rbx
            rdi = rbx;
            // 16cfa lea rsi,[rbp-0x450]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fbb0);
            // 16d01 call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16d06 lea r14,[r12+0x90]
            r14 = r12.wrapping_add(0x90);
            // 16d0e mov rdi,r14
            rdi = r14;
            // 16d11 lea rsi,[rbp-0x438]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fbc8);
            // 16d18 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16d1d lea r13,[r12+0xa8]
            r13 = r12.wrapping_add(0xa8);
            // 16d25 mov rdi,r13
            rdi = r13;
            // 16d28 lea rsi,[rbp-0x420]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fbe0);
            // 16d2f call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16d34 lea rdi,[r12+0xc0]
            rdi = r12.wrapping_add(0xc0);
            // 16d3c lea rsi,[rbp-0x408]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fbf8);
            // 16d43 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16d48 lea rax,[rbp-0x3f0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_fc10);
            // 16d4f mov rcx,rax
            rcx = rax;
            // 16d52 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 16d55 mov DWORD PTR [r12+0xf0],eax
            m.w32(r12.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 16d5d mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 16d61 mov QWORD PTR [r12+0xe8],rax
            m.w64(r12.wrapping_add(0xe8), (rax) as u64);
            // 16d69 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 16d6c mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 16d70 mov QWORD PTR [r12+0xe0],rcx
            m.w64(r12.wrapping_add(0xe0), (rcx) as u64);
            // 16d78 mov QWORD PTR [r12+0xd8],rax
            m.w64(r12.wrapping_add(0xd8), (rax) as u64);
            // 16d80 add QWORD PTR [r15+0x8],0xf8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, 0xf8u64);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 16d88 jmp 0x16d99
            pc = 0x16d99; continue;
        }
        0x16d8a => {
            // block 16d8a
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16d8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16d8a mov rdi,r15
            rdi = r15;
            // 16d8d lea rsi,[rbp-0x4c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb38);
            // 16d94 call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16d99;
        }
        0x16d99 => {
            // block 16d99
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16d99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16d99 lea rdi,[rbp-0x408]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbf8);
            // 16da0 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16da5 lea rdi,[rbp-0x420]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbe0);
            // 16dac call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16db1 lea rdi,[rbp-0x438]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbc8);
            // 16db8 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16dbd lea rdi,[rbp-0x450]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb0);
            // 16dc4 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16dc9 lea rdi,[rbp-0x468]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb98);
            // 16dd0 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16dd5 lea rdi,[rbp-0x4b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // 16ddc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16de1 mov rax,QWORD PTR [rbp-0x518]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fae8)) as u64;
            // 16de8 mov QWORD PTR [rbp-0x3d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc30), (rax) as u64);
            // 16def mov rax,QWORD PTR [rbp-0x520]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fae0)) as u64;
            // 16df6 mov QWORD PTR [rbp-0x360],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fca0), (rax) as u64);
            // 16dfd mov rax,QWORD PTR [rbp-0x510]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_faf0)) as u64;
            // 16e04 mov QWORD PTR [rbp-0x3c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc38), (rax) as u64);
            // 16e0b lea r12,[rbp-0x388]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fc78);
            // 16e12 mov rdi,r12
            rdi = r12;
            // 16e15 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16e1a lea rbx,[rbp-0x3c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc38);
            // 16e21 mov rdi,rbx
            rdi = rbx;
            // 16e24 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16e29 lea rdi,[rbp-0x3d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc30);
            // 16e30 mov rsi,QWORD PTR [rbp-0x528]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fad8)) as u64;
            // 16e37 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16e3c lea rdi,[rbp-0x360]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fca0);
            // 16e43 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16e48 lea r15,[rbp-0x2c8]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            pc = 0x16e4f;
        }
        0x16e4f => {
            // block 16e4f
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16e4f mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 16e56 mov rax,QWORD PTR [rax+0x28]
            rax = m.r64(rax.wrapping_add(0x28)) as u64;
            // 16e5a mov rdi,r15
            rdi = r15;
            // 16e5d call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16e5f jmp 0x168ae
            pc = 0x168ae; continue;
        }
        0x16e64 => {
            // block 16e64
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16e64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16e64 mov r12,rax
            r12 = rax;
            // 16e67 jmp 0x17006
            pc = 0x17006; continue;
        }
        0x16e6c => {
            // block 16e6c
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16e6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16e6c lea rdi,[rbp-0x2c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 16e73 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16e78 add rsp,0x548
            { let (r, f) = add::<64>(rsp, 0x548u64);
              rsp = r;
              fl = f; }
            // 16e7f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 16e80 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 16e82 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 16e84 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 16e86 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 16e88 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 16e89 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x16e8a => {
            // block 16e8a
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16e8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16e8a mov r12,rax
            r12 = rax;
            // 16e8d jmp 0x16f5a
            pc = 0x16f5a; continue;
        }
        0x16e92 => {
            // block 16e92
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16e92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16e92 mov r12,rax
            r12 = rax;
            // 16e95 lea rdi,[rbp-0x4e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb20);
            // 16e9c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16ea1 jmp 0x16f5a
            pc = 0x16f5a; continue;
        }
        0x16ea6 => {
            // block 16ea6
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16ea6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16ea6 lea r14,[rbp-0x360]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fca0);
            // 16ead mov r12,rax
            r12 = rax;
            // 16eb0 jmp 0x16efc
            pc = 0x16efc; continue;
        }
        0x16eb2 => {
            // block 16eb2
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16eb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16eb2 lea r14,[rbp-0x360]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fca0);
            // 16eb9 mov r12,rax
            r12 = rax;
            // 16ebc jmp 0x16ee9
            pc = 0x16ee9; continue;
        }
        0x16ebe => {
            // block 16ebe
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16ebe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16ebe lea r14,[rbp-0x360]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fca0);
            // 16ec5 mov r12,rax
            r12 = rax;
            // 16ec8 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 16ecc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16ed1 lea rdi,[rbp-0x388]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc78);
            // 16ed8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16edd lea rdi,[rbp-0x3c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc38);
            // 16ee4 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16ee9;
        }
        0x16ee9 => {
            // block 16ee9
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16ee9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16ee9 lea rdi,[rbp-0x3d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc30);
            // 16ef0 mov rsi,QWORD PTR [rbp-0x528]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fad8)) as u64;
            // 16ef7 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16efc;
        }
        0x16efc => {
            // block 16efc
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16efc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16efc mov rdi,r14
            rdi = r14;
            // 16eff jmp 0x17001
            pc = 0x17001; continue;
        }
        0x16f04 => {
            // block 16f04
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16f04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16f04 mov rdi,rax
            rdi = rax;
            // 16f07 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16f0c mov QWORD PTR [rbp-0x538],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fac8), (r14) as u64);
            // 16f13 mov r12,rax
            r12 = rax;
            // 16f16 jmp 0x16f4e
            pc = 0x16f4e; continue;
        }
        0x16f18 => {
            // block 16f18
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16f18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16f18 mov r12,rax
            r12 = rax;
            // 16f1b jmp 0x16f42
            pc = 0x16f42; continue;
        }
        0x16f1d => {
            // block 16f1d
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16f1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16f1d mov r12,rax
            r12 = rax;
            // 16f20 jmp 0x16f3a
            pc = 0x16f3a; continue;
        }
        0x16f22 => {
            // block 16f22
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16f22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16f22 mov r12,rax
            r12 = rax;
            // 16f25 jmp 0x16f32
            pc = 0x16f32; continue;
        }
        0x16f27 => {
            // block 16f27
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16f27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16f27 mov r12,rax
            r12 = rax;
            // 16f2a mov rdi,r13
            rdi = r13;
            // 16f2d call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16f32;
        }
        0x16f32 => {
            // block 16f32
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16f32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16f32 mov rdi,r14
            rdi = r14;
            // 16f35 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16f3a;
        }
        0x16f3a => {
            // block 16f3a
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16f3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16f3a mov rdi,rbx
            rdi = rbx;
            // 16f3d call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16f42;
        }
        0x16f42 => {
            // block 16f42
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16f42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16f42 mov rdi,QWORD PTR [rbp-0x540]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fac0)) as u64;
            // 16f49 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16f4e;
        }
        0x16f4e => {
            // block 16f4e
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16f4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16f4e mov rdi,QWORD PTR [rbp-0x538]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fac8)) as u64;
            // 16f55 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16f5a;
        }
        0x16f5a => {
            // block 16f5a
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 16f5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16f5a lea rbx,[rbp-0x388]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc78);
            // 16f61 lea rdi,[rbp-0x408]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbf8);
            // 16f68 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16f6d lea rdi,[rbp-0x420]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbe0);
            // 16f74 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16f79 lea rdi,[rbp-0x438]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbc8);
            // 16f80 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16f85 lea rdi,[rbp-0x450]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb0);
            // 16f8c call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16f91 lea rdi,[rbp-0x468]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb98);
            // 16f98 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16f9d lea rdi,[rbp-0x4b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // 16fa4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16fa9 mov rax,QWORD PTR [rbp-0x518]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fae8)) as u64;
            // 16fb0 mov QWORD PTR [rbp-0x3d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc30), (rax) as u64);
            // 16fb7 mov rax,QWORD PTR [rbp-0x520]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fae0)) as u64;
            // 16fbe mov QWORD PTR [rbp-0x360],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fca0), (rax) as u64);
            // 16fc5 mov rax,QWORD PTR [rbp-0x510]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_faf0)) as u64;
            // 16fcc mov QWORD PTR [rbp-0x3c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc38), (rax) as u64);
            // 16fd3 mov rdi,rbx
            rdi = rbx;
            // 16fd6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16fdb lea rdi,[rbp-0x3c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc38);
            // 16fe2 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16fe7 lea rdi,[rbp-0x3d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc30);
            // 16fee mov rsi,QWORD PTR [rbp-0x528]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fad8)) as u64;
            // 16ff5 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16ffa lea rdi,[rbp-0x360]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fca0);
            pc = 0x17001;
        }
        0x17001 => {
            // block 17001
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 17001 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17001 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x17006;
        }
        0x17006 => {
            // block 17006
            #[cfg(feature = "trace")] eprintln!("f_000167fc pc 17006 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17006 lea rdi,[rbp-0x2c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 1700d call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17012 mov rdi,r12
            rdi = r12;
            // 17015 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb0982 size=2100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b0982<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb0982;
    loop {
        match pc {
        0xb0982 => {
            // block b0982
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b0982 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0982 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b0983 mov rbp,rsp
            rbp = rsp;
            // b0986 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b0988 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b098a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b098c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b098e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b098f sub rsp,0x158
            { let (r, f) = sub::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // b0996 mov r14d,r8d
            r14 = ((r8 as u32 as u64)) as u32 as u64;
            // b0999 mov DWORD PTR [rbp-0x17c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe84), ((rcx as u32 as u64)) as u32);
            // b099f mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // b09a2 mov r13,rdi
            r13 = rdi;
            // b09a5 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b09ac lea rbx,[rbp-0x160]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b09b3 mov rax,QWORD PTR [rip+0x8f816] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // b09ba lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // b09be mov QWORD PTR [rbp-0x168],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rcx) as u64);
            // b09c5 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // b09c9 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // b09d0 mov rsi,rbx
            rsi = rbx;
            // b09d3 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b09d8 mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // b09e0 mov DWORD PTR [rbp-0x68],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0xffffffffu64) as u32);
            // b09e7 mov r12,QWORD PTR [rip+0x8f9d2] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // b09ee lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // b09f3 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // b09fa mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b0a01 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // b0a05 mov QWORD PTR [rbp-0xf8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r12) as u64);
            // b0a0c mov rdi,rbx
            rdi = rbx;
            // b0a0f call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0a14 mov rdi,rbx
            rdi = rbx;
            // b0a17 mov rax,QWORD PTR [rip+0x8f98a] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // b0a1e add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // b0a22 mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            // b0a29 mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // b0a30 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // b0a3b mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // b0a46 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // b0a51 mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // b0a5c mov DWORD PTR [rbp-0x100],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x10u64) as u32);
            // b0a66 xorps xmm0,xmm0
            x0 = 0;
            // b0a69 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // b0a6d mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // b0a75 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b0a79 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0a7e lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b0a82 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0a87 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // b0a8a call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0a8f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b0a91 je 0xb106f
            if fl.e() { pc = 0xb106f; continue; }
            pc = 0xb0a97;
        }
        0xb0a97 => {
            // block b0a97
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b0a97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0a97 lea eax,[r15-0x14a]
            rax = (r15.wrapping_add(0xffff_ffff_ffff_feb6)) as u32 as u64;
            // b0a9e cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // b0aa1 jae 0xb0b90
            if fl.ae() { pc = 0xb0b90; continue; }
            pc = 0xb0aa7;
        }
        0xb0aa7 => {
            // block b0aa7
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b0aa7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0aa7 mov eax,DWORD PTR [rbp-0x17c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64) as u32 as u64;
            // b0aad movzx ebx,ax
            rbx = ((rax as u16 as u64)) as u32 as u64;
            // b0ab0 cmp ebx,0xc350
            fl = sub::<32>((rbx as u32 as u64), 0xc350u64).1;
            // b0ab6 ja 0xb0cde
            if fl.a() { pc = 0xb0cde; continue; }
            pc = 0xb0abc;
        }
        0xb0abc => {
            // block b0abc
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b0abc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0abc lea rsi,[rip+0x8440d] # 0x134ed0
            rsi = m.env.data_va(300701);
            // b0ac3 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0aca mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // b0acf call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0ad4 cvtsi2sd xmm0,ebx
            x0 = set_lo64(x0, (sx::<32>((rbx as u32 as u64)) as f64).to_bits());
            // b0ad8 mulsd xmm0,QWORD PTR [rip+0x4fcc0] # 0x1007a0
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(85869)) as u128))).to_bits());
            // b0ae0 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0ae3 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0ae7 mov edx,0xfffffefb
            rdx = (0xfffffefbu64) as u32 as u64;
            // b0aec and edx,DWORD PTR [rcx+rax*1+0x8]
            { let r = (rdx as u32 as u64) & m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0af0 or edx,0x4
            { let r = (rdx as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0af3 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0af7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0afa mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0afe mov QWORD PTR [rcx+rax*1+0x18],0x7
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x7u64) as u64);
            // b0b07 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0b0a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0b0e mov QWORD PTR [rcx+rax*1+0x10],0x5
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x10), (0x5u64) as u64);
            // b0b17 mov rdi,rax
            rdi = rax;
            // b0b1a call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b1f mov rbx,rax
            rbx = rax;
            // b0b22 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0b25 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0b29 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0b2d and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b0b30 or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b0b33 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // b0b37 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0b3a mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0b3e add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b0b41 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0b45 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b4a mov rsi,QWORD PTR [rip+0x8f63f] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0b51 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0b55 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b5a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0b5d mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0b61 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0b66 mov rdi,rax
            rdi = rax;
            // b0b69 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b6b mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0b6e lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0b72 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b77 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b0b7b mov rdi,rbx
            rdi = rbx;
            // b0b7e call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b83 mov rdi,rbx
            rdi = rbx;
            // b0b86 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b8b jmp 0xb0de7
            pc = 0xb0de7; continue;
        }
        0xb0b90 => {
            // block b0b90
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b0b90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0b90 cmp r15d,0x14e
            fl = sub::<32>((r15 as u32 as u64), 0x14eu64).1;
            // b0b97 jne 0xb0f19
            if fl.ne() { pc = 0xb0f19; continue; }
            pc = 0xb0b9d;
        }
        0xb0b9d => {
            // block b0b9d
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b0b9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0b9d lea rsi,[rip+0x8433e] # 0x134ee2
            rsi = m.env.data_va(300719);
            // b0ba4 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0bab mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b0bb0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0bb5 mov ecx,DWORD PTR [rbp-0x17c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64) as u32 as u64;
            // b0bbb shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // b0bbe cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // b0bc2 mulsd xmm0,QWORD PTR [rip+0x511e6] # 0x101db0
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(91517)) as u128))).to_bits());
            // b0bca mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0bcd mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0bd1 mov edx,0xfffffefb
            rdx = (0xfffffefbu64) as u32 as u64;
            // b0bd6 and edx,DWORD PTR [rcx+rax*1+0x8]
            { let r = (rdx as u32 as u64) & m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0bda or edx,0x4
            { let r = (rdx as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0bdd mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0be1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0be4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0be8 mov QWORD PTR [rcx+rax*1+0x18],0x7
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x7u64) as u64);
            // b0bf1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0bf4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0bf8 mov QWORD PTR [rcx+rax*1+0x10],0x5
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x10), (0x5u64) as u64);
            // b0c01 mov rdi,rax
            rdi = rax;
            // b0c04 call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c09 mov rbx,rax
            rbx = rax;
            // b0c0c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0c0f mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0c13 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0c17 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b0c1a or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b0c1d mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // b0c21 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0c24 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0c28 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b0c2b lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b0c2f call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c34 mov rsi,QWORD PTR [rip+0x8f555] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0c3b lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b0c3f call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c44 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0c47 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0c4b mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0c50 mov rdi,rax
            rdi = rax;
            // b0c53 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c55 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0c58 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b0c5c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c61 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b0c65 mov rdi,rbx
            rdi = rbx;
            // b0c68 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c6d mov rdi,rbx
            rdi = rbx;
            // b0c70 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c75 lea rsi,[rip+0x8426c] # 0x134ee8
            rsi = m.env.data_va(300725);
            // b0c7c lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0c83 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b0c88 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c8d mov ecx,DWORD PTR [rbp-0x17c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64) as u32 as u64;
            // b0c93 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // b0c96 cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // b0c9a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0c9d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0ca1 mov edx,0xfffffefb
            rdx = (0xfffffefbu64) as u32 as u64;
            // b0ca6 and edx,DWORD PTR [rcx+rax*1+0x8]
            { let r = (rdx as u32 as u64) & m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0caa or edx,0x4
            { let r = (rdx as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0cad mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0cb1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0cb4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0cb8 mov QWORD PTR [rcx+rax*1+0x18],0x7
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x7u64) as u64);
            // b0cc1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0cc4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0cc8 mov QWORD PTR [rcx+rax*1+0x10],0x5
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x10), (0x5u64) as u64);
            // b0cd1 mov rdi,rax
            rdi = rax;
            // b0cd4 call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0cd9 jmp 0xb105a
            pc = 0xb105a; continue;
        }
        0xb0cde => {
            // block b0cde
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b0cde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0cde lea rsi,[rip+0x841eb] # 0x134ed0
            rsi = m.env.data_va(300701);
            // b0ce5 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0cec mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // b0cf1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0cf6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0cf9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0cfd mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0d01 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0d04 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0d07 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0d0b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0d0e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d12 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b0d1a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0d1d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d21 mov QWORD PTR [rcx+rax*1+0x18],0x4
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x4u64) as u64);
            // b0d2a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0d2d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d31 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b0d3c mov rdi,rax
            rdi = rax;
            // b0d3f mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b0d41 call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0d46 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0d49 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d4d mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0d51 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0d54 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0d57 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0d5b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0d5e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d62 mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // b0d6d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0d70 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d74 and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b0d7c lea rsi,[rip+0x84151] # 0x134ed4
            rsi = m.env.data_va(300705);
            // b0d83 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // b0d88 mov rdi,rax
            rdi = rax;
            // b0d8b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0d90 mov rbx,rax
            rbx = rax;
            // b0d93 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0d96 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d9a add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b0d9d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b0da1 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0da6 mov rsi,QWORD PTR [rip+0x8f3e3] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0dad lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b0db1 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0db6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0db9 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0dbd mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0dc2 mov rdi,rax
            rdi = rax;
            // b0dc5 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0dc7 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0dca lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b0dce call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0dd3 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b0dd7 mov rdi,rbx
            rdi = rbx;
            // b0dda call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0ddf mov rdi,rbx
            rdi = rbx;
            // b0de2 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb0de7;
        }
        0xb0de7 => {
            // block b0de7
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b0de7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0de7 mov eax,DWORD PTR [rbp-0x17c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64) as u32 as u64;
            // b0ded mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // b0def shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // b0df2 cmp eax,0xc350ffff
            fl = sub::<32>((rax as u32 as u64), 0xc350ffffu64).1;
            // b0df7 ja 0xb0e61
            if fl.a() { pc = 0xb0e61; continue; }
            pc = 0xb0df9;
        }
        0xb0df9 => {
            // block b0df9
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b0df9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0df9 lea rsi,[rip+0x840de] # 0x134ede
            rsi = m.env.data_va(300715);
            // b0e00 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0e07 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // b0e0c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0e11 cvtsi2sd xmm0,ebx
            x0 = set_lo64(x0, (sx::<32>((rbx as u32 as u64)) as f64).to_bits());
            // b0e15 mulsd xmm0,QWORD PTR [rip+0x4f983] # 0x1007a0
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(85869)) as u128))).to_bits());
            // b0e1d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0e20 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0e24 mov edx,0xfffffefb
            rdx = (0xfffffefbu64) as u32 as u64;
            // b0e29 and edx,DWORD PTR [rcx+rax*1+0x8]
            { let r = (rdx as u32 as u64) & m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0e2d or edx,0x4
            { let r = (rdx as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0e30 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0e34 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0e37 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0e3b mov QWORD PTR [rcx+rax*1+0x18],0x7
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x7u64) as u64);
            // b0e44 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0e47 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0e4b mov QWORD PTR [rcx+rax*1+0x10],0x5
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x10), (0x5u64) as u64);
            // b0e54 mov rdi,rax
            rdi = rax;
            // b0e57 call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0e5c jmp 0xb105a
            pc = 0xb105a; continue;
        }
        0xb0e61 => {
            // block b0e61
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b0e61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0e61 lea rsi,[rip+0x84076] # 0x134ede
            rsi = m.env.data_va(300715);
            // b0e68 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0e6f mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // b0e74 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0e79 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0e7c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0e80 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0e84 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0e87 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0e8a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0e8e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0e91 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0e95 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b0e9d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0ea0 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0ea4 mov QWORD PTR [rcx+rax*1+0x18],0x4
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x4u64) as u64);
            // b0ead mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0eb0 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0eb4 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b0ebf movzx esi,bx
            rsi = ((rbx as u16 as u64)) as u32 as u64;
            // b0ec2 mov rdi,rax
            rdi = rax;
            // b0ec5 call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0eca mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0ecd mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0ed1 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0ed5 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0ed8 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0edb mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0edf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0ee2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0ee6 mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // b0ef1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0ef4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0ef8 and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b0f00 lea rsi,[rip+0x83fcd] # 0x134ed4
            rsi = m.env.data_va(300705);
            // b0f07 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // b0f0c mov rdi,rax
            rdi = rax;
            // b0f0f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0f14 jmp 0xb106f
            pc = 0xb106f; continue;
        }
        0xb0f19 => {
            // block b0f19
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b0f19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0f19 cmp r15d,0x14f
            fl = sub::<32>((r15 as u32 as u64), 0x14fu64).1;
            // b0f20 jne 0xb106f
            if fl.ne() { pc = 0xb106f; continue; }
            pc = 0xb0f26;
        }
        0xb0f26 => {
            // block b0f26
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b0f26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0f26 lea rsi,[rip+0x83fc1] # 0x134eee
            rsi = m.env.data_va(300731);
            // b0f2d lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0f34 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // b0f39 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0f3e mov ecx,DWORD PTR [rbp-0x17c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64) as u32 as u64;
            // b0f44 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // b0f47 cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // b0f4b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0f4e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0f52 mov edx,0xfffffefb
            rdx = (0xfffffefbu64) as u32 as u64;
            // b0f57 and edx,DWORD PTR [rcx+rax*1+0x8]
            { let r = (rdx as u32 as u64) & m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0f5b or edx,0x4
            { let r = (rdx as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0f5e mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0f62 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0f65 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0f69 mov QWORD PTR [rcx+rax*1+0x18],0x7
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x7u64) as u64);
            // b0f72 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0f75 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0f79 mov QWORD PTR [rcx+rax*1+0x10],0x5
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x10), (0x5u64) as u64);
            // b0f82 mov rdi,rax
            rdi = rax;
            // b0f85 call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0f8a mov rbx,rax
            rbx = rax;
            // b0f8d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0f90 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0f94 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0f98 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b0f9b or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b0f9e mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // b0fa2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0fa5 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0fa9 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b0fac lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b0fb0 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0fb5 mov rsi,QWORD PTR [rip+0x8f1d4] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0fbc lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b0fc0 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0fc5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0fc8 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0fcc mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0fd1 mov rdi,rax
            rdi = rax;
            // b0fd4 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0fd6 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0fd9 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b0fdd call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0fe2 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b0fe6 mov rdi,rbx
            rdi = rbx;
            // b0fe9 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0fee mov rdi,rbx
            rdi = rbx;
            // b0ff1 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0ff6 lea rsi,[rip+0x83f0b] # 0x134f08
            rsi = m.env.data_va(300757);
            // b0ffd lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b1004 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // b1009 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b100e mov ecx,DWORD PTR [rbp-0x17c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64) as u32 as u64;
            // b1014 shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // b1017 cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // b101b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b101e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1022 mov edx,0xfffffefb
            rdx = (0xfffffefbu64) as u32 as u64;
            // b1027 and edx,DWORD PTR [rcx+rax*1+0x8]
            { let r = (rdx as u32 as u64) & m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b102b or edx,0x4
            { let r = (rdx as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b102e mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b1032 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b1035 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1039 mov QWORD PTR [rcx+rax*1+0x18],0x7
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x7u64) as u64);
            // b1042 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b1045 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1049 mov QWORD PTR [rcx+rax*1+0x10],0x5
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x10), (0x5u64) as u64);
            // b1052 mov rdi,rax
            rdi = rax;
            // b1055 call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb105a;
        }
        0xb105a => {
            // block b105a
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b105a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b105a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b105d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1061 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b1065 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b1068 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b106b mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            pc = 0xb106f;
        }
        0xb106f => {
            // block b106f
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b106f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b106f mov rdi,r13
            rdi = r13;
            // b1072 lea rsi,[rbp-0x160]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b1079 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b107e mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // b1085 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b108c mov QWORD PTR [rbp-0xf8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r12) as u64);
            // b1093 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // b109a mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // b10a1 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // b10a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b10ad lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b10b4 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b10b9 mov rsi,QWORD PTR [rip+0x8f148] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b10c0 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b10c4 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b10cb call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b10d0 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b10d7 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b10dc mov rax,r13
            rax = r13;
            // b10df add rsp,0x158
            { let (r, f) = add::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // b10e6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b10e7 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b10e9 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b10eb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b10ed pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b10ef pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b10f0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb10f1 => {
            // block b10f1
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b10f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b10f1 mov rbx,rax
            rbx = rax;
            pc = 0xb10f4;
        }
        0xb10f4 => {
            // block b10f4
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b10f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b10f4 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // b10fb mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b1102 mov QWORD PTR [rbp-0xf8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r12) as u64);
            // b1109 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // b1110 mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            pc = 0xb1117;
        }
        0xb1117 => {
            // block b1117
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b1117 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b1117 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // b111e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1123 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b112a call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb112f;
        }
        0xb112f => {
            // block b112f
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b112f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b112f mov rsi,QWORD PTR [rip+0x8f0d2] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b1136 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b113a lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b1141 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb1146;
        }
        0xb1146 => {
            // block b1146
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b1146 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b1146 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b114d call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1152 mov rdi,rbx
            rdi = rbx;
            // b1155 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b115a mov rbx,rax
            rbx = rax;
            // b115d jmp 0xb1146
            pc = 0xb1146; continue;
        }
        0xb115f => {
            // block b115f
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b115f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b115f mov rbx,rax
            rbx = rax;
            // b1162 jmp 0xb112f
            pc = 0xb112f; continue;
        }
        0xb1164 => {
            // block b1164
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b1164 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b1164 mov rbx,rax
            rbx = rax;
            // b1167 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b116b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1170 jmp 0xb1117
            pc = 0xb1117; continue;
        }
        0xb1172 => {
            // block b1172
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b1172 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b1172 mov rbx,rax
            rbx = rax;
            // b1175 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b1179 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b117e jmp 0xb10f4
            pc = 0xb10f4; continue;
        }
        0xb1183 => {
            // block b1183
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b1183 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b1183 mov rbx,rax
            rbx = rax;
            // b1186 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b118a call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b118f jmp 0xb10f4
            pc = 0xb10f4; continue;
        }
        0xb1194 => {
            // block b1194
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b1194 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b1194 mov rbx,rax
            rbx = rax;
            // b1197 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b119b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b11a0 jmp 0xb10f4
            pc = 0xb10f4; continue;
        }
        0xb11a5 => {
            // block b11a5
            #[cfg(feature = "trace")] eprintln!("f_000b0982 pc b11a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b11a5 mov rbx,rax
            rbx = rax;
            // b11a8 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b11ac call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b11b1 jmp 0xb10f4
            pc = 0xb10f4; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
