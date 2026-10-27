//! Mechanically lifted functions (scripts/lift_run.py), module g002. Each one is checked against the original
//! in the world harness (tests/world_g002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x9aad2 size=29332
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0009aad2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x9aad2;
    loop {
        match pc {
        0x9aad2 => {
            // block 9aad2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9aad2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aad2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 9aad3 mov rbp,rsp
            rbp = rsp;
            // 9aad6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 9aad8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 9aada push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 9aadc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 9aade push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 9aadf sub rsp,0x2568
            { let (r, f) = sub::<64>(rsp, 0x2568u64);
              rsp = r;
              fl = f; }
            // 9aae6 mov r15,rdi
            r15 = rdi;
            // 9aae9 lea rsi,[rip+0x9adca] # 0x1358ba
            rsi = m.env.data_va(303239);
            // 9aaf0 lea rbx,[rbp-0x58]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 9aaf4 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9aaf9 mov rdi,rbx
            rdi = rbx;
            // 9aafc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ab01 mov esi,0x2710
            rsi = (0x2710u64) as u32 as u64;
            // 9ab06 mov rdi,r15
            rdi = r15;
            // 9ab09 mov rdx,rbx
            rdx = rbx;
            // 9ab0c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ab11 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 9ab15 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ab1a lea rsi,[rip+0x9adb1] # 0x1358d2
            rsi = m.env.data_va(303263);
            // 9ab21 lea rbx,[rbp-0x70]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 9ab25 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9ab2a mov rdi,rbx
            rdi = rbx;
            // 9ab2d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ab32 mov esi,0x2710
            rsi = (0x2710u64) as u32 as u64;
            // 9ab37 mov rdi,r15
            rdi = r15;
            // 9ab3a mov rdx,rbx
            rdx = rbx;
            // 9ab3d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ab42 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 9ab46 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ab4b lea rsi,[rip+0x9ada0] # 0x1358f2
            rsi = m.env.data_va(303295);
            // 9ab52 lea rbx,[rbp-0x88]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 9ab59 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9ab5e mov rdi,rbx
            rdi = rbx;
            // 9ab61 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ab66 mov esi,0x2711
            rsi = (0x2711u64) as u32 as u64;
            // 9ab6b mov rdi,r15
            rdi = r15;
            // 9ab6e mov rdx,rbx
            rdx = rbx;
            // 9ab71 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ab76 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 9ab7d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ab82 lea rsi,[rip+0x9ad82] # 0x13590b
            rsi = m.env.data_va(303320);
            // 9ab89 lea rbx,[rbp-0xa0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 9ab90 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9ab95 mov rdi,rbx
            rdi = rbx;
            // 9ab98 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ab9d mov esi,0x2712
            rsi = (0x2712u64) as u32 as u64;
            // 9aba2 mov rdi,r15
            rdi = r15;
            // 9aba5 mov rdx,rbx
            rdx = rbx;
            // 9aba8 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9abad lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 9abb4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9abb9 lea rsi,[rip+0x9ad5e] # 0x13591e
            rsi = m.env.data_va(303339);
            // 9abc0 lea rbx,[rbp-0xb8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 9abc7 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9abcc mov rdi,rbx
            rdi = rbx;
            // 9abcf call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9abd4 mov esi,0x2713
            rsi = (0x2713u64) as u32 as u64;
            // 9abd9 mov rdi,r15
            rdi = r15;
            // 9abdc mov rdx,rbx
            rdx = rbx;
            // 9abdf call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9abe4 lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 9abeb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9abf0 lea rsi,[rip+0x9ad44] # 0x13593b
            rsi = m.env.data_va(303368);
            // 9abf7 lea rbx,[rbp-0xd0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // 9abfe mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9ac03 mov rdi,rbx
            rdi = rbx;
            // 9ac06 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ac0b mov esi,0x2714
            rsi = (0x2714u64) as u32 as u64;
            // 9ac10 mov rdi,r15
            rdi = r15;
            // 9ac13 mov rdx,rbx
            rdx = rbx;
            // 9ac16 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ac1b lea rdi,[rbp-0xd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // 9ac22 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ac27 lea rsi,[rip+0x9ad22] # 0x135950
            rsi = m.env.data_va(303389);
            // 9ac2e lea rbx,[rbp-0xe8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 9ac35 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9ac3a mov rdi,rbx
            rdi = rbx;
            // 9ac3d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ac42 mov esi,0x2715
            rsi = (0x2715u64) as u32 as u64;
            // 9ac47 mov rdi,r15
            rdi = r15;
            // 9ac4a mov rdx,rbx
            rdx = rbx;
            // 9ac4d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ac52 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 9ac59 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ac5e lea rsi,[rip+0x9acfd] # 0x135962
            rsi = m.env.data_va(303407);
            // 9ac65 lea rbx,[rbp-0x100]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 9ac6c mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 9ac71 mov rdi,rbx
            rdi = rbx;
            // 9ac74 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ac79 mov esi,0x2724
            rsi = (0x2724u64) as u32 as u64;
            // 9ac7e mov rdi,r15
            rdi = r15;
            // 9ac81 mov rdx,rbx
            rdx = rbx;
            // 9ac84 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ac89 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 9ac90 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ac95 lea rsi,[rip+0x9acd7] # 0x135973
            rsi = m.env.data_va(303424);
            // 9ac9c lea rbx,[rbp-0x118]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // 9aca3 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9aca8 mov rdi,rbx
            rdi = rbx;
            // 9acab call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9acb0 mov esi,0x2725
            rsi = (0x2725u64) as u32 as u64;
            // 9acb5 mov rdi,r15
            rdi = r15;
            // 9acb8 mov rdx,rbx
            rdx = rbx;
            // 9acbb call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9acc0 lea rdi,[rbp-0x118]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // 9acc7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9accc lea rsi,[rip+0x9acbb] # 0x13598e
            rsi = m.env.data_va(303451);
            // 9acd3 lea rbx,[rbp-0x130]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 9acda mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9acdf mov rdi,rbx
            rdi = rbx;
            // 9ace2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ace7 mov esi,0x2726
            rsi = (0x2726u64) as u32 as u64;
            // 9acec mov rdi,r15
            rdi = r15;
            // 9acef mov rdx,rbx
            rdx = rbx;
            // 9acf2 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9acf7 lea rdi,[rbp-0x130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 9acfe call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ad03 lea rsi,[rip+0x9ac9e] # 0x1359a8
            rsi = m.env.data_va(303477);
            // 9ad0a lea rbx,[rbp-0x148]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 9ad11 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9ad16 mov rdi,rbx
            rdi = rbx;
            // 9ad19 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ad1e mov esi,0x2727
            rsi = (0x2727u64) as u32 as u64;
            // 9ad23 mov rdi,r15
            rdi = r15;
            // 9ad26 mov rdx,rbx
            rdx = rbx;
            // 9ad29 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ad2e lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 9ad35 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ad3a lea rsi,[rip+0x9ac81] # 0x1359c2
            rsi = m.env.data_va(303503);
            // 9ad41 lea rbx,[rbp-0x160]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 9ad48 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9ad4d mov rdi,rbx
            rdi = rbx;
            // 9ad50 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ad55 mov esi,0x2728
            rsi = (0x2728u64) as u32 as u64;
            // 9ad5a mov rdi,r15
            rdi = r15;
            // 9ad5d mov rdx,rbx
            rdx = rbx;
            // 9ad60 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ad65 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 9ad6c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ad71 lea rsi,[rip+0x9ac62] # 0x1359da
            rsi = m.env.data_va(303527);
            // 9ad78 lea rbx,[rbp-0x178]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 9ad7f mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9ad84 mov rdi,rbx
            rdi = rbx;
            // 9ad87 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ad8c mov esi,0x2729
            rsi = (0x2729u64) as u32 as u64;
            // 9ad91 mov rdi,r15
            rdi = r15;
            // 9ad94 mov rdx,rbx
            rdx = rbx;
            // 9ad97 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ad9c lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 9ada3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ada8 lea rsi,[rip+0x9ac41] # 0x1359f0
            rsi = m.env.data_va(303549);
            // 9adaf lea rbx,[rbp-0x190]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 9adb6 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9adbb mov rdi,rbx
            rdi = rbx;
            // 9adbe call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9adc3 mov esi,0x272a
            rsi = (0x272au64) as u32 as u64;
            // 9adc8 mov rdi,r15
            rdi = r15;
            // 9adcb mov rdx,rbx
            rdx = rbx;
            // 9adce call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9add3 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 9adda call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9addf lea rsi,[rip+0x9ac23] # 0x135a09
            rsi = m.env.data_va(303574);
            // 9ade6 lea rbx,[rbp-0x1a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 9aded mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9adf2 mov rdi,rbx
            rdi = rbx;
            // 9adf5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9adfa mov esi,0x2738
            rsi = (0x2738u64) as u32 as u64;
            // 9adff mov rdi,r15
            rdi = r15;
            // 9ae02 mov rdx,rbx
            rdx = rbx;
            // 9ae05 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ae0a lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 9ae11 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ae16 lea rsi,[rip+0x9ac03] # 0x135a20
            rsi = m.env.data_va(303597);
            // 9ae1d lea rbx,[rbp-0x1c0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 9ae24 mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9ae29 mov rdi,rbx
            rdi = rbx;
            // 9ae2c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ae31 mov esi,0x2739
            rsi = (0x2739u64) as u32 as u64;
            // 9ae36 mov rdi,r15
            rdi = r15;
            // 9ae39 mov rdx,rbx
            rdx = rbx;
            // 9ae3c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ae41 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 9ae48 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ae4d lea rsi,[rip+0x9abe1] # 0x135a35
            rsi = m.env.data_va(303618);
            // 9ae54 lea rbx,[rbp-0x1d8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe28);
            // 9ae5b mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9ae60 mov rdi,rbx
            rdi = rbx;
            // 9ae63 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ae68 mov esi,0x273a
            rsi = (0x273au64) as u32 as u64;
            // 9ae6d mov rdi,r15
            rdi = r15;
            // 9ae70 mov rdx,rbx
            rdx = rbx;
            // 9ae73 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ae78 lea rdi,[rbp-0x1d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe28);
            // 9ae7f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ae84 lea rsi,[rip+0x9abbe] # 0x135a49
            rsi = m.env.data_va(303638);
            // 9ae8b lea rbx,[rbp-0x1f0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 9ae92 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 9ae97 mov rdi,rbx
            rdi = rbx;
            // 9ae9a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ae9f mov esi,0x273b
            rsi = (0x273bu64) as u32 as u64;
            // 9aea4 mov rdi,r15
            rdi = r15;
            // 9aea7 mov rdx,rbx
            rdx = rbx;
            // 9aeaa call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9aeaf lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 9aeb6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9aebb lea rsi,[rip+0x9ab96] # 0x135a58
            rsi = m.env.data_va(303653);
            // 9aec2 lea rbx,[rbp-0x208]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fdf8);
            // 9aec9 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 9aece mov rdi,rbx
            rdi = rbx;
            // 9aed1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9aed6 mov esi,0x273c
            rsi = (0x273cu64) as u32 as u64;
            // 9aedb mov rdi,r15
            rdi = r15;
            // 9aede mov rdx,rbx
            rdx = rbx;
            // 9aee1 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9aee6 lea rdi,[rbp-0x208]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf8);
            // 9aeed call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9aef2 lea rsi,[rip+0x9ab6c] # 0x135a65
            rsi = m.env.data_va(303666);
            // 9aef9 lea rbx,[rbp-0x220]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fde0);
            // 9af00 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9af05 mov rdi,rbx
            rdi = rbx;
            // 9af08 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9af0d mov esi,0x273d
            rsi = (0x273du64) as u32 as u64;
            // 9af12 mov rdi,r15
            rdi = r15;
            // 9af15 mov rdx,rbx
            rdx = rbx;
            // 9af18 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9af1d lea rdi,[rbp-0x220]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde0);
            // 9af24 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9af29 lea rsi,[rip+0x9ab45] # 0x135a75
            rsi = m.env.data_va(303682);
            // 9af30 lea rbx,[rbp-0x238]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 9af37 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 9af3c mov rdi,rbx
            rdi = rbx;
            // 9af3f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9af44 mov esi,0x273e
            rsi = (0x273eu64) as u32 as u64;
            // 9af49 mov rdi,r15
            rdi = r15;
            // 9af4c mov rdx,rbx
            rdx = rbx;
            // 9af4f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9af54 lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 9af5b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9af60 lea rsi,[rip+0x9ab1f] # 0x135a86
            rsi = m.env.data_va(303699);
            // 9af67 lea rbx,[rbp-0x250]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fdb0);
            // 9af6e mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9af73 mov rdi,rbx
            rdi = rbx;
            // 9af76 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9af7b mov esi,0x2742
            rsi = (0x2742u64) as u32 as u64;
            // 9af80 mov rdi,r15
            rdi = r15;
            // 9af83 mov rdx,rbx
            rdx = rbx;
            // 9af86 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9af8b lea rdi,[rbp-0x250]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdb0);
            // 9af92 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9af97 lea rsi,[rip+0x9aafd] # 0x135a9b
            rsi = m.env.data_va(303720);
            // 9af9e lea rbx,[rbp-0x268]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd98);
            // 9afa5 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9afaa mov rdi,rbx
            rdi = rbx;
            // 9afad call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9afb2 mov esi,0x2743
            rsi = (0x2743u64) as u32 as u64;
            // 9afb7 mov rdi,r15
            rdi = r15;
            // 9afba mov rdx,rbx
            rdx = rbx;
            // 9afbd call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9afc2 lea rdi,[rbp-0x268]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd98);
            // 9afc9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9afce lea rsi,[rip+0x9aae3] # 0x135ab8
            rsi = m.env.data_va(303749);
            // 9afd5 lea rbx,[rbp-0x280]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd80);
            // 9afdc mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // 9afe1 mov rdi,rbx
            rdi = rbx;
            // 9afe4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9afe9 mov esi,0x2744
            rsi = (0x2744u64) as u32 as u64;
            // 9afee mov rdi,r15
            rdi = r15;
            // 9aff1 mov rdx,rbx
            rdx = rbx;
            // 9aff4 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9aff9 lea rdi,[rbp-0x280]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd80);
            // 9b000 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b005 lea rsi,[rip+0x9aacd] # 0x135ad9
            rsi = m.env.data_va(303782);
            // 9b00c lea rbx,[rbp-0x298]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd68);
            // 9b013 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9b018 mov rdi,rbx
            rdi = rbx;
            // 9b01b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b020 mov esi,0x2745
            rsi = (0x2745u64) as u32 as u64;
            // 9b025 mov rdi,r15
            rdi = r15;
            // 9b028 mov rdx,rbx
            rdx = rbx;
            // 9b02b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b030 lea rdi,[rbp-0x298]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd68);
            // 9b037 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b03c lea rsi,[rip+0x9aab1] # 0x135af4
            rsi = m.env.data_va(303809);
            // 9b043 lea rbx,[rbp-0x2b0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // 9b04a mov edx,0x24
            rdx = (0x24u64) as u32 as u64;
            // 9b04f mov rdi,rbx
            rdi = rbx;
            // 9b052 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b057 mov esi,0x2746
            rsi = (0x2746u64) as u32 as u64;
            // 9b05c mov rdi,r15
            rdi = r15;
            // 9b05f mov rdx,rbx
            rdx = rbx;
            // 9b062 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b067 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // 9b06e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b073 lea rsi,[rip+0x9aa9f] # 0x135b19
            rsi = m.env.data_va(303846);
            // 9b07a lea rbx,[rbp-0x2c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 9b081 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9b086 mov rdi,rbx
            rdi = rbx;
            // 9b089 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b08e mov esi,0x2747
            rsi = (0x2747u64) as u32 as u64;
            // 9b093 mov rdi,r15
            rdi = r15;
            // 9b096 mov rdx,rbx
            rdx = rbx;
            // 9b099 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b09e lea rdi,[rbp-0x2c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 9b0a5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b0aa lea rsi,[rip+0x9aa83] # 0x135b34
            rsi = m.env.data_va(303873);
            // 9b0b1 lea rbx,[rbp-0x2e0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 9b0b8 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9b0bd mov rdi,rbx
            rdi = rbx;
            // 9b0c0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b0c5 mov esi,0x2748
            rsi = (0x2748u64) as u32 as u64;
            // 9b0ca mov rdi,r15
            rdi = r15;
            // 9b0cd mov rdx,rbx
            rdx = rbx;
            // 9b0d0 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b0d5 lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 9b0dc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b0e1 lea rsi,[rip+0x9aa67] # 0x135b4f
            rsi = m.env.data_va(303900);
            // 9b0e8 lea rbx,[rbp-0x2f8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd08);
            // 9b0ef mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 9b0f4 mov rdi,rbx
            rdi = rbx;
            // 9b0f7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b0fc mov esi,0x274c
            rsi = (0x274cu64) as u32 as u64;
            // 9b101 mov rdi,r15
            rdi = r15;
            // 9b104 mov rdx,rbx
            rdx = rbx;
            // 9b107 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b10c lea rdi,[rbp-0x2f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd08);
            // 9b113 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b118 lea rsi,[rip+0x9aa41] # 0x135b60
            rsi = m.env.data_va(303917);
            // 9b11f lea rbx,[rbp-0x310]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fcf0);
            // 9b126 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 9b12b mov rdi,rbx
            rdi = rbx;
            // 9b12e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b133 mov esi,0x274d
            rsi = (0x274du64) as u32 as u64;
            // 9b138 mov rdi,r15
            rdi = r15;
            // 9b13b mov rdx,rbx
            rdx = rbx;
            // 9b13e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b143 lea rdi,[rbp-0x310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf0);
            // 9b14a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b14f lea rsi,[rip+0x9aa1b] # 0x135b71
            rsi = m.env.data_va(303934);
            // 9b156 lea rbx,[rbp-0x328]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 9b15d mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 9b162 mov rdi,rbx
            rdi = rbx;
            // 9b165 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b16a mov esi,0x274e
            rsi = (0x274eu64) as u32 as u64;
            // 9b16f mov rdi,r15
            rdi = r15;
            // 9b172 mov rdx,rbx
            rdx = rbx;
            // 9b175 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b17a lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 9b181 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b186 lea rsi,[rip+0x9a9f5] # 0x135b82
            rsi = m.env.data_va(303951);
            // 9b18d lea rbx,[rbp-0x340]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 9b194 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9b199 mov rdi,rbx
            rdi = rbx;
            // 9b19c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b1a1 mov esi,0x274f
            rsi = (0x274fu64) as u32 as u64;
            // 9b1a6 mov rdi,r15
            rdi = r15;
            // 9b1a9 mov rdx,rbx
            rdx = rbx;
            // 9b1ac call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b1b1 lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 9b1b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b1bd lea rsi,[rip+0x9a9ce] # 0x135b92
            rsi = m.env.data_va(303967);
            // 9b1c4 lea rbx,[rbp-0x358]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fca8);
            // 9b1cb mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9b1d0 mov rdi,rbx
            rdi = rbx;
            // 9b1d3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b1d8 mov esi,0x2750
            rsi = (0x2750u64) as u32 as u64;
            // 9b1dd mov rdi,r15
            rdi = r15;
            // 9b1e0 mov rdx,rbx
            rdx = rbx;
            // 9b1e3 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b1e8 lea rdi,[rbp-0x358]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fca8);
            // 9b1ef call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b1f4 lea rsi,[rip+0x9a9a9] # 0x135ba4
            rsi = m.env.data_va(303985);
            // 9b1fb lea rbx,[rbp-0x370]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc90);
            // 9b202 mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9b207 mov rdi,rbx
            rdi = rbx;
            // 9b20a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b20f mov esi,0x2751
            rsi = (0x2751u64) as u32 as u64;
            // 9b214 mov rdi,r15
            rdi = r15;
            // 9b217 mov rdx,rbx
            rdx = rbx;
            // 9b21a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b21f lea rdi,[rbp-0x370]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc90);
            // 9b226 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b22b lea rsi,[rip+0x9a987] # 0x135bb9
            rsi = m.env.data_va(304006);
            // 9b232 lea rbx,[rbp-0x388]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc78);
            // 9b239 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9b23e mov rdi,rbx
            rdi = rbx;
            // 9b241 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b246 mov esi,0x2752
            rsi = (0x2752u64) as u32 as u64;
            // 9b24b mov rdi,r15
            rdi = r15;
            // 9b24e mov rdx,rbx
            rdx = rbx;
            // 9b251 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b256 lea rdi,[rbp-0x388]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc78);
            // 9b25d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b262 lea rsi,[rip+0x9a970] # 0x135bd9
            rsi = m.env.data_va(304038);
            // 9b269 lea rbx,[rbp-0x3a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc60);
            // 9b270 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9b275 mov rdi,rbx
            rdi = rbx;
            // 9b278 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b27d mov esi,0x2756
            rsi = (0x2756u64) as u32 as u64;
            // 9b282 mov rdi,r15
            rdi = r15;
            // 9b285 mov rdx,rbx
            rdx = rbx;
            // 9b288 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b28d lea rdi,[rbp-0x3a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc60);
            // 9b294 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b299 lea rsi,[rip+0x9a94c] # 0x135bec
            rsi = m.env.data_va(304057);
            // 9b2a0 lea rbx,[rbp-0x3b8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc48);
            // 9b2a7 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9b2ac mov rdi,rbx
            rdi = rbx;
            // 9b2af call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b2b4 mov esi,0x2757
            rsi = (0x2757u64) as u32 as u64;
            // 9b2b9 mov rdi,r15
            rdi = r15;
            // 9b2bc mov rdx,rbx
            rdx = rbx;
            // 9b2bf call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b2c4 lea rdi,[rbp-0x3b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc48);
            // 9b2cb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b2d0 lea rsi,[rip+0x9a938] # 0x135c0f
            rsi = m.env.data_va(304092);
            // 9b2d7 lea rbx,[rbp-0x3d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc30);
            // 9b2de mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9b2e3 mov rdi,rbx
            rdi = rbx;
            // 9b2e6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b2eb mov esi,0x2758
            rsi = (0x2758u64) as u32 as u64;
            // 9b2f0 mov rdi,r15
            rdi = r15;
            // 9b2f3 mov rdx,rbx
            rdx = rbx;
            // 9b2f6 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b2fb lea rdi,[rbp-0x3d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc30);
            // 9b302 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b307 lea rsi,[rip+0x9a924] # 0x135c32
            rsi = m.env.data_va(304127);
            // 9b30e lea rbx,[rbp-0x3e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // 9b315 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9b31a mov rdi,rbx
            rdi = rbx;
            // 9b31d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b322 mov esi,0x2759
            rsi = (0x2759u64) as u32 as u64;
            // 9b327 mov rdi,r15
            rdi = r15;
            // 9b32a mov rdx,rbx
            rdx = rbx;
            // 9b32d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b332 lea rdi,[rbp-0x3e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // 9b339 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b33e lea rsi,[rip+0x9a910] # 0x135c55
            rsi = m.env.data_va(304162);
            // 9b345 lea rbx,[rbp-0x400]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 9b34c mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9b351 mov rdi,rbx
            rdi = rbx;
            // 9b354 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b359 mov esi,0x275a
            rsi = (0x275au64) as u32 as u64;
            // 9b35e mov rdi,r15
            rdi = r15;
            // 9b361 mov rdx,rbx
            rdx = rbx;
            // 9b364 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b369 lea rdi,[rbp-0x400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 9b370 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b375 lea rsi,[rip+0x9a8fc] # 0x135c78
            rsi = m.env.data_va(304197);
            // 9b37c lea rbx,[rbp-0x418]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fbe8);
            // 9b383 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9b388 mov rdi,rbx
            rdi = rbx;
            // 9b38b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b390 mov esi,0x275b
            rsi = (0x275bu64) as u32 as u64;
            // 9b395 mov rdi,r15
            rdi = r15;
            // 9b398 mov rdx,rbx
            rdx = rbx;
            // 9b39b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b3a0 lea rdi,[rbp-0x418]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbe8);
            // 9b3a7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b3ac lea rsi,[rip+0x9a8e8] # 0x135c9b
            rsi = m.env.data_va(304232);
            // 9b3b3 lea rbx,[rbp-0x430]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fbd0);
            // 9b3ba mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9b3bf mov rdi,rbx
            rdi = rbx;
            // 9b3c2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b3c7 mov esi,0x275c
            rsi = (0x275cu64) as u32 as u64;
            // 9b3cc mov rdi,r15
            rdi = r15;
            // 9b3cf mov rdx,rbx
            rdx = rbx;
            // 9b3d2 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b3d7 lea rdi,[rbp-0x430]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd0);
            // 9b3de call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b3e3 lea rsi,[rip+0x9a8d4] # 0x135cbe
            rsi = m.env.data_va(304267);
            // 9b3ea lea rbx,[rbp-0x448]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fbb8);
            // 9b3f1 mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // 9b3f6 mov rdi,rbx
            rdi = rbx;
            // 9b3f9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b3fe mov esi,0x275d
            rsi = (0x275du64) as u32 as u64;
            // 9b403 mov rdi,r15
            rdi = r15;
            // 9b406 mov rdx,rbx
            rdx = rbx;
            // 9b409 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b40e lea rdi,[rbp-0x448]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb8);
            // 9b415 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b41a lea rsi,[rip+0x9a8be] # 0x135cdf
            rsi = m.env.data_va(304300);
            // 9b421 lea rbx,[rbp-0x460]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fba0);
            // 9b428 mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // 9b42d mov rdi,rbx
            rdi = rbx;
            // 9b430 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b435 mov esi,0x275e
            rsi = (0x275eu64) as u32 as u64;
            // 9b43a mov rdi,r15
            rdi = r15;
            // 9b43d mov rdx,rbx
            rdx = rbx;
            // 9b440 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b445 lea rdi,[rbp-0x460]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fba0);
            // 9b44c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b451 lea rsi,[rip+0x9a8a8] # 0x135d00
            rsi = m.env.data_va(304333);
            // 9b458 lea rbx,[rbp-0x478]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb88);
            // 9b45f mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9b464 mov rdi,rbx
            rdi = rbx;
            // 9b467 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b46c mov esi,0x275f
            rsi = (0x275fu64) as u32 as u64;
            // 9b471 mov rdi,r15
            rdi = r15;
            // 9b474 mov rdx,rbx
            rdx = rbx;
            // 9b477 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b47c lea rdi,[rbp-0x478]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb88);
            // 9b483 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b488 lea rsi,[rip+0x9a889] # 0x135d18
            rsi = m.env.data_va(304357);
            // 9b48f lea rbx,[rbp-0x490]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb70);
            // 9b496 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9b49b mov rdi,rbx
            rdi = rbx;
            // 9b49e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b4a3 mov esi,0x2760
            rsi = (0x2760u64) as u32 as u64;
            // 9b4a8 mov rdi,r15
            rdi = r15;
            // 9b4ab mov rdx,rbx
            rdx = rbx;
            // 9b4ae call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b4b3 lea rdi,[rbp-0x490]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb70);
            // 9b4ba call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b4bf lea rsi,[rip+0x9a864] # 0x135d2a
            rsi = m.env.data_va(304375);
            // 9b4c6 lea rbx,[rbp-0x4a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb58);
            // 9b4cd mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 9b4d2 mov rdi,rbx
            rdi = rbx;
            // 9b4d5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b4da mov esi,0x2761
            rsi = (0x2761u64) as u32 as u64;
            // 9b4df mov rdi,r15
            rdi = r15;
            // 9b4e2 mov rdx,rbx
            rdx = rbx;
            // 9b4e5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b4ea lea rdi,[rbp-0x4a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb58);
            // 9b4f1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b4f6 lea rsi,[rip+0x9a83e] # 0x135d3b
            rsi = m.env.data_va(304392);
            // 9b4fd lea rbx,[rbp-0x4c0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb40);
            // 9b504 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9b509 mov rdi,rbx
            rdi = rbx;
            // 9b50c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b511 mov esi,0x2761
            rsi = (0x2761u64) as u32 as u64;
            // 9b516 mov rdi,r15
            rdi = r15;
            // 9b519 mov rdx,rbx
            rdx = rbx;
            // 9b51c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b521 lea rdi,[rbp-0x4c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb40);
            // 9b528 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b52d lea rsi,[rip+0x9a819] # 0x135d4d
            rsi = m.env.data_va(304410);
            // 9b534 lea rbx,[rbp-0x4d8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb28);
            // 9b53b mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b540 mov rdi,rbx
            rdi = rbx;
            // 9b543 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b548 mov esi,0x2762
            rsi = (0x2762u64) as u32 as u64;
            // 9b54d mov rdi,r15
            rdi = r15;
            // 9b550 mov rdx,rbx
            rdx = rbx;
            // 9b553 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b558 lea rdi,[rbp-0x4d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb28);
            // 9b55f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b564 lea rsi,[rip+0x9a7f8] # 0x135d63
            rsi = m.env.data_va(304432);
            // 9b56b lea rbx,[rbp-0x4f0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb10);
            // 9b572 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b577 mov rdi,rbx
            rdi = rbx;
            // 9b57a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b57f mov esi,0x2763
            rsi = (0x2763u64) as u32 as u64;
            // 9b584 mov rdi,r15
            rdi = r15;
            // 9b587 mov rdx,rbx
            rdx = rbx;
            // 9b58a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b58f lea rdi,[rbp-0x4f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb10);
            // 9b596 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b59b lea rsi,[rip+0x9a7d7] # 0x135d79
            rsi = m.env.data_va(304454);
            // 9b5a2 lea rbx,[rbp-0x508]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_faf8);
            // 9b5a9 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b5ae mov rdi,rbx
            rdi = rbx;
            // 9b5b1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b5b6 mov esi,0x2764
            rsi = (0x2764u64) as u32 as u64;
            // 9b5bb mov rdi,r15
            rdi = r15;
            // 9b5be mov rdx,rbx
            rdx = rbx;
            // 9b5c1 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b5c6 lea rdi,[rbp-0x508]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_faf8);
            // 9b5cd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b5d2 lea rsi,[rip+0x9a7b6] # 0x135d8f
            rsi = m.env.data_va(304476);
            // 9b5d9 lea rbx,[rbp-0x520]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fae0);
            // 9b5e0 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b5e5 mov rdi,rbx
            rdi = rbx;
            // 9b5e8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b5ed mov esi,0x2765
            rsi = (0x2765u64) as u32 as u64;
            // 9b5f2 mov rdi,r15
            rdi = r15;
            // 9b5f5 mov rdx,rbx
            rdx = rbx;
            // 9b5f8 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b5fd lea rdi,[rbp-0x520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fae0);
            // 9b604 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b609 lea rsi,[rip+0x9a795] # 0x135da5
            rsi = m.env.data_va(304498);
            // 9b610 lea rbx,[rbp-0x538]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fac8);
            // 9b617 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b61c mov rdi,rbx
            rdi = rbx;
            // 9b61f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b624 mov esi,0x2766
            rsi = (0x2766u64) as u32 as u64;
            // 9b629 mov rdi,r15
            rdi = r15;
            // 9b62c mov rdx,rbx
            rdx = rbx;
            // 9b62f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b634 lea rdi,[rbp-0x538]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fac8);
            // 9b63b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b640 lea rsi,[rip+0x9a774] # 0x135dbb
            rsi = m.env.data_va(304520);
            // 9b647 lea rbx,[rbp-0x550]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fab0);
            // 9b64e mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b653 mov rdi,rbx
            rdi = rbx;
            // 9b656 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b65b mov esi,0x2767
            rsi = (0x2767u64) as u32 as u64;
            // 9b660 mov rdi,r15
            rdi = r15;
            // 9b663 mov rdx,rbx
            rdx = rbx;
            // 9b666 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b66b lea rdi,[rbp-0x550]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fab0);
            // 9b672 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b677 lea rsi,[rip+0x9a753] # 0x135dd1
            rsi = m.env.data_va(304542);
            // 9b67e lea rbx,[rbp-0x568]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fa98);
            // 9b685 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b68a mov rdi,rbx
            rdi = rbx;
            // 9b68d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b692 mov esi,0x2768
            rsi = (0x2768u64) as u32 as u64;
            // 9b697 mov rdi,r15
            rdi = r15;
            // 9b69a mov rdx,rbx
            rdx = rbx;
            // 9b69d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b6a2 lea rdi,[rbp-0x568]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa98);
            // 9b6a9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b6ae lea rsi,[rip+0x9a732] # 0x135de7
            rsi = m.env.data_va(304564);
            // 9b6b5 lea rbx,[rbp-0x580]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fa80);
            // 9b6bc mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b6c1 mov rdi,rbx
            rdi = rbx;
            // 9b6c4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b6c9 mov esi,0x2769
            rsi = (0x2769u64) as u32 as u64;
            // 9b6ce mov rdi,r15
            rdi = r15;
            // 9b6d1 mov rdx,rbx
            rdx = rbx;
            // 9b6d4 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b6d9 lea rdi,[rbp-0x580]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa80);
            // 9b6e0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b6e5 lea rsi,[rip+0x9a711] # 0x135dfd
            rsi = m.env.data_va(304586);
            // 9b6ec lea rbx,[rbp-0x598]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fa68);
            // 9b6f3 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b6f8 mov rdi,rbx
            rdi = rbx;
            // 9b6fb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b700 mov esi,0x276a
            rsi = (0x276au64) as u32 as u64;
            // 9b705 mov rdi,r15
            rdi = r15;
            // 9b708 mov rdx,rbx
            rdx = rbx;
            // 9b70b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b710 lea rdi,[rbp-0x598]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa68);
            // 9b717 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b71c lea rsi,[rip+0x9a6f0] # 0x135e13
            rsi = m.env.data_va(304608);
            // 9b723 lea rbx,[rbp-0x5b0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fa50);
            // 9b72a mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b72f mov rdi,rbx
            rdi = rbx;
            // 9b732 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b737 mov esi,0x276b
            rsi = (0x276bu64) as u32 as u64;
            // 9b73c mov rdi,r15
            rdi = r15;
            // 9b73f mov rdx,rbx
            rdx = rbx;
            // 9b742 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b747 lea rdi,[rbp-0x5b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa50);
            // 9b74e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b753 lea rsi,[rip+0x9a6cf] # 0x135e29
            rsi = m.env.data_va(304630);
            // 9b75a lea rbx,[rbp-0x5c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fa38);
            // 9b761 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b766 mov rdi,rbx
            rdi = rbx;
            // 9b769 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b76e mov esi,0x276c
            rsi = (0x276cu64) as u32 as u64;
            // 9b773 mov rdi,r15
            rdi = r15;
            // 9b776 mov rdx,rbx
            rdx = rbx;
            // 9b779 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b77e lea rdi,[rbp-0x5c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa38);
            // 9b785 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b78a lea rsi,[rip+0x9a6ae] # 0x135e3f
            rsi = m.env.data_va(304652);
            // 9b791 lea rbx,[rbp-0x5e0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fa20);
            // 9b798 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b79d mov rdi,rbx
            rdi = rbx;
            // 9b7a0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b7a5 mov esi,0x276d
            rsi = (0x276du64) as u32 as u64;
            // 9b7aa mov rdi,r15
            rdi = r15;
            // 9b7ad mov rdx,rbx
            rdx = rbx;
            // 9b7b0 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b7b5 lea rdi,[rbp-0x5e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa20);
            // 9b7bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b7c1 lea rsi,[rip+0x9a68d] # 0x135e55
            rsi = m.env.data_va(304674);
            // 9b7c8 lea rbx,[rbp-0x5f8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fa08);
            // 9b7cf mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b7d4 mov rdi,rbx
            rdi = rbx;
            // 9b7d7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b7dc mov esi,0x276e
            rsi = (0x276eu64) as u32 as u64;
            // 9b7e1 mov rdi,r15
            rdi = r15;
            // 9b7e4 mov rdx,rbx
            rdx = rbx;
            // 9b7e7 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b7ec lea rdi,[rbp-0x5f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa08);
            // 9b7f3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b7f8 lea rsi,[rip+0x9a66c] # 0x135e6b
            rsi = m.env.data_va(304696);
            // 9b7ff lea rbx,[rbp-0x610]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f9f0);
            // 9b806 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b80b mov rdi,rbx
            rdi = rbx;
            // 9b80e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b813 mov esi,0x276f
            rsi = (0x276fu64) as u32 as u64;
            // 9b818 mov rdi,r15
            rdi = r15;
            // 9b81b mov rdx,rbx
            rdx = rbx;
            // 9b81e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b823 lea rdi,[rbp-0x610]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9f0);
            // 9b82a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b82f lea rsi,[rip+0x9a64b] # 0x135e81
            rsi = m.env.data_va(304718);
            // 9b836 lea rbx,[rbp-0x628]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f9d8);
            // 9b83d mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b842 mov rdi,rbx
            rdi = rbx;
            // 9b845 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b84a mov esi,0x2770
            rsi = (0x2770u64) as u32 as u64;
            // 9b84f mov rdi,r15
            rdi = r15;
            // 9b852 mov rdx,rbx
            rdx = rbx;
            // 9b855 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b85a lea rdi,[rbp-0x628]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9d8);
            // 9b861 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b866 lea rsi,[rip+0x9a62a] # 0x135e97
            rsi = m.env.data_va(304740);
            // 9b86d lea rbx,[rbp-0x640]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f9c0);
            // 9b874 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b879 mov rdi,rbx
            rdi = rbx;
            // 9b87c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b881 mov esi,0x2771
            rsi = (0x2771u64) as u32 as u64;
            // 9b886 mov rdi,r15
            rdi = r15;
            // 9b889 mov rdx,rbx
            rdx = rbx;
            // 9b88c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b891 lea rdi,[rbp-0x640]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9c0);
            // 9b898 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b89d lea rsi,[rip+0x9a609] # 0x135ead
            rsi = m.env.data_va(304762);
            // 9b8a4 lea rbx,[rbp-0x658]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f9a8);
            // 9b8ab mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b8b0 mov rdi,rbx
            rdi = rbx;
            // 9b8b3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b8b8 mov esi,0x2772
            rsi = (0x2772u64) as u32 as u64;
            // 9b8bd mov rdi,r15
            rdi = r15;
            // 9b8c0 mov rdx,rbx
            rdx = rbx;
            // 9b8c3 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b8c8 lea rdi,[rbp-0x658]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9a8);
            // 9b8cf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b8d4 lea rsi,[rip+0x9a5e8] # 0x135ec3
            rsi = m.env.data_va(304784);
            // 9b8db lea rbx,[rbp-0x670]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f990);
            // 9b8e2 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 9b8e7 mov rdi,rbx
            rdi = rbx;
            // 9b8ea call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b8ef mov esi,0x2774
            rsi = (0x2774u64) as u32 as u64;
            // 9b8f4 mov rdi,r15
            rdi = r15;
            // 9b8f7 mov rdx,rbx
            rdx = rbx;
            // 9b8fa call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b8ff lea rdi,[rbp-0x670]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f990);
            // 9b906 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b90b lea rsi,[rip+0x9a5bd] # 0x135ecf
            rsi = m.env.data_va(304796);
            // 9b912 lea rbx,[rbp-0x688]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f978);
            // 9b919 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9b91e mov rdi,rbx
            rdi = rbx;
            // 9b921 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b926 mov esi,0x2788
            rsi = (0x2788u64) as u32 as u64;
            // 9b92b mov rdi,r15
            rdi = r15;
            // 9b92e mov rdx,rbx
            rdx = rbx;
            // 9b931 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b936 lea rdi,[rbp-0x688]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f978);
            // 9b93d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b942 lea rsi,[rip+0x9a5a2] # 0x135eeb
            rsi = m.env.data_va(304824);
            // 9b949 lea rbx,[rbp-0x6a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f960);
            // 9b950 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 9b955 mov rdi,rbx
            rdi = rbx;
            // 9b958 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b95d mov esi,0x2789
            rsi = (0x2789u64) as u32 as u64;
            // 9b962 mov rdi,r15
            rdi = r15;
            // 9b965 mov rdx,rbx
            rdx = rbx;
            // 9b968 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b96d lea rdi,[rbp-0x6a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f960);
            // 9b974 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b979 lea rsi,[rip+0x9a579] # 0x135ef9
            rsi = m.env.data_va(304838);
            // 9b980 lea rbx,[rbp-0x6b8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f948);
            // 9b987 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9b98c mov rdi,rbx
            rdi = rbx;
            // 9b98f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b994 mov esi,0x278a
            rsi = (0x278au64) as u32 as u64;
            // 9b999 mov rdi,r15
            rdi = r15;
            // 9b99c mov rdx,rbx
            rdx = rbx;
            // 9b99f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b9a4 lea rdi,[rbp-0x6b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f948);
            // 9b9ab call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b9b0 lea rsi,[rip+0x9a55f] # 0x135f16
            rsi = m.env.data_va(304867);
            // 9b9b7 lea rbx,[rbp-0x6d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f930);
            // 9b9be mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 9b9c3 mov rdi,rbx
            rdi = rbx;
            // 9b9c6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b9cb mov esi,0x278b
            rsi = (0x278bu64) as u32 as u64;
            // 9b9d0 mov rdi,r15
            rdi = r15;
            // 9b9d3 mov rdx,rbx
            rdx = rbx;
            // 9b9d6 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b9db lea rdi,[rbp-0x6d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f930);
            // 9b9e2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9b9e7 lea rsi,[rip+0x9a535] # 0x135f23
            rsi = m.env.data_va(304880);
            // 9b9ee lea rbx,[rbp-0x6e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f918);
            // 9b9f5 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9b9fa mov rdi,rbx
            rdi = rbx;
            // 9b9fd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ba02 mov esi,0x278c
            rsi = (0x278cu64) as u32 as u64;
            // 9ba07 mov rdi,r15
            rdi = r15;
            // 9ba0a mov rdx,rbx
            rdx = rbx;
            // 9ba0d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ba12 lea rdi,[rbp-0x6e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f918);
            // 9ba19 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ba1e lea rsi,[rip+0x9a514] # 0x135f39
            rsi = m.env.data_va(304902);
            // 9ba25 lea rbx,[rbp-0x700]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f900);
            // 9ba2c mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9ba31 mov rdi,rbx
            rdi = rbx;
            // 9ba34 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ba39 mov esi,0x278d
            rsi = (0x278du64) as u32 as u64;
            // 9ba3e mov rdi,r15
            rdi = r15;
            // 9ba41 mov rdx,rbx
            rdx = rbx;
            // 9ba44 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ba49 lea rdi,[rbp-0x700]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f900);
            // 9ba50 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ba55 lea rsi,[rip+0x9a4ef] # 0x135f4b
            rsi = m.env.data_va(304920);
            // 9ba5c lea rbx,[rbp-0x718]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f8e8);
            // 9ba63 mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9ba68 mov rdi,rbx
            rdi = rbx;
            // 9ba6b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ba70 mov esi,0x278e
            rsi = (0x278eu64) as u32 as u64;
            // 9ba75 mov rdi,r15
            rdi = r15;
            // 9ba78 mov rdx,rbx
            rdx = rbx;
            // 9ba7b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ba80 lea rdi,[rbp-0x718]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8e8);
            // 9ba87 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ba8c lea rsi,[rip+0x9a4cf] # 0x135f62
            rsi = m.env.data_va(304943);
            // 9ba93 lea rbx,[rbp-0x730]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f8d0);
            // 9ba9a mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9ba9f mov rdi,rbx
            rdi = rbx;
            // 9baa2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9baa7 mov esi,0x278f
            rsi = (0x278fu64) as u32 as u64;
            // 9baac mov rdi,r15
            rdi = r15;
            // 9baaf mov rdx,rbx
            rdx = rbx;
            // 9bab2 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bab7 lea rdi,[rbp-0x730]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8d0);
            // 9babe call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bac3 lea rsi,[rip+0x9a4af] # 0x135f79
            rsi = m.env.data_va(304966);
            // 9baca lea rbx,[rbp-0x748]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f8b8);
            // 9bad1 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9bad6 mov rdi,rbx
            rdi = rbx;
            // 9bad9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bade mov esi,0x2790
            rsi = (0x2790u64) as u32 as u64;
            // 9bae3 mov rdi,r15
            rdi = r15;
            // 9bae6 mov rdx,rbx
            rdx = rbx;
            // 9bae9 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9baee lea rdi,[rbp-0x748]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8b8);
            // 9baf5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bafa lea rsi,[rip+0x9a494] # 0x135f95
            rsi = m.env.data_va(304994);
            // 9bb01 lea rbx,[rbp-0x760]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f8a0);
            // 9bb08 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9bb0d mov rdi,rbx
            rdi = rbx;
            // 9bb10 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bb15 mov esi,0x2792
            rsi = (0x2792u64) as u32 as u64;
            // 9bb1a mov rdi,r15
            rdi = r15;
            // 9bb1d mov rdx,rbx
            rdx = rbx;
            // 9bb20 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bb25 lea rdi,[rbp-0x760]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8a0);
            // 9bb2c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bb31 lea rsi,[rip+0x9a471] # 0x135fa9
            rsi = m.env.data_va(305014);
            // 9bb38 lea rbx,[rbp-0x778]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f888);
            // 9bb3f mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9bb44 mov rdi,rbx
            rdi = rbx;
            // 9bb47 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bb4c mov esi,0x2793
            rsi = (0x2793u64) as u32 as u64;
            // 9bb51 mov rdi,r15
            rdi = r15;
            // 9bb54 mov rdx,rbx
            rdx = rbx;
            // 9bb57 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bb5c lea rdi,[rbp-0x778]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f888);
            // 9bb63 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bb68 lea rsi,[rip+0x9a452] # 0x135fc1
            rsi = m.env.data_va(305038);
            // 9bb6f lea rbx,[rbp-0x790]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f870);
            // 9bb76 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9bb7b mov rdi,rbx
            rdi = rbx;
            // 9bb7e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bb83 mov esi,0x2794
            rsi = (0x2794u64) as u32 as u64;
            // 9bb88 mov rdi,r15
            rdi = r15;
            // 9bb8b mov rdx,rbx
            rdx = rbx;
            // 9bb8e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bb93 lea rdi,[rbp-0x790]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f870);
            // 9bb9a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bb9f lea rsi,[rip+0x9a436] # 0x135fdc
            rsi = m.env.data_va(305065);
            // 9bba6 lea rbx,[rbp-0x7a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f858);
            // 9bbad mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9bbb2 mov rdi,rbx
            rdi = rbx;
            // 9bbb5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bbba mov esi,0x2795
            rsi = (0x2795u64) as u32 as u64;
            // 9bbbf mov rdi,r15
            rdi = r15;
            // 9bbc2 mov rdx,rbx
            rdx = rbx;
            // 9bbc5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bbca lea rdi,[rbp-0x7a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f858);
            // 9bbd1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bbd6 lea rsi,[rip+0x9a41a] # 0x135ff7
            rsi = m.env.data_va(305092);
            // 9bbdd lea rbx,[rbp-0x7c0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f840);
            // 9bbe4 mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9bbe9 mov rdi,rbx
            rdi = rbx;
            // 9bbec call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bbf1 mov esi,0x2796
            rsi = (0x2796u64) as u32 as u64;
            // 9bbf6 mov rdi,r15
            rdi = r15;
            // 9bbf9 mov rdx,rbx
            rdx = rbx;
            // 9bbfc call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bc01 lea rdi,[rbp-0x7c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f840);
            // 9bc08 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bc0d lea rsi,[rip+0x9a3fa] # 0x13600e
            rsi = m.env.data_va(305115);
            // 9bc14 lea rbx,[rbp-0x7d8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f828);
            // 9bc1b mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9bc20 mov rdi,rbx
            rdi = rbx;
            // 9bc23 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bc28 mov esi,0x2797
            rsi = (0x2797u64) as u32 as u64;
            // 9bc2d mov rdi,r15
            rdi = r15;
            // 9bc30 mov rdx,rbx
            rdx = rbx;
            // 9bc33 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bc38 lea rdi,[rbp-0x7d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f828);
            // 9bc3f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bc44 lea rsi,[rip+0x9a3da] # 0x136025
            rsi = m.env.data_va(305138);
            // 9bc4b lea rbx,[rbp-0x7f0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f810);
            // 9bc52 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9bc57 mov rdi,rbx
            rdi = rbx;
            // 9bc5a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bc5f mov esi,0x2798
            rsi = (0x2798u64) as u32 as u64;
            // 9bc64 mov rdi,r15
            rdi = r15;
            // 9bc67 mov rdx,rbx
            rdx = rbx;
            // 9bc6a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bc6f lea rdi,[rbp-0x7f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f810);
            // 9bc76 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bc7b lea rsi,[rip+0x9a3be] # 0x136040
            rsi = m.env.data_va(305165);
            // 9bc82 lea rbx,[rbp-0x808]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f7f8);
            // 9bc89 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9bc8e mov rdi,rbx
            rdi = rbx;
            // 9bc91 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bc96 mov esi,0x2799
            rsi = (0x2799u64) as u32 as u64;
            // 9bc9b mov rdi,r15
            rdi = r15;
            // 9bc9e mov rdx,rbx
            rdx = rbx;
            // 9bca1 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bca6 lea rdi,[rbp-0x808]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7f8);
            // 9bcad call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bcb2 lea rsi,[rip+0x9a3a2] # 0x13605b
            rsi = m.env.data_va(305192);
            // 9bcb9 lea rbx,[rbp-0x820]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f7e0);
            // 9bcc0 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9bcc5 mov rdi,rbx
            rdi = rbx;
            // 9bcc8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bccd mov esi,0x279a
            rsi = (0x279au64) as u32 as u64;
            // 9bcd2 mov rdi,r15
            rdi = r15;
            // 9bcd5 mov rdx,rbx
            rdx = rbx;
            // 9bcd8 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bcdd lea rdi,[rbp-0x820]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7e0);
            // 9bce4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bce9 lea rsi,[rip+0x9a385] # 0x136075
            rsi = m.env.data_va(305218);
            // 9bcf0 lea rbx,[rbp-0x838]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f7c8);
            // 9bcf7 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9bcfc mov rdi,rbx
            rdi = rbx;
            // 9bcff call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bd04 mov esi,0x279b
            rsi = (0x279bu64) as u32 as u64;
            // 9bd09 mov rdi,r15
            rdi = r15;
            // 9bd0c mov rdx,rbx
            rdx = rbx;
            // 9bd0f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bd14 lea rdi,[rbp-0x838]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7c8);
            // 9bd1b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bd20 lea rsi,[rip+0x9a36c] # 0x136093
            rsi = m.env.data_va(305248);
            // 9bd27 lea rbx,[rbp-0x850]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f7b0);
            // 9bd2e mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9bd33 mov rdi,rbx
            rdi = rbx;
            // 9bd36 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bd3b mov esi,0x279e
            rsi = (0x279eu64) as u32 as u64;
            // 9bd40 mov rdi,r15
            rdi = r15;
            // 9bd43 mov rdx,rbx
            rdx = rbx;
            // 9bd46 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bd4b lea rdi,[rbp-0x850]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7b0);
            // 9bd52 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bd57 lea rsi,[rip+0x9a34d] # 0x1360ab
            rsi = m.env.data_va(305272);
            // 9bd5e lea rbx,[rbp-0x868]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f798);
            // 9bd65 mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9bd6a mov rdi,rbx
            rdi = rbx;
            // 9bd6d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bd72 mov esi,0x279f
            rsi = (0x279fu64) as u32 as u64;
            // 9bd77 mov rdi,r15
            rdi = r15;
            // 9bd7a mov rdx,rbx
            rdx = rbx;
            // 9bd7d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bd82 lea rdi,[rbp-0x868]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f798);
            // 9bd89 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bd8e lea rsi,[rip+0x9a32b] # 0x1360c0
            rsi = m.env.data_va(305293);
            // 9bd95 lea rbx,[rbp-0x880]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f780);
            // 9bd9c mov edx,0x1e
            rdx = (0x1eu64) as u32 as u64;
            // 9bda1 mov rdi,rbx
            rdi = rbx;
            // 9bda4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bda9 mov esi,0x27a0
            rsi = (0x27a0u64) as u32 as u64;
            // 9bdae mov rdi,r15
            rdi = r15;
            // 9bdb1 mov rdx,rbx
            rdx = rbx;
            // 9bdb4 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bdb9 lea rdi,[rbp-0x880]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f780);
            // 9bdc0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bdc5 lea rsi,[rip+0x9a313] # 0x1360df
            rsi = m.env.data_va(305324);
            // 9bdcc lea rbx,[rbp-0x898]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f768);
            // 9bdd3 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9bdd8 mov rdi,rbx
            rdi = rbx;
            // 9bddb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bde0 mov esi,0x27a1
            rsi = (0x27a1u64) as u32 as u64;
            // 9bde5 mov rdi,r15
            rdi = r15;
            // 9bde8 mov rdx,rbx
            rdx = rbx;
            // 9bdeb call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bdf0 lea rdi,[rbp-0x898]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f768);
            // 9bdf7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bdfc lea rsi,[rip+0x9a2f5] # 0x1360f8
            rsi = m.env.data_va(305349);
            // 9be03 lea rbx,[rbp-0x8b0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f750);
            // 9be0a mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9be0f mov rdi,rbx
            rdi = rbx;
            // 9be12 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9be17 mov esi,0x27a2
            rsi = (0x27a2u64) as u32 as u64;
            // 9be1c mov rdi,r15
            rdi = r15;
            // 9be1f mov rdx,rbx
            rdx = rbx;
            // 9be22 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9be27 lea rdi,[rbp-0x8b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f750);
            // 9be2e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9be33 lea rsi,[rip+0x9a2d7] # 0x136111
            rsi = m.env.data_va(305374);
            // 9be3a lea rbx,[rbp-0x8c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f738);
            // 9be41 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9be46 mov rdi,rbx
            rdi = rbx;
            // 9be49 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9be4e mov esi,0x27a3
            rsi = (0x27a3u64) as u32 as u64;
            // 9be53 mov rdi,r15
            rdi = r15;
            // 9be56 mov rdx,rbx
            rdx = rbx;
            // 9be59 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9be5e lea rdi,[rbp-0x8c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f738);
            // 9be65 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9be6a lea rsi,[rip+0x9a2be] # 0x13612f
            rsi = m.env.data_va(305404);
            // 9be71 lea rbx,[rbp-0x8e0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f720);
            // 9be78 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9be7d mov rdi,rbx
            rdi = rbx;
            // 9be80 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9be85 mov esi,0x27a4
            rsi = (0x27a4u64) as u32 as u64;
            // 9be8a mov rdi,r15
            rdi = r15;
            // 9be8d mov rdx,rbx
            rdx = rbx;
            // 9be90 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9be95 lea rdi,[rbp-0x8e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f720);
            // 9be9c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bea1 lea rsi,[rip+0x9a2a2] # 0x13614a
            rsi = m.env.data_va(305431);
            // 9bea8 lea rbx,[rbp-0x8f8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f708);
            // 9beaf mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9beb4 mov rdi,rbx
            rdi = rbx;
            // 9beb7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bebc mov esi,0x27a5
            rsi = (0x27a5u64) as u32 as u64;
            // 9bec1 mov rdi,r15
            rdi = r15;
            // 9bec4 mov rdx,rbx
            rdx = rbx;
            // 9bec7 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9becc lea rdi,[rbp-0x8f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f708);
            // 9bed3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bed8 lea rsi,[rip+0x9a288] # 0x136167
            rsi = m.env.data_va(305460);
            // 9bedf lea rbx,[rbp-0x910]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f6f0);
            // 9bee6 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9beeb mov rdi,rbx
            rdi = rbx;
            // 9beee call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bef3 mov esi,0x27a6
            rsi = (0x27a6u64) as u32 as u64;
            // 9bef8 mov rdi,r15
            rdi = r15;
            // 9befb mov rdx,rbx
            rdx = rbx;
            // 9befe call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bf03 lea rdi,[rbp-0x910]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6f0);
            // 9bf0a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bf0f lea rsi,[rip+0x9a26e] # 0x136184
            rsi = m.env.data_va(305489);
            // 9bf16 lea rbx,[rbp-0x928]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f6d8);
            // 9bf1d mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9bf22 mov rdi,rbx
            rdi = rbx;
            // 9bf25 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bf2a mov esi,0x27a7
            rsi = (0x27a7u64) as u32 as u64;
            // 9bf2f mov rdi,r15
            rdi = r15;
            // 9bf32 mov rdx,rbx
            rdx = rbx;
            // 9bf35 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bf3a lea rdi,[rbp-0x928]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6d8);
            // 9bf41 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bf46 lea rsi,[rip+0x9a24f] # 0x13619c
            rsi = m.env.data_va(305513);
            // 9bf4d lea rbx,[rbp-0x940]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f6c0);
            // 9bf54 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9bf59 mov rdi,rbx
            rdi = rbx;
            // 9bf5c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bf61 mov esi,0x27a8
            rsi = (0x27a8u64) as u32 as u64;
            // 9bf66 mov rdi,r15
            rdi = r15;
            // 9bf69 mov rdx,rbx
            rdx = rbx;
            // 9bf6c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bf71 lea rdi,[rbp-0x940]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6c0);
            // 9bf78 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bf7d lea rsi,[rip+0x9a234] # 0x1361b8
            rsi = m.env.data_va(305541);
            // 9bf84 lea rbx,[rbp-0x958]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f6a8);
            // 9bf8b mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9bf90 mov rdi,rbx
            rdi = rbx;
            // 9bf93 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bf98 mov esi,0x27a9
            rsi = (0x27a9u64) as u32 as u64;
            // 9bf9d mov rdi,r15
            rdi = r15;
            // 9bfa0 mov rdx,rbx
            rdx = rbx;
            // 9bfa3 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bfa8 lea rdi,[rbp-0x958]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6a8);
            // 9bfaf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bfb4 lea rsi,[rip+0x9a219] # 0x1361d4
            rsi = m.env.data_va(305569);
            // 9bfbb lea rbx,[rbp-0x970]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f690);
            // 9bfc2 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9bfc7 mov rdi,rbx
            rdi = rbx;
            // 9bfca call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bfcf mov esi,0x27aa
            rsi = (0x27aau64) as u32 as u64;
            // 9bfd4 mov rdi,r15
            rdi = r15;
            // 9bfd7 mov rdx,rbx
            rdx = rbx;
            // 9bfda call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bfdf lea rdi,[rbp-0x970]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f690);
            // 9bfe6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9bfeb lea rsi,[rip+0x9a1fc] # 0x1361ee
            rsi = m.env.data_va(305595);
            // 9bff2 lea rbx,[rbp-0x988]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f678);
            // 9bff9 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9bffe mov rdi,rbx
            rdi = rbx;
            // 9c001 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c006 mov esi,0x27ab
            rsi = (0x27abu64) as u32 as u64;
            // 9c00b mov rdi,r15
            rdi = r15;
            // 9c00e mov rdx,rbx
            rdx = rbx;
            // 9c011 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c016 lea rdi,[rbp-0x988]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f678);
            // 9c01d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c022 lea rsi,[rip+0x9a1e3] # 0x13620c
            rsi = m.env.data_va(305625);
            // 9c029 lea rbx,[rbp-0x9a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f660);
            // 9c030 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9c035 mov rdi,rbx
            rdi = rbx;
            // 9c038 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c03d mov esi,0x27ac
            rsi = (0x27acu64) as u32 as u64;
            // 9c042 mov rdi,r15
            rdi = r15;
            // 9c045 mov rdx,rbx
            rdx = rbx;
            // 9c048 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c04d lea rdi,[rbp-0x9a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f660);
            // 9c054 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c059 lea rsi,[rip+0x9a1c2] # 0x136222
            rsi = m.env.data_va(305647);
            // 9c060 lea rbx,[rbp-0x9b8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f648);
            // 9c067 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9c06c mov rdi,rbx
            rdi = rbx;
            // 9c06f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c074 mov esi,0x27ad
            rsi = (0x27adu64) as u32 as u64;
            // 9c079 mov rdi,r15
            rdi = r15;
            // 9c07c mov rdx,rbx
            rdx = rbx;
            // 9c07f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c084 lea rdi,[rbp-0x9b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f648);
            // 9c08b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c090 lea rsi,[rip+0x9a19f] # 0x136236
            rsi = m.env.data_va(305667);
            // 9c097 lea rbx,[rbp-0x9d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f630);
            // 9c09e mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9c0a3 mov rdi,rbx
            rdi = rbx;
            // 9c0a6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c0ab mov esi,0x27ae
            rsi = (0x27aeu64) as u32 as u64;
            // 9c0b0 mov rdi,r15
            rdi = r15;
            // 9c0b3 mov rdx,rbx
            rdx = rbx;
            // 9c0b6 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c0bb lea rdi,[rbp-0x9d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f630);
            // 9c0c2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c0c7 lea rsi,[rip+0x9a182] # 0x136250
            rsi = m.env.data_va(305693);
            // 9c0ce lea rbx,[rbp-0x9e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f618);
            // 9c0d5 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9c0da mov rdi,rbx
            rdi = rbx;
            // 9c0dd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c0e2 mov esi,0x27af
            rsi = (0x27afu64) as u32 as u64;
            // 9c0e7 mov rdi,r15
            rdi = r15;
            // 9c0ea mov rdx,rbx
            rdx = rbx;
            // 9c0ed call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c0f2 lea rdi,[rbp-0x9e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f618);
            // 9c0f9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c0fe lea rsi,[rip+0x9a164] # 0x136269
            rsi = m.env.data_va(305718);
            // 9c105 lea rbx,[rbp-0xa00]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f600);
            // 9c10c mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9c111 mov rdi,rbx
            rdi = rbx;
            // 9c114 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c119 mov esi,0x27b0
            rsi = (0x27b0u64) as u32 as u64;
            // 9c11e mov rdi,r15
            rdi = r15;
            // 9c121 mov rdx,rbx
            rdx = rbx;
            // 9c124 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c129 lea rdi,[rbp-0xa00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f600);
            // 9c130 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c135 lea rsi,[rip+0x9a142] # 0x13627e
            rsi = m.env.data_va(305739);
            // 9c13c lea rbx,[rbp-0xa18]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f5e8);
            // 9c143 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9c148 mov rdi,rbx
            rdi = rbx;
            // 9c14b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c150 mov esi,0x27b1
            rsi = (0x27b1u64) as u32 as u64;
            // 9c155 mov rdi,r15
            rdi = r15;
            // 9c158 mov rdx,rbx
            rdx = rbx;
            // 9c15b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c160 lea rdi,[rbp-0xa18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5e8);
            // 9c167 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c16c lea rsi,[rip+0x9a124] # 0x136297
            rsi = m.env.data_va(305764);
            // 9c173 lea rbx,[rbp-0xa30]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f5d0);
            // 9c17a mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9c17f mov rdi,rbx
            rdi = rbx;
            // 9c182 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c187 mov esi,0x27b2
            rsi = (0x27b2u64) as u32 as u64;
            // 9c18c mov rdi,r15
            rdi = r15;
            // 9c18f mov rdx,rbx
            rdx = rbx;
            // 9c192 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c197 lea rdi,[rbp-0xa30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5d0);
            // 9c19e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c1a3 lea rsi,[rip+0x9a105] # 0x1362af
            rsi = m.env.data_va(305788);
            // 9c1aa lea rbx,[rbp-0xa48]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f5b8);
            // 9c1b1 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9c1b6 mov rdi,rbx
            rdi = rbx;
            // 9c1b9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c1be mov esi,0x27b3
            rsi = (0x27b3u64) as u32 as u64;
            // 9c1c3 mov rdi,r15
            rdi = r15;
            // 9c1c6 mov rdx,rbx
            rdx = rbx;
            // 9c1c9 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c1ce lea rdi,[rbp-0xa48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5b8);
            // 9c1d5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c1da lea rsi,[rip+0x9a0e1] # 0x1362c2
            rsi = m.env.data_va(305807);
            // 9c1e1 lea rbx,[rbp-0xa60]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f5a0);
            // 9c1e8 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9c1ed mov rdi,rbx
            rdi = rbx;
            // 9c1f0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c1f5 mov esi,0x27b4
            rsi = (0x27b4u64) as u32 as u64;
            // 9c1fa mov rdi,r15
            rdi = r15;
            // 9c1fd mov rdx,rbx
            rdx = rbx;
            // 9c200 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c205 lea rdi,[rbp-0xa60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5a0);
            // 9c20c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c211 lea rsi,[rip+0x9a0bc] # 0x1362d4
            rsi = m.env.data_va(305825);
            // 9c218 lea rbx,[rbp-0xa78]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f588);
            // 9c21f mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9c224 mov rdi,rbx
            rdi = rbx;
            // 9c227 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c22c mov esi,0x27b5
            rsi = (0x27b5u64) as u32 as u64;
            // 9c231 mov rdi,r15
            rdi = r15;
            // 9c234 mov rdx,rbx
            rdx = rbx;
            // 9c237 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c23c lea rdi,[rbp-0xa78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f588);
            // 9c243 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c248 lea rsi,[rip+0x9a09c] # 0x1362eb
            rsi = m.env.data_va(305848);
            // 9c24f lea rbx,[rbp-0xa90]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f570);
            // 9c256 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9c25b mov rdi,rbx
            rdi = rbx;
            // 9c25e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c263 mov esi,0x27b6
            rsi = (0x27b6u64) as u32 as u64;
            // 9c268 mov rdi,r15
            rdi = r15;
            // 9c26b mov rdx,rbx
            rdx = rbx;
            // 9c26e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c273 lea rdi,[rbp-0xa90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f570);
            // 9c27a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c27f lea rsi,[rip+0x9a07b] # 0x136301
            rsi = m.env.data_va(305870);
            // 9c286 lea rbx,[rbp-0xaa8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f558);
            // 9c28d mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // 9c292 mov rdi,rbx
            rdi = rbx;
            // 9c295 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c29a mov esi,0x27b7
            rsi = (0x27b7u64) as u32 as u64;
            // 9c29f mov rdi,r15
            rdi = r15;
            // 9c2a2 mov rdx,rbx
            rdx = rbx;
            // 9c2a5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c2aa lea rdi,[rbp-0xaa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f558);
            // 9c2b1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c2b6 lea rsi,[rip+0x9a065] # 0x136322
            rsi = m.env.data_va(305903);
            // 9c2bd lea rbx,[rbp-0xac0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f540);
            // 9c2c4 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9c2c9 mov rdi,rbx
            rdi = rbx;
            // 9c2cc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c2d1 mov esi,0x27b8
            rsi = (0x27b8u64) as u32 as u64;
            // 9c2d6 mov rdi,r15
            rdi = r15;
            // 9c2d9 mov rdx,rbx
            rdx = rbx;
            // 9c2dc call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c2e1 lea rdi,[rbp-0xac0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f540);
            // 9c2e8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c2ed lea rsi,[rip+0x9a040] # 0x136334
            rsi = m.env.data_va(305921);
            // 9c2f4 lea rbx,[rbp-0xad8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f528);
            // 9c2fb mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9c300 mov rdi,rbx
            rdi = rbx;
            // 9c303 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c308 mov esi,0x27b9
            rsi = (0x27b9u64) as u32 as u64;
            // 9c30d mov rdi,r15
            rdi = r15;
            // 9c310 mov rdx,rbx
            rdx = rbx;
            // 9c313 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c318 lea rdi,[rbp-0xad8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f528);
            // 9c31f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c324 lea rsi,[rip+0x9a01e] # 0x136349
            rsi = m.env.data_va(305942);
            // 9c32b lea rbx,[rbp-0xaf0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f510);
            // 9c332 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9c337 mov rdi,rbx
            rdi = rbx;
            // 9c33a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c33f mov esi,0x27ba
            rsi = (0x27bau64) as u32 as u64;
            // 9c344 mov rdi,r15
            rdi = r15;
            // 9c347 mov rdx,rbx
            rdx = rbx;
            // 9c34a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c34f lea rdi,[rbp-0xaf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f510);
            // 9c356 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c35b lea rsi,[rip+0x9a005] # 0x136367
            rsi = m.env.data_va(305972);
            // 9c362 lea rbx,[rbp-0xb08]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f4f8);
            // 9c369 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9c36e mov rdi,rbx
            rdi = rbx;
            // 9c371 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c376 mov esi,0x27bb
            rsi = (0x27bbu64) as u32 as u64;
            // 9c37b mov rdi,r15
            rdi = r15;
            // 9c37e mov rdx,rbx
            rdx = rbx;
            // 9c381 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c386 lea rdi,[rbp-0xb08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4f8);
            // 9c38d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c392 lea rsi,[rip+0x99fec] # 0x136385
            rsi = m.env.data_va(306002);
            // 9c399 lea rbx,[rbp-0xb20]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f4e0);
            // 9c3a0 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9c3a5 mov rdi,rbx
            rdi = rbx;
            // 9c3a8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c3ad mov esi,0x27bc
            rsi = (0x27bcu64) as u32 as u64;
            // 9c3b2 mov rdi,r15
            rdi = r15;
            // 9c3b5 mov rdx,rbx
            rdx = rbx;
            // 9c3b8 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c3bd lea rdi,[rbp-0xb20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4e0);
            // 9c3c4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c3c9 lea rsi,[rip+0x99fd3] # 0x1363a3
            rsi = m.env.data_va(306032);
            // 9c3d0 lea rbx,[rbp-0xb38]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f4c8);
            // 9c3d7 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9c3dc mov rdi,rbx
            rdi = rbx;
            // 9c3df call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c3e4 mov esi,0x27bd
            rsi = (0x27bdu64) as u32 as u64;
            // 9c3e9 mov rdi,r15
            rdi = r15;
            // 9c3ec mov rdx,rbx
            rdx = rbx;
            // 9c3ef call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c3f4 lea rdi,[rbp-0xb38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4c8);
            // 9c3fb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c400 lea rsi,[rip+0x99fb2] # 0x1363b9
            rsi = m.env.data_va(306054);
            // 9c407 lea rbx,[rbp-0xb50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f4b0);
            // 9c40e mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9c413 mov rdi,rbx
            rdi = rbx;
            // 9c416 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c41b mov esi,0x27be
            rsi = (0x27beu64) as u32 as u64;
            // 9c420 mov rdi,r15
            rdi = r15;
            // 9c423 mov rdx,rbx
            rdx = rbx;
            // 9c426 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c42b lea rdi,[rbp-0xb50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4b0);
            // 9c432 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c437 lea rsi,[rip+0x99f90] # 0x1363ce
            rsi = m.env.data_va(306075);
            // 9c43e lea rbx,[rbp-0xb68]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f498);
            // 9c445 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9c44a mov rdi,rbx
            rdi = rbx;
            // 9c44d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c452 mov esi,0x27bf
            rsi = (0x27bfu64) as u32 as u64;
            // 9c457 mov rdi,r15
            rdi = r15;
            // 9c45a mov rdx,rbx
            rdx = rbx;
            // 9c45d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c462 lea rdi,[rbp-0xb68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f498);
            // 9c469 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c46e lea rsi,[rip+0x99f74] # 0x1363e9
            rsi = m.env.data_va(306102);
            // 9c475 lea rbx,[rbp-0xb80]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f480);
            // 9c47c mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9c481 mov rdi,rbx
            rdi = rbx;
            // 9c484 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c489 mov esi,0x27c0
            rsi = (0x27c0u64) as u32 as u64;
            // 9c48e mov rdi,r15
            rdi = r15;
            // 9c491 mov rdx,rbx
            rdx = rbx;
            // 9c494 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c499 lea rdi,[rbp-0xb80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f480);
            // 9c4a0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c4a5 lea rsi,[rip+0x99f58] # 0x136404
            rsi = m.env.data_va(306129);
            // 9c4ac lea rbx,[rbp-0xb98]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f468);
            // 9c4b3 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9c4b8 mov rdi,rbx
            rdi = rbx;
            // 9c4bb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c4c0 mov esi,0x27c1
            rsi = (0x27c1u64) as u32 as u64;
            // 9c4c5 mov rdi,r15
            rdi = r15;
            // 9c4c8 mov rdx,rbx
            rdx = rbx;
            // 9c4cb call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c4d0 lea rdi,[rbp-0xb98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f468);
            // 9c4d7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c4dc lea rsi,[rip+0x99f3d] # 0x136420
            rsi = m.env.data_va(306157);
            // 9c4e3 lea rbx,[rbp-0xbb0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f450);
            // 9c4ea mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9c4ef mov rdi,rbx
            rdi = rbx;
            // 9c4f2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c4f7 mov esi,0x27c2
            rsi = (0x27c2u64) as u32 as u64;
            // 9c4fc mov rdi,r15
            rdi = r15;
            // 9c4ff mov rdx,rbx
            rdx = rbx;
            // 9c502 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c507 lea rdi,[rbp-0xbb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f450);
            // 9c50e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c513 lea rsi,[rip+0x99f22] # 0x13643c
            rsi = m.env.data_va(306185);
            // 9c51a lea rbx,[rbp-0xbc8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f438);
            // 9c521 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9c526 mov rdi,rbx
            rdi = rbx;
            // 9c529 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c52e mov esi,0x27c3
            rsi = (0x27c3u64) as u32 as u64;
            // 9c533 mov rdi,r15
            rdi = r15;
            // 9c536 mov rdx,rbx
            rdx = rbx;
            // 9c539 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c53e lea rdi,[rbp-0xbc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f438);
            // 9c545 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c54a lea rsi,[rip+0x99f05] # 0x136456
            rsi = m.env.data_va(306211);
            // 9c551 lea rbx,[rbp-0xbe0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f420);
            // 9c558 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9c55d mov rdi,rbx
            rdi = rbx;
            // 9c560 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c565 mov esi,0x27c4
            rsi = (0x27c4u64) as u32 as u64;
            // 9c56a mov rdi,r15
            rdi = r15;
            // 9c56d mov rdx,rbx
            rdx = rbx;
            // 9c570 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c575 lea rdi,[rbp-0xbe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f420);
            // 9c57c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c581 lea rsi,[rip+0x99ee4] # 0x13646c
            rsi = m.env.data_va(306233);
            // 9c588 lea rbx,[rbp-0xbf8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f408);
            // 9c58f mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9c594 mov rdi,rbx
            rdi = rbx;
            // 9c597 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c59c mov esi,0x27c5
            rsi = (0x27c5u64) as u32 as u64;
            // 9c5a1 mov rdi,r15
            rdi = r15;
            // 9c5a4 mov rdx,rbx
            rdx = rbx;
            // 9c5a7 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c5ac lea rdi,[rbp-0xbf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f408);
            // 9c5b3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c5b8 lea rsi,[rip+0x99ec7] # 0x136486
            rsi = m.env.data_va(306259);
            // 9c5bf lea rbx,[rbp-0xc10]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f3f0);
            // 9c5c6 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9c5cb mov rdi,rbx
            rdi = rbx;
            // 9c5ce call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c5d3 mov esi,0x27c6
            rsi = (0x27c6u64) as u32 as u64;
            // 9c5d8 mov rdi,r15
            rdi = r15;
            // 9c5db mov rdx,rbx
            rdx = rbx;
            // 9c5de call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c5e3 lea rdi,[rbp-0xc10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3f0);
            // 9c5ea call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c5ef lea rsi,[rip+0x99ea8] # 0x13649e
            rsi = m.env.data_va(306283);
            // 9c5f6 lea rbx,[rbp-0xc28]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f3d8);
            // 9c5fd mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9c602 mov rdi,rbx
            rdi = rbx;
            // 9c605 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c60a mov esi,0x27c7
            rsi = (0x27c7u64) as u32 as u64;
            // 9c60f mov rdi,r15
            rdi = r15;
            // 9c612 mov rdx,rbx
            rdx = rbx;
            // 9c615 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c61a lea rdi,[rbp-0xc28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3d8);
            // 9c621 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c626 lea rsi,[rip+0x99e8a] # 0x1364b7
            rsi = m.env.data_va(306308);
            // 9c62d lea rbx,[rbp-0xc40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f3c0);
            // 9c634 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9c639 mov rdi,rbx
            rdi = rbx;
            // 9c63c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c641 mov esi,0x27c8
            rsi = (0x27c8u64) as u32 as u64;
            // 9c646 mov rdi,r15
            rdi = r15;
            // 9c649 mov rdx,rbx
            rdx = rbx;
            // 9c64c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c651 lea rdi,[rbp-0xc40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3c0);
            // 9c658 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c65d lea rsi,[rip+0x99e67] # 0x1364cb
            rsi = m.env.data_va(306328);
            // 9c664 lea rbx,[rbp-0xc58]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f3a8);
            // 9c66b mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9c670 mov rdi,rbx
            rdi = rbx;
            // 9c673 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c678 mov esi,0x27c9
            rsi = (0x27c9u64) as u32 as u64;
            // 9c67d mov rdi,r15
            rdi = r15;
            // 9c680 mov rdx,rbx
            rdx = rbx;
            // 9c683 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c688 lea rdi,[rbp-0xc58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3a8);
            // 9c68f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c694 lea rsi,[rip+0x99e45] # 0x1364e0
            rsi = m.env.data_va(306349);
            // 9c69b lea rbx,[rbp-0xc70]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f390);
            // 9c6a2 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9c6a7 mov rdi,rbx
            rdi = rbx;
            // 9c6aa call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c6af mov esi,0x27ca
            rsi = (0x27cau64) as u32 as u64;
            // 9c6b4 mov rdi,r15
            rdi = r15;
            // 9c6b7 mov rdx,rbx
            rdx = rbx;
            // 9c6ba call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c6bf lea rdi,[rbp-0xc70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f390);
            // 9c6c6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c6cb lea rsi,[rip+0x99e26] # 0x1364f8
            rsi = m.env.data_va(306373);
            // 9c6d2 lea rbx,[rbp-0xc88]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f378);
            // 9c6d9 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9c6de mov rdi,rbx
            rdi = rbx;
            // 9c6e1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c6e6 mov esi,0x27cb
            rsi = (0x27cbu64) as u32 as u64;
            // 9c6eb mov rdi,r15
            rdi = r15;
            // 9c6ee mov rdx,rbx
            rdx = rbx;
            // 9c6f1 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c6f6 lea rdi,[rbp-0xc88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f378);
            // 9c6fd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c702 lea rsi,[rip+0x99e08] # 0x136511
            rsi = m.env.data_va(306398);
            // 9c709 lea rbx,[rbp-0xca0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f360);
            // 9c710 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9c715 mov rdi,rbx
            rdi = rbx;
            // 9c718 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c71d mov esi,0x27cc
            rsi = (0x27ccu64) as u32 as u64;
            // 9c722 mov rdi,r15
            rdi = r15;
            // 9c725 mov rdx,rbx
            rdx = rbx;
            // 9c728 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c72d lea rdi,[rbp-0xca0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f360);
            // 9c734 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c739 lea rsi,[rip+0x99deb] # 0x13652b
            rsi = m.env.data_va(306424);
            // 9c740 lea rbx,[rbp-0xcb8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f348);
            // 9c747 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9c74c mov rdi,rbx
            rdi = rbx;
            // 9c74f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c754 mov esi,0x27d8
            rsi = (0x27d8u64) as u32 as u64;
            // 9c759 mov rdi,r15
            rdi = r15;
            // 9c75c mov rdx,rbx
            rdx = rbx;
            // 9c75f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c764 lea rdi,[rbp-0xcb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f348);
            // 9c76b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c770 lea rsi,[rip+0x99dd2] # 0x136549
            rsi = m.env.data_va(306454);
            // 9c777 lea rbx,[rbp-0xcd0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f330);
            // 9c77e mov edx,0x21
            rdx = (0x21u64) as u32 as u64;
            // 9c783 mov rdi,rbx
            rdi = rbx;
            // 9c786 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c78b mov esi,0x27d9
            rsi = (0x27d9u64) as u32 as u64;
            // 9c790 mov rdi,r15
            rdi = r15;
            // 9c793 mov rdx,rbx
            rdx = rbx;
            // 9c796 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c79b lea rdi,[rbp-0xcd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f330);
            // 9c7a2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c7a7 lea rsi,[rip+0x99dbd] # 0x13656b
            rsi = m.env.data_va(306488);
            // 9c7ae lea rbx,[rbp-0xce8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f318);
            // 9c7b5 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9c7ba mov rdi,rbx
            rdi = rbx;
            // 9c7bd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c7c2 mov esi,0x27da
            rsi = (0x27dau64) as u32 as u64;
            // 9c7c7 mov rdi,r15
            rdi = r15;
            // 9c7ca mov rdx,rbx
            rdx = rbx;
            // 9c7cd call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c7d2 lea rdi,[rbp-0xce8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f318);
            // 9c7d9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c7de lea rsi,[rip+0x99da6] # 0x13658b
            rsi = m.env.data_va(306520);
            // 9c7e5 lea rbx,[rbp-0xd00]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f300);
            // 9c7ec mov edx,0x21
            rdx = (0x21u64) as u32 as u64;
            // 9c7f1 mov rdi,rbx
            rdi = rbx;
            // 9c7f4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c7f9 mov esi,0x27db
            rsi = (0x27dbu64) as u32 as u64;
            // 9c7fe mov rdi,r15
            rdi = r15;
            // 9c801 mov rdx,rbx
            rdx = rbx;
            // 9c804 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c809 lea rdi,[rbp-0xd00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f300);
            // 9c810 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c815 lea rsi,[rip+0x99d91] # 0x1365ad
            rsi = m.env.data_va(306554);
            // 9c81c lea rbx,[rbp-0xd18]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f2e8);
            // 9c823 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9c828 mov rdi,rbx
            rdi = rbx;
            // 9c82b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c830 mov esi,0x27dc
            rsi = (0x27dcu64) as u32 as u64;
            // 9c835 mov rdi,r15
            rdi = r15;
            // 9c838 mov rdx,rbx
            rdx = rbx;
            // 9c83b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c840 lea rdi,[rbp-0xd18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2e8);
            // 9c847 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c84c lea rsi,[rip+0x99d75] # 0x1365c8
            rsi = m.env.data_va(306581);
            // 9c853 lea rbx,[rbp-0xd30]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f2d0);
            // 9c85a mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9c85f mov rdi,rbx
            rdi = rbx;
            // 9c862 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c867 mov esi,0x27dd
            rsi = (0x27ddu64) as u32 as u64;
            // 9c86c mov rdi,r15
            rdi = r15;
            // 9c86f mov rdx,rbx
            rdx = rbx;
            // 9c872 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c877 lea rdi,[rbp-0xd30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2d0);
            // 9c87e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c883 lea rsi,[rip+0x99d5e] # 0x1365e8
            rsi = m.env.data_va(306613);
            // 9c88a lea rbx,[rbp-0xd48]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f2b8);
            // 9c891 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9c896 mov rdi,rbx
            rdi = rbx;
            // 9c899 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c89e mov esi,0x27de
            rsi = (0x27deu64) as u32 as u64;
            // 9c8a3 mov rdi,r15
            rdi = r15;
            // 9c8a6 mov rdx,rbx
            rdx = rbx;
            // 9c8a9 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c8ae lea rdi,[rbp-0xd48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2b8);
            // 9c8b5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c8ba lea rsi,[rip+0x99d47] # 0x136608
            rsi = m.env.data_va(306645);
            // 9c8c1 lea rbx,[rbp-0xd60]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f2a0);
            // 9c8c8 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9c8cd mov rdi,rbx
            rdi = rbx;
            // 9c8d0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c8d5 mov esi,0x27df
            rsi = (0x27dfu64) as u32 as u64;
            // 9c8da mov rdi,r15
            rdi = r15;
            // 9c8dd mov rdx,rbx
            rdx = rbx;
            // 9c8e0 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c8e5 lea rdi,[rbp-0xd60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2a0);
            // 9c8ec call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c8f1 lea rsi,[rip+0x99d30] # 0x136628
            rsi = m.env.data_va(306677);
            // 9c8f8 lea rbx,[rbp-0xd78]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f288);
            // 9c8ff mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9c904 mov rdi,rbx
            rdi = rbx;
            // 9c907 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c90c mov esi,0x27ec
            rsi = (0x27ecu64) as u32 as u64;
            // 9c911 mov rdi,r15
            rdi = r15;
            // 9c914 mov rdx,rbx
            rdx = rbx;
            // 9c917 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c91c lea rdi,[rbp-0xd78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f288);
            // 9c923 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c928 lea rsi,[rip+0x99d17] # 0x136646
            rsi = m.env.data_va(306707);
            // 9c92f lea rbx,[rbp-0xd90]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f270);
            // 9c936 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9c93b mov rdi,rbx
            rdi = rbx;
            // 9c93e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c943 mov esi,0x27ed
            rsi = (0x27edu64) as u32 as u64;
            // 9c948 mov rdi,r15
            rdi = r15;
            // 9c94b mov rdx,rbx
            rdx = rbx;
            // 9c94e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c953 lea rdi,[rbp-0xd90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f270);
            // 9c95a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c95f lea rsi,[rip+0x99cfe] # 0x136664
            rsi = m.env.data_va(306737);
            // 9c966 lea rbx,[rbp-0xda8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f258);
            // 9c96d mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9c972 mov rdi,rbx
            rdi = rbx;
            // 9c975 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c97a mov esi,0x27ee
            rsi = (0x27eeu64) as u32 as u64;
            // 9c97f mov rdi,r15
            rdi = r15;
            // 9c982 mov rdx,rbx
            rdx = rbx;
            // 9c985 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c98a lea rdi,[rbp-0xda8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f258);
            // 9c991 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c996 lea rsi,[rip+0x99ce3] # 0x136680
            rsi = m.env.data_va(306765);
            // 9c99d lea rbx,[rbp-0xdc0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f240);
            // 9c9a4 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9c9a9 mov rdi,rbx
            rdi = rbx;
            // 9c9ac call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c9b1 mov esi,0x27ef
            rsi = (0x27efu64) as u32 as u64;
            // 9c9b6 mov rdi,r15
            rdi = r15;
            // 9c9b9 mov rdx,rbx
            rdx = rbx;
            // 9c9bc call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c9c1 lea rdi,[rbp-0xdc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f240);
            // 9c9c8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c9cd lea rsi,[rip+0x99ccc] # 0x1366a0
            rsi = m.env.data_va(306797);
            // 9c9d4 lea rbx,[rbp-0xdd8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f228);
            // 9c9db mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9c9e0 mov rdi,rbx
            rdi = rbx;
            // 9c9e3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c9e8 mov esi,0x27f0
            rsi = (0x27f0u64) as u32 as u64;
            // 9c9ed mov rdi,r15
            rdi = r15;
            // 9c9f0 mov rdx,rbx
            rdx = rbx;
            // 9c9f3 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9c9f8 lea rdi,[rbp-0xdd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f228);
            // 9c9ff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ca04 lea rsi,[rip+0x99cb5] # 0x1366c0
            rsi = m.env.data_va(306829);
            // 9ca0b lea rbx,[rbp-0xdf0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f210);
            // 9ca12 mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9ca17 mov rdi,rbx
            rdi = rbx;
            // 9ca1a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ca1f mov esi,0x27f1
            rsi = (0x27f1u64) as u32 as u64;
            // 9ca24 mov rdi,r15
            rdi = r15;
            // 9ca27 mov rdx,rbx
            rdx = rbx;
            // 9ca2a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ca2f lea rdi,[rbp-0xdf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f210);
            // 9ca36 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ca3b lea rsi,[rip+0x99c95] # 0x1366d7
            rsi = m.env.data_va(306852);
            // 9ca42 lea rbx,[rbp-0xe08]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f1f8);
            // 9ca49 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9ca4e mov rdi,rbx
            rdi = rbx;
            // 9ca51 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ca56 mov esi,0x27f2
            rsi = (0x27f2u64) as u32 as u64;
            // 9ca5b mov rdi,r15
            rdi = r15;
            // 9ca5e mov rdx,rbx
            rdx = rbx;
            // 9ca61 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ca66 lea rdi,[rbp-0xe08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1f8);
            // 9ca6d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ca72 lea rsi,[rip+0x99c7a] # 0x1366f3
            rsi = m.env.data_va(306880);
            // 9ca79 lea rbx,[rbp-0xe20]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f1e0);
            // 9ca80 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9ca85 mov rdi,rbx
            rdi = rbx;
            // 9ca88 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ca8d mov esi,0x27f3
            rsi = (0x27f3u64) as u32 as u64;
            // 9ca92 mov rdi,r15
            rdi = r15;
            // 9ca95 mov rdx,rbx
            rdx = rbx;
            // 9ca98 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ca9d lea rdi,[rbp-0xe20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1e0);
            // 9caa4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9caa9 lea rsi,[rip+0x99c5f] # 0x13670f
            rsi = m.env.data_va(306908);
            // 9cab0 lea rbx,[rbp-0xe38]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f1c8);
            // 9cab7 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9cabc mov rdi,rbx
            rdi = rbx;
            // 9cabf call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cac4 mov esi,0x27f4
            rsi = (0x27f4u64) as u32 as u64;
            // 9cac9 mov rdi,r15
            rdi = r15;
            // 9cacc mov rdx,rbx
            rdx = rbx;
            // 9cacf call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cad4 lea rdi,[rbp-0xe38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1c8);
            // 9cadb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cae0 lea rsi,[rip+0x99c44] # 0x13672b
            rsi = m.env.data_va(306936);
            // 9cae7 lea rbx,[rbp-0xe50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f1b0);
            // 9caee mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9caf3 mov rdi,rbx
            rdi = rbx;
            // 9caf6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cafb mov esi,0x2800
            rsi = (0x2800u64) as u32 as u64;
            // 9cb00 mov rdi,r15
            rdi = r15;
            // 9cb03 mov rdx,rbx
            rdx = rbx;
            // 9cb06 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cb0b lea rdi,[rbp-0xe50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1b0);
            // 9cb12 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cb17 lea rsi,[rip+0x99c2a] # 0x136748
            rsi = m.env.data_va(306965);
            // 9cb1e lea rbx,[rbp-0xe68]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f198);
            // 9cb25 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9cb2a mov rdi,rbx
            rdi = rbx;
            // 9cb2d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cb32 mov esi,0x2801
            rsi = (0x2801u64) as u32 as u64;
            // 9cb37 mov rdi,r15
            rdi = r15;
            // 9cb3a mov rdx,rbx
            rdx = rbx;
            // 9cb3d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cb42 lea rdi,[rbp-0xe68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f198);
            // 9cb49 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cb4e lea rsi,[rip+0x99c10] # 0x136765
            rsi = m.env.data_va(306994);
            // 9cb55 lea rbx,[rbp-0xe80]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f180);
            // 9cb5c mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9cb61 mov rdi,rbx
            rdi = rbx;
            // 9cb64 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cb69 mov esi,0x2802
            rsi = (0x2802u64) as u32 as u64;
            // 9cb6e mov rdi,r15
            rdi = r15;
            // 9cb71 mov rdx,rbx
            rdx = rbx;
            // 9cb74 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cb79 lea rdi,[rbp-0xe80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f180);
            // 9cb80 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cb85 lea rsi,[rip+0x99bf4] # 0x136780
            rsi = m.env.data_va(307021);
            // 9cb8c lea rbx,[rbp-0xe98]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f168);
            // 9cb93 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9cb98 mov rdi,rbx
            rdi = rbx;
            // 9cb9b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cba0 mov esi,0x2803
            rsi = (0x2803u64) as u32 as u64;
            // 9cba5 mov rdi,r15
            rdi = r15;
            // 9cba8 mov rdx,rbx
            rdx = rbx;
            // 9cbab call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cbb0 lea rdi,[rbp-0xe98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f168);
            // 9cbb7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cbbc lea rsi,[rip+0x99bd8] # 0x13679b
            rsi = m.env.data_va(307048);
            // 9cbc3 lea rbx,[rbp-0xeb0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f150);
            // 9cbca mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9cbcf mov rdi,rbx
            rdi = rbx;
            // 9cbd2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cbd7 mov esi,0x2814
            rsi = (0x2814u64) as u32 as u64;
            // 9cbdc mov rdi,r15
            rdi = r15;
            // 9cbdf mov rdx,rbx
            rdx = rbx;
            // 9cbe2 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cbe7 lea rdi,[rbp-0xeb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f150);
            // 9cbee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cbf3 lea rsi,[rip+0x99bb9] # 0x1367b3
            rsi = m.env.data_va(307072);
            // 9cbfa lea rbx,[rbp-0xec8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f138);
            // 9cc01 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 9cc06 mov rdi,rbx
            rdi = rbx;
            // 9cc09 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cc0e mov esi,0x2815
            rsi = (0x2815u64) as u32 as u64;
            // 9cc13 mov rdi,r15
            rdi = r15;
            // 9cc16 mov rdx,rbx
            rdx = rbx;
            // 9cc19 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cc1e lea rdi,[rbp-0xec8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f138);
            // 9cc25 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cc2a lea rsi,[rip+0x99b90] # 0x1367c1
            rsi = m.env.data_va(307086);
            // 9cc31 lea rbx,[rbp-0xee0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f120);
            // 9cc38 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 9cc3d mov rdi,rbx
            rdi = rbx;
            // 9cc40 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cc45 mov esi,0x2816
            rsi = (0x2816u64) as u32 as u64;
            // 9cc4a mov rdi,r15
            rdi = r15;
            // 9cc4d mov rdx,rbx
            rdx = rbx;
            // 9cc50 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cc55 lea rdi,[rbp-0xee0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f120);
            // 9cc5c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cc61 lea rsi,[rip+0x99b67] # 0x1367cf
            rsi = m.env.data_va(307100);
            // 9cc68 lea rbx,[rbp-0xef8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f108);
            // 9cc6f mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 9cc74 mov rdi,rbx
            rdi = rbx;
            // 9cc77 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cc7c mov esi,0x2817
            rsi = (0x2817u64) as u32 as u64;
            // 9cc81 mov rdi,r15
            rdi = r15;
            // 9cc84 mov rdx,rbx
            rdx = rbx;
            // 9cc87 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cc8c lea rdi,[rbp-0xef8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f108);
            // 9cc93 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cc98 lea rsi,[rip+0x99b3e] # 0x1367dd
            rsi = m.env.data_va(307114);
            // 9cc9f lea rbx,[rbp-0xf10]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f0f0);
            // 9cca6 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9ccab mov rdi,rbx
            rdi = rbx;
            // 9ccae call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ccb3 mov esi,0x2818
            rsi = (0x2818u64) as u32 as u64;
            // 9ccb8 mov rdi,r15
            rdi = r15;
            // 9ccbb mov rdx,rbx
            rdx = rbx;
            // 9ccbe call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ccc3 lea rdi,[rbp-0xf10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0f0);
            // 9ccca call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cccf lea rsi,[rip+0x99b21] # 0x1367f7
            rsi = m.env.data_va(307140);
            // 9ccd6 lea rbx,[rbp-0xf28]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f0d8);
            // 9ccdd mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9cce2 mov rdi,rbx
            rdi = rbx;
            // 9cce5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ccea mov esi,0x2819
            rsi = (0x2819u64) as u32 as u64;
            // 9ccef mov rdi,r15
            rdi = r15;
            // 9ccf2 mov rdx,rbx
            rdx = rbx;
            // 9ccf5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ccfa lea rdi,[rbp-0xf28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0d8);
            // 9cd01 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cd06 lea rsi,[rip+0x99b04] # 0x136811
            rsi = m.env.data_va(307166);
            // 9cd0d lea rbx,[rbp-0xf40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f0c0);
            // 9cd14 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9cd19 mov rdi,rbx
            rdi = rbx;
            // 9cd1c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cd21 mov esi,0x281a
            rsi = (0x281au64) as u32 as u64;
            // 9cd26 mov rdi,r15
            rdi = r15;
            // 9cd29 mov rdx,rbx
            rdx = rbx;
            // 9cd2c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cd31 lea rdi,[rbp-0xf40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0c0);
            // 9cd38 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cd3d lea rsi,[rip+0x99ae3] # 0x136827
            rsi = m.env.data_va(307188);
            // 9cd44 lea rbx,[rbp-0xf58]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f0a8);
            // 9cd4b mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9cd50 mov rdi,rbx
            rdi = rbx;
            // 9cd53 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cd58 mov esi,0x281b
            rsi = (0x281bu64) as u32 as u64;
            // 9cd5d mov rdi,r15
            rdi = r15;
            // 9cd60 mov rdx,rbx
            rdx = rbx;
            // 9cd63 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cd68 lea rdi,[rbp-0xf58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0a8);
            // 9cd6f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cd74 lea rsi,[rip+0x99aca] # 0x136845
            rsi = m.env.data_va(307218);
            // 9cd7b lea rbx,[rbp-0xf70]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f090);
            // 9cd82 mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9cd87 mov rdi,rbx
            rdi = rbx;
            // 9cd8a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cd8f mov esi,0x281c
            rsi = (0x281cu64) as u32 as u64;
            // 9cd94 mov rdi,r15
            rdi = r15;
            // 9cd97 mov rdx,rbx
            rdx = rbx;
            // 9cd9a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cd9f lea rdi,[rbp-0xf70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f090);
            // 9cda6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cdab lea rsi,[rip+0x99aaa] # 0x13685c
            rsi = m.env.data_va(307241);
            // 9cdb2 lea rbx,[rbp-0xf88]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f078);
            // 9cdb9 mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9cdbe mov rdi,rbx
            rdi = rbx;
            // 9cdc1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cdc6 mov esi,0x281d
            rsi = (0x281du64) as u32 as u64;
            // 9cdcb mov rdi,r15
            rdi = r15;
            // 9cdce mov rdx,rbx
            rdx = rbx;
            // 9cdd1 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cdd6 lea rdi,[rbp-0xf88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f078);
            // 9cddd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cde2 lea rsi,[rip+0x99a8a] # 0x136873
            rsi = m.env.data_va(307264);
            // 9cde9 lea rbx,[rbp-0xfa0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f060);
            // 9cdf0 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9cdf5 mov rdi,rbx
            rdi = rbx;
            // 9cdf8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cdfd mov esi,0x281e
            rsi = (0x281eu64) as u32 as u64;
            // 9ce02 mov rdi,r15
            rdi = r15;
            // 9ce05 mov rdx,rbx
            rdx = rbx;
            // 9ce08 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ce0d lea rdi,[rbp-0xfa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f060);
            // 9ce14 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ce19 lea rsi,[rip+0x99a6d] # 0x13688d
            rsi = m.env.data_va(307290);
            // 9ce20 lea rbx,[rbp-0xfb8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f048);
            // 9ce27 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9ce2c mov rdi,rbx
            rdi = rbx;
            // 9ce2f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ce34 mov esi,0x281f
            rsi = (0x281fu64) as u32 as u64;
            // 9ce39 mov rdi,r15
            rdi = r15;
            // 9ce3c mov rdx,rbx
            rdx = rbx;
            // 9ce3f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ce44 lea rdi,[rbp-0xfb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f048);
            // 9ce4b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ce50 lea rsi,[rip+0x99a48] # 0x13689f
            rsi = m.env.data_va(307308);
            // 9ce57 lea rbx,[rbp-0xfd0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f030);
            // 9ce5e mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9ce63 mov rdi,rbx
            rdi = rbx;
            // 9ce66 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ce6b mov esi,0x2820
            rsi = (0x2820u64) as u32 as u64;
            // 9ce70 mov rdi,r15
            rdi = r15;
            // 9ce73 mov rdx,rbx
            rdx = rbx;
            // 9ce76 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ce7b lea rdi,[rbp-0xfd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f030);
            // 9ce82 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ce87 lea rsi,[rip+0x99a2a] # 0x1368b8
            rsi = m.env.data_va(307333);
            // 9ce8e lea rbx,[rbp-0xfe8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f018);
            // 9ce95 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9ce9a mov rdi,rbx
            rdi = rbx;
            // 9ce9d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cea2 mov esi,0x2821
            rsi = (0x2821u64) as u32 as u64;
            // 9cea7 mov rdi,r15
            rdi = r15;
            // 9ceaa mov rdx,rbx
            rdx = rbx;
            // 9cead call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ceb2 lea rdi,[rbp-0xfe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f018);
            // 9ceb9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cebe lea rsi,[rip+0x99a0b] # 0x1368d0
            rsi = m.env.data_va(307357);
            // 9cec5 lea rbx,[rbp-0x1000]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f000);
            // 9cecc mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9ced1 mov rdi,rbx
            rdi = rbx;
            // 9ced4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ced9 mov esi,0x2822
            rsi = (0x2822u64) as u32 as u64;
            // 9cede mov rdi,r15
            rdi = r15;
            // 9cee1 mov rdx,rbx
            rdx = rbx;
            // 9cee4 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cee9 lea rdi,[rbp-0x1000]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f000);
            // 9cef0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cef5 lea rsi,[rip+0x999ee] # 0x1368ea
            rsi = m.env.data_va(307383);
            // 9cefc lea rbx,[rbp-0x1018]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_efe8);
            // 9cf03 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 9cf08 mov rdi,rbx
            rdi = rbx;
            // 9cf0b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cf10 mov esi,0x2823
            rsi = (0x2823u64) as u32 as u64;
            // 9cf15 mov rdi,r15
            rdi = r15;
            // 9cf18 mov rdx,rbx
            rdx = rbx;
            // 9cf1b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cf20 lea rdi,[rbp-0x1018]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efe8);
            // 9cf27 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cf2c lea rsi,[rip+0x999c5] # 0x1368f8
            rsi = m.env.data_va(307397);
            // 9cf33 lea rbx,[rbp-0x1030]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_efd0);
            // 9cf3a mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9cf3f mov rdi,rbx
            rdi = rbx;
            // 9cf42 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cf47 mov esi,0x283c
            rsi = (0x283cu64) as u32 as u64;
            // 9cf4c mov rdi,r15
            rdi = r15;
            // 9cf4f mov rdx,rbx
            rdx = rbx;
            // 9cf52 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cf57 lea rdi,[rbp-0x1030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efd0);
            // 9cf5e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cf63 lea rsi,[rip+0x999a9] # 0x136913
            rsi = m.env.data_va(307424);
            // 9cf6a lea rbx,[rbp-0x1048]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_efb8);
            // 9cf71 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9cf76 mov rdi,rbx
            rdi = rbx;
            // 9cf79 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cf7e mov esi,0x283d
            rsi = (0x283du64) as u32 as u64;
            // 9cf83 mov rdi,r15
            rdi = r15;
            // 9cf86 mov rdx,rbx
            rdx = rbx;
            // 9cf89 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cf8e lea rdi,[rbp-0x1048]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efb8);
            // 9cf95 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cf9a lea rsi,[rip+0x9998f] # 0x136930
            rsi = m.env.data_va(307453);
            // 9cfa1 lea rbx,[rbp-0x1060]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_efa0);
            // 9cfa8 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9cfad mov rdi,rbx
            rdi = rbx;
            // 9cfb0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cfb5 mov esi,0x283e
            rsi = (0x283eu64) as u32 as u64;
            // 9cfba mov rdi,r15
            rdi = r15;
            // 9cfbd mov rdx,rbx
            rdx = rbx;
            // 9cfc0 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cfc5 lea rdi,[rbp-0x1060]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efa0);
            // 9cfcc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cfd1 lea rsi,[rip+0x99972] # 0x13694a
            rsi = m.env.data_va(307479);
            // 9cfd8 lea rbx,[rbp-0x1078]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ef88);
            // 9cfdf mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9cfe4 mov rdi,rbx
            rdi = rbx;
            // 9cfe7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cfec mov esi,0x283f
            rsi = (0x283fu64) as u32 as u64;
            // 9cff1 mov rdi,r15
            rdi = r15;
            // 9cff4 mov rdx,rbx
            rdx = rbx;
            // 9cff7 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9cffc lea rdi,[rbp-0x1078]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef88);
            // 9d003 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d008 lea rsi,[rip+0x99957] # 0x136966
            rsi = m.env.data_va(307507);
            // 9d00f lea rbx,[rbp-0x1090]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ef70);
            // 9d016 mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9d01b mov rdi,rbx
            rdi = rbx;
            // 9d01e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d023 mov esi,0x2840
            rsi = (0x2840u64) as u32 as u64;
            // 9d028 mov rdi,r15
            rdi = r15;
            // 9d02b mov rdx,rbx
            rdx = rbx;
            // 9d02e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d033 lea rdi,[rbp-0x1090]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef70);
            // 9d03a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d03f lea rsi,[rip+0x99937] # 0x13697d
            rsi = m.env.data_va(307530);
            // 9d046 lea rbx,[rbp-0x10a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ef58);
            // 9d04d mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9d052 mov rdi,rbx
            rdi = rbx;
            // 9d055 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d05a mov esi,0x2841
            rsi = (0x2841u64) as u32 as u64;
            // 9d05f mov rdi,r15
            rdi = r15;
            // 9d062 mov rdx,rbx
            rdx = rbx;
            // 9d065 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d06a lea rdi,[rbp-0x10a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef58);
            // 9d071 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d076 lea rsi,[rip+0x99917] # 0x136994
            rsi = m.env.data_va(307553);
            // 9d07d lea rbx,[rbp-0x10c0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ef40);
            // 9d084 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9d089 mov rdi,rbx
            rdi = rbx;
            // 9d08c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d091 mov esi,0x2842
            rsi = (0x2842u64) as u32 as u64;
            // 9d096 mov rdi,r15
            rdi = r15;
            // 9d099 mov rdx,rbx
            rdx = rbx;
            // 9d09c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d0a1 lea rdi,[rbp-0x10c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef40);
            // 9d0a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d0ad lea rsi,[rip+0x998f6] # 0x1369aa
            rsi = m.env.data_va(307575);
            // 9d0b4 lea rbx,[rbp-0x10d8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ef28);
            // 9d0bb mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9d0c0 mov rdi,rbx
            rdi = rbx;
            // 9d0c3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d0c8 mov esi,0x2843
            rsi = (0x2843u64) as u32 as u64;
            // 9d0cd mov rdi,r15
            rdi = r15;
            // 9d0d0 mov rdx,rbx
            rdx = rbx;
            // 9d0d3 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d0d8 lea rdi,[rbp-0x10d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef28);
            // 9d0df call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d0e4 lea rsi,[rip+0x998d4] # 0x1369bf
            rsi = m.env.data_va(307596);
            // 9d0eb lea rbx,[rbp-0x10f0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ef10);
            // 9d0f2 mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9d0f7 mov rdi,rbx
            rdi = rbx;
            // 9d0fa call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d0ff mov esi,0x2844
            rsi = (0x2844u64) as u32 as u64;
            // 9d104 mov rdi,r15
            rdi = r15;
            // 9d107 mov rdx,rbx
            rdx = rbx;
            // 9d10a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d10f lea rdi,[rbp-0x10f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef10);
            // 9d116 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d11b lea rsi,[rip+0x998b4] # 0x1369d6
            rsi = m.env.data_va(307619);
            // 9d122 lea rbx,[rbp-0x1108]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eef8);
            // 9d129 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9d12e mov rdi,rbx
            rdi = rbx;
            // 9d131 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d136 mov esi,0x2845
            rsi = (0x2845u64) as u32 as u64;
            // 9d13b mov rdi,r15
            rdi = r15;
            // 9d13e mov rdx,rbx
            rdx = rbx;
            // 9d141 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d146 lea rdi,[rbp-0x1108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eef8);
            // 9d14d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d152 lea rsi,[rip+0x9989d] # 0x1369f6
            rsi = m.env.data_va(307651);
            // 9d159 lea rbx,[rbp-0x1120]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eee0);
            // 9d160 mov edx,0x1e
            rdx = (0x1eu64) as u32 as u64;
            // 9d165 mov rdi,rbx
            rdi = rbx;
            // 9d168 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d16d mov esi,0x2846
            rsi = (0x2846u64) as u32 as u64;
            // 9d172 mov rdi,r15
            rdi = r15;
            // 9d175 mov rdx,rbx
            rdx = rbx;
            // 9d178 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d17d lea rdi,[rbp-0x1120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eee0);
            // 9d184 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d189 lea rsi,[rip+0x99885] # 0x136a15
            rsi = m.env.data_va(307682);
            // 9d190 lea rbx,[rbp-0x1138]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eec8);
            // 9d197 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9d19c mov rdi,rbx
            rdi = rbx;
            // 9d19f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d1a4 mov esi,0x2847
            rsi = (0x2847u64) as u32 as u64;
            // 9d1a9 mov rdi,r15
            rdi = r15;
            // 9d1ac mov rdx,rbx
            rdx = rbx;
            // 9d1af call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d1b4 lea rdi,[rbp-0x1138]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eec8);
            // 9d1bb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d1c0 lea rsi,[rip+0x99866] # 0x136a2d
            rsi = m.env.data_va(307706);
            // 9d1c7 lea rbx,[rbp-0x1150]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eeb0);
            // 9d1ce mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9d1d3 mov rdi,rbx
            rdi = rbx;
            // 9d1d6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d1db mov esi,0x2848
            rsi = (0x2848u64) as u32 as u64;
            // 9d1e0 mov rdi,r15
            rdi = r15;
            // 9d1e3 mov rdx,rbx
            rdx = rbx;
            // 9d1e6 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d1eb lea rdi,[rbp-0x1150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eeb0);
            // 9d1f2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d1f7 lea rsi,[rip+0x99841] # 0x136a3f
            rsi = m.env.data_va(307724);
            // 9d1fe lea rbx,[rbp-0x1168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ee98);
            // 9d205 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9d20a mov rdi,rbx
            rdi = rbx;
            // 9d20d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d212 mov esi,0x2849
            rsi = (0x2849u64) as u32 as u64;
            // 9d217 mov rdi,r15
            rdi = r15;
            // 9d21a mov rdx,rbx
            rdx = rbx;
            // 9d21d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d222 lea rdi,[rbp-0x1168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee98);
            // 9d229 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d22e lea rsi,[rip+0x99820] # 0x136a55
            rsi = m.env.data_va(307746);
            // 9d235 lea rbx,[rbp-0x1180]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ee80);
            // 9d23c mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9d241 mov rdi,rbx
            rdi = rbx;
            // 9d244 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d249 mov esi,0x2850
            rsi = (0x2850u64) as u32 as u64;
            // 9d24e mov rdi,r15
            rdi = r15;
            // 9d251 mov rdx,rbx
            rdx = rbx;
            // 9d254 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d259 lea rdi,[rbp-0x1180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee80);
            // 9d260 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d265 lea rsi,[rip+0x997fc] # 0x136a68
            rsi = m.env.data_va(307765);
            // 9d26c lea rbx,[rbp-0x1198]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ee68);
            // 9d273 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9d278 mov rdi,rbx
            rdi = rbx;
            // 9d27b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d280 mov esi,0x2851
            rsi = (0x2851u64) as u32 as u64;
            // 9d285 mov rdi,r15
            rdi = r15;
            // 9d288 mov rdx,rbx
            rdx = rbx;
            // 9d28b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d290 lea rdi,[rbp-0x1198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee68);
            // 9d297 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d29c lea rsi,[rip+0x997d8] # 0x136a7b
            rsi = m.env.data_va(307784);
            // 9d2a3 lea rbx,[rbp-0x11b0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ee50);
            // 9d2aa mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9d2af mov rdi,rbx
            rdi = rbx;
            // 9d2b2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d2b7 mov esi,0x2852
            rsi = (0x2852u64) as u32 as u64;
            // 9d2bc mov rdi,r15
            rdi = r15;
            // 9d2bf mov rdx,rbx
            rdx = rbx;
            // 9d2c2 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d2c7 lea rdi,[rbp-0x11b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee50);
            // 9d2ce call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d2d3 lea rsi,[rip+0x997b4] # 0x136a8e
            rsi = m.env.data_va(307803);
            // 9d2da lea rbx,[rbp-0x11c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ee38);
            // 9d2e1 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9d2e6 mov rdi,rbx
            rdi = rbx;
            // 9d2e9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d2ee mov esi,0x2853
            rsi = (0x2853u64) as u32 as u64;
            // 9d2f3 mov rdi,r15
            rdi = r15;
            // 9d2f6 mov rdx,rbx
            rdx = rbx;
            // 9d2f9 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d2fe lea rdi,[rbp-0x11c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee38);
            // 9d305 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d30a lea rsi,[rip+0x99796] # 0x136aa7
            rsi = m.env.data_va(307828);
            // 9d311 lea rbx,[rbp-0x11e0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ee20);
            // 9d318 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d31d mov rdi,rbx
            rdi = rbx;
            // 9d320 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d325 mov esi,0x2854
            rsi = (0x2854u64) as u32 as u64;
            // 9d32a mov rdi,r15
            rdi = r15;
            // 9d32d mov rdx,rbx
            rdx = rbx;
            // 9d330 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d335 lea rdi,[rbp-0x11e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee20);
            // 9d33c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d341 lea rsi,[rip+0x99783] # 0x136acb
            rsi = m.env.data_va(307864);
            // 9d348 lea rbx,[rbp-0x11f8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ee08);
            // 9d34f mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9d354 mov rdi,rbx
            rdi = rbx;
            // 9d357 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d35c mov esi,0x2855
            rsi = (0x2855u64) as u32 as u64;
            // 9d361 mov rdi,r15
            rdi = r15;
            // 9d364 mov rdx,rbx
            rdx = rbx;
            // 9d367 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d36c lea rdi,[rbp-0x11f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee08);
            // 9d373 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d378 lea rsi,[rip+0x9975f] # 0x136ade
            rsi = m.env.data_va(307883);
            // 9d37f lea rbx,[rbp-0x1210]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_edf0);
            // 9d386 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9d38b mov rdi,rbx
            rdi = rbx;
            // 9d38e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d393 mov esi,0x2856
            rsi = (0x2856u64) as u32 as u64;
            // 9d398 mov rdi,r15
            rdi = r15;
            // 9d39b mov rdx,rbx
            rdx = rbx;
            // 9d39e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d3a3 lea rdi,[rbp-0x1210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edf0);
            // 9d3aa call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d3af lea rsi,[rip+0x99748] # 0x136afe
            rsi = m.env.data_va(307915);
            // 9d3b6 lea rbx,[rbp-0x1228]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_edd8);
            // 9d3bd mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9d3c2 mov rdi,rbx
            rdi = rbx;
            // 9d3c5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d3ca mov esi,0x2857
            rsi = (0x2857u64) as u32 as u64;
            // 9d3cf mov rdi,r15
            rdi = r15;
            // 9d3d2 mov rdx,rbx
            rdx = rbx;
            // 9d3d5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d3da lea rdi,[rbp-0x1228]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edd8);
            // 9d3e1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d3e6 lea rsi,[rip+0x99731] # 0x136b1e
            rsi = m.env.data_va(307947);
            // 9d3ed lea rbx,[rbp-0x1240]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_edc0);
            // 9d3f4 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9d3f9 mov rdi,rbx
            rdi = rbx;
            // 9d3fc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d401 mov esi,0x2864
            rsi = (0x2864u64) as u32 as u64;
            // 9d406 mov rdi,r15
            rdi = r15;
            // 9d409 mov rdx,rbx
            rdx = rbx;
            // 9d40c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d411 lea rdi,[rbp-0x1240]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edc0);
            // 9d418 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d41d lea rsi,[rip+0x99710] # 0x136b34
            rsi = m.env.data_va(307969);
            // 9d424 lea rbx,[rbp-0x1258]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eda8);
            // 9d42b mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9d430 mov rdi,rbx
            rdi = rbx;
            // 9d433 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d438 mov esi,0x2865
            rsi = (0x2865u64) as u32 as u64;
            // 9d43d mov rdi,r15
            rdi = r15;
            // 9d440 mov rdx,rbx
            rdx = rbx;
            // 9d443 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d448 lea rdi,[rbp-0x1258]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eda8);
            // 9d44f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d454 lea rsi,[rip+0x996fc] # 0x136b57
            rsi = m.env.data_va(308004);
            // 9d45b lea rbx,[rbp-0x1270]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ed90);
            // 9d462 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9d467 mov rdi,rbx
            rdi = rbx;
            // 9d46a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d46f mov esi,0x2866
            rsi = (0x2866u64) as u32 as u64;
            // 9d474 mov rdi,r15
            rdi = r15;
            // 9d477 mov rdx,rbx
            rdx = rbx;
            // 9d47a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d47f lea rdi,[rbp-0x1270]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed90);
            // 9d486 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d48b lea rsi,[rip+0x996e8] # 0x136b7a
            rsi = m.env.data_va(308039);
            // 9d492 lea rbx,[rbp-0x1288]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ed78);
            // 9d499 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9d49e mov rdi,rbx
            rdi = rbx;
            // 9d4a1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d4a6 mov esi,0x2867
            rsi = (0x2867u64) as u32 as u64;
            // 9d4ab mov rdi,r15
            rdi = r15;
            // 9d4ae mov rdx,rbx
            rdx = rbx;
            // 9d4b1 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d4b6 lea rdi,[rbp-0x1288]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed78);
            // 9d4bd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d4c2 lea rsi,[rip+0x996d4] # 0x136b9d
            rsi = m.env.data_va(308074);
            // 9d4c9 lea rbx,[rbp-0x12a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ed60);
            // 9d4d0 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9d4d5 mov rdi,rbx
            rdi = rbx;
            // 9d4d8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d4dd mov esi,0x2868
            rsi = (0x2868u64) as u32 as u64;
            // 9d4e2 mov rdi,r15
            rdi = r15;
            // 9d4e5 mov rdx,rbx
            rdx = rbx;
            // 9d4e8 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d4ed lea rdi,[rbp-0x12a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed60);
            // 9d4f4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d4f9 lea rsi,[rip+0x996c0] # 0x136bc0
            rsi = m.env.data_va(308109);
            // 9d500 lea rbx,[rbp-0x12b8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ed48);
            // 9d507 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9d50c mov rdi,rbx
            rdi = rbx;
            // 9d50f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d514 mov esi,0x2869
            rsi = (0x2869u64) as u32 as u64;
            // 9d519 mov rdi,r15
            rdi = r15;
            // 9d51c mov rdx,rbx
            rdx = rbx;
            // 9d51f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d524 lea rdi,[rbp-0x12b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed48);
            // 9d52b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d530 lea rsi,[rip+0x996ac] # 0x136be3
            rsi = m.env.data_va(308144);
            // 9d537 lea rbx,[rbp-0x12d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ed30);
            // 9d53e mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9d543 mov rdi,rbx
            rdi = rbx;
            // 9d546 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d54b mov esi,0x286a
            rsi = (0x286au64) as u32 as u64;
            // 9d550 mov rdi,r15
            rdi = r15;
            // 9d553 mov rdx,rbx
            rdx = rbx;
            // 9d556 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d55b lea rdi,[rbp-0x12d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed30);
            // 9d562 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d567 lea rsi,[rip+0x99698] # 0x136c06
            rsi = m.env.data_va(308179);
            // 9d56e lea rbx,[rbp-0x12e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ed18);
            // 9d575 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9d57a mov rdi,rbx
            rdi = rbx;
            // 9d57d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d582 mov esi,0x286b
            rsi = (0x286bu64) as u32 as u64;
            // 9d587 mov rdi,r15
            rdi = r15;
            // 9d58a mov rdx,rbx
            rdx = rbx;
            // 9d58d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d592 lea rdi,[rbp-0x12e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed18);
            // 9d599 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d59e lea rsi,[rip+0x99684] # 0x136c29
            rsi = m.env.data_va(308214);
            // 9d5a5 lea rbx,[rbp-0x1300]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ed00);
            // 9d5ac mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9d5b1 mov rdi,rbx
            rdi = rbx;
            // 9d5b4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d5b9 mov esi,0x286c
            rsi = (0x286cu64) as u32 as u64;
            // 9d5be mov rdi,r15
            rdi = r15;
            // 9d5c1 mov rdx,rbx
            rdx = rbx;
            // 9d5c4 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d5c9 lea rdi,[rbp-0x1300]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed00);
            // 9d5d0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d5d5 lea rsi,[rip+0x99670] # 0x136c4c
            rsi = m.env.data_va(308249);
            // 9d5dc lea rbx,[rbp-0x1318]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ece8);
            // 9d5e3 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9d5e8 mov rdi,rbx
            rdi = rbx;
            // 9d5eb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d5f0 mov esi,0x286d
            rsi = (0x286du64) as u32 as u64;
            // 9d5f5 mov rdi,r15
            rdi = r15;
            // 9d5f8 mov rdx,rbx
            rdx = rbx;
            // 9d5fb call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d600 lea rdi,[rbp-0x1318]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ece8);
            // 9d607 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d60c lea rsi,[rip+0x9965c] # 0x136c6f
            rsi = m.env.data_va(308284);
            // 9d613 lea rbx,[rbp-0x1330]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ecd0);
            // 9d61a mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9d61f mov rdi,rbx
            rdi = rbx;
            // 9d622 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d627 mov esi,0x286e
            rsi = (0x286eu64) as u32 as u64;
            // 9d62c mov rdi,r15
            rdi = r15;
            // 9d62f mov rdx,rbx
            rdx = rbx;
            // 9d632 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d637 lea rdi,[rbp-0x1330]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecd0);
            // 9d63e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d643 lea rsi,[rip+0x99648] # 0x136c92
            rsi = m.env.data_va(308319);
            // 9d64a lea rbx,[rbp-0x1348]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ecb8);
            // 9d651 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9d656 mov rdi,rbx
            rdi = rbx;
            // 9d659 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d65e mov esi,0x286f
            rsi = (0x286fu64) as u32 as u64;
            // 9d663 mov rdi,r15
            rdi = r15;
            // 9d666 mov rdx,rbx
            rdx = rbx;
            // 9d669 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d66e lea rdi,[rbp-0x1348]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecb8);
            // 9d675 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d67a lea rsi,[rip+0x99634] # 0x136cb5
            rsi = m.env.data_va(308354);
            // 9d681 lea rbx,[rbp-0x1360]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eca0);
            // 9d688 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9d68d mov rdi,rbx
            rdi = rbx;
            // 9d690 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d695 mov esi,0x2870
            rsi = (0x2870u64) as u32 as u64;
            // 9d69a mov rdi,r15
            rdi = r15;
            // 9d69d mov rdx,rbx
            rdx = rbx;
            // 9d6a0 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d6a5 lea rdi,[rbp-0x1360]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eca0);
            // 9d6ac call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d6b1 lea rsi,[rip+0x99620] # 0x136cd8
            rsi = m.env.data_va(308389);
            // 9d6b8 lea rbx,[rbp-0x1378]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ec88);
            // 9d6bf mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d6c4 mov rdi,rbx
            rdi = rbx;
            // 9d6c7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d6cc mov esi,0x2871
            rsi = (0x2871u64) as u32 as u64;
            // 9d6d1 mov rdi,r15
            rdi = r15;
            // 9d6d4 mov rdx,rbx
            rdx = rbx;
            // 9d6d7 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d6dc lea rdi,[rbp-0x1378]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec88);
            // 9d6e3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d6e8 lea rsi,[rip+0x9960d] # 0x136cfc
            rsi = m.env.data_va(308425);
            // 9d6ef lea rbx,[rbp-0x1390]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ec70);
            // 9d6f6 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d6fb mov rdi,rbx
            rdi = rbx;
            // 9d6fe call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d703 mov esi,0x2872
            rsi = (0x2872u64) as u32 as u64;
            // 9d708 mov rdi,r15
            rdi = r15;
            // 9d70b mov rdx,rbx
            rdx = rbx;
            // 9d70e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d713 lea rdi,[rbp-0x1390]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec70);
            // 9d71a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d71f lea rsi,[rip+0x995fa] # 0x136d20
            rsi = m.env.data_va(308461);
            // 9d726 lea rbx,[rbp-0x13a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ec58);
            // 9d72d mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d732 mov rdi,rbx
            rdi = rbx;
            // 9d735 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d73a mov esi,0x2873
            rsi = (0x2873u64) as u32 as u64;
            // 9d73f mov rdi,r15
            rdi = r15;
            // 9d742 mov rdx,rbx
            rdx = rbx;
            // 9d745 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d74a lea rdi,[rbp-0x13a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec58);
            // 9d751 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d756 lea rsi,[rip+0x995e7] # 0x136d44
            rsi = m.env.data_va(308497);
            // 9d75d lea rbx,[rbp-0x13c0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ec40);
            // 9d764 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d769 mov rdi,rbx
            rdi = rbx;
            // 9d76c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d771 mov esi,0x2874
            rsi = (0x2874u64) as u32 as u64;
            // 9d776 mov rdi,r15
            rdi = r15;
            // 9d779 mov rdx,rbx
            rdx = rbx;
            // 9d77c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d781 lea rdi,[rbp-0x13c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec40);
            // 9d788 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d78d lea rsi,[rip+0x995d4] # 0x136d68
            rsi = m.env.data_va(308533);
            // 9d794 lea rbx,[rbp-0x13d8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ec28);
            // 9d79b mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d7a0 mov rdi,rbx
            rdi = rbx;
            // 9d7a3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d7a8 mov esi,0x2875
            rsi = (0x2875u64) as u32 as u64;
            // 9d7ad mov rdi,r15
            rdi = r15;
            // 9d7b0 mov rdx,rbx
            rdx = rbx;
            // 9d7b3 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d7b8 lea rdi,[rbp-0x13d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec28);
            // 9d7bf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d7c4 lea rsi,[rip+0x995c1] # 0x136d8c
            rsi = m.env.data_va(308569);
            // 9d7cb lea rbx,[rbp-0x13f0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ec10);
            // 9d7d2 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d7d7 mov rdi,rbx
            rdi = rbx;
            // 9d7da call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d7df mov esi,0x2876
            rsi = (0x2876u64) as u32 as u64;
            // 9d7e4 mov rdi,r15
            rdi = r15;
            // 9d7e7 mov rdx,rbx
            rdx = rbx;
            // 9d7ea call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d7ef lea rdi,[rbp-0x13f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec10);
            // 9d7f6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d7fb lea rsi,[rip+0x995ae] # 0x136db0
            rsi = m.env.data_va(308605);
            // 9d802 lea rbx,[rbp-0x1408]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ebf8);
            // 9d809 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d80e mov rdi,rbx
            rdi = rbx;
            // 9d811 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d816 mov esi,0x2877
            rsi = (0x2877u64) as u32 as u64;
            // 9d81b mov rdi,r15
            rdi = r15;
            // 9d81e mov rdx,rbx
            rdx = rbx;
            // 9d821 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d826 lea rdi,[rbp-0x1408]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebf8);
            // 9d82d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d832 lea rsi,[rip+0x9959b] # 0x136dd4
            rsi = m.env.data_va(308641);
            // 9d839 lea rbx,[rbp-0x1420]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ebe0);
            // 9d840 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d845 mov rdi,rbx
            rdi = rbx;
            // 9d848 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d84d mov esi,0x2878
            rsi = (0x2878u64) as u32 as u64;
            // 9d852 mov rdi,r15
            rdi = r15;
            // 9d855 mov rdx,rbx
            rdx = rbx;
            // 9d858 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d85d lea rdi,[rbp-0x1420]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebe0);
            // 9d864 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d869 lea rsi,[rip+0x99588] # 0x136df8
            rsi = m.env.data_va(308677);
            // 9d870 lea rbx,[rbp-0x1438]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ebc8);
            // 9d877 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d87c mov rdi,rbx
            rdi = rbx;
            // 9d87f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d884 mov esi,0x2879
            rsi = (0x2879u64) as u32 as u64;
            // 9d889 mov rdi,r15
            rdi = r15;
            // 9d88c mov rdx,rbx
            rdx = rbx;
            // 9d88f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d894 lea rdi,[rbp-0x1438]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebc8);
            // 9d89b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d8a0 lea rsi,[rip+0x99575] # 0x136e1c
            rsi = m.env.data_va(308713);
            // 9d8a7 lea rbx,[rbp-0x1450]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ebb0);
            // 9d8ae mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d8b3 mov rdi,rbx
            rdi = rbx;
            // 9d8b6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d8bb mov esi,0x287a
            rsi = (0x287au64) as u32 as u64;
            // 9d8c0 mov rdi,r15
            rdi = r15;
            // 9d8c3 mov rdx,rbx
            rdx = rbx;
            // 9d8c6 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d8cb lea rdi,[rbp-0x1450]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebb0);
            // 9d8d2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d8d7 lea rsi,[rip+0x99562] # 0x136e40
            rsi = m.env.data_va(308749);
            // 9d8de lea rbx,[rbp-0x1468]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eb98);
            // 9d8e5 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d8ea mov rdi,rbx
            rdi = rbx;
            // 9d8ed call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d8f2 mov esi,0x287b
            rsi = (0x287bu64) as u32 as u64;
            // 9d8f7 mov rdi,r15
            rdi = r15;
            // 9d8fa mov rdx,rbx
            rdx = rbx;
            // 9d8fd call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d902 lea rdi,[rbp-0x1468]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb98);
            // 9d909 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d90e lea rsi,[rip+0x9954f] # 0x136e64
            rsi = m.env.data_va(308785);
            // 9d915 lea rbx,[rbp-0x1480]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eb80);
            // 9d91c mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d921 mov rdi,rbx
            rdi = rbx;
            // 9d924 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d929 mov esi,0x287c
            rsi = (0x287cu64) as u32 as u64;
            // 9d92e mov rdi,r15
            rdi = r15;
            // 9d931 mov rdx,rbx
            rdx = rbx;
            // 9d934 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d939 lea rdi,[rbp-0x1480]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb80);
            // 9d940 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d945 lea rsi,[rip+0x9953c] # 0x136e88
            rsi = m.env.data_va(308821);
            // 9d94c lea rbx,[rbp-0x1498]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eb68);
            // 9d953 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9d958 mov rdi,rbx
            rdi = rbx;
            // 9d95b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d960 mov esi,0x287d
            rsi = (0x287du64) as u32 as u64;
            // 9d965 mov rdi,r15
            rdi = r15;
            // 9d968 mov rdx,rbx
            rdx = rbx;
            // 9d96b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d970 lea rdi,[rbp-0x1498]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb68);
            // 9d977 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d97c lea rsi,[rip+0x99529] # 0x136eac
            rsi = m.env.data_va(308857);
            // 9d983 lea rbx,[rbp-0x14b0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eb50);
            // 9d98a mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9d98f mov rdi,rbx
            rdi = rbx;
            // 9d992 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d997 mov esi,0x287e
            rsi = (0x287eu64) as u32 as u64;
            // 9d99c mov rdi,r15
            rdi = r15;
            // 9d99f mov rdx,rbx
            rdx = rbx;
            // 9d9a2 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d9a7 lea rdi,[rbp-0x14b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb50);
            // 9d9ae call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d9b3 lea rsi,[rip+0x99507] # 0x136ec1
            rsi = m.env.data_va(308878);
            // 9d9ba lea rbx,[rbp-0x14c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eb38);
            // 9d9c1 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9d9c6 mov rdi,rbx
            rdi = rbx;
            // 9d9c9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d9ce mov esi,0x287f
            rsi = (0x287fu64) as u32 as u64;
            // 9d9d3 mov rdi,r15
            rdi = r15;
            // 9d9d6 mov rdx,rbx
            rdx = rbx;
            // 9d9d9 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d9de lea rdi,[rbp-0x14c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb38);
            // 9d9e5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9d9ea lea rsi,[rip+0x994e4] # 0x136ed5
            rsi = m.env.data_va(308898);
            // 9d9f1 lea rbx,[rbp-0x14e0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eb20);
            // 9d9f8 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 9d9fd mov rdi,rbx
            rdi = rbx;
            // 9da00 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9da05 mov esi,0x2880
            rsi = (0x2880u64) as u32 as u64;
            // 9da0a mov rdi,r15
            rdi = r15;
            // 9da0d mov rdx,rbx
            rdx = rbx;
            // 9da10 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9da15 lea rdi,[rbp-0x14e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb20);
            // 9da1c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9da21 lea rsi,[rip+0x994bc] # 0x136ee4
            rsi = m.env.data_va(308913);
            // 9da28 lea rbx,[rbp-0x14f8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eb08);
            // 9da2f mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 9da34 mov rdi,rbx
            rdi = rbx;
            // 9da37 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9da3c mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // 9da41 mov rdi,r15
            rdi = r15;
            // 9da44 mov rdx,rbx
            rdx = rbx;
            // 9da47 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9da4c lea rdi,[rbp-0x14f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb08);
            // 9da53 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9da58 lea rsi,[rip+0x99496] # 0x136ef5
            rsi = m.env.data_va(308930);
            // 9da5f lea rbx,[rbp-0x1510]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eaf0);
            // 9da66 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9da6b mov rdi,rbx
            rdi = rbx;
            // 9da6e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9da73 mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // 9da78 mov rdi,r15
            rdi = r15;
            // 9da7b mov rdx,rbx
            rdx = rbx;
            // 9da7e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9da83 lea rdi,[rbp-0x1510]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eaf0);
            // 9da8a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9da8f lea rsi,[rip+0x9946f] # 0x136f05
            rsi = m.env.data_va(308946);
            // 9da96 lea rbx,[rbp-0x1528]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ead8);
            // 9da9d mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 9daa2 mov rdi,rbx
            rdi = rbx;
            // 9daa5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9daaa mov esi,0x2883
            rsi = (0x2883u64) as u32 as u64;
            // 9daaf mov rdi,r15
            rdi = r15;
            // 9dab2 mov rdx,rbx
            rdx = rbx;
            // 9dab5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9daba lea rdi,[rbp-0x1528]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ead8);
            // 9dac1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dac6 lea rsi,[rip+0x99449] # 0x136f16
            rsi = m.env.data_va(308963);
            // 9dacd lea rbx,[rbp-0x1540]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eac0);
            // 9dad4 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9dad9 mov rdi,rbx
            rdi = rbx;
            // 9dadc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dae1 mov esi,0x2884
            rsi = (0x2884u64) as u32 as u64;
            // 9dae6 mov rdi,r15
            rdi = r15;
            // 9dae9 mov rdx,rbx
            rdx = rbx;
            // 9daec call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9daf1 lea rdi,[rbp-0x1540]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eac0);
            // 9daf8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dafd lea rsi,[rip+0x99426] # 0x136f2a
            rsi = m.env.data_va(308983);
            // 9db04 lea rbx,[rbp-0x1558]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_eaa8);
            // 9db0b mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9db10 mov rdi,rbx
            rdi = rbx;
            // 9db13 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9db18 mov esi,0x2885
            rsi = (0x2885u64) as u32 as u64;
            // 9db1d mov rdi,r15
            rdi = r15;
            // 9db20 mov rdx,rbx
            rdx = rbx;
            // 9db23 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9db28 lea rdi,[rbp-0x1558]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eaa8);
            // 9db2f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9db34 lea rsi,[rip+0x99405] # 0x136f40
            rsi = m.env.data_va(309005);
            // 9db3b lea rbx,[rbp-0x1570]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ea90);
            // 9db42 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9db47 mov rdi,rbx
            rdi = rbx;
            // 9db4a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9db4f mov esi,0x2886
            rsi = (0x2886u64) as u32 as u64;
            // 9db54 mov rdi,r15
            rdi = r15;
            // 9db57 mov rdx,rbx
            rdx = rbx;
            // 9db5a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9db5f lea rdi,[rbp-0x1570]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea90);
            // 9db66 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9db6b lea rsi,[rip+0x993e6] # 0x136f58
            rsi = m.env.data_va(309029);
            // 9db72 lea rbx,[rbp-0x1588]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ea78);
            // 9db79 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9db7e mov rdi,rbx
            rdi = rbx;
            // 9db81 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9db86 mov esi,0x2887
            rsi = (0x2887u64) as u32 as u64;
            // 9db8b mov rdi,r15
            rdi = r15;
            // 9db8e mov rdx,rbx
            rdx = rbx;
            // 9db91 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9db96 lea rdi,[rbp-0x1588]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea78);
            // 9db9d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dba2 lea rsi,[rip+0x993cc] # 0x136f75
            rsi = m.env.data_va(309058);
            // 9dba9 lea rbx,[rbp-0x15a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ea60);
            // 9dbb0 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 9dbb5 mov rdi,rbx
            rdi = rbx;
            // 9dbb8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dbbd mov esi,0x2888
            rsi = (0x2888u64) as u32 as u64;
            // 9dbc2 mov rdi,r15
            rdi = r15;
            // 9dbc5 mov rdx,rbx
            rdx = rbx;
            // 9dbc8 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dbcd lea rdi,[rbp-0x15a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea60);
            // 9dbd4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dbd9 lea rsi,[rip+0x993b9] # 0x136f99
            rsi = m.env.data_va(309094);
            // 9dbe0 lea rbx,[rbp-0x15b8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ea48);
            // 9dbe7 mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9dbec mov rdi,rbx
            rdi = rbx;
            // 9dbef call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dbf4 mov esi,0x2889
            rsi = (0x2889u64) as u32 as u64;
            // 9dbf9 mov rdi,r15
            rdi = r15;
            // 9dbfc mov rdx,rbx
            rdx = rbx;
            // 9dbff call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dc04 lea rdi,[rbp-0x15b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea48);
            // 9dc0b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dc10 lea rsi,[rip+0x99399] # 0x136fb0
            rsi = m.env.data_va(309117);
            // 9dc17 lea rbx,[rbp-0x15d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ea30);
            // 9dc1e mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9dc23 mov rdi,rbx
            rdi = rbx;
            // 9dc26 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dc2b mov esi,0x288a
            rsi = (0x288au64) as u32 as u64;
            // 9dc30 mov rdi,r15
            rdi = r15;
            // 9dc33 mov rdx,rbx
            rdx = rbx;
            // 9dc36 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dc3b lea rdi,[rbp-0x15d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea30);
            // 9dc42 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dc47 lea rsi,[rip+0x9937b] # 0x136fc9
            rsi = m.env.data_va(309142);
            // 9dc4e lea rbx,[rbp-0x15e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ea18);
            // 9dc55 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9dc5a mov rdi,rbx
            rdi = rbx;
            // 9dc5d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dc62 mov esi,0x288b
            rsi = (0x288bu64) as u32 as u64;
            // 9dc67 mov rdi,r15
            rdi = r15;
            // 9dc6a mov rdx,rbx
            rdx = rbx;
            // 9dc6d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dc72 lea rdi,[rbp-0x15e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea18);
            // 9dc79 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dc7e lea rsi,[rip+0x9935e] # 0x136fe3
            rsi = m.env.data_va(309168);
            // 9dc85 lea rbx,[rbp-0x1600]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ea00);
            // 9dc8c mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9dc91 mov rdi,rbx
            rdi = rbx;
            // 9dc94 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dc99 mov esi,0x288c
            rsi = (0x288cu64) as u32 as u64;
            // 9dc9e mov rdi,r15
            rdi = r15;
            // 9dca1 mov rdx,rbx
            rdx = rbx;
            // 9dca4 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dca9 lea rdi,[rbp-0x1600]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea00);
            // 9dcb0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dcb5 lea rsi,[rip+0x99343] # 0x136fff
            rsi = m.env.data_va(309196);
            // 9dcbc lea rbx,[rbp-0x1618]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e9e8);
            // 9dcc3 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9dcc8 mov rdi,rbx
            rdi = rbx;
            // 9dccb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dcd0 mov esi,0x2893
            rsi = (0x2893u64) as u32 as u64;
            // 9dcd5 mov rdi,r15
            rdi = r15;
            // 9dcd8 mov rdx,rbx
            rdx = rbx;
            // 9dcdb call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dce0 lea rdi,[rbp-0x1618]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9e8);
            // 9dce7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dcec lea rsi,[rip+0x99327] # 0x13701a
            rsi = m.env.data_va(309223);
            // 9dcf3 lea rbx,[rbp-0x1630]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e9d0);
            // 9dcfa mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9dcff mov rdi,rbx
            rdi = rbx;
            // 9dd02 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dd07 mov esi,0x2894
            rsi = (0x2894u64) as u32 as u64;
            // 9dd0c mov rdi,r15
            rdi = r15;
            // 9dd0f mov rdx,rbx
            rdx = rbx;
            // 9dd12 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dd17 lea rdi,[rbp-0x1630]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9d0);
            // 9dd1e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dd23 lea rsi,[rip+0x99308] # 0x137032
            rsi = m.env.data_va(309247);
            // 9dd2a lea rbx,[rbp-0x1648]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e9b8);
            // 9dd31 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9dd36 mov rdi,rbx
            rdi = rbx;
            // 9dd39 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dd3e mov esi,0x2895
            rsi = (0x2895u64) as u32 as u64;
            // 9dd43 mov rdi,r15
            rdi = r15;
            // 9dd46 mov rdx,rbx
            rdx = rbx;
            // 9dd49 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dd4e lea rdi,[rbp-0x1648]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9b8);
            // 9dd55 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dd5a lea rsi,[rip+0x992e9] # 0x13704a
            rsi = m.env.data_va(309271);
            // 9dd61 lea rbx,[rbp-0x1660]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e9a0);
            // 9dd68 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9dd6d mov rdi,rbx
            rdi = rbx;
            // 9dd70 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dd75 mov esi,0x2896
            rsi = (0x2896u64) as u32 as u64;
            // 9dd7a mov rdi,r15
            rdi = r15;
            // 9dd7d mov rdx,rbx
            rdx = rbx;
            // 9dd80 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dd85 lea rdi,[rbp-0x1660]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9a0);
            // 9dd8c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dd91 lea rsi,[rip+0x992ca] # 0x137062
            rsi = m.env.data_va(309295);
            // 9dd98 lea rbx,[rbp-0x1678]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e988);
            // 9dd9f mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9dda4 mov rdi,rbx
            rdi = rbx;
            // 9dda7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ddac mov esi,0x2897
            rsi = (0x2897u64) as u32 as u64;
            // 9ddb1 mov rdi,r15
            rdi = r15;
            // 9ddb4 mov rdx,rbx
            rdx = rbx;
            // 9ddb7 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ddbc lea rdi,[rbp-0x1678]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e988);
            // 9ddc3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ddc8 lea rsi,[rip+0x992aa] # 0x137079
            rsi = m.env.data_va(309318);
            // 9ddcf lea rbx,[rbp-0x1690]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e970);
            // 9ddd6 mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9dddb mov rdi,rbx
            rdi = rbx;
            // 9ddde call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dde3 mov esi,0x2898
            rsi = (0x2898u64) as u32 as u64;
            // 9dde8 mov rdi,r15
            rdi = r15;
            // 9ddeb mov rdx,rbx
            rdx = rbx;
            // 9ddee call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ddf3 lea rdi,[rbp-0x1690]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e970);
            // 9ddfa call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ddff lea rsi,[rip+0x99288] # 0x13708e
            rsi = m.env.data_va(309339);
            // 9de06 lea rbx,[rbp-0x16a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e958);
            // 9de0d mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9de12 mov rdi,rbx
            rdi = rbx;
            // 9de15 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9de1a mov esi,0x2899
            rsi = (0x2899u64) as u32 as u64;
            // 9de1f mov rdi,r15
            rdi = r15;
            // 9de22 mov rdx,rbx
            rdx = rbx;
            // 9de25 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9de2a lea rdi,[rbp-0x16a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e958);
            // 9de31 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9de36 lea rsi,[rip+0x99266] # 0x1370a3
            rsi = m.env.data_va(309360);
            // 9de3d lea rbx,[rbp-0x16c0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e940);
            // 9de44 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9de49 mov rdi,rbx
            rdi = rbx;
            // 9de4c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9de51 mov esi,0x289a
            rsi = (0x289au64) as u32 as u64;
            // 9de56 mov rdi,r15
            rdi = r15;
            // 9de59 mov rdx,rbx
            rdx = rbx;
            // 9de5c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9de61 lea rdi,[rbp-0x16c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e940);
            // 9de68 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9de6d lea rsi,[rip+0x9923f] # 0x1370b3
            rsi = m.env.data_va(309376);
            // 9de74 lea rbx,[rbp-0x16d8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e928);
            // 9de7b mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 9de80 mov rdi,rbx
            rdi = rbx;
            // 9de83 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9de88 mov esi,0x289b
            rsi = (0x289bu64) as u32 as u64;
            // 9de8d mov rdi,r15
            rdi = r15;
            // 9de90 mov rdx,rbx
            rdx = rbx;
            // 9de93 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9de98 lea rdi,[rbp-0x16d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e928);
            // 9de9f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dea4 lea rsi,[rip+0x99216] # 0x1370c1
            rsi = m.env.data_va(309390);
            // 9deab lea rbx,[rbp-0x16f0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e910);
            // 9deb2 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9deb7 mov rdi,rbx
            rdi = rbx;
            // 9deba call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9debf mov esi,0x289c
            rsi = (0x289cu64) as u32 as u64;
            // 9dec4 mov rdi,r15
            rdi = r15;
            // 9dec7 mov rdx,rbx
            rdx = rbx;
            // 9deca call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9decf lea rdi,[rbp-0x16f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e910);
            // 9ded6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dedb lea rsi,[rip+0x991ff] # 0x1370e1
            rsi = m.env.data_va(309422);
            // 9dee2 lea rbx,[rbp-0x1708]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e8f8);
            // 9dee9 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9deee mov rdi,rbx
            rdi = rbx;
            // 9def1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9def6 mov esi,0x289d
            rsi = (0x289du64) as u32 as u64;
            // 9defb mov rdi,r15
            rdi = r15;
            // 9defe mov rdx,rbx
            rdx = rbx;
            // 9df01 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9df06 lea rdi,[rbp-0x1708]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8f8);
            // 9df0d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9df12 lea rsi,[rip+0x991e2] # 0x1370fb
            rsi = m.env.data_va(309448);
            // 9df19 lea rbx,[rbp-0x1720]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e8e0);
            // 9df20 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9df25 mov rdi,rbx
            rdi = rbx;
            // 9df28 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9df2d mov esi,0x289e
            rsi = (0x289eu64) as u32 as u64;
            // 9df32 mov rdi,r15
            rdi = r15;
            // 9df35 mov rdx,rbx
            rdx = rbx;
            // 9df38 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9df3d lea rdi,[rbp-0x1720]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8e0);
            // 9df44 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9df49 lea rsi,[rip+0x991be] # 0x13710e
            rsi = m.env.data_va(309467);
            // 9df50 lea rbx,[rbp-0x1738]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e8c8);
            // 9df57 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9df5c mov rdi,rbx
            rdi = rbx;
            // 9df5f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9df64 mov esi,0x289f
            rsi = (0x289fu64) as u32 as u64;
            // 9df69 mov rdi,r15
            rdi = r15;
            // 9df6c mov rdx,rbx
            rdx = rbx;
            // 9df6f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9df74 lea rdi,[rbp-0x1738]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8c8);
            // 9df7b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9df80 lea rsi,[rip+0x991a0] # 0x137127
            rsi = m.env.data_va(309492);
            // 9df87 lea rbx,[rbp-0x1750]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e8b0);
            // 9df8e mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 9df93 mov rdi,rbx
            rdi = rbx;
            // 9df96 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9df9b mov esi,0x28a0
            rsi = (0x28a0u64) as u32 as u64;
            // 9dfa0 mov rdi,r15
            rdi = r15;
            // 9dfa3 mov rdx,rbx
            rdx = rbx;
            // 9dfa6 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dfab lea rdi,[rbp-0x1750]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8b0);
            // 9dfb2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dfb7 lea rsi,[rip+0x99177] # 0x137135
            rsi = m.env.data_va(309506);
            // 9dfbe lea rbx,[rbp-0x1768]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e898);
            // 9dfc5 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9dfca mov rdi,rbx
            rdi = rbx;
            // 9dfcd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dfd2 mov esi,0x28a1
            rsi = (0x28a1u64) as u32 as u64;
            // 9dfd7 mov rdi,r15
            rdi = r15;
            // 9dfda mov rdx,rbx
            rdx = rbx;
            // 9dfdd call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dfe2 lea rdi,[rbp-0x1768]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e898);
            // 9dfe9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9dfee lea rsi,[rip+0x99154] # 0x137149
            rsi = m.env.data_va(309526);
            // 9dff5 lea rbx,[rbp-0x1780]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e880);
            // 9dffc mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9e001 mov rdi,rbx
            rdi = rbx;
            // 9e004 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e009 mov esi,0x28a2
            rsi = (0x28a2u64) as u32 as u64;
            // 9e00e mov rdi,r15
            rdi = r15;
            // 9e011 mov rdx,rbx
            rdx = rbx;
            // 9e014 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e019 lea rdi,[rbp-0x1780]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e880);
            // 9e020 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e025 lea rsi,[rip+0x99131] # 0x13715d
            rsi = m.env.data_va(309546);
            // 9e02c lea rbx,[rbp-0x1798]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e868);
            // 9e033 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9e038 mov rdi,rbx
            rdi = rbx;
            // 9e03b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e040 mov esi,0x28a3
            rsi = (0x28a3u64) as u32 as u64;
            // 9e045 mov rdi,r15
            rdi = r15;
            // 9e048 mov rdx,rbx
            rdx = rbx;
            // 9e04b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e050 lea rdi,[rbp-0x1798]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e868);
            // 9e057 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e05c lea rsi,[rip+0x9910e] # 0x137171
            rsi = m.env.data_va(309566);
            // 9e063 lea rbx,[rbp-0x17b0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e850);
            // 9e06a mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9e06f mov rdi,rbx
            rdi = rbx;
            // 9e072 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e077 mov esi,0x28a4
            rsi = (0x28a4u64) as u32 as u64;
            // 9e07c mov rdi,r15
            rdi = r15;
            // 9e07f mov rdx,rbx
            rdx = rbx;
            // 9e082 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e087 lea rdi,[rbp-0x17b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e850);
            // 9e08e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e093 lea rsi,[rip+0x990eb] # 0x137185
            rsi = m.env.data_va(309586);
            // 9e09a lea rbx,[rbp-0x17c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e838);
            // 9e0a1 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9e0a6 mov rdi,rbx
            rdi = rbx;
            // 9e0a9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e0ae mov esi,0x28a5
            rsi = (0x28a5u64) as u32 as u64;
            // 9e0b3 mov rdi,r15
            rdi = r15;
            // 9e0b6 mov rdx,rbx
            rdx = rbx;
            // 9e0b9 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e0be lea rdi,[rbp-0x17c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e838);
            // 9e0c5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e0ca lea rsi,[rip+0x990c8] # 0x137199
            rsi = m.env.data_va(309606);
            // 9e0d1 lea rbx,[rbp-0x17e0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e820);
            // 9e0d8 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9e0dd mov rdi,rbx
            rdi = rbx;
            // 9e0e0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e0e5 mov esi,0x28a6
            rsi = (0x28a6u64) as u32 as u64;
            // 9e0ea mov rdi,r15
            rdi = r15;
            // 9e0ed mov rdx,rbx
            rdx = rbx;
            // 9e0f0 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e0f5 lea rdi,[rbp-0x17e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e820);
            // 9e0fc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e101 lea rsi,[rip+0x990a5] # 0x1371ad
            rsi = m.env.data_va(309626);
            // 9e108 lea rbx,[rbp-0x17f8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e808);
            // 9e10f mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9e114 mov rdi,rbx
            rdi = rbx;
            // 9e117 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e11c mov esi,0x28a7
            rsi = (0x28a7u64) as u32 as u64;
            // 9e121 mov rdi,r15
            rdi = r15;
            // 9e124 mov rdx,rbx
            rdx = rbx;
            // 9e127 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e12c lea rdi,[rbp-0x17f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e808);
            // 9e133 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e138 lea rsi,[rip+0x99082] # 0x1371c1
            rsi = m.env.data_va(309646);
            // 9e13f lea rbx,[rbp-0x1810]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e7f0);
            // 9e146 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9e14b mov rdi,rbx
            rdi = rbx;
            // 9e14e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e153 mov esi,0x28a8
            rsi = (0x28a8u64) as u32 as u64;
            // 9e158 mov rdi,r15
            rdi = r15;
            // 9e15b mov rdx,rbx
            rdx = rbx;
            // 9e15e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e163 lea rdi,[rbp-0x1810]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7f0);
            // 9e16a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e16f lea rsi,[rip+0x9905f] # 0x1371d5
            rsi = m.env.data_va(309666);
            // 9e176 lea rbx,[rbp-0x1828]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e7d8);
            // 9e17d mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9e182 mov rdi,rbx
            rdi = rbx;
            // 9e185 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e18a mov esi,0x28a9
            rsi = (0x28a9u64) as u32 as u64;
            // 9e18f mov rdi,r15
            rdi = r15;
            // 9e192 mov rdx,rbx
            rdx = rbx;
            // 9e195 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e19a lea rdi,[rbp-0x1828]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7d8);
            // 9e1a1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e1a6 lea rsi,[rip+0x99038] # 0x1371e5
            rsi = m.env.data_va(309682);
            // 9e1ad lea rbx,[rbp-0x1840]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e7c0);
            // 9e1b4 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9e1b9 mov rdi,rbx
            rdi = rbx;
            // 9e1bc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e1c1 mov esi,0x28aa
            rsi = (0x28aau64) as u32 as u64;
            // 9e1c6 mov rdi,r15
            rdi = r15;
            // 9e1c9 mov rdx,rbx
            rdx = rbx;
            // 9e1cc call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e1d1 lea rdi,[rbp-0x1840]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7c0);
            // 9e1d8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e1dd lea rsi,[rip+0x99011] # 0x1371f5
            rsi = m.env.data_va(309698);
            // 9e1e4 lea rbx,[rbp-0x1858]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e7a8);
            // 9e1eb mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 9e1f0 mov rdi,rbx
            rdi = rbx;
            // 9e1f3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e1f8 mov esi,0x28ab
            rsi = (0x28abu64) as u32 as u64;
            // 9e1fd mov rdi,r15
            rdi = r15;
            // 9e200 mov rdx,rbx
            rdx = rbx;
            // 9e203 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e208 lea rdi,[rbp-0x1858]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7a8);
            // 9e20f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e214 lea rsi,[rip+0x98feb] # 0x137206
            rsi = m.env.data_va(309715);
            // 9e21b lea rbx,[rbp-0x1870]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e790);
            // 9e222 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9e227 mov rdi,rbx
            rdi = rbx;
            // 9e22a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e22f mov esi,0x28ac
            rsi = (0x28acu64) as u32 as u64;
            // 9e234 mov rdi,r15
            rdi = r15;
            // 9e237 mov rdx,rbx
            rdx = rbx;
            // 9e23a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e23f lea rdi,[rbp-0x1870]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e790);
            // 9e246 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e24b lea rsi,[rip+0x98fc4] # 0x137216
            rsi = m.env.data_va(309731);
            // 9e252 lea rbx,[rbp-0x1888]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e778);
            // 9e259 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9e25e mov rdi,rbx
            rdi = rbx;
            // 9e261 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e266 mov esi,0x28ad
            rsi = (0x28adu64) as u32 as u64;
            // 9e26b mov rdi,r15
            rdi = r15;
            // 9e26e mov rdx,rbx
            rdx = rbx;
            // 9e271 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e276 lea rdi,[rbp-0x1888]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e778);
            // 9e27d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e282 lea rsi,[rip+0x98f9d] # 0x137226
            rsi = m.env.data_va(309747);
            // 9e289 lea rbx,[rbp-0x18a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e760);
            // 9e290 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 9e295 mov rdi,rbx
            rdi = rbx;
            // 9e298 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e29d mov esi,0x28ae
            rsi = (0x28aeu64) as u32 as u64;
            // 9e2a2 mov rdi,r15
            rdi = r15;
            // 9e2a5 mov rdx,rbx
            rdx = rbx;
            // 9e2a8 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e2ad lea rdi,[rbp-0x18a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e760);
            // 9e2b4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e2b9 lea rsi,[rip+0x98f77] # 0x137237
            rsi = m.env.data_va(309764);
            // 9e2c0 lea rbx,[rbp-0x18b8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e748);
            // 9e2c7 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9e2cc mov rdi,rbx
            rdi = rbx;
            // 9e2cf call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e2d4 mov esi,0x28af
            rsi = (0x28afu64) as u32 as u64;
            // 9e2d9 mov rdi,r15
            rdi = r15;
            // 9e2dc mov rdx,rbx
            rdx = rbx;
            // 9e2df call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e2e4 lea rdi,[rbp-0x18b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e748);
            // 9e2eb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e2f0 lea rsi,[rip+0x98f50] # 0x137247
            rsi = m.env.data_va(309780);
            // 9e2f7 lea rbx,[rbp-0x18d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e730);
            // 9e2fe mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9e303 mov rdi,rbx
            rdi = rbx;
            // 9e306 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e30b mov esi,0x28b0
            rsi = (0x28b0u64) as u32 as u64;
            // 9e310 mov rdi,r15
            rdi = r15;
            // 9e313 mov rdx,rbx
            rdx = rbx;
            // 9e316 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e31b lea rdi,[rbp-0x18d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e730);
            // 9e322 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e327 lea rsi,[rip+0x98f31] # 0x13725f
            rsi = m.env.data_va(309804);
            // 9e32e lea rbx,[rbp-0x18e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e718);
            // 9e335 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9e33a mov rdi,rbx
            rdi = rbx;
            // 9e33d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e342 mov esi,0x28b1
            rsi = (0x28b1u64) as u32 as u64;
            // 9e347 mov rdi,r15
            rdi = r15;
            // 9e34a mov rdx,rbx
            rdx = rbx;
            // 9e34d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e352 lea rdi,[rbp-0x18e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e718);
            // 9e359 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e35e lea rsi,[rip+0x98f13] # 0x137278
            rsi = m.env.data_va(309829);
            // 9e365 lea rbx,[rbp-0x1900]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e700);
            // 9e36c mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9e371 mov rdi,rbx
            rdi = rbx;
            // 9e374 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e379 mov esi,0x28b2
            rsi = (0x28b2u64) as u32 as u64;
            // 9e37e mov rdi,r15
            rdi = r15;
            // 9e381 mov rdx,rbx
            rdx = rbx;
            // 9e384 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e389 lea rdi,[rbp-0x1900]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e700);
            // 9e390 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e395 lea rsi,[rip+0x98ef3] # 0x13728f
            rsi = m.env.data_va(309852);
            // 9e39c lea rbx,[rbp-0x1918]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e6e8);
            // 9e3a3 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9e3a8 mov rdi,rbx
            rdi = rbx;
            // 9e3ab call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e3b0 mov esi,0x28b3
            rsi = (0x28b3u64) as u32 as u64;
            // 9e3b5 mov rdi,r15
            rdi = r15;
            // 9e3b8 mov rdx,rbx
            rdx = rbx;
            // 9e3bb call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e3c0 lea rdi,[rbp-0x1918]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6e8);
            // 9e3c7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e3cc lea rsi,[rip+0x98ed7] # 0x1372aa
            rsi = m.env.data_va(309879);
            // 9e3d3 lea rbx,[rbp-0x1930]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e6d0);
            // 9e3da mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9e3df mov rdi,rbx
            rdi = rbx;
            // 9e3e2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e3e7 mov esi,0x28b4
            rsi = (0x28b4u64) as u32 as u64;
            // 9e3ec mov rdi,r15
            rdi = r15;
            // 9e3ef mov rdx,rbx
            rdx = rbx;
            // 9e3f2 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e3f7 lea rdi,[rbp-0x1930]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6d0);
            // 9e3fe call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e403 lea rsi,[rip+0x98eb9] # 0x1372c3
            rsi = m.env.data_va(309904);
            // 9e40a lea rbx,[rbp-0x1948]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e6b8);
            // 9e411 mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9e416 mov rdi,rbx
            rdi = rbx;
            // 9e419 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e41e mov esi,0x28b5
            rsi = (0x28b5u64) as u32 as u64;
            // 9e423 mov rdi,r15
            rdi = r15;
            // 9e426 mov rdx,rbx
            rdx = rbx;
            // 9e429 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e42e lea rdi,[rbp-0x1948]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6b8);
            // 9e435 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e43a lea rsi,[rip+0x98e97] # 0x1372d8
            rsi = m.env.data_va(309925);
            // 9e441 lea rbx,[rbp-0x1960]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e6a0);
            // 9e448 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9e44d mov rdi,rbx
            rdi = rbx;
            // 9e450 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e455 mov esi,0x28b6
            rsi = (0x28b6u64) as u32 as u64;
            // 9e45a mov rdi,r15
            rdi = r15;
            // 9e45d mov rdx,rbx
            rdx = rbx;
            // 9e460 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e465 lea rdi,[rbp-0x1960]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6a0);
            // 9e46c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e471 lea rsi,[rip+0x98e7a] # 0x1372f2
            rsi = m.env.data_va(309951);
            // 9e478 lea rbx,[rbp-0x1978]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e688);
            // 9e47f mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9e484 mov rdi,rbx
            rdi = rbx;
            // 9e487 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e48c mov esi,0x28b7
            rsi = (0x28b7u64) as u32 as u64;
            // 9e491 mov rdi,r15
            rdi = r15;
            // 9e494 mov rdx,rbx
            rdx = rbx;
            // 9e497 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e49c lea rdi,[rbp-0x1978]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e688);
            // 9e4a3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e4a8 lea rsi,[rip+0x98e5e] # 0x13730d
            rsi = m.env.data_va(309978);
            // 9e4af lea rbx,[rbp-0x1990]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e670);
            // 9e4b6 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9e4bb mov rdi,rbx
            rdi = rbx;
            // 9e4be call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e4c3 mov esi,0x28b8
            rsi = (0x28b8u64) as u32 as u64;
            // 9e4c8 mov rdi,r15
            rdi = r15;
            // 9e4cb mov rdx,rbx
            rdx = rbx;
            // 9e4ce call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e4d3 lea rdi,[rbp-0x1990]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e670);
            // 9e4da call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e4df lea rsi,[rip+0x98e40] # 0x137326
            rsi = m.env.data_va(310003);
            // 9e4e6 lea rbx,[rbp-0x19a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e658);
            // 9e4ed mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9e4f2 mov rdi,rbx
            rdi = rbx;
            // 9e4f5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e4fa mov esi,0x28b9
            rsi = (0x28b9u64) as u32 as u64;
            // 9e4ff mov rdi,r15
            rdi = r15;
            // 9e502 mov rdx,rbx
            rdx = rbx;
            // 9e505 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e50a lea rdi,[rbp-0x19a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e658);
            // 9e511 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e516 lea rsi,[rip+0x98e26] # 0x137343
            rsi = m.env.data_va(310032);
            // 9e51d lea rbx,[rbp-0x19c0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e640);
            // 9e524 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9e529 mov rdi,rbx
            rdi = rbx;
            // 9e52c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e531 mov esi,0x28ba
            rsi = (0x28bau64) as u32 as u64;
            // 9e536 mov rdi,r15
            rdi = r15;
            // 9e539 mov rdx,rbx
            rdx = rbx;
            // 9e53c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e541 lea rdi,[rbp-0x19c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e640);
            // 9e548 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e54d lea rsi,[rip+0x98e0a] # 0x13735e
            rsi = m.env.data_va(310059);
            // 9e554 lea rbx,[rbp-0x19d8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e628);
            // 9e55b mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9e560 mov rdi,rbx
            rdi = rbx;
            // 9e563 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e568 mov esi,0x28bb
            rsi = (0x28bbu64) as u32 as u64;
            // 9e56d mov rdi,r15
            rdi = r15;
            // 9e570 mov rdx,rbx
            rdx = rbx;
            // 9e573 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e578 lea rdi,[rbp-0x19d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e628);
            // 9e57f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e584 lea rsi,[rip+0x98dea] # 0x137375
            rsi = m.env.data_va(310082);
            // 9e58b lea rbx,[rbp-0x19f0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e610);
            // 9e592 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 9e597 mov rdi,rbx
            rdi = rbx;
            // 9e59a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e59f mov esi,0x28bc
            rsi = (0x28bcu64) as u32 as u64;
            // 9e5a4 mov rdi,r15
            rdi = r15;
            // 9e5a7 mov rdx,rbx
            rdx = rbx;
            // 9e5aa call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e5af lea rdi,[rbp-0x19f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e610);
            // 9e5b6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e5bb lea rsi,[rip+0x98dc1] # 0x137383
            rsi = m.env.data_va(310096);
            // 9e5c2 lea rbx,[rbp-0x1a08]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e5f8);
            // 9e5c9 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9e5ce mov rdi,rbx
            rdi = rbx;
            // 9e5d1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e5d6 mov esi,0x28bd
            rsi = (0x28bdu64) as u32 as u64;
            // 9e5db mov rdi,r15
            rdi = r15;
            // 9e5de mov rdx,rbx
            rdx = rbx;
            // 9e5e1 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e5e6 lea rdi,[rbp-0x1a08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e5f8);
            // 9e5ed call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e5f2 lea rsi,[rip+0x98da0] # 0x137399
            rsi = m.env.data_va(310118);
            // 9e5f9 lea rbx,[rbp-0x1a20]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e5e0);
            // 9e600 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9e605 mov rdi,rbx
            rdi = rbx;
            // 9e608 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e60d mov esi,0x28be
            rsi = (0x28beu64) as u32 as u64;
            // 9e612 mov rdi,r15
            rdi = r15;
            // 9e615 mov rdx,rbx
            rdx = rbx;
            // 9e618 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e61d lea rdi,[rbp-0x1a20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e5e0);
            // 9e624 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e629 lea rsi,[rip+0x98d79] # 0x1373a9
            rsi = m.env.data_va(310134);
            // 9e630 lea rbx,[rbp-0x1a38]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e5c8);
            // 9e637 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9e63c mov rdi,rbx
            rdi = rbx;
            // 9e63f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e644 mov esi,0x28bf
            rsi = (0x28bfu64) as u32 as u64;
            // 9e649 mov rdi,r15
            rdi = r15;
            // 9e64c mov rdx,rbx
            rdx = rbx;
            // 9e64f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e654 lea rdi,[rbp-0x1a38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e5c8);
            // 9e65b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e660 lea rsi,[rip+0x98d5a] # 0x1373c1
            rsi = m.env.data_va(310158);
            // 9e667 lea rbx,[rbp-0x1a50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e5b0);
            // 9e66e mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9e673 mov rdi,rbx
            rdi = rbx;
            // 9e676 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e67b mov esi,0x28c0
            rsi = (0x28c0u64) as u32 as u64;
            // 9e680 mov rdi,r15
            rdi = r15;
            // 9e683 mov rdx,rbx
            rdx = rbx;
            // 9e686 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e68b lea rdi,[rbp-0x1a50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e5b0);
            // 9e692 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e697 lea rsi,[rip+0x98d3c] # 0x1373da
            rsi = m.env.data_va(310183);
            // 9e69e lea rbx,[rbp-0x1a68]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e598);
            // 9e6a5 mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9e6aa mov rdi,rbx
            rdi = rbx;
            // 9e6ad call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e6b2 mov esi,0x28c1
            rsi = (0x28c1u64) as u32 as u64;
            // 9e6b7 mov rdi,r15
            rdi = r15;
            // 9e6ba mov rdx,rbx
            rdx = rbx;
            // 9e6bd call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e6c2 lea rdi,[rbp-0x1a68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e598);
            // 9e6c9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e6ce lea rsi,[rip+0x98d1c] # 0x1373f1
            rsi = m.env.data_va(310206);
            // 9e6d5 lea rbx,[rbp-0x1a80]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e580);
            // 9e6dc mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9e6e1 mov rdi,rbx
            rdi = rbx;
            // 9e6e4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e6e9 mov esi,0x28c2
            rsi = (0x28c2u64) as u32 as u64;
            // 9e6ee mov rdi,r15
            rdi = r15;
            // 9e6f1 mov rdx,rbx
            rdx = rbx;
            // 9e6f4 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e6f9 lea rdi,[rbp-0x1a80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e580);
            // 9e700 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e705 lea rsi,[rip+0x98d00] # 0x13740c
            rsi = m.env.data_va(310233);
            // 9e70c lea rbx,[rbp-0x1a98]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e568);
            // 9e713 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9e718 mov rdi,rbx
            rdi = rbx;
            // 9e71b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e720 mov esi,0x28c3
            rsi = (0x28c3u64) as u32 as u64;
            // 9e725 mov rdi,r15
            rdi = r15;
            // 9e728 mov rdx,rbx
            rdx = rbx;
            // 9e72b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e730 lea rdi,[rbp-0x1a98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e568);
            // 9e737 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e73c lea rsi,[rip+0x98ce2] # 0x137425
            rsi = m.env.data_va(310258);
            // 9e743 lea rbx,[rbp-0x1ab0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e550);
            // 9e74a mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9e74f mov rdi,rbx
            rdi = rbx;
            // 9e752 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e757 mov esi,0x28c4
            rsi = (0x28c4u64) as u32 as u64;
            // 9e75c mov rdi,r15
            rdi = r15;
            // 9e75f mov rdx,rbx
            rdx = rbx;
            // 9e762 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e767 lea rdi,[rbp-0x1ab0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e550);
            // 9e76e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e773 lea rsi,[rip+0x98cc0] # 0x13743a
            rsi = m.env.data_va(310279);
            // 9e77a lea rbx,[rbp-0x1ac8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e538);
            // 9e781 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9e786 mov rdi,rbx
            rdi = rbx;
            // 9e789 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e78e mov esi,0x28c5
            rsi = (0x28c5u64) as u32 as u64;
            // 9e793 mov rdi,r15
            rdi = r15;
            // 9e796 mov rdx,rbx
            rdx = rbx;
            // 9e799 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e79e lea rdi,[rbp-0x1ac8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e538);
            // 9e7a5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e7aa lea rsi,[rip+0x98ca3] # 0x137454
            rsi = m.env.data_va(310305);
            // 9e7b1 lea rbx,[rbp-0x1ae0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e520);
            // 9e7b8 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9e7bd mov rdi,rbx
            rdi = rbx;
            // 9e7c0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e7c5 mov esi,0x28c6
            rsi = (0x28c6u64) as u32 as u64;
            // 9e7ca mov rdi,r15
            rdi = r15;
            // 9e7cd mov rdx,rbx
            rdx = rbx;
            // 9e7d0 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e7d5 lea rdi,[rbp-0x1ae0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e520);
            // 9e7dc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e7e1 lea rsi,[rip+0x98c87] # 0x13746f
            rsi = m.env.data_va(310332);
            // 9e7e8 lea rbx,[rbp-0x1af8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e508);
            // 9e7ef mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9e7f4 mov rdi,rbx
            rdi = rbx;
            // 9e7f7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e7fc mov esi,0x28c7
            rsi = (0x28c7u64) as u32 as u64;
            // 9e801 mov rdi,r15
            rdi = r15;
            // 9e804 mov rdx,rbx
            rdx = rbx;
            // 9e807 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e80c lea rdi,[rbp-0x1af8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e508);
            // 9e813 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e818 lea rsi,[rip+0x98c69] # 0x137488
            rsi = m.env.data_va(310357);
            // 9e81f lea rbx,[rbp-0x1b10]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e4f0);
            // 9e826 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9e82b mov rdi,rbx
            rdi = rbx;
            // 9e82e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e833 mov esi,0x28c8
            rsi = (0x28c8u64) as u32 as u64;
            // 9e838 mov rdi,r15
            rdi = r15;
            // 9e83b mov rdx,rbx
            rdx = rbx;
            // 9e83e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e843 lea rdi,[rbp-0x1b10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e4f0);
            // 9e84a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e84f lea rsi,[rip+0x98c4f] # 0x1374a5
            rsi = m.env.data_va(310386);
            // 9e856 lea rbx,[rbp-0x1b28]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e4d8);
            // 9e85d mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9e862 mov rdi,rbx
            rdi = rbx;
            // 9e865 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e86a mov esi,0x28c9
            rsi = (0x28c9u64) as u32 as u64;
            // 9e86f mov rdi,r15
            rdi = r15;
            // 9e872 mov rdx,rbx
            rdx = rbx;
            // 9e875 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e87a lea rdi,[rbp-0x1b28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e4d8);
            // 9e881 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e886 lea rsi,[rip+0x98c33] # 0x1374c0
            rsi = m.env.data_va(310413);
            // 9e88d lea rbx,[rbp-0x1b40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e4c0);
            // 9e894 mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 9e899 mov rdi,rbx
            rdi = rbx;
            // 9e89c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e8a1 mov esi,0x28ca
            rsi = (0x28cau64) as u32 as u64;
            // 9e8a6 mov rdi,r15
            rdi = r15;
            // 9e8a9 mov rdx,rbx
            rdx = rbx;
            // 9e8ac call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e8b1 lea rdi,[rbp-0x1b40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e4c0);
            // 9e8b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e8bd lea rsi,[rip+0x98c13] # 0x1374d7
            rsi = m.env.data_va(310436);
            // 9e8c4 lea rbx,[rbp-0x1b58]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e4a8);
            // 9e8cb mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 9e8d0 mov rdi,rbx
            rdi = rbx;
            // 9e8d3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e8d8 mov esi,0x28cb
            rsi = (0x28cbu64) as u32 as u64;
            // 9e8dd mov rdi,r15
            rdi = r15;
            // 9e8e0 mov rdx,rbx
            rdx = rbx;
            // 9e8e3 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e8e8 lea rdi,[rbp-0x1b58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e4a8);
            // 9e8ef call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e8f4 lea rsi,[rip+0x98bea] # 0x1374e5
            rsi = m.env.data_va(310450);
            // 9e8fb lea rbx,[rbp-0x1b70]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e490);
            // 9e902 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9e907 mov rdi,rbx
            rdi = rbx;
            // 9e90a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e90f mov esi,0x28cc
            rsi = (0x28ccu64) as u32 as u64;
            // 9e914 mov rdi,r15
            rdi = r15;
            // 9e917 mov rdx,rbx
            rdx = rbx;
            // 9e91a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e91f lea rdi,[rbp-0x1b70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e490);
            // 9e926 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e92b lea rsi,[rip+0x98bc9] # 0x1374fb
            rsi = m.env.data_va(310472);
            // 9e932 lea rbx,[rbp-0x1b88]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e478);
            // 9e939 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9e93e mov rdi,rbx
            rdi = rbx;
            // 9e941 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e946 mov esi,0x28cd
            rsi = (0x28cdu64) as u32 as u64;
            // 9e94b mov rdi,r15
            rdi = r15;
            // 9e94e mov rdx,rbx
            rdx = rbx;
            // 9e951 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e956 lea rdi,[rbp-0x1b88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e478);
            // 9e95d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e962 lea rsi,[rip+0x98ba2] # 0x13750b
            rsi = m.env.data_va(310488);
            // 9e969 lea rbx,[rbp-0x1ba0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e460);
            // 9e970 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9e975 mov rdi,rbx
            rdi = rbx;
            // 9e978 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e97d mov esi,0x28ce
            rsi = (0x28ceu64) as u32 as u64;
            // 9e982 mov rdi,r15
            rdi = r15;
            // 9e985 mov rdx,rbx
            rdx = rbx;
            // 9e988 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e98d lea rdi,[rbp-0x1ba0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e460);
            // 9e994 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e999 lea rsi,[rip+0x98b85] # 0x137525
            rsi = m.env.data_va(310514);
            // 9e9a0 lea rbx,[rbp-0x1bb8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e448);
            // 9e9a7 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9e9ac mov rdi,rbx
            rdi = rbx;
            // 9e9af call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e9b4 mov esi,0x28cf
            rsi = (0x28cfu64) as u32 as u64;
            // 9e9b9 mov rdi,r15
            rdi = r15;
            // 9e9bc mov rdx,rbx
            rdx = rbx;
            // 9e9bf call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e9c4 lea rdi,[rbp-0x1bb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e448);
            // 9e9cb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e9d0 lea rsi,[rip+0x98b67] # 0x13753e
            rsi = m.env.data_va(310539);
            // 9e9d7 lea rbx,[rbp-0x1bd0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e430);
            // 9e9de mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9e9e3 mov rdi,rbx
            rdi = rbx;
            // 9e9e6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e9eb mov esi,0x28d0
            rsi = (0x28d0u64) as u32 as u64;
            // 9e9f0 mov rdi,r15
            rdi = r15;
            // 9e9f3 mov rdx,rbx
            rdx = rbx;
            // 9e9f6 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9e9fb lea rdi,[rbp-0x1bd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e430);
            // 9ea02 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ea07 lea rsi,[rip+0x98b4b] # 0x137559
            rsi = m.env.data_va(310566);
            // 9ea0e lea rbx,[rbp-0x1be8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e418);
            // 9ea15 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9ea1a mov rdi,rbx
            rdi = rbx;
            // 9ea1d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ea22 mov esi,0x28d1
            rsi = (0x28d1u64) as u32 as u64;
            // 9ea27 mov rdi,r15
            rdi = r15;
            // 9ea2a mov rdx,rbx
            rdx = rbx;
            // 9ea2d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ea32 lea rdi,[rbp-0x1be8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e418);
            // 9ea39 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ea3e lea rsi,[rip+0x98b30] # 0x137575
            rsi = m.env.data_va(310594);
            // 9ea45 lea rbx,[rbp-0x1c00]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e400);
            // 9ea4c mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9ea51 mov rdi,rbx
            rdi = rbx;
            // 9ea54 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ea59 mov esi,0x28d2
            rsi = (0x28d2u64) as u32 as u64;
            // 9ea5e mov rdi,r15
            rdi = r15;
            // 9ea61 mov rdx,rbx
            rdx = rbx;
            // 9ea64 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ea69 lea rdi,[rbp-0x1c00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e400);
            // 9ea70 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ea75 lea rsi,[rip+0x98b14] # 0x137590
            rsi = m.env.data_va(310621);
            // 9ea7c lea rbx,[rbp-0x1c18]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e3e8);
            // 9ea83 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9ea88 mov rdi,rbx
            rdi = rbx;
            // 9ea8b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ea90 mov esi,0x28d3
            rsi = (0x28d3u64) as u32 as u64;
            // 9ea95 mov rdi,r15
            rdi = r15;
            // 9ea98 mov rdx,rbx
            rdx = rbx;
            // 9ea9b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eaa0 lea rdi,[rbp-0x1c18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e3e8);
            // 9eaa7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eaac lea rsi,[rip+0x98afa] # 0x1375ad
            rsi = m.env.data_va(310650);
            // 9eab3 lea rbx,[rbp-0x1c30]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e3d0);
            // 9eaba mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9eabf mov rdi,rbx
            rdi = rbx;
            // 9eac2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eac7 mov esi,0x28d4
            rsi = (0x28d4u64) as u32 as u64;
            // 9eacc mov rdi,r15
            rdi = r15;
            // 9eacf mov rdx,rbx
            rdx = rbx;
            // 9ead2 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ead7 lea rdi,[rbp-0x1c30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e3d0);
            // 9eade call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eae3 lea rsi,[rip+0x98ad3] # 0x1375bd
            rsi = m.env.data_va(310666);
            // 9eaea lea rbx,[rbp-0x1c48]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e3b8);
            // 9eaf1 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9eaf6 mov rdi,rbx
            rdi = rbx;
            // 9eaf9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eafe mov esi,0x28d5
            rsi = (0x28d5u64) as u32 as u64;
            // 9eb03 mov rdi,r15
            rdi = r15;
            // 9eb06 mov rdx,rbx
            rdx = rbx;
            // 9eb09 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eb0e lea rdi,[rbp-0x1c48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e3b8);
            // 9eb15 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eb1a lea rsi,[rip+0x98ab6] # 0x1375d7
            rsi = m.env.data_va(310692);
            // 9eb21 lea rbx,[rbp-0x1c60]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e3a0);
            // 9eb28 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9eb2d mov rdi,rbx
            rdi = rbx;
            // 9eb30 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eb35 mov esi,0x28d6
            rsi = (0x28d6u64) as u32 as u64;
            // 9eb3a mov rdi,r15
            rdi = r15;
            // 9eb3d mov rdx,rbx
            rdx = rbx;
            // 9eb40 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eb45 lea rdi,[rbp-0x1c60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e3a0);
            // 9eb4c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eb51 lea rsi,[rip+0x98a98] # 0x1375f0
            rsi = m.env.data_va(310717);
            // 9eb58 lea rbx,[rbp-0x1c78]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e388);
            // 9eb5f mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9eb64 mov rdi,rbx
            rdi = rbx;
            // 9eb67 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eb6c mov esi,0x28d7
            rsi = (0x28d7u64) as u32 as u64;
            // 9eb71 mov rdi,r15
            rdi = r15;
            // 9eb74 mov rdx,rbx
            rdx = rbx;
            // 9eb77 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eb7c lea rdi,[rbp-0x1c78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e388);
            // 9eb83 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eb88 lea rsi,[rip+0x98a7c] # 0x13760b
            rsi = m.env.data_va(310744);
            // 9eb8f lea rbx,[rbp-0x1c90]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e370);
            // 9eb96 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9eb9b mov rdi,rbx
            rdi = rbx;
            // 9eb9e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eba3 mov esi,0x28d8
            rsi = (0x28d8u64) as u32 as u64;
            // 9eba8 mov rdi,r15
            rdi = r15;
            // 9ebab mov rdx,rbx
            rdx = rbx;
            // 9ebae call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ebb3 lea rdi,[rbp-0x1c90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e370);
            // 9ebba call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ebbf lea rsi,[rip+0x98a61] # 0x137627
            rsi = m.env.data_va(310772);
            // 9ebc6 lea rbx,[rbp-0x1ca8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e358);
            // 9ebcd mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9ebd2 mov rdi,rbx
            rdi = rbx;
            // 9ebd5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ebda mov esi,0x28d9
            rsi = (0x28d9u64) as u32 as u64;
            // 9ebdf mov rdi,r15
            rdi = r15;
            // 9ebe2 mov rdx,rbx
            rdx = rbx;
            // 9ebe5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ebea lea rdi,[rbp-0x1ca8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e358);
            // 9ebf1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ebf6 lea rsi,[rip+0x98a45] # 0x137642
            rsi = m.env.data_va(310799);
            // 9ebfd lea rbx,[rbp-0x1cc0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e340);
            // 9ec04 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9ec09 mov rdi,rbx
            rdi = rbx;
            // 9ec0c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ec11 mov esi,0x28da
            rsi = (0x28dau64) as u32 as u64;
            // 9ec16 mov rdi,r15
            rdi = r15;
            // 9ec19 mov rdx,rbx
            rdx = rbx;
            // 9ec1c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ec21 lea rdi,[rbp-0x1cc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e340);
            // 9ec28 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ec2d lea rsi,[rip+0x98a2b] # 0x13765f
            rsi = m.env.data_va(310828);
            // 9ec34 lea rbx,[rbp-0x1cd8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e328);
            // 9ec3b mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9ec40 mov rdi,rbx
            rdi = rbx;
            // 9ec43 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ec48 mov esi,0x28db
            rsi = (0x28dbu64) as u32 as u64;
            // 9ec4d mov rdi,r15
            rdi = r15;
            // 9ec50 mov rdx,rbx
            rdx = rbx;
            // 9ec53 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ec58 lea rdi,[rbp-0x1cd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e328);
            // 9ec5f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ec64 lea rsi,[rip+0x98a08] # 0x137673
            rsi = m.env.data_va(310848);
            // 9ec6b lea rbx,[rbp-0x1cf0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e310);
            // 9ec72 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9ec77 mov rdi,rbx
            rdi = rbx;
            // 9ec7a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ec7f mov esi,0x28dc
            rsi = (0x28dcu64) as u32 as u64;
            // 9ec84 mov rdi,r15
            rdi = r15;
            // 9ec87 mov rdx,rbx
            rdx = rbx;
            // 9ec8a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ec8f lea rdi,[rbp-0x1cf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e310);
            // 9ec96 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ec9b lea rsi,[rip+0x989e5] # 0x137687
            rsi = m.env.data_va(310868);
            // 9eca2 lea rbx,[rbp-0x1d08]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e2f8);
            // 9eca9 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9ecae mov rdi,rbx
            rdi = rbx;
            // 9ecb1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ecb6 mov esi,0x28dd
            rsi = (0x28ddu64) as u32 as u64;
            // 9ecbb mov rdi,r15
            rdi = r15;
            // 9ecbe mov rdx,rbx
            rdx = rbx;
            // 9ecc1 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ecc6 lea rdi,[rbp-0x1d08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e2f8);
            // 9eccd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ecd2 lea rsi,[rip+0x989c0] # 0x137699
            rsi = m.env.data_va(310886);
            // 9ecd9 lea rbx,[rbp-0x1d20]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e2e0);
            // 9ece0 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9ece5 mov rdi,rbx
            rdi = rbx;
            // 9ece8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eced mov esi,0x28de
            rsi = (0x28deu64) as u32 as u64;
            // 9ecf2 mov rdi,r15
            rdi = r15;
            // 9ecf5 mov rdx,rbx
            rdx = rbx;
            // 9ecf8 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ecfd lea rdi,[rbp-0x1d20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e2e0);
            // 9ed04 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ed09 lea rsi,[rip+0x9899f] # 0x1376af
            rsi = m.env.data_va(310908);
            // 9ed10 lea rbx,[rbp-0x1d38]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e2c8);
            // 9ed17 mov edx,0x1e
            rdx = (0x1eu64) as u32 as u64;
            // 9ed1c mov rdi,rbx
            rdi = rbx;
            // 9ed1f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ed24 mov esi,0x28df
            rsi = (0x28dfu64) as u32 as u64;
            // 9ed29 mov rdi,r15
            rdi = r15;
            // 9ed2c mov rdx,rbx
            rdx = rbx;
            // 9ed2f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ed34 lea rdi,[rbp-0x1d38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e2c8);
            // 9ed3b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ed40 lea rsi,[rip+0x98987] # 0x1376ce
            rsi = m.env.data_va(310939);
            // 9ed47 lea rbx,[rbp-0x1d50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e2b0);
            // 9ed4e mov edx,0x1e
            rdx = (0x1eu64) as u32 as u64;
            // 9ed53 mov rdi,rbx
            rdi = rbx;
            // 9ed56 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ed5b mov esi,0x28e0
            rsi = (0x28e0u64) as u32 as u64;
            // 9ed60 mov rdi,r15
            rdi = r15;
            // 9ed63 mov rdx,rbx
            rdx = rbx;
            // 9ed66 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ed6b lea rdi,[rbp-0x1d50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e2b0);
            // 9ed72 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ed77 lea rsi,[rip+0x9896f] # 0x1376ed
            rsi = m.env.data_va(310970);
            // 9ed7e lea rbx,[rbp-0x1d68]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e298);
            // 9ed85 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9ed8a mov rdi,rbx
            rdi = rbx;
            // 9ed8d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ed92 mov esi,0x28e1
            rsi = (0x28e1u64) as u32 as u64;
            // 9ed97 mov rdi,r15
            rdi = r15;
            // 9ed9a mov rdx,rbx
            rdx = rbx;
            // 9ed9d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eda2 lea rdi,[rbp-0x1d68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e298);
            // 9eda9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9edae lea rsi,[rip+0x98954] # 0x137709
            rsi = m.env.data_va(310998);
            // 9edb5 lea rbx,[rbp-0x1d80]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e280);
            // 9edbc mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9edc1 mov rdi,rbx
            rdi = rbx;
            // 9edc4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9edc9 mov esi,0x28e2
            rsi = (0x28e2u64) as u32 as u64;
            // 9edce mov rdi,r15
            rdi = r15;
            // 9edd1 mov rdx,rbx
            rdx = rbx;
            // 9edd4 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9edd9 lea rdi,[rbp-0x1d80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e280);
            // 9ede0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ede5 lea rsi,[rip+0x9893a] # 0x137726
            rsi = m.env.data_va(311027);
            // 9edec lea rbx,[rbp-0x1d98]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e268);
            // 9edf3 mov edx,0x1e
            rdx = (0x1eu64) as u32 as u64;
            // 9edf8 mov rdi,rbx
            rdi = rbx;
            // 9edfb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ee00 mov esi,0x28e3
            rsi = (0x28e3u64) as u32 as u64;
            // 9ee05 mov rdi,r15
            rdi = r15;
            // 9ee08 mov rdx,rbx
            rdx = rbx;
            // 9ee0b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ee10 lea rdi,[rbp-0x1d98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e268);
            // 9ee17 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ee1c lea rsi,[rip+0x98922] # 0x137745
            rsi = m.env.data_va(311058);
            // 9ee23 lea rbx,[rbp-0x1db0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e250);
            // 9ee2a mov edx,0x1e
            rdx = (0x1eu64) as u32 as u64;
            // 9ee2f mov rdi,rbx
            rdi = rbx;
            // 9ee32 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ee37 mov esi,0x28e4
            rsi = (0x28e4u64) as u32 as u64;
            // 9ee3c mov rdi,r15
            rdi = r15;
            // 9ee3f mov rdx,rbx
            rdx = rbx;
            // 9ee42 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ee47 lea rdi,[rbp-0x1db0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e250);
            // 9ee4e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ee53 lea rsi,[rip+0x9890a] # 0x137764
            rsi = m.env.data_va(311089);
            // 9ee5a lea rbx,[rbp-0x1dc8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e238);
            // 9ee61 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9ee66 mov rdi,rbx
            rdi = rbx;
            // 9ee69 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ee6e mov esi,0x28e5
            rsi = (0x28e5u64) as u32 as u64;
            // 9ee73 mov rdi,r15
            rdi = r15;
            // 9ee76 mov rdx,rbx
            rdx = rbx;
            // 9ee79 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ee7e lea rdi,[rbp-0x1dc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e238);
            // 9ee85 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ee8a lea rsi,[rip+0x988ef] # 0x137780
            rsi = m.env.data_va(311117);
            // 9ee91 lea rbx,[rbp-0x1de0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e220);
            // 9ee98 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9ee9d mov rdi,rbx
            rdi = rbx;
            // 9eea0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eea5 mov esi,0x28e6
            rsi = (0x28e6u64) as u32 as u64;
            // 9eeaa mov rdi,r15
            rdi = r15;
            // 9eead mov rdx,rbx
            rdx = rbx;
            // 9eeb0 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eeb5 lea rdi,[rbp-0x1de0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e220);
            // 9eebc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eec1 lea rsi,[rip+0x988d5] # 0x13779d
            rsi = m.env.data_va(311146);
            // 9eec8 lea rbx,[rbp-0x1df8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e208);
            // 9eecf mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9eed4 mov rdi,rbx
            rdi = rbx;
            // 9eed7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eedc mov esi,0x28e7
            rsi = (0x28e7u64) as u32 as u64;
            // 9eee1 mov rdi,r15
            rdi = r15;
            // 9eee4 mov rdx,rbx
            rdx = rbx;
            // 9eee7 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eeec lea rdi,[rbp-0x1df8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e208);
            // 9eef3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9eef8 lea rsi,[rip+0x988bb] # 0x1377ba
            rsi = m.env.data_va(311175);
            // 9eeff lea rbx,[rbp-0x1e10]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e1f0);
            // 9ef06 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9ef0b mov rdi,rbx
            rdi = rbx;
            // 9ef0e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ef13 mov esi,0x28e8
            rsi = (0x28e8u64) as u32 as u64;
            // 9ef18 mov rdi,r15
            rdi = r15;
            // 9ef1b mov rdx,rbx
            rdx = rbx;
            // 9ef1e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ef23 lea rdi,[rbp-0x1e10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e1f0);
            // 9ef2a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ef2f lea rsi,[rip+0x9889d] # 0x1377d3
            rsi = m.env.data_va(311200);
            // 9ef36 lea rbx,[rbp-0x1e28]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e1d8);
            // 9ef3d mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9ef42 mov rdi,rbx
            rdi = rbx;
            // 9ef45 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ef4a mov esi,0x28e9
            rsi = (0x28e9u64) as u32 as u64;
            // 9ef4f mov rdi,r15
            rdi = r15;
            // 9ef52 mov rdx,rbx
            rdx = rbx;
            // 9ef55 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ef5a lea rdi,[rbp-0x1e28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e1d8);
            // 9ef61 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ef66 lea rsi,[rip+0x98880] # 0x1377ed
            rsi = m.env.data_va(311226);
            // 9ef6d lea rbx,[rbp-0x1e40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e1c0);
            // 9ef74 mov edx,0x1e
            rdx = (0x1eu64) as u32 as u64;
            // 9ef79 mov rdi,rbx
            rdi = rbx;
            // 9ef7c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ef81 mov esi,0x28ea
            rsi = (0x28eau64) as u32 as u64;
            // 9ef86 mov rdi,r15
            rdi = r15;
            // 9ef89 mov rdx,rbx
            rdx = rbx;
            // 9ef8c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ef91 lea rdi,[rbp-0x1e40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e1c0);
            // 9ef98 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ef9d lea rsi,[rip+0x98868] # 0x13780c
            rsi = m.env.data_va(311257);
            // 9efa4 lea rbx,[rbp-0x1e58]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e1a8);
            // 9efab mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9efb0 mov rdi,rbx
            rdi = rbx;
            // 9efb3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9efb8 mov esi,0x28eb
            rsi = (0x28ebu64) as u32 as u64;
            // 9efbd mov rdi,r15
            rdi = r15;
            // 9efc0 mov rdx,rbx
            rdx = rbx;
            // 9efc3 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9efc8 lea rdi,[rbp-0x1e58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e1a8);
            // 9efcf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9efd4 lea rsi,[rip+0x98847] # 0x137822
            rsi = m.env.data_va(311279);
            // 9efdb lea rbx,[rbp-0x1e70]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e190);
            // 9efe2 mov edx,0x1e
            rdx = (0x1eu64) as u32 as u64;
            // 9efe7 mov rdi,rbx
            rdi = rbx;
            // 9efea call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9efef mov esi,0x28ec
            rsi = (0x28ecu64) as u32 as u64;
            // 9eff4 mov rdi,r15
            rdi = r15;
            // 9eff7 mov rdx,rbx
            rdx = rbx;
            // 9effa call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9efff lea rdi,[rbp-0x1e70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e190);
            // 9f006 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f00b lea rsi,[rip+0x9882f] # 0x137841
            rsi = m.env.data_va(311310);
            // 9f012 lea rbx,[rbp-0x1e88]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e178);
            // 9f019 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9f01e mov rdi,rbx
            rdi = rbx;
            // 9f021 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f026 mov esi,0x28ed
            rsi = (0x28edu64) as u32 as u64;
            // 9f02b mov rdi,r15
            rdi = r15;
            // 9f02e mov rdx,rbx
            rdx = rbx;
            // 9f031 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f036 lea rdi,[rbp-0x1e88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e178);
            // 9f03d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f042 lea rsi,[rip+0x98814] # 0x13785d
            rsi = m.env.data_va(311338);
            // 9f049 lea rbx,[rbp-0x1ea0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e160);
            // 9f050 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9f055 mov rdi,rbx
            rdi = rbx;
            // 9f058 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f05d mov esi,0x28ee
            rsi = (0x28eeu64) as u32 as u64;
            // 9f062 mov rdi,r15
            rdi = r15;
            // 9f065 mov rdx,rbx
            rdx = rbx;
            // 9f068 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f06d lea rdi,[rbp-0x1ea0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e160);
            // 9f074 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f079 lea rsi,[rip+0x987f9] # 0x137879
            rsi = m.env.data_va(311366);
            // 9f080 lea rbx,[rbp-0x1eb8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e148);
            // 9f087 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9f08c mov rdi,rbx
            rdi = rbx;
            // 9f08f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f094 mov esi,0x28ef
            rsi = (0x28efu64) as u32 as u64;
            // 9f099 mov rdi,r15
            rdi = r15;
            // 9f09c mov rdx,rbx
            rdx = rbx;
            // 9f09f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f0a4 lea rdi,[rbp-0x1eb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e148);
            // 9f0ab call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f0b0 lea rsi,[rip+0x987dc] # 0x137893
            rsi = m.env.data_va(311392);
            // 9f0b7 lea rbx,[rbp-0x1ed0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e130);
            // 9f0be mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9f0c3 mov rdi,rbx
            rdi = rbx;
            // 9f0c6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f0cb mov esi,0x28f0
            rsi = (0x28f0u64) as u32 as u64;
            // 9f0d0 mov rdi,r15
            rdi = r15;
            // 9f0d3 mov rdx,rbx
            rdx = rbx;
            // 9f0d6 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f0db lea rdi,[rbp-0x1ed0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e130);
            // 9f0e2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f0e7 lea rsi,[rip+0x987bd] # 0x1378ab
            rsi = m.env.data_va(311416);
            // 9f0ee lea rbx,[rbp-0x1ee8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e118);
            // 9f0f5 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9f0fa mov rdi,rbx
            rdi = rbx;
            // 9f0fd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f102 mov esi,0x28f1
            rsi = (0x28f1u64) as u32 as u64;
            // 9f107 mov rdi,r15
            rdi = r15;
            // 9f10a mov rdx,rbx
            rdx = rbx;
            // 9f10d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f112 lea rdi,[rbp-0x1ee8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e118);
            // 9f119 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f11e lea rsi,[rip+0x987a1] # 0x1378c6
            rsi = m.env.data_va(311443);
            // 9f125 lea rbx,[rbp-0x1f00]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e100);
            // 9f12c mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9f131 mov rdi,rbx
            rdi = rbx;
            // 9f134 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f139 mov esi,0x28f2
            rsi = (0x28f2u64) as u32 as u64;
            // 9f13e mov rdi,r15
            rdi = r15;
            // 9f141 mov rdx,rbx
            rdx = rbx;
            // 9f144 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f149 lea rdi,[rbp-0x1f00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e100);
            // 9f150 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f155 lea rsi,[rip+0x98787] # 0x1378e3
            rsi = m.env.data_va(311472);
            // 9f15c lea rbx,[rbp-0x1f18]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e0e8);
            // 9f163 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // 9f168 mov rdi,rbx
            rdi = rbx;
            // 9f16b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f170 mov esi,0x28f3
            rsi = (0x28f3u64) as u32 as u64;
            // 9f175 mov rdi,r15
            rdi = r15;
            // 9f178 mov rdx,rbx
            rdx = rbx;
            // 9f17b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f180 lea rdi,[rbp-0x1f18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e0e8);
            // 9f187 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f18c lea rsi,[rip+0x98769] # 0x1378fc
            rsi = m.env.data_va(311497);
            // 9f193 lea rbx,[rbp-0x1f30]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e0d0);
            // 9f19a mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9f19f mov rdi,rbx
            rdi = rbx;
            // 9f1a2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f1a7 mov esi,0x28f4
            rsi = (0x28f4u64) as u32 as u64;
            // 9f1ac mov rdi,r15
            rdi = r15;
            // 9f1af mov rdx,rbx
            rdx = rbx;
            // 9f1b2 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f1b7 lea rdi,[rbp-0x1f30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e0d0);
            // 9f1be call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f1c3 lea rsi,[rip+0x9874c] # 0x137916
            rsi = m.env.data_va(311523);
            // 9f1ca lea rbx,[rbp-0x1f48]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e0b8);
            // 9f1d1 mov edx,0x1e
            rdx = (0x1eu64) as u32 as u64;
            // 9f1d6 mov rdi,rbx
            rdi = rbx;
            // 9f1d9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f1de mov esi,0x28f5
            rsi = (0x28f5u64) as u32 as u64;
            // 9f1e3 mov rdi,r15
            rdi = r15;
            // 9f1e6 mov rdx,rbx
            rdx = rbx;
            // 9f1e9 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f1ee lea rdi,[rbp-0x1f48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e0b8);
            // 9f1f5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f1fa lea rsi,[rip+0x98734] # 0x137935
            rsi = m.env.data_va(311554);
            // 9f201 lea rbx,[rbp-0x1f60]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e0a0);
            // 9f208 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 9f20d mov rdi,rbx
            rdi = rbx;
            // 9f210 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f215 mov esi,0x28f6
            rsi = (0x28f6u64) as u32 as u64;
            // 9f21a mov rdi,r15
            rdi = r15;
            // 9f21d mov rdx,rbx
            rdx = rbx;
            // 9f220 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f225 lea rdi,[rbp-0x1f60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e0a0);
            // 9f22c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f231 lea rsi,[rip+0x98713] # 0x13794b
            rsi = m.env.data_va(311576);
            // 9f238 lea rbx,[rbp-0x1f78]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e088);
            // 9f23f mov edx,0x1e
            rdx = (0x1eu64) as u32 as u64;
            // 9f244 mov rdi,rbx
            rdi = rbx;
            // 9f247 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f24c mov esi,0x28f7
            rsi = (0x28f7u64) as u32 as u64;
            // 9f251 mov rdi,r15
            rdi = r15;
            // 9f254 mov rdx,rbx
            rdx = rbx;
            // 9f257 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f25c lea rdi,[rbp-0x1f78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e088);
            // 9f263 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f268 lea rsi,[rip+0x986fb] # 0x13796a
            rsi = m.env.data_va(311607);
            // 9f26f lea rbx,[rbp-0x1f90]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e070);
            // 9f276 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9f27b mov rdi,rbx
            rdi = rbx;
            // 9f27e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f283 mov esi,0x28f8
            rsi = (0x28f8u64) as u32 as u64;
            // 9f288 mov rdi,r15
            rdi = r15;
            // 9f28b mov rdx,rbx
            rdx = rbx;
            // 9f28e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f293 lea rdi,[rbp-0x1f90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e070);
            // 9f29a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f29f lea rsi,[rip+0x986e0] # 0x137986
            rsi = m.env.data_va(311635);
            // 9f2a6 lea rbx,[rbp-0x1fa8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e058);
            // 9f2ad mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9f2b2 mov rdi,rbx
            rdi = rbx;
            // 9f2b5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f2ba mov esi,0x28f9
            rsi = (0x28f9u64) as u32 as u64;
            // 9f2bf mov rdi,r15
            rdi = r15;
            // 9f2c2 mov rdx,rbx
            rdx = rbx;
            // 9f2c5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f2ca lea rdi,[rbp-0x1fa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e058);
            // 9f2d1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f2d6 lea rsi,[rip+0x986c5] # 0x1379a2
            rsi = m.env.data_va(311663);
            // 9f2dd lea rbx,[rbp-0x1fc0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e040);
            // 9f2e4 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9f2e9 mov rdi,rbx
            rdi = rbx;
            // 9f2ec call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f2f1 mov esi,0x28fa
            rsi = (0x28fau64) as u32 as u64;
            // 9f2f6 mov rdi,r15
            rdi = r15;
            // 9f2f9 mov rdx,rbx
            rdx = rbx;
            // 9f2fc call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f301 lea rdi,[rbp-0x1fc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e040);
            // 9f308 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f30d lea rsi,[rip+0x986a8] # 0x1379bc
            rsi = m.env.data_va(311689);
            // 9f314 lea rbx,[rbp-0x1fd8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e028);
            // 9f31b mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 9f320 mov rdi,rbx
            rdi = rbx;
            // 9f323 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f328 mov esi,0x28fb
            rsi = (0x28fbu64) as u32 as u64;
            // 9f32d mov rdi,r15
            rdi = r15;
            // 9f330 mov rdx,rbx
            rdx = rbx;
            // 9f333 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f338 lea rdi,[rbp-0x1fd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e028);
            // 9f33f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f344 lea rsi,[rip+0x98689] # 0x1379d4
            rsi = m.env.data_va(311713);
            // 9f34b lea rbx,[rbp-0x1ff0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_e010);
            // 9f352 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9f357 mov rdi,rbx
            rdi = rbx;
            // 9f35a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f35f mov esi,0x28fc
            rsi = (0x28fcu64) as u32 as u64;
            // 9f364 mov rdi,r15
            rdi = r15;
            // 9f367 mov rdx,rbx
            rdx = rbx;
            // 9f36a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f36f lea rdi,[rbp-0x1ff0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e010);
            // 9f376 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f37b lea rsi,[rip+0x9866d] # 0x1379ef
            rsi = m.env.data_va(311740);
            // 9f382 lea rbx,[rbp-0x2008]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dff8);
            // 9f389 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9f38e mov rdi,rbx
            rdi = rbx;
            // 9f391 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f396 mov esi,0x28fd
            rsi = (0x28fdu64) as u32 as u64;
            // 9f39b mov rdi,r15
            rdi = r15;
            // 9f39e mov rdx,rbx
            rdx = rbx;
            // 9f3a1 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f3a6 lea rdi,[rbp-0x2008]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dff8);
            // 9f3ad call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f3b2 lea rsi,[rip+0x98648] # 0x137a01
            rsi = m.env.data_va(311758);
            // 9f3b9 lea rbx,[rbp-0x2020]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dfe0);
            // 9f3c0 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9f3c5 mov rdi,rbx
            rdi = rbx;
            // 9f3c8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f3cd mov esi,0x28fe
            rsi = (0x28feu64) as u32 as u64;
            // 9f3d2 mov rdi,r15
            rdi = r15;
            // 9f3d5 mov rdx,rbx
            rdx = rbx;
            // 9f3d8 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f3dd lea rdi,[rbp-0x2020]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dfe0);
            // 9f3e4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f3e9 lea rsi,[rip+0x9862c] # 0x137a1c
            rsi = m.env.data_va(311785);
            // 9f3f0 lea rbx,[rbp-0x2038]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dfc8);
            // 9f3f7 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9f3fc mov rdi,rbx
            rdi = rbx;
            // 9f3ff call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f404 mov esi,0x28ff
            rsi = (0x28ffu64) as u32 as u64;
            // 9f409 mov rdi,r15
            rdi = r15;
            // 9f40c mov rdx,rbx
            rdx = rbx;
            // 9f40f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f414 lea rdi,[rbp-0x2038]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dfc8);
            // 9f41b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f420 lea rsi,[rip+0x98608] # 0x137a2f
            rsi = m.env.data_va(311804);
            // 9f427 lea rbx,[rbp-0x2050]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dfb0);
            // 9f42e mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9f433 mov rdi,rbx
            rdi = rbx;
            // 9f436 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f43b mov esi,0x2900
            rsi = (0x2900u64) as u32 as u64;
            // 9f440 mov rdi,r15
            rdi = r15;
            // 9f443 mov rdx,rbx
            rdx = rbx;
            // 9f446 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f44b lea rdi,[rbp-0x2050]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dfb0);
            // 9f452 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f457 lea rsi,[rip+0x985e4] # 0x137a42
            rsi = m.env.data_va(311823);
            // 9f45e lea rbx,[rbp-0x2068]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_df98);
            // 9f465 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9f46a mov rdi,rbx
            rdi = rbx;
            // 9f46d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f472 mov esi,0x2901
            rsi = (0x2901u64) as u32 as u64;
            // 9f477 mov rdi,r15
            rdi = r15;
            // 9f47a mov rdx,rbx
            rdx = rbx;
            // 9f47d call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f482 lea rdi,[rbp-0x2068]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df98);
            // 9f489 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f48e lea rsi,[rip+0x985c1] # 0x137a56
            rsi = m.env.data_va(311843);
            // 9f495 lea rbx,[rbp-0x2080]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_df80);
            // 9f49c mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9f4a1 mov rdi,rbx
            rdi = rbx;
            // 9f4a4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f4a9 mov esi,0x2902
            rsi = (0x2902u64) as u32 as u64;
            // 9f4ae mov rdi,r15
            rdi = r15;
            // 9f4b1 mov rdx,rbx
            rdx = rbx;
            // 9f4b4 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f4b9 lea rdi,[rbp-0x2080]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df80);
            // 9f4c0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f4c5 lea rsi,[rip+0x9859e] # 0x137a6a
            rsi = m.env.data_va(311863);
            // 9f4cc lea rbx,[rbp-0x2098]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_df68);
            // 9f4d3 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9f4d8 mov rdi,rbx
            rdi = rbx;
            // 9f4db call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f4e0 mov esi,0x2903
            rsi = (0x2903u64) as u32 as u64;
            // 9f4e5 mov rdi,r15
            rdi = r15;
            // 9f4e8 mov rdx,rbx
            rdx = rbx;
            // 9f4eb call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f4f0 lea rdi,[rbp-0x2098]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df68);
            // 9f4f7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f4fc lea rsi,[rip+0x9857b] # 0x137a7e
            rsi = m.env.data_va(311883);
            // 9f503 lea rbx,[rbp-0x20b0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_df50);
            // 9f50a mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9f50f mov rdi,rbx
            rdi = rbx;
            // 9f512 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f517 mov esi,0x2904
            rsi = (0x2904u64) as u32 as u64;
            // 9f51c mov rdi,r15
            rdi = r15;
            // 9f51f mov rdx,rbx
            rdx = rbx;
            // 9f522 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f527 lea rdi,[rbp-0x20b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df50);
            // 9f52e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f533 lea rsi,[rip+0x98558] # 0x137a92
            rsi = m.env.data_va(311903);
            // 9f53a lea rbx,[rbp-0x20c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_df38);
            // 9f541 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 9f546 mov rdi,rbx
            rdi = rbx;
            // 9f549 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f54e mov esi,0x2905
            rsi = (0x2905u64) as u32 as u64;
            // 9f553 mov rdi,r15
            rdi = r15;
            // 9f556 mov rdx,rbx
            rdx = rbx;
            // 9f559 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f55e lea rdi,[rbp-0x20c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df38);
            // 9f565 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f56a lea rsi,[rip+0x98490] # 0x137a01
            rsi = m.env.data_va(311758);
            // 9f571 lea rbx,[rbp-0x20e0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_df20);
            // 9f578 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            // 9f57d mov rdi,rbx
            rdi = rbx;
            // 9f580 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f585 mov esi,0x28fe
            rsi = (0x28feu64) as u32 as u64;
            // 9f58a mov rdi,r15
            rdi = r15;
            // 9f58d mov rdx,rbx
            rdx = rbx;
            // 9f590 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f595 lea rdi,[rbp-0x20e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df20);
            // 9f59c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f5a1 lea rsi,[rip+0x984f9] # 0x137aa1
            rsi = m.env.data_va(311918);
            // 9f5a8 lea rbx,[rbp-0x20f8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_df08);
            // 9f5af mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9f5b4 mov rdi,rbx
            rdi = rbx;
            // 9f5b7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f5bc mov esi,0x2906
            rsi = (0x2906u64) as u32 as u64;
            // 9f5c1 mov rdi,r15
            rdi = r15;
            // 9f5c4 mov rdx,rbx
            rdx = rbx;
            // 9f5c7 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f5cc lea rdi,[rbp-0x20f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df08);
            // 9f5d3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f5d8 lea rsi,[rip+0x984d6] # 0x137ab5
            rsi = m.env.data_va(311938);
            // 9f5df lea rbx,[rbp-0x2110]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_def0);
            // 9f5e6 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // 9f5eb mov rdi,rbx
            rdi = rbx;
            // 9f5ee call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f5f3 mov esi,0x2907
            rsi = (0x2907u64) as u32 as u64;
            // 9f5f8 mov rdi,r15
            rdi = r15;
            // 9f5fb mov rdx,rbx
            rdx = rbx;
            // 9f5fe call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f603 lea rdi,[rbp-0x2110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_def0);
            // 9f60a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f60f lea rsi,[rip+0x984b9] # 0x137acf
            rsi = m.env.data_va(311964);
            // 9f616 lea rbx,[rbp-0x2128]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ded8);
            // 9f61d mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9f622 mov rdi,rbx
            rdi = rbx;
            // 9f625 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f62a mov esi,0x2908
            rsi = (0x2908u64) as u32 as u64;
            // 9f62f mov rdi,r15
            rdi = r15;
            // 9f632 mov rdx,rbx
            rdx = rbx;
            // 9f635 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f63a lea rdi,[rbp-0x2128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ded8);
            // 9f641 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f646 lea rsi,[rip+0x98497] # 0x137ae4
            rsi = m.env.data_va(311985);
            // 9f64d lea rbx,[rbp-0x2140]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dec0);
            // 9f654 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 9f659 mov rdi,rbx
            rdi = rbx;
            // 9f65c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f661 mov esi,0x2909
            rsi = (0x2909u64) as u32 as u64;
            // 9f666 mov rdi,r15
            rdi = r15;
            // 9f669 mov rdx,rbx
            rdx = rbx;
            // 9f66c call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f671 lea rdi,[rbp-0x2140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dec0);
            // 9f678 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f67d lea rsi,[rip+0x9847c] # 0x137b00
            rsi = m.env.data_va(312013);
            // 9f684 lea rbx,[rbp-0x2158]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dea8);
            // 9f68b mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9f690 mov rdi,rbx
            rdi = rbx;
            // 9f693 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f698 mov esi,0x290a
            rsi = (0x290au64) as u32 as u64;
            // 9f69d mov rdi,r15
            rdi = r15;
            // 9f6a0 mov rdx,rbx
            rdx = rbx;
            // 9f6a3 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f6a8 lea rdi,[rbp-0x2158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dea8);
            // 9f6af call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f6b4 lea rsi,[rip+0x98463] # 0x137b1e
            rsi = m.env.data_va(312043);
            // 9f6bb lea rbx,[rbp-0x2170]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_de90);
            // 9f6c2 mov edx,0x1e
            rdx = (0x1eu64) as u32 as u64;
            // 9f6c7 mov rdi,rbx
            rdi = rbx;
            // 9f6ca call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f6cf mov esi,0x290b
            rsi = (0x290bu64) as u32 as u64;
            // 9f6d4 mov rdi,r15
            rdi = r15;
            // 9f6d7 mov rdx,rbx
            rdx = rbx;
            // 9f6da call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f6df lea rdi,[rbp-0x2170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de90);
            // 9f6e6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f6eb lea rsi,[rip+0x9844b] # 0x137b3d
            rsi = m.env.data_va(312074);
            // 9f6f2 lea rbx,[rbp-0x2188]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_de78);
            // 9f6f9 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9f6fe mov rdi,rbx
            rdi = rbx;
            // 9f701 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f706 mov esi,0x290c
            rsi = (0x290cu64) as u32 as u64;
            // 9f70b mov rdi,r15
            rdi = r15;
            // 9f70e mov rdx,rbx
            rdx = rbx;
            // 9f711 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f716 lea rdi,[rbp-0x2188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de78);
            // 9f71d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f722 lea rsi,[rip+0x98431] # 0x137b5a
            rsi = m.env.data_va(312103);
            // 9f729 lea rbx,[rbp-0x21a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_de60);
            // 9f730 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9f735 mov rdi,rbx
            rdi = rbx;
            // 9f738 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f73d mov esi,0x290d
            rsi = (0x290du64) as u32 as u64;
            // 9f742 mov rdi,r15
            rdi = r15;
            // 9f745 mov rdx,rbx
            rdx = rbx;
            // 9f748 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f74d lea rdi,[rbp-0x21a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de60);
            // 9f754 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f759 lea rsi,[rip+0x9841a] # 0x137b7a
            rsi = m.env.data_va(312135);
            // 9f760 lea rbx,[rbp-0x21b8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_de48);
            // 9f767 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9f76c mov rdi,rbx
            rdi = rbx;
            // 9f76f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f774 mov esi,0x290e
            rsi = (0x290eu64) as u32 as u64;
            // 9f779 mov rdi,r15
            rdi = r15;
            // 9f77c mov rdx,rbx
            rdx = rbx;
            // 9f77f call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f784 lea rdi,[rbp-0x21b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de48);
            // 9f78b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f790 lea rsi,[rip+0x98403] # 0x137b9a
            rsi = m.env.data_va(312167);
            // 9f797 lea rbx,[rbp-0x21d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_de30);
            // 9f79e mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9f7a3 mov rdi,rbx
            rdi = rbx;
            // 9f7a6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f7ab mov esi,0x290f
            rsi = (0x290fu64) as u32 as u64;
            // 9f7b0 mov rdi,r15
            rdi = r15;
            // 9f7b3 mov rdx,rbx
            rdx = rbx;
            // 9f7b6 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f7bb lea rdi,[rbp-0x21d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de30);
            // 9f7c2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f7c7 lea rsi,[rip+0x983ec] # 0x137bba
            rsi = m.env.data_va(312199);
            // 9f7ce lea rbx,[rbp-0x21e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_de18);
            // 9f7d5 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9f7da mov rdi,rbx
            rdi = rbx;
            // 9f7dd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f7e2 mov esi,0x2910
            rsi = (0x2910u64) as u32 as u64;
            // 9f7e7 mov rdi,r15
            rdi = r15;
            // 9f7ea mov rdx,rbx
            rdx = rbx;
            // 9f7ed call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f7f2 lea rdi,[rbp-0x21e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de18);
            // 9f7f9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f7fe lea rsi,[rip+0x983d3] # 0x137bd8
            rsi = m.env.data_va(312229);
            // 9f805 lea rbx,[rbp-0x2200]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_de00);
            // 9f80c mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9f811 mov rdi,rbx
            rdi = rbx;
            // 9f814 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f819 mov esi,0x2911
            rsi = (0x2911u64) as u32 as u64;
            // 9f81e mov rdi,r15
            rdi = r15;
            // 9f821 mov rdx,rbx
            rdx = rbx;
            // 9f824 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f829 lea rdi,[rbp-0x2200]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de00);
            // 9f830 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f835 lea rsi,[rip+0x983ba] # 0x137bf6
            rsi = m.env.data_va(312259);
            // 9f83c lea rbx,[rbp-0x2218]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dde8);
            // 9f843 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9f848 mov rdi,rbx
            rdi = rbx;
            // 9f84b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f850 mov esi,0x2912
            rsi = (0x2912u64) as u32 as u64;
            // 9f855 mov rdi,r15
            rdi = r15;
            // 9f858 mov rdx,rbx
            rdx = rbx;
            // 9f85b call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f860 lea rdi,[rbp-0x2218]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dde8);
            // 9f867 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f86c lea rsi,[rip+0x983a1] # 0x137c14
            rsi = m.env.data_va(312289);
            // 9f873 lea rbx,[rbp-0x2230]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ddd0);
            // 9f87a mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9f87f mov rdi,rbx
            rdi = rbx;
            // 9f882 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f887 mov esi,0x2913
            rsi = (0x2913u64) as u32 as u64;
            // 9f88c mov rdi,r15
            rdi = r15;
            // 9f88f mov rdx,rbx
            rdx = rbx;
            // 9f892 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f897 lea rdi,[rbp-0x2230]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ddd0);
            // 9f89e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f8a3 lea rsi,[rip+0x9838d] # 0x137c37
            rsi = m.env.data_va(312324);
            // 9f8aa lea rbx,[rbp-0x2248]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ddb8);
            // 9f8b1 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9f8b6 mov rdi,rbx
            rdi = rbx;
            // 9f8b9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f8be mov esi,0x2914
            rsi = (0x2914u64) as u32 as u64;
            // 9f8c3 mov rdi,r15
            rdi = r15;
            // 9f8c6 mov rdx,rbx
            rdx = rbx;
            // 9f8c9 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f8ce lea rdi,[rbp-0x2248]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ddb8);
            // 9f8d5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f8da lea rsi,[rip+0x98379] # 0x137c5a
            rsi = m.env.data_va(312359);
            // 9f8e1 lea rbx,[rbp-0x2260]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dda0);
            // 9f8e8 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // 9f8ed mov rdi,rbx
            rdi = rbx;
            // 9f8f0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f8f5 mov esi,0x2915
            rsi = (0x2915u64) as u32 as u64;
            // 9f8fa mov rdi,r15
            rdi = r15;
            // 9f8fd mov rdx,rbx
            rdx = rbx;
            // 9f900 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f905 lea rdi,[rbp-0x2260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dda0);
            // 9f90c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f911 lea rsi,[rip+0x98365] # 0x137c7d
            rsi = m.env.data_va(312394);
            // 9f918 lea rbx,[rbp-0x2278]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dd88);
            // 9f91f mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // 9f924 mov rdi,rbx
            rdi = rbx;
            // 9f927 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f92c mov esi,0x2916
            rsi = (0x2916u64) as u32 as u64;
            // 9f931 mov rdi,r15
            rdi = r15;
            // 9f934 mov rdx,rbx
            rdx = rbx;
            // 9f937 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f93c lea rdi,[rbp-0x2278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dd88);
            // 9f943 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f948 lea rsi,[rip+0x9834f] # 0x137c9e
            rsi = m.env.data_va(312427);
            // 9f94f lea rbx,[rbp-0x2290]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dd70);
            // 9f956 mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // 9f95b mov rdi,rbx
            rdi = rbx;
            // 9f95e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f963 mov esi,0x2917
            rsi = (0x2917u64) as u32 as u64;
            // 9f968 mov rdi,r15
            rdi = r15;
            // 9f96b mov rdx,rbx
            rdx = rbx;
            // 9f96e call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f973 lea rdi,[rbp-0x2290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dd70);
            // 9f97a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f97f lea rsi,[rip+0x98339] # 0x137cbf
            rsi = m.env.data_va(312460);
            // 9f986 lea rbx,[rbp-0x22a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dd58);
            // 9f98d mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // 9f992 mov rdi,rbx
            rdi = rbx;
            // 9f995 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f99a mov esi,0x2918
            rsi = (0x2918u64) as u32 as u64;
            // 9f99f mov rdi,r15
            rdi = r15;
            // 9f9a2 mov rdx,rbx
            rdx = rbx;
            // 9f9a5 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f9aa lea rdi,[rbp-0x22a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dd58);
            // 9f9b1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f9b6 lea rsi,[rip+0x98323] # 0x137ce0
            rsi = m.env.data_va(312493);
            // 9f9bd lea rbx,[rbp-0x22c0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dd40);
            // 9f9c4 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9f9c9 mov rdi,rbx
            rdi = rbx;
            // 9f9cc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f9d1 mov esi,0x2919
            rsi = (0x2919u64) as u32 as u64;
            // 9f9d6 mov rdi,r15
            rdi = r15;
            // 9f9d9 mov rdx,rbx
            rdx = rbx;
            // 9f9dc call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f9e1 lea rdi,[rbp-0x22c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dd40);
            // 9f9e8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9f9ed lea rsi,[rip+0x98300] # 0x137cf4
            rsi = m.env.data_va(312513);
            // 9f9f4 lea rbx,[rbp-0x22d8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dd28);
            // 9f9fb mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 9fa00 mov rdi,rbx
            rdi = rbx;
            // 9fa03 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fa08 mov esi,0x291a
            rsi = (0x291au64) as u32 as u64;
            // 9fa0d mov rdi,r15
            rdi = r15;
            // 9fa10 mov rdx,rbx
            rdx = rbx;
            // 9fa13 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fa18 lea rdi,[rbp-0x22d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dd28);
            // 9fa1f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fa24 lea rsi,[rip+0x982de] # 0x137d09
            rsi = m.env.data_va(312534);
            // 9fa2b lea rbx,[rbp-0x22f0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dd10);
            // 9fa32 mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // 9fa37 mov rdi,rbx
            rdi = rbx;
            // 9fa3a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fa3f mov esi,0x291b
            rsi = (0x291bu64) as u32 as u64;
            // 9fa44 mov rdi,r15
            rdi = r15;
            // 9fa47 mov rdx,rbx
            rdx = rbx;
            // 9fa4a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fa4f lea rdi,[rbp-0x22f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dd10);
            // 9fa56 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fa5b lea rsi,[rip+0x982c7] # 0x137d29
            rsi = m.env.data_va(312566);
            // 9fa62 lea rbx,[rbp-0x2308]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dcf8);
            // 9fa69 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 9fa6e mov rdi,rbx
            rdi = rbx;
            // 9fa71 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fa76 mov esi,0x291d
            rsi = (0x291du64) as u32 as u64;
            // 9fa7b mov rdi,r15
            rdi = r15;
            // 9fa7e mov rdx,rbx
            rdx = rbx;
            // 9fa81 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fa86 lea rdi,[rbp-0x2308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dcf8);
            // 9fa8d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fa92 lea rsi,[rip+0x9829d] # 0x137d36
            rsi = m.env.data_va(312579);
            // 9fa99 lea rbx,[rbp-0x2320]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_dce0);
            // 9faa0 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 9faa5 mov rdi,rbx
            rdi = rbx;
            // 9faa8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9faad mov esi,0x291e
            rsi = (0x291eu64) as u32 as u64;
            // 9fab2 mov rdi,r15
            rdi = r15;
            // 9fab5 mov rdx,rbx
            rdx = rbx;
            // 9fab8 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fabd lea rdi,[rbp-0x2320]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dce0);
            // 9fac4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fac9 lea r12,[rbp-0x23b8]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_dc48);
            // 9fad0 lea r14,[rbp-0x2420]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_dbe0);
            // 9fad7 lea r13,[rbp-0x23e0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_dc20);
            // 9fade lea rax,[r15+0x180]
            rax = r15.wrapping_add(0x180);
            // 9fae5 mov QWORD PTR [rbp-0x2558],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_daa8), (rax) as u64);
            // 9faec lea rax,[r15+0x198]
            rax = r15.wrapping_add(0x198);
            // 9faf3 mov QWORD PTR [rbp-0x2570],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_da90), (rax) as u64);
            // 9fafa lea rax,[r15+0x168]
            rax = r15.wrapping_add(0x168);
            // 9fb01 mov QWORD PTR [rbp-0x2560],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_daa0), (rax) as u64);
            // 9fb08 lea rax,[r15+0x170]
            rax = r15.wrapping_add(0x170);
            // 9fb0f mov QWORD PTR [rbp-0x2568],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_da98), (rax) as u64);
            // 9fb16 lea rax,[r15+0x1b0]
            rax = r15.wrapping_add(0x1b0);
            // 9fb1d mov QWORD PTR [rbp-0x2578],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_da88), (rax) as u64);
            // 9fb24 mov QWORD PTR [rbp-0x2550],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dab0), (r15) as u64);
            // 9fb2b mov r15,r14
            r15 = r14;
            // 9fb2e xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 9fb31 mov rax,QWORD PTR [rip+0xa0870] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 9fb38 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 9fb3c mov QWORD PTR [rbp-0x2528],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dad8), (rax) as u64);
            // 9fb43 mov rax,QWORD PTR [rip+0xa06be] # 0x140208
            rax = m.r64(m.env.data_va(346581)) as u64;
            // 9fb4a add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 9fb4e mov QWORD PTR [rbp-0x2548],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dab8), (rax) as u64);
            pc = 0x9fb55;
        }
        0x9fb55 => {
            // block 9fb55
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fb55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fb55 mov rax,QWORD PTR [rip+0xa0674] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 9fb5c mov rcx,rax
            rcx = rax;
            // 9fb5f lea rax,[rcx+0x18]
            rax = rcx.wrapping_add(0x18);
            // 9fb63 mov QWORD PTR [rbp-0x2428],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dbd8), (rax) as u64);
            // 9fb6a lea rax,[rcx+0x40]
            rax = rcx.wrapping_add(0x40);
            // 9fb6e mov QWORD PTR [rbp-0x23b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dc48), (rax) as u64);
            // 9fb75 mov rdi,r12
            rdi = r12;
            // 9fb78 mov rsi,r15
            rsi = r15;
            // 9fb7b call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fb80 mov QWORD PTR [rbp-0x2330],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dcd0), (0x0u64) as u64);
            // 9fb8b mov DWORD PTR [rbp-0x2328],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_dcd8), (0xffffffffu64) as u32);
            // 9fb95 mov rax,QWORD PTR [rip+0xa0824] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 9fb9c mov rcx,rax
            rcx = rax;
            // 9fb9f lea rax,[rcx+0x18]
            rax = rcx.wrapping_add(0x18);
            // 9fba3 mov QWORD PTR [rbp-0x2530],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dad0), (rax) as u64);
            // 9fbaa mov QWORD PTR [rbp-0x2428],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dbd8), (rax) as u64);
            // 9fbb1 lea rax,[rcx+0x40]
            rax = rcx.wrapping_add(0x40);
            // 9fbb5 mov QWORD PTR [rbp-0x2538],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dac8), (rax) as u64);
            // 9fbbc mov QWORD PTR [rbp-0x23b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dc48), (rax) as u64);
            // 9fbc3 mov rdi,r15
            rdi = r15;
            // 9fbc6 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fbcb mov rax,QWORD PTR [rbp-0x2528]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dad8)) as u64;
            // 9fbd2 mov QWORD PTR [rbp-0x2420],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dbe0), (rax) as u64);
            // 9fbd9 mov QWORD PTR [r13+0x18],0x0
            m.w64(r13.wrapping_add(0x18), (0x0u64) as u64);
            // 9fbe1 mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // 9fbe9 mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // 9fbf1 mov QWORD PTR [r13+0x0],0x0
            m.w64(r13, (0x0u64) as u64);
            // 9fbf9 mov DWORD PTR [rbp-0x23c0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_dc40), (0x10u64) as u32);
            // 9fc03 xorps xmm0,xmm0
            x0 = 0;
            // 9fc06 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 9fc0a mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 9fc12 mov rdi,r15
            rdi = r15;
            // 9fc15 mov rbx,r15
            rbx = r15;
            // 9fc18 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9fc1c mov rsi,r15
            rsi = r15;
            // 9fc1f call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fc24 mov rdi,r15
            rdi = r15;
            // 9fc27 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fc2c xorps xmm0,xmm0
            x0 = 0;
            // 9fc2f movaps XMMWORD PTR [rbp-0x2440],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_dbc0), x0);
            // 9fc36 mov QWORD PTR [rbp-0x2430],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dbd0), (0x0u64) as u64);
            // 9fc41 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9fc46 lea rdi,[rbp-0x2428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dbd8);
            // 9fc4d lea rsi,[rip+0x980f0] # 0x137d44
            rsi = m.env.data_va(312593);
            // 9fc54 mov r15,rbx
            r15 = rbx;
            // 9fc57 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fc5c lea r12,[rbp-0x2460]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_dba0);
            // 9fc63 mov rdi,rax
            rdi = rax;
            // 9fc66 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 9fc69 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fc6e lea ebx,[r14+0x2710]
            rbx = (r14.wrapping_add(0x2710)) as u32 as u64;
            // 9fc75 mov DWORD PTR [rbp-0x2444],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_dbbc), ((rbx as u32 as u64)) as u32);
            // 9fc7b mov rdi,r12
            rdi = r12;
            // 9fc7e mov rsi,r15
            rsi = r15;
            // 9fc81 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fc86 movzx eax,BYTE PTR [rbp-0x2460]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_dba0)) as u64) as u32 as u64;
            // 9fc8d test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 9fc8f jne 0x9fc96
            if crate::cov::brk(0x9fc8f, fl.ne()) { pc = 0x9fc96; continue; }
            pc = 0x9fc91;
        }
        0x9fc91 => {
            // block 9fc91
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fc91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fc91 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 9fc94 jmp 0x9fc9d
            pc = 0x9fc9d; continue;
        }
        0x9fc96 => {
            // block 9fc96
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fc96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fc96 mov rax,QWORD PTR [rbp-0x2458]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dba8)) as u64;
            pc = 0x9fc9d;
        }
        0x9fc9d => {
            // block 9fc9d
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fc9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fc9d test rax,rax
            fl = logic::<64>(rax & rax);
            // 9fca0 je 0x9fd7e
            if crate::cov::brk(0x9fca0, fl.e()) { pc = 0x9fd7e; continue; }
            pc = 0x9fca6;
        }
        0x9fca6 => {
            // block 9fca6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fca6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fca6 mov rdi,QWORD PTR [rbp-0x2560]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_daa0)) as u64;
            // 9fcad lea rsi,[rbp-0x2444]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_dbbc);
            // 9fcb4 call 0xebc66
            { let r = { let t = m.env.stub_va(0x433); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fcb9 cmp rax,QWORD PTR [rbp-0x2568]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_da98)) as u64).1;
            // 9fcc0 jne 0x9fd7e
            if crate::cov::brk(0x9fcc0, fl.ne()) { pc = 0x9fd7e; continue; }
            pc = 0x9fcc6;
        }
        0x9fcc6 => {
            // block 9fcc6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fcc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fcc6 mov DWORD PTR [rbp-0x2480],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_db80), ((rbx as u32 as u64)) as u32);
            // 9fccc lea rdi,[rbp-0x2478]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_db88);
            // 9fcd3 mov rsi,r12
            rsi = r12;
            // 9fcd6 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fcdb mov rdi,QWORD PTR [rbp-0x2560]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_daa0)) as u64;
            // 9fce2 lea rsi,[rbp-0x2480]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_db80);
            // 9fce9 call 0xebc54
            { let r = { let t = m.env.stub_va(0x421); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fcee lea rdi,[rbp-0x2478]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_db88);
            // 9fcf5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fcfa lea rdi,[rbp-0x24c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_db40);
            // 9fd01 mov rsi,r12
            rsi = r12;
            // 9fd04 call 0xb5094
            { let r = { let t = m.env.code_va(0xb5094); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fd09 mov rax,QWORD PTR [rbp-0x24b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_db50)) as u64;
            // 9fd10 mov QWORD PTR [rbp-0x2490],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_db70), (rax) as u64);
            // 9fd17 mov rax,QWORD PTR [rbp-0x24c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_db40)) as u64;
            // 9fd1e mov rcx,QWORD PTR [rbp-0x24b8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_db48)) as u64;
            // 9fd25 mov QWORD PTR [rbp-0x2498],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_db68), (rcx) as u64);
            // 9fd2c mov QWORD PTR [rbp-0x24a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_db60), (rax) as u64);
            // 9fd33 xorps xmm0,xmm0
            x0 = 0;
            // 9fd36 movaps XMMWORD PTR [rbp-0x24c0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_db40), x0);
            // 9fd3d mov QWORD PTR [rbp-0x24b0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_db50), (0x0u64) as u64);
            // 9fd48 mov DWORD PTR [rbp-0x2488],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_db78), ((rbx as u32 as u64)) as u32);
            // 9fd4e mov r13d,ebx
            r13 = ((rbx as u32 as u64)) as u32 as u64;
            // 9fd51 mov rdi,QWORD PTR [rbp-0x2578]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_da88)) as u64;
            // 9fd58 lea rbx,[rbp-0x24a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_db60);
            // 9fd5f mov rsi,rbx
            rsi = rbx;
            // 9fd62 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fd67 mov rdi,rbx
            rdi = rbx;
            // 9fd6a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fd6f lea rdi,[rbp-0x24c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_db40);
            // 9fd76 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fd7b mov ebx,r13d
            rbx = ((r13 as u32 as u64)) as u32 as u64;
            pc = 0x9fd7e;
        }
        0x9fd7e => {
            // block 9fd7e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fd7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fd7e mov DWORD PTR [rbp-0x24d0],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_db30), ((rbx as u32 as u64)) as u32);
            // 9fd84 mov rax,QWORD PTR [rbp-0x2550]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dab0)) as u64;
            // 9fd8b mov QWORD PTR [rbp-0x24c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_db38), (rax) as u64);
            // 9fd92 mov rdi,QWORD PTR [rbp-0x2558]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_daa8)) as u64;
            // 9fd99 lea rsi,[rbp-0x24d0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_db30);
            // 9fda0 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fda5 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9fdaa lea rdi,[rbp-0x24e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_db18);
            // 9fdb1 lea rsi,[rip+0x97f9e] # 0x137d56
            rsi = m.env.data_va(312611);
            // 9fdb8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fdbd mov DWORD PTR [rbp-0x2540],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_dac0), ((rbx as u32 as u64)) as u32);
            // 9fdc3 movzx ebx,BYTE PTR [rbp-0x24e8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_db18)) as u64) as u32 as u64;
            // 9fdca test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9fdcd jne 0x9fdd4
            if crate::cov::brk(0x9fdcd, fl.ne()) { pc = 0x9fdd4; continue; }
            pc = 0x9fdcf;
        }
        0x9fdcf => {
            // block 9fdcf
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fdcf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fdcf shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9fdd2 jmp 0x9fddb
            pc = 0x9fddb; continue;
        }
        0x9fdd4 => {
            // block 9fdd4
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fdd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fdd4 mov rbx,QWORD PTR [rbp-0x24e0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_db20)) as u64;
            pc = 0x9fddb;
        }
        0x9fddb => {
            // block 9fddb
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fddb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fddb lea rdi,[rbp-0x24e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_db18);
            // 9fde2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fde7 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9fdea je 0x9fe7d
            if crate::cov::brk(0x9fdea, fl.e()) { pc = 0x9fe7d; continue; }
            pc = 0x9fdf0;
        }
        0x9fdf0 => {
            // block 9fdf0
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fdf0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fdf0 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 9fdf5 lea rdi,[rbp-0x2520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dae0);
            // 9fdfc lea rsi,[rip+0x97f53] # 0x137d56
            rsi = m.env.data_va(312611);
            // 9fe03 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fe08 mov edx,DWORD PTR [rbp-0x2540]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_dac0)) as u64) as u32 as u64;
            // 9fe0e mov rax,QWORD PTR [rbp-0x2510]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_daf0)) as u64;
            // 9fe15 mov QWORD PTR [rbp-0x24f8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_db08), (rax) as u64);
            // 9fe1c mov rax,QWORD PTR [rbp-0x2520]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dae0)) as u64;
            // 9fe23 mov rcx,QWORD PTR [rbp-0x2518]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dae8)) as u64;
            // 9fe2a mov QWORD PTR [rbp-0x2500],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_db00), (rcx) as u64);
            // 9fe31 mov QWORD PTR [rbp-0x2508],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_daf8), (rax) as u64);
            // 9fe38 xorps xmm0,xmm0
            x0 = 0;
            // 9fe3b movaps XMMWORD PTR [rbp-0x2520],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_dae0), x0);
            // 9fe42 mov QWORD PTR [rbp-0x2510],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_daf0), (0x0u64) as u64);
            // 9fe4d mov DWORD PTR [rbp-0x24f0],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_db10), ((rdx as u32 as u64)) as u32);
            // 9fe53 mov rdi,QWORD PTR [rbp-0x2570]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_da90)) as u64;
            // 9fe5a lea rbx,[rbp-0x2508]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_daf8);
            // 9fe61 mov rsi,rbx
            rsi = rbx;
            // 9fe64 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fe69 mov rdi,rbx
            rdi = rbx;
            // 9fe6c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fe71 lea rdi,[rbp-0x2520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dae0);
            // 9fe78 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x9fe7d;
        }
        0x9fe7d => {
            // block 9fe7d
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fe7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fe7d mov rdi,r12
            rdi = r12;
            // 9fe80 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fe85 lea rdi,[rbp-0x2440]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dbc0);
            // 9fe8c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fe91 mov rax,QWORD PTR [rbp-0x2530]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dad0)) as u64;
            // 9fe98 mov QWORD PTR [rbp-0x2428],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dbd8), (rax) as u64);
            // 9fe9f mov rax,QWORD PTR [rbp-0x2538]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dac8)) as u64;
            // 9fea6 mov QWORD PTR [rbp-0x23b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dc48), (rax) as u64);
            // 9fead mov rax,QWORD PTR [rbp-0x2528]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dad8)) as u64;
            // 9feb4 mov QWORD PTR [rbp-0x2420],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dbe0), (rax) as u64);
            // 9febb lea r13,[rbp-0x23e0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_dc20);
            // 9fec2 mov rdi,r13
            rdi = r13;
            // 9fec5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9feca mov rdi,r15
            rdi = r15;
            // 9fecd call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fed2 lea rdi,[rbp-0x2428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dbd8);
            // 9fed9 mov rsi,QWORD PTR [rbp-0x2548]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dab8)) as u64;
            // 9fee0 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fee5 lea r12,[rbp-0x23b8]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_dc48);
            // 9feec mov rdi,r12
            rdi = r12;
            // 9feef call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9fef4 inc r14d
            { let (r, f) = inc::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // 9fef7 cmp r14d,0x400
            fl = sub::<32>((r14 as u32 as u64), 0x400u64).1;
            // 9fefe jb 0x9fb55
            if crate::cov::brk(0x9fefe, fl.b()) { pc = 0x9fb55; continue; }
            pc = 0x9ff04;
        }
        0x9ff04 => {
            // block 9ff04
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9ff04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9ff04 add rsp,0x2568
            { let (r, f) = add::<64>(rsp, 0x2568u64);
              rsp = r;
              fl = f; }
            // 9ff0b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 9ff0c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 9ff0e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 9ff10 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 9ff12 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 9ff14 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 9ff15 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x9ff16 => {
            // block 9ff16
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9ff16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9ff16 mov r14,r15
            r14 = r15;
            // 9ff19 mov r13,QWORD PTR [rbp-0x2528]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dad8)) as u64;
            // 9ff20 mov r12,rax
            r12 = rax;
            // 9ff23 jmp 0x9fff8
            pc = 0x9fff8; continue;
        }
        0x9ff28 => {
            // block 9ff28
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9ff28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9ff28 mov r14,r15
            r14 = r15;
            // 9ff2b mov r13,QWORD PTR [rbp-0x2528]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dad8)) as u64;
            // 9ff32 mov r12,rax
            r12 = rax;
            // 9ff35 lea r15,[rbp-0x23e0]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_dc20);
            // 9ff3c mov rbx,QWORD PTR [rbp-0x2530]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dad0)) as u64;
            // 9ff43 jmp 0xa0012
            pc = 0xa0012; continue;
        }
        0x9ff48 => {
            // block 9ff48
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9ff48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9ff48 mov r14,r12
            r14 = r12;
            // 9ff4b mov r12,rax
            r12 = rax;
            // 9ff4e jmp 0x9ff8e
            pc = 0x9ff8e; continue;
        }
        0x9ff50 => {
            // block 9ff50
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9ff50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9ff50 mov r14,r12
            r14 = r12;
            // 9ff53 mov r12,rax
            r12 = rax;
            // 9ff56 jmp 0x9ff7b
            pc = 0x9ff7b; continue;
        }
        0x9ff58 => {
            // block 9ff58
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9ff58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9ff58 mov r14,r12
            r14 = r12;
            // 9ff5b mov r12,rax
            r12 = rax;
            // 9ff5e lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9ff62 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ff67 lea rdi,[rbp-0x23e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dc20);
            // 9ff6e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ff73 mov rdi,rbx
            rdi = rbx;
            // 9ff76 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x9ff7b;
        }
        0x9ff7b => {
            // block 9ff7b
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9ff7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9ff7b lea rdi,[rbp-0x2428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dbd8);
            // 9ff82 mov rsi,QWORD PTR [rbp-0x2548]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dab8)) as u64;
            // 9ff89 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x9ff8e;
        }
        0x9ff8e => {
            // block 9ff8e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9ff8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9ff8e mov rdi,r14
            rdi = r14;
            // 9ff91 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ff96 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0x9ff9b => {
            // block 9ff9b
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9ff9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9ff9b mov r14,r15
            r14 = r15;
            // 9ff9e mov r13,QWORD PTR [rbp-0x2528]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dad8)) as u64;
            // 9ffa5 mov r12,rax
            r12 = rax;
            // 9ffa8 lea rdi,[rbp-0x2508]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_daf8);
            // 9ffaf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ffb4 lea rdi,[rbp-0x2520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dae0);
            // 9ffbb jmp 0x9fff3
            pc = 0x9fff3; continue;
        }
        0x9ffbd => {
            // block 9ffbd
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9ffbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9ffbd mov r14,r15
            r14 = r15;
            // 9ffc0 mov r13,QWORD PTR [rbp-0x2528]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dad8)) as u64;
            // 9ffc7 mov r12,rax
            r12 = rax;
            // 9ffca lea rdi,[rbp-0x2478]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_db88);
            // 9ffd1 jmp 0x9fff3
            pc = 0x9fff3; continue;
        }
        0x9ffd3 => {
            // block 9ffd3
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9ffd3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9ffd3 mov r14,r15
            r14 = r15;
            // 9ffd6 mov r13,QWORD PTR [rbp-0x2528]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dad8)) as u64;
            // 9ffdd mov r12,rax
            r12 = rax;
            // 9ffe0 lea rdi,[rbp-0x24a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_db60);
            // 9ffe7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9ffec lea rdi,[rbp-0x24c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_db40);
            pc = 0x9fff3;
        }
        0x9fff3 => {
            // block 9fff3
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fff3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fff3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x9fff8;
        }
        0x9fff8 => {
            // block 9fff8
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc 9fff8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9fff8 mov rbx,QWORD PTR [rbp-0x2530]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dad0)) as u64;
            // 9ffff lea rdi,[rbp-0x2460]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dba0);
            // a0006 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a000b lea r15,[rbp-0x23e0]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_dc20);
            pc = 0xa0012;
        }
        0xa0012 => {
            // block a0012
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0012 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0012 lea rdi,[rbp-0x2440]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dbc0);
            // a0019 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a001e mov QWORD PTR [rbp-0x2428],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dbd8), (rbx) as u64);
            // a0025 mov rax,QWORD PTR [rbp-0x2538]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dac8)) as u64;
            // a002c mov QWORD PTR [rbp-0x23b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dc48), (rax) as u64);
            // a0033 mov QWORD PTR [rbp-0x2420],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_dbe0), (r13) as u64);
            // a003a mov rdi,r15
            rdi = r15;
            // a003d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0042 mov rdi,r14
            rdi = r14;
            // a0045 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a004a lea rdi,[rbp-0x2428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dbd8);
            // a0051 mov rsi,QWORD PTR [rbp-0x2548]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_dab8)) as u64;
            // a0058 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a005d lea rdi,[rbp-0x23b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dc48);
            // a0064 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xa0069;
        }
        0xa0069 => {
            // block a0069
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0069 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0069 mov rdi,r12
            rdi = r12;
            // a006c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0071 mov r12,rax
            r12 = rax;
            // a0074 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // a0078 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a007d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa007f => {
            // block a007f
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a007f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a007f mov r12,rax
            r12 = rax;
            // a0082 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // a0086 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a008b jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa008d => {
            // block a008d
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a008d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a008d mov r12,rax
            r12 = rax;
            // a0090 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // a0097 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a009c jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa009e => {
            // block a009e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a009e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a009e mov r12,rax
            r12 = rax;
            // a00a1 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // a00a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a00ad jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa00af => {
            // block a00af
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a00af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a00af mov r12,rax
            r12 = rax;
            // a00b2 lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // a00b9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a00be jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa00c0 => {
            // block a00c0
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a00c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a00c0 mov r12,rax
            r12 = rax;
            // a00c3 lea rdi,[rbp-0xd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // a00ca call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a00cf jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa00d1 => {
            // block a00d1
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a00d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a00d1 mov r12,rax
            r12 = rax;
            // a00d4 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a00db call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a00e0 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa00e2 => {
            // block a00e2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a00e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a00e2 mov r12,rax
            r12 = rax;
            // a00e5 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // a00ec call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a00f1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa00f6 => {
            // block a00f6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a00f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a00f6 mov r12,rax
            r12 = rax;
            // a00f9 lea rdi,[rbp-0x118]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // a0100 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0105 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa010a => {
            // block a010a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a010a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a010a mov r12,rax
            r12 = rax;
            // a010d lea rdi,[rbp-0x130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // a0114 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0119 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa011e => {
            // block a011e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a011e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a011e mov r12,rax
            r12 = rax;
            // a0121 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // a0128 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a012d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0132 => {
            // block a0132
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0132 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0132 mov r12,rax
            r12 = rax;
            // a0135 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // a013c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0141 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0146 => {
            // block a0146
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0146 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0146 mov r12,rax
            r12 = rax;
            // a0149 lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // a0150 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0155 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa015a => {
            // block a015a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a015a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a015a mov r12,rax
            r12 = rax;
            // a015d lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // a0164 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0169 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa016e => {
            // block a016e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a016e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a016e mov r12,rax
            r12 = rax;
            // a0171 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // a0178 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a017d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0182 => {
            // block a0182
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0182 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0182 mov r12,rax
            r12 = rax;
            // a0185 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // a018c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0191 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0196 => {
            // block a0196
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0196 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0196 mov r12,rax
            r12 = rax;
            // a0199 lea rdi,[rbp-0x1d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe28);
            // a01a0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a01a5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa01aa => {
            // block a01aa
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a01aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a01aa mov r12,rax
            r12 = rax;
            // a01ad lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // a01b4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a01b9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa01be => {
            // block a01be
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a01be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a01be mov r12,rax
            r12 = rax;
            // a01c1 lea rdi,[rbp-0x208]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf8);
            // a01c8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a01cd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa01d2 => {
            // block a01d2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a01d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a01d2 mov r12,rax
            r12 = rax;
            // a01d5 lea rdi,[rbp-0x220]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde0);
            // a01dc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a01e1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa01e6 => {
            // block a01e6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a01e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a01e6 mov r12,rax
            r12 = rax;
            // a01e9 lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // a01f0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a01f5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa01fa => {
            // block a01fa
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a01fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a01fa mov r12,rax
            r12 = rax;
            // a01fd lea rdi,[rbp-0x250]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdb0);
            // a0204 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0209 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa020e => {
            // block a020e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a020e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a020e mov r12,rax
            r12 = rax;
            // a0211 lea rdi,[rbp-0x268]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd98);
            // a0218 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a021d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0222 => {
            // block a0222
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0222 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0222 mov r12,rax
            r12 = rax;
            // a0225 lea rdi,[rbp-0x280]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd80);
            // a022c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0231 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0236 => {
            // block a0236
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0236 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0236 mov r12,rax
            r12 = rax;
            // a0239 lea rdi,[rbp-0x298]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd68);
            // a0240 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0245 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa024a => {
            // block a024a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a024a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a024a mov r12,rax
            r12 = rax;
            // a024d lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // a0254 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0259 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa025e => {
            // block a025e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a025e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a025e mov r12,rax
            r12 = rax;
            // a0261 lea rdi,[rbp-0x2c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // a0268 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a026d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0272 => {
            // block a0272
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0272 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0272 mov r12,rax
            r12 = rax;
            // a0275 lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // a027c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0281 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0286 => {
            // block a0286
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0286 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0286 mov r12,rax
            r12 = rax;
            // a0289 lea rdi,[rbp-0x2f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd08);
            // a0290 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0295 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa029a => {
            // block a029a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a029a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a029a mov r12,rax
            r12 = rax;
            // a029d lea rdi,[rbp-0x310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf0);
            // a02a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a02a9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa02ae => {
            // block a02ae
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a02ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a02ae mov r12,rax
            r12 = rax;
            // a02b1 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // a02b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a02bd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa02c2 => {
            // block a02c2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a02c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a02c2 mov r12,rax
            r12 = rax;
            // a02c5 lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // a02cc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a02d1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa02d6 => {
            // block a02d6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a02d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a02d6 mov r12,rax
            r12 = rax;
            // a02d9 lea rdi,[rbp-0x358]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fca8);
            // a02e0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a02e5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa02ea => {
            // block a02ea
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a02ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a02ea mov r12,rax
            r12 = rax;
            // a02ed lea rdi,[rbp-0x370]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc90);
            // a02f4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a02f9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa02fe => {
            // block a02fe
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a02fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a02fe mov r12,rax
            r12 = rax;
            // a0301 lea rdi,[rbp-0x388]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc78);
            // a0308 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a030d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0312 => {
            // block a0312
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0312 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0312 mov r12,rax
            r12 = rax;
            // a0315 lea rdi,[rbp-0x3a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc60);
            // a031c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0321 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0326 => {
            // block a0326
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0326 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0326 mov r12,rax
            r12 = rax;
            // a0329 lea rdi,[rbp-0x3b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc48);
            // a0330 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0335 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa033a => {
            // block a033a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a033a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a033a mov r12,rax
            r12 = rax;
            // a033d lea rdi,[rbp-0x3d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc30);
            // a0344 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0349 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa034e => {
            // block a034e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a034e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a034e mov r12,rax
            r12 = rax;
            // a0351 lea rdi,[rbp-0x3e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // a0358 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a035d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0362 => {
            // block a0362
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0362 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0362 mov r12,rax
            r12 = rax;
            // a0365 lea rdi,[rbp-0x400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // a036c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0371 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0376 => {
            // block a0376
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0376 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0376 mov r12,rax
            r12 = rax;
            // a0379 lea rdi,[rbp-0x418]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbe8);
            // a0380 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0385 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa038a => {
            // block a038a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a038a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a038a mov r12,rax
            r12 = rax;
            // a038d lea rdi,[rbp-0x430]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd0);
            // a0394 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0399 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa039e => {
            // block a039e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a039e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a039e mov r12,rax
            r12 = rax;
            // a03a1 lea rdi,[rbp-0x448]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb8);
            // a03a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a03ad jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa03b2 => {
            // block a03b2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a03b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a03b2 mov r12,rax
            r12 = rax;
            // a03b5 lea rdi,[rbp-0x460]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fba0);
            // a03bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a03c1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa03c6 => {
            // block a03c6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a03c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a03c6 mov r12,rax
            r12 = rax;
            // a03c9 lea rdi,[rbp-0x478]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb88);
            // a03d0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a03d5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa03da => {
            // block a03da
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a03da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a03da mov r12,rax
            r12 = rax;
            // a03dd lea rdi,[rbp-0x490]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb70);
            // a03e4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a03e9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa03ee => {
            // block a03ee
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a03ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a03ee mov r12,rax
            r12 = rax;
            // a03f1 lea rdi,[rbp-0x4a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb58);
            // a03f8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a03fd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0402 => {
            // block a0402
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0402 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0402 mov r12,rax
            r12 = rax;
            // a0405 lea rdi,[rbp-0x4c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb40);
            // a040c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0411 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0416 => {
            // block a0416
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0416 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0416 mov r12,rax
            r12 = rax;
            // a0419 lea rdi,[rbp-0x4d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb28);
            // a0420 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0425 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa042a => {
            // block a042a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a042a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a042a mov r12,rax
            r12 = rax;
            // a042d lea rdi,[rbp-0x4f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb10);
            // a0434 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0439 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa043e => {
            // block a043e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a043e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a043e mov r12,rax
            r12 = rax;
            // a0441 lea rdi,[rbp-0x508]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_faf8);
            // a0448 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a044d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0452 => {
            // block a0452
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0452 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0452 mov r12,rax
            r12 = rax;
            // a0455 lea rdi,[rbp-0x520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fae0);
            // a045c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0461 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0466 => {
            // block a0466
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0466 mov r12,rax
            r12 = rax;
            // a0469 lea rdi,[rbp-0x538]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fac8);
            // a0470 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0475 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa047a => {
            // block a047a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a047a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a047a mov r12,rax
            r12 = rax;
            // a047d lea rdi,[rbp-0x550]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fab0);
            // a0484 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0489 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa048e => {
            // block a048e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a048e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a048e mov r12,rax
            r12 = rax;
            // a0491 lea rdi,[rbp-0x568]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa98);
            // a0498 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a049d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa04a2 => {
            // block a04a2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a04a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a04a2 mov r12,rax
            r12 = rax;
            // a04a5 lea rdi,[rbp-0x580]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa80);
            // a04ac call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a04b1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa04b6 => {
            // block a04b6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a04b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a04b6 mov r12,rax
            r12 = rax;
            // a04b9 lea rdi,[rbp-0x598]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa68);
            // a04c0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a04c5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa04ca => {
            // block a04ca
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a04ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a04ca mov r12,rax
            r12 = rax;
            // a04cd lea rdi,[rbp-0x5b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa50);
            // a04d4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a04d9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa04de => {
            // block a04de
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a04de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a04de mov r12,rax
            r12 = rax;
            // a04e1 lea rdi,[rbp-0x5c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa38);
            // a04e8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a04ed jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa04f2 => {
            // block a04f2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a04f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a04f2 mov r12,rax
            r12 = rax;
            // a04f5 lea rdi,[rbp-0x5e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa20);
            // a04fc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0501 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0506 => {
            // block a0506
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0506 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0506 mov r12,rax
            r12 = rax;
            // a0509 lea rdi,[rbp-0x5f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa08);
            // a0510 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0515 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa051a => {
            // block a051a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a051a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a051a mov r12,rax
            r12 = rax;
            // a051d lea rdi,[rbp-0x610]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9f0);
            // a0524 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0529 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa052e => {
            // block a052e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a052e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a052e mov r12,rax
            r12 = rax;
            // a0531 lea rdi,[rbp-0x628]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9d8);
            // a0538 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a053d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0542 => {
            // block a0542
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0542 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0542 mov r12,rax
            r12 = rax;
            // a0545 lea rdi,[rbp-0x640]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9c0);
            // a054c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0551 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0556 => {
            // block a0556
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0556 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0556 mov r12,rax
            r12 = rax;
            // a0559 lea rdi,[rbp-0x658]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9a8);
            // a0560 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0565 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa056a => {
            // block a056a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a056a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a056a mov r12,rax
            r12 = rax;
            // a056d lea rdi,[rbp-0x670]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f990);
            // a0574 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0579 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa057e => {
            // block a057e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a057e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a057e mov r12,rax
            r12 = rax;
            // a0581 lea rdi,[rbp-0x688]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f978);
            // a0588 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a058d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0592 => {
            // block a0592
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0592 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0592 mov r12,rax
            r12 = rax;
            // a0595 lea rdi,[rbp-0x6a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f960);
            // a059c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a05a1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa05a6 => {
            // block a05a6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a05a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a05a6 mov r12,rax
            r12 = rax;
            // a05a9 lea rdi,[rbp-0x6b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f948);
            // a05b0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a05b5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa05ba => {
            // block a05ba
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a05ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a05ba mov r12,rax
            r12 = rax;
            // a05bd lea rdi,[rbp-0x6d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f930);
            // a05c4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a05c9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa05ce => {
            // block a05ce
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a05ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a05ce mov r12,rax
            r12 = rax;
            // a05d1 lea rdi,[rbp-0x6e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f918);
            // a05d8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a05dd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa05e2 => {
            // block a05e2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a05e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a05e2 mov r12,rax
            r12 = rax;
            // a05e5 lea rdi,[rbp-0x700]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f900);
            // a05ec call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a05f1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa05f6 => {
            // block a05f6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a05f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a05f6 mov r12,rax
            r12 = rax;
            // a05f9 lea rdi,[rbp-0x718]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8e8);
            // a0600 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0605 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa060a => {
            // block a060a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a060a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a060a mov r12,rax
            r12 = rax;
            // a060d lea rdi,[rbp-0x730]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8d0);
            // a0614 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0619 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa061e => {
            // block a061e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a061e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a061e mov r12,rax
            r12 = rax;
            // a0621 lea rdi,[rbp-0x748]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8b8);
            // a0628 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a062d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0632 => {
            // block a0632
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0632 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0632 mov r12,rax
            r12 = rax;
            // a0635 lea rdi,[rbp-0x760]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8a0);
            // a063c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0641 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0646 => {
            // block a0646
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0646 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0646 mov r12,rax
            r12 = rax;
            // a0649 lea rdi,[rbp-0x778]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f888);
            // a0650 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0655 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa065a => {
            // block a065a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a065a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a065a mov r12,rax
            r12 = rax;
            // a065d lea rdi,[rbp-0x790]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f870);
            // a0664 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0669 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa066e => {
            // block a066e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a066e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a066e mov r12,rax
            r12 = rax;
            // a0671 lea rdi,[rbp-0x7a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f858);
            // a0678 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a067d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0682 => {
            // block a0682
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0682 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0682 mov r12,rax
            r12 = rax;
            // a0685 lea rdi,[rbp-0x7c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f840);
            // a068c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0691 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0696 => {
            // block a0696
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0696 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0696 mov r12,rax
            r12 = rax;
            // a0699 lea rdi,[rbp-0x7d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f828);
            // a06a0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a06a5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa06aa => {
            // block a06aa
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a06aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a06aa mov r12,rax
            r12 = rax;
            // a06ad lea rdi,[rbp-0x7f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f810);
            // a06b4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a06b9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa06be => {
            // block a06be
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a06be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a06be mov r12,rax
            r12 = rax;
            // a06c1 lea rdi,[rbp-0x808]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7f8);
            // a06c8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a06cd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa06d2 => {
            // block a06d2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a06d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a06d2 mov r12,rax
            r12 = rax;
            // a06d5 lea rdi,[rbp-0x820]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7e0);
            // a06dc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a06e1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa06e6 => {
            // block a06e6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a06e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a06e6 mov r12,rax
            r12 = rax;
            // a06e9 lea rdi,[rbp-0x838]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7c8);
            // a06f0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a06f5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa06fa => {
            // block a06fa
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a06fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a06fa mov r12,rax
            r12 = rax;
            // a06fd lea rdi,[rbp-0x850]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7b0);
            // a0704 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0709 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa070e => {
            // block a070e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a070e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a070e mov r12,rax
            r12 = rax;
            // a0711 lea rdi,[rbp-0x868]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f798);
            // a0718 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a071d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0722 => {
            // block a0722
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0722 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0722 mov r12,rax
            r12 = rax;
            // a0725 lea rdi,[rbp-0x880]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f780);
            // a072c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0731 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0736 => {
            // block a0736
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0736 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0736 mov r12,rax
            r12 = rax;
            // a0739 lea rdi,[rbp-0x898]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f768);
            // a0740 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0745 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa074a => {
            // block a074a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a074a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a074a mov r12,rax
            r12 = rax;
            // a074d lea rdi,[rbp-0x8b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f750);
            // a0754 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0759 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa075e => {
            // block a075e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a075e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a075e mov r12,rax
            r12 = rax;
            // a0761 lea rdi,[rbp-0x8c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f738);
            // a0768 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a076d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0772 => {
            // block a0772
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0772 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0772 mov r12,rax
            r12 = rax;
            // a0775 lea rdi,[rbp-0x8e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f720);
            // a077c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0781 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0786 => {
            // block a0786
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0786 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0786 mov r12,rax
            r12 = rax;
            // a0789 lea rdi,[rbp-0x8f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f708);
            // a0790 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0795 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa079a => {
            // block a079a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a079a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a079a mov r12,rax
            r12 = rax;
            // a079d lea rdi,[rbp-0x910]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6f0);
            // a07a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a07a9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa07ae => {
            // block a07ae
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a07ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a07ae mov r12,rax
            r12 = rax;
            // a07b1 lea rdi,[rbp-0x928]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6d8);
            // a07b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a07bd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa07c2 => {
            // block a07c2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a07c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a07c2 mov r12,rax
            r12 = rax;
            // a07c5 lea rdi,[rbp-0x940]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6c0);
            // a07cc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a07d1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa07d6 => {
            // block a07d6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a07d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a07d6 mov r12,rax
            r12 = rax;
            // a07d9 lea rdi,[rbp-0x958]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6a8);
            // a07e0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a07e5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa07ea => {
            // block a07ea
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a07ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a07ea mov r12,rax
            r12 = rax;
            // a07ed lea rdi,[rbp-0x970]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f690);
            // a07f4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a07f9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa07fe => {
            // block a07fe
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a07fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a07fe mov r12,rax
            r12 = rax;
            // a0801 lea rdi,[rbp-0x988]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f678);
            // a0808 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a080d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0812 => {
            // block a0812
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0812 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0812 mov r12,rax
            r12 = rax;
            // a0815 lea rdi,[rbp-0x9a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f660);
            // a081c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0821 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0826 => {
            // block a0826
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0826 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0826 mov r12,rax
            r12 = rax;
            // a0829 lea rdi,[rbp-0x9b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f648);
            // a0830 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0835 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa083a => {
            // block a083a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a083a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a083a mov r12,rax
            r12 = rax;
            // a083d lea rdi,[rbp-0x9d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f630);
            // a0844 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0849 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa084e => {
            // block a084e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a084e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a084e mov r12,rax
            r12 = rax;
            // a0851 lea rdi,[rbp-0x9e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f618);
            // a0858 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a085d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0862 => {
            // block a0862
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0862 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0862 mov r12,rax
            r12 = rax;
            // a0865 lea rdi,[rbp-0xa00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f600);
            // a086c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0871 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0876 => {
            // block a0876
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0876 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0876 mov r12,rax
            r12 = rax;
            // a0879 lea rdi,[rbp-0xa18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5e8);
            // a0880 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0885 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa088a => {
            // block a088a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a088a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a088a mov r12,rax
            r12 = rax;
            // a088d lea rdi,[rbp-0xa30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5d0);
            // a0894 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0899 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa089e => {
            // block a089e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a089e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a089e mov r12,rax
            r12 = rax;
            // a08a1 lea rdi,[rbp-0xa48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5b8);
            // a08a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a08ad jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa08b2 => {
            // block a08b2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a08b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a08b2 mov r12,rax
            r12 = rax;
            // a08b5 lea rdi,[rbp-0xa60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5a0);
            // a08bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a08c1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa08c6 => {
            // block a08c6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a08c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a08c6 mov r12,rax
            r12 = rax;
            // a08c9 lea rdi,[rbp-0xa78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f588);
            // a08d0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a08d5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa08da => {
            // block a08da
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a08da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a08da mov r12,rax
            r12 = rax;
            // a08dd lea rdi,[rbp-0xa90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f570);
            // a08e4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a08e9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa08ee => {
            // block a08ee
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a08ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a08ee mov r12,rax
            r12 = rax;
            // a08f1 lea rdi,[rbp-0xaa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f558);
            // a08f8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a08fd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0902 => {
            // block a0902
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0902 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0902 mov r12,rax
            r12 = rax;
            // a0905 lea rdi,[rbp-0xac0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f540);
            // a090c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0911 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0916 => {
            // block a0916
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0916 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0916 mov r12,rax
            r12 = rax;
            // a0919 lea rdi,[rbp-0xad8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f528);
            // a0920 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0925 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa092a => {
            // block a092a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a092a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a092a mov r12,rax
            r12 = rax;
            // a092d lea rdi,[rbp-0xaf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f510);
            // a0934 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0939 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa093e => {
            // block a093e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a093e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a093e mov r12,rax
            r12 = rax;
            // a0941 lea rdi,[rbp-0xb08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4f8);
            // a0948 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a094d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0952 => {
            // block a0952
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0952 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0952 mov r12,rax
            r12 = rax;
            // a0955 lea rdi,[rbp-0xb20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4e0);
            // a095c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0961 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0966 => {
            // block a0966
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0966 mov r12,rax
            r12 = rax;
            // a0969 lea rdi,[rbp-0xb38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4c8);
            // a0970 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0975 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa097a => {
            // block a097a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a097a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a097a mov r12,rax
            r12 = rax;
            // a097d lea rdi,[rbp-0xb50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4b0);
            // a0984 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0989 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa098e => {
            // block a098e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a098e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a098e mov r12,rax
            r12 = rax;
            // a0991 lea rdi,[rbp-0xb68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f498);
            // a0998 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a099d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa09a2 => {
            // block a09a2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a09a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a09a2 mov r12,rax
            r12 = rax;
            // a09a5 lea rdi,[rbp-0xb80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f480);
            // a09ac call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a09b1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa09b6 => {
            // block a09b6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a09b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a09b6 mov r12,rax
            r12 = rax;
            // a09b9 lea rdi,[rbp-0xb98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f468);
            // a09c0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a09c5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa09ca => {
            // block a09ca
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a09ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a09ca mov r12,rax
            r12 = rax;
            // a09cd lea rdi,[rbp-0xbb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f450);
            // a09d4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a09d9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa09de => {
            // block a09de
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a09de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a09de mov r12,rax
            r12 = rax;
            // a09e1 lea rdi,[rbp-0xbc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f438);
            // a09e8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a09ed jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa09f2 => {
            // block a09f2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a09f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a09f2 mov r12,rax
            r12 = rax;
            // a09f5 lea rdi,[rbp-0xbe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f420);
            // a09fc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0a01 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0a06 => {
            // block a0a06
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0a06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0a06 mov r12,rax
            r12 = rax;
            // a0a09 lea rdi,[rbp-0xbf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f408);
            // a0a10 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0a15 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0a1a => {
            // block a0a1a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0a1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0a1a mov r12,rax
            r12 = rax;
            // a0a1d lea rdi,[rbp-0xc10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3f0);
            // a0a24 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0a29 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0a2e => {
            // block a0a2e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0a2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0a2e mov r12,rax
            r12 = rax;
            // a0a31 lea rdi,[rbp-0xc28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3d8);
            // a0a38 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0a3d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0a42 => {
            // block a0a42
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0a42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0a42 mov r12,rax
            r12 = rax;
            // a0a45 lea rdi,[rbp-0xc40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3c0);
            // a0a4c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0a51 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0a56 => {
            // block a0a56
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0a56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0a56 mov r12,rax
            r12 = rax;
            // a0a59 lea rdi,[rbp-0xc58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3a8);
            // a0a60 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0a65 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0a6a => {
            // block a0a6a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0a6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0a6a mov r12,rax
            r12 = rax;
            // a0a6d lea rdi,[rbp-0xc70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f390);
            // a0a74 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0a79 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0a7e => {
            // block a0a7e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0a7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0a7e mov r12,rax
            r12 = rax;
            // a0a81 lea rdi,[rbp-0xc88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f378);
            // a0a88 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0a8d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0a92 => {
            // block a0a92
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0a92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0a92 mov r12,rax
            r12 = rax;
            // a0a95 lea rdi,[rbp-0xca0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f360);
            // a0a9c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0aa1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0aa6 => {
            // block a0aa6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0aa6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0aa6 mov r12,rax
            r12 = rax;
            // a0aa9 lea rdi,[rbp-0xcb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f348);
            // a0ab0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0ab5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0aba => {
            // block a0aba
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0aba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0aba mov r12,rax
            r12 = rax;
            // a0abd lea rdi,[rbp-0xcd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f330);
            // a0ac4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0ac9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0ace => {
            // block a0ace
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0ace rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0ace mov r12,rax
            r12 = rax;
            // a0ad1 lea rdi,[rbp-0xce8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f318);
            // a0ad8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0add jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0ae2 => {
            // block a0ae2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0ae2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0ae2 mov r12,rax
            r12 = rax;
            // a0ae5 lea rdi,[rbp-0xd00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f300);
            // a0aec call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0af1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0af6 => {
            // block a0af6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0af6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0af6 mov r12,rax
            r12 = rax;
            // a0af9 lea rdi,[rbp-0xd18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2e8);
            // a0b00 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0b05 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0b0a => {
            // block a0b0a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0b0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0b0a mov r12,rax
            r12 = rax;
            // a0b0d lea rdi,[rbp-0xd30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2d0);
            // a0b14 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0b19 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0b1e => {
            // block a0b1e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0b1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0b1e mov r12,rax
            r12 = rax;
            // a0b21 lea rdi,[rbp-0xd48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2b8);
            // a0b28 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0b2d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0b32 => {
            // block a0b32
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0b32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0b32 mov r12,rax
            r12 = rax;
            // a0b35 lea rdi,[rbp-0xd60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2a0);
            // a0b3c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0b41 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0b46 => {
            // block a0b46
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0b46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0b46 mov r12,rax
            r12 = rax;
            // a0b49 lea rdi,[rbp-0xd78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f288);
            // a0b50 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0b55 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0b5a => {
            // block a0b5a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0b5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0b5a mov r12,rax
            r12 = rax;
            // a0b5d lea rdi,[rbp-0xd90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f270);
            // a0b64 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0b69 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0b6e => {
            // block a0b6e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0b6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0b6e mov r12,rax
            r12 = rax;
            // a0b71 lea rdi,[rbp-0xda8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f258);
            // a0b78 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0b7d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0b82 => {
            // block a0b82
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0b82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0b82 mov r12,rax
            r12 = rax;
            // a0b85 lea rdi,[rbp-0xdc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f240);
            // a0b8c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0b91 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0b96 => {
            // block a0b96
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0b96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0b96 mov r12,rax
            r12 = rax;
            // a0b99 lea rdi,[rbp-0xdd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f228);
            // a0ba0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0ba5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0baa => {
            // block a0baa
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0baa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0baa mov r12,rax
            r12 = rax;
            // a0bad lea rdi,[rbp-0xdf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f210);
            // a0bb4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0bb9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0bbe => {
            // block a0bbe
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0bbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0bbe mov r12,rax
            r12 = rax;
            // a0bc1 lea rdi,[rbp-0xe08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1f8);
            // a0bc8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0bcd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0bd2 => {
            // block a0bd2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0bd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0bd2 mov r12,rax
            r12 = rax;
            // a0bd5 lea rdi,[rbp-0xe20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1e0);
            // a0bdc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0be1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0be6 => {
            // block a0be6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0be6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0be6 mov r12,rax
            r12 = rax;
            // a0be9 lea rdi,[rbp-0xe38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1c8);
            // a0bf0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0bf5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0bfa => {
            // block a0bfa
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0bfa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0bfa mov r12,rax
            r12 = rax;
            // a0bfd lea rdi,[rbp-0xe50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f1b0);
            // a0c04 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0c09 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0c0e => {
            // block a0c0e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0c0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0c0e mov r12,rax
            r12 = rax;
            // a0c11 lea rdi,[rbp-0xe68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f198);
            // a0c18 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0c1d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0c22 => {
            // block a0c22
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0c22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0c22 mov r12,rax
            r12 = rax;
            // a0c25 lea rdi,[rbp-0xe80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f180);
            // a0c2c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0c31 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0c36 => {
            // block a0c36
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0c36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0c36 mov r12,rax
            r12 = rax;
            // a0c39 lea rdi,[rbp-0xe98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f168);
            // a0c40 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0c45 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0c4a => {
            // block a0c4a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0c4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0c4a mov r12,rax
            r12 = rax;
            // a0c4d lea rdi,[rbp-0xeb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f150);
            // a0c54 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0c59 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0c5e => {
            // block a0c5e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0c5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0c5e mov r12,rax
            r12 = rax;
            // a0c61 lea rdi,[rbp-0xec8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f138);
            // a0c68 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0c6d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0c72 => {
            // block a0c72
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0c72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0c72 mov r12,rax
            r12 = rax;
            // a0c75 lea rdi,[rbp-0xee0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f120);
            // a0c7c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0c81 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0c86 => {
            // block a0c86
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0c86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0c86 mov r12,rax
            r12 = rax;
            // a0c89 lea rdi,[rbp-0xef8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f108);
            // a0c90 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0c95 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0c9a => {
            // block a0c9a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0c9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0c9a mov r12,rax
            r12 = rax;
            // a0c9d lea rdi,[rbp-0xf10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0f0);
            // a0ca4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0ca9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0cae => {
            // block a0cae
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0cae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0cae mov r12,rax
            r12 = rax;
            // a0cb1 lea rdi,[rbp-0xf28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0d8);
            // a0cb8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0cbd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0cc2 => {
            // block a0cc2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0cc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0cc2 mov r12,rax
            r12 = rax;
            // a0cc5 lea rdi,[rbp-0xf40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0c0);
            // a0ccc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0cd1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0cd6 => {
            // block a0cd6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0cd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0cd6 mov r12,rax
            r12 = rax;
            // a0cd9 lea rdi,[rbp-0xf58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f0a8);
            // a0ce0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0ce5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0cea => {
            // block a0cea
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0cea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0cea mov r12,rax
            r12 = rax;
            // a0ced lea rdi,[rbp-0xf70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f090);
            // a0cf4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0cf9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0cfe => {
            // block a0cfe
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0cfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0cfe mov r12,rax
            r12 = rax;
            // a0d01 lea rdi,[rbp-0xf88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f078);
            // a0d08 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0d0d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0d12 => {
            // block a0d12
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0d12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0d12 mov r12,rax
            r12 = rax;
            // a0d15 lea rdi,[rbp-0xfa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f060);
            // a0d1c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0d21 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0d26 => {
            // block a0d26
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0d26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0d26 mov r12,rax
            r12 = rax;
            // a0d29 lea rdi,[rbp-0xfb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f048);
            // a0d30 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0d35 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0d3a => {
            // block a0d3a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0d3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0d3a mov r12,rax
            r12 = rax;
            // a0d3d lea rdi,[rbp-0xfd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f030);
            // a0d44 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0d49 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0d4e => {
            // block a0d4e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0d4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0d4e mov r12,rax
            r12 = rax;
            // a0d51 lea rdi,[rbp-0xfe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f018);
            // a0d58 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0d5d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0d62 => {
            // block a0d62
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0d62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0d62 mov r12,rax
            r12 = rax;
            // a0d65 lea rdi,[rbp-0x1000]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f000);
            // a0d6c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0d71 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0d76 => {
            // block a0d76
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0d76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0d76 mov r12,rax
            r12 = rax;
            // a0d79 lea rdi,[rbp-0x1018]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efe8);
            // a0d80 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0d85 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0d8a => {
            // block a0d8a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0d8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0d8a mov r12,rax
            r12 = rax;
            // a0d8d lea rdi,[rbp-0x1030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efd0);
            // a0d94 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0d99 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0d9e => {
            // block a0d9e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0d9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0d9e mov r12,rax
            r12 = rax;
            // a0da1 lea rdi,[rbp-0x1048]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efb8);
            // a0da8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0dad jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0db2 => {
            // block a0db2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0db2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0db2 mov r12,rax
            r12 = rax;
            // a0db5 lea rdi,[rbp-0x1060]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_efa0);
            // a0dbc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0dc1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0dc6 => {
            // block a0dc6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0dc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0dc6 mov r12,rax
            r12 = rax;
            // a0dc9 lea rdi,[rbp-0x1078]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef88);
            // a0dd0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0dd5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0dda => {
            // block a0dda
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0dda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0dda mov r12,rax
            r12 = rax;
            // a0ddd lea rdi,[rbp-0x1090]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef70);
            // a0de4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0de9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0dee => {
            // block a0dee
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0dee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0dee mov r12,rax
            r12 = rax;
            // a0df1 lea rdi,[rbp-0x10a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef58);
            // a0df8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0dfd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0e02 => {
            // block a0e02
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0e02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0e02 mov r12,rax
            r12 = rax;
            // a0e05 lea rdi,[rbp-0x10c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef40);
            // a0e0c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0e11 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0e16 => {
            // block a0e16
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0e16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0e16 mov r12,rax
            r12 = rax;
            // a0e19 lea rdi,[rbp-0x10d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef28);
            // a0e20 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0e25 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0e2a => {
            // block a0e2a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0e2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0e2a mov r12,rax
            r12 = rax;
            // a0e2d lea rdi,[rbp-0x10f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ef10);
            // a0e34 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0e39 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0e3e => {
            // block a0e3e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0e3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0e3e mov r12,rax
            r12 = rax;
            // a0e41 lea rdi,[rbp-0x1108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eef8);
            // a0e48 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0e4d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0e52 => {
            // block a0e52
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0e52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0e52 mov r12,rax
            r12 = rax;
            // a0e55 lea rdi,[rbp-0x1120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eee0);
            // a0e5c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0e61 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0e66 => {
            // block a0e66
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0e66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0e66 mov r12,rax
            r12 = rax;
            // a0e69 lea rdi,[rbp-0x1138]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eec8);
            // a0e70 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0e75 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0e7a => {
            // block a0e7a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0e7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0e7a mov r12,rax
            r12 = rax;
            // a0e7d lea rdi,[rbp-0x1150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eeb0);
            // a0e84 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0e89 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0e8e => {
            // block a0e8e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0e8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0e8e mov r12,rax
            r12 = rax;
            // a0e91 lea rdi,[rbp-0x1168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee98);
            // a0e98 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0e9d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0ea2 => {
            // block a0ea2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0ea2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0ea2 mov r12,rax
            r12 = rax;
            // a0ea5 lea rdi,[rbp-0x1180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee80);
            // a0eac call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0eb1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0eb6 => {
            // block a0eb6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0eb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0eb6 mov r12,rax
            r12 = rax;
            // a0eb9 lea rdi,[rbp-0x1198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee68);
            // a0ec0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0ec5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0eca => {
            // block a0eca
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0eca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0eca mov r12,rax
            r12 = rax;
            // a0ecd lea rdi,[rbp-0x11b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee50);
            // a0ed4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0ed9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0ede => {
            // block a0ede
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0ede rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0ede mov r12,rax
            r12 = rax;
            // a0ee1 lea rdi,[rbp-0x11c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee38);
            // a0ee8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0eed jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0ef2 => {
            // block a0ef2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0ef2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0ef2 mov r12,rax
            r12 = rax;
            // a0ef5 lea rdi,[rbp-0x11e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee20);
            // a0efc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0f01 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0f06 => {
            // block a0f06
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0f06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0f06 mov r12,rax
            r12 = rax;
            // a0f09 lea rdi,[rbp-0x11f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ee08);
            // a0f10 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0f15 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0f1a => {
            // block a0f1a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0f1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0f1a mov r12,rax
            r12 = rax;
            // a0f1d lea rdi,[rbp-0x1210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edf0);
            // a0f24 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0f29 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0f2e => {
            // block a0f2e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0f2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0f2e mov r12,rax
            r12 = rax;
            // a0f31 lea rdi,[rbp-0x1228]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edd8);
            // a0f38 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0f3d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0f42 => {
            // block a0f42
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0f42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0f42 mov r12,rax
            r12 = rax;
            // a0f45 lea rdi,[rbp-0x1240]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_edc0);
            // a0f4c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0f51 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0f56 => {
            // block a0f56
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0f56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0f56 mov r12,rax
            r12 = rax;
            // a0f59 lea rdi,[rbp-0x1258]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eda8);
            // a0f60 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0f65 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0f6a => {
            // block a0f6a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0f6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0f6a mov r12,rax
            r12 = rax;
            // a0f6d lea rdi,[rbp-0x1270]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed90);
            // a0f74 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0f79 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0f7e => {
            // block a0f7e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0f7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0f7e mov r12,rax
            r12 = rax;
            // a0f81 lea rdi,[rbp-0x1288]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed78);
            // a0f88 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0f8d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0f92 => {
            // block a0f92
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0f92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0f92 mov r12,rax
            r12 = rax;
            // a0f95 lea rdi,[rbp-0x12a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed60);
            // a0f9c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0fa1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0fa6 => {
            // block a0fa6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0fa6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0fa6 mov r12,rax
            r12 = rax;
            // a0fa9 lea rdi,[rbp-0x12b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed48);
            // a0fb0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0fb5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0fba => {
            // block a0fba
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0fba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0fba mov r12,rax
            r12 = rax;
            // a0fbd lea rdi,[rbp-0x12d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed30);
            // a0fc4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0fc9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0fce => {
            // block a0fce
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0fce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0fce mov r12,rax
            r12 = rax;
            // a0fd1 lea rdi,[rbp-0x12e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed18);
            // a0fd8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0fdd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0fe2 => {
            // block a0fe2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0fe2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0fe2 mov r12,rax
            r12 = rax;
            // a0fe5 lea rdi,[rbp-0x1300]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ed00);
            // a0fec call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a0ff1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa0ff6 => {
            // block a0ff6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a0ff6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a0ff6 mov r12,rax
            r12 = rax;
            // a0ff9 lea rdi,[rbp-0x1318]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ece8);
            // a1000 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1005 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa100a => {
            // block a100a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a100a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a100a mov r12,rax
            r12 = rax;
            // a100d lea rdi,[rbp-0x1330]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecd0);
            // a1014 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1019 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa101e => {
            // block a101e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a101e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a101e mov r12,rax
            r12 = rax;
            // a1021 lea rdi,[rbp-0x1348]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ecb8);
            // a1028 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a102d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1032 => {
            // block a1032
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1032 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1032 mov r12,rax
            r12 = rax;
            // a1035 lea rdi,[rbp-0x1360]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eca0);
            // a103c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1041 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1046 => {
            // block a1046
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1046 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1046 mov r12,rax
            r12 = rax;
            // a1049 lea rdi,[rbp-0x1378]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec88);
            // a1050 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1055 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa105a => {
            // block a105a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a105a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a105a mov r12,rax
            r12 = rax;
            // a105d lea rdi,[rbp-0x1390]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec70);
            // a1064 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1069 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa106e => {
            // block a106e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a106e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a106e mov r12,rax
            r12 = rax;
            // a1071 lea rdi,[rbp-0x13a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec58);
            // a1078 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a107d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1082 => {
            // block a1082
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1082 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1082 mov r12,rax
            r12 = rax;
            // a1085 lea rdi,[rbp-0x13c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec40);
            // a108c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1091 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1096 => {
            // block a1096
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1096 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1096 mov r12,rax
            r12 = rax;
            // a1099 lea rdi,[rbp-0x13d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec28);
            // a10a0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a10a5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa10aa => {
            // block a10aa
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a10aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a10aa mov r12,rax
            r12 = rax;
            // a10ad lea rdi,[rbp-0x13f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ec10);
            // a10b4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a10b9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa10be => {
            // block a10be
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a10be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a10be mov r12,rax
            r12 = rax;
            // a10c1 lea rdi,[rbp-0x1408]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebf8);
            // a10c8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a10cd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa10d2 => {
            // block a10d2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a10d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a10d2 mov r12,rax
            r12 = rax;
            // a10d5 lea rdi,[rbp-0x1420]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebe0);
            // a10dc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a10e1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa10e6 => {
            // block a10e6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a10e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a10e6 mov r12,rax
            r12 = rax;
            // a10e9 lea rdi,[rbp-0x1438]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebc8);
            // a10f0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a10f5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa10fa => {
            // block a10fa
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a10fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a10fa mov r12,rax
            r12 = rax;
            // a10fd lea rdi,[rbp-0x1450]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ebb0);
            // a1104 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1109 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa110e => {
            // block a110e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a110e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a110e mov r12,rax
            r12 = rax;
            // a1111 lea rdi,[rbp-0x1468]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb98);
            // a1118 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a111d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1122 => {
            // block a1122
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1122 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1122 mov r12,rax
            r12 = rax;
            // a1125 lea rdi,[rbp-0x1480]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb80);
            // a112c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1131 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1136 => {
            // block a1136
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1136 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1136 mov r12,rax
            r12 = rax;
            // a1139 lea rdi,[rbp-0x1498]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb68);
            // a1140 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1145 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa114a => {
            // block a114a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a114a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a114a mov r12,rax
            r12 = rax;
            // a114d lea rdi,[rbp-0x14b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb50);
            // a1154 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1159 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa115e => {
            // block a115e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a115e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a115e mov r12,rax
            r12 = rax;
            // a1161 lea rdi,[rbp-0x14c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb38);
            // a1168 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a116d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1172 => {
            // block a1172
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1172 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1172 mov r12,rax
            r12 = rax;
            // a1175 lea rdi,[rbp-0x14e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb20);
            // a117c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1181 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1186 => {
            // block a1186
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1186 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1186 mov r12,rax
            r12 = rax;
            // a1189 lea rdi,[rbp-0x14f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eb08);
            // a1190 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1195 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa119a => {
            // block a119a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a119a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a119a mov r12,rax
            r12 = rax;
            // a119d lea rdi,[rbp-0x1510]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eaf0);
            // a11a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a11a9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa11ae => {
            // block a11ae
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a11ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a11ae mov r12,rax
            r12 = rax;
            // a11b1 lea rdi,[rbp-0x1528]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ead8);
            // a11b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a11bd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa11c2 => {
            // block a11c2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a11c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a11c2 mov r12,rax
            r12 = rax;
            // a11c5 lea rdi,[rbp-0x1540]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eac0);
            // a11cc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a11d1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa11d6 => {
            // block a11d6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a11d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a11d6 mov r12,rax
            r12 = rax;
            // a11d9 lea rdi,[rbp-0x1558]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_eaa8);
            // a11e0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a11e5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa11ea => {
            // block a11ea
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a11ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a11ea mov r12,rax
            r12 = rax;
            // a11ed lea rdi,[rbp-0x1570]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea90);
            // a11f4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a11f9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa11fe => {
            // block a11fe
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a11fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a11fe mov r12,rax
            r12 = rax;
            // a1201 lea rdi,[rbp-0x1588]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea78);
            // a1208 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a120d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1212 => {
            // block a1212
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1212 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1212 mov r12,rax
            r12 = rax;
            // a1215 lea rdi,[rbp-0x15a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea60);
            // a121c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1221 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1226 => {
            // block a1226
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1226 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1226 mov r12,rax
            r12 = rax;
            // a1229 lea rdi,[rbp-0x15b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea48);
            // a1230 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1235 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa123a => {
            // block a123a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a123a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a123a mov r12,rax
            r12 = rax;
            // a123d lea rdi,[rbp-0x15d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea30);
            // a1244 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1249 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa124e => {
            // block a124e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a124e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a124e mov r12,rax
            r12 = rax;
            // a1251 lea rdi,[rbp-0x15e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea18);
            // a1258 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a125d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1262 => {
            // block a1262
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1262 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1262 mov r12,rax
            r12 = rax;
            // a1265 lea rdi,[rbp-0x1600]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ea00);
            // a126c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1271 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1276 => {
            // block a1276
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1276 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1276 mov r12,rax
            r12 = rax;
            // a1279 lea rdi,[rbp-0x1618]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9e8);
            // a1280 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1285 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa128a => {
            // block a128a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a128a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a128a mov r12,rax
            r12 = rax;
            // a128d lea rdi,[rbp-0x1630]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9d0);
            // a1294 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1299 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa129e => {
            // block a129e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a129e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a129e mov r12,rax
            r12 = rax;
            // a12a1 lea rdi,[rbp-0x1648]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9b8);
            // a12a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a12ad jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa12b2 => {
            // block a12b2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a12b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a12b2 mov r12,rax
            r12 = rax;
            // a12b5 lea rdi,[rbp-0x1660]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e9a0);
            // a12bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a12c1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa12c6 => {
            // block a12c6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a12c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a12c6 mov r12,rax
            r12 = rax;
            // a12c9 lea rdi,[rbp-0x1678]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e988);
            // a12d0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a12d5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa12da => {
            // block a12da
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a12da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a12da mov r12,rax
            r12 = rax;
            // a12dd lea rdi,[rbp-0x1690]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e970);
            // a12e4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a12e9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa12ee => {
            // block a12ee
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a12ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a12ee mov r12,rax
            r12 = rax;
            // a12f1 lea rdi,[rbp-0x16a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e958);
            // a12f8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a12fd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1302 => {
            // block a1302
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1302 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1302 mov r12,rax
            r12 = rax;
            // a1305 lea rdi,[rbp-0x16c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e940);
            // a130c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1311 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1316 => {
            // block a1316
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1316 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1316 mov r12,rax
            r12 = rax;
            // a1319 lea rdi,[rbp-0x16d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e928);
            // a1320 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1325 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa132a => {
            // block a132a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a132a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a132a mov r12,rax
            r12 = rax;
            // a132d lea rdi,[rbp-0x16f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e910);
            // a1334 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1339 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa133e => {
            // block a133e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a133e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a133e mov r12,rax
            r12 = rax;
            // a1341 lea rdi,[rbp-0x1708]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8f8);
            // a1348 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a134d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1352 => {
            // block a1352
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1352 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1352 mov r12,rax
            r12 = rax;
            // a1355 lea rdi,[rbp-0x1720]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8e0);
            // a135c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1361 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1366 => {
            // block a1366
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1366 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1366 mov r12,rax
            r12 = rax;
            // a1369 lea rdi,[rbp-0x1738]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8c8);
            // a1370 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1375 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa137a => {
            // block a137a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a137a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a137a mov r12,rax
            r12 = rax;
            // a137d lea rdi,[rbp-0x1750]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e8b0);
            // a1384 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1389 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa138e => {
            // block a138e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a138e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a138e mov r12,rax
            r12 = rax;
            // a1391 lea rdi,[rbp-0x1768]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e898);
            // a1398 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a139d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa13a2 => {
            // block a13a2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a13a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a13a2 mov r12,rax
            r12 = rax;
            // a13a5 lea rdi,[rbp-0x1780]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e880);
            // a13ac call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a13b1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa13b6 => {
            // block a13b6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a13b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a13b6 mov r12,rax
            r12 = rax;
            // a13b9 lea rdi,[rbp-0x1798]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e868);
            // a13c0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a13c5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa13ca => {
            // block a13ca
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a13ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a13ca mov r12,rax
            r12 = rax;
            // a13cd lea rdi,[rbp-0x17b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e850);
            // a13d4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a13d9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa13de => {
            // block a13de
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a13de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a13de mov r12,rax
            r12 = rax;
            // a13e1 lea rdi,[rbp-0x17c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e838);
            // a13e8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a13ed jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa13f2 => {
            // block a13f2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a13f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a13f2 mov r12,rax
            r12 = rax;
            // a13f5 lea rdi,[rbp-0x17e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e820);
            // a13fc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1401 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1406 => {
            // block a1406
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1406 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1406 mov r12,rax
            r12 = rax;
            // a1409 lea rdi,[rbp-0x17f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e808);
            // a1410 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1415 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa141a => {
            // block a141a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a141a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a141a mov r12,rax
            r12 = rax;
            // a141d lea rdi,[rbp-0x1810]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7f0);
            // a1424 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1429 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa142e => {
            // block a142e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a142e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a142e mov r12,rax
            r12 = rax;
            // a1431 lea rdi,[rbp-0x1828]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7d8);
            // a1438 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a143d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1442 => {
            // block a1442
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1442 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1442 mov r12,rax
            r12 = rax;
            // a1445 lea rdi,[rbp-0x1840]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7c0);
            // a144c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1451 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1456 => {
            // block a1456
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1456 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1456 mov r12,rax
            r12 = rax;
            // a1459 lea rdi,[rbp-0x1858]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e7a8);
            // a1460 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1465 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa146a => {
            // block a146a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a146a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a146a mov r12,rax
            r12 = rax;
            // a146d lea rdi,[rbp-0x1870]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e790);
            // a1474 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1479 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa147e => {
            // block a147e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a147e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a147e mov r12,rax
            r12 = rax;
            // a1481 lea rdi,[rbp-0x1888]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e778);
            // a1488 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a148d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1492 => {
            // block a1492
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1492 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1492 mov r12,rax
            r12 = rax;
            // a1495 lea rdi,[rbp-0x18a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e760);
            // a149c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a14a1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa14a6 => {
            // block a14a6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a14a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a14a6 mov r12,rax
            r12 = rax;
            // a14a9 lea rdi,[rbp-0x18b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e748);
            // a14b0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a14b5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa14ba => {
            // block a14ba
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a14ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a14ba mov r12,rax
            r12 = rax;
            // a14bd lea rdi,[rbp-0x18d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e730);
            // a14c4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a14c9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa14ce => {
            // block a14ce
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a14ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a14ce mov r12,rax
            r12 = rax;
            // a14d1 lea rdi,[rbp-0x18e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e718);
            // a14d8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a14dd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa14e2 => {
            // block a14e2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a14e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a14e2 mov r12,rax
            r12 = rax;
            // a14e5 lea rdi,[rbp-0x1900]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e700);
            // a14ec call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a14f1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa14f6 => {
            // block a14f6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a14f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a14f6 mov r12,rax
            r12 = rax;
            // a14f9 lea rdi,[rbp-0x1918]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6e8);
            // a1500 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1505 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa150a => {
            // block a150a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a150a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a150a mov r12,rax
            r12 = rax;
            // a150d lea rdi,[rbp-0x1930]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6d0);
            // a1514 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1519 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa151e => {
            // block a151e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a151e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a151e mov r12,rax
            r12 = rax;
            // a1521 lea rdi,[rbp-0x1948]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6b8);
            // a1528 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a152d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1532 => {
            // block a1532
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1532 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1532 mov r12,rax
            r12 = rax;
            // a1535 lea rdi,[rbp-0x1960]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e6a0);
            // a153c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1541 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1546 => {
            // block a1546
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1546 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1546 mov r12,rax
            r12 = rax;
            // a1549 lea rdi,[rbp-0x1978]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e688);
            // a1550 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1555 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa155a => {
            // block a155a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a155a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a155a mov r12,rax
            r12 = rax;
            // a155d lea rdi,[rbp-0x1990]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e670);
            // a1564 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1569 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa156e => {
            // block a156e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a156e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a156e mov r12,rax
            r12 = rax;
            // a1571 lea rdi,[rbp-0x19a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e658);
            // a1578 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a157d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1582 => {
            // block a1582
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1582 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1582 mov r12,rax
            r12 = rax;
            // a1585 lea rdi,[rbp-0x19c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e640);
            // a158c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1591 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1596 => {
            // block a1596
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1596 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1596 mov r12,rax
            r12 = rax;
            // a1599 lea rdi,[rbp-0x19d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e628);
            // a15a0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a15a5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa15aa => {
            // block a15aa
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a15aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a15aa mov r12,rax
            r12 = rax;
            // a15ad lea rdi,[rbp-0x19f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e610);
            // a15b4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a15b9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa15be => {
            // block a15be
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a15be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a15be mov r12,rax
            r12 = rax;
            // a15c1 lea rdi,[rbp-0x1a08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e5f8);
            // a15c8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a15cd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa15d2 => {
            // block a15d2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a15d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a15d2 mov r12,rax
            r12 = rax;
            // a15d5 lea rdi,[rbp-0x1a20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e5e0);
            // a15dc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a15e1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa15e6 => {
            // block a15e6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a15e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a15e6 mov r12,rax
            r12 = rax;
            // a15e9 lea rdi,[rbp-0x1a38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e5c8);
            // a15f0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a15f5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa15fa => {
            // block a15fa
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a15fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a15fa mov r12,rax
            r12 = rax;
            // a15fd lea rdi,[rbp-0x1a50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e5b0);
            // a1604 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1609 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa160e => {
            // block a160e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a160e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a160e mov r12,rax
            r12 = rax;
            // a1611 lea rdi,[rbp-0x1a68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e598);
            // a1618 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a161d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1622 => {
            // block a1622
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1622 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1622 mov r12,rax
            r12 = rax;
            // a1625 lea rdi,[rbp-0x1a80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e580);
            // a162c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1631 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1636 => {
            // block a1636
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1636 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1636 mov r12,rax
            r12 = rax;
            // a1639 lea rdi,[rbp-0x1a98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e568);
            // a1640 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1645 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa164a => {
            // block a164a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a164a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a164a mov r12,rax
            r12 = rax;
            // a164d lea rdi,[rbp-0x1ab0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e550);
            // a1654 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1659 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa165e => {
            // block a165e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a165e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a165e mov r12,rax
            r12 = rax;
            // a1661 lea rdi,[rbp-0x1ac8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e538);
            // a1668 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a166d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1672 => {
            // block a1672
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1672 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1672 mov r12,rax
            r12 = rax;
            // a1675 lea rdi,[rbp-0x1ae0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e520);
            // a167c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1681 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1686 => {
            // block a1686
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1686 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1686 mov r12,rax
            r12 = rax;
            // a1689 lea rdi,[rbp-0x1af8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e508);
            // a1690 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1695 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa169a => {
            // block a169a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a169a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a169a mov r12,rax
            r12 = rax;
            // a169d lea rdi,[rbp-0x1b10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e4f0);
            // a16a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a16a9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa16ae => {
            // block a16ae
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a16ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a16ae mov r12,rax
            r12 = rax;
            // a16b1 lea rdi,[rbp-0x1b28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e4d8);
            // a16b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a16bd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa16c2 => {
            // block a16c2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a16c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a16c2 mov r12,rax
            r12 = rax;
            // a16c5 lea rdi,[rbp-0x1b40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e4c0);
            // a16cc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a16d1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa16d6 => {
            // block a16d6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a16d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a16d6 mov r12,rax
            r12 = rax;
            // a16d9 lea rdi,[rbp-0x1b58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e4a8);
            // a16e0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a16e5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa16ea => {
            // block a16ea
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a16ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a16ea mov r12,rax
            r12 = rax;
            // a16ed lea rdi,[rbp-0x1b70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e490);
            // a16f4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a16f9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa16fe => {
            // block a16fe
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a16fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a16fe mov r12,rax
            r12 = rax;
            // a1701 lea rdi,[rbp-0x1b88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e478);
            // a1708 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a170d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1712 => {
            // block a1712
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1712 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1712 mov r12,rax
            r12 = rax;
            // a1715 lea rdi,[rbp-0x1ba0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e460);
            // a171c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1721 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1726 => {
            // block a1726
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1726 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1726 mov r12,rax
            r12 = rax;
            // a1729 lea rdi,[rbp-0x1bb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e448);
            // a1730 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1735 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa173a => {
            // block a173a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a173a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a173a mov r12,rax
            r12 = rax;
            // a173d lea rdi,[rbp-0x1bd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e430);
            // a1744 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1749 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa174e => {
            // block a174e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a174e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a174e mov r12,rax
            r12 = rax;
            // a1751 lea rdi,[rbp-0x1be8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e418);
            // a1758 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a175d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1762 => {
            // block a1762
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1762 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1762 mov r12,rax
            r12 = rax;
            // a1765 lea rdi,[rbp-0x1c00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e400);
            // a176c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1771 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1776 => {
            // block a1776
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1776 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1776 mov r12,rax
            r12 = rax;
            // a1779 lea rdi,[rbp-0x1c18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e3e8);
            // a1780 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1785 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa178a => {
            // block a178a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a178a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a178a mov r12,rax
            r12 = rax;
            // a178d lea rdi,[rbp-0x1c30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e3d0);
            // a1794 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1799 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa179e => {
            // block a179e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a179e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a179e mov r12,rax
            r12 = rax;
            // a17a1 lea rdi,[rbp-0x1c48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e3b8);
            // a17a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a17ad jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa17b2 => {
            // block a17b2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a17b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a17b2 mov r12,rax
            r12 = rax;
            // a17b5 lea rdi,[rbp-0x1c60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e3a0);
            // a17bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a17c1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa17c6 => {
            // block a17c6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a17c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a17c6 mov r12,rax
            r12 = rax;
            // a17c9 lea rdi,[rbp-0x1c78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e388);
            // a17d0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a17d5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa17da => {
            // block a17da
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a17da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a17da mov r12,rax
            r12 = rax;
            // a17dd lea rdi,[rbp-0x1c90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e370);
            // a17e4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a17e9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa17ee => {
            // block a17ee
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a17ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a17ee mov r12,rax
            r12 = rax;
            // a17f1 lea rdi,[rbp-0x1ca8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e358);
            // a17f8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a17fd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1802 => {
            // block a1802
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1802 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1802 mov r12,rax
            r12 = rax;
            // a1805 lea rdi,[rbp-0x1cc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e340);
            // a180c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1811 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1816 => {
            // block a1816
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1816 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1816 mov r12,rax
            r12 = rax;
            // a1819 lea rdi,[rbp-0x1cd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e328);
            // a1820 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1825 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa182a => {
            // block a182a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a182a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a182a mov r12,rax
            r12 = rax;
            // a182d lea rdi,[rbp-0x1cf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e310);
            // a1834 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1839 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa183e => {
            // block a183e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a183e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a183e mov r12,rax
            r12 = rax;
            // a1841 lea rdi,[rbp-0x1d08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e2f8);
            // a1848 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a184d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1852 => {
            // block a1852
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1852 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1852 mov r12,rax
            r12 = rax;
            // a1855 lea rdi,[rbp-0x1d20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e2e0);
            // a185c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1861 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1866 => {
            // block a1866
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1866 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1866 mov r12,rax
            r12 = rax;
            // a1869 lea rdi,[rbp-0x1d38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e2c8);
            // a1870 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1875 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa187a => {
            // block a187a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a187a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a187a mov r12,rax
            r12 = rax;
            // a187d lea rdi,[rbp-0x1d50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e2b0);
            // a1884 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1889 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa188e => {
            // block a188e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a188e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a188e mov r12,rax
            r12 = rax;
            // a1891 lea rdi,[rbp-0x1d68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e298);
            // a1898 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a189d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa18a2 => {
            // block a18a2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a18a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a18a2 mov r12,rax
            r12 = rax;
            // a18a5 lea rdi,[rbp-0x1d80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e280);
            // a18ac call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a18b1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa18b6 => {
            // block a18b6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a18b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a18b6 mov r12,rax
            r12 = rax;
            // a18b9 lea rdi,[rbp-0x1d98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e268);
            // a18c0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a18c5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa18ca => {
            // block a18ca
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a18ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a18ca mov r12,rax
            r12 = rax;
            // a18cd lea rdi,[rbp-0x1db0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e250);
            // a18d4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a18d9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa18de => {
            // block a18de
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a18de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a18de mov r12,rax
            r12 = rax;
            // a18e1 lea rdi,[rbp-0x1dc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e238);
            // a18e8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a18ed jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa18f2 => {
            // block a18f2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a18f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a18f2 mov r12,rax
            r12 = rax;
            // a18f5 lea rdi,[rbp-0x1de0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e220);
            // a18fc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1901 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1906 => {
            // block a1906
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1906 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1906 mov r12,rax
            r12 = rax;
            // a1909 lea rdi,[rbp-0x1df8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e208);
            // a1910 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1915 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa191a => {
            // block a191a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a191a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a191a mov r12,rax
            r12 = rax;
            // a191d lea rdi,[rbp-0x1e10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e1f0);
            // a1924 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1929 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa192e => {
            // block a192e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a192e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a192e mov r12,rax
            r12 = rax;
            // a1931 lea rdi,[rbp-0x1e28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e1d8);
            // a1938 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a193d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1942 => {
            // block a1942
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1942 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1942 mov r12,rax
            r12 = rax;
            // a1945 lea rdi,[rbp-0x1e40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e1c0);
            // a194c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1951 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1956 => {
            // block a1956
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1956 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1956 mov r12,rax
            r12 = rax;
            // a1959 lea rdi,[rbp-0x1e58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e1a8);
            // a1960 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1965 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa196a => {
            // block a196a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a196a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a196a mov r12,rax
            r12 = rax;
            // a196d lea rdi,[rbp-0x1e70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e190);
            // a1974 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1979 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa197e => {
            // block a197e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a197e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a197e mov r12,rax
            r12 = rax;
            // a1981 lea rdi,[rbp-0x1e88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e178);
            // a1988 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a198d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1992 => {
            // block a1992
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1992 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1992 mov r12,rax
            r12 = rax;
            // a1995 lea rdi,[rbp-0x1ea0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e160);
            // a199c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a19a1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa19a6 => {
            // block a19a6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a19a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a19a6 mov r12,rax
            r12 = rax;
            // a19a9 lea rdi,[rbp-0x1eb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e148);
            // a19b0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a19b5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa19ba => {
            // block a19ba
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a19ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a19ba mov r12,rax
            r12 = rax;
            // a19bd lea rdi,[rbp-0x1ed0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e130);
            // a19c4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a19c9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa19ce => {
            // block a19ce
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a19ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a19ce mov r12,rax
            r12 = rax;
            // a19d1 lea rdi,[rbp-0x1ee8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e118);
            // a19d8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a19dd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa19e2 => {
            // block a19e2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a19e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a19e2 mov r12,rax
            r12 = rax;
            // a19e5 lea rdi,[rbp-0x1f00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e100);
            // a19ec call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a19f1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa19f6 => {
            // block a19f6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a19f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a19f6 mov r12,rax
            r12 = rax;
            // a19f9 lea rdi,[rbp-0x1f18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e0e8);
            // a1a00 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1a05 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1a0a => {
            // block a1a0a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1a0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1a0a mov r12,rax
            r12 = rax;
            // a1a0d lea rdi,[rbp-0x1f30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e0d0);
            // a1a14 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1a19 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1a1e => {
            // block a1a1e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1a1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1a1e mov r12,rax
            r12 = rax;
            // a1a21 lea rdi,[rbp-0x1f48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e0b8);
            // a1a28 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1a2d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1a32 => {
            // block a1a32
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1a32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1a32 mov r12,rax
            r12 = rax;
            // a1a35 lea rdi,[rbp-0x1f60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e0a0);
            // a1a3c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1a41 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1a46 => {
            // block a1a46
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1a46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1a46 mov r12,rax
            r12 = rax;
            // a1a49 lea rdi,[rbp-0x1f78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e088);
            // a1a50 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1a55 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1a5a => {
            // block a1a5a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1a5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1a5a mov r12,rax
            r12 = rax;
            // a1a5d lea rdi,[rbp-0x1f90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e070);
            // a1a64 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1a69 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1a6e => {
            // block a1a6e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1a6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1a6e mov r12,rax
            r12 = rax;
            // a1a71 lea rdi,[rbp-0x1fa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e058);
            // a1a78 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1a7d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1a82 => {
            // block a1a82
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1a82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1a82 mov r12,rax
            r12 = rax;
            // a1a85 lea rdi,[rbp-0x1fc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e040);
            // a1a8c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1a91 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1a96 => {
            // block a1a96
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1a96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1a96 mov r12,rax
            r12 = rax;
            // a1a99 lea rdi,[rbp-0x1fd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e028);
            // a1aa0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1aa5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1aaa => {
            // block a1aaa
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1aaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1aaa mov r12,rax
            r12 = rax;
            // a1aad lea rdi,[rbp-0x1ff0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_e010);
            // a1ab4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1ab9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1abe => {
            // block a1abe
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1abe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1abe mov r12,rax
            r12 = rax;
            // a1ac1 lea rdi,[rbp-0x2008]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dff8);
            // a1ac8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1acd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1ad2 => {
            // block a1ad2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1ad2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1ad2 mov r12,rax
            r12 = rax;
            // a1ad5 lea rdi,[rbp-0x2020]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dfe0);
            // a1adc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1ae1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1ae6 => {
            // block a1ae6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1ae6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1ae6 mov r12,rax
            r12 = rax;
            // a1ae9 lea rdi,[rbp-0x2038]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dfc8);
            // a1af0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1af5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1afa => {
            // block a1afa
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1afa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1afa mov r12,rax
            r12 = rax;
            // a1afd lea rdi,[rbp-0x2050]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dfb0);
            // a1b04 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1b09 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1b0e => {
            // block a1b0e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1b0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1b0e mov r12,rax
            r12 = rax;
            // a1b11 lea rdi,[rbp-0x2068]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df98);
            // a1b18 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1b1d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1b22 => {
            // block a1b22
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1b22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1b22 mov r12,rax
            r12 = rax;
            // a1b25 lea rdi,[rbp-0x2080]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df80);
            // a1b2c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1b31 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1b36 => {
            // block a1b36
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1b36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1b36 mov r12,rax
            r12 = rax;
            // a1b39 lea rdi,[rbp-0x2098]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df68);
            // a1b40 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1b45 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1b4a => {
            // block a1b4a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1b4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1b4a mov r12,rax
            r12 = rax;
            // a1b4d lea rdi,[rbp-0x20b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df50);
            // a1b54 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1b59 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1b5e => {
            // block a1b5e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1b5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1b5e mov r12,rax
            r12 = rax;
            // a1b61 lea rdi,[rbp-0x20c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df38);
            // a1b68 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1b6d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1b72 => {
            // block a1b72
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1b72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1b72 mov r12,rax
            r12 = rax;
            // a1b75 lea rdi,[rbp-0x20e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df20);
            // a1b7c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1b81 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1b86 => {
            // block a1b86
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1b86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1b86 mov r12,rax
            r12 = rax;
            // a1b89 lea rdi,[rbp-0x20f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df08);
            // a1b90 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1b95 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1b9a => {
            // block a1b9a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1b9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1b9a mov r12,rax
            r12 = rax;
            // a1b9d lea rdi,[rbp-0x2110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_def0);
            // a1ba4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1ba9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1bae => {
            // block a1bae
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1bae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1bae mov r12,rax
            r12 = rax;
            // a1bb1 lea rdi,[rbp-0x2128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ded8);
            // a1bb8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1bbd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1bc2 => {
            // block a1bc2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1bc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1bc2 mov r12,rax
            r12 = rax;
            // a1bc5 lea rdi,[rbp-0x2140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dec0);
            // a1bcc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1bd1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1bd6 => {
            // block a1bd6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1bd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1bd6 mov r12,rax
            r12 = rax;
            // a1bd9 lea rdi,[rbp-0x2158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dea8);
            // a1be0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1be5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1bea => {
            // block a1bea
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1bea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1bea mov r12,rax
            r12 = rax;
            // a1bed lea rdi,[rbp-0x2170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de90);
            // a1bf4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1bf9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1bfe => {
            // block a1bfe
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1bfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1bfe mov r12,rax
            r12 = rax;
            // a1c01 lea rdi,[rbp-0x2188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de78);
            // a1c08 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1c0d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1c12 => {
            // block a1c12
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1c12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1c12 mov r12,rax
            r12 = rax;
            // a1c15 lea rdi,[rbp-0x21a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de60);
            // a1c1c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1c21 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1c26 => {
            // block a1c26
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1c26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1c26 mov r12,rax
            r12 = rax;
            // a1c29 lea rdi,[rbp-0x21b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de48);
            // a1c30 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1c35 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1c3a => {
            // block a1c3a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1c3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1c3a mov r12,rax
            r12 = rax;
            // a1c3d lea rdi,[rbp-0x21d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de30);
            // a1c44 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1c49 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1c4e => {
            // block a1c4e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1c4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1c4e mov r12,rax
            r12 = rax;
            // a1c51 lea rdi,[rbp-0x21e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de18);
            // a1c58 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1c5d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1c62 => {
            // block a1c62
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1c62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1c62 mov r12,rax
            r12 = rax;
            // a1c65 lea rdi,[rbp-0x2200]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_de00);
            // a1c6c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1c71 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1c76 => {
            // block a1c76
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1c76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1c76 mov r12,rax
            r12 = rax;
            // a1c79 lea rdi,[rbp-0x2218]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dde8);
            // a1c80 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1c85 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1c8a => {
            // block a1c8a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1c8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1c8a mov r12,rax
            r12 = rax;
            // a1c8d lea rdi,[rbp-0x2230]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ddd0);
            // a1c94 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1c99 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1c9e => {
            // block a1c9e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1c9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1c9e mov r12,rax
            r12 = rax;
            // a1ca1 lea rdi,[rbp-0x2248]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ddb8);
            // a1ca8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1cad jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1cb2 => {
            // block a1cb2
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1cb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1cb2 mov r12,rax
            r12 = rax;
            // a1cb5 lea rdi,[rbp-0x2260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dda0);
            // a1cbc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1cc1 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1cc6 => {
            // block a1cc6
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1cc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1cc6 mov r12,rax
            r12 = rax;
            // a1cc9 lea rdi,[rbp-0x2278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dd88);
            // a1cd0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1cd5 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1cda => {
            // block a1cda
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1cda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1cda mov r12,rax
            r12 = rax;
            // a1cdd lea rdi,[rbp-0x2290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dd70);
            // a1ce4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1ce9 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1cee => {
            // block a1cee
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1cee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1cee mov r12,rax
            r12 = rax;
            // a1cf1 lea rdi,[rbp-0x22a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dd58);
            // a1cf8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1cfd jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1d02 => {
            // block a1d02
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1d02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1d02 mov r12,rax
            r12 = rax;
            // a1d05 lea rdi,[rbp-0x22c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dd40);
            // a1d0c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1d11 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1d16 => {
            // block a1d16
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1d16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1d16 mov r12,rax
            r12 = rax;
            // a1d19 lea rdi,[rbp-0x22d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dd28);
            // a1d20 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1d25 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1d2a => {
            // block a1d2a
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1d2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1d2a mov r12,rax
            r12 = rax;
            // a1d2d lea rdi,[rbp-0x22f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dd10);
            // a1d34 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1d39 jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1d3e => {
            // block a1d3e
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1d3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1d3e mov r12,rax
            r12 = rax;
            // a1d41 lea rdi,[rbp-0x2308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dcf8);
            // a1d48 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1d4d jmp 0xa0069
            pc = 0xa0069; continue;
        }
        0xa1d52 => {
            // block a1d52
            #[cfg(feature = "trace")] eprintln!("f_0009aad2 pc a1d52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1d52 mov r12,rax
            r12 = rax;
            // a1d55 lea rdi,[rbp-0x2320]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dce0);
            // a1d5c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // a1d61 jmp 0xa0069
            pc = 0xa0069; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
