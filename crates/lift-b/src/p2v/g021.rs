//! Mechanically lifted functions (scripts/lift_run.py), module g021. Each one is checked against the original
//! in the world harness (tests/world_g021.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xd6c76 size=14
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6c76<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd6c76;
    loop {
        match pc {
        0xd6c76 => {
            // block d6c76
            // d6c76 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6c77 mov rbp,rsp
            rbp = rsp;
            // d6c7a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d6c7c xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d6c7e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6c7f jmp 0xd6814
            { let r = { let t = m.env.code_va(0xd6814); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b695b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b695d je 0xb6ad6
            if fl.e() { pc = 0xb6ad6; continue; }
            pc = 0xb6963;
        }
        0xb6963 => {
            // block b6963
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
            // b6971 mov rdi,QWORD PTR [rip+0x89808] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // b6978 lea rsi,[rip+0x813f8] # 0x137d77
            rsi = m.env.data_va(312644);
            // b697f mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // b6984 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x853); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x5c0a9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // b69ec lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // b69f0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b69f3 jmp 0xb69fd
            pc = 0xb69fd; continue;
        }
        0xb69f5 => {
            // block b69f5
            // b69f5 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b69f9 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xb69fd;
        }
        0xb69fd => {
            // block b69fd
            // b69fd mov rdi,r15
            rdi = r15;
            // b6a00 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a05 lea rsi,[rip+0x8139a] # 0x137da6
            rsi = m.env.data_va(312691);
            // b6a0c mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // b6a11 mov rdi,rax
            rdi = rax;
            // b6a14 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a19 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // b6a1e mov rdi,rax
            rdi = rax;
            // b6a21 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a26 lea rsi,[rip+0x8138e] # 0x137dbb
            rsi = m.env.data_va(312712);
            // b6a2d mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // b6a32 mov rdi,rax
            rdi = rax;
            // b6a35 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x5c0a9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // b6a54 lea rsi,[rbp-0x57]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa9);
            // b6a58 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b6a5b jmp 0xb6a65
            pc = 0xb6a65; continue;
        }
        0xb6a5d => {
            // block b6a5d
            // b6a5d mov rdx,QWORD PTR [rbp-0x50]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // b6a61 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0xb6a65;
        }
        0xb6a65 => {
            // block b6a65
            // b6a65 mov rdi,r15
            rdi = r15;
            // b6a68 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a83 mov rsi,QWORD PTR [rip+0x89706] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b6a8a lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // b6a8e call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a93 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b6a96 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b6a9a mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b6a9f mov rdi,rax
            rdi = rax;
            // b6aa2 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6aa4 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b6aa7 lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // b6aab call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6ab0 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b6ab4 mov rdi,rbx
            rdi = rbx;
            // b6ab7 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6abc mov rdi,rbx
            rdi = rbx;
            // b6abf call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6ac4 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b6ac8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6acd lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b6ad1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb6ad6;
        }
        0xb6ad6 => {
            // block b6ad6
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
            // b6ae4 mov rbx,rax
            rbx = rax;
            // b6ae7 jmp 0xb6b03
            pc = 0xb6b03; continue;
        }
        0xb6ae9 => {
            // block b6ae9
            // b6ae9 mov rbx,rax
            rbx = rax;
            // b6aec jmp 0xb6afa
            pc = 0xb6afa; continue;
        }
        0xb6aee => {
            // block b6aee
            // b6aee mov rbx,rax
            rbx = rax;
            // b6af1 lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // b6af5 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb6afa;
        }
        0xb6afa => {
            // block b6afa
            // b6afa lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b6afe call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb6b03;
        }
        0xb6b03 => {
            // block b6b03
            // b6b03 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b6b07 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6b0c mov rdi,rbx
            rdi = rbx;
            // b6b0f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0e8e size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0e8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0e8e;
    loop {
        match pc {
        0xe0e8e => {
            // block e0e8e
            // e0e8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0e8f mov rbp,rsp
            rbp = rsp;
            // e0e92 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e96 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0e99 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0e9d call QWORD PTR [rax+0x1ad8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ad8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ea3 mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0ea6 shr eax,0x1e
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1eu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0ea9 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0ead pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0eae ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0cbc size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0cbc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0cbc;
    loop {
        match pc {
        0xe0cbc => {
            // block e0cbc
            // e0cbc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0cbd mov rbp,rsp
            rbp = rsp;
            // e0cc0 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0cc4 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0cc7 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0ccb call QWORD PTR [rax+0x1ac8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0cd1 mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0cd4 shr eax,0x4
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0cd7 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0cda add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0cde pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0cdf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0e26 size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0e26<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0e26;
    loop {
        match pc {
        0xe0e26 => {
            // block e0e26
            // e0e26 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0e27 mov rbp,rsp
            rbp = rsp;
            // e0e2a sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e2e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0e31 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0e35 call QWORD PTR [rax+0x1ae8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0e3b mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0e3e shr eax,0x8
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0e41 and eax,0x3
            { let r = (rax as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0e44 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0e49 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2ee6 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002ee6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2ee6;
    loop {
        match pc {
        0x2ee6 => {
            // block 2ee6
            // 2ee6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2ee7 mov rbp,rsp
            rbp = rsp;
            // 2eea sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2eee mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 2ef1 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // 2ef5 call QWORD PTR [rax+0x9a8]
            { let r = { let t = m.r64(rax.wrapping_add(0x9a8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2efb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2efd je 0x2f07
            if fl.e() { pc = 0x2f07; continue; }
            pc = 0x2eff;
        }
        0x2eff => {
            // block 2eff
            // 2eff mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2f01 test BYTE PTR [rbp-0x3],0x2
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fffd)) as u64 & 0x2u64);
            // 2f05 jne 0x2f09
            if fl.ne() { pc = 0x2f09; continue; }
            pc = 0x2f07;
        }
        0x2f07 => {
            // block 2f07
            // 2f07 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2f09;
        }
        0x2f09 => {
            // block 2f09
            // 2f09 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f0d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f0e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe478a size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e478a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe478a;
    loop {
        match pc {
        0xe478a => {
            // block e478a
            // e478a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e478b mov rbp,rsp
            rbp = rsp;
            // e478e sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e4792 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e4795 mov r9,QWORD PTR [rax]
            r9 = m.r64(rax) as u64;
            // e4798 mov esi,DWORD PTR [rdi+0x30]
            rsi = (m.r32(rdi.wrapping_add(0x30)) as u64) as u32 as u64;
            // e479b add esi,0x4
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x4u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e479e lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e47a2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e47a7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e47aa mov rdi,rax
            rdi = rax;
            // e47ad call QWORD PTR [r9+0x38]
            { let r = { let t = m.r64(r9.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e47b1 mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e47b4 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e47b8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e47b9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe384 size=52
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e384<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe384;
    loop {
        match pc {
        0xe384 => {
            // block e384
            // e384 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e385 mov rbp,rsp
            rbp = rsp;
            // e388 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e389 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e38a mov rbx,rsi
            rbx = rsi;
            // e38d mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // e394 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e397 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // e39b mov esi,0x166
            rsi = (0x166u64) as u32 as u64;
            // e3a0 mov ecx,0x3
            rcx = (0x3u64) as u32 as u64;
            // e3a5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e3a8 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3ab mov ecx,DWORD PTR [rbp-0xc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // e3ae mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            // e3b0 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e3b4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e3b5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e3b6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xebfe size=52
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000ebfe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xebfe;
    loop {
        match pc {
        0xebfe => {
            // block ebfe
            // ebfe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ebff mov rbp,rsp
            rbp = rsp;
            // ec02 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // ec06 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // ec0d mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // ec10 lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // ec14 mov esi,0x28dc
            rsi = (0x28dcu64) as u32 as u64;
            // ec19 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ec1e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ec21 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ec24 cmp DWORD PTR [rbp-0x4],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64, 0x0u64).1;
            // ec28 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // ec2b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // ec2f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ec30 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7d52 size=52
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7d52<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7d52;
    loop {
        match pc {
        0xb7d52 => {
            // block b7d52
            // b7d52 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7d53 mov rbp,rsp
            rbp = rsp;
            // b7d56 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7d5a mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // b7d61 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b7d64 lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // b7d68 mov esi,0x16
            rsi = (0x16u64) as u32 as u64;
            // b7d6d mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b7d72 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b7d75 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7d78 test BYTE PTR [rbp-0x4],0x8
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64 & 0x8u64);
            // b7d7c sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // b7d7f add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7d83 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7d84 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7dba size=52
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7dba<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7dba;
    loop {
        match pc {
        0xb7dba => {
            // block b7dba
            // b7dba push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7dbb mov rbp,rsp
            rbp = rsp;
            // b7dbe sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7dc2 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // b7dc9 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b7dcc lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // b7dd0 mov esi,0x51
            rsi = (0x51u64) as u32 as u64;
            // b7dd5 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // b7dda xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b7ddd call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7de0 cmp DWORD PTR [rbp-0x4],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64, 0x1u64).1;
            // b7de4 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // b7de7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7deb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7dec ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe114 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e114<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe114;
    loop {
        match pc {
        0xe114 => {
            // block e114
            // e114 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e115 mov rbp,rsp
            rbp = rsp;
            // e118 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e119 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e11a mov rbx,rsi
            rbx = rsi;
            // e11d mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // e124 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e127 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // e12b mov esi,0x162
            rsi = (0x162u64) as u32 as u64;
            // e130 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // e135 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e138 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e13b cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // e13f setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            // e142 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e146 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e147 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e148 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7894 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7894<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7894;
    loop {
        match pc {
        0xb7894 => {
            // block b7894
            // b7894 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7895 mov rbp,rsp
            rbp = rsp;
            // b7898 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7899 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b789a mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // b78a1 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b78a4 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // b78a8 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // b78aa mov esi,0x36
            rsi = (0x36u64) as u32 as u64;
            // b78af mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b78b4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b78b7 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b78ba test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b78bc cmovne ebx,DWORD PTR [rbp-0xc]
            if fl.ne() { rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            // b78c0 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // b78c2 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b78c6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b78c7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b78c8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8154 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8154<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb8154;
    loop {
        match pc {
        0xb8154 => {
            // block b8154
            // b8154 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8155 mov rbp,rsp
            rbp = rsp;
            // b8158 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b815c mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // b8163 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b8166 lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // b816a mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // b816f mov ecx,0x40000
            rcx = (0x40000u64) as u32 as u64;
            // b8174 mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // b817a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b817d cmp DWORD PTR [rbp-0x4],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64, 0x1u64).1;
            // b8181 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // b8184 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b8188 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b8189 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb818a size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b818a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb818a;
    loop {
        match pc {
        0xb818a => {
            // block b818a
            // b818a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b818b mov rbp,rsp
            rbp = rsp;
            // b818e sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b8192 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // b8199 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b819c lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // b81a0 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // b81a5 mov ecx,0x80000
            rcx = (0x80000u64) as u32 as u64;
            // b81aa mov r8d,0x13
            r8 = (0x13u64) as u32 as u64;
            // b81b0 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b81b3 cmp DWORD PTR [rbp-0x4],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64, 0x1u64).1;
            // b81b7 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // b81ba add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b81be pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b81bf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4754 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4754<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe4754;
    loop {
        match pc {
        0xe4754 => {
            // block e4754
            // e4754 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4755 mov rbp,rsp
            rbp = rsp;
            // e4758 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e475c mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e475f mov r9,QWORD PTR [rax]
            r9 = m.r64(rax) as u64;
            // e4762 mov esi,DWORD PTR [rdi+0x30]
            rsi = (m.r32(rdi.wrapping_add(0x30)) as u64) as u32 as u64;
            // e4765 add esi,0x4
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x4u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e4768 lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e476c mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e4771 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e4774 mov rdi,rax
            rdi = rax;
            // e4777 call QWORD PTR [r9+0x38]
            { let r = { let t = m.r64(r9.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e477b mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e477e test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // e4780 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // e4783 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e4787 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4788 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x718c size=56
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000718c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x718c;
    loop {
        match pc {
        0x718c => {
            // block 718c
            // 718c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 718d mov rbp,rsp
            rbp = rsp;
            // 7190 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7191 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 7192 mov rbx,rsi
            rbx = rsi;
            // 7195 movzx eax,BYTE PTR [rbx]
            rax = (m.r8(rbx) as u64) as u32 as u64;
            // 7198 mov DWORD PTR [rbp-0xc],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((rax as u32 as u64)) as u32);
            // 719b mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 719e lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 71a2 mov esi,0x83
            rsi = (0x83u64) as u32 as u64;
            // 71a7 mov ecx,0x80000000
            rcx = (0x80000000u64) as u32 as u64;
            // 71ac mov r8d,0x1f
            r8 = (0x1fu64) as u32 as u64;
            // 71b2 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 71b5 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 71b9 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            // 71bc add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 71c0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 71c1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 71c2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe1c6 size=56
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e1c6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe1c6;
    loop {
        match pc {
        0xe1c6 => {
            // block e1c6
            // e1c6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e1c7 mov rbp,rsp
            rbp = rsp;
            // e1ca push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e1cb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e1cc mov rbx,rsi
            rbx = rsi;
            // e1cf mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // e1d6 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e1d9 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // e1dd mov esi,0x162
            rsi = (0x162u64) as u32 as u64;
            // e1e2 mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // e1e7 mov r8d,0x2
            r8 = (0x2u64) as u32 as u64;
            // e1ed call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1f0 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // e1f4 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            // e1f7 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e1fb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e1fc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e1fd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe27a size=56
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e27a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe27a;
    loop {
        match pc {
        0xe27a => {
            // block e27a
            // e27a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e27b mov rbp,rsp
            rbp = rsp;
            // e27e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e27f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e280 mov rbx,rsi
            rbx = rsi;
            // e283 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // e28a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e28d lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // e291 mov esi,0x166
            rsi = (0x166u64) as u32 as u64;
            // e296 mov ecx,0x100
            rcx = (0x100u64) as u32 as u64;
            // e29b mov r8d,0x8
            r8 = (0x8u64) as u32 as u64;
            // e2a1 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2a4 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // e2a8 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            // e2ab add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e2af pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2b0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2b1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe32e size=56
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e32e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe32e;
    loop {
        match pc {
        0xe32e => {
            // block e32e
            // e32e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e32f mov rbp,rsp
            rbp = rsp;
            // e332 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e333 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e334 mov rbx,rsi
            rbx = rsi;
            // e337 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // e33e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e341 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // e345 mov esi,0x166
            rsi = (0x166u64) as u32 as u64;
            // e34a mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // e34f mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // e355 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e358 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // e35c setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            // e35f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e363 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e364 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e365 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x442e6 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000442e6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x442e6;
    loop {
        match pc {
        0x442e6 => {
            // block 442e6
            // 442e6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 442e7 mov rbp,rsp
            rbp = rsp;
            // 442ea push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 442eb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 442ec mov rbx,rsi
            rbx = rsi;
            // 442ef mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 442f6 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 442f9 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 442fd mov esi,0x2886
            rsi = (0x2886u64) as u32 as u64;
            // 44302 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 44307 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4430a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4430d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4430f je 0x44318
            if fl.e() { pc = 0x44318; continue; }
            pc = 0x44311;
        }
        0x44311 => {
            // block 44311
            // 44311 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 44315 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0x44318;
        }
        0x44318 => {
            // block 44318
            // 44318 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4431c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4431d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4431e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8f44 size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008f44<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x8f44;
    loop {
        match pc {
        0x8f44 => {
            // block 8f44
            // 8f44 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8f45 mov rbp,rsp
            rbp = rsp;
            // 8f48 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8f4a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8f4b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8f4f mov r14,rdx
            r14 = rdx;
            // 8f52 mov BYTE PTR [rbp-0x11],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffef), (0x0u64) as u8);
            // 8f56 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 8f59 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 8f5b lea rcx,[rbp-0x11]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffef);
            // 8f5f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 8f61 call QWORD PTR [rax+0x10f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x10f0)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8f67 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8f69 je 0x8f75
            if fl.e() { pc = 0x8f75; continue; }
            pc = 0x8f6b;
        }
        0x8f6b => {
            // block 8f6b
            // 8f6b mov al,BYTE PTR [rbp-0x11]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64) as u8 as u64);
            // 8f6e xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 8f70 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // 8f73 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x8f75;
        }
        0x8f75 => {
            // block 8f75
            // 8f75 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 8f77 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8f7b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8f7c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8f7e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8f7f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd274 size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d274<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd274;
    loop {
        match pc {
        0xd274 => {
            // block d274
            // d274 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d275 mov rbp,rsp
            rbp = rsp;
            // d278 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d279 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d27a mov rbx,rsi
            rbx = rsi;
            // d27d mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // d284 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d287 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // d28b mov esi,0x5f
            rsi = (0x5fu64) as u32 as u64;
            // d290 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // d295 mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // d29b call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d29e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d2a0 je 0xd2a9
            if fl.e() { pc = 0xd2a9; continue; }
            pc = 0xd2a2;
        }
        0xd2a2 => {
            // block d2a2
            // d2a2 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // d2a6 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0xd2a9;
        }
        0xd2a9 => {
            // block d2a9
            // d2a9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d2ad pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d2ae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d2af ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd2ec size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d2ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd2ec;
    loop {
        match pc {
        0xd2ec => {
            // block d2ec
            // d2ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d2ed mov rbp,rsp
            rbp = rsp;
            // d2f0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d2f1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d2f2 mov rbx,rsi
            rbx = rsi;
            // d2f5 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // d2fc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d2ff lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // d303 mov esi,0x15
            rsi = (0x15u64) as u32 as u64;
            // d308 mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // d30d mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // d313 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d316 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d318 je 0xd321
            if fl.e() { pc = 0xd321; continue; }
            pc = 0xd31a;
        }
        0xd31a => {
            // block d31a
            // d31a cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // d31e setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0xd321;
        }
        0xd321 => {
            // block d321
            // d321 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d325 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d326 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d327 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xde36 size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000de36<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xde36;
    loop {
        match pc {
        0xde36 => {
            // block de36
            // de36 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // de37 mov rbp,rsp
            rbp = rsp;
            // de3a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // de3b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // de3c mov rbx,rsi
            rbx = rsi;
            // de3f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // de42 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // de46 mov esi,0x136
            rsi = (0x136u64) as u32 as u64;
            // de4b mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // de50 mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // de56 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de59 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // de5b je 0xde68
            if fl.e() { pc = 0xde68; continue; }
            pc = 0xde5d;
        }
        0xde5d => {
            // block de5d
            // de5d cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // de61 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            // de64 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // de66 jmp 0xde6a
            pc = 0xde6a; continue;
        }
        0xde68 => {
            // block de68
            // de68 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xde6a;
        }
        0xde6a => {
            // block de6a
            // de6a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // de6e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // de6f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // de70 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43354 size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043354<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x43354;
    loop {
        match pc {
        0x43354 => {
            // block 43354
            // 43354 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43355 mov rbp,rsp
            rbp = rsp;
            // 43358 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43359 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4335a mov rbx,rsi
            rbx = rsi;
            // 4335d mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 43364 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43367 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 4336b mov esi,0x7f
            rsi = (0x7fu64) as u32 as u64;
            // 43370 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // 43375 mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // 4337b call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4337e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43380 je 0x43389
            if fl.e() { pc = 0x43389; continue; }
            pc = 0x43382;
        }
        0x43382 => {
            // block 43382
            // 43382 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 43386 sete BYTE PTR [rbx]
            m.w8(rbx, ((fl.e() as u64)) as u8);
            pc = 0x43389;
        }
        0x43389 => {
            // block 43389
            // 43389 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4338d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4338e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4338f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7e22 size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7e22<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7e22;
    loop {
        match pc {
        0xb7e22 => {
            // block b7e22
            // b7e22 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7e23 mov rbp,rsp
            rbp = rsp;
            // b7e26 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7e27 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b7e28 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b7e2a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b7e2d lea rsi,[rbp-0xc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // b7e31 call QWORD PTR [rax+0xa40]
            { let r = { let t = m.r64(rax.wrapping_add(0xa40)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7e37 cmp ebx,0x1
            fl = sub::<32>((rbx as u32 as u64), 0x1u64).1;
            // b7e3a jne 0xb7e44
            if fl.ne() { pc = 0xb7e44; continue; }
            pc = 0xb7e3c;
        }
        0xb7e3c => {
            // block b7e3c
            // b7e3c mov eax,DWORD PTR [rbp-0xc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // b7e3f shr eax,0xb
            { let (r, f) = shr::<32>((rax as u32 as u64), 0xbu64, fl); rax = (r) as u32 as u64; fl = f; }
            // b7e42 jmp 0xb7e4e
            pc = 0xb7e4e; continue;
        }
        0xb7e44 => {
            // block b7e44
            // b7e44 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // b7e46 jne 0xb7e55
            if fl.ne() { pc = 0xb7e55; continue; }
            pc = 0xb7e48;
        }
        0xb7e48 => {
            // block b7e48
            // b7e48 mov eax,DWORD PTR [rbp-0xc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // b7e4b shr eax,0x3
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x3u64, fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0xb7e4e;
        }
        0xb7e4e => {
            // block b7e4e
            // b7e4e not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // b7e50 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b7e53 jmp 0xb7e57
            pc = 0xb7e57; continue;
        }
        0xb7e55 => {
            // block b7e55
            // b7e55 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb7e57;
        }
        0xb7e57 => {
            // block b7e57
            // b7e57 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b7e5b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7e5c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7e5d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb730a size=62
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b730a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb730a;
    loop {
        match pc {
        0xb730a => {
            // block b730a
            // b730a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b730b mov rbp,rsp
            rbp = rsp;
            // b730e sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7312 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // b7319 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b731c lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // b7320 mov esi,0x30
            rsi = (0x30u64) as u32 as u64;
            // b7325 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b732a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b732d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7330 mov cx,0xffff
            rcx = (rcx & !0xffff) | ((0xffffu64) as u16 as u64);
            // b7334 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7336 je 0xb733e
            if fl.e() { pc = 0xb733e; continue; }
            pc = 0xb7338;
        }
        0xb7338 => {
            // block b7338
            // b7338 mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // b733b movzx ecx,ah
            rcx = (((rax >> 8) as u8 as u64)) as u32 as u64;
            pc = 0xb733e;
        }
        0xb733e => {
            // block b733e
            // b733e movsx eax,cx
            rax = (sx::<16>((rcx as u16 as u64)) as u64) as u32 as u64;
            // b7341 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7345 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7346 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdc66 size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000dc66<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdc66;
    loop {
        match pc {
        0xdc66 => {
            // block dc66
            // dc66 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dc67 mov rbp,rsp
            rbp = rsp;
            // dc6a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dc6b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // dc6c mov rbx,rsi
            rbx = rsi;
            // dc6f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc72 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // dc76 mov esi,0x136
            rsi = (0x136u64) as u32 as u64;
            // dc7b mov ecx,0x1000
            rcx = (0x1000u64) as u32 as u64;
            // dc80 mov r8d,0xc
            r8 = (0xcu64) as u32 as u64;
            // dc86 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc89 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // dc8b je 0xdc9d
            if fl.e() { pc = 0xdc9d; continue; }
            pc = 0xdc8d;
        }
        0xdc8d => {
            // block dc8d
            // dc8d cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // dc91 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // dc94 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // dc97 mov DWORD PTR [rbx],eax
            m.w32(rbx, ((rax as u32 as u64)) as u32);
            // dc99 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // dc9b jmp 0xdc9f
            pc = 0xdc9f; continue;
        }
        0xdc9d => {
            // block dc9d
            // dc9d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xdc9f;
        }
        0xdc9f => {
            // block dc9f
            // dc9f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // dca3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dca4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dca5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdce6 size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000dce6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdce6;
    loop {
        match pc {
        0xdce6 => {
            // block dce6
            // dce6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dce7 mov rbp,rsp
            rbp = rsp;
            // dcea push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dceb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // dcec mov rbx,rsi
            rbx = rsi;
            // dcef mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dcf2 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // dcf6 mov esi,0x136
            rsi = (0x136u64) as u32 as u64;
            // dcfb mov ecx,0x200
            rcx = (0x200u64) as u32 as u64;
            // dd00 mov r8d,0x9
            r8 = (0x9u64) as u32 as u64;
            // dd06 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd09 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // dd0b je 0xdd1d
            if fl.e() { pc = 0xdd1d; continue; }
            pc = 0xdd0d;
        }
        0xdd0d => {
            // block dd0d
            // dd0d cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // dd11 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // dd14 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // dd17 mov DWORD PTR [rbx],eax
            m.w32(rbx, ((rax as u32 as u64)) as u32);
            // dd19 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // dd1b jmp 0xdd1f
            pc = 0xdd1f; continue;
        }
        0xdd1d => {
            // block dd1d
            // dd1d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xdd1f;
        }
        0xdd1f => {
            // block dd1f
            // dd1f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // dd23 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dd24 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dd25 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0b6a size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0b6a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0b6a;
    loop {
        match pc {
        0xe0b6a => {
            // block e0b6a
            // e0b6a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0b6b mov rbp,rsp
            rbp = rsp;
            // e0b6e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0b70 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0b71 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0b75 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e0b78 mov rbx,rdi
            rbx = rdi;
            // e0b7b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0b7e lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e0b82 call QWORD PTR [rax+0x1ac8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0b88 mov esi,DWORD PTR [rbp-0x14]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // e0b8b and esi,0xfffffff0
            { let r = (rsi as u32 as u64) & 0xfffffff0u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0b8e or esi,r14d
            { let r = (rsi as u32 as u64) | (r14 as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0b91 mov DWORD PTR [rbp-0x14],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rsi as u32 as u64)) as u32);
            // e0b94 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0b97 mov rdi,rbx
            rdi = rbx;
            // e0b9a call QWORD PTR [rax+0x1ac0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ba0 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0ba4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0ba5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0ba7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0ba8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8f02 size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008f02<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x8f02;
    loop {
        match pc {
        0x8f02 => {
            // block 8f02
            // 8f02 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8f03 mov rbp,rsp
            rbp = rsp;
            // 8f06 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8f08 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8f09 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8f0d mov rbx,rcx
            rbx = rcx;
            // 8f10 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 8f13 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 8f1a mov BYTE PTR [rbx],0x0
            m.w8(rbx, (0x0u64) as u8);
            // 8f1d mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 8f20 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 8f24 call QWORD PTR [rax+0x10c8]
            { let r = { let t = m.r64(rax.wrapping_add(0x10c8)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8f2a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8f2c je 0x8f39
            if fl.e() { pc = 0x8f39; continue; }
            pc = 0x8f2e;
        }
        0x8f2e => {
            // block 8f2e
            // 8f2e cmp DWORD PTR [rbp-0x14],r14d
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, (r14 as u32 as u64)).1;
            // 8f32 sete BYTE PTR [rbx]
            m.w8(rbx, ((fl.e() as u64)) as u8);
            // 8f35 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 8f37 jmp 0x8f3b
            pc = 0x8f3b; continue;
        }
        0x8f39 => {
            // block 8f39
            // 8f39 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x8f3b;
        }
        0x8f3b => {
            // block 8f3b
            // 8f3b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8f3f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8f40 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8f42 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8f43 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1007e size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001007e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1007e;
    loop {
        match pc {
        0x1007e => {
            // block 1007e
            // 1007e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1007f mov rbp,rsp
            rbp = rsp;
            // 10082 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10083 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 10084 mov rbx,rsi
            rbx = rsi;
            // 10087 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 1008e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 10091 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 10095 mov esi,0x7f
            rsi = (0x7fu64) as u32 as u64;
            // 1009a mov ecx,0x1000000
            rcx = (0x1000000u64) as u32 as u64;
            // 1009f mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // 100a5 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 100a8 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 100ac setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            // 100af add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 100b3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 100b4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 100b5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x41ed2 size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00041ed2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x41ed2;
    loop {
        match pc {
        0x41ed2 => {
            // block 41ed2
            // 41ed2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 41ed3 mov rbp,rsp
            rbp = rsp;
            // 41ed6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 41ed7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 41ed8 mov rbx,rsi
            rbx = rsi;
            // 41edb mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 41ee2 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 41ee5 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 41ee7 lea rdx,[rip+0xbe4d2] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 41eee mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 41ef1 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 41ef5 mov ecx,0x40000
            rcx = (0x40000u64) as u32 as u64;
            // 41efa mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 41f00 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41f03 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 41f05 je 0x41f0c
            if fl.e() { pc = 0x41f0c; continue; }
            pc = 0x41f07;
        }
        0x41f07 => {
            // block 41f07
            // 41f07 mov ecx,DWORD PTR [rbp-0xc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // 41f0a mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            pc = 0x41f0c;
        }
        0x41f0c => {
            // block 41f0c
            // 41f0c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 41f10 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 41f11 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 41f12 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x41fda size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00041fda<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x41fda;
    loop {
        match pc {
        0x41fda => {
            // block 41fda
            // 41fda push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 41fdb mov rbp,rsp
            rbp = rsp;
            // 41fde push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 41fdf push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 41fe0 mov rbx,rsi
            rbx = rsi;
            // 41fe3 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 41fea mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 41fed mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 41fef lea rdx,[rip+0xbe3ca] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 41ff6 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 41ff9 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 41ffd mov ecx,0x60000000
            rcx = (0x60000000u64) as u32 as u64;
            // 42002 mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 42008 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4200b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4200d je 0x42014
            if fl.e() { pc = 0x42014; continue; }
            pc = 0x4200f;
        }
        0x4200f => {
            // block 4200f
            // 4200f mov ecx,DWORD PTR [rbp-0xc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // 42012 mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            pc = 0x42014;
        }
        0x42014 => {
            // block 42014
            // 42014 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 42018 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42019 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4201a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
