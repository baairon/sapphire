//! Mechanically lifted functions (scripts/lift_run.py), module b1. Each one is checked against the original
//! in the world harness (tests/world_b1.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xaa710 size=2994
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000aa710<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xaa710;
    loop {
        match pc {
        0xaa710 => {
            // block aa710
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aa710 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa710 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // aa711 mov rbp,rsp
            rbp = rsp;
            // aa714 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // aa716 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // aa718 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // aa71a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // aa71c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // aa71d sub rsp,0x238
            { let (r, f) = sub::<64>(rsp, 0x238u64);
              rsp = r;
              fl = f; }
            // aa724 mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // aa727 mov QWORD PTR [rbp-0x238],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (rdi) as u64);
            // aa72e mov al,BYTE PTR [rip+0x9d1d4] # 0x147908
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(377045)) as u64) as u8 as u64);
            // aa734 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // aa736 jne 0xaa7f5
            if fl.ne() { pc = 0xaa7f5; continue; }
            pc = 0xaa73c;
        }
        0xaa73c => {
            // block aa73c
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aa73c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa73c lea rdi,[rip+0x9d1c5] # 0x147908
            rdi = m.env.data_va(377045);
            // aa743 call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa748 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // aa74a je 0xaa7f5
            if fl.e() { pc = 0xaa7f5; continue; }
            pc = 0xaa750;
        }
        0xaa750 => {
            // block aa750
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aa750 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa750 lea r14,[rip+0x9d139] # 0x147890
            r14 = m.env.data_va(376925);
            // aa757 lea rsi,[rip+0x89f77] # 0x1346d5
            rsi = m.env.data_va(298658);
            // aa75e mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // aa763 mov rdi,r14
            rdi = r14;
            // aa766 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa76b lea rbx,[rip+0x9d136] # 0x1478a8
            rbx = m.env.data_va(376949);
            // aa772 lea rsi,[rip+0x89f6c] # 0x1346e5
            rsi = m.env.data_va(298674);
            // aa779 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // aa77e mov rdi,rbx
            rdi = rbx;
            // aa781 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa786 lea rbx,[rip+0x9d133] # 0x1478c0
            rbx = m.env.data_va(376973);
            // aa78d lea rsi,[rip+0x89f61] # 0x1346f5
            rsi = m.env.data_va(298690);
            // aa794 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // aa799 mov rdi,rbx
            rdi = rbx;
            // aa79c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa7a1 lea rbx,[rip+0x9d130] # 0x1478d8
            rbx = m.env.data_va(376997);
            // aa7a8 lea rsi,[rip+0x89f57] # 0x134706
            rsi = m.env.data_va(298707);
            // aa7af mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // aa7b4 mov rdi,rbx
            rdi = rbx;
            // aa7b7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa7bc lea rbx,[rip+0x9d12d] # 0x1478f0
            rbx = m.env.data_va(377021);
            // aa7c3 lea rsi,[rip+0x8e4fe] # 0x138cc8
            rsi = m.env.data_va(316565);
            // aa7ca xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aa7cc mov rdi,rbx
            rdi = rbx;
            // aa7cf call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa7d4 lea rdi,[rip+0xfffffffffffbde89] # 0x68664
            rdi = m.env.code_va(0x68664);
            // aa7db lea rdx,[rip+0xfffffffffff5498e] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // aa7e2 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // aa7e4 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa7e9 lea rdi,[rip+0x9d118] # 0x147908
            rdi = m.env.data_va(377045);
            // aa7f0 call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaa7f5;
        }
        0xaa7f5 => {
            // block aa7f5
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aa7f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa7f5 mov al,BYTE PTR [rip+0x9d2ad] # 0x147aa8
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(377461)) as u64) as u8 as u64);
            // aa7fb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // aa7fd jne 0xaaa00
            if fl.ne() { pc = 0xaaa00; continue; }
            pc = 0xaa803;
        }
        0xaa803 => {
            // block aa803
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aa803 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa803 lea rdi,[rip+0x9d29e] # 0x147aa8
            rdi = m.env.data_va(377461);
            // aa80a call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa80f test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // aa811 je 0xaaa00
            if fl.e() { pc = 0xaaa00; continue; }
            pc = 0xaa817;
        }
        0xaa817 => {
            // block aa817
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aa817 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa817 lea r14,[rip+0x9d0f2] # 0x147910
            r14 = m.env.data_va(377053);
            // aa81e lea rsi,[rip+0x89ef3] # 0x134718
            rsi = m.env.data_va(298725);
            // aa825 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // aa82a mov rdi,r14
            rdi = r14;
            // aa82d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa832 lea rbx,[rip+0x9d0ef] # 0x147928
            rbx = m.env.data_va(377077);
            // aa839 lea rsi,[rip+0x89ef4] # 0x134734
            rsi = m.env.data_va(298753);
            // aa840 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // aa845 mov rdi,rbx
            rdi = rbx;
            // aa848 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa84d lea rbx,[rip+0x9d0ec] # 0x147940
            rbx = m.env.data_va(377101);
            // aa854 lea rsi,[rip+0x89ef5] # 0x134750
            rsi = m.env.data_va(298781);
            // aa85b mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // aa860 mov rdi,rbx
            rdi = rbx;
            // aa863 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa868 lea rbx,[rip+0x9d0e9] # 0x147958
            rbx = m.env.data_va(377125);
            // aa86f lea rsi,[rip+0x89ef7] # 0x13476d
            rsi = m.env.data_va(298810);
            // aa876 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // aa87b mov rdi,rbx
            rdi = rbx;
            // aa87e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa883 lea rbx,[rip+0x9d0e6] # 0x147970
            rbx = m.env.data_va(377149);
            // aa88a lea rsi,[rip+0x89efa] # 0x13478b
            rsi = m.env.data_va(298840);
            // aa891 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // aa896 mov rdi,rbx
            rdi = rbx;
            // aa899 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa89e lea rbx,[rip+0x9d0e3] # 0x147988
            rbx = m.env.data_va(377173);
            // aa8a5 lea rsi,[rip+0x89efb] # 0x1347a7
            rsi = m.env.data_va(298868);
            // aa8ac mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // aa8b1 mov rdi,rbx
            rdi = rbx;
            // aa8b4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa8b9 lea rbx,[rip+0x9d0e0] # 0x1479a0
            rbx = m.env.data_va(377197);
            // aa8c0 lea rsi,[rip+0x89efc] # 0x1347c3
            rsi = m.env.data_va(298896);
            // aa8c7 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // aa8cc mov rdi,rbx
            rdi = rbx;
            // aa8cf call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa8d4 lea rbx,[rip+0x9d0dd] # 0x1479b8
            rbx = m.env.data_va(377221);
            // aa8db lea rsi,[rip+0x89efe] # 0x1347e0
            rsi = m.env.data_va(298925);
            // aa8e2 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // aa8e7 mov rdi,rbx
            rdi = rbx;
            // aa8ea call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa8ef lea rbx,[rip+0x9d0da] # 0x1479d0
            rbx = m.env.data_va(377245);
            // aa8f6 lea rsi,[rip+0x89f01] # 0x1347fe
            rsi = m.env.data_va(298955);
            // aa8fd mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // aa902 mov rdi,rbx
            rdi = rbx;
            // aa905 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa90a lea rbx,[rip+0x9d0d7] # 0x1479e8
            rbx = m.env.data_va(377269);
            // aa911 lea rsi,[rip+0x89f02] # 0x13481a
            rsi = m.env.data_va(298983);
            // aa918 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // aa91d mov rdi,rbx
            rdi = rbx;
            // aa920 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa925 lea rbx,[rip+0x9d0d4] # 0x147a00
            rbx = m.env.data_va(377293);
            // aa92c lea rsi,[rip+0x89f03] # 0x134836
            rsi = m.env.data_va(299011);
            // aa933 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // aa938 mov rdi,rbx
            rdi = rbx;
            // aa93b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa940 lea rbx,[rip+0x9d0d1] # 0x147a18
            rbx = m.env.data_va(377317);
            // aa947 lea rsi,[rip+0x89f05] # 0x134853
            rsi = m.env.data_va(299040);
            // aa94e mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // aa953 mov rdi,rbx
            rdi = rbx;
            // aa956 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa95b lea rbx,[rip+0x9d0ce] # 0x147a30
            rbx = m.env.data_va(377341);
            // aa962 lea rsi,[rip+0x89f08] # 0x134871
            rsi = m.env.data_va(299070);
            // aa969 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // aa96e mov rdi,rbx
            rdi = rbx;
            // aa971 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa976 lea rbx,[rip+0x9d0cb] # 0x147a48
            rbx = m.env.data_va(377365);
            // aa97d lea rsi,[rip+0x89f09] # 0x13488d
            rsi = m.env.data_va(299098);
            // aa984 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // aa989 mov rdi,rbx
            rdi = rbx;
            // aa98c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa991 lea rbx,[rip+0x9d0c8] # 0x147a60
            rbx = m.env.data_va(377389);
            // aa998 lea rsi,[rip+0x89f0a] # 0x1348a9
            rsi = m.env.data_va(299126);
            // aa99f mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // aa9a4 mov rdi,rbx
            rdi = rbx;
            // aa9a7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa9ac lea rbx,[rip+0x9d0c5] # 0x147a78
            rbx = m.env.data_va(377413);
            // aa9b3 lea rsi,[rip+0x89f0c] # 0x1348c6
            rsi = m.env.data_va(299155);
            // aa9ba mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // aa9bf mov rdi,rbx
            rdi = rbx;
            // aa9c2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa9c7 lea rbx,[rip+0x9d0c2] # 0x147a90
            rbx = m.env.data_va(377437);
            // aa9ce lea rsi,[rip+0x8e2f3] # 0x138cc8
            rsi = m.env.data_va(316565);
            // aa9d5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aa9d7 mov rdi,rbx
            rdi = rbx;
            // aa9da call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa9df lea rdi,[rip+0xfffffffffffbdca9] # 0x6868f
            rdi = m.env.code_va(0x6868f);
            // aa9e6 lea rdx,[rip+0xfffffffffff54783] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // aa9ed xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // aa9ef call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa9f4 lea rdi,[rip+0x9d0ad] # 0x147aa8
            rdi = m.env.data_va(377461);
            // aa9fb call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaaa00;
        }
        0xaaa00 => {
            // block aaa00
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aaa00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aaa00 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // aaa07 lea r14,[rbp-0x160]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // aaa0e mov rax,QWORD PTR [rip+0x957bb] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // aaa15 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // aaa19 mov QWORD PTR [rbp-0x168],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rcx) as u64);
            // aaa20 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // aaa24 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // aaa2b mov rsi,r14
            rsi = r14;
            // aaa2e call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaa33 mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // aaa3b mov DWORD PTR [rbp-0x68],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0xffffffffu64) as u32);
            // aaa42 mov rbx,QWORD PTR [rip+0x95977] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // aaa49 lea r12,[rbx+0x18]
            r12 = rbx.wrapping_add(0x18);
            // aaa4d mov QWORD PTR [rbp-0x168],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (r12) as u64);
            // aaa54 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // aaa58 mov QWORD PTR [rbp-0xf8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rbx) as u64);
            // aaa5f mov rdi,r14
            rdi = r14;
            // aaa62 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaa67 mov QWORD PTR [rbp-0x240],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc0), (r12) as u64);
            // aaa6e mov rax,QWORD PTR [rip+0x95933] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // aaa75 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // aaa79 mov QWORD PTR [rbp-0x248],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb8), (rax) as u64);
            // aaa80 mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // aaa87 lea r12,[rbp-0x120]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // aaa8e mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // aaa99 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // aaaa4 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // aaaaf mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // aaaba mov DWORD PTR [rbp-0x100],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x10u64) as u32);
            // aaac4 xorps xmm0,xmm0
            x0 = 0;
            // aaac7 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // aaacb mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // aaad3 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // aaad7 mov rdi,r14
            rdi = r14;
            // aaada call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaadf mov QWORD PTR [rbp-0x250],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb0), (rbx) as u64);
            // aaae6 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // aaaea call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaaef movzx eax,r15w
            rax = ((r15 as u16 as u64)) as u32 as u64;
            // aaaf3 mov DWORD PTR [rbp-0x230],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fdd0), ((rax as u32 as u64)) as u32);
            // aaaf9 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // aaafc shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // aaaff mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // aab02 mov DWORD PTR [rbp-0x254],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fdac), ((r15 as u32 as u64)) as u32);
            // aab09 shr ecx,0x14
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x14u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // aab0c and ecx,0x3
            { let r = (rcx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // aab0f mov DWORD PTR [rbp-0x258],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fda8), ((rcx as u32 as u64)) as u32);
            // aab15 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // aab18 mov DWORD PTR [rbp-0x25c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fda4), ((rax as u32 as u64)) as u32);
            // aab1e lea rbx,[rip+0x9cd6b] # 0x147890
            rbx = m.env.data_va(376925);
            // aab25 lea r14,[rip+0x57614] # 0x102140
            r14 = m.env.data_va(92429);
            // aab2c xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xaab2f;
        }
        0xaab2f => {
            // block aab2f
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aab2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aab2f lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // aab36 mov rsi,rbx
            rsi = rbx;
            // aab39 call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aab3e mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // aab43 mov rdi,rax
            rdi = rax;
            // aab46 lea rsi,[rip+0x89d97] # 0x1348e4
            rsi = m.env.data_va(299185);
            // aab4d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aab52 mov cl,BYTE PTR [r14]
            rcx = (rcx & !0xff) | ((m.r8(r14) as u64) as u8 as u64);
            // aab55 mov edx,DWORD PTR [rbp-0x230]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fdd0)) as u64) as u32 as u64;
            // aab5b shr edx,cl
            { let (r, f) = shr::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // aab5d and edx,0xf
            { let r = (rdx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aab60 lea rcx,[rdx+rdx*2]
            rcx = rdx.wrapping_add(rdx.wrapping_mul(2));
            // aab64 lea rdx,[rip+0x9cda5] # 0x147910
            rdx = m.env.data_va(377053);
            // aab6b lea rsi,[rdx+rcx*8]
            rsi = rdx.wrapping_add(rcx.wrapping_mul(8));
            // aab6f mov rdi,rax
            rdi = rax;
            // aab72 call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aab77 mov r12,rax
            r12 = rax;
            // aab7a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // aab7e mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // aab82 add rsi,r12
            { let (r, f) = add::<64>(rsi, r12);
              rsi = r;
              fl = f; }
            // aab85 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // aab89 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aab8e lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // aab92 mov rsi,QWORD PTR [rip+0x955f7] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // aab99 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aab9e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // aaba1 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // aaba5 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // aabaa mov rdi,rax
            rdi = rax;
            // aabad call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aabaf mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // aabb2 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // aabb6 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aabbb movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // aabbf mov rdi,r12
            rdi = r12;
            // aabc2 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aabc7 mov rdi,r12
            rdi = r12;
            // aabca call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aabcf inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // aabd2 add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // aabd6 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // aabda cmp r13,0x4
            fl = sub::<64>(r13, 0x4u64).1;
            // aabde jb 0xaab2f
            if fl.b() { pc = 0xaab2f; continue; }
            pc = 0xaabe4;
        }
        0xaabe4 => {
            // block aabe4
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aabe4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aabe4 lea rsi,[rip+0x89d03] # 0x1348ee
            rsi = m.env.data_va(299195);
            // aabeb lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // aabf2 mov edx,0x24
            rdx = (0x24u64) as u32 as u64;
            // aabf7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aabfc mov rbx,rax
            rbx = rax;
            // aabff mov r13d,DWORD PTR [rbp-0x254]
            r13 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fdac)) as u64) as u32 as u64;
            // aac06 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // aac0d mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // aac12 mov esi,DWORD PTR [rbp-0x258]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fda8)) as u64) as u32 as u64;
            // aac18 call 0x5d0c0
            { let r = { let t = m.env.code_va(0x5d0c0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aac1d movzx edx,BYTE PTR [rbp-0x180]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64) as u32 as u64;
            // aac24 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // aac27 jne 0xaac35
            if fl.ne() { pc = 0xaac35; continue; }
            pc = 0xaac29;
        }
        0xaac29 => {
            // block aac29
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aac29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aac29 lea rsi,[rbp-0x17f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe81);
            // aac30 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // aac33 jmp 0xaac43
            pc = 0xaac43; continue;
        }
        0xaac35 => {
            // block aac35
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aac35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aac35 mov rdx,QWORD PTR [rbp-0x178]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // aac3c mov rsi,QWORD PTR [rbp-0x170]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            pc = 0xaac43;
        }
        0xaac43 => {
            // block aac43
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aac43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aac43 mov rdi,rbx
            rdi = rbx;
            // aac46 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aac4b lea rsi,[rip+0x89cc1] # 0x134913
            rsi = m.env.data_va(299232);
            // aac52 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // aac57 mov rdi,rax
            rdi = rax;
            // aac5a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aac5f mov rbx,rax
            rbx = rax;
            // aac62 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // aac69 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // aac6e mov esi,DWORD PTR [rbp-0x25c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fda4)) as u64) as u32 as u64;
            // aac74 call 0x60c44
            { let r = { let t = m.env.code_va(0x60c44); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aac79 movzx edx,BYTE PTR [rbp-0x198]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64) as u32 as u64;
            // aac80 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // aac83 jne 0xaac91
            if fl.ne() { pc = 0xaac91; continue; }
            pc = 0xaac85;
        }
        0xaac85 => {
            // block aac85
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aac85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aac85 lea rsi,[rbp-0x197]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe69);
            // aac8c shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // aac8f jmp 0xaac9f
            pc = 0xaac9f; continue;
        }
        0xaac91 => {
            // block aac91
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aac91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aac91 mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // aac98 mov rsi,QWORD PTR [rbp-0x188]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            pc = 0xaac9f;
        }
        0xaac9f => {
            // block aac9f
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aac9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aac9f mov rdi,rbx
            rdi = rbx;
            // aaca2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaca7 mov rbx,rax
            rbx = rax;
            // aacaa mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // aacad mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // aacb1 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // aacb4 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // aacb8 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aacbd mov rsi,QWORD PTR [rip+0x954cc] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // aacc4 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // aacc8 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaccd mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // aacd0 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // aacd4 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // aacd9 mov rdi,rax
            rdi = rax;
            // aacdc call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aacde mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // aace1 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // aace5 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aacea movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // aacee mov rdi,rbx
            rdi = rbx;
            // aacf1 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aacf6 mov rdi,rbx
            rdi = rbx;
            // aacf9 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aacfe lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // aad05 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aad0a lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // aad11 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aad16 lea rsi,[rip+0x89c02] # 0x13491f
            rsi = m.env.data_va(299244);
            // aad1d lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // aad24 mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // aad29 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aad2e mov rbx,rax
            rbx = rax;
            // aad31 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // aad34 shr esi,0x1c
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x1cu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // aad37 mov DWORD PTR [rbp-0x230],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fdd0), ((rsi as u32 as u64)) as u32);
            // aad3d lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // aad44 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // aad49 call 0x5d0c0
            { let r = { let t = m.env.code_va(0x5d0c0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aad4e movzx edx,BYTE PTR [rbp-0x1b0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64) as u32 as u64;
            // aad55 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // aad58 jne 0xaad66
            if fl.ne() { pc = 0xaad66; continue; }
            pc = 0xaad5a;
        }
        0xaad5a => {
            // block aad5a
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aad5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aad5a lea rsi,[rbp-0x1af]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe51);
            // aad61 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // aad64 jmp 0xaad74
            pc = 0xaad74; continue;
        }
        0xaad66 => {
            // block aad66
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aad66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aad66 mov rdx,QWORD PTR [rbp-0x1a8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // aad6d mov rsi,QWORD PTR [rbp-0x1a0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            pc = 0xaad74;
        }
        0xaad74 => {
            // block aad74
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aad74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aad74 mov rdi,rbx
            rdi = rbx;
            // aad77 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aad7c lea rsi,[rip+0x89b90] # 0x134913
            rsi = m.env.data_va(299232);
            // aad83 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // aad88 mov rdi,rax
            rdi = rax;
            // aad8b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aad90 mov rbx,rax
            rbx = rax;
            // aad93 mov r15d,r13d
            r15 = ((r13 as u32 as u64)) as u32 as u64;
            // aad96 shr r15d,0x18
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x18u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // aad9a mov r14d,r15d
            r14 = ((r15 as u32 as u64)) as u32 as u64;
            // aad9d and r14d,0xf
            { let r = (r14 as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // aada1 lea rdi,[rbp-0x1c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe38);
            // aada8 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // aadad mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // aadb0 call 0x60c44
            { let r = { let t = m.env.code_va(0x60c44); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aadb5 movzx edx,BYTE PTR [rbp-0x1c8]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64) as u32 as u64;
            // aadbc test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // aadbf jne 0xaadcd
            if fl.ne() { pc = 0xaadcd; continue; }
            pc = 0xaadc1;
        }
        0xaadc1 => {
            // block aadc1
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aadc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aadc1 lea rsi,[rbp-0x1c7]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe39);
            // aadc8 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // aadcb jmp 0xaaddb
            pc = 0xaaddb; continue;
        }
        0xaadcd => {
            // block aadcd
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aadcd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aadcd mov rdx,QWORD PTR [rbp-0x1c0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // aadd4 mov rsi,QWORD PTR [rbp-0x1b8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            pc = 0xaaddb;
        }
        0xaaddb => {
            // block aaddb
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aaddb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aaddb mov rdi,rbx
            rdi = rbx;
            // aadde call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aade3 mov rbx,rax
            rbx = rax;
            // aade6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // aade9 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // aaded add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // aadf0 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // aadf4 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aadf9 mov rsi,QWORD PTR [rip+0x95390] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // aae00 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // aae04 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aae09 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // aae0c mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // aae10 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // aae15 mov rdi,rax
            rdi = rax;
            // aae18 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aae1a mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // aae1d lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // aae21 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aae26 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // aae2a mov rdi,rbx
            rdi = rbx;
            // aae2d call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aae32 mov rdi,rbx
            rdi = rbx;
            // aae35 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aae3a lea rdi,[rbp-0x1c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe38);
            // aae41 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aae46 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // aae4d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aae52 lea rsi,[rip+0x89ae7] # 0x134940
            rsi = m.env.data_va(299277);
            // aae59 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // aae60 mov edx,0x27
            rdx = (0x27u64) as u32 as u64;
            // aae65 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aae6a mov rbx,rax
            rbx = rax;
            // aae6d shr r14d,0x2
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x2u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // aae71 lea rdi,[rbp-0x1e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe20);
            // aae78 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // aae7d mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // aae80 call 0x5d0c0
            { let r = { let t = m.env.code_va(0x5d0c0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aae85 movzx edx,BYTE PTR [rbp-0x1e0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64) as u32 as u64;
            // aae8c test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // aae8f jne 0xaae9d
            if fl.ne() { pc = 0xaae9d; continue; }
            pc = 0xaae91;
        }
        0xaae91 => {
            // block aae91
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aae91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aae91 lea rsi,[rbp-0x1df]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe21);
            // aae98 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // aae9b jmp 0xaaeab
            pc = 0xaaeab; continue;
        }
        0xaae9d => {
            // block aae9d
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aae9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aae9d mov rdx,QWORD PTR [rbp-0x1d8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // aaea4 mov rsi,QWORD PTR [rbp-0x1d0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            pc = 0xaaeab;
        }
        0xaaeab => {
            // block aaeab
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aaeab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aaeab mov rdi,rbx
            rdi = rbx;
            // aaeae call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaeb3 lea rsi,[rip+0x89a59] # 0x134913
            rsi = m.env.data_va(299232);
            // aaeba mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // aaebf mov rdi,rax
            rdi = rax;
            // aaec2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaec7 mov rbx,rax
            rbx = rax;
            // aaeca and r15d,0x3
            { let r = (r15 as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // aaece lea rdi,[rbp-0x1f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe08);
            // aaed5 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // aaeda mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // aaedd call 0x60ec3
            { let r = { let t = m.env.code_va(0x60ec3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaee2 movzx edx,BYTE PTR [rbp-0x1f8]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe08)) as u64) as u32 as u64;
            // aaee9 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // aaeec jne 0xaaefa
            if fl.ne() { pc = 0xaaefa; continue; }
            pc = 0xaaeee;
        }
        0xaaeee => {
            // block aaeee
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aaeee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aaeee lea rsi,[rbp-0x1f7]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe09);
            // aaef5 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // aaef8 jmp 0xaaf08
            pc = 0xaaf08; continue;
        }
        0xaaefa => {
            // block aaefa
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aaefa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aaefa mov rdx,QWORD PTR [rbp-0x1f0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe10)) as u64;
            // aaf01 mov rsi,QWORD PTR [rbp-0x1e8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe18)) as u64;
            pc = 0xaaf08;
        }
        0xaaf08 => {
            // block aaf08
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aaf08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aaf08 mov rdi,rbx
            rdi = rbx;
            // aaf0b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaf10 mov rbx,rax
            rbx = rax;
            // aaf13 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // aaf16 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // aaf1a add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // aaf1d lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // aaf21 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaf26 mov rsi,QWORD PTR [rip+0x95263] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // aaf2d lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // aaf31 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaf36 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // aaf39 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // aaf3d mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // aaf42 mov rdi,rax
            rdi = rax;
            // aaf45 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaf47 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // aaf4a lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // aaf4e call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaf53 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // aaf57 mov rdi,rbx
            rdi = rbx;
            // aaf5a call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaf5f mov rdi,rbx
            rdi = rbx;
            // aaf62 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaf67 lea rsi,[rip+0x899fa] # 0x134968
            rsi = m.env.data_va(299317);
            // aaf6e mov edx,0x27
            rdx = (0x27u64) as u32 as u64;
            // aaf73 mov rdi,rbx
            rdi = rbx;
            // aaf76 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaf7b mov rbx,rax
            rbx = rax;
            // aaf7e shr r13d,0x1e
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x1eu64, fl); r13 = (r) as u32 as u64; fl = f; }
            // aaf82 lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // aaf89 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // aaf8e mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // aaf91 call 0x5d0c0
            { let r = { let t = m.env.code_va(0x5d0c0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaf96 movzx edx,BYTE PTR [rbp-0x210]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fdf0)) as u64) as u32 as u64;
            // aaf9d test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // aafa0 jne 0xaafae
            if fl.ne() { pc = 0xaafae; continue; }
            pc = 0xaafa2;
        }
        0xaafa2 => {
            // block aafa2
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aafa2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aafa2 lea rsi,[rbp-0x20f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fdf1);
            // aafa9 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // aafac jmp 0xaafbc
            pc = 0xaafbc; continue;
        }
        0xaafae => {
            // block aafae
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aafae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aafae mov rdx,QWORD PTR [rbp-0x208]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdf8)) as u64;
            // aafb5 mov rsi,QWORD PTR [rbp-0x200]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe00)) as u64;
            pc = 0xaafbc;
        }
        0xaafbc => {
            // block aafbc
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc aafbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aafbc mov rdi,rbx
            rdi = rbx;
            // aafbf call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aafc4 lea rsi,[rip+0x89948] # 0x134913
            rsi = m.env.data_va(299232);
            // aafcb mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // aafd0 mov rdi,rax
            rdi = rax;
            // aafd3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aafd8 mov rbx,rax
            rbx = rax;
            // aafdb mov esi,DWORD PTR [rbp-0x230]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fdd0)) as u64) as u32 as u64;
            // aafe1 and esi,0x3
            { let r = (rsi as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // aafe4 lea rdi,[rbp-0x228]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdd8);
            // aafeb mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // aaff0 call 0x60ec3
            { let r = { let t = m.env.code_va(0x60ec3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aaff5 movzx edx,BYTE PTR [rbp-0x228]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fdd8)) as u64) as u32 as u64;
            // aaffc test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // aafff jne 0xab00d
            if fl.ne() { pc = 0xab00d; continue; }
            pc = 0xab001;
        }
        0xab001 => {
            // block ab001
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab001 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab001 lea rsi,[rbp-0x227]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fdd9);
            // ab008 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // ab00b jmp 0xab01b
            pc = 0xab01b; continue;
        }
        0xab00d => {
            // block ab00d
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab00d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab00d mov rdx,QWORD PTR [rbp-0x220]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde0)) as u64;
            // ab014 mov rsi,QWORD PTR [rbp-0x218]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde8)) as u64;
            pc = 0xab01b;
        }
        0xab01b => {
            // block ab01b
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab01b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab01b mov rdi,rbx
            rdi = rbx;
            // ab01e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab023 lea rdi,[rbp-0x228]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdd8);
            // ab02a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab02f lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // ab036 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab03b lea rdi,[rbp-0x1f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe08);
            // ab042 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab047 lea rdi,[rbp-0x1e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe20);
            // ab04e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab053 mov rdi,QWORD PTR [rbp-0x238]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8)) as u64;
            // ab05a lea rsi,[rbp-0x160]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // ab061 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab066 mov rax,QWORD PTR [rbp-0x240]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdc0)) as u64;
            // ab06d mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // ab074 mov rax,QWORD PTR [rbp-0x250]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdb0)) as u64;
            // ab07b mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // ab082 mov rax,QWORD PTR [rbp-0x248]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdb8)) as u64;
            // ab089 mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // ab090 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // ab097 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab09c lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // ab0a3 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab0a8 mov rsi,QWORD PTR [rip+0x95159] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // ab0af add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // ab0b3 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // ab0ba call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab0bf lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // ab0c6 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab0cb mov rax,QWORD PTR [rbp-0x238]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8)) as u64;
            // ab0d2 add rsp,0x238
            { let (r, f) = add::<64>(rsp, 0x238u64);
              rsp = r;
              fl = f; }
            // ab0d9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ab0da pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ab0dc pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ab0de pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ab0e0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ab0e2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ab0e3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xab0e4 => {
            // block ab0e4
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab0e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab0e4 mov r15,rax
            r15 = rax;
            // ab0e7 jmp 0xab21a
            pc = 0xab21a; continue;
        }
        0xab0ec => {
            // block ab0ec
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab0ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab0ec mov r15,rax
            r15 = rax;
            // ab0ef lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // ab0f3 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab0f8 jmp 0xab21a
            pc = 0xab21a; continue;
        }
        0xab0fd => {
            // block ab0fd
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab0fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab0fd jmp 0xab0e4
            pc = 0xab0e4; continue;
        }
        0xab0ff => {
            // block ab0ff
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab0ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab0ff mov r15,rax
            r15 = rax;
            // ab102 jmp 0xab202
            pc = 0xab202; continue;
        }
        0xab107 => {
            // block ab107
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab107 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab107 mov r15,rax
            r15 = rax;
            // ab10a jmp 0xab150
            pc = 0xab150; continue;
        }
        0xab10c => {
            // block ab10c
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab10c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab10c mov r15,rax
            r15 = rax;
            // ab10f jmp 0xab174
            pc = 0xab174; continue;
        }
        0xab111 => {
            // block ab111
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab111 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab111 mov r15,rax
            r15 = rax;
            pc = 0xab114;
        }
        0xab114 => {
            // block ab114
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab114 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab114 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // ab118 mov rdi,rbx
            rdi = rbx;
            // ab11b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab120 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // ab123 jne 0xab114
            if fl.ne() { pc = 0xab114; continue; }
            pc = 0xab125;
        }
        0xab125 => {
            // block ab125
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab125 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab125 jmp 0xab2b4
            pc = 0xab2b4; continue;
        }
        0xab12a => {
            // block ab12a
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab12a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab12a mov r15,rax
            r15 = rax;
            // ab12d jmp 0xab15c
            pc = 0xab15c; continue;
        }
        0xab12f => {
            // block ab12f
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab12f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab12f mov r15,rax
            r15 = rax;
            // ab132 jmp 0xab180
            pc = 0xab180; continue;
        }
        0xab134 => {
            // block ab134
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab134 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab134 mov r15,rax
            r15 = rax;
            // ab137 jmp 0xab20e
            pc = 0xab20e; continue;
        }
        0xab13c => {
            // block ab13c
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab13c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab13c mov r15,rax
            r15 = rax;
            // ab13f jmp 0xab1f6
            pc = 0xab1f6; continue;
        }
        0xab144 => {
            // block ab144
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab144 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab144 mov r15,rax
            r15 = rax;
            // ab147 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ab14b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab150;
        }
        0xab150 => {
            // block ab150
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab150 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab150 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // ab157 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab15c;
        }
        0xab15c => {
            // block ab15c
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab15c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab15c lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // ab163 jmp 0xab215
            pc = 0xab215; continue;
        }
        0xab168 => {
            // block ab168
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab168 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab168 mov r15,rax
            r15 = rax;
            // ab16b lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // ab16f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab174;
        }
        0xab174 => {
            // block ab174
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab174 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab174 lea rdi,[rbp-0x1c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe38);
            // ab17b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab180;
        }
        0xab180 => {
            // block ab180
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab180 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab180 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // ab187 jmp 0xab215
            pc = 0xab215; continue;
        }
        0xab18c => {
            // block ab18c
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab18c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab18c mov r15,rax
            r15 = rax;
            // ab18f lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // ab193 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab198 jmp 0xab202
            pc = 0xab202; continue;
        }
        0xab19a => {
            // block ab19a
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab19a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab19a mov r15,rax
            r15 = rax;
            // ab19d jmp 0xab1db
            pc = 0xab1db; continue;
        }
        0xab19f => {
            // block ab19f
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab19f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab19f mov r15,rax
            r15 = rax;
            // ab1a2 jmp 0xab1c4
            pc = 0xab1c4; continue;
        }
        0xab1a4 => {
            // block ab1a4
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab1a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab1a4 mov r15,rax
            r15 = rax;
            // ab1a7 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // ab1ab call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab1b0 mov rdi,r12
            rdi = r12;
            // ab1b3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab1b8 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // ab1bf call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab1c4;
        }
        0xab1c4 => {
            // block ab1c4
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab1c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab1c4 mov rsi,QWORD PTR [rip+0x9503d] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // ab1cb add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // ab1cf lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // ab1d6 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab1db;
        }
        0xab1db => {
            // block ab1db
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab1db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab1db lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // ab1e2 jmp 0xab27d
            pc = 0xab27d; continue;
        }
        0xab1e7 => {
            // block ab1e7
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab1e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab1e7 mov r15,rax
            r15 = rax;
            // ab1ea lea rdi,[rbp-0x228]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdd8);
            // ab1f1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab1f6;
        }
        0xab1f6 => {
            // block ab1f6
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab1f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab1f6 lea rdi,[rbp-0x210]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf0);
            // ab1fd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab202;
        }
        0xab202 => {
            // block ab202
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab202 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab202 lea rdi,[rbp-0x1f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe08);
            // ab209 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab20e;
        }
        0xab20e => {
            // block ab20e
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab20e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab20e lea rdi,[rbp-0x1e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe20);
            pc = 0xab215;
        }
        0xab215 => {
            // block ab215
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab215 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab215 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab21a;
        }
        0xab21a => {
            // block ab21a
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab21a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab21a lea rbx,[rbp-0xf8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // ab221 mov rax,QWORD PTR [rbp-0x250]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdb0)) as u64;
            // ab228 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // ab22f mov rcx,QWORD PTR [rbp-0x240]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdc0)) as u64;
            // ab236 mov rdx,QWORD PTR [rbp-0x248]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdb8)) as u64;
            // ab23d mov QWORD PTR [rbp-0x168],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rcx) as u64);
            // ab244 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // ab24b mov QWORD PTR [rbp-0x160],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rdx) as u64);
            // ab252 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab257 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // ab25e call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab263 mov rsi,QWORD PTR [rip+0x94f9e] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // ab26a add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // ab26e lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // ab275 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab27a mov rdi,rbx
            rdi = rbx;
            pc = 0xab27d;
        }
        0xab27d => {
            // block ab27d
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab27d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab27d call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xab282;
        }
        0xab282 => {
            // block ab282
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab282 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab282 mov rdi,r15
            rdi = r15;
            // ab285 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab28a mov r15,rax
            r15 = rax;
            pc = 0xab28d;
        }
        0xab28d => {
            // block ab28d
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab28d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab28d add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // ab291 mov rdi,rbx
            rdi = rbx;
            // ab294 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab299 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // ab29c jne 0xab28d
            if fl.ne() { pc = 0xab28d; continue; }
            pc = 0xab29e;
        }
        0xab29e => {
            // block ab29e
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab29e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab29e jmp 0xab2a3
            pc = 0xab2a3; continue;
        }
        0xab2a0 => {
            // block ab2a0
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab2a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab2a0 mov r15,rax
            r15 = rax;
            pc = 0xab2a3;
        }
        0xab2a3 => {
            // block ab2a3
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab2a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab2a3 lea rdi,[rip+0x9c65e] # 0x147908
            rdi = m.env.data_va(377045);
            // ab2aa call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab2af jmp 0xab282
            pc = 0xab282; continue;
        }
        0xab2b1 => {
            // block ab2b1
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab2b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab2b1 mov r15,rax
            r15 = rax;
            pc = 0xab2b4;
        }
        0xab2b4 => {
            // block ab2b4
            #[cfg(feature = "trace")] eprintln!("f_000aa710 pc ab2b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ab2b4 lea rdi,[rip+0x9c7ed] # 0x147aa8
            rdi = m.env.data_va(377461);
            // ab2bb call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab2c0 jmp 0xab282
            pc = 0xab282; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xafd60 size=3096
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000afd60<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xafd60;
    loop {
        match pc {
        0xafd60 => {
            // block afd60
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc afd60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd60 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // afd61 mov rbp,rsp
            rbp = rsp;
            // afd64 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // afd66 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // afd68 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // afd6a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // afd6c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // afd6d sub rsp,0x1b8
            { let (r, f) = sub::<64>(rsp, 0x1b8u64);
              rsp = r;
              fl = f; }
            // afd74 mov r14d,r8d
            r14 = ((r8 as u32 as u64)) as u32 as u64;
            // afd77 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // afd7a mov QWORD PTR [rbp-0x1c0],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe40), (rdi) as u64);
            // afd81 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // afd88 lea r15,[rbp-0x190]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // afd8f mov rax,QWORD PTR [rip+0x9043a] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // afd96 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // afd9a mov QWORD PTR [rbp-0x198],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rcx) as u64);
            // afda1 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // afda5 mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // afdac mov rsi,r15
            rsi = r15;
            // afdaf call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afdb4 mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // afdbf mov DWORD PTR [rbp-0x98],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0xffffffffu64) as u32);
            // afdc9 mov rbx,QWORD PTR [rip+0x905f0] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // afdd0 lea r13,[rbx+0x18]
            r13 = rbx.wrapping_add(0x18);
            // afdd4 mov QWORD PTR [rbp-0x198],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (r13) as u64);
            // afddb add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // afddf mov QWORD PTR [rbp-0x128],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rbx) as u64);
            // afde6 mov rdi,r15
            rdi = r15;
            // afde9 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afdee mov QWORD PTR [rbp-0x1d0],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe30), (r13) as u64);
            // afdf5 mov QWORD PTR [rbp-0x1c8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe38), (rbx) as u64);
            // afdfc mov rax,QWORD PTR [rip+0x905a5] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // afe03 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // afe07 mov QWORD PTR [rbp-0x1d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe28), (rax) as u64);
            // afe0e mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // afe15 mov QWORD PTR [rbp-0x138],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (0x0u64) as u64);
            // afe20 mov QWORD PTR [rbp-0x140],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (0x0u64) as u64);
            // afe2b mov QWORD PTR [rbp-0x148],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (0x0u64) as u64);
            // afe36 mov QWORD PTR [rbp-0x150],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (0x0u64) as u64);
            // afe41 mov DWORD PTR [rbp-0x130],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (0x10u64) as u32);
            // afe4b xorps xmm0,xmm0
            x0 = 0;
            // afe4e movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // afe55 mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // afe5d lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // afe64 mov rdi,r15
            rdi = r15;
            // afe67 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afe6c lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // afe73 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afe78 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // afe7b call 0xce64c
            { let r = { let t = m.env.code_va(0xce64c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afe80 mov DWORD PTR [rbp-0x1b8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe48), ((rax as u32 as u64)) as u32);
            // afe86 cmp DWORD PTR [rbp-0x1b8],0x2
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64, 0x2u64).1;
            // afe8d jne 0xafe9f
            if fl.ne() { pc = 0xafe9f; continue; }
            pc = 0xafe8f;
        }
        0xafe8f => {
            // block afe8f
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc afe8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afe8f mov r15d,r12d
            r15 = ((r12 as u32 as u64)) as u32 as u64;
            // afe92 mov r13d,r15d
            r13 = ((r15 as u32 as u64)) as u32 as u64;
            // afe95 shr r13d,0x4
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x4u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // afe99 and r13d,0x3f
            { let r = (r13 as u32 as u64) & 0x3fu64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // afe9d jmp 0xafead
            pc = 0xafead; continue;
        }
        0xafe9f => {
            // block afe9f
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc afe9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afe9f mov r15d,r12d
            r15 = ((r12 as u32 as u64)) as u32 as u64;
            // afea2 mov r13d,r15d
            r13 = ((r15 as u32 as u64)) as u32 as u64;
            // afea5 shr r13d,0x18
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x18u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // afea9 and r13d,0x7
            { let r = (r13 as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xafead;
        }
        0xafead => {
            // block afead
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc afead rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afead mov al,BYTE PTR [rip+0x986bd] # 0x148570
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(380221)) as u64) as u8 as u64);
            // afeb3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // afeb5 jne 0xb000f
            if fl.ne() { pc = 0xb000f; continue; }
            pc = 0xafebb;
        }
        0xafebb => {
            // block afebb
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc afebb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afebb lea rdi,[rip+0x986ae] # 0x148570
            rdi = m.env.data_va(380221);
            // afec2 call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afec7 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // afec9 je 0xb000f
            if fl.e() { pc = 0xb000f; continue; }
            pc = 0xafecf;
        }
        0xafecf => {
            // block afecf
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc afecf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afecf lea r14,[rip+0x9839a] # 0x148270
            r14 = m.env.data_va(379453);
            // afed6 lea rsi,[rip+0x7f872] # 0x12f74f
            rsi = m.env.data_va(278300);
            // afedd mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // afee2 mov rdi,r14
            rdi = r14;
            // afee5 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afeea lea r12,[rip+0x98397] # 0x148288
            r12 = m.env.data_va(379477);
            // afef1 lea rsi,[rip+0x7f870] # 0x12f768
            rsi = m.env.data_va(278325);
            // afef8 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // afefd mov rdi,r12
            rdi = r12;
            // aff00 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aff05 lea r12,[rip+0x98394] # 0x1482a0
            r12 = m.env.data_va(379501);
            // aff0c lea rsi,[rip+0x84e7f] # 0x134d92
            rsi = m.env.data_va(300383);
            // aff13 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // aff18 mov rdi,r12
            rdi = r12;
            // aff1b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aff20 lea r12,[rip+0x98391] # 0x1482b8
            r12 = m.env.data_va(379525);
            // aff27 lea rsi,[rip+0x84e69] # 0x134d97
            rsi = m.env.data_va(300388);
            // aff2e mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // aff33 mov rdi,r12
            rdi = r12;
            // aff36 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aff3b lea r12,[rip+0x9838e] # 0x1482d0
            r12 = m.env.data_va(379549);
            // aff42 lea rsi,[rip+0x7f864] # 0x12f7ad
            rsi = m.env.data_va(278394);
            // aff49 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // aff4e mov rdi,r12
            rdi = r12;
            // aff51 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aff56 lea r12,[rip+0x9838b] # 0x1482e8
            r12 = m.env.data_va(379573);
            // aff5d lea rsi,[rip+0x84e38] # 0x134d9c
            rsi = m.env.data_va(300393);
            // aff64 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // aff69 mov rdi,r12
            rdi = r12;
            // aff6c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aff71 lea r12,[rip+0x98388] # 0x148300
            r12 = m.env.data_va(379597);
            // aff78 lea rsi,[rip+0x84ee1] # 0x134e60
            rsi = m.env.data_va(300589);
            // aff7f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // aff84 mov rdi,r12
            rdi = r12;
            // aff87 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aff8c lea r12,[rip+0x98385] # 0x148318
            r12 = m.env.data_va(379621);
            // aff93 lea rsi,[rip+0x84ec8] # 0x134e62
            rsi = m.env.data_va(300591);
            // aff9a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // aff9f mov rdi,r12
            rdi = r12;
            // affa2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // affa7 lea r12,[rip+0x98382] # 0x148330
            r12 = m.env.data_va(379645);
            // affae lea rsi,[rip+0x84eaf] # 0x134e64
            rsi = m.env.data_va(300593);
            // affb5 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // affba mov rdi,r12
            rdi = r12;
            // affbd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // affc2 lea r12,[rip+0x9837f] # 0x148348
            r12 = m.env.data_va(379669);
            // affc9 lea rsi,[rip+0x84e9a] # 0x134e6a
            rsi = m.env.data_va(300599);
            // affd0 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // affd5 mov rdi,r12
            rdi = r12;
            // affd8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // affdd lea rdi,[rip+0x9837c] # 0x148360
            rdi = m.env.data_va(379693);
            // affe4 mov esi,0x210
            rsi = (0x210u64) as u32 as u64;
            // affe9 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // affee lea rdi,[rip+0xfffffffffffb8771] # 0x68766
            rdi = m.env.code_va(0x68766);
            // afff5 lea rdx,[rip+0xfffffffffff4f174] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // afffc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // afffe call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0003 lea rdi,[rip+0x98566] # 0x148570
            rdi = m.env.data_va(380221);
            // b000a call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb000f;
        }
        0xb000f => {
            // block b000f
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b000f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b000f mov al,BYTE PTR [rip+0x9886b] # 0x148880
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(381005)) as u64) as u8 as u64);
            // b0015 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b0017 jne 0xb013b
            if fl.ne() { pc = 0xb013b; continue; }
            pc = 0xb001d;
        }
        0xb001d => {
            // block b001d
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b001d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b001d lea rdi,[rip+0x9885c] # 0x148880
            rdi = m.env.data_va(381005);
            // b0024 call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0029 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b002b je 0xb013b
            if fl.e() { pc = 0xb013b; continue; }
            pc = 0xb0031;
        }
        0xb0031 => {
            // block b0031
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0031 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0031 lea r14,[rip+0x98548] # 0x148580
            r14 = m.env.data_va(380237);
            // b0038 lea rsi,[rip+0x84e31] # 0x134e70
            rsi = m.env.data_va(300605);
            // b003f mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // b0044 mov rdi,r14
            rdi = r14;
            // b0047 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b004c lea rbx,[rip+0x98545] # 0x148598
            rbx = m.env.data_va(380261);
            // b0053 lea rsi,[rip+0x836c5] # 0x13371f
            rsi = m.env.data_va(294636);
            // b005a mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b005f mov rdi,rbx
            rdi = rbx;
            // b0062 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0067 lea rbx,[rip+0x98542] # 0x1485b0
            rbx = m.env.data_va(380285);
            // b006e lea rsi,[rip+0x836b0] # 0x133725
            rsi = m.env.data_va(294642);
            // b0075 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b007a mov rdi,rbx
            rdi = rbx;
            // b007d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0082 lea rbx,[rip+0x9853f] # 0x1485c8
            rbx = m.env.data_va(380309);
            // b0089 lea rsi,[rip+0x8369b] # 0x13372b
            rsi = m.env.data_va(294648);
            // b0090 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b0095 mov rdi,rbx
            rdi = rbx;
            // b0098 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b009d lea rbx,[rip+0x9853c] # 0x1485e0
            rbx = m.env.data_va(380333);
            // b00a4 lea rsi,[rip+0x83686] # 0x133731
            rsi = m.env.data_va(294654);
            // b00ab mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b00b0 mov rdi,rbx
            rdi = rbx;
            // b00b3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b00b8 lea rbx,[rip+0x98539] # 0x1485f8
            rbx = m.env.data_va(380357);
            // b00bf lea rsi,[rip+0x83671] # 0x133737
            rsi = m.env.data_va(294660);
            // b00c6 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b00cb mov rdi,rbx
            rdi = rbx;
            // b00ce call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b00d3 lea rbx,[rip+0x98536] # 0x148610
            rbx = m.env.data_va(380381);
            // b00da lea rsi,[rip+0x8365c] # 0x13373d
            rsi = m.env.data_va(294666);
            // b00e1 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b00e6 mov rdi,rbx
            rdi = rbx;
            // b00e9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b00ee lea rbx,[rip+0x98533] # 0x148628
            rbx = m.env.data_va(380405);
            // b00f5 lea rsi,[rip+0x83647] # 0x133743
            rsi = m.env.data_va(294672);
            // b00fc mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b0101 mov rdi,rbx
            rdi = rbx;
            // b0104 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0109 lea rdi,[rip+0x98530] # 0x148640
            rdi = m.env.data_va(380429);
            // b0110 mov esi,0x240
            rsi = (0x240u64) as u32 as u64;
            // b0115 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b011a lea rdi,[rip+0xfffffffffffb8670] # 0x68791
            rdi = m.env.code_va(0x68791);
            // b0121 lea rdx,[rip+0xfffffffffff4f048] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // b0128 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b012a call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b012f lea rdi,[rip+0x9874a] # 0x148880
            rdi = m.env.data_va(381005);
            // b0136 call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb013b;
        }
        0xb013b => {
            // block b013b
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b013b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b013b lea rsi,[rip+0x84d36] # 0x134e78
            rsi = m.env.data_va(300613);
            // b0142 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // b0149 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // b014e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0153 lea rcx,[rip+0x84d2b] # 0x134e85
            rcx = m.env.data_va(300626);
            // b015a lea rsi,[rip+0x84d2b] # 0x134e8c
            rsi = m.env.data_va(300633);
            // b0161 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // b0164 and edx,0x1
            { let r = (rdx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0167 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b016b add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // b016e xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // b0172 add rdx,0x6
            { let (r, f) = add::<64>(rdx, 0x6u64);
              rdx = r;
              fl = f; }
            // b0176 mov rdi,rax
            rdi = rax;
            // b0179 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b017e mov rbx,rax
            rbx = rax;
            // b0181 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0184 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0188 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b018b lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b018f call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0194 mov rsi,QWORD PTR [rip+0x8fff5] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b019b lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b019f call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b01a4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b01a7 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b01ab mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b01b0 mov rdi,rax
            rdi = rax;
            // b01b3 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b01b5 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b01b8 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b01bc call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b01c1 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b01c5 mov rdi,rbx
            rdi = rbx;
            // b01c8 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b01cd mov rdi,rbx
            rdi = rbx;
            // b01d0 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b01d5 lea rsi,[rip+0x84c9c] # 0x134e78
            rsi = m.env.data_va(300613);
            // b01dc mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // b01e1 mov rdi,rbx
            rdi = rbx;
            // b01e4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b01e9 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // b01ec and edx,0x2
            { let r = (rdx as u32 as u64) & 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b01ef lea rcx,[rip+0x84c9f] # 0x134e95
            rcx = m.env.data_va(300642);
            // b01f6 lea rsi,[rip+0x84c9f] # 0x134e9c
            rsi = m.env.data_va(300649);
            // b01fd cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b0201 xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // b0205 add rdx,0x6
            { let (r, f) = add::<64>(rdx, 0x6u64);
              rdx = r;
              fl = f; }
            // b0209 mov rdi,rax
            rdi = rax;
            // b020c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0211 mov rbx,rax
            rbx = rax;
            // b0214 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0217 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b021b add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b021e lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // b0222 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0227 mov rsi,QWORD PTR [rip+0x8ff62] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b022e lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // b0232 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0237 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b023a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b023e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0243 mov rdi,rax
            rdi = rax;
            // b0246 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0248 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b024b lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // b024f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0254 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b0258 mov rdi,rbx
            rdi = rbx;
            // b025b call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0260 mov rdi,rbx
            rdi = rbx;
            // b0263 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0268 lea rsi,[rip+0x84b5b] # 0x134dca
            rsi = m.env.data_va(300439);
            // b026f mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // b0274 mov rdi,rbx
            rdi = rbx;
            // b0277 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b027c mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // b027f and edx,0x4
            { let r = (rdx as u32 as u64) & 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0282 lea rcx,[rip+0x7a990] # 0x12ac19
            rcx = m.env.data_va(259046);
            // b0289 lea rsi,[rip+0x7a979] # 0x12ac09
            rsi = m.env.data_va(259030);
            // b0290 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b0294 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b0297 xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // b029b add rdx,0x3
            { let (r, f) = add::<64>(rdx, 0x3u64);
              rdx = r;
              fl = f; }
            // b029f mov rdi,rax
            rdi = rax;
            // b02a2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02a7 mov rbx,rax
            rbx = rax;
            // b02aa mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b02ad mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b02b1 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b02b4 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b02b8 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02bd mov rsi,QWORD PTR [rip+0x8fecc] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b02c4 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b02c8 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02cd mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b02d0 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b02d4 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b02d9 mov rdi,rax
            rdi = rax;
            // b02dc call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02de mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b02e1 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b02e5 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02ea movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b02ee mov rdi,rbx
            rdi = rbx;
            // b02f1 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02f6 mov rdi,rbx
            rdi = rbx;
            // b02f9 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b02fe lea rsi,[rip+0x84ba0] # 0x134ea5
            rsi = m.env.data_va(300658);
            // b0305 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // b030a mov rdi,rbx
            rdi = rbx;
            // b030d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0312 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // b0315 and edx,0x8
            { let r = (rdx as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0318 lea rcx,[rip+0x84add] # 0x134dfc
            rcx = m.env.data_va(300489);
            // b031f lea rsi,[rip+0x84add] # 0x134e03
            rsi = m.env.data_va(300496);
            // b0326 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b032a shr rdx,0x3
            { let (r, f) = shr::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // b032e add rdx,0x5
            { let (r, f) = add::<64>(rdx, 0x5u64);
              rdx = r;
              fl = f; }
            // b0332 mov rdi,rax
            rdi = rax;
            // b0335 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b033a mov rbx,rax
            rbx = rax;
            // b033d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0340 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0344 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b0347 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b034b call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0350 mov rsi,QWORD PTR [rip+0x8fe39] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0357 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b035b call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0360 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0363 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0367 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b036c mov rdi,rax
            rdi = rax;
            // b036f call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0371 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0374 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b0378 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b037d movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b0381 mov rdi,rbx
            rdi = rbx;
            // b0384 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0389 mov rdi,rbx
            rdi = rbx;
            // b038c call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0391 lea rsi,[rip+0x84aa8] # 0x134e40
            rsi = m.env.data_va(300557);
            // b0398 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // b039d mov rdi,rbx
            rdi = rbx;
            // b03a0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03a5 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // b03a8 test ch,0x10
            fl = logic::<8>(((rcx >> 8) as u8 as u64) & 0x10u64);
            // b03ab mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // b03b0 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // b03b5 cmovne esi,ecx
            if fl.ne() { rsi = ((rcx as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // b03b8 mov rdi,rax
            rdi = rax;
            // b03bb call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03c0 mov rbx,rax
            rbx = rax;
            // b03c3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b03c6 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b03ca add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b03cd lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b03d1 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03d6 mov rsi,QWORD PTR [rip+0x8fdb3] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b03dd lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b03e1 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03e6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b03e9 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b03ed mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b03f2 mov rdi,rax
            rdi = rax;
            // b03f5 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03f7 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b03fa lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b03fe call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0403 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b0407 mov rdi,rbx
            rdi = rbx;
            // b040a call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b040f mov rdi,rbx
            rdi = rbx;
            // b0412 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0417 lea rsi,[rip+0x849c6] # 0x134de4
            rsi = m.env.data_va(300465);
            // b041e mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // b0423 mov rdi,rbx
            rdi = rbx;
            // b0426 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b042b mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // b042e and edx,0x2000
            { let r = (rdx as u32 as u64) & 0x2000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0434 lea rcx,[rip+0x83e8a] # 0x1342c5
            rcx = m.env.data_va(297618);
            // b043b lea rsi,[rip+0x83e8f] # 0x1342d1
            rsi = m.env.data_va(297630);
            // b0442 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b0446 shr rdx,0xd
            { let (r, f) = shr::<64>(rdx, 0xdu64, fl); rdx = r; fl = f; }
            // b044a or rdx,0xa
            { let r = rdx | 0xau64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // b044e mov rdi,rax
            rdi = rax;
            // b0451 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0456 mov rbx,rax
            rbx = rax;
            // b0459 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b045c mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0460 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b0463 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0467 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b046c mov rsi,QWORD PTR [rip+0x8fd1d] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0473 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0477 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b047c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b047f mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0483 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0488 mov rdi,rax
            rdi = rax;
            // b048b call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b048d mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0490 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0494 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0499 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b049d mov rdi,rbx
            rdi = rbx;
            // b04a0 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b04a5 mov rdi,rbx
            rdi = rbx;
            // b04a8 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b04ad lea rsi,[rip+0x835a5] # 0x133a59
            rsi = m.env.data_va(295462);
            // b04b4 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // b04b9 mov rdi,rbx
            rdi = rbx;
            // b04bc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b04c1 lea rcx,[rip+0x849e8] # 0x134eb0
            rcx = m.env.data_va(300669);
            // b04c8 lea rsi,[rip+0x849e4] # 0x134eb3
            rsi = m.env.data_va(300672);
            // b04cf mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // b04d2 test dh,0x40
            fl = logic::<8>(((rdx >> 8) as u8 as u64) & 0x40u64);
            // b04d5 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b04d9 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // b04de mov rdi,rax
            rdi = rax;
            // b04e1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b04e6 mov rbx,rax
            rbx = rax;
            // b04e9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b04ec mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b04f0 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b04f3 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b04f7 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b04fc mov rsi,QWORD PTR [rip+0x8fc8d] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0503 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b0507 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b050c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b050f mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0513 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0518 mov rdi,rax
            rdi = rax;
            // b051b call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b051d mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0520 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b0524 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0529 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b052d mov rdi,rbx
            rdi = rbx;
            // b0530 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0535 mov rdi,rbx
            rdi = rbx;
            // b0538 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b053d mov eax,DWORD PTR [rbp-0x1b8]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64) as u32 as u64;
            // b0543 dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // b0545 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // b0548 ja 0xb05d7
            if fl.a() { pc = 0xb05d7; continue; }
            pc = 0xb054e;
        }
        0xb054e => {
            // block b054e
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b054e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b054e lea rsi,[rip+0x84864] # 0x134db9
            rsi = m.env.data_va(300422);
            // b0555 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // b055c mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // b0561 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0566 mov ecx,r13d
            rcx = ((r13 as u32 as u64)) as u32 as u64;
            // b0569 lea rcx,[rcx+rcx*2]
            rcx = rcx.wrapping_add(rcx.wrapping_mul(2));
            // b056d lea rdx,[rip+0x97cfc] # 0x148270
            rdx = m.env.data_va(379453);
            // b0574 lea rsi,[rdx+rcx*8]
            rsi = rdx.wrapping_add(rcx.wrapping_mul(8));
            // b0578 mov rdi,rax
            rdi = rax;
            // b057b call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0580 mov rbx,rax
            rbx = rax;
            // b0583 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0586 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b058a add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b058d lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b0591 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0596 mov rsi,QWORD PTR [rip+0x8fbf3] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b059d lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b05a1 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b05a6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b05a9 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b05ad mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b05b2 mov rdi,rax
            rdi = rax;
            // b05b5 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b05b7 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b05ba lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b05be call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b05c3 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b05c7 mov rdi,rbx
            rdi = rbx;
            // b05ca call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b05cf mov rdi,rbx
            rdi = rbx;
            // b05d2 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb05d7;
        }
        0xb05d7 => {
            // block b05d7
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b05d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b05d7 lea rsi,[rip+0x848d8] # 0x134eb6
            rsi = m.env.data_va(300675);
            // b05de lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // b05e5 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // b05ea call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b05ef mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // b05f2 and ecx,0x8000000
            { let r = (rcx as u32 as u64) & 0x8000000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b05f8 lea rdx,[rip+0x8485d] # 0x134e5c
            rdx = m.env.data_va(300585);
            // b05ff lea rsi,[rip+0x7f0cf] # 0x12f6d5
            rsi = m.env.data_va(278178);
            // b0606 cmovne rsi,rdx
            if fl.ne() { rsi = rdx; }
            // b060a shr rcx,0x1b
            { let (r, f) = shr::<64>(rcx, 0x1bu64, fl); rcx = r; fl = f; }
            // b060e mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // b0613 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // b0616 mov rdi,rax
            rdi = rax;
            // b0619 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b061e mov rbx,rax
            rbx = rax;
            // b0621 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0624 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0628 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b062b lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b062f call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0634 mov rsi,QWORD PTR [rip+0x8fb55] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b063b lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b063f call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0644 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0647 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b064b mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0650 mov rdi,rax
            rdi = rax;
            // b0653 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0655 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0658 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b065c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0661 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // b0665 mov rdi,rbx
            rdi = rbx;
            // b0668 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b066d mov rdi,rbx
            rdi = rbx;
            // b0670 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0675 lea rsi,[rip+0x84846] # 0x134ec2
            rsi = m.env.data_va(300687);
            // b067c mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // b0681 mov rdi,rbx
            rdi = rbx;
            // b0684 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0689 mov rbx,rax
            rbx = rax;
            // b068c test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // b068f js 0xb06b5
            if fl.s() { pc = 0xb06b5; continue; }
            pc = 0xb0691;
        }
        0xb0691 => {
            // block b0691
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0691 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0691 shr r15d,0x1c
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x1cu64, fl); r15 = (r) as u32 as u64; fl = f; }
            // b0695 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // b0698 lea rax,[rax+rax*2]
            rax = rax.wrapping_add(rax.wrapping_mul(2));
            // b069c lea rcx,[rip+0x97edd] # 0x148580
            rcx = m.env.data_va(380237);
            // b06a3 lea rsi,[rcx+rax*8]
            rsi = rcx.wrapping_add(rax.wrapping_mul(8));
            // b06a7 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // b06ae call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b06b3 jmp 0xb06cd
            pc = 0xb06cd; continue;
        }
        0xb06b5 => {
            // block b06b5
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b06b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b06b5 lea rsi,[rip+0x847b4] # 0x134e70
            rsi = m.env.data_va(300605);
            // b06bc lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // b06c3 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // b06c8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb06cd;
        }
        0xb06cd => {
            // block b06cd
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b06cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b06cd movzx edx,BYTE PTR [rbp-0x1b0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64) as u32 as u64;
            // b06d4 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b06d7 jne 0xb06e5
            if fl.ne() { pc = 0xb06e5; continue; }
            pc = 0xb06d9;
        }
        0xb06d9 => {
            // block b06d9
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b06d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b06d9 lea rsi,[rbp-0x1af]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe51);
            // b06e0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b06e3 jmp 0xb06f3
            pc = 0xb06f3; continue;
        }
        0xb06e5 => {
            // block b06e5
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b06e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b06e5 mov rdx,QWORD PTR [rbp-0x1a8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // b06ec mov rsi,QWORD PTR [rbp-0x1a0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            pc = 0xb06f3;
        }
        0xb06f3 => {
            // block b06f3
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b06f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b06f3 mov rdi,rbx
            rdi = rbx;
            // b06f6 lea rbx,[rbp-0x190]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // b06fd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0702 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // b0709 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b070e mov rdi,QWORD PTR [rbp-0x1c0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // b0715 mov rsi,rbx
            rsi = rbx;
            // b0718 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b071d mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // b0724 mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // b072b mov rax,QWORD PTR [rbp-0x1c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64;
            // b0732 mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // b0739 mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // b0740 mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // b0747 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b074e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0753 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // b075a call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b075f mov rsi,QWORD PTR [rip+0x8faa2] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b0766 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b076a lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // b0771 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0776 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // b077d call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0782 mov rax,QWORD PTR [rbp-0x1c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // b0789 add rsp,0x1b8
            { let (r, f) = add::<64>(rsp, 0x1b8u64);
              rsp = r;
              fl = f; }
            // b0790 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b0791 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b0793 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b0795 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b0797 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b0799 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b079a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb079b => {
            // block b079b
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b079b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b079b mov r15,rax
            r15 = rax;
            pc = 0xb079e;
        }
        0xb079e => {
            // block b079e
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b079e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b079e mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // b07a5 mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // b07ac mov rax,QWORD PTR [rbp-0x1c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64;
            // b07b3 mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // b07ba mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // b07c1 mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // b07c8 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b07cf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b07d4 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // b07db call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b07e0 mov rsi,QWORD PTR [rip+0x8fa21] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b07e7 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b07eb lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // b07f2 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b07f7 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            pc = 0xb07fe;
        }
        0xb07fe => {
            // block b07fe
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b07fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b07fe call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0803 mov rdi,r15
            rdi = r15;
            // b0806 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b080b mov r15,rax
            r15 = rax;
            pc = 0xb080e;
        }
        0xb080e => {
            // block b080e
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b080e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b080e add r12,0xffffffffffffffe8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffe8u64);
              r12 = r;
              fl = f; }
            // b0812 mov rdi,r12
            rdi = r12;
            // b0815 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b081a cmp r14,r12
            fl = sub::<64>(r14, r12).1;
            // b081d jne 0xb080e
            if fl.ne() { pc = 0xb080e; continue; }
            pc = 0xb081f;
        }
        0xb081f => {
            // block b081f
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b081f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b081f jmp 0xb0952
            pc = 0xb0952; continue;
        }
        0xb0824 => {
            // block b0824
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0824 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0824 mov r15,rax
            r15 = rax;
            // b0827 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b082b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0830 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb0835 => {
            // block b0835
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0835 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0835 mov r15,rax
            r15 = rax;
            // b0838 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // b083c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0841 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb0846 => {
            // block b0846
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0846 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0846 mov r15,rax
            r15 = rax;
            // b0849 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b084d call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0852 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb0857 => {
            // block b0857
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0857 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0857 mov r15,rax
            r15 = rax;
            // b085a lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b085e call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0863 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb0868 => {
            // block b0868
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0868 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0868 mov r15,rax
            r15 = rax;
            // b086b lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b086f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0874 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb0879 => {
            // block b0879
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0879 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0879 mov r15,rax
            r15 = rax;
            // b087c lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0880 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0885 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb088a => {
            // block b088a
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b088a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b088a mov r15,rax
            r15 = rax;
            // b088d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b0891 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0896 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb089b => {
            // block b089b
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b089b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b089b mov r15,rax
            r15 = rax;
            // b089e lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b08a2 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b08a7 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb08ac => {
            // block b08ac
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b08ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b08ac mov r15,rax
            r15 = rax;
            pc = 0xb08af;
        }
        0xb08af => {
            // block b08af
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b08af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b08af add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // b08b3 mov rdi,rbx
            rdi = rbx;
            // b08b6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b08bb cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // b08be jne 0xb08af
            if fl.ne() { pc = 0xb08af; continue; }
            pc = 0xb08c0;
        }
        0xb08c0 => {
            // block b08c0
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b08c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b08c0 jmp 0xb0966
            pc = 0xb0966; continue;
        }
        0xb08c5 => {
            // block b08c5
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b08c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b08c5 lea rbx,[rbp-0x128]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // b08cc mov r15,rax
            r15 = rax;
            // b08cf jmp 0xb0922
            pc = 0xb0922; continue;
        }
        0xb08d1 => {
            // block b08d1
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b08d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b08d1 lea rbx,[rbp-0x128]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // b08d8 mov r15,rax
            r15 = rax;
            // b08db jmp 0xb090b
            pc = 0xb090b; continue;
        }
        0xb08dd => {
            // block b08dd
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b08dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b08dd lea rbx,[rbp-0x128]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // b08e4 mov r15,rax
            r15 = rax;
            // b08e7 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // b08ee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b08f3 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b08fa call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b08ff lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // b0906 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb090b;
        }
        0xb090b => {
            // block b090b
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b090b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b090b mov rsi,QWORD PTR [rip+0x8f8f6] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b0912 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b0916 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // b091d call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb0922;
        }
        0xb0922 => {
            // block b0922
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0922 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0922 mov rdi,rbx
            rdi = rbx;
            // b0925 jmp 0xb07fe
            pc = 0xb07fe; continue;
        }
        0xb092a => {
            // block b092a
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b092a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b092a mov r15,rax
            r15 = rax;
            // b092d lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b0931 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0936 jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb093b => {
            // block b093b
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b093b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b093b mov r15,rax
            r15 = rax;
            // b093e lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // b0945 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b094a jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb094f => {
            // block b094f
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b094f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b094f mov r15,rax
            r15 = rax;
            pc = 0xb0952;
        }
        0xb0952 => {
            // block b0952
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0952 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0952 lea rdi,[rip+0x97c17] # 0x148570
            rdi = m.env.data_va(380221);
            // b0959 call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b095e jmp 0xb079e
            pc = 0xb079e; continue;
        }
        0xb0963 => {
            // block b0963
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0963 mov r15,rax
            r15 = rax;
            pc = 0xb0966;
        }
        0xb0966 => {
            // block b0966
            #[cfg(feature = "trace")] eprintln!("f_000afd60 pc b0966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0966 lea rdi,[rip+0x97f13] # 0x148880
            rdi = m.env.data_va(381005);
            // b096d call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0972 jmp 0xb079e
            pc = 0xb079e; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xaef04 size=3666
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000aef04<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xaef04;
    loop {
        match pc {
        0xaef04 => {
            // block aef04
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc aef04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aef04 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // aef05 mov rbp,rsp
            rbp = rsp;
            // aef08 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // aef0a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // aef0c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // aef0e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // aef10 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // aef11 sub rsp,0x1b8
            { let (r, f) = sub::<64>(rsp, 0x1b8u64);
              rsp = r;
              fl = f; }
            // aef18 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // aef1b mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // aef1e mov r13,rdi
            r13 = rdi;
            // aef21 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // aef28 lea r14,[rbp-0x190]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // aef2f mov rax,QWORD PTR [rip+0x9129a] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // aef36 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // aef3a mov QWORD PTR [rbp-0x198],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rcx) as u64);
            // aef41 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // aef45 mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // aef4c mov rsi,r14
            rsi = r14;
            // aef4f call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aef54 mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // aef5f mov DWORD PTR [rbp-0x98],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0xffffffffu64) as u32);
            // aef69 mov rbx,QWORD PTR [rip+0x91450] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // aef70 lea rax,[rbx+0x18]
            rax = rbx.wrapping_add(0x18);
            // aef74 mov QWORD PTR [rbp-0x1d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe30), (rax) as u64);
            // aef7b mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // aef82 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // aef86 mov QWORD PTR [rbp-0x128],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rbx) as u64);
            // aef8d mov rdi,r14
            rdi = r14;
            // aef90 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aef95 mov QWORD PTR [rbp-0x1e0],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe20), (rbx) as u64);
            // aef9c mov rax,QWORD PTR [rip+0x91405] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // aefa3 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // aefa7 mov QWORD PTR [rbp-0x1d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe28), (rax) as u64);
            // aefae mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // aefb5 mov QWORD PTR [rbp-0x138],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (0x0u64) as u64);
            // aefc0 mov QWORD PTR [rbp-0x140],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (0x0u64) as u64);
            // aefcb mov QWORD PTR [rbp-0x148],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (0x0u64) as u64);
            // aefd6 mov QWORD PTR [rbp-0x150],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (0x0u64) as u64);
            // aefe1 mov DWORD PTR [rbp-0x130],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (0x10u64) as u32);
            // aefeb xorps xmm0,xmm0
            x0 = 0;
            // aefee movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // aeff5 mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // aeffd lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // af004 mov rdi,r14
            rdi = r14;
            // af007 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af00c lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // af013 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af018 mov al,BYTE PTR [rip+0x98fda] # 0x147ff8
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(378821)) as u64) as u8 as u64);
            // af01e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // af020 jne 0xaf115
            if fl.ne() { pc = 0xaf115; continue; }
            pc = 0xaf026;
        }
        0xaf026 => {
            // block af026
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af026 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af026 lea rdi,[rip+0x98fcb] # 0x147ff8
            rdi = m.env.data_va(378821);
            // af02d call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af032 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // af034 je 0xaf115
            if fl.e() { pc = 0xaf115; continue; }
            pc = 0xaf03a;
        }
        0xaf03a => {
            // block af03a
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af03a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af03a lea r14,[rip+0x98f0f] # 0x147f50
            r14 = m.env.data_va(378653);
            // af041 lea rsi,[rip+0x80707] # 0x12f74f
            rsi = m.env.data_va(278300);
            // af048 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af04d mov rdi,r14
            rdi = r14;
            // af050 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af055 lea rbx,[rip+0x98f0c] # 0x147f68
            rbx = m.env.data_va(378677);
            // af05c lea rsi,[rip+0x80705] # 0x12f768
            rsi = m.env.data_va(278325);
            // af063 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af068 mov rdi,rbx
            rdi = rbx;
            // af06b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af070 lea rbx,[rip+0x98f09] # 0x147f80
            rbx = m.env.data_va(378701);
            // af077 lea rsi,[rip+0x85d14] # 0x134d92
            rsi = m.env.data_va(300383);
            // af07e mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af083 mov rdi,rbx
            rdi = rbx;
            // af086 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af08b lea rbx,[rip+0x98f06] # 0x147f98
            rbx = m.env.data_va(378725);
            // af092 lea rsi,[rip+0x85cfe] # 0x134d97
            rsi = m.env.data_va(300388);
            // af099 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af09e mov rdi,rbx
            rdi = rbx;
            // af0a1 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af0a6 lea rbx,[rip+0x98f03] # 0x147fb0
            rbx = m.env.data_va(378749);
            // af0ad lea rsi,[rip+0x806f9] # 0x12f7ad
            rsi = m.env.data_va(278394);
            // af0b4 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af0b9 mov rdi,rbx
            rdi = rbx;
            // af0bc call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af0c1 lea rbx,[rip+0x98f00] # 0x147fc8
            rbx = m.env.data_va(378773);
            // af0c8 lea rsi,[rip+0x85ccd] # 0x134d9c
            rsi = m.env.data_va(300393);
            // af0cf mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af0d4 mov rdi,rbx
            rdi = rbx;
            // af0d7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af0dc lea rbx,[rip+0x98efd] # 0x147fe0
            rbx = m.env.data_va(378797);
            // af0e3 lea rsi,[rip+0x89bde] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af0ea xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af0ec mov rdi,rbx
            rdi = rbx;
            // af0ef call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af0f4 lea rdi,[rip+0xfffffffffffb95ea] # 0x686e5
            rdi = m.env.code_va(0x686e5);
            // af0fb lea rdx,[rip+0xfffffffffff5006e] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // af102 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // af104 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af109 lea rdi,[rip+0x98ee8] # 0x147ff8
            rdi = m.env.data_va(378821);
            // af110 call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaf115;
        }
        0xaf115 => {
            // block af115
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af115 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af115 mov al,BYTE PTR [rip+0x98fed] # 0x148108
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(379093)) as u64) as u8 as u64);
            // af11b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // af11d jne 0xaf27e
            if fl.ne() { pc = 0xaf27e; continue; }
            pc = 0xaf123;
        }
        0xaf123 => {
            // block af123
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af123 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af123 lea rdi,[rip+0x98fde] # 0x148108
            rdi = m.env.data_va(379093);
            // af12a call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af12f test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // af131 je 0xaf27e
            if fl.e() { pc = 0xaf27e; continue; }
            pc = 0xaf137;
        }
        0xaf137 => {
            // block af137
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af137 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af137 lea rbx,[rip+0x98ec2] # 0x148000
            rbx = m.env.data_va(378829);
            // af13e lea rsi,[rip+0x8060a] # 0x12f74f
            rsi = m.env.data_va(278300);
            // af145 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af14a mov rdi,rbx
            rdi = rbx;
            // af14d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af152 lea r14,[rip+0x98ebf] # 0x148018
            r14 = m.env.data_va(378853);
            // af159 lea rsi,[rip+0x80608] # 0x12f768
            rsi = m.env.data_va(278325);
            // af160 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af165 mov rdi,r14
            rdi = r14;
            // af168 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af16d lea r14,[rip+0x98ebc] # 0x148030
            r14 = m.env.data_va(378877);
            // af174 lea rsi,[rip+0x85c17] # 0x134d92
            rsi = m.env.data_va(300383);
            // af17b mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af180 mov rdi,r14
            rdi = r14;
            // af183 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af188 lea r14,[rip+0x98eb9] # 0x148048
            r14 = m.env.data_va(378901);
            // af18f lea rsi,[rip+0x85c01] # 0x134d97
            rsi = m.env.data_va(300388);
            // af196 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // af19b mov rdi,r14
            rdi = r14;
            // af19e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af1a3 lea r14,[rip+0x98eb6] # 0x148060
            r14 = m.env.data_va(378925);
            // af1aa lea rsi,[rip+0x805fc] # 0x12f7ad
            rsi = m.env.data_va(278394);
            // af1b1 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af1b6 mov rdi,r14
            rdi = r14;
            // af1b9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af1be lea r14,[rip+0x98eb3] # 0x148078
            r14 = m.env.data_va(378949);
            // af1c5 lea rsi,[rip+0x85bd5] # 0x134da1
            rsi = m.env.data_va(300398);
            // af1cc mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af1d1 mov rdi,r14
            rdi = r14;
            // af1d4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af1d9 lea r14,[rip+0x98eb0] # 0x148090
            r14 = m.env.data_va(378973);
            // af1e0 lea rsi,[rip+0x85bc0] # 0x134da7
            rsi = m.env.data_va(300404);
            // af1e7 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // af1ec mov rdi,r14
            rdi = r14;
            // af1ef call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af1f4 lea r14,[rip+0x98ead] # 0x1480a8
            r14 = m.env.data_va(378997);
            // af1fb lea rsi,[rip+0x85bae] # 0x134db0
            rsi = m.env.data_va(300413);
            // af202 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // af207 mov rdi,r14
            rdi = r14;
            // af20a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af20f lea r14,[rip+0x98eaa] # 0x1480c0
            r14 = m.env.data_va(379021);
            // af216 lea rsi,[rip+0x805fc] # 0x12f819
            rsi = m.env.data_va(278502);
            // af21d mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // af222 mov rdi,r14
            rdi = r14;
            // af225 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af22a lea r14,[rip+0x98ea7] # 0x1480d8
            r14 = m.env.data_va(379045);
            // af231 lea rsi,[rip+0x805fe] # 0x12f836
            rsi = m.env.data_va(278531);
            // af238 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // af23d mov rdi,r14
            rdi = r14;
            // af240 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af245 lea r14,[rip+0x98ea4] # 0x1480f0
            r14 = m.env.data_va(379069);
            // af24c lea rsi,[rip+0x89a75] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af253 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af255 mov rdi,r14
            rdi = r14;
            // af258 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af25d lea rdi,[rip+0xfffffffffffb94ac] # 0x68710
            rdi = m.env.code_va(0x68710);
            // af264 lea rdx,[rip+0xfffffffffff4ff05] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // af26b xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // af26d call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af272 lea rdi,[rip+0x98e8f] # 0x148108
            rdi = m.env.data_va(379093);
            // af279 call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaf27e;
        }
        0xaf27e => {
            // block af27e
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af27e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af27e mov al,BYTE PTR [rip+0x98f94] # 0x148218
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(379365)) as u64) as u8 as u64);
            // af284 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // af286 jne 0xaf3de
            if fl.ne() { pc = 0xaf3de; continue; }
            pc = 0xaf28c;
        }
        0xaf28c => {
            // block af28c
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af28c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af28c lea rdi,[rip+0x98f85] # 0x148218
            rdi = m.env.data_va(379365);
            // af293 call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af298 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // af29a je 0xaf3de
            if fl.e() { pc = 0xaf3de; continue; }
            pc = 0xaf2a0;
        }
        0xaf2a0 => {
            // block af2a0
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af2a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af2a0 lea r14,[rip+0x98e69] # 0x148110
            r14 = m.env.data_va(379101);
            // af2a7 lea rsi,[rip+0x89a1a] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af2ae xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af2b0 mov rdi,r14
            rdi = r14;
            // af2b3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af2b8 lea rbx,[rip+0x98e69] # 0x148128
            rbx = m.env.data_va(379125);
            // af2bf lea rsi,[rip+0x84459] # 0x13371f
            rsi = m.env.data_va(294636);
            // af2c6 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af2cb mov rdi,rbx
            rdi = rbx;
            // af2ce call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af2d3 lea rbx,[rip+0x98e66] # 0x148140
            rbx = m.env.data_va(379149);
            // af2da lea rsi,[rip+0x84444] # 0x133725
            rsi = m.env.data_va(294642);
            // af2e1 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af2e6 mov rdi,rbx
            rdi = rbx;
            // af2e9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af2ee lea rbx,[rip+0x98e63] # 0x148158
            rbx = m.env.data_va(379173);
            // af2f5 lea rsi,[rip+0x8442f] # 0x13372b
            rsi = m.env.data_va(294648);
            // af2fc mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af301 mov rdi,rbx
            rdi = rbx;
            // af304 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af309 lea rbx,[rip+0x98e60] # 0x148170
            rbx = m.env.data_va(379197);
            // af310 lea rsi,[rip+0x8441a] # 0x133731
            rsi = m.env.data_va(294654);
            // af317 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af31c mov rdi,rbx
            rdi = rbx;
            // af31f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af324 lea rbx,[rip+0x98e5d] # 0x148188
            rbx = m.env.data_va(379221);
            // af32b lea rsi,[rip+0x84405] # 0x133737
            rsi = m.env.data_va(294660);
            // af332 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af337 mov rdi,rbx
            rdi = rbx;
            // af33a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af33f lea rbx,[rip+0x98e5a] # 0x1481a0
            rbx = m.env.data_va(379245);
            // af346 lea rsi,[rip+0x843f0] # 0x13373d
            rsi = m.env.data_va(294666);
            // af34d mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af352 mov rdi,rbx
            rdi = rbx;
            // af355 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af35a lea rbx,[rip+0x98e57] # 0x1481b8
            rbx = m.env.data_va(379269);
            // af361 lea rsi,[rip+0x843db] # 0x133743
            rsi = m.env.data_va(294672);
            // af368 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af36d mov rdi,rbx
            rdi = rbx;
            // af370 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af375 lea rbx,[rip+0x98e54] # 0x1481d0
            rbx = m.env.data_va(379293);
            // af37c lea rsi,[rip+0x89945] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af383 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af385 mov rdi,rbx
            rdi = rbx;
            // af388 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af38d lea rbx,[rip+0x98e54] # 0x1481e8
            rbx = m.env.data_va(379317);
            // af394 lea rsi,[rip+0x8992d] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af39b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af39d mov rdi,rbx
            rdi = rbx;
            // af3a0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af3a5 lea rbx,[rip+0x98e54] # 0x148200
            rbx = m.env.data_va(379341);
            // af3ac lea rsi,[rip+0x89915] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af3b3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af3b5 mov rdi,rbx
            rdi = rbx;
            // af3b8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af3bd lea rdi,[rip+0xfffffffffffb9377] # 0x6873b
            rdi = m.env.code_va(0x6873b);
            // af3c4 lea rdx,[rip+0xfffffffffff4fda5] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // af3cb xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // af3cd call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af3d2 lea rdi,[rip+0x98e3f] # 0x148218
            rdi = m.env.data_va(379365);
            // af3d9 call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaf3de;
        }
        0xaf3de => {
            // block af3de
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af3de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af3de mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // af3e1 call 0xce64c
            { let r = { let t = m.env.code_va(0xce64c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af3e6 and eax,0x3
            { let r = (rax as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // af3e9 lea rcx,[rip+0x53080] # 0x102470
            rcx = m.env.data_va(93245);
            // af3f0 mov ebx,DWORD PTR [rcx+rax*4]
            rbx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // af3f3 and ebx,r12d
            { let r = (rbx as u32 as u64) & (r12 as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // af3f6 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // af3f9 jb 0xaf41a
            if fl.b() { pc = 0xaf41a; continue; }
            pc = 0xaf3fb;
        }
        0xaf3fb => {
            // block af3fb
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af3fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af3fb mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // af3fd lea rax,[rax+rax*2]
            rax = rax.wrapping_add(rax.wrapping_mul(2));
            // af401 lea rcx,[rip+0x98bf8] # 0x148000
            rcx = m.env.data_va(378829);
            // af408 lea rsi,[rcx+rax*8]
            rsi = rcx.wrapping_add(rax.wrapping_mul(8));
            // af40c lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // af413 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af418 jmp 0xaf453
            pc = 0xaf453; continue;
        }
        0xaf41a => {
            // block af41a
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af41a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af41a cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // af41d jne 0xaf43e
            if fl.ne() { pc = 0xaf43e; continue; }
            pc = 0xaf41f;
        }
        0xaf41f => {
            // block af41f
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af41f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af41f mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // af421 lea rax,[rax+rax*2]
            rax = rax.wrapping_add(rax.wrapping_mul(2));
            // af425 lea rcx,[rip+0x98b24] # 0x147f50
            rcx = m.env.data_va(378653);
            // af42c lea rsi,[rcx+rax*8]
            rsi = rcx.wrapping_add(rax.wrapping_mul(8));
            // af430 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // af437 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af43c jmp 0xaf453
            pc = 0xaf453; continue;
        }
        0xaf43e => {
            // block af43e
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af43e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af43e lea rsi,[rip+0x89883] # 0x138cc8
            rsi = m.env.data_va(316565);
            // af445 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // af44c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af44e call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaf453;
        }
        0xaf453 => {
            // block af453
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af453 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af453 lea rdi,[rbp-0x1c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe38);
            // af45a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // af45f mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // af461 call 0x5b5c9
            { let r = { let t = m.env.code_va(0x5b5c9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af466 lea rsi,[rip+0x8594c] # 0x134db9
            rsi = m.env.data_va(300422);
            // af46d lea rbx,[rbp-0x198]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // af474 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // af479 mov rdi,rbx
            rdi = rbx;
            // af47c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af481 movzx edx,BYTE PTR [rbp-0x1b0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64) as u32 as u64;
            // af488 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // af48b jne 0xaf499
            if fl.ne() { pc = 0xaf499; continue; }
            pc = 0xaf48d;
        }
        0xaf48d => {
            // block af48d
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af48d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af48d lea rsi,[rbp-0x1af]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe51);
            // af494 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // af497 jmp 0xaf4a7
            pc = 0xaf4a7; continue;
        }
        0xaf499 => {
            // block af499
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af499 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af499 mov rdx,QWORD PTR [rbp-0x1a8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // af4a0 mov rsi,QWORD PTR [rbp-0x1a0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            pc = 0xaf4a7;
        }
        0xaf4a7 => {
            // block af4a7
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4a7 mov rdi,rax
            rdi = rax;
            // af4aa call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af4af movzx edx,BYTE PTR [rbp-0x1b0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64) as u32 as u64;
            // af4b6 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // af4b8 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // af4ba jne 0xaf4c1
            if fl.ne() { pc = 0xaf4c1; continue; }
            pc = 0xaf4bc;
        }
        0xaf4bc => {
            // block af4bc
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4bc shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // af4bf jmp 0xaf4c8
            pc = 0xaf4c8; continue;
        }
        0xaf4c1 => {
            // block af4c1
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4c1 mov rdx,QWORD PTR [rbp-0x1a8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            pc = 0xaf4c8;
        }
        0xaf4c8 => {
            // block af4c8
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4c8 movzx esi,BYTE PTR [rbp-0x1c8]
            rsi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64) as u32 as u64;
            // af4cf mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // af4d2 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // af4d5 jne 0xaf4dc
            if fl.ne() { pc = 0xaf4dc; continue; }
            pc = 0xaf4d7;
        }
        0xaf4d7 => {
            // block af4d7
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4d7 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // af4da jmp 0xaf4e3
            pc = 0xaf4e3; continue;
        }
        0xaf4dc => {
            // block af4dc
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4dc mov rsi,QWORD PTR [rbp-0x1c0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            pc = 0xaf4e3;
        }
        0xaf4e3 => {
            // block af4e3
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4e3 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // af4e6 jne 0xaf53e
            if fl.ne() { pc = 0xaf53e; continue; }
            pc = 0xaf4e8;
        }
        0xaf4e8 => {
            // block af4e8
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4e8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // af4ea jne 0xaf4f5
            if fl.ne() { pc = 0xaf4f5; continue; }
            pc = 0xaf4ec;
        }
        0xaf4ec => {
            // block af4ec
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4ec lea rdi,[rbp-0x1af]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe51);
            // af4f3 jmp 0xaf4fc
            pc = 0xaf4fc; continue;
        }
        0xaf4f5 => {
            // block af4f5
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4f5 mov rdi,QWORD PTR [rbp-0x1a0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            pc = 0xaf4fc;
        }
        0xaf4fc => {
            // block af4fc
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af4fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af4fc test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // af4fe jne 0xaf509
            if fl.ne() { pc = 0xaf509; continue; }
            pc = 0xaf500;
        }
        0xaf500 => {
            // block af500
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af500 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af500 lea rsi,[rbp-0x1c7]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe39);
            // af507 jmp 0xaf510
            pc = 0xaf510; continue;
        }
        0xaf509 => {
            // block af509
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af509 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af509 mov rsi,QWORD PTR [rbp-0x1b8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            pc = 0xaf510;
        }
        0xaf510 => {
            // block af510
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af510 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af510 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // af512 jne 0xaf530
            if fl.ne() { pc = 0xaf530; continue; }
            pc = 0xaf514;
        }
        0xaf514 => {
            // block af514
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af514 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af514 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // af517 je 0xaf598
            if fl.e() { pc = 0xaf598; continue; }
            pc = 0xaf519;
        }
        0xaf519 => {
            // block af519
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af519 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af519 movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // af51c movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // af51f cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // af521 jne 0xaf53e
            if fl.ne() { pc = 0xaf53e; continue; }
            pc = 0xaf523;
        }
        0xaf523 => {
            // block af523
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af523 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af523 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // af526 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // af529 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // af52c jne 0xaf519
            if fl.ne() { pc = 0xaf519; continue; }
            pc = 0xaf52e;
        }
        0xaf52e => {
            // block af52e
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af52e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af52e jmp 0xaf598
            pc = 0xaf598; continue;
        }
        0xaf530 => {
            // block af530
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af530 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af530 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // af533 je 0xaf598
            if fl.e() { pc = 0xaf598; continue; }
            pc = 0xaf535;
        }
        0xaf535 => {
            // block af535
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af535 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af535 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af53a test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // af53c je 0xaf598
            if fl.e() { pc = 0xaf598; continue; }
            pc = 0xaf53e;
        }
        0xaf53e => {
            // block af53e
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af53e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af53e lea rsi,[rip+0x84a22] # 0x133f67
            rsi = m.env.data_va(296756);
            // af545 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // af54c mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // af551 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af556 movzx edx,BYTE PTR [rbp-0x1c8]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64) as u32 as u64;
            // af55d test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // af560 jne 0xaf56e
            if fl.ne() { pc = 0xaf56e; continue; }
            pc = 0xaf562;
        }
        0xaf562 => {
            // block af562
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af562 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af562 lea rsi,[rbp-0x1c7]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe39);
            // af569 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // af56c jmp 0xaf57c
            pc = 0xaf57c; continue;
        }
        0xaf56e => {
            // block af56e
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af56e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af56e mov rdx,QWORD PTR [rbp-0x1c0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // af575 mov rsi,QWORD PTR [rbp-0x1b8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            pc = 0xaf57c;
        }
        0xaf57c => {
            // block af57c
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af57c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af57c mov rdi,rax
            rdi = rax;
            // af57f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af584 lea rsi,[rip+0x7b0f7] # 0x12a682
            rsi = m.env.data_va(257615);
            // af58b mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // af590 mov rdi,rax
            rdi = rax;
            // af593 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaf598;
        }
        0xaf598 => {
            // block af598
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc af598 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af598 mov rax,QWORD PTR [rbp-0x198]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64;
            // af59f add rbx,QWORD PTR [rax-0x18]
            { let (r, f) = add::<64>(rbx, m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64);
              rbx = r;
              fl = f; }
            // af5a3 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // af5a7 mov rsi,rbx
            rsi = rbx;
            // af5aa call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af5af mov rsi,QWORD PTR [rip+0x90bda] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af5b6 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // af5ba call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af5bf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af5c2 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af5c6 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af5cb mov rdi,rax
            rdi = rax;
            // af5ce call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af5d0 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af5d2 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // af5d6 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af5db movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // af5de lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // af5e5 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af5ea lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // af5f1 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af5f6 lea rsi,[rip+0x857cd] # 0x134dca
            rsi = m.env.data_va(300439);
            // af5fd lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // af604 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // af609 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af60e mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // af611 and edx,0x100
            { let r = (rdx as u32 as u64) & 0x100u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af617 lea rcx,[rip+0x7b5fb] # 0x12ac19
            rcx = m.env.data_va(259046);
            // af61e lea rsi,[rip+0x7b5e4] # 0x12ac09
            rsi = m.env.data_va(259030);
            // af625 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // af629 shr rdx,0x7
            { let (r, f) = shr::<64>(rdx, 0x7u64, fl); rdx = r; fl = f; }
            // af62d xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // af631 add rdx,0x3
            { let (r, f) = add::<64>(rdx, 0x3u64);
              rdx = r;
              fl = f; }
            // af635 mov rdi,rax
            rdi = rax;
            // af638 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af63d mov rbx,rax
            rbx = rax;
            // af640 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af643 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af647 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af64a lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // af64e call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af653 mov rsi,QWORD PTR [rip+0x90b36] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af65a lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // af65e call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af663 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af666 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af66a mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af66f mov rdi,rax
            rdi = rax;
            // af672 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af674 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af677 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // af67b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af680 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af684 mov rdi,rbx
            rdi = rbx;
            // af687 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af68c mov rdi,rbx
            rdi = rbx;
            // af68f call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af694 lea rsi,[rip+0x8573c] # 0x134dd7
            rsi = m.env.data_va(300452);
            // af69b mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // af6a0 mov rdi,rbx
            rdi = rbx;
            // af6a3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af6a8 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // af6ab shr ecx,0x9
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x9u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // af6ae and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // af6b1 lea rcx,[rcx+rcx*2]
            rcx = rcx.wrapping_add(rcx.wrapping_mul(2));
            // af6b5 lea rdx,[rip+0x98a54] # 0x148110
            rdx = m.env.data_va(379101);
            // af6bc lea rsi,[rdx+rcx*8]
            rsi = rdx.wrapping_add(rcx.wrapping_mul(8));
            // af6c0 mov rdi,rax
            rdi = rax;
            // af6c3 call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af6c8 mov rbx,rax
            rbx = rax;
            // af6cb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af6ce mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af6d2 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af6d5 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // af6d9 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af6de mov rsi,QWORD PTR [rip+0x90aab] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af6e5 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // af6e9 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af6ee mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af6f1 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af6f5 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af6fa mov rdi,rax
            rdi = rax;
            // af6fd call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af6ff mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af702 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // af706 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af70b movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af70f mov rdi,rbx
            rdi = rbx;
            // af712 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af717 mov rdi,rbx
            rdi = rbx;
            // af71a call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af71f lea rsi,[rip+0x856be] # 0x134de4
            rsi = m.env.data_va(300465);
            // af726 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // af72b mov rdi,rbx
            rdi = rbx;
            // af72e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af733 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // af736 and edx,0x2000
            { let r = (rdx as u32 as u64) & 0x2000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af73c lea rcx,[rip+0x84b82] # 0x1342c5
            rcx = m.env.data_va(297618);
            // af743 lea rsi,[rip+0x84b87] # 0x1342d1
            rsi = m.env.data_va(297630);
            // af74a cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // af74e shr rdx,0xd
            { let (r, f) = shr::<64>(rdx, 0xdu64, fl); rdx = r; fl = f; }
            // af752 or rdx,0xa
            { let r = rdx | 0xau64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // af756 mov rdi,rax
            rdi = rax;
            // af759 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af75e mov rbx,rax
            rbx = rax;
            // af761 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af764 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af768 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af76b lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // af76f call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af774 mov rsi,QWORD PTR [rip+0x90a15] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af77b lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // af77f call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af784 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af787 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af78b mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af790 mov rdi,rax
            rdi = rax;
            // af793 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af795 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af798 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // af79c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af7a1 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af7a5 mov rdi,rbx
            rdi = rbx;
            // af7a8 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af7ad mov rdi,rbx
            rdi = rbx;
            // af7b0 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af7b5 lea rsi,[rip+0x85634] # 0x134df0
            rsi = m.env.data_va(300477);
            // af7bc mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // af7c1 mov rdi,rbx
            rdi = rbx;
            // af7c4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af7c9 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // af7cc and edx,0x4000
            { let r = (rdx as u32 as u64) & 0x4000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af7d2 lea rcx,[rip+0x85623] # 0x134dfc
            rcx = m.env.data_va(300489);
            // af7d9 lea rsi,[rip+0x85623] # 0x134e03
            rsi = m.env.data_va(300496);
            // af7e0 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // af7e4 shr rdx,0xe
            { let (r, f) = shr::<64>(rdx, 0xeu64, fl); rdx = r; fl = f; }
            // af7e8 add rdx,0x5
            { let (r, f) = add::<64>(rdx, 0x5u64);
              rdx = r;
              fl = f; }
            // af7ec mov rdi,rax
            rdi = rax;
            // af7ef call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af7f4 mov rbx,rax
            rbx = rax;
            // af7f7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af7fa mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af7fe add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af801 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // af805 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af80a mov rsi,QWORD PTR [rip+0x9097f] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af811 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // af815 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af81a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af81d mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af821 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af826 mov rdi,rax
            rdi = rax;
            // af829 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af82b mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af82e lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // af832 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af837 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af83b mov rdi,rbx
            rdi = rbx;
            // af83e call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af843 mov rdi,rbx
            rdi = rbx;
            // af846 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af84b lea rsi,[rip+0x855b7] # 0x134e09
            rsi = m.env.data_va(300502);
            // af852 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // af857 mov rdi,rbx
            rdi = rbx;
            // af85a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af85f lea rcx,[rip+0x855b4] # 0x134e1a
            rcx = m.env.data_va(300519);
            // af866 lea rsi,[rip+0x855b3] # 0x134e20
            rsi = m.env.data_va(300525);
            // af86d mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // af870 test dh,0x80
            fl = logic::<8>(((rdx >> 8) as u8 as u64) & 0x80u64);
            // af873 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // af877 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af87c mov rdi,rax
            rdi = rax;
            // af87f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af884 mov rbx,rax
            rbx = rax;
            // af887 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af88a mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af88e add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af891 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // af895 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af89a mov rsi,QWORD PTR [rip+0x908ef] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af8a1 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // af8a5 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af8aa mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af8ad mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af8b1 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af8b6 mov rdi,rax
            rdi = rax;
            // af8b9 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af8bb mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af8be lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // af8c2 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af8c7 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af8cb mov rdi,rbx
            rdi = rbx;
            // af8ce call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af8d3 mov rdi,rbx
            rdi = rbx;
            // af8d6 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af8db lea rsi,[rip+0x85544] # 0x134e26
            rsi = m.env.data_va(300531);
            // af8e2 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // af8e7 mov rdi,rbx
            rdi = rbx;
            // af8ea call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af8ef mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // af8f2 and ecx,0x10000000
            { let r = (rcx as u32 as u64) & 0x10000000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // af8f8 lea rdx,[rip+0x85536] # 0x134e35
            rdx = m.env.data_va(300546);
            // af8ff lea rsi,[rip+0x85534] # 0x134e3a
            rsi = m.env.data_va(300551);
            // af906 cmovne rsi,rdx
            if fl.ne() { rsi = rdx; }
            // af90a shr rcx,0x1c
            { let (r, f) = shr::<64>(rcx, 0x1cu64, fl); rcx = r; fl = f; }
            // af90e mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // af913 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // af916 mov rdi,rax
            rdi = rax;
            // af919 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af91e mov rbx,rax
            rbx = rax;
            // af921 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af924 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af928 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af92b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // af92f call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af934 mov rsi,QWORD PTR [rip+0x90855] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af93b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // af93f call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af944 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af947 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af94b mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af950 mov rdi,rax
            rdi = rax;
            // af953 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af955 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af958 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // af95c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af961 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af965 mov rdi,rbx
            rdi = rbx;
            // af968 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af96d mov rdi,rbx
            rdi = rbx;
            // af970 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af975 lea rsi,[rip+0x854c4] # 0x134e40
            rsi = m.env.data_va(300557);
            // af97c mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // af981 mov rdi,rbx
            rdi = rbx;
            // af984 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af989 lea rcx,[rip+0x854c1] # 0x134e51
            rcx = m.env.data_va(300574);
            // af990 lea rsi,[rip+0x7b311] # 0x12aca8
            rsi = m.env.data_va(259189);
            // af997 test r12d,0x20000000
            fl = logic::<32>((r12 as u32 as u64) & 0x20000000u64);
            // af99e cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // af9a2 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // af9a7 mov rdi,rax
            rdi = rax;
            // af9aa call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af9af mov rbx,rax
            rbx = rax;
            // af9b2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af9b5 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // af9b9 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // af9bc lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // af9c0 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af9c5 mov rsi,QWORD PTR [rip+0x907c4] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // af9cc lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // af9d0 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af9d5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // af9d8 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // af9dc mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // af9e1 mov rdi,rax
            rdi = rax;
            // af9e4 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af9e6 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // af9e9 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // af9ed call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af9f2 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // af9f6 mov rdi,rbx
            rdi = rbx;
            // af9f9 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af9fe mov rdi,rbx
            rdi = rbx;
            // afa01 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa06 lea rsi,[rip+0x85446] # 0x134e53
            rsi = m.env.data_va(300576);
            // afa0d mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // afa12 mov rdi,rbx
            rdi = rbx;
            // afa15 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa1a mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // afa1d and ecx,0x40000000
            { let r = (rcx as u32 as u64) & 0x40000000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // afa23 lea rdx,[rip+0x85432] # 0x134e5c
            rdx = m.env.data_va(300585);
            // afa2a lea rsi,[rip+0x7fca4] # 0x12f6d5
            rsi = m.env.data_va(278178);
            // afa31 cmovne rsi,rdx
            if fl.ne() { rsi = rdx; }
            // afa35 shr rcx,0x1e
            { let (r, f) = shr::<64>(rcx, 0x1eu64, fl); rcx = r; fl = f; }
            // afa39 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // afa3e sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // afa41 mov rdi,rax
            rdi = rax;
            // afa44 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa49 mov rbx,rax
            rbx = rax;
            // afa4c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // afa4f mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // afa53 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // afa56 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // afa5a call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa5f mov rsi,QWORD PTR [rip+0x9072a] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // afa66 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // afa6a call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa6f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // afa72 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // afa76 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // afa7b mov rdi,rax
            rdi = rax;
            // afa7e call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa80 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // afa83 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // afa87 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa8c movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // afa90 mov rdi,rbx
            rdi = rbx;
            // afa93 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afa98 mov rdi,rbx
            rdi = rbx;
            // afa9b call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afaa0 lea rsi,[rip+0x849f2] # 0x134499
            rsi = m.env.data_va(298086);
            // afaa7 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // afaac mov rdi,rbx
            rdi = rbx;
            // afaaf call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afab4 test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // afab7 lea rcx,[rip+0x7c2b2] # 0x12bd70
            rcx = m.env.data_va(263485);
            // afabe lea rsi,[rip+0x7c28f] # 0x12bd54
            rsi = m.env.data_va(263457);
            // afac5 cmovs rsi,rcx
            if fl.s() { rsi = rcx; }
            // afac9 setns cl
            rcx = (rcx & !0xff) | (((fl.ns() as u64)) as u8 as u64);
            // afacc movzx edx,cl
            rdx = ((rcx as u8 as u64)) as u32 as u64;
            // afacf or rdx,0x2
            { let r = rdx | 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // afad3 mov rdi,rax
            rdi = rax;
            // afad6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afadb mov rdi,r13
            rdi = r13;
            // afade lea rsi,[rbp-0x190]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // afae5 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afaea lea rdi,[rbp-0x1c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe38);
            // afaf1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afaf6 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // afafd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afb02 mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // afb09 mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // afb10 mov rax,QWORD PTR [rbp-0x1e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            // afb17 mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // afb1e mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // afb25 mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            // afb2c lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // afb33 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afb38 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // afb3f call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afb44 mov rsi,QWORD PTR [rip+0x906bd] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // afb4b add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // afb4f lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // afb56 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afb5b lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // afb62 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afb67 mov rax,r13
            rax = r13;
            // afb6a add rsp,0x1b8
            { let (r, f) = add::<64>(rsp, 0x1b8u64);
              rsp = r;
              fl = f; }
            // afb71 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // afb72 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // afb74 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // afb76 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // afb78 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // afb7a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // afb7b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xafb7c => {
            // block afb7c
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afb7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afb7c mov r15,rax
            r15 = rax;
            pc = 0xafb7f;
        }
        0xafb7f => {
            // block afb7f
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afb7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afb7f lea rdi,[rbp-0x1c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe38);
            // afb86 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xafb8b;
        }
        0xafb8b => {
            // block afb8b
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afb8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afb8b lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // afb92 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xafb97;
        }
        0xafb97 => {
            // block afb97
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afb97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afb97 mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // afb9e mov QWORD PTR [rbp-0x198],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (rax) as u64);
            // afba5 mov rax,QWORD PTR [rbp-0x1e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            // afbac mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // afbb3 mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // afbba mov QWORD PTR [rbp-0x190],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rax) as u64);
            pc = 0xafbc1;
        }
        0xafbc1 => {
            // block afbc1
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afbc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afbc1 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // afbc8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afbcd lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // afbd4 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xafbd9;
        }
        0xafbd9 => {
            // block afbd9
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afbd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afbd9 mov rsi,QWORD PTR [rip+0x90628] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // afbe0 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // afbe4 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // afbeb call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xafbf0;
        }
        0xafbf0 => {
            // block afbf0
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afbf0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afbf0 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // afbf7 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afbfc mov rdi,r15
            rdi = r15;
            // afbff call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc04 mov r15,rax
            r15 = rax;
            pc = 0xafc07;
        }
        0xafc07 => {
            // block afc07
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc07 add r14,0xffffffffffffffe8
            { let (r, f) = add::<64>(r14, 0xffff_ffff_ffff_ffe8u64);
              r14 = r;
              fl = f; }
            // afc0b mov rdi,r14
            rdi = r14;
            // afc0e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc13 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // afc16 jne 0xafc07
            if fl.ne() { pc = 0xafc07; continue; }
            pc = 0xafc18;
        }
        0xafc18 => {
            // block afc18
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc18 jmp 0xafd30
            pc = 0xafd30; continue;
        }
        0xafc1d => {
            // block afc1d
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc1d mov r15,rax
            r15 = rax;
            pc = 0xafc20;
        }
        0xafc20 => {
            // block afc20
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc20 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // afc24 mov rdi,rbx
            rdi = rbx;
            // afc27 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc2c cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // afc2f jne 0xafc20
            if fl.ne() { pc = 0xafc20; continue; }
            pc = 0xafc31;
        }
        0xafc31 => {
            // block afc31
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc31 jmp 0xafd44
            pc = 0xafd44; continue;
        }
        0xafc36 => {
            // block afc36
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc36 mov r15,rax
            r15 = rax;
            // afc39 jmp 0xafb97
            pc = 0xafb97; continue;
        }
        0xafc3e => {
            // block afc3e
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc3e mov r15,rax
            r15 = rax;
            // afc41 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // afc45 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc4a jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafc4f => {
            // block afc4f
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc4f mov r15,rax
            r15 = rax;
            // afc52 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // afc56 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc5b jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafc60 => {
            // block afc60
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc60 mov r15,rax
            r15 = rax;
            // afc63 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // afc67 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc6c jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafc71 => {
            // block afc71
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc71 mov r15,rax
            r15 = rax;
            // afc74 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // afc78 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc7d jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafc82 => {
            // block afc82
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc82 mov r15,rax
            r15 = rax;
            // afc85 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // afc89 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc8e jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafc93 => {
            // block afc93
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afc93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afc93 mov r15,rax
            r15 = rax;
            // afc96 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // afc9a call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc9f jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafca4 => {
            // block afca4
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afca4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afca4 mov r15,rax
            r15 = rax;
            // afca7 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // afcab call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afcb0 jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafcb5 => {
            // block afcb5
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afcb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afcb5 mov r15,rax
            r15 = rax;
            // afcb8 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // afcbc call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afcc1 jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafcc6 => {
            // block afcc6
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afcc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afcc6 mov r15,rax
            r15 = rax;
            // afcc9 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // afccd call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afcd2 jmp 0xafb7f
            pc = 0xafb7f; continue;
        }
        0xafcd7 => {
            // block afcd7
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afcd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afcd7 mov r15,rax
            r15 = rax;
            pc = 0xafcda;
        }
        0xafcda => {
            // block afcda
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afcda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afcda add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // afcde mov rdi,rbx
            rdi = rbx;
            // afce1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afce6 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // afce9 jne 0xafcda
            if fl.ne() { pc = 0xafcda; continue; }
            pc = 0xafceb;
        }
        0xafceb => {
            // block afceb
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afceb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afceb jmp 0xafd1c
            pc = 0xafd1c; continue;
        }
        0xafced => {
            // block afced
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afced rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afced mov r15,rax
            r15 = rax;
            // afcf0 jmp 0xafbf0
            pc = 0xafbf0; continue;
        }
        0xafcf5 => {
            // block afcf5
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afcf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afcf5 mov r15,rax
            r15 = rax;
            // afcf8 jmp 0xafbd9
            pc = 0xafbd9; continue;
        }
        0xafcfd => {
            // block afcfd
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afcfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afcfd mov r15,rax
            r15 = rax;
            // afd00 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // afd07 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afd0c jmp 0xafbc1
            pc = 0xafbc1; continue;
        }
        0xafd11 => {
            // block afd11
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd11 mov r15,rax
            r15 = rax;
            // afd14 jmp 0xafb8b
            pc = 0xafb8b; continue;
        }
        0xafd19 => {
            // block afd19
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd19 mov r15,rax
            r15 = rax;
            pc = 0xafd1c;
        }
        0xafd1c => {
            // block afd1c
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd1c lea rdi,[rip+0x982d5] # 0x147ff8
            rdi = m.env.data_va(378821);
            // afd23 call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afd28 jmp 0xafb97
            pc = 0xafb97; continue;
        }
        0xafd2d => {
            // block afd2d
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd2d mov r15,rax
            r15 = rax;
            pc = 0xafd30;
        }
        0xafd30 => {
            // block afd30
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd30 lea rdi,[rip+0x983d1] # 0x148108
            rdi = m.env.data_va(379093);
            // afd37 call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afd3c jmp 0xafb97
            pc = 0xafb97; continue;
        }
        0xafd41 => {
            // block afd41
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd41 mov r15,rax
            r15 = rax;
            pc = 0xafd44;
        }
        0xafd44 => {
            // block afd44
            #[cfg(feature = "trace")] eprintln!("f_000aef04 pc afd44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // afd44 lea rdi,[rip+0x984cd] # 0x148218
            rdi = m.env.data_va(379365);
            // afd4b call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afd50 jmp 0xafb97
            pc = 0xafb97; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x99644 size=5262
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00099644<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x99644;
    loop {
        match pc {
        0x99644 => {
            // block 99644
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99644 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99644 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 99645 mov rbp,rsp
            rbp = rsp;
            // 99648 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 9964a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 9964c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 9964e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 99650 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 99651 sub rsp,0x7f8
            { let (r, f) = sub::<64>(rsp, 0x7f8u64);
              rsp = r;
              fl = f; }
            // 99658 mov r14,rdi
            r14 = rdi;
            // 9965b mov al,BYTE PTR [rip+0xafd4f] # 0x1493b0
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(383869)) as u64) as u8 as u64);
            // 99661 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 99663 jne 0x99740
            if fl.ne() { pc = 0x99740; continue; }
            pc = 0x99669;
        }
        0x99669 => {
            // block 99669
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99669 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99669 lea rdi,[rip+0xafd40] # 0x1493b0
            rdi = m.env.data_va(383869);
            // 99670 call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99675 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 99677 je 0x99740
            if fl.e() { pc = 0x99740; continue; }
            pc = 0x9967d;
        }
        0x9967d => {
            // block 9967d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9967d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9967d lea r15,[rip+0xafc9c] # 0x149320
            r15 = m.env.data_va(383725);
            // 99684 lea rsi,[rip+0x9c182] # 0x13580d
            rsi = m.env.data_va(303066);
            // 9968b mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 99690 mov rdi,r15
            rdi = r15;
            // 99693 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99698 lea rbx,[rip+0xafc99] # 0x149338
            rbx = m.env.data_va(383749);
            // 9969f lea rsi,[rip+0x9c16e] # 0x135814
            rsi = m.env.data_va(303073);
            // 996a6 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 996ab mov rdi,rbx
            rdi = rbx;
            // 996ae call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 996b3 lea rbx,[rip+0xafc96] # 0x149350
            rbx = m.env.data_va(383773);
            // 996ba lea rsi,[rip+0x9c161] # 0x135822
            rsi = m.env.data_va(303087);
            // 996c1 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 996c6 mov rdi,rbx
            rdi = rbx;
            // 996c9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 996ce lea rbx,[rip+0xafc93] # 0x149368
            rbx = m.env.data_va(383797);
            // 996d5 lea rsi,[rip+0x9c154] # 0x135830
            rsi = m.env.data_va(303101);
            // 996dc mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 996e1 mov rdi,rbx
            rdi = rbx;
            // 996e4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 996e9 lea rbx,[rip+0xafc90] # 0x149380
            rbx = m.env.data_va(383821);
            // 996f0 lea rsi,[rip+0x9c148] # 0x13583f
            rsi = m.env.data_va(303116);
            // 996f7 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 996fc mov rdi,rbx
            rdi = rbx;
            // 996ff call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99704 lea rbx,[rip+0xafc8d] # 0x149398
            rbx = m.env.data_va(383845);
            // 9970b lea rsi,[rip+0x9c13e] # 0x135850
            rsi = m.env.data_va(303133);
            // 99712 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 99717 mov rdi,rbx
            rdi = rbx;
            // 9971a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9971f lea rdi,[rip+0xfffffffffffcf16d] # 0x68893
            rdi = m.env.code_va(0x68893);
            // 99726 lea rdx,[rip+0xfffffffffff65a43] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // 9972d xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 9972f call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99734 lea rdi,[rip+0xafc75] # 0x1493b0
            rdi = m.env.data_va(383869);
            // 9973b call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99740;
        }
        0x99740 => {
            // block 99740
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99740 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99740 lea rbx,[rbp-0x138]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // 99747 lea r13,[rbp-0x1a0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 9974e lea rax,[r14+0x180]
            rax = r14.wrapping_add(0x180);
            // 99755 mov QWORD PTR [rbp-0x788],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f878), (rax) as u64);
            // 9975c lea rax,[r14+0x198]
            rax = r14.wrapping_add(0x198);
            // 99763 mov QWORD PTR [rbp-0x748],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8), (rax) as u64);
            // 9976a lea rax,[r14+0x158]
            rax = r14.wrapping_add(0x158);
            // 99771 mov QWORD PTR [rbp-0x7a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f858), (rax) as u64);
            // 99778 lea rax,[r14+0x160]
            rax = r14.wrapping_add(0x160);
            // 9977f mov QWORD PTR [rbp-0x7a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f860), (rax) as u64);
            // 99786 mov QWORD PTR [rbp-0x770],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f890), (r14) as u64);
            // 9978d xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 9978f mov rax,QWORD PTR [rip+0xa6c12] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 99796 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 9979a mov QWORD PTR [rbp-0x750],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8b0), (rax) as u64);
            // 997a1 mov rax,QWORD PTR [rip+0xa6a60] # 0x140208
            rax = m.r64(m.env.data_va(346581)) as u64;
            // 997a8 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 997ac mov QWORD PTR [rbp-0x790],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f870), (rax) as u64);
            // 997b3 lea r14,[rbp-0xa0]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            pc = 0x997ba;
        }
        0x997ba => {
            // block 997ba
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 997ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 997ba mov QWORD PTR [rbp-0x780],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f880), (rcx) as u64);
            // 997c1 lea rax,[rcx+0x1]
            rax = rcx.wrapping_add(0x1);
            // 997c5 mov QWORD PTR [rbp-0x778],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f888), (rax) as u64);
            // 997cc lea rax,[rcx+rcx*2]
            rax = rcx.wrapping_add(rcx.wrapping_mul(2));
            // 997d0 lea rcx,[rip+0x12eae9] # 0x1c82c0
            rcx = m.env.data_va(903821);
            // 997d7 lea rdx,[rcx+rax*8]
            rdx = rcx.wrapping_add(rax.wrapping_mul(8));
            // 997db mov QWORD PTR [rbp-0x768],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f898), (rdx) as u64);
            // 997e2 lea rax,[rcx+rax*8+0x8]
            rax = rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8);
            // 997e7 mov QWORD PTR [rbp-0x798],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f868), (rax) as u64);
            // 997ee lea r12,[rip+0xafb2b] # 0x149320
            r12 = m.env.data_va(383725);
            // 997f5 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x997f8;
        }
        0x997f8 => {
            // block 997f8
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 997f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 997f8 mov rax,QWORD PTR [rip+0xa69d1] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 997ff mov rcx,rax
            rcx = rax;
            // 99802 lea rax,[rcx+0x18]
            rax = rcx.wrapping_add(0x18);
            // 99806 mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 9980d lea rax,[rcx+0x40]
            rax = rcx.wrapping_add(0x40);
            // 99811 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // 99818 mov rdi,rbx
            rdi = rbx;
            // 9981b mov rsi,r13
            rsi = r13;
            // 9981e call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99823 mov QWORD PTR [rbp-0xb0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x0u64) as u64);
            // 9982e mov DWORD PTR [rbp-0xa8],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0xffffffffu64) as u32);
            // 99838 mov rax,QWORD PTR [rip+0xa6b81] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 9983f lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 99843 mov QWORD PTR [rbp-0x760],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8a0), (rcx) as u64);
            // 9984a mov QWORD PTR [rbp-0x1a8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rcx) as u64);
            // 99851 lea rax,[rax+0x40]
            rax = rax.wrapping_add(0x40);
            // 99855 mov QWORD PTR [rbp-0x758],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8a8), (rax) as u64);
            // 9985c mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // 99863 mov rdi,r13
            rdi = r13;
            // 99866 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9986b mov rax,QWORD PTR [rbp-0x750]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b0)) as u64;
            // 99872 mov QWORD PTR [rbp-0x1a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe60), (rax) as u64);
            // 99879 lea rax,[rbp-0x160]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 99880 mov QWORD PTR [rax+0x18],0x0
            m.w64(rax.wrapping_add(0x18), (0x0u64) as u64);
            // 99888 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 99890 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 99898 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 9989f mov DWORD PTR [rbp-0x140],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (0x10u64) as u32);
            // 998a9 xorps xmm0,xmm0
            x0 = 0;
            // 998ac movaps XMMWORD PTR [rbp-0xa0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff60), x0);
            // 998b3 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 998be mov rdi,r13
            rdi = r13;
            // 998c1 mov rsi,r14
            rsi = r14;
            // 998c4 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 998c9 mov rdi,r14
            rdi = r14;
            // 998cc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 998d1 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 998d6 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 998dd lea rsi,[rip+0x9bf7e] # 0x135862
            rsi = m.env.data_va(303151);
            // 998e4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 998e9 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 998ec mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 998f0 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 998f4 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 998f7 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 998fa mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 998fe mov rdi,rax
            rdi = rax;
            // 99901 mov rsi,QWORD PTR [rbp-0x778]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f888)) as u64;
            // 99908 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9990d mov rdi,rax
            rdi = rax;
            // 99910 mov rsi,r12
            rsi = r12;
            // 99913 call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99918 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 9991f mov rsi,r13
            rsi = r13;
            // 99922 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99927 mov rax,QWORD PTR [rbp-0x780]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f880)) as u64;
            // 9992e lea rcx,[rip+0x68ddb] # 0x102710
            rcx = m.env.data_va(93917);
            // 99935 mov r13d,DWORD PTR [rcx+rax*4]
            r13 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 99939 add r13,r15
            { let (r, f) = add::<64>(r13, r15);
              r13 = r;
              fl = f; }
            // 9993c movzx eax,r15w
            rax = ((r15 as u16 as u64)) as u32 as u64;
            // 99940 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 99943 jne 0x999e8
            if fl.ne() { pc = 0x999e8; continue; }
            pc = 0x99949;
        }
        0x99949 => {
            // block 99949
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99949 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99949 mov rdi,QWORD PTR [rbp-0x770]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f890)) as u64;
            // 99950 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 99953 lea rdx,[rbp-0x1c0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 9995a call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9995f mov DWORD PTR [rbp-0x2f0],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd10), ((r13 as u32 as u64)) as u32);
            // 99966 mov rax,QWORD PTR [rbp-0x7a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f860)) as u64;
            // 9996d mov QWORD PTR [rbp-0x2e8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd18), (rax) as u64);
            // 99974 mov rdi,QWORD PTR [rbp-0x788]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f878)) as u64;
            // 9997b lea rsi,[rbp-0x2f0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd10);
            // 99982 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99987 mov rax,r15
            rax = r15;
            // 9998a and rax,0xfffe
            { let r = rax & 0xfffeu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 99990 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // 99994 jne 0x99bda
            if fl.ne() { pc = 0x99bda; continue; }
            pc = 0x9999a;
        }
        0x9999a => {
            // block 9999a
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9999a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9999a mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9999f lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 999a3 lea rsi,[rip+0x9b9b7] # 0x135361
            rsi = m.env.data_va(301870);
            // 999aa call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 999af lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 999b3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 999b8 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 999bd lea rdi,[rbp-0x308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf8);
            // 999c4 lea rsi,[rip+0x9a01f] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 999cb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 999d0 movzx ebx,BYTE PTR [rbp-0x308]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fcf8)) as u64) as u32 as u64;
            // 999d7 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 999da jne 0x99b36
            if fl.ne() { pc = 0x99b36; continue; }
            pc = 0x999e0;
        }
        0x999e0 => {
            // block 999e0
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 999e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 999e0 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 999e3 jmp 0x99b3d
            pc = 0x99b3d; continue;
        }
        0x999e8 => {
            // block 999e8
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 999e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 999e8 test r15w,r15w
            fl = logic::<16>((r15 as u16 as u64) & (r15 as u16 as u64));
            // 999ec jne 0x99a97
            if fl.ne() { pc = 0x99a97; continue; }
            pc = 0x999f2;
        }
        0x999f2 => {
            // block 999f2
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 999f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 999f2 mov rdi,QWORD PTR [rbp-0x770]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f890)) as u64;
            // 999f9 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 999fc lea rdx,[rbp-0x1c0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 99a03 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99a08 mov DWORD PTR [rbp-0x1d0],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe30), ((r13 as u32 as u64)) as u32);
            // 99a0f mov rax,QWORD PTR [rbp-0x7a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f858)) as u64;
            // 99a16 mov QWORD PTR [rbp-0x1c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe38), (rax) as u64);
            // 99a1d mov rdi,QWORD PTR [rbp-0x788]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f878)) as u64;
            // 99a24 lea rsi,[rbp-0x1d0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe30);
            // 99a2b call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99a30 mov rax,r15
            rax = r15;
            // 99a33 and rax,0xfffe
            { let r = rax & 0xfffeu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 99a39 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // 99a3d jne 0x99e6e
            if fl.ne() { pc = 0x99e6e; continue; }
            pc = 0x99a43;
        }
        0x99a43 => {
            // block 99a43
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99a43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99a43 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 99a48 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 99a4f lea rsi,[rip+0x9b90b] # 0x135361
            rsi = m.env.data_va(301870);
            // 99a56 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99a5b lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 99a62 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99a67 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99a6c lea rdi,[rbp-0x1e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe18);
            // 99a73 lea rsi,[rip+0x99f70] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 99a7a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99a7f movzx ebx,BYTE PTR [rbp-0x1e8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe18)) as u64) as u32 as u64;
            // 99a86 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 99a89 jne 0x99dca
            if fl.ne() { pc = 0x99dca; continue; }
            pc = 0x99a8f;
        }
        0x99a8f => {
            // block 99a8f
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99a8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99a8f shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 99a92 jmp 0x99dd1
            pc = 0x99dd1; continue;
        }
        0x99a97 => {
            // block 99a97
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99a97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99a97 mov rdi,QWORD PTR [rbp-0x770]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f890)) as u64;
            // 99a9e mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 99aa1 lea rdx,[rbp-0x1c0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 99aa8 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99aad mov DWORD PTR [rbp-0x410],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fbf0), ((r13 as u32 as u64)) as u32);
            // 99ab4 mov rax,QWORD PTR [rbp-0x770]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f890)) as u64;
            // 99abb mov QWORD PTR [rbp-0x408],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbf8), (rax) as u64);
            // 99ac2 mov rdi,QWORD PTR [rbp-0x788]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f878)) as u64;
            // 99ac9 lea rsi,[rbp-0x410]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fbf0);
            // 99ad0 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99ad5 mov rax,r15
            rax = r15;
            // 99ad8 and rax,0xfffe
            { let r = rax & 0xfffeu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 99ade cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // 99ae2 jne 0x9a102
            if fl.ne() { pc = 0x9a102; continue; }
            pc = 0x99ae8;
        }
        0x99ae8 => {
            // block 99ae8
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99ae8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99ae8 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 99aed lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 99af1 lea rsi,[rip+0x9b869] # 0x135361
            rsi = m.env.data_va(301870);
            // 99af8 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99afd lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 99b01 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99b06 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99b0b lea rdi,[rbp-0x428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd8);
            // 99b12 lea rsi,[rip+0x99ed1] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 99b19 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99b1e movzx ebx,BYTE PTR [rbp-0x428]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fbd8)) as u64) as u32 as u64;
            // 99b25 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 99b28 jne 0x9a05e
            if fl.ne() { pc = 0x9a05e; continue; }
            pc = 0x99b2e;
        }
        0x99b2e => {
            // block 99b2e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99b2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99b2e shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 99b31 jmp 0x9a065
            pc = 0x9a065; continue;
        }
        0x99b36 => {
            // block 99b36
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99b36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99b36 mov rbx,QWORD PTR [rbp-0x300]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd00)) as u64;
            pc = 0x99b3d;
        }
        0x99b3d => {
            // block 99b3d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99b3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99b3d lea rdi,[rbp-0x308]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf8);
            // 99b44 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99b49 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 99b4c je 0x99bda
            if fl.e() { pc = 0x99bda; continue; }
            pc = 0x99b52;
        }
        0x99b52 => {
            // block 99b52
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99b52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99b52 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99b57 lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 99b5e lea rsi,[rip+0x99e85] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 99b65 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99b6a mov rax,QWORD PTR [rbp-0x330]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64;
            // 99b71 mov QWORD PTR [rbp-0x318],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce8), (rax) as u64);
            // 99b78 mov rax,QWORD PTR [rbp-0x340]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcc0)) as u64;
            // 99b7f mov rcx,QWORD PTR [rbp-0x338]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcc8)) as u64;
            // 99b86 mov QWORD PTR [rbp-0x320],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce0), (rcx) as u64);
            // 99b8d mov QWORD PTR [rbp-0x328],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcd8), (rax) as u64);
            // 99b94 xorps xmm0,xmm0
            x0 = 0;
            // 99b97 movaps XMMWORD PTR [rbp-0x340],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fcc0), x0);
            // 99b9e mov QWORD PTR [rbp-0x330],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0), (0x0u64) as u64);
            // 99ba9 mov DWORD PTR [rbp-0x310],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fcf0), ((r13 as u32 as u64)) as u32);
            // 99bb0 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99bb7 lea rbx,[rbp-0x328]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 99bbe mov rsi,rbx
            rsi = rbx;
            // 99bc1 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99bc6 mov rdi,rbx
            rdi = rbx;
            // 99bc9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99bce lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 99bd5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99bda;
        }
        0x99bda => {
            // block 99bda
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99bda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99bda mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99bdf lea rdi,[rbp-0x358]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fca8);
            // 99be6 lea rsi,[rip+0x9bc7f] # 0x13586c
            rsi = m.env.data_va(303161);
            // 99bed call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99bf2 movzx ebx,BYTE PTR [rbp-0x358]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fca8)) as u64) as u32 as u64;
            // 99bf9 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 99bfc jne 0x99c03
            if fl.ne() { pc = 0x99c03; continue; }
            pc = 0x99bfe;
        }
        0x99bfe => {
            // block 99bfe
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99bfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99bfe shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 99c01 jmp 0x99c0a
            pc = 0x99c0a; continue;
        }
        0x99c03 => {
            // block 99c03
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99c03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99c03 mov rbx,QWORD PTR [rbp-0x350]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcb0)) as u64;
            pc = 0x99c0a;
        }
        0x99c0a => {
            // block 99c0a
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99c0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99c0a lea rdi,[rbp-0x358]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fca8);
            // 99c11 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99c16 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 99c19 je 0x99ca7
            if fl.e() { pc = 0x99ca7; continue; }
            pc = 0x99c1f;
        }
        0x99c1f => {
            // block 99c1f
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99c1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99c1f mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99c24 lea rdi,[rbp-0x390]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc70);
            // 99c2b lea rsi,[rip+0x9bc3a] # 0x13586c
            rsi = m.env.data_va(303161);
            // 99c32 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99c37 mov rax,QWORD PTR [rbp-0x380]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc80)) as u64;
            // 99c3e mov QWORD PTR [rbp-0x368],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc98), (rax) as u64);
            // 99c45 mov rax,QWORD PTR [rbp-0x390]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc70)) as u64;
            // 99c4c mov rcx,QWORD PTR [rbp-0x388]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc78)) as u64;
            // 99c53 mov QWORD PTR [rbp-0x370],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc90), (rcx) as u64);
            // 99c5a mov QWORD PTR [rbp-0x378],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc88), (rax) as u64);
            // 99c61 xorps xmm0,xmm0
            x0 = 0;
            // 99c64 movaps XMMWORD PTR [rbp-0x390],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fc70), x0);
            // 99c6b mov QWORD PTR [rbp-0x380],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc80), (0x0u64) as u64);
            // 99c76 mov DWORD PTR [rbp-0x360],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fca0), ((r13 as u32 as u64)) as u32);
            // 99c7d mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99c84 lea rbx,[rbp-0x378]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc88);
            // 99c8b mov rsi,rbx
            rsi = rbx;
            // 99c8e call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99c93 mov rdi,rbx
            rdi = rbx;
            // 99c96 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99c9b lea rdi,[rbp-0x390]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc70);
            // 99ca2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99ca7;
        }
        0x99ca7 => {
            // block 99ca7
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99ca7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99ca7 mov rax,QWORD PTR [rbp-0x768]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f898)) as u64;
            // 99cae movzx eax,BYTE PTR [rax]
            rax = (m.r8(rax) as u64) as u32 as u64;
            // 99cb1 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 99cb3 jne 0x99cba
            if fl.ne() { pc = 0x99cba; continue; }
            pc = 0x99cb5;
        }
        0x99cb5 => {
            // block 99cb5
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99cb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99cb5 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 99cb8 jmp 0x99cc4
            pc = 0x99cc4; continue;
        }
        0x99cba => {
            // block 99cba
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99cba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99cba mov rax,QWORD PTR [rbp-0x798]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f868)) as u64;
            // 99cc1 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            pc = 0x99cc4;
        }
        0x99cc4 => {
            // block 99cc4
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99cc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99cc4 lea rdi,[rbp-0x3b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc50);
            // 99ccb test rax,rax
            fl = logic::<64>(rax & rax);
            // 99cce je 0x99cfd
            if fl.e() { pc = 0x99cfd; continue; }
            pc = 0x99cd0;
        }
        0x99cd0 => {
            // block 99cd0
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99cd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99cd0 mov rbx,rdi
            rbx = rdi;
            // 99cd3 mov rsi,QWORD PTR [rbp-0x768]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f898)) as u64;
            // 99cda call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99cdf mov DWORD PTR [rbp-0x398],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc68), ((r13 as u32 as u64)) as u32);
            // 99ce6 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99ced mov rsi,rbx
            rsi = rbx;
            // 99cf0 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99cf5 mov rdi,rbx
            rdi = rbx;
            // 99cf8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99cfd;
        }
        0x99cfd => {
            // block 99cfd
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99cfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99cfd mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 99d02 lea rdi,[rbp-0x3c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc38);
            // 99d09 lea rsi,[rip+0x99cfb] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 99d10 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99d15 movzx ebx,BYTE PTR [rbp-0x3c8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fc38)) as u64) as u32 as u64;
            // 99d1c test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 99d1f jne 0x99d26
            if fl.ne() { pc = 0x99d26; continue; }
            pc = 0x99d21;
        }
        0x99d21 => {
            // block 99d21
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99d21 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 99d24 jmp 0x99d2d
            pc = 0x99d2d; continue;
        }
        0x99d26 => {
            // block 99d26
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99d26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99d26 mov rbx,QWORD PTR [rbp-0x3c0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc40)) as u64;
            pc = 0x99d2d;
        }
        0x99d2d => {
            // block 99d2d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99d2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99d2d lea rdi,[rbp-0x3c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc38);
            // 99d34 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99d39 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 99d3c je 0x9a2f5
            if fl.e() { pc = 0x9a2f5; continue; }
            pc = 0x99d42;
        }
        0x99d42 => {
            // block 99d42
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99d42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99d42 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 99d47 lea rdi,[rbp-0x400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 99d4e lea rsi,[rip+0x99cb6] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 99d55 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99d5a mov rax,QWORD PTR [rbp-0x3f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc10)) as u64;
            // 99d61 mov QWORD PTR [rbp-0x3d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc28), (rax) as u64);
            // 99d68 mov rax,QWORD PTR [rbp-0x400]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc00)) as u64;
            // 99d6f mov rcx,QWORD PTR [rbp-0x3f8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc08)) as u64;
            // 99d76 mov QWORD PTR [rbp-0x3e0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc20), (rcx) as u64);
            // 99d7d mov QWORD PTR [rbp-0x3e8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc18), (rax) as u64);
            // 99d84 xorps xmm0,xmm0
            x0 = 0;
            // 99d87 movaps XMMWORD PTR [rbp-0x400],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fc00), x0);
            // 99d8e mov QWORD PTR [rbp-0x3f0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc10), (0x0u64) as u64);
            // 99d99 mov DWORD PTR [rbp-0x3d0],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fc30), ((r13 as u32 as u64)) as u32);
            // 99da0 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99da7 lea rbx,[rbp-0x3e8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // 99dae mov rsi,rbx
            rsi = rbx;
            // 99db1 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99db6 mov rdi,rbx
            rdi = rbx;
            // 99db9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99dbe lea rdi,[rbp-0x400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 99dc5 jmp 0x9a2f0
            pc = 0x9a2f0; continue;
        }
        0x99dca => {
            // block 99dca
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99dca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99dca mov rbx,QWORD PTR [rbp-0x1e0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            pc = 0x99dd1;
        }
        0x99dd1 => {
            // block 99dd1
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99dd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99dd1 lea rdi,[rbp-0x1e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe18);
            // 99dd8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99ddd test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 99de0 je 0x99e6e
            if fl.e() { pc = 0x99e6e; continue; }
            pc = 0x99de6;
        }
        0x99de6 => {
            // block 99de6
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99de6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99de6 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99deb lea rdi,[rbp-0x220]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde0);
            // 99df2 lea rsi,[rip+0x99bf1] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 99df9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99dfe mov rax,QWORD PTR [rbp-0x210]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdf0)) as u64;
            // 99e05 mov QWORD PTR [rbp-0x1f8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe08), (rax) as u64);
            // 99e0c mov rax,QWORD PTR [rbp-0x220]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde0)) as u64;
            // 99e13 mov rcx,QWORD PTR [rbp-0x218]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fde8)) as u64;
            // 99e1a mov QWORD PTR [rbp-0x200],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe00), (rcx) as u64);
            // 99e21 mov QWORD PTR [rbp-0x208],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdf8), (rax) as u64);
            // 99e28 xorps xmm0,xmm0
            x0 = 0;
            // 99e2b movaps XMMWORD PTR [rbp-0x220],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fde0), x0);
            // 99e32 mov QWORD PTR [rbp-0x210],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdf0), (0x0u64) as u64);
            // 99e3d mov DWORD PTR [rbp-0x1f0],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe10), ((r13 as u32 as u64)) as u32);
            // 99e44 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99e4b lea rbx,[rbp-0x208]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fdf8);
            // 99e52 mov rsi,rbx
            rsi = rbx;
            // 99e55 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99e5a mov rdi,rbx
            rdi = rbx;
            // 99e5d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99e62 lea rdi,[rbp-0x220]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde0);
            // 99e69 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99e6e;
        }
        0x99e6e => {
            // block 99e6e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99e6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99e6e mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99e73 lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 99e7a lea rsi,[rip+0x9b9eb] # 0x13586c
            rsi = m.env.data_va(303161);
            // 99e81 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99e86 movzx ebx,BYTE PTR [rbp-0x238]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fdc8)) as u64) as u32 as u64;
            // 99e8d test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 99e90 jne 0x99e97
            if fl.ne() { pc = 0x99e97; continue; }
            pc = 0x99e92;
        }
        0x99e92 => {
            // block 99e92
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99e92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99e92 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 99e95 jmp 0x99e9e
            pc = 0x99e9e; continue;
        }
        0x99e97 => {
            // block 99e97
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99e97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99e97 mov rbx,QWORD PTR [rbp-0x230]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fdd0)) as u64;
            pc = 0x99e9e;
        }
        0x99e9e => {
            // block 99e9e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99e9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99e9e lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 99ea5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99eaa test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 99ead je 0x99f3b
            if fl.e() { pc = 0x99f3b; continue; }
            pc = 0x99eb3;
        }
        0x99eb3 => {
            // block 99eb3
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99eb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99eb3 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 99eb8 lea rdi,[rbp-0x270]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd90);
            // 99ebf lea rsi,[rip+0x9b9a6] # 0x13586c
            rsi = m.env.data_va(303161);
            // 99ec6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99ecb mov rax,QWORD PTR [rbp-0x260]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fda0)) as u64;
            // 99ed2 mov QWORD PTR [rbp-0x248],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb8), (rax) as u64);
            // 99ed9 mov rax,QWORD PTR [rbp-0x270]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd90)) as u64;
            // 99ee0 mov rcx,QWORD PTR [rbp-0x268]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd98)) as u64;
            // 99ee7 mov QWORD PTR [rbp-0x250],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb0), (rcx) as u64);
            // 99eee mov QWORD PTR [rbp-0x258],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda8), (rax) as u64);
            // 99ef5 xorps xmm0,xmm0
            x0 = 0;
            // 99ef8 movaps XMMWORD PTR [rbp-0x270],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fd90), x0);
            // 99eff mov QWORD PTR [rbp-0x260],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda0), (0x0u64) as u64);
            // 99f0a mov DWORD PTR [rbp-0x240],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fdc0), ((r13 as u32 as u64)) as u32);
            // 99f11 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99f18 lea rbx,[rbp-0x258]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fda8);
            // 99f1f mov rsi,rbx
            rsi = rbx;
            // 99f22 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99f27 mov rdi,rbx
            rdi = rbx;
            // 99f2a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99f2f lea rdi,[rbp-0x270]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd90);
            // 99f36 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99f3b;
        }
        0x99f3b => {
            // block 99f3b
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99f3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99f3b mov rax,QWORD PTR [rbp-0x768]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f898)) as u64;
            // 99f42 movzx eax,BYTE PTR [rax]
            rax = (m.r8(rax) as u64) as u32 as u64;
            // 99f45 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 99f47 jne 0x99f4e
            if fl.ne() { pc = 0x99f4e; continue; }
            pc = 0x99f49;
        }
        0x99f49 => {
            // block 99f49
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99f49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99f49 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 99f4c jmp 0x99f58
            pc = 0x99f58; continue;
        }
        0x99f4e => {
            // block 99f4e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99f4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99f4e mov rax,QWORD PTR [rbp-0x798]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f868)) as u64;
            // 99f55 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            pc = 0x99f58;
        }
        0x99f58 => {
            // block 99f58
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99f58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99f58 lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // 99f5f test rax,rax
            fl = logic::<64>(rax & rax);
            // 99f62 je 0x99f91
            if fl.e() { pc = 0x99f91; continue; }
            pc = 0x99f64;
        }
        0x99f64 => {
            // block 99f64
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99f64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99f64 mov rbx,rdi
            rbx = rdi;
            // 99f67 mov rsi,QWORD PTR [rbp-0x768]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f898)) as u64;
            // 99f6e call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99f73 mov DWORD PTR [rbp-0x278],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd88), ((r13 as u32 as u64)) as u32);
            // 99f7a mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 99f81 mov rsi,rbx
            rsi = rbx;
            // 99f84 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99f89 mov rdi,rbx
            rdi = rbx;
            // 99f8c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x99f91;
        }
        0x99f91 => {
            // block 99f91
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99f91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99f91 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 99f96 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 99f9d lea rsi,[rip+0x99a67] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 99fa4 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99fa9 movzx ebx,BYTE PTR [rbp-0x2a8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd58)) as u64) as u32 as u64;
            // 99fb0 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 99fb3 jne 0x99fba
            if fl.ne() { pc = 0x99fba; continue; }
            pc = 0x99fb5;
        }
        0x99fb5 => {
            // block 99fb5
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99fb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99fb5 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 99fb8 jmp 0x99fc1
            pc = 0x99fc1; continue;
        }
        0x99fba => {
            // block 99fba
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99fba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99fba mov rbx,QWORD PTR [rbp-0x2a0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd60)) as u64;
            pc = 0x99fc1;
        }
        0x99fc1 => {
            // block 99fc1
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99fc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99fc1 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 99fc8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99fcd test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 99fd0 je 0x9a2f5
            if fl.e() { pc = 0x9a2f5; continue; }
            pc = 0x99fd6;
        }
        0x99fd6 => {
            // block 99fd6
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 99fd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 99fd6 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 99fdb lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 99fe2 lea rsi,[rip+0x99a22] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 99fe9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 99fee mov rax,QWORD PTR [rbp-0x2d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd30)) as u64;
            // 99ff5 mov QWORD PTR [rbp-0x2b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd48), (rax) as u64);
            // 99ffc mov rax,QWORD PTR [rbp-0x2e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd20)) as u64;
            // 9a003 mov rcx,QWORD PTR [rbp-0x2d8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd28)) as u64;
            // 9a00a mov QWORD PTR [rbp-0x2c0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd40), (rcx) as u64);
            // 9a011 mov QWORD PTR [rbp-0x2c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd38), (rax) as u64);
            // 9a018 xorps xmm0,xmm0
            x0 = 0;
            // 9a01b movaps XMMWORD PTR [rbp-0x2e0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fd20), x0);
            // 9a022 mov QWORD PTR [rbp-0x2d0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd30), (0x0u64) as u64);
            // 9a02d mov DWORD PTR [rbp-0x2b0],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd50), ((r13 as u32 as u64)) as u32);
            // 9a034 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a03b lea rbx,[rbp-0x2c8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 9a042 mov rsi,rbx
            rsi = rbx;
            // 9a045 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a04a mov rdi,rbx
            rdi = rbx;
            // 9a04d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a052 lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 9a059 jmp 0x9a2f0
            pc = 0x9a2f0; continue;
        }
        0x9a05e => {
            // block 9a05e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a05e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a05e mov rbx,QWORD PTR [rbp-0x420]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbe0)) as u64;
            pc = 0x9a065;
        }
        0x9a065 => {
            // block 9a065
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a065 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a065 lea rdi,[rbp-0x428]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbd8);
            // 9a06c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a071 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a074 je 0x9a102
            if fl.e() { pc = 0x9a102; continue; }
            pc = 0x9a07a;
        }
        0x9a07a => {
            // block 9a07a
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a07a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a07a mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a07f lea rdi,[rbp-0x460]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fba0);
            // 9a086 lea rsi,[rip+0x9995d] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 9a08d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a092 mov rax,QWORD PTR [rbp-0x450]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fbb0)) as u64;
            // 9a099 mov QWORD PTR [rbp-0x438],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc8), (rax) as u64);
            // 9a0a0 mov rax,QWORD PTR [rbp-0x460]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fba0)) as u64;
            // 9a0a7 mov rcx,QWORD PTR [rbp-0x458]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fba8)) as u64;
            // 9a0ae mov QWORD PTR [rbp-0x440],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbc0), (rcx) as u64);
            // 9a0b5 mov QWORD PTR [rbp-0x448],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbb8), (rax) as u64);
            // 9a0bc xorps xmm0,xmm0
            x0 = 0;
            // 9a0bf movaps XMMWORD PTR [rbp-0x460],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fba0), x0);
            // 9a0c6 mov QWORD PTR [rbp-0x450],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fbb0), (0x0u64) as u64);
            // 9a0d1 mov DWORD PTR [rbp-0x430],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fbd0), ((r13 as u32 as u64)) as u32);
            // 9a0d8 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a0df lea rbx,[rbp-0x448]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fbb8);
            // 9a0e6 mov rsi,rbx
            rsi = rbx;
            // 9a0e9 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a0ee mov rdi,rbx
            rdi = rbx;
            // 9a0f1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a0f6 lea rdi,[rbp-0x460]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fba0);
            // 9a0fd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a102;
        }
        0x9a102 => {
            // block 9a102
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a102 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a102 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a107 lea rdi,[rbp-0x478]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb88);
            // 9a10e lea rsi,[rip+0x9b757] # 0x13586c
            rsi = m.env.data_va(303161);
            // 9a115 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a11a movzx ebx,BYTE PTR [rbp-0x478]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fb88)) as u64) as u32 as u64;
            // 9a121 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9a124 jne 0x9a12b
            if fl.ne() { pc = 0x9a12b; continue; }
            pc = 0x9a126;
        }
        0x9a126 => {
            // block 9a126
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a126 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a126 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9a129 jmp 0x9a132
            pc = 0x9a132; continue;
        }
        0x9a12b => {
            // block 9a12b
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a12b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a12b mov rbx,QWORD PTR [rbp-0x470]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb90)) as u64;
            pc = 0x9a132;
        }
        0x9a132 => {
            // block 9a132
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a132 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a132 lea rdi,[rbp-0x478]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb88);
            // 9a139 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a13e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a141 lea rsi,[rbp-0x498]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb68);
            // 9a148 je 0x9a1d2
            if fl.e() { pc = 0x9a1d2; continue; }
            pc = 0x9a14e;
        }
        0x9a14e => {
            // block 9a14e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a14e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a14e mov rbx,rsi
            rbx = rsi;
            // 9a151 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a156 lea rdi,[rbp-0x4b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // 9a15d lea rsi,[rip+0x9b708] # 0x13586c
            rsi = m.env.data_va(303161);
            // 9a164 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a169 mov rax,QWORD PTR [rbp-0x4a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb60)) as u64;
            // 9a170 mov QWORD PTR [rbp-0x488],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb78), (rax) as u64);
            // 9a177 mov rax,QWORD PTR [rbp-0x4b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb50)) as u64;
            // 9a17e mov rcx,QWORD PTR [rbp-0x4a8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb58)) as u64;
            // 9a185 mov QWORD PTR [rbp-0x490],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb70), (rcx) as u64);
            // 9a18c mov QWORD PTR [rbp-0x498],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb68), (rax) as u64);
            // 9a193 xorps xmm0,xmm0
            x0 = 0;
            // 9a196 movaps XMMWORD PTR [rbp-0x4b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fb50), x0);
            // 9a19d mov QWORD PTR [rbp-0x4a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb60), (0x0u64) as u64);
            // 9a1a8 mov DWORD PTR [rbp-0x480],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb80), ((r13 as u32 as u64)) as u32);
            // 9a1af mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a1b6 mov rsi,rbx
            rsi = rbx;
            // 9a1b9 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a1be mov rdi,rbx
            rdi = rbx;
            // 9a1c1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a1c6 lea rdi,[rbp-0x4b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // 9a1cd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a1d2;
        }
        0x9a1d2 => {
            // block 9a1d2
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a1d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a1d2 mov rax,QWORD PTR [rbp-0x768]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f898)) as u64;
            // 9a1d9 movzx eax,BYTE PTR [rax]
            rax = (m.r8(rax) as u64) as u32 as u64;
            // 9a1dc test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 9a1de jne 0x9a1e5
            if fl.ne() { pc = 0x9a1e5; continue; }
            pc = 0x9a1e0;
        }
        0x9a1e0 => {
            // block 9a1e0
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a1e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a1e0 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 9a1e3 jmp 0x9a1ef
            pc = 0x9a1ef; continue;
        }
        0x9a1e5 => {
            // block 9a1e5
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a1e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a1e5 mov rax,QWORD PTR [rbp-0x798]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f868)) as u64;
            // 9a1ec mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            pc = 0x9a1ef;
        }
        0x9a1ef => {
            // block 9a1ef
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a1ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a1ef lea rdi,[rbp-0x4d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb30);
            // 9a1f6 test rax,rax
            fl = logic::<64>(rax & rax);
            // 9a1f9 je 0x9a228
            if fl.e() { pc = 0x9a228; continue; }
            pc = 0x9a1fb;
        }
        0x9a1fb => {
            // block 9a1fb
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a1fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a1fb mov rbx,rdi
            rbx = rdi;
            // 9a1fe mov rsi,QWORD PTR [rbp-0x768]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f898)) as u64;
            // 9a205 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a20a mov DWORD PTR [rbp-0x4b8],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb48), ((r13 as u32 as u64)) as u32);
            // 9a211 mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a218 mov rsi,rbx
            rsi = rbx;
            // 9a21b call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a220 mov rdi,rbx
            rdi = rbx;
            // 9a223 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a228;
        }
        0x9a228 => {
            // block 9a228
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a228 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a228 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9a22d lea rdi,[rbp-0x4e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb18);
            // 9a234 lea rsi,[rip+0x997d0] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 9a23b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a240 movzx ebx,BYTE PTR [rbp-0x4e8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fb18)) as u64) as u32 as u64;
            // 9a247 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9a24a jne 0x9a251
            if fl.ne() { pc = 0x9a251; continue; }
            pc = 0x9a24c;
        }
        0x9a24c => {
            // block 9a24c
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a24c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a24c shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9a24f jmp 0x9a258
            pc = 0x9a258; continue;
        }
        0x9a251 => {
            // block 9a251
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a251 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a251 mov rbx,QWORD PTR [rbp-0x4e0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb20)) as u64;
            pc = 0x9a258;
        }
        0x9a258 => {
            // block 9a258
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a258 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a258 lea rdi,[rbp-0x4e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb18);
            // 9a25f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a264 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a267 je 0x9a2f5
            if fl.e() { pc = 0x9a2f5; continue; }
            pc = 0x9a26d;
        }
        0x9a26d => {
            // block 9a26d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a26d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a26d mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 9a272 lea rdi,[rbp-0x520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fae0);
            // 9a279 lea rsi,[rip+0x9978b] # 0x133a0b
            rsi = m.env.data_va(295384);
            // 9a280 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a285 mov rax,QWORD PTR [rbp-0x510]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_faf0)) as u64;
            // 9a28c mov QWORD PTR [rbp-0x4f8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb08), (rax) as u64);
            // 9a293 mov rax,QWORD PTR [rbp-0x520]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fae0)) as u64;
            // 9a29a mov rcx,QWORD PTR [rbp-0x518]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fae8)) as u64;
            // 9a2a1 mov QWORD PTR [rbp-0x500],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb00), (rcx) as u64);
            // 9a2a8 mov QWORD PTR [rbp-0x508],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_faf8), (rax) as u64);
            // 9a2af xorps xmm0,xmm0
            x0 = 0;
            // 9a2b2 movaps XMMWORD PTR [rbp-0x520],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fae0), x0);
            // 9a2b9 mov QWORD PTR [rbp-0x510],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_faf0), (0x0u64) as u64);
            // 9a2c4 mov DWORD PTR [rbp-0x4f0],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb10), ((r13 as u32 as u64)) as u32);
            // 9a2cb mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a2d2 lea rbx,[rbp-0x508]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_faf8);
            // 9a2d9 mov rsi,rbx
            rsi = rbx;
            // 9a2dc call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a2e1 mov rdi,rbx
            rdi = rbx;
            // 9a2e4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a2e9 lea rdi,[rbp-0x520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fae0);
            pc = 0x9a2f0;
        }
        0x9a2f0 => {
            // block 9a2f0
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a2f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a2f0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a2f5;
        }
        0x9a2f5 => {
            // block 9a2f5
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a2f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a2f5 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 9a2fc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a301 mov rax,QWORD PTR [rbp-0x760]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8a0)) as u64;
            // 9a308 mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 9a30f mov rax,QWORD PTR [rbp-0x758]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8a8)) as u64;
            // 9a316 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // 9a31d mov rax,QWORD PTR [rbp-0x750]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b0)) as u64;
            // 9a324 mov QWORD PTR [rbp-0x1a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe60), (rax) as u64);
            // 9a32b lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 9a332 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a337 lea r13,[rbp-0x1a0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 9a33e mov rdi,r13
            rdi = r13;
            // 9a341 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a346 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 9a34d mov rsi,QWORD PTR [rbp-0x790]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f870)) as u64;
            // 9a354 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a359 lea rbx,[rbp-0x138]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // 9a360 mov rdi,rbx
            rdi = rbx;
            // 9a363 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a368 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 9a36b add r12,0x18
            { let (r, f) = add::<64>(r12, 0x18u64);
              r12 = r;
              fl = f; }
            // 9a36f cmp r15,0x6
            fl = sub::<64>(r15, 0x6u64).1;
            // 9a373 jb 0x997f8
            if fl.b() { pc = 0x997f8; continue; }
            pc = 0x9a379;
        }
        0x9a379 => {
            // block 9a379
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a379 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a379 mov rax,QWORD PTR [rbp-0x778]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f888)) as u64;
            // 9a380 cmp rax,0x8
            fl = sub::<64>(rax, 0x8u64).1;
            // 9a384 mov rcx,rax
            rcx = rax;
            // 9a387 jb 0x997ba
            if fl.b() { pc = 0x997ba; continue; }
            pc = 0x9a38d;
        }
        0x9a38d => {
            // block 9a38d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a38d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a38d lea rsi,[rip+0x9b4eb] # 0x13587f
            rsi = m.env.data_va(303180);
            // 9a394 lea rbx,[rbp-0x538]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fac8);
            // 9a39b mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // 9a3a0 mov rdi,rbx
            rdi = rbx;
            // 9a3a3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a3a8 mov esi,0x840
            rsi = (0x840u64) as u32 as u64;
            // 9a3ad mov r14,QWORD PTR [rbp-0x770]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f890)) as u64;
            // 9a3b4 mov rdi,r14
            rdi = r14;
            // 9a3b7 mov rdx,rbx
            rdx = rbx;
            // 9a3ba call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a3bf lea rdi,[rbp-0x538]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fac8);
            // 9a3c6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a3cb mov DWORD PTR [rbp-0x548],0x840
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fab8), (0x840u64) as u32);
            // 9a3d5 mov QWORD PTR [rbp-0x540],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fac0), (r14) as u64);
            // 9a3dc lea rsi,[rbp-0x548]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fab8);
            // 9a3e3 mov rdi,QWORD PTR [rbp-0x788]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f878)) as u64;
            // 9a3ea call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a3ef lea rsi,[rip+0x9af6b] # 0x135361
            rsi = m.env.data_va(301870);
            // 9a3f6 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9a3fa mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9a3ff mov rdi,rbx
            rdi = rbx;
            // 9a402 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a407 mov rdi,rbx
            rdi = rbx;
            // 9a40a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a40f lea rsi,[rip+0x995d4] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 9a416 lea rdi,[rbp-0x560]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_faa0);
            // 9a41d mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a422 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a427 movzx ebx,BYTE PTR [rbp-0x560]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_faa0)) as u64) as u32 as u64;
            // 9a42e test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9a431 jne 0x9a438
            if fl.ne() { pc = 0x9a438; continue; }
            pc = 0x9a433;
        }
        0x9a433 => {
            // block 9a433
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a433 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a433 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9a436 jmp 0x9a43f
            pc = 0x9a43f; continue;
        }
        0x9a438 => {
            // block 9a438
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a438 mov rbx,QWORD PTR [rbp-0x558]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_faa8)) as u64;
            pc = 0x9a43f;
        }
        0x9a43f => {
            // block 9a43f
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a43f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a43f lea rdi,[rbp-0x560]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_faa0);
            // 9a446 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a44b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a44e je 0x9a4e0
            if fl.e() { pc = 0x9a4e0; continue; }
            pc = 0x9a454;
        }
        0x9a454 => {
            // block 9a454
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a454 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a454 lea rsi,[rip+0x9958f] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 9a45b lea rdi,[rbp-0x5a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa60);
            // 9a462 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a467 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a46c mov rax,QWORD PTR [rbp-0x590]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa70)) as u64;
            // 9a473 mov QWORD PTR [rbp-0x570],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa90), (rax) as u64);
            // 9a47a mov rax,QWORD PTR [rbp-0x5a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa60)) as u64;
            // 9a481 mov rcx,QWORD PTR [rbp-0x598]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa68)) as u64;
            // 9a488 mov QWORD PTR [rbp-0x578],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa88), (rcx) as u64);
            // 9a48f mov QWORD PTR [rbp-0x580],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa80), (rax) as u64);
            // 9a496 xorps xmm0,xmm0
            x0 = 0;
            // 9a499 movaps XMMWORD PTR [rbp-0x5a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fa60), x0);
            // 9a4a0 mov QWORD PTR [rbp-0x590],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa70), (0x0u64) as u64);
            // 9a4ab mov DWORD PTR [rbp-0x568],0x840
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa98), (0x840u64) as u32);
            // 9a4b5 lea rsi,[rbp-0x580]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa80);
            // 9a4bc mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a4c3 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a4c8 lea rdi,[rbp-0x580]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa80);
            // 9a4cf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a4d4 lea rdi,[rbp-0x5a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa60);
            // 9a4db call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a4e0;
        }
        0x9a4e0 => {
            // block 9a4e0
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a4e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a4e0 lea rsi,[rip+0x9b385] # 0x13586c
            rsi = m.env.data_va(303161);
            // 9a4e7 lea rdi,[rbp-0x5b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa48);
            // 9a4ee mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a4f3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a4f8 movzx ebx,BYTE PTR [rbp-0x5b8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fa48)) as u64) as u32 as u64;
            // 9a4ff test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9a502 jne 0x9a509
            if fl.ne() { pc = 0x9a509; continue; }
            pc = 0x9a504;
        }
        0x9a504 => {
            // block 9a504
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a504 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a504 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9a507 jmp 0x9a510
            pc = 0x9a510; continue;
        }
        0x9a509 => {
            // block 9a509
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a509 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a509 mov rbx,QWORD PTR [rbp-0x5b0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa50)) as u64;
            pc = 0x9a510;
        }
        0x9a510 => {
            // block 9a510
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a510 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a510 lea rdi,[rbp-0x5b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa48);
            // 9a517 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a51c test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a51f je 0x9a5b1
            if fl.e() { pc = 0x9a5b1; continue; }
            pc = 0x9a525;
        }
        0x9a525 => {
            // block 9a525
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a525 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a525 lea rsi,[rip+0x9b340] # 0x13586c
            rsi = m.env.data_va(303161);
            // 9a52c lea rdi,[rbp-0x5f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa10);
            // 9a533 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a538 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a53d mov rax,QWORD PTR [rbp-0x5e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa20)) as u64;
            // 9a544 mov QWORD PTR [rbp-0x5c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa38), (rax) as u64);
            // 9a54b mov rax,QWORD PTR [rbp-0x5f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa10)) as u64;
            // 9a552 mov rcx,QWORD PTR [rbp-0x5e8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fa18)) as u64;
            // 9a559 mov QWORD PTR [rbp-0x5d0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa30), (rcx) as u64);
            // 9a560 mov QWORD PTR [rbp-0x5d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa28), (rax) as u64);
            // 9a567 xorps xmm0,xmm0
            x0 = 0;
            // 9a56a movaps XMMWORD PTR [rbp-0x5f0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fa10), x0);
            // 9a571 mov QWORD PTR [rbp-0x5e0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa20), (0x0u64) as u64);
            // 9a57c mov DWORD PTR [rbp-0x5c0],0x840
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fa40), (0x840u64) as u32);
            // 9a586 lea rsi,[rbp-0x5d8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fa28);
            // 9a58d mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a594 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a599 lea rdi,[rbp-0x5d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa28);
            // 9a5a0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a5a5 lea rdi,[rbp-0x5f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa10);
            // 9a5ac call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a5b1;
        }
        0x9a5b1 => {
            // block 9a5b1
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a5b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a5b1 xorps xmm0,xmm0
            x0 = 0;
            // 9a5b4 movaps XMMWORD PTR [rbp-0x610],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f9f0), x0);
            // 9a5bb mov QWORD PTR [rbp-0x600],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fa00), (0x0u64) as u64);
            // 9a5c6 lea rdi,[rbp-0x610]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9f0);
            // 9a5cd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a5d2 xorps xmm0,xmm0
            x0 = 0;
            // 9a5d5 movaps XMMWORD PTR [rbp-0x630],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f9d0), x0);
            // 9a5dc mov QWORD PTR [rbp-0x620],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f9e0), (0x0u64) as u64);
            // 9a5e7 lea rdi,[rbp-0x630]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9d0);
            // 9a5ee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a5f3 lea rsi,[rip+0x9b2a2] # 0x13589c
            rsi = m.env.data_va(303209);
            // 9a5fa lea rbx,[rbp-0x648]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_f9b8);
            // 9a601 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 9a606 mov rdi,rbx
            rdi = rbx;
            // 9a609 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a60e mov esi,0x841
            rsi = (0x841u64) as u32 as u64;
            // 9a613 mov rdi,r14
            rdi = r14;
            // 9a616 mov rdx,rbx
            rdx = rbx;
            // 9a619 call 0x72d00
            { let r = { let t = m.env.code_va(0x72d00); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a61e lea rdi,[rbp-0x648]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9b8);
            // 9a625 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a62a mov DWORD PTR [rbp-0x658],0x841
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f9a8), (0x841u64) as u32);
            // 9a634 mov QWORD PTR [rbp-0x650],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f9b0), (r14) as u64);
            // 9a63b lea rsi,[rbp-0x658]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f9a8);
            // 9a642 mov rdi,QWORD PTR [rbp-0x788]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f878)) as u64;
            // 9a649 call 0xebc78
            { let r = { let t = m.env.stub_va(0x445); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a64e lea rsi,[rip+0x9ad0c] # 0x135361
            rsi = m.env.data_va(301870);
            // 9a655 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9a659 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 9a65e mov rdi,rbx
            rdi = rbx;
            // 9a661 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a666 mov rdi,rbx
            rdi = rbx;
            // 9a669 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a66e lea rsi,[rip+0x99375] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 9a675 lea rdi,[rbp-0x670]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f990);
            // 9a67c mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a681 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a686 movzx ebx,BYTE PTR [rbp-0x670]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f990)) as u64) as u32 as u64;
            // 9a68d test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9a690 jne 0x9a697
            if fl.ne() { pc = 0x9a697; continue; }
            pc = 0x9a692;
        }
        0x9a692 => {
            // block 9a692
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a692 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a692 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9a695 jmp 0x9a69e
            pc = 0x9a69e; continue;
        }
        0x9a697 => {
            // block 9a697
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a697 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a697 mov rbx,QWORD PTR [rbp-0x668]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f998)) as u64;
            pc = 0x9a69e;
        }
        0x9a69e => {
            // block 9a69e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a69e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a69e lea rdi,[rbp-0x670]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f990);
            // 9a6a5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a6aa test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a6ad je 0x9a73f
            if fl.e() { pc = 0x9a73f; continue; }
            pc = 0x9a6b3;
        }
        0x9a6b3 => {
            // block 9a6b3
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a6b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a6b3 lea rsi,[rip+0x99330] # 0x1339ea
            rsi = m.env.data_va(295351);
            // 9a6ba lea rdi,[rbp-0x6b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f950);
            // 9a6c1 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a6c6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a6cb mov rax,QWORD PTR [rbp-0x6a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f960)) as u64;
            // 9a6d2 mov QWORD PTR [rbp-0x680],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f980), (rax) as u64);
            // 9a6d9 mov rax,QWORD PTR [rbp-0x6b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f950)) as u64;
            // 9a6e0 mov rcx,QWORD PTR [rbp-0x6a8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f958)) as u64;
            // 9a6e7 mov QWORD PTR [rbp-0x688],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f978), (rcx) as u64);
            // 9a6ee mov QWORD PTR [rbp-0x690],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f970), (rax) as u64);
            // 9a6f5 xorps xmm0,xmm0
            x0 = 0;
            // 9a6f8 movaps XMMWORD PTR [rbp-0x6b0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f950), x0);
            // 9a6ff mov QWORD PTR [rbp-0x6a0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f960), (0x0u64) as u64);
            // 9a70a mov DWORD PTR [rbp-0x678],0x841
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f988), (0x841u64) as u32);
            // 9a714 lea rsi,[rbp-0x690]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f970);
            // 9a71b mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a722 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a727 lea rdi,[rbp-0x690]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f970);
            // 9a72e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a733 lea rdi,[rbp-0x6b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f950);
            // 9a73a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a73f;
        }
        0x9a73f => {
            // block 9a73f
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a73f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a73f lea rsi,[rip+0x9b126] # 0x13586c
            rsi = m.env.data_va(303161);
            // 9a746 lea rdi,[rbp-0x6c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f938);
            // 9a74d mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a752 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a757 movzx ebx,BYTE PTR [rbp-0x6c8]
            rbx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_f938)) as u64) as u32 as u64;
            // 9a75e test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // 9a761 jne 0x9a768
            if fl.ne() { pc = 0x9a768; continue; }
            pc = 0x9a763;
        }
        0x9a763 => {
            // block 9a763
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a763 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a763 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 9a766 jmp 0x9a76f
            pc = 0x9a76f; continue;
        }
        0x9a768 => {
            // block 9a768
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a768 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a768 mov rbx,QWORD PTR [rbp-0x6c0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f940)) as u64;
            pc = 0x9a76f;
        }
        0x9a76f => {
            // block 9a76f
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a76f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a76f lea rdi,[rbp-0x6c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f938);
            // 9a776 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a77b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 9a77e je 0x9a810
            if fl.e() { pc = 0x9a810; continue; }
            pc = 0x9a784;
        }
        0x9a784 => {
            // block 9a784
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a784 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a784 lea rsi,[rip+0x9b0e1] # 0x13586c
            rsi = m.env.data_va(303161);
            // 9a78b lea rdi,[rbp-0x700]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f900);
            // 9a792 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 9a797 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a79c mov rax,QWORD PTR [rbp-0x6f0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f910)) as u64;
            // 9a7a3 mov QWORD PTR [rbp-0x6d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f928), (rax) as u64);
            // 9a7aa mov rax,QWORD PTR [rbp-0x700]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f900)) as u64;
            // 9a7b1 mov rcx,QWORD PTR [rbp-0x6f8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f908)) as u64;
            // 9a7b8 mov QWORD PTR [rbp-0x6e0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f920), (rcx) as u64);
            // 9a7bf mov QWORD PTR [rbp-0x6e8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f918), (rax) as u64);
            // 9a7c6 xorps xmm0,xmm0
            x0 = 0;
            // 9a7c9 movaps XMMWORD PTR [rbp-0x700],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f900), x0);
            // 9a7d0 mov QWORD PTR [rbp-0x6f0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f910), (0x0u64) as u64);
            // 9a7db mov DWORD PTR [rbp-0x6d0],0x841
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_f930), (0x841u64) as u32);
            // 9a7e5 lea rsi,[rbp-0x6e8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_f918);
            // 9a7ec mov rdi,QWORD PTR [rbp-0x748]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b8)) as u64;
            // 9a7f3 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a7f8 lea rdi,[rbp-0x6e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f918);
            // 9a7ff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a804 lea rdi,[rbp-0x700]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f900);
            // 9a80b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a810;
        }
        0x9a810 => {
            // block 9a810
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a810 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a810 xorps xmm0,xmm0
            x0 = 0;
            // 9a813 movaps XMMWORD PTR [rbp-0x720],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f8e0), x0);
            // 9a81a mov QWORD PTR [rbp-0x710],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8f0), (0x0u64) as u64);
            // 9a825 lea rdi,[rbp-0x720]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8e0);
            // 9a82c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a831 xorps xmm0,xmm0
            x0 = 0;
            // 9a834 movaps XMMWORD PTR [rbp-0x740],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_f8c0), x0);
            // 9a83b mov QWORD PTR [rbp-0x730],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_f8d0), (0x0u64) as u64);
            // 9a846 lea rdi,[rbp-0x740]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f8c0);
            // 9a84d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a852 add rsp,0x7f8
            { let (r, f) = add::<64>(rsp, 0x7f8u64);
              rsp = r;
              fl = f; }
            // 9a859 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 9a85a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 9a85c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 9a85e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 9a860 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 9a862 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 9a863 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x9a864 => {
            // block 9a864
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a864 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a864 mov r12,rax
            r12 = rax;
            // 9a867 jmp 0x9a9a9
            pc = 0x9a9a9; continue;
        }
        0x9a86c => {
            // block 9a86c
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a86c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a86c mov r12,rax
            r12 = rax;
            // 9a86f jmp 0x9a9c3
            pc = 0x9a9c3; continue;
        }
        0x9a874 => {
            // block 9a874
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a874 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a874 mov r12,rax
            r12 = rax;
            // 9a877 jmp 0x9aa14
            pc = 0x9aa14; continue;
        }
        0x9a87c => {
            // block 9a87c
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a87c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a87c mov r12,rax
            r12 = rax;
            // 9a87f jmp 0x9aa01
            pc = 0x9aa01; continue;
        }
        0x9a884 => {
            // block 9a884
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a884 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a884 mov r12,rax
            r12 = rax;
            // 9a887 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 9a88e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a893 jmp 0x9a9ed
            pc = 0x9a9ed; continue;
        }
        0x9a898 => {
            // block 9a898
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a898 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a898 mov r12,rax
            r12 = rax;
            // 9a89b lea rdi,[rbp-0x378]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc88);
            // 9a8a2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a8a7 lea rdi,[rbp-0x390]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc70);
            // 9a8ae jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a8b3 => {
            // block 9a8b3
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a8b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a8b3 mov r12,rax
            r12 = rax;
            // 9a8b6 lea rdi,[rbp-0x3b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc50);
            // 9a8bd jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a8c2 => {
            // block 9a8c2
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a8c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a8c2 mov r12,rax
            r12 = rax;
            // 9a8c5 lea rdi,[rbp-0x3e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc18);
            // 9a8cc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a8d1 lea rdi,[rbp-0x400]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fc00);
            // 9a8d8 jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a8dd => {
            // block 9a8dd
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a8dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a8dd mov r12,rax
            r12 = rax;
            // 9a8e0 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // 9a8e7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a8ec lea rdi,[rbp-0x340]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcc0);
            // 9a8f3 jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a8f8 => {
            // block 9a8f8
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a8f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a8f8 mov r12,rax
            r12 = rax;
            // 9a8fb lea rdi,[rbp-0x258]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fda8);
            // 9a902 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a907 lea rdi,[rbp-0x270]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd90);
            // 9a90e jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a913 => {
            // block 9a913
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a913 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a913 mov r12,rax
            r12 = rax;
            // 9a916 lea rdi,[rbp-0x290]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd70);
            // 9a91d jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a922 => {
            // block 9a922
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a922 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a922 mov r12,rax
            r12 = rax;
            // 9a925 lea rdi,[rbp-0x2c8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd38);
            // 9a92c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a931 lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // 9a938 jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a93a => {
            // block 9a93a
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a93a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a93a mov r12,rax
            r12 = rax;
            // 9a93d lea rdi,[rbp-0x498]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb68);
            // 9a944 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a949 lea rdi,[rbp-0x4b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // 9a950 jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a952 => {
            // block 9a952
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a952 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a952 mov r12,rax
            r12 = rax;
            // 9a955 lea rdi,[rbp-0x4d0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb30);
            // 9a95c jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a95e => {
            // block 9a95e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a95e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a95e mov r12,rax
            r12 = rax;
            // 9a961 lea rdi,[rbp-0x508]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_faf8);
            // 9a968 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a96d lea rdi,[rbp-0x520]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fae0);
            // 9a974 jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a976 => {
            // block 9a976
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a976 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a976 mov r12,rax
            r12 = rax;
            // 9a979 lea rdi,[rbp-0x208]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdf8);
            // 9a980 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a985 lea rdi,[rbp-0x220]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fde0);
            // 9a98c jmp 0x9a9a4
            pc = 0x9a9a4; continue;
        }
        0x9a98e => {
            // block 9a98e
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a98e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a98e mov r12,rax
            r12 = rax;
            // 9a991 lea rdi,[rbp-0x448]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fbb8);
            // 9a998 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a99d lea rdi,[rbp-0x460]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fba0);
            pc = 0x9a9a4;
        }
        0x9a9a4 => {
            // block 9a9a4
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a9a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a9a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a9a9;
        }
        0x9a9a9 => {
            // block 9a9a9
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a9a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a9a9 lea rbx,[rbp-0x138]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // 9a9b0 lea r13,[rbp-0x1a0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 9a9b7 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 9a9be call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9a9c3;
        }
        0x9a9c3 => {
            // block 9a9c3
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a9c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a9c3 mov rax,QWORD PTR [rbp-0x758]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8a8)) as u64;
            // 9a9ca mov rcx,QWORD PTR [rbp-0x760]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8a0)) as u64;
            // 9a9d1 mov QWORD PTR [rbp-0x1a8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rcx) as u64);
            // 9a9d8 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // 9a9df mov rax,QWORD PTR [rbp-0x750]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f8b0)) as u64;
            // 9a9e6 mov QWORD PTR [rbp-0x1a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe60), (rax) as u64);
            pc = 0x9a9ed;
        }
        0x9a9ed => {
            // block 9a9ed
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9a9ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9a9ed lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 9a9f4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9a9f9 mov rdi,r13
            rdi = r13;
            // 9a9fc call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9aa01;
        }
        0x9aa01 => {
            // block 9aa01
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa01 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 9aa08 mov rsi,QWORD PTR [rbp-0x790]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_f870)) as u64;
            // 9aa0f call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9aa14;
        }
        0x9aa14 => {
            // block 9aa14
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa14 mov rdi,rbx
            rdi = rbx;
            // 9aa17 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9aa1c;
        }
        0x9aa1c => {
            // block 9aa1c
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa1c mov rdi,r12
            rdi = r12;
            // 9aa1f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aa24 mov r12,rax
            r12 = rax;
            pc = 0x9aa27;
        }
        0x9aa27 => {
            // block 9aa27
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa27 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 9aa2b mov rdi,rbx
            rdi = rbx;
            // 9aa2e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aa33 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 9aa36 jne 0x9aa27
            if fl.ne() { pc = 0x9aa27; continue; }
            pc = 0x9aa38;
        }
        0x9aa38 => {
            // block 9aa38
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa38 jmp 0x9aac0
            pc = 0x9aac0; continue;
        }
        0x9aa3d => {
            // block 9aa3d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa3d mov r12,rax
            r12 = rax;
            // 9aa40 lea rdi,[rbp-0x538]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fac8);
            // 9aa47 jmp 0x9aab3
            pc = 0x9aab3; continue;
        }
        0x9aa49 => {
            // block 9aa49
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa49 mov r12,rax
            r12 = rax;
            // 9aa4c lea rdi,[rbp-0x648]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f9b8);
            // 9aa53 jmp 0x9aab3
            pc = 0x9aab3; continue;
        }
        0x9aa55 => {
            // block 9aa55
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa55 mov r12,rax
            r12 = rax;
            // 9aa58 lea rdi,[rbp-0x580]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa80);
            // 9aa5f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aa64 lea rdi,[rbp-0x5a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa60);
            // 9aa6b jmp 0x9aab3
            pc = 0x9aab3; continue;
        }
        0x9aa6d => {
            // block 9aa6d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa6d mov r12,rax
            r12 = rax;
            // 9aa70 lea rdi,[rbp-0x5d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa28);
            // 9aa77 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aa7c lea rdi,[rbp-0x5f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fa10);
            // 9aa83 jmp 0x9aab3
            pc = 0x9aab3; continue;
        }
        0x9aa85 => {
            // block 9aa85
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa85 mov r12,rax
            r12 = rax;
            // 9aa88 lea rdi,[rbp-0x690]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f970);
            // 9aa8f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aa94 lea rdi,[rbp-0x6b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f950);
            // 9aa9b jmp 0x9aab3
            pc = 0x9aab3; continue;
        }
        0x9aa9d => {
            // block 9aa9d
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aa9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aa9d mov r12,rax
            r12 = rax;
            // 9aaa0 lea rdi,[rbp-0x6e8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f918);
            // 9aaa7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aaac lea rdi,[rbp-0x700]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_f900);
            pc = 0x9aab3;
        }
        0x9aab3 => {
            // block 9aab3
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aab3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aab3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aab8 jmp 0x9aa1c
            pc = 0x9aa1c; continue;
        }
        0x9aabd => {
            // block 9aabd
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aabd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aabd mov r12,rax
            r12 = rax;
            pc = 0x9aac0;
        }
        0x9aac0 => {
            // block 9aac0
            #[cfg(feature = "trace")] eprintln!("f_00099644 pc 9aac0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9aac0 lea rdi,[rip+0xae8e9] # 0x1493b0
            rdi = m.env.data_va(383869);
            // 9aac7 call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9aacc jmp 0x9aa1c
            pc = 0x9aa1c; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
