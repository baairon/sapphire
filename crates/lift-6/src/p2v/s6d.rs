//! Mechanically lifted functions (scripts/lift_run.py), module s6d. Each one is checked against the original
//! in the world harness (tests/world_s6d.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xbca4e size=27732
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bca4e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbca4e;
    loop {
        match pc {
        0xbca4e => {
            // block bca4e
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bca4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bca4e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bca4f mov rbp,rsp
            rbp = rsp;
            // bca52 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bca54 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bca56 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // bca58 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bca5a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bca5b sub rsp,0x12b8
            { let (r, f) = sub::<64>(rsp, 0x12b8u64);
              rsp = r;
              fl = f; }
            // bca62 mov DWORD PTR [rbp-0x2c],0x6e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x6eu64) as u32);
            // bca69 lea rbx,[rip+0x10bba0] # 0x1c8610
            rbx = m.env.data_va(904669);
            // bca70 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // bca74 mov rdi,rbx
            rdi = rbx;
            // bca77 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bca7c lea rsi,[rip+0x7b40b] # 0x137e8e
            rsi = m.env.data_va(312923);
            // bca83 mov rdi,rax
            rdi = rax;
            // bca86 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bca8b mov DWORD PTR [rbp-0x30],0x6f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x6fu64) as u32);
            // bca92 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // bca96 mov rdi,rbx
            rdi = rbx;
            // bca99 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bca9e lea rsi,[rip+0x7b3f2] # 0x137e97
            rsi = m.env.data_va(312932);
            // bcaa5 mov rdi,rax
            rdi = rax;
            // bcaa8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcaad mov DWORD PTR [rbp-0x34],0x70
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), (0x70u64) as u32);
            // bcab4 lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // bcab8 mov rdi,rbx
            rdi = rbx;
            // bcabb call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcac0 lea rsi,[rip+0x7b3d9] # 0x137ea0
            rsi = m.env.data_va(312941);
            // bcac7 mov rdi,rax
            rdi = rax;
            // bcaca call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcacf mov DWORD PTR [rbp-0x38],0x71
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x71u64) as u32);
            // bcad6 lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bcada mov rdi,rbx
            rdi = rbx;
            // bcadd call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcae2 lea rsi,[rip+0x7b3c0] # 0x137ea9
            rsi = m.env.data_va(312950);
            // bcae9 mov rdi,rax
            rdi = rax;
            // bcaec call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcaf1 mov DWORD PTR [rbp-0x3c],0x72
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), (0x72u64) as u32);
            // bcaf8 lea rsi,[rbp-0x3c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            // bcafc mov rdi,rbx
            rdi = rbx;
            // bcaff call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcb04 lea rsi,[rip+0x7b3a7] # 0x137eb2
            rsi = m.env.data_va(312959);
            // bcb0b mov rdi,rax
            rdi = rax;
            // bcb0e call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcb13 mov DWORD PTR [rbp-0x40],0x73
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x73u64) as u32);
            // bcb1a lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bcb1e mov rdi,rbx
            rdi = rbx;
            // bcb21 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcb26 lea rsi,[rip+0x7b38e] # 0x137ebb
            rsi = m.env.data_va(312968);
            // bcb2d mov rdi,rax
            rdi = rax;
            // bcb30 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcb35 mov DWORD PTR [rbp-0x44],0x74
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), (0x74u64) as u32);
            // bcb3c lea rsi,[rbp-0x44]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // bcb40 mov rdi,rbx
            rdi = rbx;
            // bcb43 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcb48 lea rsi,[rip+0x7b375] # 0x137ec4
            rsi = m.env.data_va(312977);
            // bcb4f mov rdi,rax
            rdi = rax;
            // bcb52 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcb57 mov DWORD PTR [rbp-0x48],0x75
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0x75u64) as u32);
            // bcb5e lea rsi,[rbp-0x48]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // bcb62 mov rdi,rbx
            rdi = rbx;
            // bcb65 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcb6a lea rsi,[rip+0x7b35c] # 0x137ecd
            rsi = m.env.data_va(312986);
            // bcb71 mov rdi,rax
            rdi = rax;
            // bcb74 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcb79 mov DWORD PTR [rbp-0x4c],0x6a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), (0x6au64) as u32);
            // bcb80 lea rsi,[rbp-0x4c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb4);
            // bcb84 mov rdi,rbx
            rdi = rbx;
            // bcb87 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcb8c lea rsi,[rip+0x7b343] # 0x137ed6
            rsi = m.env.data_va(312995);
            // bcb93 mov rdi,rax
            rdi = rax;
            // bcb96 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcb9b mov DWORD PTR [rbp-0x50],0x6b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x6bu64) as u32);
            // bcba2 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // bcba6 mov rdi,rbx
            rdi = rbx;
            // bcba9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcbae lea rsi,[rip+0x7b328] # 0x137edd
            rsi = m.env.data_va(313002);
            // bcbb5 mov rdi,rax
            rdi = rax;
            // bcbb8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcbbd mov DWORD PTR [rbp-0x54],0x6c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), (0x6cu64) as u32);
            // bcbc4 lea rsi,[rbp-0x54]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffac);
            // bcbc8 mov rdi,rbx
            rdi = rbx;
            // bcbcb call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcbd0 lea rsi,[rip+0x7b30d] # 0x137ee4
            rsi = m.env.data_va(313009);
            // bcbd7 mov rdi,rax
            rdi = rax;
            // bcbda call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcbdf mov DWORD PTR [rbp-0x58],0x6d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0x6du64) as u32);
            // bcbe6 lea rsi,[rbp-0x58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // bcbea mov rdi,rbx
            rdi = rbx;
            // bcbed call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcbf2 lea rsi,[rip+0x7b2f2] # 0x137eeb
            rsi = m.env.data_va(313016);
            // bcbf9 mov rdi,rax
            rdi = rax;
            // bcbfc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcc01 mov DWORD PTR [rbp-0x5c],0x12
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), (0x12u64) as u32);
            // bcc08 lea rsi,[rbp-0x5c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa4);
            // bcc0c mov rdi,rbx
            rdi = rbx;
            // bcc0f call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcc14 lea rsi,[rip+0x7b2d7] # 0x137ef2
            rsi = m.env.data_va(313023);
            // bcc1b mov rdi,rax
            rdi = rax;
            // bcc1e call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcc23 mov DWORD PTR [rbp-0x60],0x11
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x11u64) as u32);
            // bcc2a lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // bcc2e mov rdi,rbx
            rdi = rbx;
            // bcc31 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcc36 lea rsi,[rip+0x7196d] # 0x12e5aa
            rsi = m.env.data_va(273783);
            // bcc3d mov rdi,rax
            rdi = rax;
            // bcc40 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcc45 mov DWORD PTR [rbp-0x64],0x14
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x14u64) as u32);
            // bcc4c lea rsi,[rbp-0x64]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // bcc50 mov rdi,rbx
            rdi = rbx;
            // bcc53 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcc58 lea rsi,[rip+0x7b29b] # 0x137efa
            rsi = m.env.data_va(313031);
            // bcc5f mov rdi,rax
            rdi = rax;
            // bcc62 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcc67 mov DWORD PTR [rbp-0x68],0x13
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0x13u64) as u32);
            // bcc6e lea rsi,[rbp-0x68]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // bcc72 mov rdi,rbx
            rdi = rbx;
            // bcc75 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcc7a lea rsi,[rip+0x7193b] # 0x12e5bc
            rsi = m.env.data_va(273801);
            // bcc81 mov rdi,rax
            rdi = rax;
            // bcc84 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcc89 mov DWORD PTR [rbp-0x6c],0x16
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff94), (0x16u64) as u32);
            // bcc90 lea rsi,[rbp-0x6c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff94);
            // bcc94 mov rdi,rbx
            rdi = rbx;
            // bcc97 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcc9c lea rsi,[rip+0x7b25f] # 0x137f02
            rsi = m.env.data_va(313039);
            // bcca3 mov rdi,rax
            rdi = rax;
            // bcca6 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bccab mov DWORD PTR [rbp-0x70],0x15
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x15u64) as u32);
            // bccb2 lea rsi,[rbp-0x70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // bccb6 mov rdi,rbx
            rdi = rbx;
            // bccb9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bccbe lea rsi,[rip+0x71e2c] # 0x12eaf1
            rsi = m.env.data_va(275134);
            // bccc5 mov rdi,rax
            rdi = rax;
            // bccc8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcccd mov DWORD PTR [rbp-0x74],0x18
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c), (0x18u64) as u32);
            // bccd4 lea rsi,[rbp-0x74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // bccd8 mov rdi,rbx
            rdi = rbx;
            // bccdb call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcce0 lea rsi,[rip+0x7b223] # 0x137f0a
            rsi = m.env.data_va(313047);
            // bcce7 mov rdi,rax
            rdi = rax;
            // bccea call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bccef mov DWORD PTR [rbp-0x78],0x17
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x17u64) as u32);
            // bccf6 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // bccfa mov rdi,rbx
            rdi = rbx;
            // bccfd call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcd02 lea rsi,[rip+0x71dfa] # 0x12eb03
            rsi = m.env.data_va(275152);
            // bcd09 mov rdi,rax
            rdi = rax;
            // bcd0c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcd11 mov DWORD PTR [rbp-0x7c],0x1a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff84), (0x1au64) as u32);
            // bcd18 lea rsi,[rbp-0x7c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff84);
            // bcd1c mov rdi,rbx
            rdi = rbx;
            // bcd1f call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcd24 lea rsi,[rip+0x7b1e7] # 0x137f12
            rsi = m.env.data_va(313055);
            // bcd2b mov rdi,rax
            rdi = rax;
            // bcd2e call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcd33 mov DWORD PTR [rbp-0x80],0x19
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x19u64) as u32);
            // bcd3a lea rsi,[rbp-0x80]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // bcd3e mov rdi,rbx
            rdi = rbx;
            // bcd41 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcd46 lea rsi,[rip+0x71e20] # 0x12eb6d
            rsi = m.env.data_va(275258);
            // bcd4d mov rdi,rax
            rdi = rax;
            // bcd50 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcd55 mov DWORD PTR [rbp-0x84],0x1c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c), (0x1cu64) as u32);
            // bcd5f lea rsi,[rbp-0x84]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff7c);
            // bcd66 mov rdi,rbx
            rdi = rbx;
            // bcd69 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcd6e lea rsi,[rip+0x7b1a5] # 0x137f1a
            rsi = m.env.data_va(313063);
            // bcd75 mov rdi,rax
            rdi = rax;
            // bcd78 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcd7d mov DWORD PTR [rbp-0x88],0x1b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (0x1bu64) as u32);
            // bcd87 lea rsi,[rbp-0x88]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // bcd8e mov rdi,rbx
            rdi = rbx;
            // bcd91 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcd96 lea rsi,[rip+0x71fe1] # 0x12ed7e
            rsi = m.env.data_va(275787);
            // bcd9d mov rdi,rax
            rdi = rax;
            // bcda0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcda5 mov DWORD PTR [rbp-0x8c],0x1e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff74), (0x1eu64) as u32);
            // bcdaf lea rsi,[rbp-0x8c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff74);
            // bcdb6 mov rdi,rbx
            rdi = rbx;
            // bcdb9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcdbe lea rsi,[rip+0x7b15d] # 0x137f22
            rsi = m.env.data_va(313071);
            // bcdc5 mov rdi,rax
            rdi = rax;
            // bcdc8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcdcd mov DWORD PTR [rbp-0x90],0x1d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x1du64) as u32);
            // bcdd7 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // bcdde mov rdi,rbx
            rdi = rbx;
            // bcde1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcde6 lea rsi,[rip+0x71fa3] # 0x12ed90
            rsi = m.env.data_va(275805);
            // bcded mov rdi,rax
            rdi = rax;
            // bcdf0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcdf5 mov DWORD PTR [rbp-0x94],0x20
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c), (0x20u64) as u32);
            // bcdff lea rsi,[rbp-0x94]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff6c);
            // bce06 mov rdi,rbx
            rdi = rbx;
            // bce09 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bce0e lea rsi,[rip+0x7b115] # 0x137f2a
            rsi = m.env.data_va(313079);
            // bce15 mov rdi,rax
            rdi = rax;
            // bce18 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bce1d mov DWORD PTR [rbp-0x98],0x1f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0x1fu64) as u32);
            // bce27 lea rsi,[rbp-0x98]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // bce2e mov rdi,rbx
            rdi = rbx;
            // bce31 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bce36 lea rsi,[rip+0x71f65] # 0x12eda2
            rsi = m.env.data_va(275823);
            // bce3d mov rdi,rax
            rdi = rax;
            // bce40 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bce45 mov DWORD PTR [rbp-0x9c],0x3d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff64), (0x3du64) as u32);
            // bce4f lea rsi,[rbp-0x9c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff64);
            // bce56 mov rdi,rbx
            rdi = rbx;
            // bce59 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bce5e lea rsi,[rip+0x7191f] # 0x12e784
            rsi = m.env.data_va(274257);
            // bce65 mov rdi,rax
            rdi = rax;
            // bce68 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bce6d mov DWORD PTR [rbp-0xa0],0x3e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x3eu64) as u32);
            // bce77 lea rsi,[rbp-0xa0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // bce7e mov rdi,rbx
            rdi = rbx;
            // bce81 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bce86 lea rsi,[rip+0x7b0a5] # 0x137f32
            rsi = m.env.data_va(313087);
            // bce8d mov rdi,rax
            rdi = rax;
            // bce90 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bce95 mov DWORD PTR [rbp-0xa4],0x3f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff5c), (0x3fu64) as u32);
            // bce9f lea rsi,[rbp-0xa4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff5c);
            // bcea6 mov rdi,rbx
            rdi = rbx;
            // bcea9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bceae lea rsi,[rip+0x7b085] # 0x137f3a
            rsi = m.env.data_va(313095);
            // bceb5 mov rdi,rax
            rdi = rax;
            // bceb8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcebd mov DWORD PTR [rbp-0xa8],0x40
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x40u64) as u32);
            // bcec7 lea rsi,[rbp-0xa8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // bcece mov rdi,rbx
            rdi = rbx;
            // bced1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bced6 lea rsi,[rip+0x7b063] # 0x137f40
            rsi = m.env.data_va(313101);
            // bcedd mov rdi,rax
            rdi = rax;
            // bcee0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcee5 mov DWORD PTR [rbp-0xac],0x41
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff54), (0x41u64) as u32);
            // bceef lea rsi,[rbp-0xac]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff54);
            // bcef6 mov rdi,rbx
            rdi = rbx;
            // bcef9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcefe lea rsi,[rip+0x7b043] # 0x137f48
            rsi = m.env.data_va(313109);
            // bcf05 mov rdi,rax
            rdi = rax;
            // bcf08 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcf0d mov DWORD PTR [rbp-0xb0],0x42
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x42u64) as u32);
            // bcf17 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // bcf1e mov rdi,rbx
            rdi = rbx;
            // bcf21 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcf26 lea rsi,[rip+0x7b021] # 0x137f4e
            rsi = m.env.data_va(313115);
            // bcf2d mov rdi,rax
            rdi = rax;
            // bcf30 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcf35 mov DWORD PTR [rbp-0xb4],0x43
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff4c), (0x43u64) as u32);
            // bcf3f lea rsi,[rbp-0xb4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff4c);
            // bcf46 mov rdi,rbx
            rdi = rbx;
            // bcf49 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcf4e lea rsi,[rip+0x7b001] # 0x137f56
            rsi = m.env.data_va(313123);
            // bcf55 mov rdi,rax
            rdi = rax;
            // bcf58 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcf5d mov DWORD PTR [rbp-0xb8],0x44
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (0x44u64) as u32);
            // bcf67 lea rsi,[rbp-0xb8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // bcf6e mov rdi,rbx
            rdi = rbx;
            // bcf71 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcf76 lea rsi,[rip+0x7afdf] # 0x137f5c
            rsi = m.env.data_va(313129);
            // bcf7d mov rdi,rax
            rdi = rax;
            // bcf80 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcf85 mov DWORD PTR [rbp-0xbc],0x45
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff44), (0x45u64) as u32);
            // bcf8f lea rsi,[rbp-0xbc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff44);
            // bcf96 mov rdi,rbx
            rdi = rbx;
            // bcf99 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcf9e lea rsi,[rip+0x7afbf] # 0x137f64
            rsi = m.env.data_va(313137);
            // bcfa5 mov rdi,rax
            rdi = rax;
            // bcfa8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcfad mov DWORD PTR [rbp-0xc0],0x46
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x46u64) as u32);
            // bcfb7 lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // bcfbe mov rdi,rbx
            rdi = rbx;
            // bcfc1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcfc6 lea rsi,[rip+0x7af9d] # 0x137f6a
            rsi = m.env.data_va(313143);
            // bcfcd mov rdi,rax
            rdi = rax;
            // bcfd0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcfd5 mov DWORD PTR [rbp-0xc4],0x47
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff3c), (0x47u64) as u32);
            // bcfdf lea rsi,[rbp-0xc4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff3c);
            // bcfe6 mov rdi,rbx
            rdi = rbx;
            // bcfe9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcfee lea rsi,[rip+0x7af7d] # 0x137f72
            rsi = m.env.data_va(313151);
            // bcff5 mov rdi,rax
            rdi = rax;
            // bcff8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bcffd mov DWORD PTR [rbp-0xc8],0x48
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (0x48u64) as u32);
            // bd007 lea rsi,[rbp-0xc8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // bd00e mov rdi,rbx
            rdi = rbx;
            // bd011 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd016 lea rsi,[rip+0x7af5b] # 0x137f78
            rsi = m.env.data_va(313157);
            // bd01d mov rdi,rax
            rdi = rax;
            // bd020 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd025 mov DWORD PTR [rbp-0xcc],0x49
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff34), (0x49u64) as u32);
            // bd02f lea rsi,[rbp-0xcc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff34);
            // bd036 mov rdi,rbx
            rdi = rbx;
            // bd039 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd03e lea rsi,[rip+0x7af3b] # 0x137f80
            rsi = m.env.data_va(313165);
            // bd045 mov rdi,rax
            rdi = rax;
            // bd048 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd04d mov DWORD PTR [rbp-0xd0],0x4a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (0x4au64) as u32);
            // bd057 lea rsi,[rbp-0xd0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // bd05e mov rdi,rbx
            rdi = rbx;
            // bd061 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd066 lea rsi,[rip+0x7af19] # 0x137f86
            rsi = m.env.data_va(313171);
            // bd06d mov rdi,rax
            rdi = rax;
            // bd070 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd075 mov DWORD PTR [rbp-0xd4],0x4b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff2c), (0x4bu64) as u32);
            // bd07f lea rsi,[rbp-0xd4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff2c);
            // bd086 mov rdi,rbx
            rdi = rbx;
            // bd089 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd08e lea rsi,[rip+0x7aef9] # 0x137f8e
            rsi = m.env.data_va(313179);
            // bd095 mov rdi,rax
            rdi = rax;
            // bd098 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd09d mov DWORD PTR [rbp-0xd8],0x4c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (0x4cu64) as u32);
            // bd0a7 lea rsi,[rbp-0xd8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // bd0ae mov rdi,rbx
            rdi = rbx;
            // bd0b1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd0b6 lea rsi,[rip+0x7aed7] # 0x137f94
            rsi = m.env.data_va(313185);
            // bd0bd mov rdi,rax
            rdi = rax;
            // bd0c0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd0c5 mov DWORD PTR [rbp-0xdc],0x4d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff24), (0x4du64) as u32);
            // bd0cf lea rsi,[rbp-0xdc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff24);
            // bd0d6 mov rdi,rbx
            rdi = rbx;
            // bd0d9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd0de lea rsi,[rip+0x716f5] # 0x12e7da
            rsi = m.env.data_va(274343);
            // bd0e5 mov rdi,rax
            rdi = rax;
            // bd0e8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd0ed mov DWORD PTR [rbp-0xe0],0x4e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x4eu64) as u32);
            // bd0f7 lea rsi,[rbp-0xe0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // bd0fe mov rdi,rbx
            rdi = rbx;
            // bd101 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd106 lea rsi,[rip+0x716e9] # 0x12e7f6
            rsi = m.env.data_va(274371);
            // bd10d mov rdi,rax
            rdi = rax;
            // bd110 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd115 mov DWORD PTR [rbp-0xe4],0x4f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c), (0x4fu64) as u32);
            // bd11f lea rsi,[rbp-0xe4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff1c);
            // bd126 mov rdi,rbx
            rdi = rbx;
            // bd129 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd12e lea rsi,[rip+0x7ae67] # 0x137f9c
            rsi = m.env.data_va(313193);
            // bd135 mov rdi,rax
            rdi = rax;
            // bd138 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd13d mov DWORD PTR [rbp-0xe8],0x50
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x50u64) as u32);
            // bd147 lea rsi,[rbp-0xe8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // bd14e mov rdi,rbx
            rdi = rbx;
            // bd151 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd156 lea rsi,[rip+0x7ae46] # 0x137fa3
            rsi = m.env.data_va(313200);
            // bd15d mov rdi,rax
            rdi = rax;
            // bd160 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd165 mov DWORD PTR [rbp-0xec],0x51
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff14), (0x51u64) as u32);
            // bd16f lea rsi,[rbp-0xec]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff14);
            // bd176 mov rdi,rbx
            rdi = rbx;
            // bd179 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd17e lea rsi,[rip+0x7ae25] # 0x137faa
            rsi = m.env.data_va(313207);
            // bd185 mov rdi,rax
            rdi = rax;
            // bd188 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd18d mov DWORD PTR [rbp-0xf0],0x52
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x52u64) as u32);
            // bd197 lea rsi,[rbp-0xf0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // bd19e mov rdi,rbx
            rdi = rbx;
            // bd1a1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd1a6 lea rsi,[rip+0x7ae04] # 0x137fb1
            rsi = m.env.data_va(313214);
            // bd1ad mov rdi,rax
            rdi = rax;
            // bd1b0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd1b5 mov DWORD PTR [rbp-0xf4],0x53
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff0c), (0x53u64) as u32);
            // bd1bf lea rsi,[rbp-0xf4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff0c);
            // bd1c6 mov rdi,rbx
            rdi = rbx;
            // bd1c9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd1ce lea rsi,[rip+0x7ade3] # 0x137fb8
            rsi = m.env.data_va(313221);
            // bd1d5 mov rdi,rax
            rdi = rax;
            // bd1d8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd1dd mov DWORD PTR [rbp-0xf8],0x54
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x54u64) as u32);
            // bd1e7 lea rsi,[rbp-0xf8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // bd1ee mov rdi,rbx
            rdi = rbx;
            // bd1f1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd1f6 lea rsi,[rip+0x7adc2] # 0x137fbf
            rsi = m.env.data_va(313228);
            // bd1fd mov rdi,rax
            rdi = rax;
            // bd200 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd205 mov DWORD PTR [rbp-0xfc],0x1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff04), (0x1u64) as u32);
            // bd20f lea rsi,[rbp-0xfc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff04);
            // bd216 mov rdi,rbx
            rdi = rbx;
            // bd219 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd21e lea rsi,[rip+0x71321] # 0x12e546
            rsi = m.env.data_va(273683);
            // bd225 mov rdi,rax
            rdi = rax;
            // bd228 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd22d mov DWORD PTR [rbp-0x100],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x2u64) as u32);
            // bd237 lea rsi,[rbp-0x100]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // bd23e mov rdi,rbx
            rdi = rbx;
            // bd241 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd246 lea rsi,[rip+0x7ad79] # 0x137fc6
            rsi = m.env.data_va(313235);
            // bd24d mov rdi,rax
            rdi = rax;
            // bd250 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd255 mov DWORD PTR [rbp-0x104],0x3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fefc), (0x3u64) as u32);
            // bd25f lea rsi,[rbp-0x104]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fefc);
            // bd266 mov rdi,rbx
            rdi = rbx;
            // bd269 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd26e lea rsi,[rip+0x712ea] # 0x12e55f
            rsi = m.env.data_va(273708);
            // bd275 mov rdi,rax
            rdi = rax;
            // bd278 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd27d mov DWORD PTR [rbp-0x108],0x4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x4u64) as u32);
            // bd287 lea rsi,[rbp-0x108]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // bd28e mov rdi,rbx
            rdi = rbx;
            // bd291 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd296 lea rsi,[rip+0x7ad2e] # 0x137fcb
            rsi = m.env.data_va(313240);
            // bd29d mov rdi,rax
            rdi = rax;
            // bd2a0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd2a5 mov DWORD PTR [rbp-0x10c],0x5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef4), (0x5u64) as u32);
            // bd2af lea rsi,[rbp-0x10c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fef4);
            // bd2b6 mov rdi,rbx
            rdi = rbx;
            // bd2b9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd2be lea rsi,[rip+0x712b3] # 0x12e578
            rsi = m.env.data_va(273733);
            // bd2c5 mov rdi,rax
            rdi = rax;
            // bd2c8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd2cd mov DWORD PTR [rbp-0x110],0x6
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x6u64) as u32);
            // bd2d7 lea rsi,[rbp-0x110]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // bd2de mov rdi,rbx
            rdi = rbx;
            // bd2e1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd2e6 lea rsi,[rip+0x7ace3] # 0x137fd0
            rsi = m.env.data_va(313245);
            // bd2ed mov rdi,rax
            rdi = rax;
            // bd2f0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd2f5 mov DWORD PTR [rbp-0x114],0x7
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feec), (0x7u64) as u32);
            // bd2ff lea rsi,[rbp-0x114]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feec);
            // bd306 mov rdi,rbx
            rdi = rbx;
            // bd309 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd30e lea rsi,[rip+0x7127c] # 0x12e591
            rsi = m.env.data_va(273758);
            // bd315 mov rdi,rax
            rdi = rax;
            // bd318 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd31d mov DWORD PTR [rbp-0x118],0x8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x8u64) as u32);
            // bd327 lea rsi,[rbp-0x118]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // bd32e mov rdi,rbx
            rdi = rbx;
            // bd331 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd336 lea rsi,[rip+0x7ac98] # 0x137fd5
            rsi = m.env.data_va(313250);
            // bd33d mov rdi,rax
            rdi = rax;
            // bd340 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd345 mov DWORD PTR [rbp-0x11c],0x9
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fee4), (0x9u64) as u32);
            // bd34f lea rsi,[rbp-0x11c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fee4);
            // bd356 mov rdi,rbx
            rdi = rbx;
            // bd359 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd35e lea rsi,[rip+0x71aa4] # 0x12ee09
            rsi = m.env.data_va(275926);
            // bd365 mov rdi,rax
            rdi = rax;
            // bd368 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd36d mov DWORD PTR [rbp-0x120],0xa
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0xau64) as u32);
            // bd377 lea rsi,[rbp-0x120]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // bd37e mov rdi,rbx
            rdi = rbx;
            // bd381 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd386 lea rsi,[rip+0x7ac4d] # 0x137fda
            rsi = m.env.data_va(313255);
            // bd38d mov rdi,rax
            rdi = rax;
            // bd390 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd395 mov DWORD PTR [rbp-0x124],0xb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fedc), (0xbu64) as u32);
            // bd39f lea rsi,[rbp-0x124]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fedc);
            // bd3a6 mov rdi,rbx
            rdi = rbx;
            // bd3a9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd3ae lea rsi,[rip+0x71a6d] # 0x12ee22
            rsi = m.env.data_va(275951);
            // bd3b5 mov rdi,rax
            rdi = rax;
            // bd3b8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd3bd mov DWORD PTR [rbp-0x128],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0xcu64) as u32);
            // bd3c7 lea rsi,[rbp-0x128]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // bd3ce mov rdi,rbx
            rdi = rbx;
            // bd3d1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd3d6 lea rsi,[rip+0x7ac02] # 0x137fdf
            rsi = m.env.data_va(313260);
            // bd3dd mov rdi,rax
            rdi = rax;
            // bd3e0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd3e5 mov DWORD PTR [rbp-0x12c],0xd
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fed4), (0xdu64) as u32);
            // bd3ef lea rsi,[rbp-0x12c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fed4);
            // bd3f6 mov rdi,rbx
            rdi = rbx;
            // bd3f9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd3fe lea rsi,[rip+0x71a36] # 0x12ee3b
            rsi = m.env.data_va(275976);
            // bd405 mov rdi,rax
            rdi = rax;
            // bd408 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd40d mov DWORD PTR [rbp-0x130],0xe
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (0xeu64) as u32);
            // bd417 lea rsi,[rbp-0x130]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // bd41e mov rdi,rbx
            rdi = rbx;
            // bd421 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd426 lea rsi,[rip+0x7abb7] # 0x137fe4
            rsi = m.env.data_va(313265);
            // bd42d mov rdi,rax
            rdi = rax;
            // bd430 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd435 mov DWORD PTR [rbp-0x134],0xf
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fecc), (0xfu64) as u32);
            // bd43f lea rsi,[rbp-0x134]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fecc);
            // bd446 mov rdi,rbx
            rdi = rbx;
            // bd449 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd44e lea rsi,[rip+0x719ff] # 0x12ee54
            rsi = m.env.data_va(276001);
            // bd455 mov rdi,rax
            rdi = rax;
            // bd458 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd45d mov DWORD PTR [rbp-0x138],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (0x10u64) as u32);
            // bd467 lea rsi,[rbp-0x138]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // bd46e mov rdi,rbx
            rdi = rbx;
            // bd471 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd476 lea rsi,[rip+0x7ab6c] # 0x137fe9
            rsi = m.env.data_va(313270);
            // bd47d mov rdi,rax
            rdi = rax;
            // bd480 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd485 mov DWORD PTR [rbp-0x13c],0x21
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fec4), (0x21u64) as u32);
            // bd48f lea rsi,[rbp-0x13c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fec4);
            // bd496 mov rdi,rbx
            rdi = rbx;
            // bd499 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd49e lea rsi,[rip+0x71129] # 0x12e5ce
            rsi = m.env.data_va(273819);
            // bd4a5 mov rdi,rax
            rdi = rax;
            // bd4a8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd4ad mov DWORD PTR [rbp-0x140],0x22
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (0x22u64) as u32);
            // bd4b7 lea rsi,[rbp-0x140]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // bd4be mov rdi,rbx
            rdi = rbx;
            // bd4c1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd4c6 lea rsi,[rip+0x71113] # 0x12e5e0
            rsi = m.env.data_va(273837);
            // bd4cd mov rdi,rax
            rdi = rax;
            // bd4d0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd4d5 mov DWORD PTR [rbp-0x144],0x23
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_febc), (0x23u64) as u32);
            // bd4df lea rsi,[rbp-0x144]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_febc);
            // bd4e6 mov rdi,rbx
            rdi = rbx;
            // bd4e9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd4ee lea rsi,[rip+0x7155c] # 0x12ea51
            rsi = m.env.data_va(274974);
            // bd4f5 mov rdi,rax
            rdi = rax;
            // bd4f8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd4fd mov DWORD PTR [rbp-0x148],0x24
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (0x24u64) as u32);
            // bd507 lea rsi,[rbp-0x148]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // bd50e mov rdi,rbx
            rdi = rbx;
            // bd511 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd516 lea rsi,[rip+0x71546] # 0x12ea63
            rsi = m.env.data_va(274992);
            // bd51d mov rdi,rax
            rdi = rax;
            // bd520 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd525 mov DWORD PTR [rbp-0x14c],0x25
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feb4), (0x25u64) as u32);
            // bd52f lea rsi,[rbp-0x14c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb4);
            // bd536 mov rdi,rbx
            rdi = rbx;
            // bd539 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd53e lea rsi,[rip+0x7163a] # 0x12eb7f
            rsi = m.env.data_va(275276);
            // bd545 mov rdi,rax
            rdi = rax;
            // bd548 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd54d mov DWORD PTR [rbp-0x150],0x26
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (0x26u64) as u32);
            // bd557 lea rsi,[rbp-0x150]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // bd55e mov rdi,rbx
            rdi = rbx;
            // bd561 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd566 lea rsi,[rip+0x7183a] # 0x12eda7
            rsi = m.env.data_va(275828);
            // bd56d mov rdi,rax
            rdi = rax;
            // bd570 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd575 mov DWORD PTR [rbp-0x154],0x27
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feac), (0x27u64) as u32);
            // bd57f lea rsi,[rbp-0x154]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feac);
            // bd586 mov rdi,rbx
            rdi = rbx;
            // bd589 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd58e lea rsi,[rip+0x71824] # 0x12edb9
            rsi = m.env.data_va(275846);
            // bd595 mov rdi,rax
            rdi = rax;
            // bd598 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd59d mov DWORD PTR [rbp-0x158],0x28
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (0x28u64) as u32);
            // bd5a7 lea rsi,[rbp-0x158]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // bd5ae mov rdi,rbx
            rdi = rbx;
            // bd5b1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd5b6 lea rsi,[rip+0x7180e] # 0x12edcb
            rsi = m.env.data_va(275864);
            // bd5bd mov rdi,rax
            rdi = rax;
            // bd5c0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd5c5 mov DWORD PTR [rbp-0x15c],0x55
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fea4), (0x55u64) as u32);
            // bd5cf lea rsi,[rbp-0x15c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea4);
            // bd5d6 mov rdi,rbx
            rdi = rbx;
            // bd5d9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd5de lea rsi,[rip+0x7aa09] # 0x137fee
            rsi = m.env.data_va(313275);
            // bd5e5 mov rdi,rax
            rdi = rax;
            // bd5e8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd5ed mov DWORD PTR [rbp-0x160],0x56
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (0x56u64) as u32);
            // bd5f7 lea rsi,[rbp-0x160]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // bd5fe mov rdi,rbx
            rdi = rbx;
            // bd601 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd606 lea rsi,[rip+0x7a9ed] # 0x137ffa
            rsi = m.env.data_va(313287);
            // bd60d mov rdi,rax
            rdi = rax;
            // bd610 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd615 mov DWORD PTR [rbp-0x164],0x57
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe9c), (0x57u64) as u32);
            // bd61f lea rsi,[rbp-0x164]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe9c);
            // bd626 mov rdi,rbx
            rdi = rbx;
            // bd629 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd62e lea rsi,[rip+0x7a9ce] # 0x138003
            rsi = m.env.data_va(313296);
            // bd635 mov rdi,rax
            rdi = rax;
            // bd638 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd63d mov DWORD PTR [rbp-0x168],0x58
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (0x58u64) as u32);
            // bd647 lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // bd64e mov rdi,rbx
            rdi = rbx;
            // bd651 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd656 lea rsi,[rip+0x7a9b2] # 0x13800f
            rsi = m.env.data_va(313308);
            // bd65d mov rdi,rax
            rdi = rax;
            // bd660 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd665 mov DWORD PTR [rbp-0x16c],0x59
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe94), (0x59u64) as u32);
            // bd66f lea rsi,[rbp-0x16c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe94);
            // bd676 mov rdi,rbx
            rdi = rbx;
            // bd679 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd67e lea rsi,[rip+0x7a993] # 0x138018
            rsi = m.env.data_va(313317);
            // bd685 mov rdi,rax
            rdi = rax;
            // bd688 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd68d mov DWORD PTR [rbp-0x170],0x5a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (0x5au64) as u32);
            // bd697 lea rsi,[rbp-0x170]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // bd69e mov rdi,rbx
            rdi = rbx;
            // bd6a1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd6a6 lea rsi,[rip+0x7a977] # 0x138024
            rsi = m.env.data_va(313329);
            // bd6ad mov rdi,rax
            rdi = rax;
            // bd6b0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd6b5 mov DWORD PTR [rbp-0x174],0x5b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe8c), (0x5bu64) as u32);
            // bd6bf lea rsi,[rbp-0x174]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe8c);
            // bd6c6 mov rdi,rbx
            rdi = rbx;
            // bd6c9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd6ce lea rsi,[rip+0x7a958] # 0x13802d
            rsi = m.env.data_va(313338);
            // bd6d5 mov rdi,rax
            rdi = rax;
            // bd6d8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd6dd mov DWORD PTR [rbp-0x178],0x5c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (0x5cu64) as u32);
            // bd6e7 lea rsi,[rbp-0x178]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // bd6ee mov rdi,rbx
            rdi = rbx;
            // bd6f1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd6f6 lea rsi,[rip+0x7a93c] # 0x138039
            rsi = m.env.data_va(313350);
            // bd6fd mov rdi,rax
            rdi = rax;
            // bd700 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd705 mov DWORD PTR [rbp-0x17c],0x5d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe84), (0x5du64) as u32);
            // bd70f lea rsi,[rbp-0x17c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe84);
            // bd716 mov rdi,rbx
            rdi = rbx;
            // bd719 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd71e lea rsi,[rip+0x7a91d] # 0x138042
            rsi = m.env.data_va(313359);
            // bd725 mov rdi,rax
            rdi = rax;
            // bd728 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd72d mov DWORD PTR [rbp-0x180],0x5e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (0x5eu64) as u32);
            // bd737 lea rsi,[rbp-0x180]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // bd73e mov rdi,rbx
            rdi = rbx;
            // bd741 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd746 lea rsi,[rip+0x7a901] # 0x13804e
            rsi = m.env.data_va(313371);
            // bd74d mov rdi,rax
            rdi = rax;
            // bd750 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd755 mov DWORD PTR [rbp-0x184],0x5f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe7c), (0x5fu64) as u32);
            // bd75f lea rsi,[rbp-0x184]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe7c);
            // bd766 mov rdi,rbx
            rdi = rbx;
            // bd769 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd76e lea rsi,[rip+0x7a8e2] # 0x138057
            rsi = m.env.data_va(313380);
            // bd775 mov rdi,rax
            rdi = rax;
            // bd778 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd77d mov DWORD PTR [rbp-0x188],0x60
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (0x60u64) as u32);
            // bd787 lea rsi,[rbp-0x188]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // bd78e mov rdi,rbx
            rdi = rbx;
            // bd791 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd796 lea rsi,[rip+0x7a8c6] # 0x138063
            rsi = m.env.data_va(313392);
            // bd79d mov rdi,rax
            rdi = rax;
            // bd7a0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd7a5 mov DWORD PTR [rbp-0x18c],0x61
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe74), (0x61u64) as u32);
            // bd7af lea rsi,[rbp-0x18c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe74);
            // bd7b6 mov rdi,rbx
            rdi = rbx;
            // bd7b9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd7be lea rsi,[rip+0x7a8a7] # 0x13806c
            rsi = m.env.data_va(313401);
            // bd7c5 mov rdi,rax
            rdi = rax;
            // bd7c8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd7cd mov DWORD PTR [rbp-0x190],0x62
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (0x62u64) as u32);
            // bd7d7 lea rsi,[rbp-0x190]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // bd7de mov rdi,rbx
            rdi = rbx;
            // bd7e1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd7e6 lea rsi,[rip+0x7a88b] # 0x138078
            rsi = m.env.data_va(313413);
            // bd7ed mov rdi,rax
            rdi = rax;
            // bd7f0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd7f5 mov DWORD PTR [rbp-0x194],0x63
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe6c), (0x63u64) as u32);
            // bd7ff lea rsi,[rbp-0x194]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe6c);
            // bd806 mov rdi,rbx
            rdi = rbx;
            // bd809 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd80e lea rsi,[rip+0x7a86c] # 0x138081
            rsi = m.env.data_va(313422);
            // bd815 mov rdi,rax
            rdi = rax;
            // bd818 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd81d mov DWORD PTR [rbp-0x198],0x64
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (0x64u64) as u32);
            // bd827 lea rsi,[rbp-0x198]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // bd82e mov rdi,rbx
            rdi = rbx;
            // bd831 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd836 lea rsi,[rip+0x7a850] # 0x13808d
            rsi = m.env.data_va(313434);
            // bd83d mov rdi,rax
            rdi = rax;
            // bd840 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd845 mov DWORD PTR [rbp-0x19c],0x85
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe64), (0x85u64) as u32);
            // bd84f lea rsi,[rbp-0x19c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe64);
            // bd856 mov rdi,rbx
            rdi = rbx;
            // bd859 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd85e lea rsi,[rip+0x70d8d] # 0x12e5f2
            rsi = m.env.data_va(273855);
            // bd865 mov rdi,rax
            rdi = rax;
            // bd868 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd86d mov DWORD PTR [rbp-0x1a0],0x84
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe60), (0x84u64) as u32);
            // bd877 lea rsi,[rbp-0x1a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // bd87e mov rdi,rbx
            rdi = rbx;
            // bd881 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd886 lea rsi,[rip+0x70d7c] # 0x12e609
            rsi = m.env.data_va(273878);
            // bd88d mov rdi,rax
            rdi = rax;
            // bd890 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd895 mov DWORD PTR [rbp-0x1a4],0x81
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe5c), (0x81u64) as u32);
            // bd89f lea rsi,[rbp-0x1a4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe5c);
            // bd8a6 mov rdi,rbx
            rdi = rbx;
            // bd8a9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd8ae lea rsi,[rip+0x7a7e1] # 0x138096
            rsi = m.env.data_va(313443);
            // bd8b5 mov rdi,rax
            rdi = rax;
            // bd8b8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd8bd mov DWORD PTR [rbp-0x1a8],0x82
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (0x82u64) as u32);
            // bd8c7 lea rsi,[rbp-0x1a8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // bd8ce mov rdi,rbx
            rdi = rbx;
            // bd8d1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd8d6 lea rsi,[rip+0x7a7be] # 0x13809b
            rsi = m.env.data_va(313448);
            // bd8dd mov rdi,rax
            rdi = rax;
            // bd8e0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd8e5 mov DWORD PTR [rbp-0x1ac],0x76
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe54), (0x76u64) as u32);
            // bd8ef lea rsi,[rbp-0x1ac]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe54);
            // bd8f6 mov rdi,rbx
            rdi = rbx;
            // bd8f9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd8fe lea rsi,[rip+0x7a79b] # 0x1380a0
            rsi = m.env.data_va(313453);
            // bd905 mov rdi,rax
            rdi = rax;
            // bd908 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd90d mov DWORD PTR [rbp-0x1b0],0x2d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (0x2du64) as u32);
            // bd917 lea rsi,[rbp-0x1b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // bd91e mov rdi,rbx
            rdi = rbx;
            // bd921 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd926 lea rsi,[rip+0x70d87] # 0x12e6b4
            rsi = m.env.data_va(274049);
            // bd92d mov rdi,rax
            rdi = rax;
            // bd930 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd935 mov DWORD PTR [rbp-0x1b4],0x2e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe4c), (0x2eu64) as u32);
            // bd93f lea rsi,[rbp-0x1b4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe4c);
            // bd946 mov rdi,rbx
            rdi = rbx;
            // bd949 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd94e lea rsi,[rip+0x7a755] # 0x1380aa
            rsi = m.env.data_va(313463);
            // bd955 mov rdi,rax
            rdi = rax;
            // bd958 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd95d mov DWORD PTR [rbp-0x1b8],0x2f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe48), (0x2fu64) as u32);
            // bd967 lea rsi,[rbp-0x1b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe48);
            // bd96e mov rdi,rbx
            rdi = rbx;
            // bd971 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd976 lea rsi,[rip+0x70d4f] # 0x12e6cc
            rsi = m.env.data_va(274073);
            // bd97d mov rdi,rax
            rdi = rax;
            // bd980 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd985 mov DWORD PTR [rbp-0x1bc],0x30
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe44), (0x30u64) as u32);
            // bd98f lea rsi,[rbp-0x1bc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe44);
            // bd996 mov rdi,rbx
            rdi = rbx;
            // bd999 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd99e lea rsi,[rip+0x7a70f] # 0x1380b4
            rsi = m.env.data_va(313473);
            // bd9a5 mov rdi,rax
            rdi = rax;
            // bd9a8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd9ad mov DWORD PTR [rbp-0x1c0],0x31
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe40), (0x31u64) as u32);
            // bd9b7 lea rsi,[rbp-0x1c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // bd9be mov rdi,rbx
            rdi = rbx;
            // bd9c1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd9c6 lea rsi,[rip+0x70d17] # 0x12e6e4
            rsi = m.env.data_va(274097);
            // bd9cd mov rdi,rax
            rdi = rax;
            // bd9d0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd9d5 mov DWORD PTR [rbp-0x1c4],0x32
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe3c), (0x32u64) as u32);
            // bd9df lea rsi,[rbp-0x1c4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe3c);
            // bd9e6 mov rdi,rbx
            rdi = rbx;
            // bd9e9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd9ee lea rsi,[rip+0x7a6c9] # 0x1380be
            rsi = m.env.data_va(313483);
            // bd9f5 mov rdi,rax
            rdi = rax;
            // bd9f8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bd9fd mov DWORD PTR [rbp-0x1c8],0x33
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe38), (0x33u64) as u32);
            // bda07 lea rsi,[rbp-0x1c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe38);
            // bda0e mov rdi,rbx
            rdi = rbx;
            // bda11 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bda16 lea rsi,[rip+0x70cdf] # 0x12e6fc
            rsi = m.env.data_va(274121);
            // bda1d mov rdi,rax
            rdi = rax;
            // bda20 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bda25 mov DWORD PTR [rbp-0x1cc],0x34
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe34), (0x34u64) as u32);
            // bda2f lea rsi,[rbp-0x1cc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe34);
            // bda36 mov rdi,rbx
            rdi = rbx;
            // bda39 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bda3e lea rsi,[rip+0x7a683] # 0x1380c8
            rsi = m.env.data_va(313493);
            // bda45 mov rdi,rax
            rdi = rax;
            // bda48 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bda4d mov DWORD PTR [rbp-0x1d0],0x35
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe30), (0x35u64) as u32);
            // bda57 lea rsi,[rbp-0x1d0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe30);
            // bda5e mov rdi,rbx
            rdi = rbx;
            // bda61 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bda66 lea rsi,[rip+0x7a665] # 0x1380d2
            rsi = m.env.data_va(313503);
            // bda6d mov rdi,rax
            rdi = rax;
            // bda70 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bda75 mov DWORD PTR [rbp-0x1d4],0x36
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe2c), (0x36u64) as u32);
            // bda7f lea rsi,[rbp-0x1d4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe2c);
            // bda86 mov rdi,rbx
            rdi = rbx;
            // bda89 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bda8e lea rsi,[rip+0x7a645] # 0x1380da
            rsi = m.env.data_va(313511);
            // bda95 mov rdi,rax
            rdi = rax;
            // bda98 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bda9d mov DWORD PTR [rbp-0x1d8],0x37
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe28), (0x37u64) as u32);
            // bdaa7 lea rsi,[rbp-0x1d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe28);
            // bdaae mov rdi,rbx
            rdi = rbx;
            // bdab1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdab6 lea rsi,[rip+0x7a627] # 0x1380e4
            rsi = m.env.data_va(313521);
            // bdabd mov rdi,rax
            rdi = rax;
            // bdac0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdac5 mov DWORD PTR [rbp-0x1dc],0x38
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe24), (0x38u64) as u32);
            // bdacf lea rsi,[rbp-0x1dc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe24);
            // bdad6 mov rdi,rbx
            rdi = rbx;
            // bdad9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdade lea rsi,[rip+0x7a607] # 0x1380ec
            rsi = m.env.data_va(313529);
            // bdae5 mov rdi,rax
            rdi = rax;
            // bdae8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdaed mov DWORD PTR [rbp-0x1e0],0x39
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe20), (0x39u64) as u32);
            // bdaf7 lea rsi,[rbp-0x1e0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe20);
            // bdafe mov rdi,rbx
            rdi = rbx;
            // bdb01 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdb06 lea rsi,[rip+0x7a5e9] # 0x1380f6
            rsi = m.env.data_va(313539);
            // bdb0d mov rdi,rax
            rdi = rax;
            // bdb10 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdb15 mov DWORD PTR [rbp-0x1e4],0x3a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe1c), (0x3au64) as u32);
            // bdb1f lea rsi,[rbp-0x1e4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe1c);
            // bdb26 mov rdi,rbx
            rdi = rbx;
            // bdb29 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdb2e lea rsi,[rip+0x7a5c9] # 0x1380fe
            rsi = m.env.data_va(313547);
            // bdb35 mov rdi,rax
            rdi = rax;
            // bdb38 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdb3d mov DWORD PTR [rbp-0x1e8],0x3b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe18), (0x3bu64) as u32);
            // bdb47 lea rsi,[rbp-0x1e8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe18);
            // bdb4e mov rdi,rbx
            rdi = rbx;
            // bdb51 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdb56 lea rsi,[rip+0x7a5ab] # 0x138108
            rsi = m.env.data_va(313557);
            // bdb5d mov rdi,rax
            rdi = rax;
            // bdb60 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdb65 mov DWORD PTR [rbp-0x1ec],0x3c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe14), (0x3cu64) as u32);
            // bdb6f lea rsi,[rbp-0x1ec]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe14);
            // bdb76 mov rdi,rbx
            rdi = rbx;
            // bdb79 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdb7e lea rsi,[rip+0x7a58b] # 0x138110
            rsi = m.env.data_va(313565);
            // bdb85 mov rdi,rax
            rdi = rax;
            // bdb88 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdb8d mov DWORD PTR [rbp-0x1f0],0x65
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe10), (0x65u64) as u32);
            // bdb97 lea rsi,[rbp-0x1f0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // bdb9e mov rdi,rbx
            rdi = rbx;
            // bdba1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdba6 lea rsi,[rip+0x7a56d] # 0x13811a
            rsi = m.env.data_va(313575);
            // bdbad mov rdi,rax
            rdi = rax;
            // bdbb0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdbb5 mov DWORD PTR [rbp-0x1f4],0x66
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe0c), (0x66u64) as u32);
            // bdbbf lea rsi,[rbp-0x1f4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe0c);
            // bdbc6 mov rdi,rbx
            rdi = rbx;
            // bdbc9 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdbce lea rsi,[rip+0x7a54d] # 0x138122
            rsi = m.env.data_va(313583);
            // bdbd5 mov rdi,rax
            rdi = rax;
            // bdbd8 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdbdd mov DWORD PTR [rbp-0x1f8],0x67
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe08), (0x67u64) as u32);
            // bdbe7 lea rsi,[rbp-0x1f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe08);
            // bdbee mov rdi,rbx
            rdi = rbx;
            // bdbf1 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdbf6 lea rsi,[rip+0x7a52f] # 0x13812c
            rsi = m.env.data_va(313593);
            // bdbfd mov rdi,rax
            rdi = rax;
            // bdc00 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdc05 mov DWORD PTR [rbp-0x1fc],0x68
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe04), (0x68u64) as u32);
            // bdc0f lea rsi,[rbp-0x1fc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe04);
            // bdc16 mov rdi,rbx
            rdi = rbx;
            // bdc19 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdc1e lea rsi,[rip+0x7a511] # 0x138136
            rsi = m.env.data_va(313603);
            // bdc25 mov rdi,rax
            rdi = rax;
            // bdc28 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdc2d mov DWORD PTR [rbp-0x200],0x69
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe00), (0x69u64) as u32);
            // bdc37 lea rsi,[rbp-0x200]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe00);
            // bdc3e mov rdi,rbx
            rdi = rbx;
            // bdc41 call 0xebd20
            { let r = { let t = m.env.stub_va(0x4ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdc46 lea rsi,[rip+0x7a4f3] # 0x138140
            rsi = m.env.data_va(313613);
            // bdc4d mov rdi,rax
            rdi = rax;
            // bdc50 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdc55 mov rbx,QWORD PTR [rip+0x10a9b4] # 0x1c8610
            rbx = m.r64(m.env.data_va(904669)) as u64;
            // bdc5c lea r15,[rip+0x10a9b5] # 0x1c8618
            r15 = m.env.data_va(904677);
            // bdc63 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // bdc66 je 0xbdcc6
            if crate::cov::brk(0xbdc66, fl.e()) { pc = 0xbdcc6; continue; }
            pc = 0xbdc68;
        }
        0xbdc68 => {
            // block bdc68
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bdc68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bdc68 lea r12,[rbp-0x218]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fde8);
            // bdc6f lea r14,[rip+0x10a982] # 0x1c85f8
            r14 = m.env.data_va(904645);
            pc = 0xbdc76;
        }
        0xbdc76 => {
            // block bdc76
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bdc76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bdc76 lea rsi,[rbx+0x28]
            rsi = rbx.wrapping_add(0x28);
            // bdc7a mov rdi,r12
            rdi = r12;
            // bdc7d call 0xbbdbe
            { let r = { let t = m.env.code_va(0xbbdbe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdc82 mov r13d,DWORD PTR [rbx+0x20]
            r13 = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // bdc86 mov rdi,r14
            rdi = r14;
            // bdc89 mov rsi,r12
            rsi = r12;
            // bdc8c call 0xebd50
            { let r = { let t = m.env.stub_va(0x51d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdc91 mov DWORD PTR [rax],r13d
            m.w32(rax, ((r13 as u32 as u64)) as u32);
            // bdc94 mov rdi,r12
            rdi = r12;
            // bdc97 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdc9c mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bdca0 test rax,rax
            fl = logic::<64>(rax & rax);
            // bdca3 je 0xbdcb2
            if crate::cov::brk(0xbdca3, fl.e()) { pc = 0xbdcb2; continue; }
            pc = 0xbdca5;
        }
        0xbdca5 => {
            // block bdca5
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bdca5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bdca5 mov rbx,rax
            rbx = rax;
            // bdca8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bdcab test rax,rax
            fl = logic::<64>(rax & rax);
            // bdcae jne 0xbdca5
            if crate::cov::brk(0xbdcae, fl.ne()) { pc = 0xbdca5; continue; }
            pc = 0xbdcb0;
        }
        0xbdcb0 => {
            // block bdcb0
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bdcb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bdcb0 jmp 0xbdcc1
            pc = 0xbdcc1; continue;
        }
        0xbdcb2 => {
            // block bdcb2
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bdcb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bdcb2 mov rax,rbx
            rax = rbx;
            pc = 0xbdcb5;
        }
        0xbdcb5 => {
            // block bdcb5
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bdcb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bdcb5 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // bdcb9 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // bdcbc mov rax,rbx
            rax = rbx;
            // bdcbf jne 0xbdcb5
            if crate::cov::brk(0xbdcbf, fl.ne()) { pc = 0xbdcb5; continue; }
            pc = 0xbdcc1;
        }
        0xbdcc1 => {
            // block bdcc1
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bdcc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bdcc1 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // bdcc4 jne 0xbdc76
            if crate::cov::brk(0xbdcc4, fl.ne()) { pc = 0xbdc76; continue; }
            pc = 0xbdcc6;
        }
        0xbdcc6 => {
            // block bdcc6
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bdcc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bdcc6 lea rsi,[rip+0x7a47d] # 0x13814a
            rsi = m.env.data_va(313623);
            // bdccd lea rbx,[rbp-0x230]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fdd0);
            // bdcd4 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bdcd9 mov rdi,rbx
            rdi = rbx;
            // bdcdc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdce1 lea rdi,[rip+0x10a910] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bdce8 mov rsi,rbx
            rsi = rbx;
            // bdceb call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdcf0 mov DWORD PTR [rax],0x11
            m.w32(rax, (0x11u64) as u32);
            // bdcf6 lea rdi,[rbp-0x230]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdd0);
            // bdcfd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdd02 lea rsi,[rip+0x7a449] # 0x138152
            rsi = m.env.data_va(313631);
            // bdd09 lea rbx,[rbp-0x248]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fdb8);
            // bdd10 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bdd15 mov rdi,rbx
            rdi = rbx;
            // bdd18 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdd1d lea rdi,[rip+0x10a8d4] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bdd24 mov rsi,rbx
            rsi = rbx;
            // bdd27 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdd2c mov DWORD PTR [rax],0x13
            m.w32(rax, (0x13u64) as u32);
            // bdd32 lea rdi,[rbp-0x248]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdb8);
            // bdd39 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdd3e lea rsi,[rip+0x7a415] # 0x13815a
            rsi = m.env.data_va(313639);
            // bdd45 lea rbx,[rbp-0x260]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // bdd4c mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bdd51 mov rdi,rbx
            rdi = rbx;
            // bdd54 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdd59 lea rdi,[rip+0x10a898] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bdd60 mov rsi,rbx
            rsi = rbx;
            // bdd63 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdd68 mov DWORD PTR [rax],0x15
            m.w32(rax, (0x15u64) as u32);
            // bdd6e lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // bdd75 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdd7a lea rsi,[rip+0x7a3e1] # 0x138162
            rsi = m.env.data_va(313647);
            // bdd81 lea rbx,[rbp-0x278]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd88);
            // bdd88 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bdd8d mov rdi,rbx
            rdi = rbx;
            // bdd90 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdd95 lea rdi,[rip+0x10a85c] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bdd9c mov rsi,rbx
            rsi = rbx;
            // bdd9f call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdda4 mov DWORD PTR [rax],0x17
            m.w32(rax, (0x17u64) as u32);
            // bddaa lea rdi,[rbp-0x278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd88);
            // bddb1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bddb6 lea rsi,[rip+0x7a3ad] # 0x13816a
            rsi = m.env.data_va(313655);
            // bddbd lea rbx,[rbp-0x290]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // bddc4 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bddc9 mov rdi,rbx
            rdi = rbx;
            // bddcc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bddd1 lea rdi,[rip+0x10a820] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bddd8 mov rsi,rbx
            rsi = rbx;
            // bdddb call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdde0 mov DWORD PTR [rax],0x19
            m.w32(rax, (0x19u64) as u32);
            // bdde6 lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // bdded call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bddf2 lea rsi,[rip+0x7a379] # 0x138172
            rsi = m.env.data_va(313663);
            // bddf9 lea rbx,[rbp-0x2a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // bde00 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bde05 mov rdi,rbx
            rdi = rbx;
            // bde08 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bde0d lea rdi,[rip+0x10a7e4] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bde14 mov rsi,rbx
            rsi = rbx;
            // bde17 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bde1c mov DWORD PTR [rax],0x1b
            m.w32(rax, (0x1bu64) as u32);
            // bde22 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // bde29 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bde2e lea rsi,[rip+0x7a345] # 0x13817a
            rsi = m.env.data_va(313671);
            // bde35 lea rbx,[rbp-0x2c0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // bde3c mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bde41 mov rdi,rbx
            rdi = rbx;
            // bde44 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bde49 lea rdi,[rip+0x10a7a8] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bde50 mov rsi,rbx
            rsi = rbx;
            // bde53 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bde58 mov DWORD PTR [rax],0x1d
            m.w32(rax, (0x1du64) as u32);
            // bde5e lea rdi,[rbp-0x2c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // bde65 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bde6a lea rsi,[rip+0x7a311] # 0x138182
            rsi = m.env.data_va(313679);
            // bde71 lea rbx,[rbp-0x2d8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd28);
            // bde78 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bde7d mov rdi,rbx
            rdi = rbx;
            // bde80 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bde85 lea rdi,[rip+0x10a76c] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bde8c mov rsi,rbx
            rsi = rbx;
            // bde8f call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bde94 mov DWORD PTR [rax],0x1f
            m.w32(rax, (0x1fu64) as u32);
            // bde9a lea rdi,[rbp-0x2d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd28);
            // bdea1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdea6 lea rsi,[rip+0x7a2dd] # 0x13818a
            rsi = m.env.data_va(313687);
            // bdead lea rbx,[rbp-0x2f0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd10);
            // bdeb4 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // bdeb9 mov rdi,rbx
            rdi = rbx;
            // bdebc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdec1 lea rdi,[rip+0x10a730] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bdec8 mov rsi,rbx
            rsi = rbx;
            // bdecb call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bded0 mov DWORD PTR [rax],0x56
            m.w32(rax, (0x56u64) as u32);
            // bded6 lea rdi,[rbp-0x2f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd10);
            // bdedd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdee2 lea rsi,[rip+0x7a2ad] # 0x138196
            rsi = m.env.data_va(313699);
            // bdee9 lea rbx,[rbp-0x308]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fcf8);
            // bdef0 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // bdef5 mov rdi,rbx
            rdi = rbx;
            // bdef8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdefd lea rdi,[rip+0x10a6f4] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bdf04 mov rsi,rbx
            rsi = rbx;
            // bdf07 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdf0c mov DWORD PTR [rax],0x58
            m.w32(rax, (0x58u64) as u32);
            // bdf12 lea rdi,[rbp-0x308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf8);
            // bdf19 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdf1e lea rsi,[rip+0x7a27d] # 0x1381a2
            rsi = m.env.data_va(313711);
            // bdf25 lea rbx,[rbp-0x320]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fce0);
            // bdf2c mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // bdf31 mov rdi,rbx
            rdi = rbx;
            // bdf34 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdf39 lea rdi,[rip+0x10a6b8] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bdf40 mov rsi,rbx
            rsi = rbx;
            // bdf43 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdf48 mov DWORD PTR [rax],0x5a
            m.w32(rax, (0x5au64) as u32);
            // bdf4e lea rdi,[rbp-0x320]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fce0);
            // bdf55 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdf5a lea rsi,[rip+0x7a24d] # 0x1381ae
            rsi = m.env.data_va(313723);
            // bdf61 lea rbx,[rbp-0x338]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fcc8);
            // bdf68 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // bdf6d mov rdi,rbx
            rdi = rbx;
            // bdf70 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdf75 lea rdi,[rip+0x10a67c] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bdf7c mov rsi,rbx
            rsi = rbx;
            // bdf7f call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdf84 mov DWORD PTR [rax],0x5c
            m.w32(rax, (0x5cu64) as u32);
            // bdf8a lea rdi,[rbp-0x338]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc8);
            // bdf91 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdf96 lea rsi,[rip+0x7a21d] # 0x1381ba
            rsi = m.env.data_va(313735);
            // bdf9d lea rbx,[rbp-0x350]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fcb0);
            // bdfa4 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // bdfa9 mov rdi,rbx
            rdi = rbx;
            // bdfac call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdfb1 lea rdi,[rip+0x10a640] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bdfb8 mov rsi,rbx
            rsi = rbx;
            // bdfbb call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdfc0 mov DWORD PTR [rax],0x5e
            m.w32(rax, (0x5eu64) as u32);
            // bdfc6 lea rdi,[rbp-0x350]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb0);
            // bdfcd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdfd2 lea rsi,[rip+0x7a1ed] # 0x1381c6
            rsi = m.env.data_va(313747);
            // bdfd9 lea rbx,[rbp-0x368]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc98);
            // bdfe0 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // bdfe5 mov rdi,rbx
            rdi = rbx;
            // bdfe8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdfed lea rdi,[rip+0x10a604] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // bdff4 mov rsi,rbx
            rsi = rbx;
            // bdff7 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bdffc mov DWORD PTR [rax],0x60
            m.w32(rax, (0x60u64) as u32);
            // be002 lea rdi,[rbp-0x368]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc98);
            // be009 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be00e lea rsi,[rip+0x7a1bd] # 0x1381d2
            rsi = m.env.data_va(313759);
            // be015 lea rbx,[rbp-0x380]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc80);
            // be01c mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // be021 mov rdi,rbx
            rdi = rbx;
            // be024 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be029 lea rdi,[rip+0x10a5c8] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be030 mov rsi,rbx
            rsi = rbx;
            // be033 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be038 mov DWORD PTR [rax],0x62
            m.w32(rax, (0x62u64) as u32);
            // be03e lea rdi,[rbp-0x380]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc80);
            // be045 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be04a lea rsi,[rip+0x7a18d] # 0x1381de
            rsi = m.env.data_va(313771);
            // be051 lea rbx,[rbp-0x398]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc68);
            // be058 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // be05d mov rdi,rbx
            rdi = rbx;
            // be060 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be065 lea rdi,[rip+0x10a58c] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be06c mov rsi,rbx
            rsi = rbx;
            // be06f call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be074 mov DWORD PTR [rax],0x64
            m.w32(rax, (0x64u64) as u32);
            // be07a lea rdi,[rbp-0x398]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc68);
            // be081 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be086 lea rsi,[rip+0x7a15d] # 0x1381ea
            rsi = m.env.data_va(313783);
            // be08d lea rbx,[rbp-0x3b0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc50);
            // be094 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // be099 mov rdi,rbx
            rdi = rbx;
            // be09c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be0a1 lea rdi,[rip+0x10a550] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be0a8 mov rsi,rbx
            rsi = rbx;
            // be0ab call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be0b0 mov DWORD PTR [rax],0x81
            m.w32(rax, (0x81u64) as u32);
            // be0b6 lea rdi,[rbp-0x3b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc50);
            // be0bd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be0c2 lea rsi,[rip+0x7a12a] # 0x1381f3
            rsi = m.env.data_va(313792);
            // be0c9 lea rbx,[rbp-0x3c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc38);
            // be0d0 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // be0d5 mov rdi,rbx
            rdi = rbx;
            // be0d8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be0dd lea rdi,[rip+0x10a514] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be0e4 mov rsi,rbx
            rsi = rbx;
            // be0e7 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be0ec mov DWORD PTR [rax],0x81
            m.w32(rax, (0x81u64) as u32);
            // be0f2 lea rdi,[rbp-0x3c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc38);
            // be0f9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be0fe lea rsi,[rip+0x7a0f9] # 0x1381fe
            rsi = m.env.data_va(313803);
            // be105 lea rbx,[rbp-0x3e0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc20);
            // be10c mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // be111 mov rdi,rbx
            rdi = rbx;
            // be114 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be119 lea rdi,[rip+0x10a4d8] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be120 mov rsi,rbx
            rsi = rbx;
            // be123 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be128 mov DWORD PTR [rax],0x82
            m.w32(rax, (0x82u64) as u32);
            // be12e lea rdi,[rbp-0x3e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc20);
            // be135 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be13a lea rsi,[rip+0x7a0c5] # 0x138206
            rsi = m.env.data_va(313811);
            // be141 lea rbx,[rbp-0x3f8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc08);
            // be148 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // be14d mov rdi,rbx
            rdi = rbx;
            // be150 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be155 lea rdi,[rip+0x10a49c] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be15c mov rsi,rbx
            rsi = rbx;
            // be15f call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be164 mov DWORD PTR [rax],0x82
            m.w32(rax, (0x82u64) as u32);
            // be16a lea rdi,[rbp-0x3f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc08);
            // be171 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be176 lea rsi,[rip+0x7a093] # 0x138210
            rsi = m.env.data_va(313821);
            // be17d lea rbx,[rbp-0x410]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fbf0);
            // be184 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // be189 mov rdi,rbx
            rdi = rbx;
            // be18c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be191 lea rdi,[rip+0x10a460] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be198 mov rsi,rbx
            rsi = rbx;
            // be19b call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be1a0 mov DWORD PTR [rax],0x76
            m.w32(rax, (0x76u64) as u32);
            // be1a6 lea rdi,[rbp-0x410]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbf0);
            // be1ad call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be1b2 lea rsi,[rip+0x7a05f] # 0x138218
            rsi = m.env.data_va(313829);
            // be1b9 lea rbx,[rbp-0x428]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fbd8);
            // be1c0 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // be1c5 mov rdi,rbx
            rdi = rbx;
            // be1c8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be1cd lea rdi,[rip+0x10a424] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be1d4 mov rsi,rbx
            rsi = rbx;
            // be1d7 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be1dc mov DWORD PTR [rax],0x2e
            m.w32(rax, (0x2eu64) as u32);
            // be1e2 lea rdi,[rbp-0x428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd8);
            // be1e9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be1ee lea rsi,[rip+0x7a02e] # 0x138223
            rsi = m.env.data_va(313840);
            // be1f5 lea rbx,[rbp-0x440]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fbc0);
            // be1fc mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // be201 mov rdi,rbx
            rdi = rbx;
            // be204 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be209 lea rdi,[rip+0x10a3e8] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be210 mov rsi,rbx
            rsi = rbx;
            // be213 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be218 mov DWORD PTR [rax],0x30
            m.w32(rax, (0x30u64) as u32);
            // be21e lea rdi,[rbp-0x440]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbc0);
            // be225 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be22a lea rsi,[rip+0x79ffd] # 0x13822e
            rsi = m.env.data_va(313851);
            // be231 lea rbx,[rbp-0x458]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fba8);
            // be238 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // be23d mov rdi,rbx
            rdi = rbx;
            // be240 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be245 lea rdi,[rip+0x10a3ac] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be24c mov rsi,rbx
            rsi = rbx;
            // be24f call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be254 mov DWORD PTR [rax],0x32
            m.w32(rax, (0x32u64) as u32);
            // be25a lea rdi,[rbp-0x458]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fba8);
            // be261 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be266 lea rsi,[rip+0x79fcc] # 0x138239
            rsi = m.env.data_va(313862);
            // be26d lea rbx,[rbp-0x470]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // be274 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // be279 mov rdi,rbx
            rdi = rbx;
            // be27c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be281 lea rdi,[rip+0x10a370] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be288 mov rsi,rbx
            rsi = rbx;
            // be28b call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be290 mov DWORD PTR [rax],0x34
            m.w32(rax, (0x34u64) as u32);
            // be296 lea rdi,[rbp-0x470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // be29d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be2a2 lea rsi,[rip+0x79f9b] # 0x138244
            rsi = m.env.data_va(313873);
            // be2a9 lea rbx,[rbp-0x488]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb78);
            // be2b0 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // be2b5 mov rdi,rbx
            rdi = rbx;
            // be2b8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be2bd lea rdi,[rip+0x10a334] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be2c4 mov rsi,rbx
            rsi = rbx;
            // be2c7 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be2cc mov DWORD PTR [rax],0x36
            m.w32(rax, (0x36u64) as u32);
            // be2d2 lea rdi,[rbp-0x488]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb78);
            // be2d9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be2de lea rsi,[rip+0x79f6a] # 0x13824f
            rsi = m.env.data_va(313884);
            // be2e5 lea rbx,[rbp-0x4a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb60);
            // be2ec mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // be2f1 mov rdi,rbx
            rdi = rbx;
            // be2f4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be2f9 lea rdi,[rip+0x10a2f8] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be300 mov rsi,rbx
            rsi = rbx;
            // be303 call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be308 mov DWORD PTR [rax],0x38
            m.w32(rax, (0x38u64) as u32);
            // be30e lea rdi,[rbp-0x4a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb60);
            // be315 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be31a lea rsi,[rip+0x79f39] # 0x13825a
            rsi = m.env.data_va(313895);
            // be321 lea rbx,[rbp-0x4b8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb48);
            // be328 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // be32d mov rdi,rbx
            rdi = rbx;
            // be330 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be335 lea rdi,[rip+0x10a2bc] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be33c mov rsi,rbx
            rsi = rbx;
            // be33f call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be344 mov DWORD PTR [rax],0x3a
            m.w32(rax, (0x3au64) as u32);
            // be34a lea rdi,[rbp-0x4b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb48);
            // be351 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be356 lea rsi,[rip+0x79f08] # 0x138265
            rsi = m.env.data_va(313906);
            // be35d lea rbx,[rbp-0x4d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fb30);
            // be364 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // be369 mov rdi,rbx
            rdi = rbx;
            // be36c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be371 lea rdi,[rip+0x10a280] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // be378 mov rsi,rbx
            rsi = rbx;
            // be37b call 0xebd4a
            { let r = { let t = m.env.stub_va(0x517); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be380 mov DWORD PTR [rax],0x3c
            m.w32(rax, (0x3cu64) as u32);
            // be386 lea rdi,[rbp-0x4d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb30);
            // be38d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be392 mov DWORD PTR [rbp-0x4d4],0xe8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb2c), (0xe8u64) as u32);
            // be39c lea rbx,[rip+0x10a2b5] # 0x1c8658
            rbx = m.env.data_va(904741);
            // be3a3 lea rsi,[rbp-0x4d4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb2c);
            // be3aa mov rdi,rbx
            rdi = rbx;
            // be3ad call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be3b2 lea rsi,[rip+0x79eb7] # 0x138270
            rsi = m.env.data_va(313917);
            // be3b9 mov rdi,rax
            rdi = rax;
            // be3bc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be3c1 mov DWORD PTR [rbp-0x4d8],0x68
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb28), (0x68u64) as u32);
            // be3cb lea rsi,[rbp-0x4d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb28);
            // be3d2 mov rdi,rbx
            rdi = rbx;
            // be3d5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be3da lea rsi,[rip+0x79e9b] # 0x13827c
            rsi = m.env.data_va(313929);
            // be3e1 mov rdi,rax
            rdi = rax;
            // be3e4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be3e9 mov DWORD PTR [rbp-0x4dc],0xe9
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb24), (0xe9u64) as u32);
            // be3f3 lea rsi,[rbp-0x4dc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb24);
            // be3fa mov rdi,rbx
            rdi = rbx;
            // be3fd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be402 lea rsi,[rip+0x79e7f] # 0x138288
            rsi = m.env.data_va(313941);
            // be409 mov rdi,rax
            rdi = rax;
            // be40c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be411 mov DWORD PTR [rbp-0x4e0],0x69
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb20), (0x69u64) as u32);
            // be41b lea rsi,[rbp-0x4e0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb20);
            // be422 mov rdi,rbx
            rdi = rbx;
            // be425 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be42a lea rsi,[rip+0x79e63] # 0x138294
            rsi = m.env.data_va(313953);
            // be431 mov rdi,rax
            rdi = rax;
            // be434 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be439 mov DWORD PTR [rbp-0x4e4],0xea
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb1c), (0xeau64) as u32);
            // be443 lea rsi,[rbp-0x4e4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb1c);
            // be44a mov rdi,rbx
            rdi = rbx;
            // be44d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be452 lea rsi,[rip+0x79e47] # 0x1382a0
            rsi = m.env.data_va(313965);
            // be459 mov rdi,rax
            rdi = rax;
            // be45c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be461 mov DWORD PTR [rbp-0x4e8],0x6a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb18), (0x6au64) as u32);
            // be46b lea rsi,[rbp-0x4e8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb18);
            // be472 mov rdi,rbx
            rdi = rbx;
            // be475 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be47a lea rsi,[rip+0x79e2b] # 0x1382ac
            rsi = m.env.data_va(313977);
            // be481 mov rdi,rax
            rdi = rax;
            // be484 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be489 mov DWORD PTR [rbp-0x4ec],0xeb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb14), (0xebu64) as u32);
            // be493 lea rsi,[rbp-0x4ec]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb14);
            // be49a mov rdi,rbx
            rdi = rbx;
            // be49d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be4a2 lea rsi,[rip+0x79e0f] # 0x1382b8
            rsi = m.env.data_va(313989);
            // be4a9 mov rdi,rax
            rdi = rax;
            // be4ac call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be4b1 mov DWORD PTR [rbp-0x4f0],0x6b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb10), (0x6bu64) as u32);
            // be4bb lea rsi,[rbp-0x4f0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb10);
            // be4c2 mov rdi,rbx
            rdi = rbx;
            // be4c5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be4ca lea rsi,[rip+0x79df3] # 0x1382c4
            rsi = m.env.data_va(314001);
            // be4d1 mov rdi,rax
            rdi = rax;
            // be4d4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be4d9 mov DWORD PTR [rbp-0x4f4],0xec
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb0c), (0xecu64) as u32);
            // be4e3 lea rsi,[rbp-0x4f4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb0c);
            // be4ea mov rdi,rbx
            rdi = rbx;
            // be4ed call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be4f2 lea rsi,[rip+0x79dd7] # 0x1382d0
            rsi = m.env.data_va(314013);
            // be4f9 mov rdi,rax
            rdi = rax;
            // be4fc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be501 mov DWORD PTR [rbp-0x4f8],0x6c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb08), (0x6cu64) as u32);
            // be50b lea rsi,[rbp-0x4f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb08);
            // be512 mov rdi,rbx
            rdi = rbx;
            // be515 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be51a lea rsi,[rip+0x79dbb] # 0x1382dc
            rsi = m.env.data_va(314025);
            // be521 mov rdi,rax
            rdi = rax;
            // be524 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be529 mov DWORD PTR [rbp-0x4fc],0xed
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb04), (0xedu64) as u32);
            // be533 lea rsi,[rbp-0x4fc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb04);
            // be53a mov rdi,rbx
            rdi = rbx;
            // be53d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be542 lea rsi,[rip+0x79d9f] # 0x1382e8
            rsi = m.env.data_va(314037);
            // be549 mov rdi,rax
            rdi = rax;
            // be54c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be551 mov DWORD PTR [rbp-0x500],0x6d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb00), (0x6du64) as u32);
            // be55b lea rsi,[rbp-0x500]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb00);
            // be562 mov rdi,rbx
            rdi = rbx;
            // be565 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be56a lea rsi,[rip+0x79d83] # 0x1382f4
            rsi = m.env.data_va(314049);
            // be571 mov rdi,rax
            rdi = rax;
            // be574 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be579 mov DWORD PTR [rbp-0x504],0xee
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fafc), (0xeeu64) as u32);
            // be583 lea rsi,[rbp-0x504]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fafc);
            // be58a mov rdi,rbx
            rdi = rbx;
            // be58d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be592 lea rsi,[rip+0x79d67] # 0x138300
            rsi = m.env.data_va(314061);
            // be599 mov rdi,rax
            rdi = rax;
            // be59c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be5a1 mov DWORD PTR [rbp-0x508],0x6e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_faf8), (0x6eu64) as u32);
            // be5ab lea rsi,[rbp-0x508]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_faf8);
            // be5b2 mov rdi,rbx
            rdi = rbx;
            // be5b5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be5ba lea rsi,[rip+0x79d4b] # 0x13830c
            rsi = m.env.data_va(314073);
            // be5c1 mov rdi,rax
            rdi = rax;
            // be5c4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be5c9 mov DWORD PTR [rbp-0x50c],0xef
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_faf4), (0xefu64) as u32);
            // be5d3 lea rsi,[rbp-0x50c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_faf4);
            // be5da mov rdi,rbx
            rdi = rbx;
            // be5dd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be5e2 lea rsi,[rip+0x79d2f] # 0x138318
            rsi = m.env.data_va(314085);
            // be5e9 mov rdi,rax
            rdi = rax;
            // be5ec call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be5f1 mov DWORD PTR [rbp-0x510],0x6f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_faf0), (0x6fu64) as u32);
            // be5fb lea rsi,[rbp-0x510]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_faf0);
            // be602 mov rdi,rbx
            rdi = rbx;
            // be605 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be60a lea rsi,[rip+0x79d13] # 0x138324
            rsi = m.env.data_va(314097);
            // be611 mov rdi,rax
            rdi = rax;
            // be614 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be619 mov DWORD PTR [rbp-0x514],0x44
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_faec), (0x44u64) as u32);
            // be623 lea rsi,[rbp-0x514]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_faec);
            // be62a mov rdi,rbx
            rdi = rbx;
            // be62d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be632 lea rsi,[rip+0x70578] # 0x12ebb1
            rsi = m.env.data_va(275326);
            // be639 mov rdi,rax
            rdi = rax;
            // be63c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be641 mov DWORD PTR [rbp-0x518],0xc4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fae8), (0xc4u64) as u32);
            // be64b lea rsi,[rbp-0x518]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fae8);
            // be652 mov rdi,rbx
            rdi = rbx;
            // be655 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be65a lea rsi,[rip+0x79ccf] # 0x138330
            rsi = m.env.data_va(314109);
            // be661 mov rdi,rax
            rdi = rax;
            // be664 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be669 mov DWORD PTR [rbp-0x51c],0xe
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fae4), (0xeu64) as u32);
            // be673 lea rsi,[rbp-0x51c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fae4);
            // be67a mov rdi,rbx
            rdi = rbx;
            // be67d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be682 lea rsi,[rip+0x79869] # 0x137ef2
            rsi = m.env.data_va(313023);
            // be689 mov rdi,rax
            rdi = rax;
            // be68c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be691 mov DWORD PTR [rbp-0x520],0x85
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fae0), (0x85u64) as u32);
            // be69b lea rsi,[rbp-0x520]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fae0);
            // be6a2 mov rdi,rbx
            rdi = rbx;
            // be6a5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be6aa lea rsi,[rip+0x79c87] # 0x138338
            rsi = m.env.data_va(314117);
            // be6b1 mov rdi,rax
            rdi = rax;
            // be6b4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be6b9 mov DWORD PTR [rbp-0x524],0x5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fadc), (0x5u64) as u32);
            // be6c3 lea rsi,[rbp-0x524]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fadc);
            // be6ca mov rdi,rbx
            rdi = rbx;
            // be6cd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be6d2 lea rsi,[rip+0x6fed1] # 0x12e5aa
            rsi = m.env.data_va(273783);
            // be6d9 mov rdi,rax
            rdi = rax;
            // be6dc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be6e1 mov DWORD PTR [rbp-0x528],0x11
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fad8), (0x11u64) as u32);
            // be6eb lea rsi,[rbp-0x528]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fad8);
            // be6f2 mov rdi,rbx
            rdi = rbx;
            // be6f5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be6fa lea rsi,[rip+0x797f9] # 0x137efa
            rsi = m.env.data_va(313031);
            // be701 mov rdi,rax
            rdi = rax;
            // be704 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be709 mov DWORD PTR [rbp-0x52c],0x90
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fad4), (0x90u64) as u32);
            // be713 lea rsi,[rbp-0x52c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fad4);
            // be71a mov rdi,rbx
            rdi = rbx;
            // be71d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be722 lea rsi,[rip+0x79c17] # 0x138340
            rsi = m.env.data_va(314125);
            // be729 mov rdi,rax
            rdi = rax;
            // be72c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be731 mov DWORD PTR [rbp-0x530],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fad0), (0x10u64) as u32);
            // be73b lea rsi,[rbp-0x530]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fad0);
            // be742 mov rdi,rbx
            rdi = rbx;
            // be745 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be74a lea rsi,[rip+0x6fe6b] # 0x12e5bc
            rsi = m.env.data_va(273801);
            // be751 mov rdi,rax
            rdi = rax;
            // be754 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be759 mov DWORD PTR [rbp-0x534],0x3b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_facc), (0x3bu64) as u32);
            // be763 lea rsi,[rbp-0x534]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_facc);
            // be76a mov rdi,rbx
            rdi = rbx;
            // be76d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be772 lea rsi,[rip+0x79789] # 0x137f02
            rsi = m.env.data_va(313039);
            // be779 mov rdi,rax
            rdi = rax;
            // be77c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be781 mov DWORD PTR [rbp-0x538],0xba
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fac8), (0xbau64) as u32);
            // be78b lea rsi,[rbp-0x538]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fac8);
            // be792 mov rdi,rbx
            rdi = rbx;
            // be795 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be79a lea rsi,[rip+0x79ba7] # 0x138348
            rsi = m.env.data_va(314133);
            // be7a1 mov rdi,rax
            rdi = rax;
            // be7a4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be7a9 mov DWORD PTR [rbp-0x53c],0x3a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fac4), (0x3au64) as u32);
            // be7b3 lea rsi,[rbp-0x53c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fac4);
            // be7ba mov rdi,rbx
            rdi = rbx;
            // be7bd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be7c2 lea rsi,[rip+0x70328] # 0x12eaf1
            rsi = m.env.data_va(275134);
            // be7c9 mov rdi,rax
            rdi = rax;
            // be7cc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be7d1 mov DWORD PTR [rbp-0x540],0x3d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fac0), (0x3du64) as u32);
            // be7db lea rsi,[rbp-0x540]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fac0);
            // be7e2 mov rdi,rbx
            rdi = rbx;
            // be7e5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be7ea lea rsi,[rip+0x79719] # 0x137f0a
            rsi = m.env.data_va(313047);
            // be7f1 mov rdi,rax
            rdi = rax;
            // be7f4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be7f9 mov DWORD PTR [rbp-0x544],0xbc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fabc), (0xbcu64) as u32);
            // be803 lea rsi,[rbp-0x544]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fabc);
            // be80a mov rdi,rbx
            rdi = rbx;
            // be80d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be812 lea rsi,[rip+0x79b37] # 0x138350
            rsi = m.env.data_va(314141);
            // be819 mov rdi,rax
            rdi = rax;
            // be81c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be821 mov DWORD PTR [rbp-0x548],0x3c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fab8), (0x3cu64) as u32);
            // be82b lea rsi,[rbp-0x548]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fab8);
            // be832 mov rdi,rbx
            rdi = rbx;
            // be835 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be83a lea rsi,[rip+0x702c2] # 0x12eb03
            rsi = m.env.data_va(275152);
            // be841 mov rdi,rax
            rdi = rax;
            // be844 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be849 mov DWORD PTR [rbp-0x54c],0x2d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fab4), (0x2du64) as u32);
            // be853 lea rsi,[rbp-0x54c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fab4);
            // be85a mov rdi,rbx
            rdi = rbx;
            // be85d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be862 lea rsi,[rip+0x796a9] # 0x137f12
            rsi = m.env.data_va(313055);
            // be869 mov rdi,rax
            rdi = rax;
            // be86c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be871 mov DWORD PTR [rbp-0x550],0xac
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fab0), (0xacu64) as u32);
            // be87b lea rsi,[rbp-0x550]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fab0);
            // be882 mov rdi,rbx
            rdi = rbx;
            // be885 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be88a lea rsi,[rip+0x79ac7] # 0x138358
            rsi = m.env.data_va(314149);
            // be891 mov rdi,rax
            rdi = rax;
            // be894 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be899 mov DWORD PTR [rbp-0x554],0x2c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_faac), (0x2cu64) as u32);
            // be8a3 lea rsi,[rbp-0x554]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_faac);
            // be8aa mov rdi,rbx
            rdi = rbx;
            // be8ad call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be8b2 lea rsi,[rip+0x702b4] # 0x12eb6d
            rsi = m.env.data_va(275258);
            // be8b9 mov rdi,rax
            rdi = rax;
            // be8bc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be8c1 mov DWORD PTR [rbp-0x558],0x5a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_faa8), (0x5au64) as u32);
            // be8cb lea rsi,[rbp-0x558]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_faa8);
            // be8d2 mov rdi,rbx
            rdi = rbx;
            // be8d5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be8da lea rsi,[rip+0x79639] # 0x137f1a
            rsi = m.env.data_va(313063);
            // be8e1 mov rdi,rax
            rdi = rax;
            // be8e4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be8e9 mov DWORD PTR [rbp-0x55c],0xd9
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_faa4), (0xd9u64) as u32);
            // be8f3 lea rsi,[rbp-0x55c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_faa4);
            // be8fa mov rdi,rbx
            rdi = rbx;
            // be8fd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be902 lea rsi,[rip+0x79a57] # 0x138360
            rsi = m.env.data_va(314157);
            // be909 mov rdi,rax
            rdi = rax;
            // be90c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be911 mov DWORD PTR [rbp-0x560],0x59
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_faa0), (0x59u64) as u32);
            // be91b lea rsi,[rbp-0x560]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_faa0);
            // be922 mov rdi,rbx
            rdi = rbx;
            // be925 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be92a lea rsi,[rip+0x7044d] # 0x12ed7e
            rsi = m.env.data_va(275787);
            // be931 mov rdi,rax
            rdi = rax;
            // be934 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be939 mov DWORD PTR [rbp-0x564],0x5c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa9c), (0x5cu64) as u32);
            // be943 lea rsi,[rbp-0x564]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa9c);
            // be94a mov rdi,rbx
            rdi = rbx;
            // be94d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be952 lea rsi,[rip+0x795c9] # 0x137f22
            rsi = m.env.data_va(313071);
            // be959 mov rdi,rax
            rdi = rax;
            // be95c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be961 mov DWORD PTR [rbp-0x568],0xdb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa98), (0xdbu64) as u32);
            // be96b lea rsi,[rbp-0x568]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa98);
            // be972 mov rdi,rbx
            rdi = rbx;
            // be975 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be97a lea rsi,[rip+0x799e7] # 0x138368
            rsi = m.env.data_va(314165);
            // be981 mov rdi,rax
            rdi = rax;
            // be984 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be989 mov DWORD PTR [rbp-0x56c],0x5b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa94), (0x5bu64) as u32);
            // be993 lea rsi,[rbp-0x56c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa94);
            // be99a mov rdi,rbx
            rdi = rbx;
            // be99d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be9a2 lea rsi,[rip+0x703e7] # 0x12ed90
            rsi = m.env.data_va(275805);
            // be9a9 mov rdi,rax
            rdi = rax;
            // be9ac call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be9b1 mov DWORD PTR [rbp-0x570],0x5e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa90), (0x5eu64) as u32);
            // be9bb lea rsi,[rbp-0x570]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa90);
            // be9c2 mov rdi,rbx
            rdi = rbx;
            // be9c5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be9ca lea rsi,[rip+0x79559] # 0x137f2a
            rsi = m.env.data_va(313079);
            // be9d1 mov rdi,rax
            rdi = rax;
            // be9d4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be9d9 mov DWORD PTR [rbp-0x574],0xdd
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa8c), (0xddu64) as u32);
            // be9e3 lea rsi,[rbp-0x574]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa8c);
            // be9ea mov rdi,rbx
            rdi = rbx;
            // be9ed call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // be9f2 lea rsi,[rip+0x79977] # 0x138370
            rsi = m.env.data_va(314173);
            // be9f9 mov rdi,rax
            rdi = rax;
            // be9fc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bea01 mov DWORD PTR [rbp-0x578],0x5d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa88), (0x5du64) as u32);
            // bea0b lea rsi,[rbp-0x578]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa88);
            // bea12 mov rdi,rbx
            rdi = rbx;
            // bea15 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bea1a lea rsi,[rip+0x70381] # 0x12eda2
            rsi = m.env.data_va(275823);
            // bea21 mov rdi,rax
            rdi = rax;
            // bea24 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bea29 mov DWORD PTR [rbp-0x57c],0x83
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa84), (0x83u64) as u32);
            // bea33 lea rsi,[rbp-0x57c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa84);
            // bea3a mov rdi,rbx
            rdi = rbx;
            // bea3d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bea42 lea rsi,[rip+0x6fd3b] # 0x12e784
            rsi = m.env.data_va(274257);
            // bea49 mov rdi,rax
            rdi = rax;
            // bea4c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bea51 mov DWORD PTR [rbp-0x580],0xa8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa80), (0xa8u64) as u32);
            // bea5b lea rsi,[rbp-0x580]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa80);
            // bea62 mov rdi,rbx
            rdi = rbx;
            // bea65 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bea6a lea rsi,[rip+0x794c9] # 0x137f3a
            rsi = m.env.data_va(313095);
            // bea71 mov rdi,rax
            rdi = rax;
            // bea74 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bea79 mov DWORD PTR [rbp-0x584],0xb4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa7c), (0xb4u64) as u32);
            // bea83 lea rsi,[rbp-0x584]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa7c);
            // bea8a mov rdi,rbx
            rdi = rbx;
            // bea8d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bea92 lea rsi,[rip+0x794af] # 0x137f48
            rsi = m.env.data_va(313109);
            // bea99 mov rdi,rax
            rdi = rax;
            // bea9c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beaa1 mov DWORD PTR [rbp-0x588],0xb5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa78), (0xb5u64) as u32);
            // beaab lea rsi,[rbp-0x588]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa78);
            // beab2 mov rdi,rbx
            rdi = rbx;
            // beab5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beaba lea rsi,[rip+0x79495] # 0x137f56
            rsi = m.env.data_va(313123);
            // beac1 mov rdi,rax
            rdi = rax;
            // beac4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beac9 mov DWORD PTR [rbp-0x58c],0xcd
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa74), (0xcdu64) as u32);
            // bead3 lea rsi,[rbp-0x58c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa74);
            // beada mov rdi,rbx
            rdi = rbx;
            // beadd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beae2 lea rsi,[rip+0x7947b] # 0x137f64
            rsi = m.env.data_va(313137);
            // beae9 mov rdi,rax
            rdi = rax;
            // beaec call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beaf1 mov DWORD PTR [rbp-0x590],0xce
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa70), (0xceu64) as u32);
            // beafb lea rsi,[rbp-0x590]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa70);
            // beb02 mov rdi,rbx
            rdi = rbx;
            // beb05 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beb0a lea rsi,[rip+0x79461] # 0x137f72
            rsi = m.env.data_va(313151);
            // beb11 mov rdi,rax
            rdi = rax;
            // beb14 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beb19 mov DWORD PTR [rbp-0x594],0xcf
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa6c), (0xcfu64) as u32);
            // beb23 lea rsi,[rbp-0x594]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa6c);
            // beb2a mov rdi,rbx
            rdi = rbx;
            // beb2d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beb32 lea rsi,[rip+0x79447] # 0x137f80
            rsi = m.env.data_va(313165);
            // beb39 mov rdi,rax
            rdi = rax;
            // beb3c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beb41 mov DWORD PTR [rbp-0x598],0xd0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa68), (0xd0u64) as u32);
            // beb4b lea rsi,[rbp-0x598]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa68);
            // beb52 mov rdi,rbx
            rdi = rbx;
            // beb55 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beb5a lea rsi,[rip+0x7942d] # 0x137f8e
            rsi = m.env.data_va(313179);
            // beb61 mov rdi,rax
            rdi = rax;
            // beb64 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beb69 mov DWORD PTR [rbp-0x59c],0xb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa64), (0xbu64) as u32);
            // beb73 lea rsi,[rbp-0x59c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa64);
            // beb7a mov rdi,rbx
            rdi = rbx;
            // beb7d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beb82 lea rsi,[rip+0x6fc51] # 0x12e7da
            rsi = m.env.data_va(274343);
            // beb89 mov rdi,rax
            rdi = rax;
            // beb8c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beb91 mov DWORD PTR [rbp-0x5a0],0x26
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa60), (0x26u64) as u32);
            // beb9b lea rsi,[rbp-0x5a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa60);
            // beba2 mov rdi,rbx
            rdi = rbx;
            // beba5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bebaa lea rsi,[rip+0x797c7] # 0x138378
            rsi = m.env.data_va(314181);
            // bebb1 mov rdi,rax
            rdi = rax;
            // bebb4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bebb9 mov DWORD PTR [rbp-0x5a4],0x1c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa5c), (0x1cu64) as u32);
            // bebc3 lea rsi,[rbp-0x5a4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa5c);
            // bebca mov rdi,rbx
            rdi = rbx;
            // bebcd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bebd2 lea rsi,[rip+0x6fc1d] # 0x12e7f6
            rsi = m.env.data_va(274371);
            // bebd9 mov rdi,rax
            rdi = rax;
            // bebdc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bebe1 mov DWORD PTR [rbp-0x5a8],0x27
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa58), (0x27u64) as u32);
            // bebeb lea rsi,[rbp-0x5a8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa58);
            // bebf2 mov rdi,rbx
            rdi = rbx;
            // bebf5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bebfa lea rsi,[rip+0x79780] # 0x138381
            rsi = m.env.data_va(314190);
            // bec01 mov rdi,rax
            rdi = rax;
            // bec04 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bec09 mov DWORD PTR [rbp-0x5ac],0x36
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa54), (0x36u64) as u32);
            // bec13 lea rsi,[rbp-0x5ac]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa54);
            // bec1a mov rdi,rbx
            rdi = rbx;
            // bec1d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bec22 lea rsi,[rip+0x79373] # 0x137f9c
            rsi = m.env.data_va(313193);
            // bec29 mov rdi,rax
            rdi = rax;
            // bec2c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bec31 mov DWORD PTR [rbp-0x5b0],0x37
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa50), (0x37u64) as u32);
            // bec3b lea rsi,[rbp-0x5b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa50);
            // bec42 mov rdi,rbx
            rdi = rbx;
            // bec45 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bec4a lea rsi,[rip+0x79739] # 0x13838a
            rsi = m.env.data_va(314199);
            // bec51 mov rdi,rax
            rdi = rax;
            // bec54 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bec59 mov DWORD PTR [rbp-0x5b4],0x38
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa4c), (0x38u64) as u32);
            // bec63 lea rsi,[rbp-0x5b4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa4c);
            // bec6a mov rdi,rbx
            rdi = rbx;
            // bec6d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bec72 lea rsi,[rip+0x7932a] # 0x137fa3
            rsi = m.env.data_va(313200);
            // bec79 mov rdi,rax
            rdi = rax;
            // bec7c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bec81 mov DWORD PTR [rbp-0x5b8],0x39
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa48), (0x39u64) as u32);
            // bec8b lea rsi,[rbp-0x5b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa48);
            // bec92 mov rdi,rbx
            rdi = rbx;
            // bec95 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bec9a lea rsi,[rip+0x796f2] # 0x138393
            rsi = m.env.data_va(314208);
            // beca1 mov rdi,rax
            rdi = rax;
            // beca4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beca9 mov DWORD PTR [rbp-0x5bc],0x3e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa44), (0x3eu64) as u32);
            // becb3 lea rsi,[rbp-0x5bc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa44);
            // becba mov rdi,rbx
            rdi = rbx;
            // becbd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // becc2 lea rsi,[rip+0x792e1] # 0x137faa
            rsi = m.env.data_va(313207);
            // becc9 mov rdi,rax
            rdi = rax;
            // beccc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // becd1 mov DWORD PTR [rbp-0x5c0],0x3f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa40), (0x3fu64) as u32);
            // becdb lea rsi,[rbp-0x5c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa40);
            // bece2 mov rdi,rbx
            rdi = rbx;
            // bece5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // becea lea rsi,[rip+0x796ab] # 0x13839c
            rsi = m.env.data_va(314217);
            // becf1 mov rdi,rax
            rdi = rax;
            // becf4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // becf9 mov DWORD PTR [rbp-0x5c4],0x62
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa3c), (0x62u64) as u32);
            // bed03 lea rsi,[rbp-0x5c4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa3c);
            // bed0a mov rdi,rbx
            rdi = rbx;
            // bed0d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bed12 lea rsi,[rip+0x79298] # 0x137fb1
            rsi = m.env.data_va(313214);
            // bed19 mov rdi,rax
            rdi = rax;
            // bed1c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bed21 mov DWORD PTR [rbp-0x5c8],0x63
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa38), (0x63u64) as u32);
            // bed2b lea rsi,[rbp-0x5c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa38);
            // bed32 mov rdi,rbx
            rdi = rbx;
            // bed35 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bed3a lea rsi,[rip+0x79664] # 0x1383a5
            rsi = m.env.data_va(314226);
            // bed41 mov rdi,rax
            rdi = rax;
            // bed44 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bed49 mov DWORD PTR [rbp-0x5cc],0x64
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa34), (0x64u64) as u32);
            // bed53 lea rsi,[rbp-0x5cc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa34);
            // bed5a mov rdi,rbx
            rdi = rbx;
            // bed5d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bed62 lea rsi,[rip+0x7924f] # 0x137fb8
            rsi = m.env.data_va(313221);
            // bed69 mov rdi,rax
            rdi = rax;
            // bed6c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bed71 mov DWORD PTR [rbp-0x5d0],0x65
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa30), (0x65u64) as u32);
            // bed7b lea rsi,[rbp-0x5d0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa30);
            // bed82 mov rdi,rbx
            rdi = rbx;
            // bed85 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bed8a lea rsi,[rip+0x7961d] # 0x1383ae
            rsi = m.env.data_va(314235);
            // bed91 mov rdi,rax
            rdi = rax;
            // bed94 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bed99 mov DWORD PTR [rbp-0x5d4],0x66
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa2c), (0x66u64) as u32);
            // beda3 lea rsi,[rbp-0x5d4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa2c);
            // bedaa mov rdi,rbx
            rdi = rbx;
            // bedad call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bedb2 lea rsi,[rip+0x79206] # 0x137fbf
            rsi = m.env.data_va(313228);
            // bedb9 mov rdi,rax
            rdi = rax;
            // bedbc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bedc1 mov DWORD PTR [rbp-0x5d8],0x67
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa28), (0x67u64) as u32);
            // bedcb lea rsi,[rbp-0x5d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa28);
            // bedd2 mov rdi,rbx
            rdi = rbx;
            // bedd5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bedda lea rsi,[rip+0x795d6] # 0x1383b7
            rsi = m.env.data_va(314244);
            // bede1 mov rdi,rax
            rdi = rax;
            // bede4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bede9 mov DWORD PTR [rbp-0x5dc],0xf0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa24), (0xf0u64) as u32);
            // bedf3 lea rsi,[rbp-0x5dc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa24);
            // bedfa mov rdi,rbx
            rdi = rbx;
            // bedfd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bee02 lea rsi,[rip+0x795b7] # 0x1383c0
            rsi = m.env.data_va(314253);
            // bee09 mov rdi,rax
            rdi = rax;
            // bee0c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bee11 mov DWORD PTR [rbp-0x5e0],0x70
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa20), (0x70u64) as u32);
            // bee1b lea rsi,[rbp-0x5e0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa20);
            // bee22 mov rdi,rbx
            rdi = rbx;
            // bee25 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bee2a lea rsi,[rip+0x79599] # 0x1383ca
            rsi = m.env.data_va(314263);
            // bee31 mov rdi,rax
            rdi = rax;
            // bee34 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bee39 mov DWORD PTR [rbp-0x5e4],0x88
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa1c), (0x88u64) as u32);
            // bee43 lea rsi,[rbp-0x5e4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa1c);
            // bee4a mov rdi,rbx
            rdi = rbx;
            // bee4d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bee52 lea rsi,[rip+0x7957b] # 0x1383d4
            rsi = m.env.data_va(314273);
            // bee59 mov rdi,rax
            rdi = rax;
            // bee5c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bee61 mov DWORD PTR [rbp-0x5e8],0x8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa18), (0x8u64) as u32);
            // bee6b lea rsi,[rbp-0x5e8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa18);
            // bee72 mov rdi,rbx
            rdi = rbx;
            // bee75 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bee7a lea rsi,[rip+0x6f6c5] # 0x12e546
            rsi = m.env.data_va(273683);
            // bee81 mov rdi,rax
            rdi = rax;
            // bee84 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bee89 mov DWORD PTR [rbp-0x5ec],0xf1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa14), (0xf1u64) as u32);
            // bee93 lea rsi,[rbp-0x5ec]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa14);
            // bee9a mov rdi,rbx
            rdi = rbx;
            // bee9d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beea2 lea rsi,[rip+0x79532] # 0x1383db
            rsi = m.env.data_va(314280);
            // beea9 mov rdi,rax
            rdi = rax;
            // beeac call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beeb1 mov DWORD PTR [rbp-0x5f0],0x71
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa10), (0x71u64) as u32);
            // beebb lea rsi,[rbp-0x5f0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa10);
            // beec2 mov rdi,rbx
            rdi = rbx;
            // beec5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beeca lea rsi,[rip+0x79514] # 0x1383e5
            rsi = m.env.data_va(314290);
            // beed1 mov rdi,rax
            rdi = rax;
            // beed4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beed9 mov DWORD PTR [rbp-0x5f4],0x8f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa0c), (0x8fu64) as u32);
            // beee3 lea rsi,[rbp-0x5f4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa0c);
            // beeea mov rdi,rbx
            rdi = rbx;
            // beeed call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beef2 lea rsi,[rip+0x794f6] # 0x1383ef
            rsi = m.env.data_va(314300);
            // beef9 mov rdi,rax
            rdi = rax;
            // beefc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bef01 mov DWORD PTR [rbp-0x5f8],0xf
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa08), (0xfu64) as u32);
            // bef0b lea rsi,[rbp-0x5f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa08);
            // bef12 mov rdi,rbx
            rdi = rbx;
            // bef15 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bef1a lea rsi,[rip+0x6f63e] # 0x12e55f
            rsi = m.env.data_va(273708);
            // bef21 mov rdi,rax
            rdi = rax;
            // bef24 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bef29 mov DWORD PTR [rbp-0x5fc],0xf2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa04), (0xf2u64) as u32);
            // bef33 lea rsi,[rbp-0x5fc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa04);
            // bef3a mov rdi,rbx
            rdi = rbx;
            // bef3d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bef42 lea rsi,[rip+0x794ad] # 0x1383f6
            rsi = m.env.data_va(314307);
            // bef49 mov rdi,rax
            rdi = rax;
            // bef4c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bef51 mov DWORD PTR [rbp-0x600],0x72
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa00), (0x72u64) as u32);
            // bef5b lea rsi,[rbp-0x600]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa00);
            // bef62 mov rdi,rbx
            rdi = rbx;
            // bef65 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bef6a lea rsi,[rip+0x7948f] # 0x138400
            rsi = m.env.data_va(314317);
            // bef71 mov rdi,rax
            rdi = rax;
            // bef74 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bef79 mov DWORD PTR [rbp-0x604],0xa4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9fc), (0xa4u64) as u32);
            // bef83 lea rsi,[rbp-0x604]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9fc);
            // bef8a mov rdi,rbx
            rdi = rbx;
            // bef8d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bef92 lea rsi,[rip+0x79471] # 0x13840a
            rsi = m.env.data_va(314327);
            // bef99 mov rdi,rax
            rdi = rax;
            // bef9c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // befa1 mov DWORD PTR [rbp-0x608],0x24
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9f8), (0x24u64) as u32);
            // befab lea rsi,[rbp-0x608]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9f8);
            // befb2 mov rdi,rbx
            rdi = rbx;
            // befb5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // befba lea rsi,[rip+0x6f5b7] # 0x12e578
            rsi = m.env.data_va(273733);
            // befc1 mov rdi,rax
            rdi = rax;
            // befc4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // befc9 mov DWORD PTR [rbp-0x60c],0xf3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9f4), (0xf3u64) as u32);
            // befd3 lea rsi,[rbp-0x60c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9f4);
            // befda mov rdi,rbx
            rdi = rbx;
            // befdd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // befe2 lea rsi,[rip+0x79428] # 0x138411
            rsi = m.env.data_va(314334);
            // befe9 mov rdi,rax
            rdi = rax;
            // befec call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // beff1 mov DWORD PTR [rbp-0x610],0x73
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9f0), (0x73u64) as u32);
            // beffb lea rsi,[rbp-0x610]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9f0);
            // bf002 mov rdi,rbx
            rdi = rbx;
            // bf005 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf00a lea rsi,[rip+0x7940a] # 0x13841b
            rsi = m.env.data_va(314344);
            // bf011 mov rdi,rax
            rdi = rax;
            // bf014 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf019 mov DWORD PTR [rbp-0x614],0xa5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9ec), (0xa5u64) as u32);
            // bf023 lea rsi,[rbp-0x614]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9ec);
            // bf02a mov rdi,rbx
            rdi = rbx;
            // bf02d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf032 lea rsi,[rip+0x793ec] # 0x138425
            rsi = m.env.data_va(314354);
            // bf039 mov rdi,rax
            rdi = rax;
            // bf03c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf041 mov DWORD PTR [rbp-0x618],0x25
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9e8), (0x25u64) as u32);
            // bf04b lea rsi,[rbp-0x618]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9e8);
            // bf052 mov rdi,rbx
            rdi = rbx;
            // bf055 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf05a lea rsi,[rip+0x6f530] # 0x12e591
            rsi = m.env.data_va(273758);
            // bf061 mov rdi,rax
            rdi = rax;
            // bf064 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf069 mov DWORD PTR [rbp-0x61c],0xf4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9e4), (0xf4u64) as u32);
            // bf073 lea rsi,[rbp-0x61c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9e4);
            // bf07a mov rdi,rbx
            rdi = rbx;
            // bf07d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf082 lea rsi,[rip+0x793a3] # 0x13842c
            rsi = m.env.data_va(314361);
            // bf089 mov rdi,rax
            rdi = rax;
            // bf08c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf091 mov DWORD PTR [rbp-0x620],0x74
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9e0), (0x74u64) as u32);
            // bf09b lea rsi,[rbp-0x620]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9e0);
            // bf0a2 mov rdi,rbx
            rdi = rbx;
            // bf0a5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf0aa lea rsi,[rip+0x79385] # 0x138436
            rsi = m.env.data_va(314371);
            // bf0b1 mov rdi,rax
            rdi = rax;
            // bf0b4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf0b9 mov DWORD PTR [rbp-0x624],0xd1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9dc), (0xd1u64) as u32);
            // bf0c3 lea rsi,[rbp-0x624]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9dc);
            // bf0ca mov rdi,rbx
            rdi = rbx;
            // bf0cd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf0d2 lea rsi,[rip+0x79367] # 0x138440
            rsi = m.env.data_va(314381);
            // bf0d9 mov rdi,rax
            rdi = rax;
            // bf0dc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf0e1 mov DWORD PTR [rbp-0x628],0x51
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9d8), (0x51u64) as u32);
            // bf0eb lea rsi,[rbp-0x628]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9d8);
            // bf0f2 mov rdi,rbx
            rdi = rbx;
            // bf0f5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf0fa lea rsi,[rip+0x6fd08] # 0x12ee09
            rsi = m.env.data_va(275926);
            // bf101 mov rdi,rax
            rdi = rax;
            // bf104 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf109 mov DWORD PTR [rbp-0x62c],0xf5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9d4), (0xf5u64) as u32);
            // bf113 lea rsi,[rbp-0x62c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9d4);
            // bf11a mov rdi,rbx
            rdi = rbx;
            // bf11d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf122 lea rsi,[rip+0x7931e] # 0x138447
            rsi = m.env.data_va(314388);
            // bf129 mov rdi,rax
            rdi = rax;
            // bf12c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf131 mov DWORD PTR [rbp-0x630],0x75
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9d0), (0x75u64) as u32);
            // bf13b lea rsi,[rbp-0x630]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9d0);
            // bf142 mov rdi,rbx
            rdi = rbx;
            // bf145 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf14a lea rsi,[rip+0x79300] # 0x138451
            rsi = m.env.data_va(314398);
            // bf151 mov rdi,rax
            rdi = rax;
            // bf154 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf159 mov DWORD PTR [rbp-0x634],0xd2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9cc), (0xd2u64) as u32);
            // bf163 lea rsi,[rbp-0x634]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9cc);
            // bf16a mov rdi,rbx
            rdi = rbx;
            // bf16d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf172 lea rsi,[rip+0x792e2] # 0x13845b
            rsi = m.env.data_va(314408);
            // bf179 mov rdi,rax
            rdi = rax;
            // bf17c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf181 mov DWORD PTR [rbp-0x638],0x52
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9c8), (0x52u64) as u32);
            // bf18b lea rsi,[rbp-0x638]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9c8);
            // bf192 mov rdi,rbx
            rdi = rbx;
            // bf195 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf19a lea rsi,[rip+0x6fc81] # 0x12ee22
            rsi = m.env.data_va(275951);
            // bf1a1 mov rdi,rax
            rdi = rax;
            // bf1a4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf1a9 mov DWORD PTR [rbp-0x63c],0xf6
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9c4), (0xf6u64) as u32);
            // bf1b3 lea rsi,[rbp-0x63c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9c4);
            // bf1ba mov rdi,rbx
            rdi = rbx;
            // bf1bd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf1c2 lea rsi,[rip+0x79299] # 0x138462
            rsi = m.env.data_va(314415);
            // bf1c9 mov rdi,rax
            rdi = rax;
            // bf1cc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf1d1 mov DWORD PTR [rbp-0x640],0x76
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9c0), (0x76u64) as u32);
            // bf1db lea rsi,[rbp-0x640]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9c0);
            // bf1e2 mov rdi,rbx
            rdi = rbx;
            // bf1e5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf1ea lea rsi,[rip+0x7927b] # 0x13846c
            rsi = m.env.data_va(314425);
            // bf1f1 mov rdi,rax
            rdi = rax;
            // bf1f4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf1f9 mov DWORD PTR [rbp-0x644],0xd3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9bc), (0xd3u64) as u32);
            // bf203 lea rsi,[rbp-0x644]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9bc);
            // bf20a mov rdi,rbx
            rdi = rbx;
            // bf20d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf212 lea rsi,[rip+0x7925d] # 0x138476
            rsi = m.env.data_va(314435);
            // bf219 mov rdi,rax
            rdi = rax;
            // bf21c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf221 mov DWORD PTR [rbp-0x648],0x53
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9b8), (0x53u64) as u32);
            // bf22b lea rsi,[rbp-0x648]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9b8);
            // bf232 mov rdi,rbx
            rdi = rbx;
            // bf235 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf23a lea rsi,[rip+0x6fbfa] # 0x12ee3b
            rsi = m.env.data_va(275976);
            // bf241 mov rdi,rax
            rdi = rax;
            // bf244 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf249 mov DWORD PTR [rbp-0x64c],0xf7
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9b4), (0xf7u64) as u32);
            // bf253 lea rsi,[rbp-0x64c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9b4);
            // bf25a mov rdi,rbx
            rdi = rbx;
            // bf25d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf262 lea rsi,[rip+0x79214] # 0x13847d
            rsi = m.env.data_va(314442);
            // bf269 mov rdi,rax
            rdi = rax;
            // bf26c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf271 mov DWORD PTR [rbp-0x650],0x77
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9b0), (0x77u64) as u32);
            // bf27b lea rsi,[rbp-0x650]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9b0);
            // bf282 mov rdi,rbx
            rdi = rbx;
            // bf285 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf28a lea rsi,[rip+0x791f6] # 0x138487
            rsi = m.env.data_va(314452);
            // bf291 mov rdi,rax
            rdi = rax;
            // bf294 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf299 mov DWORD PTR [rbp-0x654],0xd4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9ac), (0xd4u64) as u32);
            // bf2a3 lea rsi,[rbp-0x654]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9ac);
            // bf2aa mov rdi,rbx
            rdi = rbx;
            // bf2ad call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf2b2 lea rsi,[rip+0x791d8] # 0x138491
            rsi = m.env.data_va(314462);
            // bf2b9 mov rdi,rax
            rdi = rax;
            // bf2bc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf2c1 mov DWORD PTR [rbp-0x658],0x54
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9a8), (0x54u64) as u32);
            // bf2cb lea rsi,[rbp-0x658]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9a8);
            // bf2d2 mov rdi,rbx
            rdi = rbx;
            // bf2d5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf2da lea rsi,[rip+0x6fb73] # 0x12ee54
            rsi = m.env.data_va(276001);
            // bf2e1 mov rdi,rax
            rdi = rax;
            // bf2e4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf2e9 mov DWORD PTR [rbp-0x65c],0x84
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9a4), (0x84u64) as u32);
            // bf2f3 lea rsi,[rbp-0x65c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9a4);
            // bf2fa mov rdi,rbx
            rdi = rbx;
            // bf2fd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf302 lea rsi,[rip+0x7918f] # 0x138498
            rsi = m.env.data_va(314469);
            // bf309 mov rdi,rax
            rdi = rax;
            // bf30c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf311 mov DWORD PTR [rbp-0x660],0x4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9a0), (0x4u64) as u32);
            // bf31b lea rsi,[rbp-0x660]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9a0);
            // bf322 mov rdi,rbx
            rdi = rbx;
            // bf325 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf32a lea rsi,[rip+0x7916f] # 0x1384a0
            rsi = m.env.data_va(314477);
            // bf331 mov rdi,rax
            rdi = rax;
            // bf334 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf339 mov DWORD PTR [rbp-0x664],0x8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f99c), (0x8du64) as u32);
            // bf343 lea rsi,[rbp-0x664]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f99c);
            // bf34a mov rdi,rbx
            rdi = rbx;
            // bf34d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf352 lea rsi,[rip+0x7914f] # 0x1384a8
            rsi = m.env.data_va(314485);
            // bf359 mov rdi,rax
            rdi = rax;
            // bf35c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf361 mov DWORD PTR [rbp-0x668],0xa9
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f998), (0xa9u64) as u32);
            // bf36b lea rsi,[rbp-0x668]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f998);
            // bf372 mov rdi,rbx
            rdi = rbx;
            // bf375 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf37a lea rsi,[rip+0x7912f] # 0x1384b0
            rsi = m.env.data_va(314493);
            // bf381 mov rdi,rax
            rdi = rax;
            // bf384 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf389 mov DWORD PTR [rbp-0x66c],0xaa
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f994), (0xaau64) as u32);
            // bf393 lea rsi,[rbp-0x66c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f994);
            // bf39a mov rdi,rbx
            rdi = rbx;
            // bf39d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf3a2 lea rsi,[rip+0x7910f] # 0x1384b8
            rsi = m.env.data_va(314501);
            // bf3a9 mov rdi,rax
            rdi = rax;
            // bf3ac call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf3b1 mov DWORD PTR [rbp-0x670],0xab
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f990), (0xabu64) as u32);
            // bf3bb lea rsi,[rbp-0x670]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f990);
            // bf3c2 mov rdi,rbx
            rdi = rbx;
            // bf3c5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf3ca lea rsi,[rip+0x790ef] # 0x1384c0
            rsi = m.env.data_va(314509);
            // bf3d1 mov rdi,rax
            rdi = rax;
            // bf3d4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf3d9 mov DWORD PTR [rbp-0x674],0xdf
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f98c), (0xdfu64) as u32);
            // bf3e3 lea rsi,[rbp-0x674]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f98c);
            // bf3ea mov rdi,rbx
            rdi = rbx;
            // bf3ed call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf3f2 lea rsi,[rip+0x790cf] # 0x1384c8
            rsi = m.env.data_va(314517);
            // bf3f9 mov rdi,rax
            rdi = rax;
            // bf3fc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf401 mov DWORD PTR [rbp-0x678],0xe0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f988), (0xe0u64) as u32);
            // bf40b lea rsi,[rbp-0x678]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f988);
            // bf412 mov rdi,rbx
            rdi = rbx;
            // bf415 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf41a lea rsi,[rip+0x790af] # 0x1384d0
            rsi = m.env.data_va(314525);
            // bf421 mov rdi,rax
            rdi = rax;
            // bf424 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf429 mov DWORD PTR [rbp-0x67c],0xe1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f984), (0xe1u64) as u32);
            // bf433 lea rsi,[rbp-0x67c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f984);
            // bf43a mov rdi,rbx
            rdi = rbx;
            // bf43d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf442 lea rsi,[rip+0x7908f] # 0x1384d8
            rsi = m.env.data_va(314533);
            // bf449 mov rdi,rax
            rdi = rax;
            // bf44c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf451 mov DWORD PTR [rbp-0x680],0x13
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f980), (0x13u64) as u32);
            // bf45b lea rsi,[rbp-0x680]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f980);
            // bf462 mov rdi,rbx
            rdi = rbx;
            // bf465 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf46a lea rsi,[rip+0x7906f] # 0x1384e0
            rsi = m.env.data_va(314541);
            // bf471 mov rdi,rax
            rdi = rax;
            // bf474 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf479 mov DWORD PTR [rbp-0x684],0x12
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f97c), (0x12u64) as u32);
            // bf483 lea rsi,[rbp-0x684]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f97c);
            // bf48a mov rdi,rbx
            rdi = rbx;
            // bf48d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf492 lea rsi,[rip+0x6f486] # 0x12e91f
            rsi = m.env.data_va(274668);
            // bf499 mov rdi,rax
            rdi = rax;
            // bf49c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf4a1 mov DWORD PTR [rbp-0x688],0x21
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f978), (0x21u64) as u32);
            // bf4ab lea rsi,[rbp-0x688]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f978);
            // bf4b2 mov rdi,rbx
            rdi = rbx;
            // bf4b5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf4ba lea rsi,[rip+0x79029] # 0x1384ea
            rsi = m.env.data_va(314551);
            // bf4c1 mov rdi,rax
            rdi = rax;
            // bf4c4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf4c9 mov DWORD PTR [rbp-0x68c],0x20
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f974), (0x20u64) as u32);
            // bf4d3 lea rsi,[rbp-0x68c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f974);
            // bf4da mov rdi,rbx
            rdi = rbx;
            // bf4dd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf4e2 lea rsi,[rip+0x6f52e] # 0x12ea17
            rsi = m.env.data_va(274916);
            // bf4e9 mov rdi,rax
            rdi = rax;
            // bf4ec call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf4f1 mov DWORD PTR [rbp-0x690],0x56
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f970), (0x56u64) as u32);
            // bf4fb lea rsi,[rbp-0x690]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f970);
            // bf502 mov rdi,rbx
            rdi = rbx;
            // bf505 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf50a lea rsi,[rip+0x78fe3] # 0x1384f4
            rsi = m.env.data_va(314561);
            // bf511 mov rdi,rax
            rdi = rax;
            // bf514 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf519 mov DWORD PTR [rbp-0x694],0x55
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f96c), (0x55u64) as u32);
            // bf523 lea rsi,[rbp-0x694]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f96c);
            // bf52a mov rdi,rbx
            rdi = rbx;
            // bf52d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf532 lea rsi,[rip+0x6f8a4] # 0x12eddd
            rsi = m.env.data_va(275882);
            // bf539 mov rdi,rax
            rdi = rax;
            // bf53c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf541 mov DWORD PTR [rbp-0x698],0x58
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f968), (0x58u64) as u32);
            // bf54b lea rsi,[rbp-0x698]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f968);
            // bf552 mov rdi,rbx
            rdi = rbx;
            // bf555 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf55a lea rsi,[rip+0x78f9d] # 0x1384fe
            rsi = m.env.data_va(314571);
            // bf561 mov rdi,rax
            rdi = rax;
            // bf564 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf569 mov DWORD PTR [rbp-0x69c],0x57
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f964), (0x57u64) as u32);
            // bf573 lea rsi,[rbp-0x69c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f964);
            // bf57a mov rdi,rbx
            rdi = rbx;
            // bf57d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf582 lea rsi,[rip+0x6f86a] # 0x12edf3
            rsi = m.env.data_va(275904);
            // bf589 mov rdi,rax
            rdi = rax;
            // bf58c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf591 mov DWORD PTR [rbp-0x6a0],0x89
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f960), (0x89u64) as u32);
            // bf59b lea rsi,[rbp-0x6a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f960);
            // bf5a2 mov rdi,rbx
            rdi = rbx;
            // bf5a5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf5aa lea rsi,[rip+0x78f57] # 0x138508
            rsi = m.env.data_va(314581);
            // bf5b1 mov rdi,rax
            rdi = rax;
            // bf5b4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf5b9 mov DWORD PTR [rbp-0x6a4],0x9
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f95c), (0x9u64) as u32);
            // bf5c3 lea rsi,[rbp-0x6a4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f95c);
            // bf5ca mov rdi,rbx
            rdi = rbx;
            // bf5cd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf5d2 lea rsi,[rip+0x78f36] # 0x13850f
            rsi = m.env.data_va(314588);
            // bf5d9 mov rdi,rax
            rdi = rax;
            // bf5dc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf5e1 mov DWORD PTR [rbp-0x6a8],0x8a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f958), (0x8au64) as u32);
            // bf5eb lea rsi,[rbp-0x6a8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f958);
            // bf5f2 mov rdi,rbx
            rdi = rbx;
            // bf5f5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf5fa lea rsi,[rip+0x78f15] # 0x138516
            rsi = m.env.data_va(314595);
            // bf601 mov rdi,rax
            rdi = rax;
            // bf604 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf609 mov DWORD PTR [rbp-0x6ac],0xa
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f954), (0xau64) as u32);
            // bf613 lea rsi,[rbp-0x6ac]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f954);
            // bf61a mov rdi,rbx
            rdi = rbx;
            // bf61d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf622 lea rsi,[rip+0x78ef4] # 0x13851d
            rsi = m.env.data_va(314602);
            // bf629 mov rdi,rax
            rdi = rax;
            // bf62c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf631 mov DWORD PTR [rbp-0x6b0],0x7
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f950), (0x7u64) as u32);
            // bf63b lea rsi,[rbp-0x6b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f950);
            // bf642 mov rdi,rbx
            rdi = rbx;
            // bf645 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf64a lea rsi,[rip+0x78a45] # 0x138096
            rsi = m.env.data_va(313443);
            // bf651 mov rdi,rax
            rdi = rax;
            // bf654 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf659 mov DWORD PTR [rbp-0x6b4],0x6
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f94c), (0x6u64) as u32);
            // bf663 lea rsi,[rbp-0x6b4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f94c);
            // bf66a mov rdi,rbx
            rdi = rbx;
            // bf66d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf672 lea rsi,[rip+0x78a22] # 0x13809b
            rsi = m.env.data_va(313448);
            // bf679 mov rdi,rax
            rdi = rax;
            // bf67c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf681 mov DWORD PTR [rbp-0x6b8],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f948), (0x0u64) as u32);
            // bf68b lea rsi,[rbp-0x6b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f948);
            // bf692 mov rdi,rbx
            rdi = rbx;
            // bf695 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf69a lea rsi,[rip+0x6da93] # 0x12d134
            rsi = m.env.data_va(268545);
            // bf6a1 mov rdi,rax
            rdi = rax;
            // bf6a4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf6a9 mov DWORD PTR [rbp-0x6bc],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f944), (0xcu64) as u32);
            // bf6b3 lea rsi,[rbp-0x6bc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f944);
            // bf6ba mov rdi,rbx
            rdi = rbx;
            // bf6bd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf6c2 lea rsi,[rip+0x78e5b] # 0x138524
            rsi = m.env.data_va(314609);
            // bf6c9 mov rdi,rax
            rdi = rax;
            // bf6cc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf6d1 mov DWORD PTR [rbp-0x6c0],0x16
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f940), (0x16u64) as u32);
            // bf6db lea rsi,[rbp-0x6c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f940);
            // bf6e2 mov rdi,rbx
            rdi = rbx;
            // bf6e5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf6ea lea rsi,[rip+0x78e3c] # 0x13852d
            rsi = m.env.data_va(314618);
            // bf6f1 mov rdi,rax
            rdi = rax;
            // bf6f4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf6f9 mov DWORD PTR [rbp-0x6c4],0x1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f93c), (0x1u64) as u32);
            // bf703 lea rsi,[rbp-0x6c4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f93c);
            // bf70a mov rdi,rbx
            rdi = rbx;
            // bf70d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf712 lea rsi,[rip+0x6ef07] # 0x12e620
            rsi = m.env.data_va(273901);
            // bf719 mov rdi,rax
            rdi = rax;
            // bf71c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf721 mov DWORD PTR [rbp-0x6c8],0x1e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f938), (0x1eu64) as u32);
            // bf72b lea rsi,[rbp-0x6c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f938);
            // bf732 mov rdi,rbx
            rdi = rbx;
            // bf735 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf73a lea rsi,[rip+0x78df5] # 0x138536
            rsi = m.env.data_va(314627);
            // bf741 mov rdi,rax
            rdi = rax;
            // bf744 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf749 mov DWORD PTR [rbp-0x6cc],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f934), (0x2u64) as u32);
            // bf753 lea rsi,[rbp-0x6cc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f934);
            // bf75a mov rdi,rbx
            rdi = rbx;
            // bf75d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf762 lea rsi,[rip+0x6eecd] # 0x12e636
            rsi = m.env.data_va(273923);
            // bf769 mov rdi,rax
            rdi = rax;
            // bf76c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf771 mov DWORD PTR [rbp-0x6d0],0x1f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f930), (0x1fu64) as u32);
            // bf77b lea rsi,[rbp-0x6d0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f930);
            // bf782 mov rdi,rbx
            rdi = rbx;
            // bf785 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf78a lea rsi,[rip+0x78daf] # 0x138540
            rsi = m.env.data_va(314637);
            // bf791 mov rdi,rax
            rdi = rax;
            // bf794 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf799 mov DWORD PTR [rbp-0x6d4],0x30
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f92c), (0x30u64) as u32);
            // bf7a3 lea rsi,[rbp-0x6d4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f92c);
            // bf7aa mov rdi,rbx
            rdi = rbx;
            // bf7ad call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf7b2 lea rsi,[rip+0x78d91] # 0x13854a
            rsi = m.env.data_va(314647);
            // bf7b9 mov rdi,rax
            rdi = rax;
            // bf7bc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf7c1 mov DWORD PTR [rbp-0x6d8],0x32
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f928), (0x32u64) as u32);
            // bf7cb lea rsi,[rbp-0x6d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f928);
            // bf7d2 mov rdi,rbx
            rdi = rbx;
            // bf7d5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf7da lea rsi,[rip+0x78d70] # 0x138551
            rsi = m.env.data_va(314654);
            // bf7e1 mov rdi,rax
            rdi = rax;
            // bf7e4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf7e9 mov DWORD PTR [rbp-0x6dc],0x31
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f924), (0x31u64) as u32);
            // bf7f3 lea rsi,[rbp-0x6dc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f924);
            // bf7fa mov rdi,rbx
            rdi = rbx;
            // bf7fd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf802 lea rsi,[rip+0x78d52] # 0x13855b
            rsi = m.env.data_va(314664);
            // bf809 mov rdi,rax
            rdi = rax;
            // bf80c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf811 mov DWORD PTR [rbp-0x6e0],0x33
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f920), (0x33u64) as u32);
            // bf81b lea rsi,[rbp-0x6e0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f920);
            // bf822 mov rdi,rbx
            rdi = rbx;
            // bf825 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf82a lea rsi,[rip+0x78d31] # 0x138562
            rsi = m.env.data_va(314671);
            // bf831 mov rdi,rax
            rdi = rax;
            // bf834 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf839 mov DWORD PTR [rbp-0x6e4],0x45
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f91c), (0x45u64) as u32);
            // bf843 lea rsi,[rbp-0x6e4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f91c);
            // bf84a mov rdi,rbx
            rdi = rbx;
            // bf84d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf852 lea rsi,[rip+0x78d13] # 0x13856c
            rsi = m.env.data_va(314681);
            // bf859 mov rdi,rax
            rdi = rax;
            // bf85c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf861 mov DWORD PTR [rbp-0x6e8],0x47
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f918), (0x47u64) as u32);
            // bf86b lea rsi,[rbp-0x6e8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f918);
            // bf872 mov rdi,rbx
            rdi = rbx;
            // bf875 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf87a lea rsi,[rip+0x78cf2] # 0x138573
            rsi = m.env.data_va(314688);
            // bf881 mov rdi,rax
            rdi = rax;
            // bf884 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf889 mov DWORD PTR [rbp-0x6ec],0x46
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f914), (0x46u64) as u32);
            // bf893 lea rsi,[rbp-0x6ec]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f914);
            // bf89a mov rdi,rbx
            rdi = rbx;
            // bf89d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf8a2 lea rsi,[rip+0x78cd4] # 0x13857d
            rsi = m.env.data_va(314698);
            // bf8a9 mov rdi,rax
            rdi = rax;
            // bf8ac call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf8b1 mov DWORD PTR [rbp-0x6f0],0x48
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f910), (0x48u64) as u32);
            // bf8bb lea rsi,[rbp-0x6f0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f910);
            // bf8c2 mov rdi,rbx
            rdi = rbx;
            // bf8c5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf8ca lea rsi,[rip+0x78cb3] # 0x138584
            rsi = m.env.data_va(314705);
            // bf8d1 mov rdi,rax
            rdi = rax;
            // bf8d4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf8d9 mov DWORD PTR [rbp-0x6f4],0x49
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f90c), (0x49u64) as u32);
            // bf8e3 lea rsi,[rbp-0x6f4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f90c);
            // bf8ea mov rdi,rbx
            rdi = rbx;
            // bf8ed call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf8f2 lea rsi,[rip+0x78c95] # 0x13858e
            rsi = m.env.data_va(314715);
            // bf8f9 mov rdi,rax
            rdi = rax;
            // bf8fc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf901 mov DWORD PTR [rbp-0x6f8],0x4b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f908), (0x4bu64) as u32);
            // bf90b lea rsi,[rbp-0x6f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f908);
            // bf912 mov rdi,rbx
            rdi = rbx;
            // bf915 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf91a lea rsi,[rip+0x78c74] # 0x138595
            rsi = m.env.data_va(314722);
            // bf921 mov rdi,rax
            rdi = rax;
            // bf924 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf929 mov DWORD PTR [rbp-0x6fc],0x4a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f904), (0x4au64) as u32);
            // bf933 lea rsi,[rbp-0x6fc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f904);
            // bf93a mov rdi,rbx
            rdi = rbx;
            // bf93d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf942 lea rsi,[rip+0x78c56] # 0x13859f
            rsi = m.env.data_va(314732);
            // bf949 mov rdi,rax
            rdi = rax;
            // bf94c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf951 mov DWORD PTR [rbp-0x700],0x4c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f900), (0x4cu64) as u32);
            // bf95b lea rsi,[rbp-0x700]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f900);
            // bf962 mov rdi,rbx
            rdi = rbx;
            // bf965 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf96a lea rsi,[rip+0x78c35] # 0x1385a6
            rsi = m.env.data_va(314739);
            // bf971 mov rdi,rax
            rdi = rax;
            // bf974 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf979 mov DWORD PTR [rbp-0x704],0x17
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f8fc), (0x17u64) as u32);
            // bf983 lea rsi,[rbp-0x704]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f8fc);
            // bf98a mov rdi,rbx
            rdi = rbx;
            // bf98d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf992 lea rsi,[rip+0x78c17] # 0x1385b0
            rsi = m.env.data_va(314749);
            // bf999 mov rdi,rax
            rdi = rax;
            // bf99c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf9a1 mov DWORD PTR [rbp-0x708],0x41
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f8f8), (0x41u64) as u32);
            // bf9ab lea rsi,[rbp-0x708]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f8f8);
            // bf9b2 mov rdi,rbx
            rdi = rbx;
            // bf9b5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf9ba lea rsi,[rip+0x78bf6] # 0x1385b7
            rsi = m.env.data_va(314756);
            // bf9c1 mov rdi,rax
            rdi = rax;
            // bf9c4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf9c9 mov DWORD PTR [rbp-0x70c],0xc1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f8f4), (0xc1u64) as u32);
            // bf9d3 lea rsi,[rbp-0x70c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f8f4);
            // bf9da mov rdi,rbx
            rdi = rbx;
            // bf9dd call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf9e2 lea rsi,[rip+0x78bd7] # 0x1385c0
            rsi = m.env.data_va(314765);
            // bf9e9 mov rdi,rax
            rdi = rax;
            // bf9ec call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bf9f1 mov DWORD PTR [rbp-0x710],0x42
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f8f0), (0x42u64) as u32);
            // bf9fb lea rsi,[rbp-0x710]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f8f0);
            // bfa02 mov rdi,rbx
            rdi = rbx;
            // bfa05 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfa0a lea rsi,[rip+0x78bbb] # 0x1385cc
            rsi = m.env.data_va(314777);
            // bfa11 mov rdi,rax
            rdi = rax;
            // bfa14 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfa19 mov DWORD PTR [rbp-0x714],0xc2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f8ec), (0xc2u64) as u32);
            // bfa23 lea rsi,[rbp-0x714]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f8ec);
            // bfa2a mov rdi,rbx
            rdi = rbx;
            // bfa2d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfa32 lea rsi,[rip+0x78b9c] # 0x1385d5
            rsi = m.env.data_va(314786);
            // bfa39 mov rdi,rax
            rdi = rax;
            // bfa3c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfa41 mov DWORD PTR [rbp-0x718],0x43
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f8e8), (0x43u64) as u32);
            // bfa4b lea rsi,[rbp-0x718]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f8e8);
            // bfa52 mov rdi,rbx
            rdi = rbx;
            // bfa55 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfa5a lea rsi,[rip+0x78b80] # 0x1385e1
            rsi = m.env.data_va(314798);
            // bfa61 mov rdi,rax
            rdi = rax;
            // bfa64 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfa69 mov DWORD PTR [rbp-0x71c],0xc3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f8e4), (0xc3u64) as u32);
            // bfa73 lea rsi,[rbp-0x71c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f8e4);
            // bfa7a mov rdi,rbx
            rdi = rbx;
            // bfa7d call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfa82 lea rsi,[rip+0x78b61] # 0x1385ea
            rsi = m.env.data_va(314807);
            // bfa89 mov rdi,rax
            rdi = rax;
            // bfa8c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfa91 mov DWORD PTR [rbp-0x720],0x97
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f8e0), (0x97u64) as u32);
            // bfa9b lea rsi,[rbp-0x720]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f8e0);
            // bfaa2 mov rdi,rbx
            rdi = rbx;
            // bfaa5 call 0xebd38
            { let r = { let t = m.env.stub_va(0x505); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfaaa lea rsi,[rip+0x78b45] # 0x1385f6
            rsi = m.env.data_va(314819);
            // bfab1 mov rdi,rax
            rdi = rax;
            // bfab4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfab9 mov rbx,QWORD PTR [rip+0x108b98] # 0x1c8658
            rbx = m.r64(m.env.data_va(904741)) as u64;
            // bfac0 lea r15,[rip+0x108b99] # 0x1c8660
            r15 = m.env.data_va(904749);
            // bfac7 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // bfaca je 0xbfb2a
            if crate::cov::brk(0xbfaca, fl.e()) { pc = 0xbfb2a; continue; }
            pc = 0xbfacc;
        }
        0xbfacc => {
            // block bfacc
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bfacc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bfacc lea r12,[rbp-0x738]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_f8c8);
            // bfad3 lea r14,[rip+0x108b66] # 0x1c8640
            r14 = m.env.data_va(904717);
            pc = 0xbfada;
        }
        0xbfada => {
            // block bfada
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bfada rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bfada lea rsi,[rbx+0x28]
            rsi = rbx.wrapping_add(0x28);
            // bfade mov rdi,r12
            rdi = r12;
            // bfae1 call 0xbbdbe
            { let r = { let t = m.env.code_va(0xbbdbe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfae6 mov r13d,DWORD PTR [rbx+0x20]
            r13 = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // bfaea mov rdi,r14
            rdi = r14;
            // bfaed mov rsi,r12
            rsi = r12;
            // bfaf0 call 0xebd68
            { let r = { let t = m.env.stub_va(0x535); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfaf5 mov DWORD PTR [rax],r13d
            m.w32(rax, ((r13 as u32 as u64)) as u32);
            // bfaf8 mov rdi,r12
            rdi = r12;
            // bfafb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfb00 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bfb04 test rax,rax
            fl = logic::<64>(rax & rax);
            // bfb07 je 0xbfb16
            if crate::cov::brk(0xbfb07, fl.e()) { pc = 0xbfb16; continue; }
            pc = 0xbfb09;
        }
        0xbfb09 => {
            // block bfb09
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bfb09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bfb09 mov rbx,rax
            rbx = rax;
            // bfb0c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bfb0f test rax,rax
            fl = logic::<64>(rax & rax);
            // bfb12 jne 0xbfb09
            if crate::cov::brk(0xbfb12, fl.ne()) { pc = 0xbfb09; continue; }
            pc = 0xbfb14;
        }
        0xbfb14 => {
            // block bfb14
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bfb14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bfb14 jmp 0xbfb25
            pc = 0xbfb25; continue;
        }
        0xbfb16 => {
            // block bfb16
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bfb16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bfb16 mov rax,rbx
            rax = rbx;
            pc = 0xbfb19;
        }
        0xbfb19 => {
            // block bfb19
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bfb19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bfb19 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // bfb1d cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // bfb20 mov rax,rbx
            rax = rbx;
            // bfb23 jne 0xbfb19
            if crate::cov::brk(0xbfb23, fl.ne()) { pc = 0xbfb19; continue; }
            pc = 0xbfb25;
        }
        0xbfb25 => {
            // block bfb25
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bfb25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bfb25 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // bfb28 jne 0xbfada
            if crate::cov::brk(0xbfb28, fl.ne()) { pc = 0xbfada; continue; }
            pc = 0xbfb2a;
        }
        0xbfb2a => {
            // block bfb2a
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc bfb2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bfb2a lea rsi,[rip+0x78acf] # 0x138600
            rsi = m.env.data_va(314829);
            // bfb31 lea rbx,[rbp-0x750]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f8b0);
            // bfb38 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bfb3d mov rdi,rbx
            rdi = rbx;
            // bfb40 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfb45 lea rdi,[rip+0x108af4] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfb4c mov rsi,rbx
            rsi = rbx;
            // bfb4f call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfb54 mov DWORD PTR [rax],0x44
            m.w32(rax, (0x44u64) as u32);
            // bfb5a lea rdi,[rbp-0x750]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8b0);
            // bfb61 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfb66 lea rsi,[rip+0x78a9b] # 0x138608
            rsi = m.env.data_va(314837);
            // bfb6d lea rbx,[rbp-0x768]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f898);
            // bfb74 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bfb79 mov rdi,rbx
            rdi = rbx;
            // bfb7c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfb81 lea rdi,[rip+0x108ab8] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfb88 mov rsi,rbx
            rsi = rbx;
            // bfb8b call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfb90 mov DWORD PTR [rax],0xe
            m.w32(rax, (0xeu64) as u32);
            // bfb96 lea rdi,[rbp-0x768]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f898);
            // bfb9d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfba2 lea rsi,[rip+0x78a6a] # 0x138613
            rsi = m.env.data_va(314848);
            // bfba9 lea rbx,[rbp-0x780]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f880);
            // bfbb0 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bfbb5 mov rdi,rbx
            rdi = rbx;
            // bfbb8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfbbd lea rdi,[rip+0x108a7c] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfbc4 mov rsi,rbx
            rsi = rbx;
            // bfbc7 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfbcc mov DWORD PTR [rax],0x85
            m.w32(rax, (0x85u64) as u32);
            // bfbd2 lea rdi,[rbp-0x780]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f880);
            // bfbd9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfbde lea rsi,[rip+0x78a39] # 0x13861e
            rsi = m.env.data_va(314859);
            // bfbe5 lea rbx,[rbp-0x798]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f868);
            // bfbec mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bfbf1 mov rdi,rbx
            rdi = rbx;
            // bfbf4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfbf9 lea rdi,[rip+0x108a40] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfc00 mov rsi,rbx
            rsi = rbx;
            // bfc03 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfc08 mov DWORD PTR [rax],0x5
            m.w32(rax, (0x5u64) as u32);
            // bfc0e lea rdi,[rbp-0x798]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f868);
            // bfc15 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfc1a lea rsi,[rip+0x78a05] # 0x138626
            rsi = m.env.data_va(314867);
            // bfc21 lea rbx,[rbp-0x7b0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f850);
            // bfc28 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bfc2d mov rdi,rbx
            rdi = rbx;
            // bfc30 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfc35 lea rdi,[rip+0x108a04] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfc3c mov rsi,rbx
            rsi = rbx;
            // bfc3f call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfc44 mov DWORD PTR [rax],0x5
            m.w32(rax, (0x5u64) as u32);
            // bfc4a lea rdi,[rbp-0x7b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f850);
            // bfc51 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfc56 lea rsi,[rip+0x789d4] # 0x138631
            rsi = m.env.data_va(314878);
            // bfc5d lea rbx,[rbp-0x7c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f838);
            // bfc64 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bfc69 mov rdi,rbx
            rdi = rbx;
            // bfc6c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfc71 lea rdi,[rip+0x1089c8] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfc78 mov rsi,rbx
            rsi = rbx;
            // bfc7b call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfc80 mov DWORD PTR [rax],0x11
            m.w32(rax, (0x11u64) as u32);
            // bfc86 lea rdi,[rbp-0x7c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f838);
            // bfc8d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfc92 lea rsi,[rip+0x789a3] # 0x13863c
            rsi = m.env.data_va(314889);
            // bfc99 lea rbx,[rbp-0x7e0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f820);
            // bfca0 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bfca5 mov rdi,rbx
            rdi = rbx;
            // bfca8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfcad lea rdi,[rip+0x10898c] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfcb4 mov rsi,rbx
            rsi = rbx;
            // bfcb7 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfcbc mov DWORD PTR [rax],0x90
            m.w32(rax, (0x90u64) as u32);
            // bfcc2 lea rdi,[rbp-0x7e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f820);
            // bfcc9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfcce lea rsi,[rip+0x78972] # 0x138647
            rsi = m.env.data_va(314900);
            // bfcd5 lea rbx,[rbp-0x7f8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f808);
            // bfcdc mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bfce1 mov rdi,rbx
            rdi = rbx;
            // bfce4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfce9 lea rdi,[rip+0x108950] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfcf0 mov rsi,rbx
            rsi = rbx;
            // bfcf3 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfcf8 mov DWORD PTR [rax],0x10
            m.w32(rax, (0x10u64) as u32);
            // bfcfe lea rdi,[rbp-0x7f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f808);
            // bfd05 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfd0a lea rsi,[rip+0x7893e] # 0x13864f
            rsi = m.env.data_va(314908);
            // bfd11 lea rbx,[rbp-0x810]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f7f0);
            // bfd18 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bfd1d mov rdi,rbx
            rdi = rbx;
            // bfd20 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfd25 lea rdi,[rip+0x108914] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfd2c mov rsi,rbx
            rsi = rbx;
            // bfd2f call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfd34 mov DWORD PTR [rax],0x10
            m.w32(rax, (0x10u64) as u32);
            // bfd3a lea rdi,[rbp-0x810]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7f0);
            // bfd41 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfd46 lea rsi,[rip+0x7890d] # 0x13865a
            rsi = m.env.data_va(314919);
            // bfd4d lea rbx,[rbp-0x828]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f7d8);
            // bfd54 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bfd59 mov rdi,rbx
            rdi = rbx;
            // bfd5c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfd61 lea rdi,[rip+0x1088d8] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfd68 mov rsi,rbx
            rsi = rbx;
            // bfd6b call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfd70 mov DWORD PTR [rax],0x3b
            m.w32(rax, (0x3bu64) as u32);
            // bfd76 lea rdi,[rbp-0x828]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7d8);
            // bfd7d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfd82 lea rsi,[rip+0x788dc] # 0x138665
            rsi = m.env.data_va(314930);
            // bfd89 lea rbx,[rbp-0x840]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f7c0);
            // bfd90 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bfd95 mov rdi,rbx
            rdi = rbx;
            // bfd98 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfd9d lea rdi,[rip+0x10889c] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfda4 mov rsi,rbx
            rsi = rbx;
            // bfda7 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfdac mov DWORD PTR [rax],0xba
            m.w32(rax, (0xbau64) as u32);
            // bfdb2 lea rdi,[rbp-0x840]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7c0);
            // bfdb9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfdbe lea rsi,[rip+0x788ab] # 0x138670
            rsi = m.env.data_va(314941);
            // bfdc5 lea rbx,[rbp-0x858]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f7a8);
            // bfdcc mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bfdd1 mov rdi,rbx
            rdi = rbx;
            // bfdd4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfdd9 lea rdi,[rip+0x108860] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfde0 mov rsi,rbx
            rsi = rbx;
            // bfde3 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfde8 mov DWORD PTR [rax],0x3a
            m.w32(rax, (0x3au64) as u32);
            // bfdee lea rdi,[rbp-0x858]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7a8);
            // bfdf5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfdfa lea rsi,[rip+0x78877] # 0x138678
            rsi = m.env.data_va(314949);
            // bfe01 lea rbx,[rbp-0x870]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f790);
            // bfe08 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bfe0d mov rdi,rbx
            rdi = rbx;
            // bfe10 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfe15 lea rdi,[rip+0x108824] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfe1c mov rsi,rbx
            rsi = rbx;
            // bfe1f call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfe24 mov DWORD PTR [rax],0x3a
            m.w32(rax, (0x3au64) as u32);
            // bfe2a lea rdi,[rbp-0x870]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f790);
            // bfe31 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfe36 lea rsi,[rip+0x78846] # 0x138683
            rsi = m.env.data_va(314960);
            // bfe3d lea rbx,[rbp-0x888]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f778);
            // bfe44 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bfe49 mov rdi,rbx
            rdi = rbx;
            // bfe4c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfe51 lea rdi,[rip+0x1087e8] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfe58 mov rsi,rbx
            rsi = rbx;
            // bfe5b call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfe60 mov DWORD PTR [rax],0x3d
            m.w32(rax, (0x3du64) as u32);
            // bfe66 lea rdi,[rbp-0x888]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f778);
            // bfe6d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfe72 lea rsi,[rip+0x78815] # 0x13868e
            rsi = m.env.data_va(314971);
            // bfe79 lea rbx,[rbp-0x8a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f760);
            // bfe80 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bfe85 mov rdi,rbx
            rdi = rbx;
            // bfe88 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfe8d lea rdi,[rip+0x1087ac] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfe94 mov rsi,rbx
            rsi = rbx;
            // bfe97 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfe9c mov DWORD PTR [rax],0xbc
            m.w32(rax, (0xbcu64) as u32);
            // bfea2 lea rdi,[rbp-0x8a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f760);
            // bfea9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfeae lea rsi,[rip+0x787e4] # 0x138699
            rsi = m.env.data_va(314982);
            // bfeb5 lea rbx,[rbp-0x8b8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f748);
            // bfebc mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bfec1 mov rdi,rbx
            rdi = rbx;
            // bfec4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfec9 lea rdi,[rip+0x108770] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfed0 mov rsi,rbx
            rsi = rbx;
            // bfed3 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfed8 mov DWORD PTR [rax],0x3c
            m.w32(rax, (0x3cu64) as u32);
            // bfede lea rdi,[rbp-0x8b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f748);
            // bfee5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfeea lea rsi,[rip+0x787b0] # 0x1386a1
            rsi = m.env.data_va(314990);
            // bfef1 lea rbx,[rbp-0x8d0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f730);
            // bfef8 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bfefd mov rdi,rbx
            rdi = rbx;
            // bff00 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bff05 lea rdi,[rip+0x108734] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bff0c mov rsi,rbx
            rsi = rbx;
            // bff0f call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bff14 mov DWORD PTR [rax],0x3c
            m.w32(rax, (0x3cu64) as u32);
            // bff1a lea rdi,[rbp-0x8d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f730);
            // bff21 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bff26 lea rsi,[rip+0x7877f] # 0x1386ac
            rsi = m.env.data_va(315001);
            // bff2d lea rbx,[rbp-0x8e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f718);
            // bff34 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bff39 mov rdi,rbx
            rdi = rbx;
            // bff3c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bff41 lea rdi,[rip+0x1086f8] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bff48 mov rsi,rbx
            rsi = rbx;
            // bff4b call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bff50 mov DWORD PTR [rax],0x2d
            m.w32(rax, (0x2du64) as u32);
            // bff56 lea rdi,[rbp-0x8e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f718);
            // bff5d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bff62 lea rsi,[rip+0x7874e] # 0x1386b7
            rsi = m.env.data_va(315012);
            // bff69 lea rbx,[rbp-0x900]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f700);
            // bff70 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bff75 mov rdi,rbx
            rdi = rbx;
            // bff78 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bff7d lea rdi,[rip+0x1086bc] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bff84 mov rsi,rbx
            rsi = rbx;
            // bff87 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bff8c mov DWORD PTR [rax],0xac
            m.w32(rax, (0xacu64) as u32);
            // bff92 lea rdi,[rbp-0x900]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f700);
            // bff99 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bff9e lea rsi,[rip+0x7871d] # 0x1386c2
            rsi = m.env.data_va(315023);
            // bffa5 lea rbx,[rbp-0x918]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f6e8);
            // bffac mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // bffb1 mov rdi,rbx
            rdi = rbx;
            // bffb4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bffb9 lea rdi,[rip+0x108680] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bffc0 mov rsi,rbx
            rsi = rbx;
            // bffc3 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bffc8 mov DWORD PTR [rax],0x2c
            m.w32(rax, (0x2cu64) as u32);
            // bffce lea rdi,[rbp-0x918]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6e8);
            // bffd5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bffda lea rsi,[rip+0x786e9] # 0x1386ca
            rsi = m.env.data_va(315031);
            // bffe1 lea rbx,[rbp-0x930]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f6d0);
            // bffe8 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // bffed mov rdi,rbx
            rdi = rbx;
            // bfff0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bfff5 lea rdi,[rip+0x108644] # 0x1c8640
            rdi = m.env.data_va(904717);
            // bfffc mov rsi,rbx
            rsi = rbx;
            // bffff call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0004 mov DWORD PTR [rax],0x2c
            m.w32(rax, (0x2cu64) as u32);
            // c000a lea rdi,[rbp-0x930]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6d0);
            // c0011 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0016 lea rsi,[rip+0x786b8] # 0x1386d5
            rsi = m.env.data_va(315042);
            // c001d lea rbx,[rbp-0x948]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f6b8);
            // c0024 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // c0029 mov rdi,rbx
            rdi = rbx;
            // c002c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0031 lea rdi,[rip+0x108608] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0038 mov rsi,rbx
            rsi = rbx;
            // c003b call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0040 mov DWORD PTR [rax],0x5a
            m.w32(rax, (0x5au64) as u32);
            // c0046 lea rdi,[rbp-0x948]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6b8);
            // c004d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0052 lea rsi,[rip+0x78687] # 0x1386e0
            rsi = m.env.data_va(315053);
            // c0059 lea rbx,[rbp-0x960]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f6a0);
            // c0060 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // c0065 mov rdi,rbx
            rdi = rbx;
            // c0068 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c006d lea rdi,[rip+0x1085cc] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0074 mov rsi,rbx
            rsi = rbx;
            // c0077 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c007c mov DWORD PTR [rax],0xd9
            m.w32(rax, (0xd9u64) as u32);
            // c0082 lea rdi,[rbp-0x960]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6a0);
            // c0089 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c008e lea rsi,[rip+0x78656] # 0x1386eb
            rsi = m.env.data_va(315064);
            // c0095 lea rbx,[rbp-0x978]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f688);
            // c009c mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // c00a1 mov rdi,rbx
            rdi = rbx;
            // c00a4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c00a9 lea rdi,[rip+0x108590] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c00b0 mov rsi,rbx
            rsi = rbx;
            // c00b3 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c00b8 mov DWORD PTR [rax],0x59
            m.w32(rax, (0x59u64) as u32);
            // c00be lea rdi,[rbp-0x978]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f688);
            // c00c5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c00ca lea rsi,[rip+0x78622] # 0x1386f3
            rsi = m.env.data_va(315072);
            // c00d1 lea rbx,[rbp-0x990]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f670);
            // c00d8 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // c00dd mov rdi,rbx
            rdi = rbx;
            // c00e0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c00e5 lea rdi,[rip+0x108554] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c00ec mov rsi,rbx
            rsi = rbx;
            // c00ef call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c00f4 mov DWORD PTR [rax],0x59
            m.w32(rax, (0x59u64) as u32);
            // c00fa lea rdi,[rbp-0x990]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f670);
            // c0101 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0106 lea rsi,[rip+0x785f1] # 0x1386fe
            rsi = m.env.data_va(315083);
            // c010d lea rbx,[rbp-0x9a8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f658);
            // c0114 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // c0119 mov rdi,rbx
            rdi = rbx;
            // c011c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0121 lea rdi,[rip+0x108518] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0128 mov rsi,rbx
            rsi = rbx;
            // c012b call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0130 mov DWORD PTR [rax],0x5c
            m.w32(rax, (0x5cu64) as u32);
            // c0136 lea rdi,[rbp-0x9a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f658);
            // c013d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0142 lea rsi,[rip+0x785c0] # 0x138709
            rsi = m.env.data_va(315094);
            // c0149 lea rbx,[rbp-0x9c0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f640);
            // c0150 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // c0155 mov rdi,rbx
            rdi = rbx;
            // c0158 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c015d lea rdi,[rip+0x1084dc] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0164 mov rsi,rbx
            rsi = rbx;
            // c0167 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c016c mov DWORD PTR [rax],0xdb
            m.w32(rax, (0xdbu64) as u32);
            // c0172 lea rdi,[rbp-0x9c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f640);
            // c0179 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c017e lea rsi,[rip+0x7858f] # 0x138714
            rsi = m.env.data_va(315105);
            // c0185 lea rbx,[rbp-0x9d8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f628);
            // c018c mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // c0191 mov rdi,rbx
            rdi = rbx;
            // c0194 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0199 lea rdi,[rip+0x1084a0] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c01a0 mov rsi,rbx
            rsi = rbx;
            // c01a3 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c01a8 mov DWORD PTR [rax],0x5b
            m.w32(rax, (0x5bu64) as u32);
            // c01ae lea rdi,[rbp-0x9d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f628);
            // c01b5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c01ba lea rsi,[rip+0x7855b] # 0x13871c
            rsi = m.env.data_va(315113);
            // c01c1 lea rbx,[rbp-0x9f0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f610);
            // c01c8 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // c01cd mov rdi,rbx
            rdi = rbx;
            // c01d0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c01d5 lea rdi,[rip+0x108464] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c01dc mov rsi,rbx
            rsi = rbx;
            // c01df call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c01e4 mov DWORD PTR [rax],0x5b
            m.w32(rax, (0x5bu64) as u32);
            // c01ea lea rdi,[rbp-0x9f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f610);
            // c01f1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c01f6 lea rsi,[rip+0x7852a] # 0x138727
            rsi = m.env.data_va(315124);
            // c01fd lea rbx,[rbp-0xa08]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f5f8);
            // c0204 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // c0209 mov rdi,rbx
            rdi = rbx;
            // c020c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0211 lea rdi,[rip+0x108428] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0218 mov rsi,rbx
            rsi = rbx;
            // c021b call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0220 mov DWORD PTR [rax],0x5e
            m.w32(rax, (0x5eu64) as u32);
            // c0226 lea rdi,[rbp-0xa08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5f8);
            // c022d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0232 lea rsi,[rip+0x784f9] # 0x138732
            rsi = m.env.data_va(315135);
            // c0239 lea rbx,[rbp-0xa20]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f5e0);
            // c0240 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // c0245 mov rdi,rbx
            rdi = rbx;
            // c0248 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c024d lea rdi,[rip+0x1083ec] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0254 mov rsi,rbx
            rsi = rbx;
            // c0257 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c025c mov DWORD PTR [rax],0xdd
            m.w32(rax, (0xddu64) as u32);
            // c0262 lea rdi,[rbp-0xa20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5e0);
            // c0269 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c026e lea rsi,[rip+0x784c8] # 0x13873d
            rsi = m.env.data_va(315146);
            // c0275 lea rbx,[rbp-0xa38]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f5c8);
            // c027c mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // c0281 mov rdi,rbx
            rdi = rbx;
            // c0284 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0289 lea rdi,[rip+0x1083b0] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0290 mov rsi,rbx
            rsi = rbx;
            // c0293 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0298 mov DWORD PTR [rax],0x5d
            m.w32(rax, (0x5du64) as u32);
            // c029e lea rdi,[rbp-0xa38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5c8);
            // c02a5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c02aa lea rsi,[rip+0x78494] # 0x138745
            rsi = m.env.data_va(315154);
            // c02b1 lea rbx,[rbp-0xa50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f5b0);
            // c02b8 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // c02bd mov rdi,rbx
            rdi = rbx;
            // c02c0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c02c5 lea rdi,[rip+0x108374] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c02cc mov rsi,rbx
            rsi = rbx;
            // c02cf call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c02d4 mov DWORD PTR [rax],0x5d
            m.w32(rax, (0x5du64) as u32);
            // c02da lea rdi,[rbp-0xa50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5b0);
            // c02e1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c02e6 lea rsi,[rip+0x78463] # 0x138750
            rsi = m.env.data_va(315165);
            // c02ed lea rbx,[rbp-0xa68]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f598);
            // c02f4 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c02f9 mov rdi,rbx
            rdi = rbx;
            // c02fc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0301 lea rdi,[rip+0x108338] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0308 mov rsi,rbx
            rsi = rbx;
            // c030b call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0310 mov DWORD PTR [rax],0x26
            m.w32(rax, (0x26u64) as u32);
            // c0316 lea rdi,[rbp-0xa68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f598);
            // c031d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0322 lea rsi,[rip+0x78431] # 0x13875a
            rsi = m.env.data_va(315175);
            // c0329 lea rbx,[rbp-0xa80]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f580);
            // c0330 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0335 mov rdi,rbx
            rdi = rbx;
            // c0338 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c033d lea rdi,[rip+0x1082fc] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0344 mov rsi,rbx
            rsi = rbx;
            // c0347 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c034c mov DWORD PTR [rax],0x27
            m.w32(rax, (0x27u64) as u32);
            // c0352 lea rdi,[rbp-0xa80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f580);
            // c0359 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c035e lea rsi,[rip+0x783ff] # 0x138764
            rsi = m.env.data_va(315185);
            // c0365 lea rbx,[rbp-0xa98]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f568);
            // c036c mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0371 mov rdi,rbx
            rdi = rbx;
            // c0374 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0379 lea rdi,[rip+0x1082c0] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0380 mov rsi,rbx
            rsi = rbx;
            // c0383 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0388 mov DWORD PTR [rax],0x37
            m.w32(rax, (0x37u64) as u32);
            // c038e lea rdi,[rbp-0xa98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f568);
            // c0395 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c039a lea rsi,[rip+0x783cd] # 0x13876e
            rsi = m.env.data_va(315195);
            // c03a1 lea rbx,[rbp-0xab0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f550);
            // c03a8 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c03ad mov rdi,rbx
            rdi = rbx;
            // c03b0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c03b5 lea rdi,[rip+0x108284] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c03bc mov rsi,rbx
            rsi = rbx;
            // c03bf call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c03c4 mov DWORD PTR [rax],0x39
            m.w32(rax, (0x39u64) as u32);
            // c03ca lea rdi,[rbp-0xab0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f550);
            // c03d1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c03d6 lea rsi,[rip+0x7839b] # 0x138778
            rsi = m.env.data_va(315205);
            // c03dd lea rbx,[rbp-0xac8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f538);
            // c03e4 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c03e9 mov rdi,rbx
            rdi = rbx;
            // c03ec call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c03f1 lea rdi,[rip+0x108248] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c03f8 mov rsi,rbx
            rsi = rbx;
            // c03fb call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0400 mov DWORD PTR [rax],0x3f
            m.w32(rax, (0x3fu64) as u32);
            // c0406 lea rdi,[rbp-0xac8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f538);
            // c040d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0412 lea rsi,[rip+0x78369] # 0x138782
            rsi = m.env.data_va(315215);
            // c0419 lea rbx,[rbp-0xae0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f520);
            // c0420 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0425 mov rdi,rbx
            rdi = rbx;
            // c0428 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c042d lea rdi,[rip+0x10820c] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0434 mov rsi,rbx
            rsi = rbx;
            // c0437 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c043c mov DWORD PTR [rax],0x63
            m.w32(rax, (0x63u64) as u32);
            // c0442 lea rdi,[rbp-0xae0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f520);
            // c0449 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c044e lea rsi,[rip+0x78337] # 0x13878c
            rsi = m.env.data_va(315225);
            // c0455 lea rbx,[rbp-0xaf8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f508);
            // c045c mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0461 mov rdi,rbx
            rdi = rbx;
            // c0464 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0469 lea rdi,[rip+0x1081d0] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0470 mov rsi,rbx
            rsi = rbx;
            // c0473 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0478 mov DWORD PTR [rax],0x65
            m.w32(rax, (0x65u64) as u32);
            // c047e lea rdi,[rbp-0xaf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f508);
            // c0485 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c048a lea rsi,[rip+0x78305] # 0x138796
            rsi = m.env.data_va(315235);
            // c0491 lea rbx,[rbp-0xb10]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f4f0);
            // c0498 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c049d mov rdi,rbx
            rdi = rbx;
            // c04a0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c04a5 lea rdi,[rip+0x108194] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c04ac mov rsi,rbx
            rsi = rbx;
            // c04af call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c04b4 mov DWORD PTR [rax],0x67
            m.w32(rax, (0x67u64) as u32);
            // c04ba lea rdi,[rbp-0xb10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4f0);
            // c04c1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c04c6 lea rsi,[rip+0x782d3] # 0x1387a0
            rsi = m.env.data_va(315245);
            // c04cd lea rbx,[rbp-0xb28]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f4d8);
            // c04d4 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // c04d9 mov rdi,rbx
            rdi = rbx;
            // c04dc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c04e1 lea rdi,[rip+0x108158] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c04e8 mov rsi,rbx
            rsi = rbx;
            // c04eb call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c04f0 mov DWORD PTR [rax],0x8
            m.w32(rax, (0x8u64) as u32);
            // c04f6 lea rdi,[rbp-0xb28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4d8);
            // c04fd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0502 lea rsi,[rip+0x7829e] # 0x1387a7
            rsi = m.env.data_va(315252);
            // c0509 lea rbx,[rbp-0xb40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f4c0);
            // c0510 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // c0515 mov rdi,rbx
            rdi = rbx;
            // c0518 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c051d lea rdi,[rip+0x10811c] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0524 mov rsi,rbx
            rsi = rbx;
            // c0527 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c052c mov DWORD PTR [rax],0xf
            m.w32(rax, (0xfu64) as u32);
            // c0532 lea rdi,[rbp-0xb40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4c0);
            // c0539 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c053e lea rsi,[rip+0x78269] # 0x1387ae
            rsi = m.env.data_va(315259);
            // c0545 lea rbx,[rbp-0xb58]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f4a8);
            // c054c mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // c0551 mov rdi,rbx
            rdi = rbx;
            // c0554 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0559 lea rdi,[rip+0x1080e0] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0560 mov rsi,rbx
            rsi = rbx;
            // c0563 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0568 mov DWORD PTR [rax],0x24
            m.w32(rax, (0x24u64) as u32);
            // c056e lea rdi,[rbp-0xb58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4a8);
            // c0575 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c057a lea rsi,[rip+0x78234] # 0x1387b5
            rsi = m.env.data_va(315266);
            // c0581 lea rbx,[rbp-0xb70]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f490);
            // c0588 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // c058d mov rdi,rbx
            rdi = rbx;
            // c0590 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0595 lea rdi,[rip+0x1080a4] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c059c mov rsi,rbx
            rsi = rbx;
            // c059f call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c05a4 mov DWORD PTR [rax],0x25
            m.w32(rax, (0x25u64) as u32);
            // c05aa lea rdi,[rbp-0xb70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f490);
            // c05b1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c05b6 lea rsi,[rip+0x781ff] # 0x1387bc
            rsi = m.env.data_va(315273);
            // c05bd lea rbx,[rbp-0xb88]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f478);
            // c05c4 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // c05c9 mov rdi,rbx
            rdi = rbx;
            // c05cc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c05d1 lea rdi,[rip+0x108068] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c05d8 mov rsi,rbx
            rsi = rbx;
            // c05db call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c05e0 mov DWORD PTR [rax],0x51
            m.w32(rax, (0x51u64) as u32);
            // c05e6 lea rdi,[rbp-0xb88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f478);
            // c05ed call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c05f2 lea rsi,[rip+0x781ca] # 0x1387c3
            rsi = m.env.data_va(315280);
            // c05f9 lea rbx,[rbp-0xba0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f460);
            // c0600 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // c0605 mov rdi,rbx
            rdi = rbx;
            // c0608 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c060d lea rdi,[rip+0x10802c] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0614 mov rsi,rbx
            rsi = rbx;
            // c0617 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c061c mov DWORD PTR [rax],0x52
            m.w32(rax, (0x52u64) as u32);
            // c0622 lea rdi,[rbp-0xba0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f460);
            // c0629 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c062e lea rsi,[rip+0x78195] # 0x1387ca
            rsi = m.env.data_va(315287);
            // c0635 lea rbx,[rbp-0xbb8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f448);
            // c063c mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // c0641 mov rdi,rbx
            rdi = rbx;
            // c0644 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0649 lea rdi,[rip+0x107ff0] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0650 mov rsi,rbx
            rsi = rbx;
            // c0653 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0658 mov DWORD PTR [rax],0x53
            m.w32(rax, (0x53u64) as u32);
            // c065e lea rdi,[rbp-0xbb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f448);
            // c0665 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c066a lea rsi,[rip+0x78160] # 0x1387d1
            rsi = m.env.data_va(315294);
            // c0671 lea rbx,[rbp-0xbd0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f430);
            // c0678 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // c067d mov rdi,rbx
            rdi = rbx;
            // c0680 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0685 lea rdi,[rip+0x107fb4] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c068c mov rsi,rbx
            rsi = rbx;
            // c068f call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0694 mov DWORD PTR [rax],0x54
            m.w32(rax, (0x54u64) as u32);
            // c069a lea rdi,[rbp-0xbd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f430);
            // c06a1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c06a6 lea rsi,[rip+0x7812b] # 0x1387d8
            rsi = m.env.data_va(315301);
            // c06ad lea rbx,[rbp-0xbe8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f418);
            // c06b4 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // c06b9 mov rdi,rbx
            rdi = rbx;
            // c06bc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c06c1 lea rdi,[rip+0x107f78] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c06c8 mov rsi,rbx
            rsi = rbx;
            // c06cb call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c06d0 mov DWORD PTR [rax],0x4
            m.w32(rax, (0x4u64) as u32);
            // c06d6 lea rdi,[rbp-0xbe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f418);
            // c06dd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c06e2 lea rsi,[rip+0x780f4] # 0x1387dd
            rsi = m.env.data_va(315306);
            // c06e9 lea rbx,[rbp-0xc00]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f400);
            // c06f0 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // c06f5 mov rdi,rbx
            rdi = rbx;
            // c06f8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c06fd lea rdi,[rip+0x107f3c] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0704 mov rsi,rbx
            rsi = rbx;
            // c0707 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c070c mov DWORD PTR [rax],0x13
            m.w32(rax, (0x13u64) as u32);
            // c0712 lea rdi,[rbp-0xc00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f400);
            // c0719 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c071e lea rsi,[rip+0x780c5] # 0x1387ea
            rsi = m.env.data_va(315319);
            // c0725 lea rbx,[rbp-0xc18]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f3e8);
            // c072c mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0731 mov rdi,rbx
            rdi = rbx;
            // c0734 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0739 lea rdi,[rip+0x107f00] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0740 mov rsi,rbx
            rsi = rbx;
            // c0743 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0748 mov DWORD PTR [rax],0x12
            m.w32(rax, (0x12u64) as u32);
            // c074e lea rdi,[rbp-0xc18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3e8);
            // c0755 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c075a lea rsi,[rip+0x78093] # 0x1387f4
            rsi = m.env.data_va(315329);
            // c0761 lea rbx,[rbp-0xc30]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f3d0);
            // c0768 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // c076d mov rdi,rbx
            rdi = rbx;
            // c0770 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0775 lea rdi,[rip+0x107ec4] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c077c mov rsi,rbx
            rsi = rbx;
            // c077f call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0784 mov DWORD PTR [rax],0x12
            m.w32(rax, (0x12u64) as u32);
            // c078a lea rdi,[rbp-0xc30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3d0);
            // c0791 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0796 lea rsi,[rip+0x78064] # 0x138801
            rsi = m.env.data_va(315342);
            // c079d lea rbx,[rbp-0xc48]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f3b8);
            // c07a4 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // c07a9 mov rdi,rbx
            rdi = rbx;
            // c07ac call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c07b1 lea rdi,[rip+0x107e88] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c07b8 mov rsi,rbx
            rsi = rbx;
            // c07bb call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c07c0 mov DWORD PTR [rax],0x21
            m.w32(rax, (0x21u64) as u32);
            // c07c6 lea rdi,[rbp-0xc48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3b8);
            // c07cd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c07d2 lea rsi,[rip+0x78035] # 0x13880e
            rsi = m.env.data_va(315355);
            // c07d9 lea rbx,[rbp-0xc60]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f3a0);
            // c07e0 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c07e5 mov rdi,rbx
            rdi = rbx;
            // c07e8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c07ed lea rdi,[rip+0x107e4c] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c07f4 mov rsi,rbx
            rsi = rbx;
            // c07f7 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c07fc mov DWORD PTR [rax],0x20
            m.w32(rax, (0x20u64) as u32);
            // c0802 lea rdi,[rbp-0xc60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3a0);
            // c0809 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c080e lea rsi,[rip+0x78003] # 0x138818
            rsi = m.env.data_va(315365);
            // c0815 lea rbx,[rbp-0xc78]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f388);
            // c081c mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // c0821 mov rdi,rbx
            rdi = rbx;
            // c0824 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0829 lea rdi,[rip+0x107e10] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0830 mov rsi,rbx
            rsi = rbx;
            // c0833 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0838 mov DWORD PTR [rax],0x20
            m.w32(rax, (0x20u64) as u32);
            // c083e lea rdi,[rbp-0xc78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f388);
            // c0845 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c084a lea rsi,[rip+0x77fd4] # 0x138825
            rsi = m.env.data_va(315378);
            // c0851 lea rbx,[rbp-0xc90]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f370);
            // c0858 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // c085d mov rdi,rbx
            rdi = rbx;
            // c0860 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0865 lea rdi,[rip+0x107dd4] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c086c mov rsi,rbx
            rsi = rbx;
            // c086f call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0874 mov DWORD PTR [rax],0x56
            m.w32(rax, (0x56u64) as u32);
            // c087a lea rdi,[rbp-0xc90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f370);
            // c0881 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0886 lea rsi,[rip+0x77fa5] # 0x138832
            rsi = m.env.data_va(315391);
            // c088d lea rbx,[rbp-0xca8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f358);
            // c0894 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0899 mov rdi,rbx
            rdi = rbx;
            // c089c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c08a1 lea rdi,[rip+0x107d98] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c08a8 mov rsi,rbx
            rsi = rbx;
            // c08ab call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c08b0 mov DWORD PTR [rax],0x55
            m.w32(rax, (0x55u64) as u32);
            // c08b6 lea rdi,[rbp-0xca8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f358);
            // c08bd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c08c2 lea rsi,[rip+0x77f73] # 0x13883c
            rsi = m.env.data_va(315401);
            // c08c9 lea rbx,[rbp-0xcc0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f340);
            // c08d0 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // c08d5 mov rdi,rbx
            rdi = rbx;
            // c08d8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c08dd lea rdi,[rip+0x107d5c] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c08e4 mov rsi,rbx
            rsi = rbx;
            // c08e7 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c08ec mov DWORD PTR [rax],0x55
            m.w32(rax, (0x55u64) as u32);
            // c08f2 lea rdi,[rbp-0xcc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f340);
            // c08f9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c08fe lea rsi,[rip+0x77f44] # 0x138849
            rsi = m.env.data_va(315414);
            // c0905 lea rbx,[rbp-0xcd8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f328);
            // c090c mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // c0911 mov rdi,rbx
            rdi = rbx;
            // c0914 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0919 lea rdi,[rip+0x107d20] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0920 mov rsi,rbx
            rsi = rbx;
            // c0923 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0928 mov DWORD PTR [rax],0x58
            m.w32(rax, (0x58u64) as u32);
            // c092e lea rdi,[rbp-0xcd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f328);
            // c0935 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c093a lea rsi,[rip+0x77f15] # 0x138856
            rsi = m.env.data_va(315427);
            // c0941 lea rbx,[rbp-0xcf0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f310);
            // c0948 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c094d mov rdi,rbx
            rdi = rbx;
            // c0950 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0955 lea rdi,[rip+0x107ce4] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c095c mov rsi,rbx
            rsi = rbx;
            // c095f call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0964 mov DWORD PTR [rax],0x57
            m.w32(rax, (0x57u64) as u32);
            // c096a lea rdi,[rbp-0xcf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f310);
            // c0971 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0976 lea rsi,[rip+0x77ee3] # 0x138860
            rsi = m.env.data_va(315437);
            // c097d lea rbx,[rbp-0xd08]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f2f8);
            // c0984 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // c0989 mov rdi,rbx
            rdi = rbx;
            // c098c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0991 lea rdi,[rip+0x107ca8] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0998 mov rsi,rbx
            rsi = rbx;
            // c099b call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c09a0 mov DWORD PTR [rax],0x57
            m.w32(rax, (0x57u64) as u32);
            // c09a6 lea rdi,[rbp-0xd08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2f8);
            // c09ad call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c09b2 lea rsi,[rip+0x77831] # 0x1381ea
            rsi = m.env.data_va(313783);
            // c09b9 lea rbx,[rbp-0xd20]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f2e0);
            // c09c0 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // c09c5 mov rdi,rbx
            rdi = rbx;
            // c09c8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c09cd lea rdi,[rip+0x107c6c] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c09d4 mov rsi,rbx
            rsi = rbx;
            // c09d7 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c09dc mov DWORD PTR [rax],0x7
            m.w32(rax, (0x7u64) as u32);
            // c09e2 lea rdi,[rbp-0xd20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2e0);
            // c09e9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c09ee lea rsi,[rip+0x77809] # 0x1381fe
            rsi = m.env.data_va(313803);
            // c09f5 lea rbx,[rbp-0xd38]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f2c8);
            // c09fc mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // c0a01 mov rdi,rbx
            rdi = rbx;
            // c0a04 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0a09 lea rdi,[rip+0x107c30] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0a10 mov rsi,rbx
            rsi = rbx;
            // c0a13 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0a18 mov DWORD PTR [rax],0x6
            m.w32(rax, (0x6u64) as u32);
            // c0a1e lea rdi,[rbp-0xd38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2c8);
            // c0a25 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0a2a lea rsi,[rip+0x77e3c] # 0x13886d
            rsi = m.env.data_va(315450);
            // c0a31 lea rbx,[rbp-0xd50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f2b0);
            // c0a38 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0a3d mov rdi,rbx
            rdi = rbx;
            // c0a40 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0a45 lea rdi,[rip+0x107bf4] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0a4c mov rsi,rbx
            rsi = rbx;
            // c0a4f call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0a54 mov DWORD PTR [rax],0x1e
            m.w32(rax, (0x1eu64) as u32);
            // c0a5a lea rdi,[rbp-0xd50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2b0);
            // c0a61 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0a66 lea rsi,[rip+0x77e0a] # 0x138877
            rsi = m.env.data_va(315460);
            // c0a6d lea rbx,[rbp-0xd68]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f298);
            // c0a74 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0a79 mov rdi,rbx
            rdi = rbx;
            // c0a7c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0a81 lea rdi,[rip+0x107bb8] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0a88 mov rsi,rbx
            rsi = rbx;
            // c0a8b call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0a90 mov DWORD PTR [rax],0x1f
            m.w32(rax, (0x1fu64) as u32);
            // c0a96 lea rdi,[rbp-0xd68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f298);
            // c0a9d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0aa2 lea rsi,[rip+0x77dd8] # 0x138881
            rsi = m.env.data_va(315470);
            // c0aa9 lea rbx,[rbp-0xd80]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f280);
            // c0ab0 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0ab5 mov rdi,rbx
            rdi = rbx;
            // c0ab8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0abd lea rdi,[rip+0x107b7c] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0ac4 mov rsi,rbx
            rsi = rbx;
            // c0ac7 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0acc mov DWORD PTR [rax],0x32
            m.w32(rax, (0x32u64) as u32);
            // c0ad2 lea rdi,[rbp-0xd80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f280);
            // c0ad9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0ade lea rsi,[rip+0x77da6] # 0x13888b
            rsi = m.env.data_va(315480);
            // c0ae5 lea rbx,[rbp-0xd98]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f268);
            // c0aec mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0af1 mov rdi,rbx
            rdi = rbx;
            // c0af4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0af9 lea rdi,[rip+0x107b40] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0b00 mov rsi,rbx
            rsi = rbx;
            // c0b03 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0b08 mov DWORD PTR [rax],0x33
            m.w32(rax, (0x33u64) as u32);
            // c0b0e lea rdi,[rbp-0xd98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f268);
            // c0b15 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0b1a lea rsi,[rip+0x77d74] # 0x138895
            rsi = m.env.data_va(315490);
            // c0b21 lea rbx,[rbp-0xdb0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f250);
            // c0b28 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0b2d mov rdi,rbx
            rdi = rbx;
            // c0b30 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0b35 lea rdi,[rip+0x107b04] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0b3c mov rsi,rbx
            rsi = rbx;
            // c0b3f call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0b44 mov DWORD PTR [rax],0x47
            m.w32(rax, (0x47u64) as u32);
            // c0b4a lea rdi,[rbp-0xdb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f250);
            // c0b51 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0b56 lea rsi,[rip+0x77d42] # 0x13889f
            rsi = m.env.data_va(315500);
            // c0b5d lea rbx,[rbp-0xdc8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f238);
            // c0b64 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0b69 mov rdi,rbx
            rdi = rbx;
            // c0b6c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0b71 lea rdi,[rip+0x107ac8] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0b78 mov rsi,rbx
            rsi = rbx;
            // c0b7b call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0b80 mov DWORD PTR [rax],0x48
            m.w32(rax, (0x48u64) as u32);
            // c0b86 lea rdi,[rbp-0xdc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f238);
            // c0b8d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0b92 lea rsi,[rip+0x77d10] # 0x1388a9
            rsi = m.env.data_va(315510);
            // c0b99 lea rbx,[rbp-0xde0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f220);
            // c0ba0 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0ba5 mov rdi,rbx
            rdi = rbx;
            // c0ba8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0bad lea rdi,[rip+0x107a8c] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0bb4 mov rsi,rbx
            rsi = rbx;
            // c0bb7 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0bbc mov DWORD PTR [rax],0x4b
            m.w32(rax, (0x4bu64) as u32);
            // c0bc2 lea rdi,[rbp-0xde0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f220);
            // c0bc9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0bce lea rsi,[rip+0x77cde] # 0x1388b3
            rsi = m.env.data_va(315520);
            // c0bd5 lea rbx,[rbp-0xdf8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f208);
            // c0bdc mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // c0be1 mov rdi,rbx
            rdi = rbx;
            // c0be4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0be9 lea rdi,[rip+0x107a50] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c0bf0 mov rsi,rbx
            rsi = rbx;
            // c0bf3 call 0xebd62
            { let r = { let t = m.env.stub_va(0x52f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0bf8 mov DWORD PTR [rax],0x4c
            m.w32(rax, (0x4cu64) as u32);
            // c0bfe lea rdi,[rbp-0xdf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f208);
            // c0c05 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0c0a mov DWORD PTR [rbp-0xdfc],0x6e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f204), (0x6eu64) as u32);
            // c0c14 lea rbx,[rip+0x107a0d] # 0x1c8628
            rbx = m.env.data_va(904693);
            // c0c1b lea rsi,[rbp-0xdfc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f204);
            // c0c22 mov rdi,rbx
            rdi = rbx;
            // c0c25 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0c2a mov DWORD PTR [rax],0x5e
            m.w32(rax, (0x5eu64) as u32);
            // c0c30 mov DWORD PTR [rbp-0xe00],0x6f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f200), (0x6fu64) as u32);
            // c0c3a lea rsi,[rbp-0xe00]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f200);
            // c0c41 mov rdi,rbx
            rdi = rbx;
            // c0c44 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0c49 mov DWORD PTR [rax],0x5e
            m.w32(rax, (0x5eu64) as u32);
            // c0c4f mov DWORD PTR [rbp-0xe04],0x70
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1fc), (0x70u64) as u32);
            // c0c59 lea rsi,[rbp-0xe04]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1fc);
            // c0c60 mov rdi,rbx
            rdi = rbx;
            // c0c63 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0c68 mov DWORD PTR [rax],0x5f
            m.w32(rax, (0x5fu64) as u32);
            // c0c6e mov DWORD PTR [rbp-0xe08],0x71
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1f8), (0x71u64) as u32);
            // c0c78 lea rsi,[rbp-0xe08]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1f8);
            // c0c7f mov rdi,rbx
            rdi = rbx;
            // c0c82 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0c87 mov DWORD PTR [rax],0x5f
            m.w32(rax, (0x5fu64) as u32);
            // c0c8d mov DWORD PTR [rbp-0xe0c],0x72
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1f4), (0x72u64) as u32);
            // c0c97 lea rsi,[rbp-0xe0c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1f4);
            // c0c9e mov rdi,rbx
            rdi = rbx;
            // c0ca1 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0ca6 mov DWORD PTR [rax],0x60
            m.w32(rax, (0x60u64) as u32);
            // c0cac mov DWORD PTR [rbp-0xe10],0x73
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1f0), (0x73u64) as u32);
            // c0cb6 lea rsi,[rbp-0xe10]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1f0);
            // c0cbd mov rdi,rbx
            rdi = rbx;
            // c0cc0 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0cc5 mov DWORD PTR [rax],0x60
            m.w32(rax, (0x60u64) as u32);
            // c0ccb mov DWORD PTR [rbp-0xe14],0x74
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1ec), (0x74u64) as u32);
            // c0cd5 lea rsi,[rbp-0xe14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1ec);
            // c0cdc mov rdi,rbx
            rdi = rbx;
            // c0cdf call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0ce4 mov DWORD PTR [rax],0x61
            m.w32(rax, (0x61u64) as u32);
            // c0cea mov DWORD PTR [rbp-0xe18],0x75
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1e8), (0x75u64) as u32);
            // c0cf4 lea rsi,[rbp-0xe18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1e8);
            // c0cfb mov rdi,rbx
            rdi = rbx;
            // c0cfe call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0d03 mov DWORD PTR [rax],0x61
            m.w32(rax, (0x61u64) as u32);
            // c0d09 mov DWORD PTR [rbp-0xe1c],0x6a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1e4), (0x6au64) as u32);
            // c0d13 lea rsi,[rbp-0xe1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1e4);
            // c0d1a mov rdi,rbx
            rdi = rbx;
            // c0d1d call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0d22 mov DWORD PTR [rax],0x40
            m.w32(rax, (0x40u64) as u32);
            // c0d28 mov DWORD PTR [rbp-0xe20],0x6b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1e0), (0x6bu64) as u32);
            // c0d32 lea rsi,[rbp-0xe20]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1e0);
            // c0d39 mov rdi,rbx
            rdi = rbx;
            // c0d3c call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0d41 mov DWORD PTR [rax],0x40
            m.w32(rax, (0x40u64) as u32);
            // c0d47 mov DWORD PTR [rbp-0xe24],0x6c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1dc), (0x6cu64) as u32);
            // c0d51 lea rsi,[rbp-0xe24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1dc);
            // c0d58 mov rdi,rbx
            rdi = rbx;
            // c0d5b call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0d60 mov DWORD PTR [rax],0x40
            m.w32(rax, (0x40u64) as u32);
            // c0d66 mov DWORD PTR [rbp-0xe28],0x6d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1d8), (0x6du64) as u32);
            // c0d70 lea rsi,[rbp-0xe28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1d8);
            // c0d77 mov rdi,rbx
            rdi = rbx;
            // c0d7a call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0d7f mov DWORD PTR [rax],0x40
            m.w32(rax, (0x40u64) as u32);
            // c0d85 mov DWORD PTR [rbp-0xe2c],0x12
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1d4), (0x12u64) as u32);
            // c0d8f lea rsi,[rbp-0xe2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1d4);
            // c0d96 mov rdi,rbx
            rdi = rbx;
            // c0d99 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0d9e mov DWORD PTR [rax],0x4
            m.w32(rax, (0x4u64) as u32);
            // c0da4 mov DWORD PTR [rbp-0xe30],0x11
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1d0), (0x11u64) as u32);
            // c0dae lea rsi,[rbp-0xe30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1d0);
            // c0db5 mov rdi,rbx
            rdi = rbx;
            // c0db8 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0dbd mov DWORD PTR [rax],0x4
            m.w32(rax, (0x4u64) as u32);
            // c0dc3 mov DWORD PTR [rbp-0xe34],0x14
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1cc), (0x14u64) as u32);
            // c0dcd lea rsi,[rbp-0xe34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1cc);
            // c0dd4 mov rdi,rbx
            rdi = rbx;
            // c0dd7 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0ddc mov DWORD PTR [rax],0x5
            m.w32(rax, (0x5u64) as u32);
            // c0de2 mov DWORD PTR [rbp-0xe38],0x13
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1c8), (0x13u64) as u32);
            // c0dec lea rsi,[rbp-0xe38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1c8);
            // c0df3 mov rdi,rbx
            rdi = rbx;
            // c0df6 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0dfb mov DWORD PTR [rax],0x5
            m.w32(rax, (0x5u64) as u32);
            // c0e01 mov DWORD PTR [rbp-0xe3c],0x16
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1c4), (0x16u64) as u32);
            // c0e0b lea rsi,[rbp-0xe3c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1c4);
            // c0e12 mov rdi,rbx
            rdi = rbx;
            // c0e15 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0e1a mov DWORD PTR [rax],0x38
            m.w32(rax, (0x38u64) as u32);
            // c0e20 mov DWORD PTR [rbp-0xe40],0x15
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1c0), (0x15u64) as u32);
            // c0e2a lea rsi,[rbp-0xe40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1c0);
            // c0e31 mov rdi,rbx
            rdi = rbx;
            // c0e34 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0e39 mov DWORD PTR [rax],0x38
            m.w32(rax, (0x38u64) as u32);
            // c0e3f mov DWORD PTR [rbp-0xe44],0x18
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1bc), (0x18u64) as u32);
            // c0e49 lea rsi,[rbp-0xe44]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1bc);
            // c0e50 mov rdi,rbx
            rdi = rbx;
            // c0e53 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0e58 mov DWORD PTR [rax],0x39
            m.w32(rax, (0x39u64) as u32);
            // c0e5e mov DWORD PTR [rbp-0xe48],0x17
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1b8), (0x17u64) as u32);
            // c0e68 lea rsi,[rbp-0xe48]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1b8);
            // c0e6f mov rdi,rbx
            rdi = rbx;
            // c0e72 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0e77 mov DWORD PTR [rax],0x39
            m.w32(rax, (0x39u64) as u32);
            // c0e7d mov DWORD PTR [rbp-0xe4c],0x1a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1b4), (0x1au64) as u32);
            // c0e87 lea rsi,[rbp-0xe4c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1b4);
            // c0e8e mov rdi,rbx
            rdi = rbx;
            // c0e91 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0e96 mov DWORD PTR [rax],0x3d
            m.w32(rax, (0x3du64) as u32);
            // c0e9c mov DWORD PTR [rbp-0xe50],0x19
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1b0), (0x19u64) as u32);
            // c0ea6 lea rsi,[rbp-0xe50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1b0);
            // c0ead mov rdi,rbx
            rdi = rbx;
            // c0eb0 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0eb5 mov DWORD PTR [rax],0x3d
            m.w32(rax, (0x3du64) as u32);
            // c0ebb mov DWORD PTR [rbp-0xe54],0x1c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1ac), (0x1cu64) as u32);
            // c0ec5 lea rsi,[rbp-0xe54]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1ac);
            // c0ecc mov rdi,rbx
            rdi = rbx;
            // c0ecf call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0ed4 mov DWORD PTR [rax],0x50
            m.w32(rax, (0x50u64) as u32);
            // c0eda mov DWORD PTR [rbp-0xe58],0x1b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1a8), (0x1bu64) as u32);
            // c0ee4 lea rsi,[rbp-0xe58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1a8);
            // c0eeb mov rdi,rbx
            rdi = rbx;
            // c0eee call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0ef3 mov DWORD PTR [rax],0x50
            m.w32(rax, (0x50u64) as u32);
            // c0ef9 mov DWORD PTR [rbp-0xe5c],0x1e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1a4), (0x1eu64) as u32);
            // c0f03 lea rsi,[rbp-0xe5c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1a4);
            // c0f0a mov rdi,rbx
            rdi = rbx;
            // c0f0d call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0f12 mov DWORD PTR [rax],0x51
            m.w32(rax, (0x51u64) as u32);
            // c0f18 mov DWORD PTR [rbp-0xe60],0x1d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f1a0), (0x1du64) as u32);
            // c0f22 lea rsi,[rbp-0xe60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f1a0);
            // c0f29 mov rdi,rbx
            rdi = rbx;
            // c0f2c call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0f31 mov DWORD PTR [rax],0x51
            m.w32(rax, (0x51u64) as u32);
            // c0f37 mov DWORD PTR [rbp-0xe64],0x20
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f19c), (0x20u64) as u32);
            // c0f41 lea rsi,[rbp-0xe64]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f19c);
            // c0f48 mov rdi,rbx
            rdi = rbx;
            // c0f4b call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0f50 mov DWORD PTR [rax],0x52
            m.w32(rax, (0x52u64) as u32);
            // c0f56 mov DWORD PTR [rbp-0xe68],0x1f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f198), (0x1fu64) as u32);
            // c0f60 lea rsi,[rbp-0xe68]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f198);
            // c0f67 mov rdi,rbx
            rdi = rbx;
            // c0f6a call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0f6f mov DWORD PTR [rax],0x52
            m.w32(rax, (0x52u64) as u32);
            // c0f75 mov DWORD PTR [rbp-0xe6c],0x3d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f194), (0x3du64) as u32);
            // c0f7f lea rsi,[rbp-0xe6c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f194);
            // c0f86 mov rdi,rbx
            rdi = rbx;
            // c0f89 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0f8e mov DWORD PTR [rax],0x19
            m.w32(rax, (0x19u64) as u32);
            // c0f94 mov DWORD PTR [rbp-0xe70],0x3e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f190), (0x3eu64) as u32);
            // c0f9e lea rsi,[rbp-0xe70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f190);
            // c0fa5 mov rdi,rbx
            rdi = rbx;
            // c0fa8 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0fad mov DWORD PTR [rax],0x19
            m.w32(rax, (0x19u64) as u32);
            // c0fb3 mov DWORD PTR [rbp-0xe74],0x3f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f18c), (0x3fu64) as u32);
            // c0fbd lea rsi,[rbp-0xe74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f18c);
            // c0fc4 mov rdi,rbx
            rdi = rbx;
            // c0fc7 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0fcc mov DWORD PTR [rax],0x1a
            m.w32(rax, (0x1au64) as u32);
            // c0fd2 mov DWORD PTR [rbp-0xe78],0x40
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f188), (0x40u64) as u32);
            // c0fdc lea rsi,[rbp-0xe78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f188);
            // c0fe3 mov rdi,rbx
            rdi = rbx;
            // c0fe6 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c0feb mov DWORD PTR [rax],0x1a
            m.w32(rax, (0x1au64) as u32);
            // c0ff1 mov DWORD PTR [rbp-0xe7c],0x41
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f184), (0x41u64) as u32);
            // c0ffb lea rsi,[rbp-0xe7c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f184);
            // c1002 mov rdi,rbx
            rdi = rbx;
            // c1005 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c100a mov DWORD PTR [rax],0x34
            m.w32(rax, (0x34u64) as u32);
            // c1010 mov DWORD PTR [rbp-0xe80],0x42
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f180), (0x42u64) as u32);
            // c101a lea rsi,[rbp-0xe80]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f180);
            // c1021 mov rdi,rbx
            rdi = rbx;
            // c1024 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1029 mov DWORD PTR [rax],0x34
            m.w32(rax, (0x34u64) as u32);
            // c102f mov DWORD PTR [rbp-0xe84],0x43
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f17c), (0x43u64) as u32);
            // c1039 lea rsi,[rbp-0xe84]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f17c);
            // c1040 mov rdi,rbx
            rdi = rbx;
            // c1043 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1048 mov DWORD PTR [rax],0x35
            m.w32(rax, (0x35u64) as u32);
            // c104e mov DWORD PTR [rbp-0xe88],0x44
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f178), (0x44u64) as u32);
            // c1058 lea rsi,[rbp-0xe88]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f178);
            // c105f mov rdi,rbx
            rdi = rbx;
            // c1062 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1067 mov DWORD PTR [rax],0x35
            m.w32(rax, (0x35u64) as u32);
            // c106d mov DWORD PTR [rbp-0xe8c],0x45
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f174), (0x45u64) as u32);
            // c1077 lea rsi,[rbp-0xe8c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f174);
            // c107e mov rdi,rbx
            rdi = rbx;
            // c1081 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1086 mov DWORD PTR [rax],0x49
            m.w32(rax, (0x49u64) as u32);
            // c108c mov DWORD PTR [rbp-0xe90],0x46
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f170), (0x46u64) as u32);
            // c1096 lea rsi,[rbp-0xe90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f170);
            // c109d mov rdi,rbx
            rdi = rbx;
            // c10a0 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c10a5 mov DWORD PTR [rax],0x49
            m.w32(rax, (0x49u64) as u32);
            // c10ab mov DWORD PTR [rbp-0xe94],0x47
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f16c), (0x47u64) as u32);
            // c10b5 lea rsi,[rbp-0xe94]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f16c);
            // c10bc mov rdi,rbx
            rdi = rbx;
            // c10bf call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c10c4 mov DWORD PTR [rax],0x4a
            m.w32(rax, (0x4au64) as u32);
            // c10ca mov DWORD PTR [rbp-0xe98],0x48
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f168), (0x48u64) as u32);
            // c10d4 lea rsi,[rbp-0xe98]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f168);
            // c10db mov rdi,rbx
            rdi = rbx;
            // c10de call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c10e3 mov DWORD PTR [rax],0x4a
            m.w32(rax, (0x4au64) as u32);
            // c10e9 mov DWORD PTR [rbp-0xe9c],0x49
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f164), (0x49u64) as u32);
            // c10f3 lea rsi,[rbp-0xe9c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f164);
            // c10fa mov rdi,rbx
            rdi = rbx;
            // c10fd call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1102 mov DWORD PTR [rax],0x4b
            m.w32(rax, (0x4bu64) as u32);
            // c1108 mov DWORD PTR [rbp-0xea0],0x4a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f160), (0x4au64) as u32);
            // c1112 lea rsi,[rbp-0xea0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f160);
            // c1119 mov rdi,rbx
            rdi = rbx;
            // c111c call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1121 mov DWORD PTR [rax],0x4b
            m.w32(rax, (0x4bu64) as u32);
            // c1127 mov DWORD PTR [rbp-0xea4],0x4b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f15c), (0x4bu64) as u32);
            // c1131 lea rsi,[rbp-0xea4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f15c);
            // c1138 mov rdi,rbx
            rdi = rbx;
            // c113b call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1140 mov DWORD PTR [rax],0x4c
            m.w32(rax, (0x4cu64) as u32);
            // c1146 mov DWORD PTR [rbp-0xea8],0x4c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f158), (0x4cu64) as u32);
            // c1150 lea rsi,[rbp-0xea8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f158);
            // c1157 mov rdi,rbx
            rdi = rbx;
            // c115a call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c115f mov DWORD PTR [rax],0x4c
            m.w32(rax, (0x4cu64) as u32);
            // c1165 mov DWORD PTR [rbp-0xeac],0x4d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f154), (0x4du64) as u32);
            // c116f lea rsi,[rbp-0xeac]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f154);
            // c1176 mov rdi,rbx
            rdi = rbx;
            // c1179 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c117e mov DWORD PTR [rax],0x1d
            m.w32(rax, (0x1du64) as u32);
            // c1184 mov DWORD PTR [rbp-0xeb0],0x4e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f150), (0x4eu64) as u32);
            // c118e lea rsi,[rbp-0xeb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f150);
            // c1195 mov rdi,rbx
            rdi = rbx;
            // c1198 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c119d mov DWORD PTR [rax],0x1e
            m.w32(rax, (0x1eu64) as u32);
            // c11a3 mov DWORD PTR [rbp-0xeb4],0x4f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f14c), (0x4fu64) as u32);
            // c11ad lea rsi,[rbp-0xeb4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f14c);
            // c11b4 mov rdi,rbx
            rdi = rbx;
            // c11b7 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c11bc mov DWORD PTR [rax],0x36
            m.w32(rax, (0x36u64) as u32);
            // c11c2 mov DWORD PTR [rbp-0xeb8],0x50
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f148), (0x50u64) as u32);
            // c11cc lea rsi,[rbp-0xeb8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f148);
            // c11d3 mov rdi,rbx
            rdi = rbx;
            // c11d6 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c11db mov DWORD PTR [rax],0x37
            m.w32(rax, (0x37u64) as u32);
            // c11e1 mov DWORD PTR [rbp-0xebc],0x51
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f144), (0x51u64) as u32);
            // c11eb lea rsi,[rbp-0xebc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f144);
            // c11f2 mov rdi,rbx
            rdi = rbx;
            // c11f5 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c11fa mov DWORD PTR [rax],0x3f
            m.w32(rax, (0x3fu64) as u32);
            // c1200 mov DWORD PTR [rbp-0xec0],0x52
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f140), (0x52u64) as u32);
            // c120a lea rsi,[rbp-0xec0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f140);
            // c1211 mov rdi,rbx
            rdi = rbx;
            // c1214 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1219 mov DWORD PTR [rax],0x4d
            m.w32(rax, (0x4du64) as u32);
            // c121f mov DWORD PTR [rbp-0xec4],0x53
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f13c), (0x53u64) as u32);
            // c1229 lea rsi,[rbp-0xec4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f13c);
            // c1230 mov rdi,rbx
            rdi = rbx;
            // c1233 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1238 mov DWORD PTR [rax],0x4e
            m.w32(rax, (0x4eu64) as u32);
            // c123e mov DWORD PTR [rbp-0xec8],0x54
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f138), (0x54u64) as u32);
            // c1248 lea rsi,[rbp-0xec8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f138);
            // c124f mov rdi,rbx
            rdi = rbx;
            // c1252 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1257 mov DWORD PTR [rax],0x4f
            m.w32(rax, (0x4fu64) as u32);
            // c125d mov DWORD PTR [rbp-0xecc],0x1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f134), (0x1u64) as u32);
            // c1267 lea rsi,[rbp-0xecc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f134);
            // c126e mov rdi,rbx
            rdi = rbx;
            // c1271 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1276 mov DWORD PTR [rax],0x0
            m.w32(rax, (0x0u64) as u32);
            // c127c mov DWORD PTR [rbp-0xed0],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f130), (0x2u64) as u32);
            // c1286 lea rsi,[rbp-0xed0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f130);
            // c128d mov rdi,rbx
            rdi = rbx;
            // c1290 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1295 mov DWORD PTR [rax],0x0
            m.w32(rax, (0x0u64) as u32);
            // c129b mov DWORD PTR [rbp-0xed4],0x3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f12c), (0x3u64) as u32);
            // c12a5 lea rsi,[rbp-0xed4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f12c);
            // c12ac mov rdi,rbx
            rdi = rbx;
            // c12af call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c12b4 mov DWORD PTR [rax],0x1
            m.w32(rax, (0x1u64) as u32);
            // c12ba mov DWORD PTR [rbp-0xed8],0x4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f128), (0x4u64) as u32);
            // c12c4 lea rsi,[rbp-0xed8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f128);
            // c12cb mov rdi,rbx
            rdi = rbx;
            // c12ce call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c12d3 mov DWORD PTR [rax],0x1
            m.w32(rax, (0x1u64) as u32);
            // c12d9 mov DWORD PTR [rbp-0xedc],0x5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f124), (0x5u64) as u32);
            // c12e3 lea rsi,[rbp-0xedc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f124);
            // c12ea mov rdi,rbx
            rdi = rbx;
            // c12ed call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c12f2 mov DWORD PTR [rax],0x2
            m.w32(rax, (0x2u64) as u32);
            // c12f8 mov DWORD PTR [rbp-0xee0],0x6
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f120), (0x6u64) as u32);
            // c1302 lea rsi,[rbp-0xee0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f120);
            // c1309 mov rdi,rbx
            rdi = rbx;
            // c130c call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1311 mov DWORD PTR [rax],0x2
            m.w32(rax, (0x2u64) as u32);
            // c1317 mov DWORD PTR [rbp-0xee4],0x7
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f11c), (0x7u64) as u32);
            // c1321 lea rsi,[rbp-0xee4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f11c);
            // c1328 mov rdi,rbx
            rdi = rbx;
            // c132b call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1330 mov DWORD PTR [rax],0x3
            m.w32(rax, (0x3u64) as u32);
            // c1336 mov DWORD PTR [rbp-0xee8],0x8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f118), (0x8u64) as u32);
            // c1340 lea rsi,[rbp-0xee8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f118);
            // c1347 mov rdi,rbx
            rdi = rbx;
            // c134a call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c134f mov DWORD PTR [rax],0x3
            m.w32(rax, (0x3u64) as u32);
            // c1355 mov DWORD PTR [rbp-0xeec],0x9
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f114), (0x9u64) as u32);
            // c135f lea rsi,[rbp-0xeec]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f114);
            // c1366 mov rdi,rbx
            rdi = rbx;
            // c1369 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c136e mov DWORD PTR [rax],0x58
            m.w32(rax, (0x58u64) as u32);
            // c1374 mov DWORD PTR [rbp-0xef0],0xa
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f110), (0xau64) as u32);
            // c137e lea rsi,[rbp-0xef0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f110);
            // c1385 mov rdi,rbx
            rdi = rbx;
            // c1388 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c138d mov DWORD PTR [rax],0x58
            m.w32(rax, (0x58u64) as u32);
            // c1393 mov DWORD PTR [rbp-0xef4],0xb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f10c), (0xbu64) as u32);
            // c139d lea rsi,[rbp-0xef4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f10c);
            // c13a4 mov rdi,rbx
            rdi = rbx;
            // c13a7 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c13ac mov DWORD PTR [rax],0x59
            m.w32(rax, (0x59u64) as u32);
            // c13b2 mov DWORD PTR [rbp-0xef8],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f108), (0xcu64) as u32);
            // c13bc lea rsi,[rbp-0xef8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f108);
            // c13c3 mov rdi,rbx
            rdi = rbx;
            // c13c6 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c13cb mov DWORD PTR [rax],0x59
            m.w32(rax, (0x59u64) as u32);
            // c13d1 mov DWORD PTR [rbp-0xefc],0xd
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f104), (0xdu64) as u32);
            // c13db lea rsi,[rbp-0xefc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f104);
            // c13e2 mov rdi,rbx
            rdi = rbx;
            // c13e5 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c13ea mov DWORD PTR [rax],0x5a
            m.w32(rax, (0x5au64) as u32);
            // c13f0 mov DWORD PTR [rbp-0xf00],0xe
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f100), (0xeu64) as u32);
            // c13fa lea rsi,[rbp-0xf00]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f100);
            // c1401 mov rdi,rbx
            rdi = rbx;
            // c1404 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1409 mov DWORD PTR [rax],0x5a
            m.w32(rax, (0x5au64) as u32);
            // c140f mov DWORD PTR [rbp-0xf04],0xf
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0fc), (0xfu64) as u32);
            // c1419 lea rsi,[rbp-0xf04]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0fc);
            // c1420 mov rdi,rbx
            rdi = rbx;
            // c1423 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1428 mov DWORD PTR [rax],0x5b
            m.w32(rax, (0x5bu64) as u32);
            // c142e mov DWORD PTR [rbp-0xf08],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0f8), (0x10u64) as u32);
            // c1438 lea rsi,[rbp-0xf08]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0f8);
            // c143f mov rdi,rbx
            rdi = rbx;
            // c1442 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1447 mov DWORD PTR [rax],0x5b
            m.w32(rax, (0x5bu64) as u32);
            // c144d mov DWORD PTR [rbp-0xf0c],0x21
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0f4), (0x21u64) as u32);
            // c1457 lea rsi,[rbp-0xf0c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0f4);
            // c145e mov rdi,rbx
            rdi = rbx;
            // c1461 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1466 mov DWORD PTR [rax],0x6
            m.w32(rax, (0x6u64) as u32);
            // c146c mov DWORD PTR [rbp-0xf10],0x22
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0f0), (0x22u64) as u32);
            // c1476 lea rsi,[rbp-0xf10]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0f0);
            // c147d mov rdi,rbx
            rdi = rbx;
            // c1480 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1485 mov DWORD PTR [rax],0x7
            m.w32(rax, (0x7u64) as u32);
            // c148b mov DWORD PTR [rbp-0xf14],0x23
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0ec), (0x23u64) as u32);
            // c1495 lea rsi,[rbp-0xf14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0ec);
            // c149c mov rdi,rbx
            rdi = rbx;
            // c149f call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c14a4 mov DWORD PTR [rax],0x32
            m.w32(rax, (0x32u64) as u32);
            // c14aa mov DWORD PTR [rbp-0xf18],0x24
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0e8), (0x24u64) as u32);
            // c14b4 lea rsi,[rbp-0xf18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0e8);
            // c14bb mov rdi,rbx
            rdi = rbx;
            // c14be call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c14c3 mov DWORD PTR [rax],0x33
            m.w32(rax, (0x33u64) as u32);
            // c14c9 mov DWORD PTR [rbp-0xf1c],0x25
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0e4), (0x25u64) as u32);
            // c14d3 lea rsi,[rbp-0xf1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0e4);
            // c14da mov rdi,rbx
            rdi = rbx;
            // c14dd call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c14e2 mov DWORD PTR [rax],0x3e
            m.w32(rax, (0x3eu64) as u32);
            // c14e8 mov DWORD PTR [rbp-0xf20],0x26
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0e0), (0x26u64) as u32);
            // c14f2 lea rsi,[rbp-0xf20]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0e0);
            // c14f9 mov rdi,rbx
            rdi = rbx;
            // c14fc call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1501 mov DWORD PTR [rax],0x53
            m.w32(rax, (0x53u64) as u32);
            // c1507 mov DWORD PTR [rbp-0xf24],0x27
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0dc), (0x27u64) as u32);
            // c1511 lea rsi,[rbp-0xf24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0dc);
            // c1518 mov rdi,rbx
            rdi = rbx;
            // c151b call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1520 mov DWORD PTR [rax],0x54
            m.w32(rax, (0x54u64) as u32);
            // c1526 mov DWORD PTR [rbp-0xf28],0x28
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0d8), (0x28u64) as u32);
            // c1530 lea rsi,[rbp-0xf28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0d8);
            // c1537 mov rdi,rbx
            rdi = rbx;
            // c153a call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c153f mov DWORD PTR [rax],0x55
            m.w32(rax, (0x55u64) as u32);
            // c1545 mov DWORD PTR [rbp-0xf2c],0x55
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0d4), (0x55u64) as u32);
            // c154f lea rsi,[rbp-0xf2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0d4);
            // c1556 mov rdi,rbx
            rdi = rbx;
            // c1559 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c155e mov DWORD PTR [rax],0x26
            m.w32(rax, (0x26u64) as u32);
            // c1564 mov DWORD PTR [rbp-0xf30],0x56
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0d0), (0x56u64) as u32);
            // c156e lea rsi,[rbp-0xf30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0d0);
            // c1575 mov rdi,rbx
            rdi = rbx;
            // c1578 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c157d mov DWORD PTR [rax],0x26
            m.w32(rax, (0x26u64) as u32);
            // c1583 mov DWORD PTR [rbp-0xf34],0x57
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0cc), (0x57u64) as u32);
            // c158d lea rsi,[rbp-0xf34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0cc);
            // c1594 mov rdi,rbx
            rdi = rbx;
            // c1597 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c159c mov DWORD PTR [rax],0x26
            m.w32(rax, (0x26u64) as u32);
            // c15a2 mov DWORD PTR [rbp-0xf38],0x58
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0c8), (0x58u64) as u32);
            // c15ac lea rsi,[rbp-0xf38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0c8);
            // c15b3 mov rdi,rbx
            rdi = rbx;
            // c15b6 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c15bb mov DWORD PTR [rax],0x26
            m.w32(rax, (0x26u64) as u32);
            // c15c1 mov DWORD PTR [rbp-0xf3c],0x59
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0c4), (0x59u64) as u32);
            // c15cb lea rsi,[rbp-0xf3c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0c4);
            // c15d2 mov rdi,rbx
            rdi = rbx;
            // c15d5 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c15da mov DWORD PTR [rax],0x30
            m.w32(rax, (0x30u64) as u32);
            // c15e0 mov DWORD PTR [rbp-0xf40],0x5a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0c0), (0x5au64) as u32);
            // c15ea lea rsi,[rbp-0xf40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0c0);
            // c15f1 mov rdi,rbx
            rdi = rbx;
            // c15f4 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c15f9 mov DWORD PTR [rax],0x30
            m.w32(rax, (0x30u64) as u32);
            // c15ff mov DWORD PTR [rbp-0xf44],0x5b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0bc), (0x5bu64) as u32);
            // c1609 lea rsi,[rbp-0xf44]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0bc);
            // c1610 mov rdi,rbx
            rdi = rbx;
            // c1613 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1618 mov DWORD PTR [rax],0x30
            m.w32(rax, (0x30u64) as u32);
            // c161e mov DWORD PTR [rbp-0xf48],0x5c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0b8), (0x5cu64) as u32);
            // c1628 lea rsi,[rbp-0xf48]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0b8);
            // c162f mov rdi,rbx
            rdi = rbx;
            // c1632 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1637 mov DWORD PTR [rax],0x30
            m.w32(rax, (0x30u64) as u32);
            // c163d mov DWORD PTR [rbp-0xf4c],0x5d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0b4), (0x5du64) as u32);
            // c1647 lea rsi,[rbp-0xf4c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0b4);
            // c164e mov rdi,rbx
            rdi = rbx;
            // c1651 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1656 mov DWORD PTR [rax],0x56
            m.w32(rax, (0x56u64) as u32);
            // c165c mov DWORD PTR [rbp-0xf50],0x5e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0b0), (0x5eu64) as u32);
            // c1666 lea rsi,[rbp-0xf50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0b0);
            // c166d mov rdi,rbx
            rdi = rbx;
            // c1670 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1675 mov DWORD PTR [rax],0x56
            m.w32(rax, (0x56u64) as u32);
            // c167b mov DWORD PTR [rbp-0xf54],0x5f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0ac), (0x5fu64) as u32);
            // c1685 lea rsi,[rbp-0xf54]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0ac);
            // c168c mov rdi,rbx
            rdi = rbx;
            // c168f call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1694 mov DWORD PTR [rax],0x56
            m.w32(rax, (0x56u64) as u32);
            // c169a mov DWORD PTR [rbp-0xf58],0x60
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0a8), (0x60u64) as u32);
            // c16a4 lea rsi,[rbp-0xf58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0a8);
            // c16ab mov rdi,rbx
            rdi = rbx;
            // c16ae call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c16b3 mov DWORD PTR [rax],0x56
            m.w32(rax, (0x56u64) as u32);
            // c16b9 mov DWORD PTR [rbp-0xf5c],0x61
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0a4), (0x61u64) as u32);
            // c16c3 lea rsi,[rbp-0xf5c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0a4);
            // c16ca mov rdi,rbx
            rdi = rbx;
            // c16cd call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c16d2 mov DWORD PTR [rax],0x57
            m.w32(rax, (0x57u64) as u32);
            // c16d8 mov DWORD PTR [rbp-0xf60],0x62
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f0a0), (0x62u64) as u32);
            // c16e2 lea rsi,[rbp-0xf60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f0a0);
            // c16e9 mov rdi,rbx
            rdi = rbx;
            // c16ec call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c16f1 mov DWORD PTR [rax],0x57
            m.w32(rax, (0x57u64) as u32);
            // c16f7 mov DWORD PTR [rbp-0xf64],0x63
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f09c), (0x63u64) as u32);
            // c1701 lea rsi,[rbp-0xf64]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f09c);
            // c1708 mov rdi,rbx
            rdi = rbx;
            // c170b call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1710 mov DWORD PTR [rax],0x57
            m.w32(rax, (0x57u64) as u32);
            // c1716 mov DWORD PTR [rbp-0xf68],0x64
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f098), (0x64u64) as u32);
            // c1720 lea rsi,[rbp-0xf68]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f098);
            // c1727 mov rdi,rbx
            rdi = rbx;
            // c172a call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c172f mov DWORD PTR [rax],0x57
            m.w32(rax, (0x57u64) as u32);
            // c1735 mov DWORD PTR [rbp-0xf6c],0x7c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f094), (0x7cu64) as u32);
            // c173f lea rsi,[rbp-0xf6c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f094);
            // c1746 mov rdi,rbx
            rdi = rbx;
            // c1749 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c174e mov DWORD PTR [rax],0x2b
            m.w32(rax, (0x2bu64) as u32);
            // c1754 mov DWORD PTR [rbp-0xf70],0x77
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f090), (0x77u64) as u32);
            // c175e lea rsi,[rbp-0xf70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f090);
            // c1765 mov rdi,rbx
            rdi = rbx;
            // c1768 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c176d mov DWORD PTR [rax],0x2c
            m.w32(rax, (0x2cu64) as u32);
            // c1773 mov DWORD PTR [rbp-0xf74],0x79
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f08c), (0x79u64) as u32);
            // c177d lea rsi,[rbp-0xf74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f08c);
            // c1784 mov rdi,rbx
            rdi = rbx;
            // c1787 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c178c mov DWORD PTR [rax],0x31
            m.w32(rax, (0x31u64) as u32);
            // c1792 mov DWORD PTR [rbp-0xf78],0x7a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f088), (0x7au64) as u32);
            // c179c lea rsi,[rbp-0xf78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f088);
            // c17a3 mov rdi,rbx
            rdi = rbx;
            // c17a6 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c17ab mov DWORD PTR [rax],0x31
            m.w32(rax, (0x31u64) as u32);
            // c17b1 mov DWORD PTR [rbp-0xf7c],0x7d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f084), (0x7du64) as u32);
            // c17bb lea rsi,[rbp-0xf7c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f084);
            // c17c2 mov rdi,rbx
            rdi = rbx;
            // c17c5 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c17ca mov DWORD PTR [rax],0x29
            m.w32(rax, (0x29u64) as u32);
            // c17d0 mov DWORD PTR [rbp-0xf80],0x7e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f080), (0x7eu64) as u32);
            // c17da lea rsi,[rbp-0xf80]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f080);
            // c17e1 mov rdi,rbx
            rdi = rbx;
            // c17e4 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c17e9 mov DWORD PTR [rax],0x2a
            m.w32(rax, (0x2au64) as u32);
            // c17ef mov DWORD PTR [rbp-0xf84],0x85
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f07c), (0x85u64) as u32);
            // c17f9 lea rsi,[rbp-0xf84]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f07c);
            // c1800 mov rdi,rbx
            rdi = rbx;
            // c1803 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1808 mov DWORD PTR [rax],0x8
            m.w32(rax, (0x8u64) as u32);
            // c180e mov DWORD PTR [rbp-0xf88],0x84
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f078), (0x84u64) as u32);
            // c1818 lea rsi,[rbp-0xf88]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f078);
            // c181f mov rdi,rbx
            rdi = rbx;
            // c1822 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1827 mov DWORD PTR [rax],0x9
            m.w32(rax, (0x9u64) as u32);
            // c182d mov DWORD PTR [rbp-0xf8c],0x81
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f074), (0x81u64) as u32);
            // c1837 lea rsi,[rbp-0xf8c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f074);
            // c183e mov rdi,rbx
            rdi = rbx;
            // c1841 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1846 mov DWORD PTR [rax],0x27
            m.w32(rax, (0x27u64) as u32);
            // c184c mov DWORD PTR [rbp-0xf90],0x82
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f070), (0x82u64) as u32);
            // c1856 lea rsi,[rbp-0xf90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f070);
            // c185d mov rdi,rbx
            rdi = rbx;
            // c1860 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1865 mov DWORD PTR [rax],0x24
            m.w32(rax, (0x24u64) as u32);
            // c186b mov DWORD PTR [rbp-0xf94],0x80
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f06c), (0x80u64) as u32);
            // c1875 lea rsi,[rbp-0xf94]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f06c);
            // c187c mov rdi,rbx
            rdi = rbx;
            // c187f call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1884 mov DWORD PTR [rax],0x25
            m.w32(rax, (0x25u64) as u32);
            // c188a mov DWORD PTR [rbp-0xf98],0x76
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f068), (0x76u64) as u32);
            // c1894 lea rsi,[rbp-0xf98]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f068);
            // c189b mov rdi,rbx
            rdi = rbx;
            // c189e call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c18a3 mov DWORD PTR [rax],0x20
            m.w32(rax, (0x20u64) as u32);
            // c18a9 mov DWORD PTR [rbp-0xf9c],0x78
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f064), (0x78u64) as u32);
            // c18b3 lea rsi,[rbp-0xf9c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f064);
            // c18ba mov rdi,rbx
            rdi = rbx;
            // c18bd call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c18c2 mov DWORD PTR [rax],0x21
            m.w32(rax, (0x21u64) as u32);
            // c18c8 mov DWORD PTR [rbp-0xfa0],0x2d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f060), (0x2du64) as u32);
            // c18d2 lea rsi,[rbp-0xfa0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f060);
            // c18d9 mov rdi,rbx
            rdi = rbx;
            // c18dc call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c18e1 mov DWORD PTR [rax],0x10
            m.w32(rax, (0x10u64) as u32);
            // c18e7 mov DWORD PTR [rbp-0xfa4],0x2e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f05c), (0x2eu64) as u32);
            // c18f1 lea rsi,[rbp-0xfa4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f05c);
            // c18f8 mov rdi,rbx
            rdi = rbx;
            // c18fb call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1900 mov DWORD PTR [rax],0x14
            m.w32(rax, (0x14u64) as u32);
            // c1906 mov DWORD PTR [rbp-0xfa8],0x2f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f058), (0x2fu64) as u32);
            // c1910 lea rsi,[rbp-0xfa8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f058);
            // c1917 mov rdi,rbx
            rdi = rbx;
            // c191a call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c191f mov DWORD PTR [rax],0x11
            m.w32(rax, (0x11u64) as u32);
            // c1925 mov DWORD PTR [rbp-0xfac],0x30
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f054), (0x30u64) as u32);
            // c192f lea rsi,[rbp-0xfac]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f054);
            // c1936 mov rdi,rbx
            rdi = rbx;
            // c1939 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c193e mov DWORD PTR [rax],0x15
            m.w32(rax, (0x15u64) as u32);
            // c1944 mov DWORD PTR [rbp-0xfb0],0x31
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f050), (0x31u64) as u32);
            // c194e lea rsi,[rbp-0xfb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f050);
            // c1955 mov rdi,rbx
            rdi = rbx;
            // c1958 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c195d mov DWORD PTR [rax],0x12
            m.w32(rax, (0x12u64) as u32);
            // c1963 mov DWORD PTR [rbp-0xfb4],0x32
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f04c), (0x32u64) as u32);
            // c196d lea rsi,[rbp-0xfb4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f04c);
            // c1974 mov rdi,rbx
            rdi = rbx;
            // c1977 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c197c mov DWORD PTR [rax],0x16
            m.w32(rax, (0x16u64) as u32);
            // c1982 mov DWORD PTR [rbp-0xfb8],0x33
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f048), (0x33u64) as u32);
            // c198c lea rsi,[rbp-0xfb8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f048);
            // c1993 mov rdi,rbx
            rdi = rbx;
            // c1996 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c199b mov DWORD PTR [rax],0x13
            m.w32(rax, (0x13u64) as u32);
            // c19a1 mov DWORD PTR [rbp-0xfbc],0x34
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f044), (0x34u64) as u32);
            // c19ab lea rsi,[rbp-0xfbc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f044);
            // c19b2 mov rdi,rbx
            rdi = rbx;
            // c19b5 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c19ba mov DWORD PTR [rax],0x17
            m.w32(rax, (0x17u64) as u32);
            // c19c0 mov DWORD PTR [rbp-0xfc0],0x35
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f040), (0x35u64) as u32);
            // c19ca lea rsi,[rbp-0xfc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f040);
            // c19d1 mov rdi,rbx
            rdi = rbx;
            // c19d4 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c19d9 mov DWORD PTR [rax],0x45
            m.w32(rax, (0x45u64) as u32);
            // c19df mov DWORD PTR [rbp-0xfc4],0x36
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f03c), (0x36u64) as u32);
            // c19e9 lea rsi,[rbp-0xfc4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f03c);
            // c19f0 mov rdi,rbx
            rdi = rbx;
            // c19f3 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c19f8 mov DWORD PTR [rax],0x45
            m.w32(rax, (0x45u64) as u32);
            // c19fe mov DWORD PTR [rbp-0xfc8],0x37
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f038), (0x37u64) as u32);
            // c1a08 lea rsi,[rbp-0xfc8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f038);
            // c1a0f mov rdi,rbx
            rdi = rbx;
            // c1a12 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1a17 mov DWORD PTR [rax],0x46
            m.w32(rax, (0x46u64) as u32);
            // c1a1d mov DWORD PTR [rbp-0xfcc],0x38
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f034), (0x38u64) as u32);
            // c1a27 lea rsi,[rbp-0xfcc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f034);
            // c1a2e mov rdi,rbx
            rdi = rbx;
            // c1a31 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1a36 mov DWORD PTR [rax],0x46
            m.w32(rax, (0x46u64) as u32);
            // c1a3c mov DWORD PTR [rbp-0xfd0],0x39
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f030), (0x39u64) as u32);
            // c1a46 lea rsi,[rbp-0xfd0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f030);
            // c1a4d mov rdi,rbx
            rdi = rbx;
            // c1a50 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1a55 mov DWORD PTR [rax],0x47
            m.w32(rax, (0x47u64) as u32);
            // c1a5b mov DWORD PTR [rbp-0xfd4],0x3a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f02c), (0x3au64) as u32);
            // c1a65 lea rsi,[rbp-0xfd4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f02c);
            // c1a6c mov rdi,rbx
            rdi = rbx;
            // c1a6f call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1a74 mov DWORD PTR [rax],0x47
            m.w32(rax, (0x47u64) as u32);
            // c1a7a mov DWORD PTR [rbp-0xfd8],0x3b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f028), (0x3bu64) as u32);
            // c1a84 lea rsi,[rbp-0xfd8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f028);
            // c1a8b mov rdi,rbx
            rdi = rbx;
            // c1a8e call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1a93 mov DWORD PTR [rax],0x48
            m.w32(rax, (0x48u64) as u32);
            // c1a99 mov DWORD PTR [rbp-0xfdc],0x3c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f024), (0x3cu64) as u32);
            // c1aa3 lea rsi,[rbp-0xfdc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f024);
            // c1aaa mov rdi,rbx
            rdi = rbx;
            // c1aad call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1ab2 mov DWORD PTR [rax],0x48
            m.w32(rax, (0x48u64) as u32);
            // c1ab8 mov DWORD PTR [rbp-0xfe0],0x65
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f020), (0x65u64) as u32);
            // c1ac2 lea rsi,[rbp-0xfe0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f020);
            // c1ac9 mov rdi,rbx
            rdi = rbx;
            // c1acc call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1ad1 mov DWORD PTR [rax],0x23
            m.w32(rax, (0x23u64) as u32);
            // c1ad7 mov DWORD PTR [rbp-0xfe4],0x66
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f01c), (0x66u64) as u32);
            // c1ae1 lea rsi,[rbp-0xfe4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f01c);
            // c1ae8 mov rdi,rbx
            rdi = rbx;
            // c1aeb call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1af0 mov DWORD PTR [rax],0x5d
            m.w32(rax, (0x5du64) as u32);
            // c1af6 mov DWORD PTR [rbp-0xfe8],0x67
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f018), (0x67u64) as u32);
            // c1b00 lea rsi,[rbp-0xfe8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f018);
            // c1b07 mov rdi,rbx
            rdi = rbx;
            // c1b0a call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1b0f mov DWORD PTR [rax],0x5d
            m.w32(rax, (0x5du64) as u32);
            // c1b15 mov DWORD PTR [rbp-0xfec],0x68
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f014), (0x68u64) as u32);
            // c1b1f lea rsi,[rbp-0xfec]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f014);
            // c1b26 mov rdi,rbx
            rdi = rbx;
            // c1b29 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1b2e mov DWORD PTR [rax],0x5d
            m.w32(rax, (0x5du64) as u32);
            // c1b34 mov DWORD PTR [rbp-0xff0],0x69
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f010), (0x69u64) as u32);
            // c1b3e lea rsi,[rbp-0xff0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f010);
            // c1b45 mov rdi,rbx
            rdi = rbx;
            // c1b48 call 0xebd14
            { let r = { let t = m.env.stub_va(0x4e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1b4d mov DWORD PTR [rax],0x5d
            m.w32(rax, (0x5du64) as u32);
            // c1b53 mov rbx,QWORD PTR [rip+0x106ace] # 0x1c8628
            rbx = m.r64(m.env.data_va(904693)) as u64;
            // c1b5a lea r15,[rip+0x106acf] # 0x1c8630
            r15 = m.env.data_va(904701);
            // c1b61 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // c1b64 je 0xc1bbb
            if crate::cov::brk(0xc1b64, fl.e()) { pc = 0xc1bbb; continue; }
            pc = 0xc1b66;
        }
        0xc1b66 => {
            // block c1b66
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c1b66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c1b66 lea r14,[rip+0x106b33] # 0x1c86a0
            r14 = m.env.data_va(904813);
            // c1b6d lea r12,[rbp-0xff8]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_f008);
            pc = 0xc1b74;
        }
        0xc1b74 => {
            // block c1b74
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c1b74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c1b74 mov eax,DWORD PTR [rbx+0x20]
            rax = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // c1b77 mov DWORD PTR [rbp-0xff8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f008), ((rax as u32 as u64)) as u32);
            // c1b7d mov eax,DWORD PTR [rbx+0x1c]
            rax = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c1b80 mov DWORD PTR [rbp-0xff4],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f00c), ((rax as u32 as u64)) as u32);
            // c1b86 mov rdi,r14
            rdi = r14;
            // c1b89 mov rsi,r12
            rsi = r12;
            // c1b8c call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1b91 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c1b95 test rax,rax
            fl = logic::<64>(rax & rax);
            // c1b98 je 0xc1ba7
            if crate::cov::brk(0xc1b98, fl.e()) { pc = 0xc1ba7; continue; }
            pc = 0xc1b9a;
        }
        0xc1b9a => {
            // block c1b9a
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c1b9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c1b9a mov rbx,rax
            rbx = rax;
            // c1b9d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c1ba0 test rax,rax
            fl = logic::<64>(rax & rax);
            // c1ba3 jne 0xc1b9a
            if crate::cov::brk(0xc1ba3, fl.ne()) { pc = 0xc1b9a; continue; }
            pc = 0xc1ba5;
        }
        0xc1ba5 => {
            // block c1ba5
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c1ba5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c1ba5 jmp 0xc1bb6
            pc = 0xc1bb6; continue;
        }
        0xc1ba7 => {
            // block c1ba7
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c1ba7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c1ba7 mov rax,rbx
            rax = rbx;
            pc = 0xc1baa;
        }
        0xc1baa => {
            // block c1baa
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c1baa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c1baa mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // c1bae cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // c1bb1 mov rax,rbx
            rax = rbx;
            // c1bb4 jne 0xc1baa
            if crate::cov::brk(0xc1bb4, fl.ne()) { pc = 0xc1baa; continue; }
            pc = 0xc1bb6;
        }
        0xc1bb6 => {
            // block c1bb6
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c1bb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c1bb6 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // c1bb9 jne 0xc1b74
            if crate::cov::brk(0xc1bb9, fl.ne()) { pc = 0xc1b74; continue; }
            pc = 0xc1bbb;
        }
        0xc1bbb => {
            // block c1bbb
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c1bbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c1bbb movabs rax,0x2d00000014
            rax = 0x0_002d_0000_0014u64;
            // c1bc5 mov QWORD PTR [rbp-0x1000],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f000), (rax) as u64);
            // c1bcc lea rbx,[rip+0x106acd] # 0x1c86a0
            rbx = m.env.data_va(904813);
            // c1bd3 lea rsi,[rbp-0x1000]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f000);
            // c1bda mov rdi,rbx
            rdi = rbx;
            // c1bdd call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1be2 movabs rax,0x2f00000015
            rax = 0x0_002f_0000_0015u64;
            // c1bec mov QWORD PTR [rbp-0x1008],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eff8), (rax) as u64);
            // c1bf3 lea rsi,[rbp-0x1008]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eff8);
            // c1bfa mov rdi,rbx
            rdi = rbx;
            // c1bfd call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1c02 movabs rax,0x3100000016
            rax = 0x0_0031_0000_0016u64;
            // c1c0c mov QWORD PTR [rbp-0x1010],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_eff0), (rax) as u64);
            // c1c13 lea rsi,[rbp-0x1010]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eff0);
            // c1c1a mov rdi,rbx
            rdi = rbx;
            // c1c1d call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1c22 movabs rax,0x3300000017
            rax = 0x0_0033_0000_0017u64;
            // c1c2c mov QWORD PTR [rbp-0x1018],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efe8), (rax) as u64);
            // c1c33 lea rsi,[rbp-0x1018]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efe8);
            // c1c3a mov rdi,rbx
            rdi = rbx;
            // c1c3d call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1c42 movabs rax,0x350000003c
            rax = 0x0_0035_0000_003cu64;
            // c1c4c mov QWORD PTR [rbp-0x1020],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efe0), (rax) as u64);
            // c1c53 lea rsi,[rbp-0x1020]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efe0);
            // c1c5a mov rdi,rbx
            rdi = rbx;
            // c1c5d call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1c62 movabs rax,0x650000003a
            rax = 0x0_0065_0000_003au64;
            // c1c6c mov QWORD PTR [rbp-0x1028],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efd8), (rax) as u64);
            // c1c73 lea rsi,[rbp-0x1028]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efd8);
            // c1c7a mov rdi,rbx
            rdi = rbx;
            // c1c7d call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1c82 movabs rax,0x660000003a
            rax = 0x0_0066_0000_003au64;
            // c1c8c mov QWORD PTR [rbp-0x1030],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efd0), (rax) as u64);
            // c1c93 lea rsi,[rbp-0x1030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efd0);
            // c1c9a mov rdi,rbx
            rdi = rbx;
            // c1c9d call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1ca2 movabs rax,0x670000003a
            rax = 0x0_0067_0000_003au64;
            // c1cac mov QWORD PTR [rbp-0x1038],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efc8), (rax) as u64);
            // c1cb3 lea rsi,[rbp-0x1038]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efc8);
            // c1cba mov rdi,rbx
            rdi = rbx;
            // c1cbd call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1cc2 movabs rax,0x680000003a
            rax = 0x0_0068_0000_003au64;
            // c1ccc mov QWORD PTR [rbp-0x1040],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efc0), (rax) as u64);
            // c1cd3 lea rsi,[rbp-0x1040]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efc0);
            // c1cda mov rdi,rbx
            rdi = rbx;
            // c1cdd call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1ce2 movabs rax,0x690000003a
            rax = 0x0_0069_0000_003au64;
            // c1cec mov QWORD PTR [rbp-0x1048],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efb8), (rax) as u64);
            // c1cf3 lea rsi,[rbp-0x1048]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efb8);
            // c1cfa mov rdi,rbx
            rdi = rbx;
            // c1cfd call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1d02 movabs rax,0x650000005c
            rax = 0x0_0065_0000_005cu64;
            // c1d0c mov QWORD PTR [rbp-0x1050],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efb0), (rax) as u64);
            // c1d13 lea rsi,[rbp-0x1050]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efb0);
            // c1d1a mov rdi,rbx
            rdi = rbx;
            // c1d1d call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1d22 movabs rax,0x660000005c
            rax = 0x0_0066_0000_005cu64;
            // c1d2c mov QWORD PTR [rbp-0x1058],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efa8), (rax) as u64);
            // c1d33 lea rsi,[rbp-0x1058]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efa8);
            // c1d3a mov rdi,rbx
            rdi = rbx;
            // c1d3d call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1d42 movabs rax,0x670000005c
            rax = 0x0_0067_0000_005cu64;
            // c1d4c mov QWORD PTR [rbp-0x1060],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_efa0), (rax) as u64);
            // c1d53 lea rsi,[rbp-0x1060]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_efa0);
            // c1d5a mov rdi,rbx
            rdi = rbx;
            // c1d5d call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1d62 movabs rax,0x680000005c
            rax = 0x0_0068_0000_005cu64;
            // c1d6c mov QWORD PTR [rbp-0x1068],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef98), (rax) as u64);
            // c1d73 lea rsi,[rbp-0x1068]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef98);
            // c1d7a mov rdi,rbx
            rdi = rbx;
            // c1d7d call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1d82 movabs rax,0x690000005c
            rax = 0x0_0069_0000_005cu64;
            // c1d8c mov QWORD PTR [rbp-0x1070],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ef90), (rax) as u64);
            // c1d93 lea rsi,[rbp-0x1070]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef90);
            // c1d9a mov rdi,rbx
            rdi = rbx;
            // c1d9d call 0xebdbc
            { let r = { let t = m.env.stub_va(0x589); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1da2 mov DWORD PTR [rbp-0x1074],0x16
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef8c), (0x16u64) as u32);
            // c1dac lea rbx,[rip+0x1068bd] # 0x1c8670
            rbx = m.env.data_va(904765);
            // c1db3 lea rsi,[rbp-0x1074]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef8c);
            // c1dba mov rdi,rbx
            rdi = rbx;
            // c1dbd call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1dc2 mov DWORD PTR [rax],0x1f
            m.w32(rax, (0x1fu64) as u32);
            // c1dc8 mov DWORD PTR [rbp-0x1078],0x1d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef88), (0x1du64) as u32);
            // c1dd2 lea rsi,[rbp-0x1078]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef88);
            // c1dd9 mov rdi,rbx
            rdi = rbx;
            // c1ddc call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1de1 mov DWORD PTR [rax],0x2d
            m.w32(rax, (0x2du64) as u32);
            // c1de7 mov DWORD PTR [rbp-0x107c],0x1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef84), (0x1u64) as u32);
            // c1df1 lea rsi,[rbp-0x107c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef84);
            // c1df8 mov rdi,rbx
            rdi = rbx;
            // c1dfb call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1e00 mov DWORD PTR [rax],0xa
            m.w32(rax, (0xau64) as u32);
            // c1e06 mov DWORD PTR [rbp-0x1080],0x1e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef80), (0x1eu64) as u32);
            // c1e10 lea rsi,[rbp-0x1080]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef80);
            // c1e17 mov rdi,rbx
            rdi = rbx;
            // c1e1a call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1e1f mov DWORD PTR [rax],0xc
            m.w32(rax, (0xcu64) as u32);
            // c1e25 mov DWORD PTR [rbp-0x1084],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef7c), (0x2u64) as u32);
            // c1e2f lea rsi,[rbp-0x1084]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef7c);
            // c1e36 mov rdi,rbx
            rdi = rbx;
            // c1e39 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1e3e mov DWORD PTR [rax],0xb
            m.w32(rax, (0xbu64) as u32);
            // c1e44 mov DWORD PTR [rbp-0x1088],0x1f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef78), (0x1fu64) as u32);
            // c1e4e lea rsi,[rbp-0x1088]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef78);
            // c1e55 mov rdi,rbx
            rdi = rbx;
            // c1e58 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1e5d mov DWORD PTR [rax],0xd
            m.w32(rax, (0xdu64) as u32);
            // c1e63 mov DWORD PTR [rbp-0x108c],0x30
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef74), (0x30u64) as u32);
            // c1e6d lea rsi,[rbp-0x108c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef74);
            // c1e74 mov rdi,rbx
            rdi = rbx;
            // c1e77 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1e7c mov DWORD PTR [rax],0xe
            m.w32(rax, (0xeu64) as u32);
            // c1e82 mov DWORD PTR [rbp-0x1090],0x32
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef70), (0x32u64) as u32);
            // c1e8c lea rsi,[rbp-0x1090]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef70);
            // c1e93 mov rdi,rbx
            rdi = rbx;
            // c1e96 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1e9b mov DWORD PTR [rax],0xe
            m.w32(rax, (0xeu64) as u32);
            // c1ea1 mov DWORD PTR [rbp-0x1094],0x31
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef6c), (0x31u64) as u32);
            // c1eab lea rsi,[rbp-0x1094]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef6c);
            // c1eb2 mov rdi,rbx
            rdi = rbx;
            // c1eb5 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1eba mov DWORD PTR [rax],0xf
            m.w32(rax, (0xfu64) as u32);
            // c1ec0 mov DWORD PTR [rbp-0x1098],0x33
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef68), (0x33u64) as u32);
            // c1eca lea rsi,[rbp-0x1098]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef68);
            // c1ed1 mov rdi,rbx
            rdi = rbx;
            // c1ed4 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1ed9 mov DWORD PTR [rax],0xf
            m.w32(rax, (0xfu64) as u32);
            // c1edf mov DWORD PTR [rbp-0x109c],0x45
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef64), (0x45u64) as u32);
            // c1ee9 lea rsi,[rbp-0x109c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef64);
            // c1ef0 mov rdi,rbx
            rdi = rbx;
            // c1ef3 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1ef8 mov DWORD PTR [rax],0x41
            m.w32(rax, (0x41u64) as u32);
            // c1efe mov DWORD PTR [rbp-0x10a0],0x47
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef60), (0x47u64) as u32);
            // c1f08 lea rsi,[rbp-0x10a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef60);
            // c1f0f mov rdi,rbx
            rdi = rbx;
            // c1f12 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1f17 mov DWORD PTR [rax],0x41
            m.w32(rax, (0x41u64) as u32);
            // c1f1d mov DWORD PTR [rbp-0x10a4],0x46
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef5c), (0x46u64) as u32);
            // c1f27 lea rsi,[rbp-0x10a4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef5c);
            // c1f2e mov rdi,rbx
            rdi = rbx;
            // c1f31 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1f36 mov DWORD PTR [rax],0x42
            m.w32(rax, (0x42u64) as u32);
            // c1f3c mov DWORD PTR [rbp-0x10a8],0x48
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef58), (0x48u64) as u32);
            // c1f46 lea rsi,[rbp-0x10a8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef58);
            // c1f4d mov rdi,rbx
            rdi = rbx;
            // c1f50 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1f55 mov DWORD PTR [rax],0x42
            m.w32(rax, (0x42u64) as u32);
            // c1f5b mov DWORD PTR [rbp-0x10ac],0x49
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef54), (0x49u64) as u32);
            // c1f65 lea rsi,[rbp-0x10ac]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef54);
            // c1f6c mov rdi,rbx
            rdi = rbx;
            // c1f6f call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1f74 mov DWORD PTR [rax],0x43
            m.w32(rax, (0x43u64) as u32);
            // c1f7a mov DWORD PTR [rbp-0x10b0],0x4b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef50), (0x4bu64) as u32);
            // c1f84 lea rsi,[rbp-0x10b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef50);
            // c1f8b mov rdi,rbx
            rdi = rbx;
            // c1f8e call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1f93 mov DWORD PTR [rax],0x43
            m.w32(rax, (0x43u64) as u32);
            // c1f99 mov DWORD PTR [rbp-0x10b4],0x4a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef4c), (0x4au64) as u32);
            // c1fa3 lea rsi,[rbp-0x10b4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef4c);
            // c1faa mov rdi,rbx
            rdi = rbx;
            // c1fad call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1fb2 mov DWORD PTR [rax],0x44
            m.w32(rax, (0x44u64) as u32);
            // c1fb8 mov DWORD PTR [rbp-0x10b8],0x4c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef48), (0x4cu64) as u32);
            // c1fc2 lea rsi,[rbp-0x10b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef48);
            // c1fc9 mov rdi,rbx
            rdi = rbx;
            // c1fcc call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1fd1 mov DWORD PTR [rax],0x44
            m.w32(rax, (0x44u64) as u32);
            // c1fd7 mov DWORD PTR [rbp-0x10bc],0x17
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef44), (0x17u64) as u32);
            // c1fe1 lea rsi,[rbp-0x10bc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef44);
            // c1fe8 mov rdi,rbx
            rdi = rbx;
            // c1feb call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c1ff0 mov DWORD PTR [rax],0x22
            m.w32(rax, (0x22u64) as u32);
            // c1ff6 mov DWORD PTR [rbp-0x10c0],0x41
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef40), (0x41u64) as u32);
            // c2000 lea rsi,[rbp-0x10c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef40);
            // c2007 mov rdi,rbx
            rdi = rbx;
            // c200a call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c200f mov DWORD PTR [rax],0x5c
            m.w32(rax, (0x5cu64) as u32);
            // c2015 mov DWORD PTR [rbp-0x10c4],0xc1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef3c), (0xc1u64) as u32);
            // c201f lea rsi,[rbp-0x10c4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef3c);
            // c2026 mov rdi,rbx
            rdi = rbx;
            // c2029 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c202e mov DWORD PTR [rax],0x5c
            m.w32(rax, (0x5cu64) as u32);
            // c2034 mov DWORD PTR [rbp-0x10c8],0x42
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef38), (0x42u64) as u32);
            // c203e lea rsi,[rbp-0x10c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef38);
            // c2045 mov rdi,rbx
            rdi = rbx;
            // c2048 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c204d mov DWORD PTR [rax],0x5c
            m.w32(rax, (0x5cu64) as u32);
            // c2053 mov DWORD PTR [rbp-0x10cc],0xc2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef34), (0xc2u64) as u32);
            // c205d lea rsi,[rbp-0x10cc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef34);
            // c2064 mov rdi,rbx
            rdi = rbx;
            // c2067 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c206c mov DWORD PTR [rax],0x5c
            m.w32(rax, (0x5cu64) as u32);
            // c2072 mov DWORD PTR [rbp-0x10d0],0x43
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef30), (0x43u64) as u32);
            // c207c lea rsi,[rbp-0x10d0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef30);
            // c2083 mov rdi,rbx
            rdi = rbx;
            // c2086 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c208b mov DWORD PTR [rax],0x5c
            m.w32(rax, (0x5cu64) as u32);
            // c2091 mov DWORD PTR [rbp-0x10d4],0xc3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef2c), (0xc3u64) as u32);
            // c209b lea rsi,[rbp-0x10d4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef2c);
            // c20a2 mov rdi,rbx
            rdi = rbx;
            // c20a5 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c20aa mov DWORD PTR [rax],0x5c
            m.w32(rax, (0x5cu64) as u32);
            // c20b0 mov DWORD PTR [rbp-0x10d8],0x97
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef28), (0x97u64) as u32);
            // c20ba lea rsi,[rbp-0x10d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef28);
            // c20c1 mov rdi,rbx
            rdi = rbx;
            // c20c4 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c20c9 mov DWORD PTR [rax],0x5c
            m.w32(rax, (0x5cu64) as u32);
            // c20cf mov DWORD PTR [rbp-0x10dc],0xe8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef24), (0xe8u64) as u32);
            // c20d9 lea rsi,[rbp-0x10dc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef24);
            // c20e0 mov rdi,rbx
            rdi = rbx;
            // c20e3 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c20e8 mov DWORD PTR [rax],0x5e
            m.w32(rax, (0x5eu64) as u32);
            // c20ee mov DWORD PTR [rbp-0x10e0],0x68
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef20), (0x68u64) as u32);
            // c20f8 lea rsi,[rbp-0x10e0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef20);
            // c20ff mov rdi,rbx
            rdi = rbx;
            // c2102 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2107 mov DWORD PTR [rax],0x5e
            m.w32(rax, (0x5eu64) as u32);
            // c210d mov DWORD PTR [rbp-0x10e4],0xe9
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef1c), (0xe9u64) as u32);
            // c2117 lea rsi,[rbp-0x10e4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef1c);
            // c211e mov rdi,rbx
            rdi = rbx;
            // c2121 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2126 mov DWORD PTR [rax],0x5e
            m.w32(rax, (0x5eu64) as u32);
            // c212c mov DWORD PTR [rbp-0x10e8],0x69
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef18), (0x69u64) as u32);
            // c2136 lea rsi,[rbp-0x10e8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef18);
            // c213d mov rdi,rbx
            rdi = rbx;
            // c2140 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2145 mov DWORD PTR [rax],0x5e
            m.w32(rax, (0x5eu64) as u32);
            // c214b mov DWORD PTR [rbp-0x10ec],0xea
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef14), (0xeau64) as u32);
            // c2155 lea rsi,[rbp-0x10ec]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef14);
            // c215c mov rdi,rbx
            rdi = rbx;
            // c215f call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2164 mov DWORD PTR [rax],0x5f
            m.w32(rax, (0x5fu64) as u32);
            // c216a mov DWORD PTR [rbp-0x10f0],0x6a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef10), (0x6au64) as u32);
            // c2174 lea rsi,[rbp-0x10f0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef10);
            // c217b mov rdi,rbx
            rdi = rbx;
            // c217e call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2183 mov DWORD PTR [rax],0x5f
            m.w32(rax, (0x5fu64) as u32);
            // c2189 mov DWORD PTR [rbp-0x10f4],0xeb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef0c), (0xebu64) as u32);
            // c2193 lea rsi,[rbp-0x10f4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef0c);
            // c219a mov rdi,rbx
            rdi = rbx;
            // c219d call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c21a2 mov DWORD PTR [rax],0x5f
            m.w32(rax, (0x5fu64) as u32);
            // c21a8 mov DWORD PTR [rbp-0x10f8],0x6b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef08), (0x6bu64) as u32);
            // c21b2 lea rsi,[rbp-0x10f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef08);
            // c21b9 mov rdi,rbx
            rdi = rbx;
            // c21bc call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c21c1 mov DWORD PTR [rax],0x5f
            m.w32(rax, (0x5fu64) as u32);
            // c21c7 mov DWORD PTR [rbp-0x10fc],0xec
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef04), (0xecu64) as u32);
            // c21d1 lea rsi,[rbp-0x10fc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef04);
            // c21d8 mov rdi,rbx
            rdi = rbx;
            // c21db call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c21e0 mov DWORD PTR [rax],0x60
            m.w32(rax, (0x60u64) as u32);
            // c21e6 mov DWORD PTR [rbp-0x1100],0x6c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ef00), (0x6cu64) as u32);
            // c21f0 lea rsi,[rbp-0x1100]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ef00);
            // c21f7 mov rdi,rbx
            rdi = rbx;
            // c21fa call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c21ff mov DWORD PTR [rax],0x60
            m.w32(rax, (0x60u64) as u32);
            // c2205 mov DWORD PTR [rbp-0x1104],0xed
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eefc), (0xedu64) as u32);
            // c220f lea rsi,[rbp-0x1104]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eefc);
            // c2216 mov rdi,rbx
            rdi = rbx;
            // c2219 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c221e mov DWORD PTR [rax],0x60
            m.w32(rax, (0x60u64) as u32);
            // c2224 mov DWORD PTR [rbp-0x1108],0x6d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eef8), (0x6du64) as u32);
            // c222e lea rsi,[rbp-0x1108]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eef8);
            // c2235 mov rdi,rbx
            rdi = rbx;
            // c2238 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c223d mov DWORD PTR [rax],0x60
            m.w32(rax, (0x60u64) as u32);
            // c2243 mov DWORD PTR [rbp-0x110c],0xee
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eef4), (0xeeu64) as u32);
            // c224d lea rsi,[rbp-0x110c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eef4);
            // c2254 mov rdi,rbx
            rdi = rbx;
            // c2257 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c225c mov DWORD PTR [rax],0x61
            m.w32(rax, (0x61u64) as u32);
            // c2262 mov DWORD PTR [rbp-0x1110],0x6e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eef0), (0x6eu64) as u32);
            // c226c lea rsi,[rbp-0x1110]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eef0);
            // c2273 mov rdi,rbx
            rdi = rbx;
            // c2276 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c227b mov DWORD PTR [rax],0x61
            m.w32(rax, (0x61u64) as u32);
            // c2281 mov DWORD PTR [rbp-0x1114],0xef
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eeec), (0xefu64) as u32);
            // c228b lea rsi,[rbp-0x1114]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eeec);
            // c2292 mov rdi,rbx
            rdi = rbx;
            // c2295 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c229a mov DWORD PTR [rax],0x61
            m.w32(rax, (0x61u64) as u32);
            // c22a0 mov DWORD PTR [rbp-0x1118],0x6f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eee8), (0x6fu64) as u32);
            // c22aa lea rsi,[rbp-0x1118]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eee8);
            // c22b1 mov rdi,rbx
            rdi = rbx;
            // c22b4 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c22b9 mov DWORD PTR [rax],0x61
            m.w32(rax, (0x61u64) as u32);
            // c22bf mov DWORD PTR [rbp-0x111c],0x44
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eee4), (0x44u64) as u32);
            // c22c9 lea rsi,[rbp-0x111c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eee4);
            // c22d0 mov rdi,rbx
            rdi = rbx;
            // c22d3 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c22d8 mov DWORD PTR [rax],0x40
            m.w32(rax, (0x40u64) as u32);
            // c22de mov DWORD PTR [rbp-0x1120],0xc4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eee0), (0xc4u64) as u32);
            // c22e8 lea rsi,[rbp-0x1120]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eee0);
            // c22ef mov rdi,rbx
            rdi = rbx;
            // c22f2 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c22f7 mov DWORD PTR [rax],0x40
            m.w32(rax, (0x40u64) as u32);
            // c22fd mov DWORD PTR [rbp-0x1124],0xe
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eedc), (0xeu64) as u32);
            // c2307 lea rsi,[rbp-0x1124]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eedc);
            // c230e mov rdi,rbx
            rdi = rbx;
            // c2311 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2316 mov DWORD PTR [rax],0x4
            m.w32(rax, (0x4u64) as u32);
            // c231c mov DWORD PTR [rbp-0x1128],0x85
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eed8), (0x85u64) as u32);
            // c2326 lea rsi,[rbp-0x1128]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eed8);
            // c232d mov rdi,rbx
            rdi = rbx;
            // c2330 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2335 mov DWORD PTR [rax],0x4
            m.w32(rax, (0x4u64) as u32);
            // c233b mov DWORD PTR [rbp-0x112c],0x5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eed4), (0x5u64) as u32);
            // c2345 lea rsi,[rbp-0x112c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eed4);
            // c234c mov rdi,rbx
            rdi = rbx;
            // c234f call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2354 mov DWORD PTR [rax],0x4
            m.w32(rax, (0x4u64) as u32);
            // c235a mov DWORD PTR [rbp-0x1130],0x11
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eed0), (0x11u64) as u32);
            // c2364 lea rsi,[rbp-0x1130]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eed0);
            // c236b mov rdi,rbx
            rdi = rbx;
            // c236e call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2373 mov DWORD PTR [rax],0x5
            m.w32(rax, (0x5u64) as u32);
            // c2379 mov DWORD PTR [rbp-0x1134],0x90
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eecc), (0x90u64) as u32);
            // c2383 lea rsi,[rbp-0x1134]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eecc);
            // c238a mov rdi,rbx
            rdi = rbx;
            // c238d call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2392 mov DWORD PTR [rax],0x5
            m.w32(rax, (0x5u64) as u32);
            // c2398 mov DWORD PTR [rbp-0x1138],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eec8), (0x10u64) as u32);
            // c23a2 lea rsi,[rbp-0x1138]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eec8);
            // c23a9 mov rdi,rbx
            rdi = rbx;
            // c23ac call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c23b1 mov DWORD PTR [rax],0x5
            m.w32(rax, (0x5u64) as u32);
            // c23b7 mov DWORD PTR [rbp-0x113c],0x3b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eec4), (0x3bu64) as u32);
            // c23c1 lea rsi,[rbp-0x113c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eec4);
            // c23c8 mov rdi,rbx
            rdi = rbx;
            // c23cb call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c23d0 mov DWORD PTR [rax],0x38
            m.w32(rax, (0x38u64) as u32);
            // c23d6 mov DWORD PTR [rbp-0x1140],0xba
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eec0), (0xbau64) as u32);
            // c23e0 lea rsi,[rbp-0x1140]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eec0);
            // c23e7 mov rdi,rbx
            rdi = rbx;
            // c23ea call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c23ef mov DWORD PTR [rax],0x38
            m.w32(rax, (0x38u64) as u32);
            // c23f5 mov DWORD PTR [rbp-0x1144],0x3a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eebc), (0x3au64) as u32);
            // c23ff lea rsi,[rbp-0x1144]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eebc);
            // c2406 mov rdi,rbx
            rdi = rbx;
            // c2409 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c240e mov DWORD PTR [rax],0x38
            m.w32(rax, (0x38u64) as u32);
            // c2414 mov DWORD PTR [rbp-0x1148],0x3d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eeb8), (0x3du64) as u32);
            // c241e lea rsi,[rbp-0x1148]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eeb8);
            // c2425 mov rdi,rbx
            rdi = rbx;
            // c2428 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c242d mov DWORD PTR [rax],0x39
            m.w32(rax, (0x39u64) as u32);
            // c2433 mov DWORD PTR [rbp-0x114c],0xbc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eeb4), (0xbcu64) as u32);
            // c243d lea rsi,[rbp-0x114c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eeb4);
            // c2444 mov rdi,rbx
            rdi = rbx;
            // c2447 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c244c mov DWORD PTR [rax],0x39
            m.w32(rax, (0x39u64) as u32);
            // c2452 mov DWORD PTR [rbp-0x1150],0x3c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eeb0), (0x3cu64) as u32);
            // c245c lea rsi,[rbp-0x1150]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eeb0);
            // c2463 mov rdi,rbx
            rdi = rbx;
            // c2466 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c246b mov DWORD PTR [rax],0x39
            m.w32(rax, (0x39u64) as u32);
            // c2471 mov DWORD PTR [rbp-0x1154],0x2d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eeac), (0x2du64) as u32);
            // c247b lea rsi,[rbp-0x1154]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eeac);
            // c2482 mov rdi,rbx
            rdi = rbx;
            // c2485 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c248a mov DWORD PTR [rax],0x3d
            m.w32(rax, (0x3du64) as u32);
            // c2490 mov DWORD PTR [rbp-0x1158],0xac
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eea8), (0xacu64) as u32);
            // c249a lea rsi,[rbp-0x1158]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eea8);
            // c24a1 mov rdi,rbx
            rdi = rbx;
            // c24a4 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c24a9 mov DWORD PTR [rax],0x3d
            m.w32(rax, (0x3du64) as u32);
            // c24af mov DWORD PTR [rbp-0x115c],0x2c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eea4), (0x2cu64) as u32);
            // c24b9 lea rsi,[rbp-0x115c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eea4);
            // c24c0 mov rdi,rbx
            rdi = rbx;
            // c24c3 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c24c8 mov DWORD PTR [rax],0x3d
            m.w32(rax, (0x3du64) as u32);
            // c24ce mov DWORD PTR [rbp-0x1160],0x5a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eea0), (0x5au64) as u32);
            // c24d8 lea rsi,[rbp-0x1160]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eea0);
            // c24df mov rdi,rbx
            rdi = rbx;
            // c24e2 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c24e7 mov DWORD PTR [rax],0x50
            m.w32(rax, (0x50u64) as u32);
            // c24ed mov DWORD PTR [rbp-0x1164],0xd9
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee9c), (0xd9u64) as u32);
            // c24f7 lea rsi,[rbp-0x1164]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee9c);
            // c24fe mov rdi,rbx
            rdi = rbx;
            // c2501 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2506 mov DWORD PTR [rax],0x50
            m.w32(rax, (0x50u64) as u32);
            // c250c mov DWORD PTR [rbp-0x1168],0x59
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee98), (0x59u64) as u32);
            // c2516 lea rsi,[rbp-0x1168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee98);
            // c251d mov rdi,rbx
            rdi = rbx;
            // c2520 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2525 mov DWORD PTR [rax],0x50
            m.w32(rax, (0x50u64) as u32);
            // c252b mov DWORD PTR [rbp-0x116c],0x5c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee94), (0x5cu64) as u32);
            // c2535 lea rsi,[rbp-0x116c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee94);
            // c253c mov rdi,rbx
            rdi = rbx;
            // c253f call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2544 mov DWORD PTR [rax],0x51
            m.w32(rax, (0x51u64) as u32);
            // c254a mov DWORD PTR [rbp-0x1170],0xdb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee90), (0xdbu64) as u32);
            // c2554 lea rsi,[rbp-0x1170]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee90);
            // c255b mov rdi,rbx
            rdi = rbx;
            // c255e call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2563 mov DWORD PTR [rax],0x51
            m.w32(rax, (0x51u64) as u32);
            // c2569 mov DWORD PTR [rbp-0x1174],0x5b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee8c), (0x5bu64) as u32);
            // c2573 lea rsi,[rbp-0x1174]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee8c);
            // c257a mov rdi,rbx
            rdi = rbx;
            // c257d call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2582 mov DWORD PTR [rax],0x51
            m.w32(rax, (0x51u64) as u32);
            // c2588 mov DWORD PTR [rbp-0x1178],0x5e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee88), (0x5eu64) as u32);
            // c2592 lea rsi,[rbp-0x1178]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee88);
            // c2599 mov rdi,rbx
            rdi = rbx;
            // c259c call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c25a1 mov DWORD PTR [rax],0x52
            m.w32(rax, (0x52u64) as u32);
            // c25a7 mov DWORD PTR [rbp-0x117c],0xdd
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee84), (0xddu64) as u32);
            // c25b1 lea rsi,[rbp-0x117c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee84);
            // c25b8 mov rdi,rbx
            rdi = rbx;
            // c25bb call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c25c0 mov DWORD PTR [rax],0x52
            m.w32(rax, (0x52u64) as u32);
            // c25c6 mov DWORD PTR [rbp-0x1180],0x5d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee80), (0x5du64) as u32);
            // c25d0 lea rsi,[rbp-0x1180]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee80);
            // c25d7 mov rdi,rbx
            rdi = rbx;
            // c25da call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c25df mov DWORD PTR [rax],0x52
            m.w32(rax, (0x52u64) as u32);
            // c25e5 mov DWORD PTR [rbp-0x1184],0x83
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee7c), (0x83u64) as u32);
            // c25ef lea rsi,[rbp-0x1184]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee7c);
            // c25f6 mov rdi,rbx
            rdi = rbx;
            // c25f9 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c25fe mov DWORD PTR [rax],0x19
            m.w32(rax, (0x19u64) as u32);
            // c2604 mov DWORD PTR [rbp-0x1188],0xa8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee78), (0xa8u64) as u32);
            // c260e lea rsi,[rbp-0x1188]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee78);
            // c2615 mov rdi,rbx
            rdi = rbx;
            // c2618 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c261d mov DWORD PTR [rax],0x1a
            m.w32(rax, (0x1au64) as u32);
            // c2623 mov DWORD PTR [rbp-0x118c],0xb4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee74), (0xb4u64) as u32);
            // c262d lea rsi,[rbp-0x118c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee74);
            // c2634 mov rdi,rbx
            rdi = rbx;
            // c2637 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c263c mov DWORD PTR [rax],0x34
            m.w32(rax, (0x34u64) as u32);
            // c2642 mov DWORD PTR [rbp-0x1190],0xb5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee70), (0xb5u64) as u32);
            // c264c lea rsi,[rbp-0x1190]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee70);
            // c2653 mov rdi,rbx
            rdi = rbx;
            // c2656 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c265b mov DWORD PTR [rax],0x35
            m.w32(rax, (0x35u64) as u32);
            // c2661 mov DWORD PTR [rbp-0x1194],0xcd
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee6c), (0xcdu64) as u32);
            // c266b lea rsi,[rbp-0x1194]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee6c);
            // c2672 mov rdi,rbx
            rdi = rbx;
            // c2675 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c267a mov DWORD PTR [rax],0x49
            m.w32(rax, (0x49u64) as u32);
            // c2680 mov DWORD PTR [rbp-0x1198],0xce
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee68), (0xceu64) as u32);
            // c268a lea rsi,[rbp-0x1198]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee68);
            // c2691 mov rdi,rbx
            rdi = rbx;
            // c2694 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2699 mov DWORD PTR [rax],0x4a
            m.w32(rax, (0x4au64) as u32);
            // c269f mov DWORD PTR [rbp-0x119c],0xcf
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee64), (0xcfu64) as u32);
            // c26a9 lea rsi,[rbp-0x119c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee64);
            // c26b0 mov rdi,rbx
            rdi = rbx;
            // c26b3 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c26b8 mov DWORD PTR [rax],0x4b
            m.w32(rax, (0x4bu64) as u32);
            // c26be mov DWORD PTR [rbp-0x11a0],0xd0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee60), (0xd0u64) as u32);
            // c26c8 lea rsi,[rbp-0x11a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee60);
            // c26cf mov rdi,rbx
            rdi = rbx;
            // c26d2 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c26d7 mov DWORD PTR [rax],0x4c
            m.w32(rax, (0x4cu64) as u32);
            // c26dd mov DWORD PTR [rbp-0x11a4],0xb
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee5c), (0xbu64) as u32);
            // c26e7 lea rsi,[rbp-0x11a4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee5c);
            // c26ee mov rdi,rbx
            rdi = rbx;
            // c26f1 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c26f6 mov DWORD PTR [rax],0x1d
            m.w32(rax, (0x1du64) as u32);
            // c26fc mov DWORD PTR [rbp-0x11a8],0x26
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee58), (0x26u64) as u32);
            // c2706 lea rsi,[rbp-0x11a8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee58);
            // c270d mov rdi,rbx
            rdi = rbx;
            // c2710 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2715 mov DWORD PTR [rax],0x1d
            m.w32(rax, (0x1du64) as u32);
            // c271b mov DWORD PTR [rbp-0x11ac],0x1c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee54), (0x1cu64) as u32);
            // c2725 lea rsi,[rbp-0x11ac]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee54);
            // c272c mov rdi,rbx
            rdi = rbx;
            // c272f call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2734 mov DWORD PTR [rax],0x1e
            m.w32(rax, (0x1eu64) as u32);
            // c273a mov DWORD PTR [rbp-0x11b0],0x27
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee50), (0x27u64) as u32);
            // c2744 lea rsi,[rbp-0x11b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee50);
            // c274b mov rdi,rbx
            rdi = rbx;
            // c274e call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2753 mov DWORD PTR [rax],0x1e
            m.w32(rax, (0x1eu64) as u32);
            // c2759 mov DWORD PTR [rbp-0x11b4],0x36
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee4c), (0x36u64) as u32);
            // c2763 lea rsi,[rbp-0x11b4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee4c);
            // c276a mov rdi,rbx
            rdi = rbx;
            // c276d call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2772 mov DWORD PTR [rax],0x36
            m.w32(rax, (0x36u64) as u32);
            // c2778 mov DWORD PTR [rbp-0x11b8],0x37
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee48), (0x37u64) as u32);
            // c2782 lea rsi,[rbp-0x11b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee48);
            // c2789 mov rdi,rbx
            rdi = rbx;
            // c278c call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2791 mov DWORD PTR [rax],0x36
            m.w32(rax, (0x36u64) as u32);
            // c2797 mov DWORD PTR [rbp-0x11bc],0x38
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee44), (0x38u64) as u32);
            // c27a1 lea rsi,[rbp-0x11bc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee44);
            // c27a8 mov rdi,rbx
            rdi = rbx;
            // c27ab call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c27b0 mov DWORD PTR [rax],0x37
            m.w32(rax, (0x37u64) as u32);
            // c27b6 mov DWORD PTR [rbp-0x11c0],0x39
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee40), (0x39u64) as u32);
            // c27c0 lea rsi,[rbp-0x11c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee40);
            // c27c7 mov rdi,rbx
            rdi = rbx;
            // c27ca call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c27cf mov DWORD PTR [rax],0x37
            m.w32(rax, (0x37u64) as u32);
            // c27d5 mov DWORD PTR [rbp-0x11c4],0x3e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee3c), (0x3eu64) as u32);
            // c27df lea rsi,[rbp-0x11c4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee3c);
            // c27e6 mov rdi,rbx
            rdi = rbx;
            // c27e9 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c27ee mov DWORD PTR [rax],0x3f
            m.w32(rax, (0x3fu64) as u32);
            // c27f4 mov DWORD PTR [rbp-0x11c8],0x3f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee38), (0x3fu64) as u32);
            // c27fe lea rsi,[rbp-0x11c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee38);
            // c2805 mov rdi,rbx
            rdi = rbx;
            // c2808 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c280d mov DWORD PTR [rax],0x3f
            m.w32(rax, (0x3fu64) as u32);
            // c2813 mov DWORD PTR [rbp-0x11cc],0x62
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee34), (0x62u64) as u32);
            // c281d lea rsi,[rbp-0x11cc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee34);
            // c2824 mov rdi,rbx
            rdi = rbx;
            // c2827 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c282c mov DWORD PTR [rax],0x4d
            m.w32(rax, (0x4du64) as u32);
            // c2832 mov DWORD PTR [rbp-0x11d0],0x63
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee30), (0x63u64) as u32);
            // c283c lea rsi,[rbp-0x11d0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee30);
            // c2843 mov rdi,rbx
            rdi = rbx;
            // c2846 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c284b mov DWORD PTR [rax],0x4d
            m.w32(rax, (0x4du64) as u32);
            // c2851 mov DWORD PTR [rbp-0x11d4],0x64
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee2c), (0x64u64) as u32);
            // c285b lea rsi,[rbp-0x11d4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee2c);
            // c2862 mov rdi,rbx
            rdi = rbx;
            // c2865 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c286a mov DWORD PTR [rax],0x4e
            m.w32(rax, (0x4eu64) as u32);
            // c2870 mov DWORD PTR [rbp-0x11d8],0x65
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee28), (0x65u64) as u32);
            // c287a lea rsi,[rbp-0x11d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee28);
            // c2881 mov rdi,rbx
            rdi = rbx;
            // c2884 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2889 mov DWORD PTR [rax],0x4e
            m.w32(rax, (0x4eu64) as u32);
            // c288f mov DWORD PTR [rbp-0x11dc],0x66
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee24), (0x66u64) as u32);
            // c2899 lea rsi,[rbp-0x11dc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee24);
            // c28a0 mov rdi,rbx
            rdi = rbx;
            // c28a3 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c28a8 mov DWORD PTR [rax],0x4f
            m.w32(rax, (0x4fu64) as u32);
            // c28ae mov DWORD PTR [rbp-0x11e0],0x67
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee20), (0x67u64) as u32);
            // c28b8 lea rsi,[rbp-0x11e0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee20);
            // c28bf mov rdi,rbx
            rdi = rbx;
            // c28c2 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c28c7 mov DWORD PTR [rax],0x4f
            m.w32(rax, (0x4fu64) as u32);
            // c28cd mov DWORD PTR [rbp-0x11e4],0xf0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee1c), (0xf0u64) as u32);
            // c28d7 lea rsi,[rbp-0x11e4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee1c);
            // c28de mov rdi,rbx
            rdi = rbx;
            // c28e1 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c28e6 mov DWORD PTR [rax],0x0
            m.w32(rax, (0x0u64) as u32);
            // c28ec mov DWORD PTR [rbp-0x11e8],0x70
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee18), (0x70u64) as u32);
            // c28f6 lea rsi,[rbp-0x11e8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee18);
            // c28fd mov rdi,rbx
            rdi = rbx;
            // c2900 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2905 mov DWORD PTR [rax],0x0
            m.w32(rax, (0x0u64) as u32);
            // c290b mov DWORD PTR [rbp-0x11ec],0x88
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee14), (0x88u64) as u32);
            // c2915 lea rsi,[rbp-0x11ec]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee14);
            // c291c mov rdi,rbx
            rdi = rbx;
            // c291f call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2924 mov DWORD PTR [rax],0x0
            m.w32(rax, (0x0u64) as u32);
            // c292a mov DWORD PTR [rbp-0x11f0],0x8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee10), (0x8u64) as u32);
            // c2934 lea rsi,[rbp-0x11f0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee10);
            // c293b mov rdi,rbx
            rdi = rbx;
            // c293e call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2943 mov DWORD PTR [rax],0x0
            m.w32(rax, (0x0u64) as u32);
            // c2949 mov DWORD PTR [rbp-0x11f4],0xf1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee0c), (0xf1u64) as u32);
            // c2953 lea rsi,[rbp-0x11f4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee0c);
            // c295a mov rdi,rbx
            rdi = rbx;
            // c295d call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2962 mov DWORD PTR [rax],0x1
            m.w32(rax, (0x1u64) as u32);
            // c2968 mov DWORD PTR [rbp-0x11f8],0x71
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee08), (0x71u64) as u32);
            // c2972 lea rsi,[rbp-0x11f8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee08);
            // c2979 mov rdi,rbx
            rdi = rbx;
            // c297c call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2981 mov DWORD PTR [rax],0x1
            m.w32(rax, (0x1u64) as u32);
            // c2987 mov DWORD PTR [rbp-0x11fc],0x8f
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee04), (0x8fu64) as u32);
            // c2991 lea rsi,[rbp-0x11fc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee04);
            // c2998 mov rdi,rbx
            rdi = rbx;
            // c299b call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c29a0 mov DWORD PTR [rax],0x1
            m.w32(rax, (0x1u64) as u32);
            // c29a6 mov DWORD PTR [rbp-0x1200],0xf
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ee00), (0xfu64) as u32);
            // c29b0 lea rsi,[rbp-0x1200]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ee00);
            // c29b7 mov rdi,rbx
            rdi = rbx;
            // c29ba call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c29bf mov DWORD PTR [rax],0x1
            m.w32(rax, (0x1u64) as u32);
            // c29c5 mov DWORD PTR [rbp-0x1204],0xf2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edfc), (0xf2u64) as u32);
            // c29cf lea rsi,[rbp-0x1204]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edfc);
            // c29d6 mov rdi,rbx
            rdi = rbx;
            // c29d9 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c29de mov DWORD PTR [rax],0x2
            m.w32(rax, (0x2u64) as u32);
            // c29e4 mov DWORD PTR [rbp-0x1208],0x72
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edf8), (0x72u64) as u32);
            // c29ee lea rsi,[rbp-0x1208]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edf8);
            // c29f5 mov rdi,rbx
            rdi = rbx;
            // c29f8 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c29fd mov DWORD PTR [rax],0x2
            m.w32(rax, (0x2u64) as u32);
            // c2a03 mov DWORD PTR [rbp-0x120c],0xa4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edf4), (0xa4u64) as u32);
            // c2a0d lea rsi,[rbp-0x120c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edf4);
            // c2a14 mov rdi,rbx
            rdi = rbx;
            // c2a17 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2a1c mov DWORD PTR [rax],0x2
            m.w32(rax, (0x2u64) as u32);
            // c2a22 mov DWORD PTR [rbp-0x1210],0x24
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edf0), (0x24u64) as u32);
            // c2a2c lea rsi,[rbp-0x1210]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edf0);
            // c2a33 mov rdi,rbx
            rdi = rbx;
            // c2a36 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2a3b mov DWORD PTR [rax],0x2
            m.w32(rax, (0x2u64) as u32);
            // c2a41 mov DWORD PTR [rbp-0x1214],0xf3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edec), (0xf3u64) as u32);
            // c2a4b lea rsi,[rbp-0x1214]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edec);
            // c2a52 mov rdi,rbx
            rdi = rbx;
            // c2a55 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2a5a mov DWORD PTR [rax],0x3
            m.w32(rax, (0x3u64) as u32);
            // c2a60 mov DWORD PTR [rbp-0x1218],0x73
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ede8), (0x73u64) as u32);
            // c2a6a lea rsi,[rbp-0x1218]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ede8);
            // c2a71 mov rdi,rbx
            rdi = rbx;
            // c2a74 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2a79 mov DWORD PTR [rax],0x3
            m.w32(rax, (0x3u64) as u32);
            // c2a7f mov DWORD PTR [rbp-0x121c],0xa5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ede4), (0xa5u64) as u32);
            // c2a89 lea rsi,[rbp-0x121c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ede4);
            // c2a90 mov rdi,rbx
            rdi = rbx;
            // c2a93 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2a98 mov DWORD PTR [rax],0x3
            m.w32(rax, (0x3u64) as u32);
            // c2a9e mov DWORD PTR [rbp-0x1220],0x25
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ede0), (0x25u64) as u32);
            // c2aa8 lea rsi,[rbp-0x1220]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ede0);
            // c2aaf mov rdi,rbx
            rdi = rbx;
            // c2ab2 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2ab7 mov DWORD PTR [rax],0x3
            m.w32(rax, (0x3u64) as u32);
            // c2abd mov DWORD PTR [rbp-0x1224],0xf4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eddc), (0xf4u64) as u32);
            // c2ac7 lea rsi,[rbp-0x1224]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eddc);
            // c2ace mov rdi,rbx
            rdi = rbx;
            // c2ad1 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2ad6 mov DWORD PTR [rax],0x58
            m.w32(rax, (0x58u64) as u32);
            // c2adc mov DWORD PTR [rbp-0x1228],0x74
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edd8), (0x74u64) as u32);
            // c2ae6 lea rsi,[rbp-0x1228]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edd8);
            // c2aed mov rdi,rbx
            rdi = rbx;
            // c2af0 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2af5 mov DWORD PTR [rax],0x58
            m.w32(rax, (0x58u64) as u32);
            // c2afb mov DWORD PTR [rbp-0x122c],0xd1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edd4), (0xd1u64) as u32);
            // c2b05 lea rsi,[rbp-0x122c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edd4);
            // c2b0c mov rdi,rbx
            rdi = rbx;
            // c2b0f call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2b14 mov DWORD PTR [rax],0x58
            m.w32(rax, (0x58u64) as u32);
            // c2b1a mov DWORD PTR [rbp-0x1230],0x51
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edd0), (0x51u64) as u32);
            // c2b24 lea rsi,[rbp-0x1230]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edd0);
            // c2b2b mov rdi,rbx
            rdi = rbx;
            // c2b2e call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2b33 mov DWORD PTR [rax],0x58
            m.w32(rax, (0x58u64) as u32);
            // c2b39 mov DWORD PTR [rbp-0x1234],0xf5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edcc), (0xf5u64) as u32);
            // c2b43 lea rsi,[rbp-0x1234]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edcc);
            // c2b4a mov rdi,rbx
            rdi = rbx;
            // c2b4d call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2b52 mov DWORD PTR [rax],0x59
            m.w32(rax, (0x59u64) as u32);
            // c2b58 mov DWORD PTR [rbp-0x1238],0x75
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edc8), (0x75u64) as u32);
            // c2b62 lea rsi,[rbp-0x1238]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edc8);
            // c2b69 mov rdi,rbx
            rdi = rbx;
            // c2b6c call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2b71 mov DWORD PTR [rax],0x59
            m.w32(rax, (0x59u64) as u32);
            // c2b77 mov DWORD PTR [rbp-0x123c],0xd2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edc4), (0xd2u64) as u32);
            // c2b81 lea rsi,[rbp-0x123c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edc4);
            // c2b88 mov rdi,rbx
            rdi = rbx;
            // c2b8b call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2b90 mov DWORD PTR [rax],0x59
            m.w32(rax, (0x59u64) as u32);
            // c2b96 mov DWORD PTR [rbp-0x1240],0x52
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edc0), (0x52u64) as u32);
            // c2ba0 lea rsi,[rbp-0x1240]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edc0);
            // c2ba7 mov rdi,rbx
            rdi = rbx;
            // c2baa call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2baf mov DWORD PTR [rax],0x59
            m.w32(rax, (0x59u64) as u32);
            // c2bb5 mov DWORD PTR [rbp-0x1244],0xf6
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edbc), (0xf6u64) as u32);
            // c2bbf lea rsi,[rbp-0x1244]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edbc);
            // c2bc6 mov rdi,rbx
            rdi = rbx;
            // c2bc9 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2bce mov DWORD PTR [rax],0x5a
            m.w32(rax, (0x5au64) as u32);
            // c2bd4 mov DWORD PTR [rbp-0x1248],0x76
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edb8), (0x76u64) as u32);
            // c2bde lea rsi,[rbp-0x1248]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edb8);
            // c2be5 mov rdi,rbx
            rdi = rbx;
            // c2be8 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2bed mov DWORD PTR [rax],0x5a
            m.w32(rax, (0x5au64) as u32);
            // c2bf3 mov DWORD PTR [rbp-0x124c],0xd3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edb4), (0xd3u64) as u32);
            // c2bfd lea rsi,[rbp-0x124c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edb4);
            // c2c04 mov rdi,rbx
            rdi = rbx;
            // c2c07 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2c0c mov DWORD PTR [rax],0x5a
            m.w32(rax, (0x5au64) as u32);
            // c2c12 mov DWORD PTR [rbp-0x1250],0x53
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edb0), (0x53u64) as u32);
            // c2c1c lea rsi,[rbp-0x1250]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edb0);
            // c2c23 mov rdi,rbx
            rdi = rbx;
            // c2c26 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2c2b mov DWORD PTR [rax],0x5a
            m.w32(rax, (0x5au64) as u32);
            // c2c31 mov DWORD PTR [rbp-0x1254],0xf7
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_edac), (0xf7u64) as u32);
            // c2c3b lea rsi,[rbp-0x1254]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_edac);
            // c2c42 mov rdi,rbx
            rdi = rbx;
            // c2c45 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2c4a mov DWORD PTR [rax],0x5b
            m.w32(rax, (0x5bu64) as u32);
            // c2c50 mov DWORD PTR [rbp-0x1258],0x77
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eda8), (0x77u64) as u32);
            // c2c5a lea rsi,[rbp-0x1258]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eda8);
            // c2c61 mov rdi,rbx
            rdi = rbx;
            // c2c64 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2c69 mov DWORD PTR [rax],0x5b
            m.w32(rax, (0x5bu64) as u32);
            // c2c6f mov DWORD PTR [rbp-0x125c],0xd4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eda4), (0xd4u64) as u32);
            // c2c79 lea rsi,[rbp-0x125c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eda4);
            // c2c80 mov rdi,rbx
            rdi = rbx;
            // c2c83 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2c88 mov DWORD PTR [rax],0x5b
            m.w32(rax, (0x5bu64) as u32);
            // c2c8e mov DWORD PTR [rbp-0x1260],0x54
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_eda0), (0x54u64) as u32);
            // c2c98 lea rsi,[rbp-0x1260]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_eda0);
            // c2c9f mov rdi,rbx
            rdi = rbx;
            // c2ca2 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2ca7 mov DWORD PTR [rax],0x5b
            m.w32(rax, (0x5bu64) as u32);
            // c2cad mov DWORD PTR [rbp-0x1264],0x84
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed9c), (0x84u64) as u32);
            // c2cb7 lea rsi,[rbp-0x1264]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed9c);
            // c2cbe mov rdi,rbx
            rdi = rbx;
            // c2cc1 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2cc6 mov DWORD PTR [rax],0x6
            m.w32(rax, (0x6u64) as u32);
            // c2ccc mov DWORD PTR [rbp-0x1268],0x4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed98), (0x4u64) as u32);
            // c2cd6 lea rsi,[rbp-0x1268]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed98);
            // c2cdd mov rdi,rbx
            rdi = rbx;
            // c2ce0 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2ce5 mov DWORD PTR [rax],0x6
            m.w32(rax, (0x6u64) as u32);
            // c2ceb mov DWORD PTR [rbp-0x126c],0x8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed94), (0x8du64) as u32);
            // c2cf5 lea rsi,[rbp-0x126c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed94);
            // c2cfc mov rdi,rbx
            rdi = rbx;
            // c2cff call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2d04 mov DWORD PTR [rax],0x7
            m.w32(rax, (0x7u64) as u32);
            // c2d0a mov DWORD PTR [rbp-0x1270],0xa9
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed90), (0xa9u64) as u32);
            // c2d14 lea rsi,[rbp-0x1270]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed90);
            // c2d1b mov rdi,rbx
            rdi = rbx;
            // c2d1e call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2d23 mov DWORD PTR [rax],0x32
            m.w32(rax, (0x32u64) as u32);
            // c2d29 mov DWORD PTR [rbp-0x1274],0xaa
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed8c), (0xaau64) as u32);
            // c2d33 lea rsi,[rbp-0x1274]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed8c);
            // c2d3a mov rdi,rbx
            rdi = rbx;
            // c2d3d call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2d42 mov DWORD PTR [rax],0x33
            m.w32(rax, (0x33u64) as u32);
            // c2d48 mov DWORD PTR [rbp-0x1278],0xab
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed88), (0xabu64) as u32);
            // c2d52 lea rsi,[rbp-0x1278]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed88);
            // c2d59 mov rdi,rbx
            rdi = rbx;
            // c2d5c call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2d61 mov DWORD PTR [rax],0x3e
            m.w32(rax, (0x3eu64) as u32);
            // c2d67 mov DWORD PTR [rbp-0x127c],0xdf
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed84), (0xdfu64) as u32);
            // c2d71 lea rsi,[rbp-0x127c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed84);
            // c2d78 mov rdi,rbx
            rdi = rbx;
            // c2d7b call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2d80 mov DWORD PTR [rax],0x53
            m.w32(rax, (0x53u64) as u32);
            // c2d86 mov DWORD PTR [rbp-0x1280],0xe0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed80), (0xe0u64) as u32);
            // c2d90 lea rsi,[rbp-0x1280]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed80);
            // c2d97 mov rdi,rbx
            rdi = rbx;
            // c2d9a call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2d9f mov DWORD PTR [rax],0x54
            m.w32(rax, (0x54u64) as u32);
            // c2da5 mov DWORD PTR [rbp-0x1284],0xe1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed7c), (0xe1u64) as u32);
            // c2daf lea rsi,[rbp-0x1284]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed7c);
            // c2db6 mov rdi,rbx
            rdi = rbx;
            // c2db9 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2dbe mov DWORD PTR [rax],0x55
            m.w32(rax, (0x55u64) as u32);
            // c2dc4 mov DWORD PTR [rbp-0x1288],0x13
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed78), (0x13u64) as u32);
            // c2dce lea rsi,[rbp-0x1288]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed78);
            // c2dd5 mov rdi,rbx
            rdi = rbx;
            // c2dd8 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2ddd mov DWORD PTR [rax],0x26
            m.w32(rax, (0x26u64) as u32);
            // c2de3 mov DWORD PTR [rbp-0x128c],0x12
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed74), (0x12u64) as u32);
            // c2ded lea rsi,[rbp-0x128c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed74);
            // c2df4 mov rdi,rbx
            rdi = rbx;
            // c2df7 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2dfc mov DWORD PTR [rax],0x26
            m.w32(rax, (0x26u64) as u32);
            // c2e02 mov DWORD PTR [rbp-0x1290],0x21
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed70), (0x21u64) as u32);
            // c2e0c lea rsi,[rbp-0x1290]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed70);
            // c2e13 mov rdi,rbx
            rdi = rbx;
            // c2e16 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2e1b mov DWORD PTR [rax],0x30
            m.w32(rax, (0x30u64) as u32);
            // c2e21 mov DWORD PTR [rbp-0x1294],0x20
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed6c), (0x20u64) as u32);
            // c2e2b lea rsi,[rbp-0x1294]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed6c);
            // c2e32 mov rdi,rbx
            rdi = rbx;
            // c2e35 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2e3a mov DWORD PTR [rax],0x30
            m.w32(rax, (0x30u64) as u32);
            // c2e40 mov DWORD PTR [rbp-0x1298],0x56
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed68), (0x56u64) as u32);
            // c2e4a lea rsi,[rbp-0x1298]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed68);
            // c2e51 mov rdi,rbx
            rdi = rbx;
            // c2e54 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2e59 mov DWORD PTR [rax],0x56
            m.w32(rax, (0x56u64) as u32);
            // c2e5f mov DWORD PTR [rbp-0x129c],0x55
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed64), (0x55u64) as u32);
            // c2e69 lea rsi,[rbp-0x129c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed64);
            // c2e70 mov rdi,rbx
            rdi = rbx;
            // c2e73 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2e78 mov DWORD PTR [rax],0x56
            m.w32(rax, (0x56u64) as u32);
            // c2e7e mov DWORD PTR [rbp-0x12a0],0x58
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed60), (0x58u64) as u32);
            // c2e88 lea rsi,[rbp-0x12a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed60);
            // c2e8f mov rdi,rbx
            rdi = rbx;
            // c2e92 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2e97 mov DWORD PTR [rax],0x57
            m.w32(rax, (0x57u64) as u32);
            // c2e9d mov DWORD PTR [rbp-0x12a4],0x57
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed5c), (0x57u64) as u32);
            // c2ea7 lea rsi,[rbp-0x12a4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed5c);
            // c2eae mov rdi,rbx
            rdi = rbx;
            // c2eb1 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2eb6 mov DWORD PTR [rax],0x57
            m.w32(rax, (0x57u64) as u32);
            // c2ebc mov DWORD PTR [rbp-0x12a8],0x95
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed58), (0x95u64) as u32);
            // c2ec6 lea rsi,[rbp-0x12a8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed58);
            // c2ecd mov rdi,rbx
            rdi = rbx;
            // c2ed0 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2ed5 mov DWORD PTR [rax],0x2b
            m.w32(rax, (0x2bu64) as u32);
            // c2edb mov DWORD PTR [rbp-0x12ac],0x9b
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed54), (0x9bu64) as u32);
            // c2ee5 lea rsi,[rbp-0x12ac]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed54);
            // c2eec mov rdi,rbx
            rdi = rbx;
            // c2eef call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2ef4 mov DWORD PTR [rax],0x2c
            m.w32(rax, (0x2cu64) as u32);
            // c2efa mov DWORD PTR [rbp-0x12b0],0x23
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed50), (0x23u64) as u32);
            // c2f04 lea rsi,[rbp-0x12b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed50);
            // c2f0b mov rdi,rbx
            rdi = rbx;
            // c2f0e call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2f13 mov DWORD PTR [rax],0x31
            m.w32(rax, (0x31u64) as u32);
            // c2f19 mov DWORD PTR [rbp-0x12b4],0x94
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed4c), (0x94u64) as u32);
            // c2f23 lea rsi,[rbp-0x12b4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed4c);
            // c2f2a mov rdi,rbx
            rdi = rbx;
            // c2f2d call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2f32 mov DWORD PTR [rax],0x29
            m.w32(rax, (0x29u64) as u32);
            // c2f38 mov DWORD PTR [rbp-0x12b8],0x14
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed48), (0x14u64) as u32);
            // c2f42 lea rsi,[rbp-0x12b8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed48);
            // c2f49 mov rdi,rbx
            rdi = rbx;
            // c2f4c call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2f51 mov DWORD PTR [rax],0x29
            m.w32(rax, (0x29u64) as u32);
            // c2f57 mov DWORD PTR [rbp-0x12bc],0x9a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed44), (0x9au64) as u32);
            // c2f61 lea rsi,[rbp-0x12bc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed44);
            // c2f68 mov rdi,rbx
            rdi = rbx;
            // c2f6b call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2f70 mov DWORD PTR [rax],0x2a
            m.w32(rax, (0x2au64) as u32);
            // c2f76 mov DWORD PTR [rbp-0x12c0],0x1a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed40), (0x1au64) as u32);
            // c2f80 lea rsi,[rbp-0x12c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed40);
            // c2f87 mov rdi,rbx
            rdi = rbx;
            // c2f8a call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2f8f mov DWORD PTR [rax],0x2a
            m.w32(rax, (0x2au64) as u32);
            // c2f95 mov DWORD PTR [rbp-0x12c4],0x89
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed3c), (0x89u64) as u32);
            // c2f9f lea rsi,[rbp-0x12c4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed3c);
            // c2fa6 mov rdi,rbx
            rdi = rbx;
            // c2fa9 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2fae mov DWORD PTR [rax],0x8
            m.w32(rax, (0x8u64) as u32);
            // c2fb4 mov DWORD PTR [rbp-0x12c8],0x9
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed38), (0x9u64) as u32);
            // c2fbe lea rsi,[rbp-0x12c8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed38);
            // c2fc5 mov rdi,rbx
            rdi = rbx;
            // c2fc8 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2fcd mov DWORD PTR [rax],0x8
            m.w32(rax, (0x8u64) as u32);
            // c2fd3 mov DWORD PTR [rbp-0x12cc],0x8a
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed34), (0x8au64) as u32);
            // c2fdd lea rsi,[rbp-0x12cc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed34);
            // c2fe4 mov rdi,rbx
            rdi = rbx;
            // c2fe7 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c2fec mov DWORD PTR [rax],0x9
            m.w32(rax, (0x9u64) as u32);
            // c2ff2 mov DWORD PTR [rbp-0x12d0],0xa
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed30), (0xau64) as u32);
            // c2ffc lea rsi,[rbp-0x12d0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed30);
            // c3003 mov rdi,rbx
            rdi = rbx;
            // c3006 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c300b mov DWORD PTR [rax],0x9
            m.w32(rax, (0x9u64) as u32);
            // c3011 mov DWORD PTR [rbp-0x12d4],0x7
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed2c), (0x7u64) as u32);
            // c301b lea rsi,[rbp-0x12d4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ed2c);
            // c3022 mov rdi,rbx
            rdi = rbx;
            // c3025 call 0xebd2c
            { let r = { let t = m.env.stub_va(0x4f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c302a mov DWORD PTR [rax],0x27
            m.w32(rax, (0x27u64) as u32);
            // c3030 mov rbx,QWORD PTR [rip+0x105639] # 0x1c8670
            rbx = m.r64(m.env.data_va(904765)) as u64;
            // c3037 lea r15,[rip+0x10563a] # 0x1c8678
            r15 = m.env.data_va(904773);
            // c303e cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // c3041 je 0xc3098
            if crate::cov::brk(0xc3041, fl.e()) { pc = 0xc3098; continue; }
            pc = 0xc3043;
        }
        0xc3043 => {
            // block c3043
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3043 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3043 lea r14,[rip+0x10563e] # 0x1c8688
            r14 = m.env.data_va(904789);
            // c304a lea r12,[rbp-0x12e0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ed20);
            pc = 0xc3051;
        }
        0xc3051 => {
            // block c3051
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3051 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3051 mov eax,DWORD PTR [rbx+0x20]
            rax = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // c3054 mov DWORD PTR [rbp-0x12e0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed20), ((rax as u32 as u64)) as u32);
            // c305a mov eax,DWORD PTR [rbx+0x1c]
            rax = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c305d mov DWORD PTR [rbp-0x12dc],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ed24), ((rax as u32 as u64)) as u32);
            // c3063 mov rdi,r14
            rdi = r14;
            // c3066 mov rsi,r12
            rsi = r12;
            // c3069 call 0xebdd4
            { let r = { let t = m.env.stub_va(0x5a1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c306e mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c3072 test rax,rax
            fl = logic::<64>(rax & rax);
            // c3075 je 0xc3084
            if crate::cov::brk(0xc3075, fl.e()) { pc = 0xc3084; continue; }
            pc = 0xc3077;
        }
        0xc3077 => {
            // block c3077
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3077 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3077 mov rbx,rax
            rbx = rax;
            // c307a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c307d test rax,rax
            fl = logic::<64>(rax & rax);
            // c3080 jne 0xc3077
            if crate::cov::brk(0xc3080, fl.ne()) { pc = 0xc3077; continue; }
            pc = 0xc3082;
        }
        0xc3082 => {
            // block c3082
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3082 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3082 jmp 0xc3093
            pc = 0xc3093; continue;
        }
        0xc3084 => {
            // block c3084
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3084 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3084 mov rax,rbx
            rax = rbx;
            pc = 0xc3087;
        }
        0xc3087 => {
            // block c3087
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3087 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3087 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // c308b cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // c308e mov rax,rbx
            rax = rbx;
            // c3091 jne 0xc3087
            if crate::cov::brk(0xc3091, fl.ne()) { pc = 0xc3087; continue; }
            pc = 0xc3093;
        }
        0xc3093 => {
            // block c3093
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3093 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3093 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // c3096 jne 0xc3051
            if crate::cov::brk(0xc3096, fl.ne()) { pc = 0xc3051; continue; }
            pc = 0xc3098;
        }
        0xc3098 => {
            // block c3098
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3098 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3098 call 0xc36a2
            { let r = { let t = m.env.code_va(0xc36a2); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c309d add rsp,0x12b8
            { let (r, f) = add::<64>(rsp, 0x12b8u64);
              rsp = r;
              fl = f; }
            // c30a4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c30a5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c30a7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c30a9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c30ab pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c30ad pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c30ae ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc30af => {
            // block c30af
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c30af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c30af mov rbx,rax
            rbx = rax;
            // c30b2 lea rdi,[rbp-0x218]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde8);
            // c30b9 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc30bb => {
            // block c30bb
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c30bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c30bb mov rbx,rax
            rbx = rax;
            // c30be lea rdi,[rbp-0x738]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8c8);
            pc = 0xc30c5;
        }
        0xc30c5 => {
            // block c30c5
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c30c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c30c5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c30ca mov rdi,rbx
            rdi = rbx;
            // c30cd call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c30d2 mov rbx,rax
            rbx = rax;
            // c30d5 lea rdi,[rbp-0x230]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdd0);
            // c30dc jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc30de => {
            // block c30de
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c30de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c30de mov rbx,rax
            rbx = rax;
            // c30e1 lea rdi,[rbp-0x248]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdb8);
            // c30e8 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc30ea => {
            // block c30ea
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c30ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c30ea mov rbx,rax
            rbx = rax;
            // c30ed lea rdi,[rbp-0x260]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // c30f4 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc30f6 => {
            // block c30f6
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c30f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c30f6 mov rbx,rax
            rbx = rax;
            // c30f9 lea rdi,[rbp-0x278]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd88);
            // c3100 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3102 => {
            // block c3102
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3102 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3102 mov rbx,rax
            rbx = rax;
            // c3105 lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // c310c jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc310e => {
            // block c310e
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c310e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c310e mov rbx,rax
            rbx = rax;
            // c3111 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // c3118 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc311a => {
            // block c311a
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c311a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c311a mov rbx,rax
            rbx = rax;
            // c311d lea rdi,[rbp-0x2c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // c3124 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3126 => {
            // block c3126
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3126 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3126 mov rbx,rax
            rbx = rax;
            // c3129 lea rdi,[rbp-0x2d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd28);
            // c3130 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3132 => {
            // block c3132
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3132 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3132 mov rbx,rax
            rbx = rax;
            // c3135 lea rdi,[rbp-0x2f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd10);
            // c313c jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc313e => {
            // block c313e
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c313e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c313e mov rbx,rax
            rbx = rax;
            // c3141 lea rdi,[rbp-0x308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf8);
            // c3148 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc314d => {
            // block c314d
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c314d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c314d mov rbx,rax
            rbx = rax;
            // c3150 lea rdi,[rbp-0x320]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fce0);
            // c3157 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc315c => {
            // block c315c
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c315c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c315c mov rbx,rax
            rbx = rax;
            // c315f lea rdi,[rbp-0x338]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc8);
            // c3166 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc316b => {
            // block c316b
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c316b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c316b mov rbx,rax
            rbx = rax;
            // c316e lea rdi,[rbp-0x350]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb0);
            // c3175 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc317a => {
            // block c317a
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c317a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c317a mov rbx,rax
            rbx = rax;
            // c317d lea rdi,[rbp-0x368]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc98);
            // c3184 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3189 => {
            // block c3189
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3189 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3189 mov rbx,rax
            rbx = rax;
            // c318c lea rdi,[rbp-0x380]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc80);
            // c3193 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3198 => {
            // block c3198
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3198 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3198 mov rbx,rax
            rbx = rax;
            // c319b lea rdi,[rbp-0x398]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc68);
            // c31a2 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc31a7 => {
            // block c31a7
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c31a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c31a7 mov rbx,rax
            rbx = rax;
            // c31aa lea rdi,[rbp-0x3b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc50);
            // c31b1 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc31b6 => {
            // block c31b6
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c31b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c31b6 mov rbx,rax
            rbx = rax;
            // c31b9 lea rdi,[rbp-0x3c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc38);
            // c31c0 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc31c5 => {
            // block c31c5
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c31c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c31c5 mov rbx,rax
            rbx = rax;
            // c31c8 lea rdi,[rbp-0x3e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc20);
            // c31cf jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc31d4 => {
            // block c31d4
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c31d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c31d4 mov rbx,rax
            rbx = rax;
            // c31d7 lea rdi,[rbp-0x3f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc08);
            // c31de jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc31e3 => {
            // block c31e3
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c31e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c31e3 mov rbx,rax
            rbx = rax;
            // c31e6 lea rdi,[rbp-0x410]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbf0);
            // c31ed jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc31f2 => {
            // block c31f2
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c31f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c31f2 mov rbx,rax
            rbx = rax;
            // c31f5 lea rdi,[rbp-0x428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd8);
            // c31fc jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3201 => {
            // block c3201
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3201 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3201 mov rbx,rax
            rbx = rax;
            // c3204 lea rdi,[rbp-0x440]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbc0);
            // c320b jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3210 => {
            // block c3210
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3210 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3210 mov rbx,rax
            rbx = rax;
            // c3213 lea rdi,[rbp-0x458]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fba8);
            // c321a jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc321f => {
            // block c321f
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c321f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c321f mov rbx,rax
            rbx = rax;
            // c3222 lea rdi,[rbp-0x470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // c3229 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc322e => {
            // block c322e
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c322e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c322e mov rbx,rax
            rbx = rax;
            // c3231 lea rdi,[rbp-0x488]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb78);
            // c3238 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc323d => {
            // block c323d
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c323d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c323d mov rbx,rax
            rbx = rax;
            // c3240 lea rdi,[rbp-0x4a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb60);
            // c3247 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc324c => {
            // block c324c
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c324c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c324c mov rbx,rax
            rbx = rax;
            // c324f lea rdi,[rbp-0x4b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb48);
            // c3256 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc325b => {
            // block c325b
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c325b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c325b mov rbx,rax
            rbx = rax;
            // c325e lea rdi,[rbp-0x4d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb30);
            // c3265 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc326a => {
            // block c326a
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c326a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c326a mov rbx,rax
            rbx = rax;
            // c326d lea rdi,[rbp-0x750]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8b0);
            // c3274 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3279 => {
            // block c3279
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3279 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3279 mov rbx,rax
            rbx = rax;
            // c327c lea rdi,[rbp-0x768]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f898);
            // c3283 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3288 => {
            // block c3288
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3288 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3288 mov rbx,rax
            rbx = rax;
            // c328b lea rdi,[rbp-0x780]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f880);
            // c3292 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3297 => {
            // block c3297
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3297 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3297 mov rbx,rax
            rbx = rax;
            // c329a lea rdi,[rbp-0x798]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f868);
            // c32a1 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc32a6 => {
            // block c32a6
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c32a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c32a6 mov rbx,rax
            rbx = rax;
            // c32a9 lea rdi,[rbp-0x7b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f850);
            // c32b0 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc32b5 => {
            // block c32b5
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c32b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c32b5 mov rbx,rax
            rbx = rax;
            // c32b8 lea rdi,[rbp-0x7c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f838);
            // c32bf jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc32c4 => {
            // block c32c4
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c32c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c32c4 mov rbx,rax
            rbx = rax;
            // c32c7 lea rdi,[rbp-0x7e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f820);
            // c32ce jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc32d3 => {
            // block c32d3
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c32d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c32d3 mov rbx,rax
            rbx = rax;
            // c32d6 lea rdi,[rbp-0x7f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f808);
            // c32dd jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc32e2 => {
            // block c32e2
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c32e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c32e2 mov rbx,rax
            rbx = rax;
            // c32e5 lea rdi,[rbp-0x810]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7f0);
            // c32ec jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc32f1 => {
            // block c32f1
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c32f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c32f1 mov rbx,rax
            rbx = rax;
            // c32f4 lea rdi,[rbp-0x828]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7d8);
            // c32fb jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3300 => {
            // block c3300
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3300 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3300 mov rbx,rax
            rbx = rax;
            // c3303 lea rdi,[rbp-0x840]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7c0);
            // c330a jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc330f => {
            // block c330f
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c330f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c330f mov rbx,rax
            rbx = rax;
            // c3312 lea rdi,[rbp-0x858]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f7a8);
            // c3319 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc331e => {
            // block c331e
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c331e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c331e mov rbx,rax
            rbx = rax;
            // c3321 lea rdi,[rbp-0x870]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f790);
            // c3328 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc332d => {
            // block c332d
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c332d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c332d mov rbx,rax
            rbx = rax;
            // c3330 lea rdi,[rbp-0x888]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f778);
            // c3337 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc333c => {
            // block c333c
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c333c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c333c mov rbx,rax
            rbx = rax;
            // c333f lea rdi,[rbp-0x8a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f760);
            // c3346 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc334b => {
            // block c334b
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c334b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c334b mov rbx,rax
            rbx = rax;
            // c334e lea rdi,[rbp-0x8b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f748);
            // c3355 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc335a => {
            // block c335a
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c335a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c335a mov rbx,rax
            rbx = rax;
            // c335d lea rdi,[rbp-0x8d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f730);
            // c3364 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3369 => {
            // block c3369
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3369 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3369 mov rbx,rax
            rbx = rax;
            // c336c lea rdi,[rbp-0x8e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f718);
            // c3373 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3378 => {
            // block c3378
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3378 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3378 mov rbx,rax
            rbx = rax;
            // c337b lea rdi,[rbp-0x900]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f700);
            // c3382 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3387 => {
            // block c3387
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3387 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3387 mov rbx,rax
            rbx = rax;
            // c338a lea rdi,[rbp-0x918]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6e8);
            // c3391 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3396 => {
            // block c3396
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3396 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3396 mov rbx,rax
            rbx = rax;
            // c3399 lea rdi,[rbp-0x930]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6d0);
            // c33a0 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc33a5 => {
            // block c33a5
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c33a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c33a5 mov rbx,rax
            rbx = rax;
            // c33a8 lea rdi,[rbp-0x948]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6b8);
            // c33af jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc33b4 => {
            // block c33b4
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c33b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c33b4 mov rbx,rax
            rbx = rax;
            // c33b7 lea rdi,[rbp-0x960]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f6a0);
            // c33be jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc33c3 => {
            // block c33c3
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c33c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c33c3 mov rbx,rax
            rbx = rax;
            // c33c6 lea rdi,[rbp-0x978]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f688);
            // c33cd jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc33d2 => {
            // block c33d2
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c33d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c33d2 mov rbx,rax
            rbx = rax;
            // c33d5 lea rdi,[rbp-0x990]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f670);
            // c33dc jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc33e1 => {
            // block c33e1
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c33e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c33e1 mov rbx,rax
            rbx = rax;
            // c33e4 lea rdi,[rbp-0x9a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f658);
            // c33eb jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc33f0 => {
            // block c33f0
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c33f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c33f0 mov rbx,rax
            rbx = rax;
            // c33f3 lea rdi,[rbp-0x9c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f640);
            // c33fa jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc33ff => {
            // block c33ff
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c33ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c33ff mov rbx,rax
            rbx = rax;
            // c3402 lea rdi,[rbp-0x9d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f628);
            // c3409 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc340e => {
            // block c340e
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c340e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c340e mov rbx,rax
            rbx = rax;
            // c3411 lea rdi,[rbp-0x9f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f610);
            // c3418 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc341d => {
            // block c341d
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c341d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c341d mov rbx,rax
            rbx = rax;
            // c3420 lea rdi,[rbp-0xa08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5f8);
            // c3427 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc342c => {
            // block c342c
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c342c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c342c mov rbx,rax
            rbx = rax;
            // c342f lea rdi,[rbp-0xa20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5e0);
            // c3436 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc343b => {
            // block c343b
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c343b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c343b mov rbx,rax
            rbx = rax;
            // c343e lea rdi,[rbp-0xa38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5c8);
            // c3445 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc344a => {
            // block c344a
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c344a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c344a mov rbx,rax
            rbx = rax;
            // c344d lea rdi,[rbp-0xa50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f5b0);
            // c3454 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3459 => {
            // block c3459
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3459 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3459 mov rbx,rax
            rbx = rax;
            // c345c lea rdi,[rbp-0xa68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f598);
            // c3463 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3468 => {
            // block c3468
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3468 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3468 mov rbx,rax
            rbx = rax;
            // c346b lea rdi,[rbp-0xa80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f580);
            // c3472 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3477 => {
            // block c3477
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3477 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3477 mov rbx,rax
            rbx = rax;
            // c347a lea rdi,[rbp-0xa98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f568);
            // c3481 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3486 => {
            // block c3486
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3486 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3486 mov rbx,rax
            rbx = rax;
            // c3489 lea rdi,[rbp-0xab0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f550);
            // c3490 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3495 => {
            // block c3495
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3495 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3495 mov rbx,rax
            rbx = rax;
            // c3498 lea rdi,[rbp-0xac8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f538);
            // c349f jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc34a4 => {
            // block c34a4
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c34a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c34a4 mov rbx,rax
            rbx = rax;
            // c34a7 lea rdi,[rbp-0xae0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f520);
            // c34ae jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc34b3 => {
            // block c34b3
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c34b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c34b3 mov rbx,rax
            rbx = rax;
            // c34b6 lea rdi,[rbp-0xaf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f508);
            // c34bd jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc34c2 => {
            // block c34c2
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c34c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c34c2 mov rbx,rax
            rbx = rax;
            // c34c5 lea rdi,[rbp-0xb10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4f0);
            // c34cc jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc34d1 => {
            // block c34d1
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c34d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c34d1 mov rbx,rax
            rbx = rax;
            // c34d4 lea rdi,[rbp-0xb28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4d8);
            // c34db jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc34e0 => {
            // block c34e0
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c34e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c34e0 mov rbx,rax
            rbx = rax;
            // c34e3 lea rdi,[rbp-0xb40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4c0);
            // c34ea jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc34ef => {
            // block c34ef
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c34ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c34ef mov rbx,rax
            rbx = rax;
            // c34f2 lea rdi,[rbp-0xb58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f4a8);
            // c34f9 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc34fe => {
            // block c34fe
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c34fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c34fe mov rbx,rax
            rbx = rax;
            // c3501 lea rdi,[rbp-0xb70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f490);
            // c3508 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc350d => {
            // block c350d
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c350d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c350d mov rbx,rax
            rbx = rax;
            // c3510 lea rdi,[rbp-0xb88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f478);
            // c3517 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc351c => {
            // block c351c
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c351c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c351c mov rbx,rax
            rbx = rax;
            // c351f lea rdi,[rbp-0xba0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f460);
            // c3526 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc352b => {
            // block c352b
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c352b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c352b mov rbx,rax
            rbx = rax;
            // c352e lea rdi,[rbp-0xbb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f448);
            // c3535 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc353a => {
            // block c353a
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c353a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c353a mov rbx,rax
            rbx = rax;
            // c353d lea rdi,[rbp-0xbd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f430);
            // c3544 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3549 => {
            // block c3549
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3549 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3549 mov rbx,rax
            rbx = rax;
            // c354c lea rdi,[rbp-0xbe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f418);
            // c3553 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3558 => {
            // block c3558
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3558 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3558 mov rbx,rax
            rbx = rax;
            // c355b lea rdi,[rbp-0xc00]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f400);
            // c3562 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3567 => {
            // block c3567
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3567 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3567 mov rbx,rax
            rbx = rax;
            // c356a lea rdi,[rbp-0xc18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3e8);
            // c3571 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3576 => {
            // block c3576
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3576 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3576 mov rbx,rax
            rbx = rax;
            // c3579 lea rdi,[rbp-0xc30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3d0);
            // c3580 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3585 => {
            // block c3585
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3585 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3585 mov rbx,rax
            rbx = rax;
            // c3588 lea rdi,[rbp-0xc48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3b8);
            // c358f jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3594 => {
            // block c3594
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3594 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3594 mov rbx,rax
            rbx = rax;
            // c3597 lea rdi,[rbp-0xc60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f3a0);
            // c359e jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc35a3 => {
            // block c35a3
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c35a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c35a3 mov rbx,rax
            rbx = rax;
            // c35a6 lea rdi,[rbp-0xc78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f388);
            // c35ad jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc35b2 => {
            // block c35b2
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c35b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c35b2 mov rbx,rax
            rbx = rax;
            // c35b5 lea rdi,[rbp-0xc90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f370);
            // c35bc jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc35c1 => {
            // block c35c1
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c35c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c35c1 mov rbx,rax
            rbx = rax;
            // c35c4 lea rdi,[rbp-0xca8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f358);
            // c35cb jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc35d0 => {
            // block c35d0
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c35d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c35d0 mov rbx,rax
            rbx = rax;
            // c35d3 lea rdi,[rbp-0xcc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f340);
            // c35da jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc35df => {
            // block c35df
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c35df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c35df mov rbx,rax
            rbx = rax;
            // c35e2 lea rdi,[rbp-0xcd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f328);
            // c35e9 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc35ee => {
            // block c35ee
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c35ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c35ee mov rbx,rax
            rbx = rax;
            // c35f1 lea rdi,[rbp-0xcf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f310);
            // c35f8 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc35fd => {
            // block c35fd
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c35fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c35fd mov rbx,rax
            rbx = rax;
            // c3600 lea rdi,[rbp-0xd08]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2f8);
            // c3607 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc360c => {
            // block c360c
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c360c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c360c mov rbx,rax
            rbx = rax;
            // c360f lea rdi,[rbp-0xd20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2e0);
            // c3616 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc361b => {
            // block c361b
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c361b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c361b mov rbx,rax
            rbx = rax;
            // c361e lea rdi,[rbp-0xd38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2c8);
            // c3625 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc362a => {
            // block c362a
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c362a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c362a mov rbx,rax
            rbx = rax;
            // c362d lea rdi,[rbp-0xd50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f2b0);
            // c3634 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3639 => {
            // block c3639
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3639 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3639 mov rbx,rax
            rbx = rax;
            // c363c lea rdi,[rbp-0xd68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f298);
            // c3643 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3648 => {
            // block c3648
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3648 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3648 mov rbx,rax
            rbx = rax;
            // c364b lea rdi,[rbp-0xd80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f280);
            // c3652 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3657 => {
            // block c3657
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3657 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3657 mov rbx,rax
            rbx = rax;
            // c365a lea rdi,[rbp-0xd98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f268);
            // c3661 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3666 => {
            // block c3666
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3666 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3666 mov rbx,rax
            rbx = rax;
            // c3669 lea rdi,[rbp-0xdb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f250);
            // c3670 jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3675 => {
            // block c3675
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3675 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3675 mov rbx,rax
            rbx = rax;
            // c3678 lea rdi,[rbp-0xdc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f238);
            // c367f jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3684 => {
            // block c3684
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3684 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3684 mov rbx,rax
            rbx = rax;
            // c3687 lea rdi,[rbp-0xde0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f220);
            // c368e jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
        0xc3693 => {
            // block c3693
            #[cfg(feature = "trace")] eprintln!("f_000bca4e pc c3693 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3693 mov rbx,rax
            rbx = rax;
            // c3696 lea rdi,[rbp-0xdf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f208);
            // c369d jmp 0xc30c5
            pc = 0xc30c5; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
