//! Mechanically lifted functions (scripts/lift_run.py), module s6c. Each one is checked against the original
//! in the world harness (tests/world_s6c.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x1e68c size=9545
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001e68c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1e68c;
    loop {
        match pc {
        0x1e68c => {
            // block 1e68c
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1e68c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1e68c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1e68d mov rbp,rsp
            rbp = rsp;
            // 1e690 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1e692 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1e694 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1e696 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1e698 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1e699 sub rsp,0x158
            { let (r, f) = sub::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // 1e6a0 mov r13,rcx
            r13 = rcx;
            // 1e6a3 mov r14,rdx
            r14 = rdx;
            // 1e6a6 mov r15,rsi
            r15 = rsi;
            // 1e6a9 mov r12,rdi
            r12 = rdi;
            // 1e6ac mov rdi,r15
            rdi = r15;
            // 1e6af mov rsi,r12
            rsi = r12;
            // 1e6b2 call QWORD PTR [r13+0x0]
            { let r = { let t = m.r64(r13) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1e6b6 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1e6b8 mov rdi,r14
            rdi = r14;
            // 1e6bb mov rsi,r15
            rsi = r15;
            // 1e6be call QWORD PTR [r13+0x0]
            { let r = { let t = m.r64(r13) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1e6c2 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1e6c4 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // 1e6c6 je 0x1e949
            if crate::cov::brk(0x1e6c6, fl.e()) { pc = 0x1e949; continue; }
            pc = 0x1e6cc;
        }
        0x1e6cc => {
            // block 1e6cc
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1e6cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1e6cc test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 1e6ce je 0x1eb96
            if crate::cov::brk(0x1e6ce, fl.e()) { pc = 0x1eb96; continue; }
            pc = 0x1e6d4;
        }
        0x1e6d4 => {
            // block 1e6d4
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1e6d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1e6d4 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1e6d9 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 1e6e0 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 1e6e4 mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1e6e9 mov QWORD PTR [rbp-0x118],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rcx) as u64);
            // 1e6f0 mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 1e6f7 lea rbx,[r12+0x18]
            rbx = r12.wrapping_add(0x18);
            // 1e6fc mov rax,QWORD PTR [r12+0x28]
            rax = m.r64(r12.wrapping_add(0x28)) as u64;
            // 1e701 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // 1e708 mov rax,QWORD PTR [r12+0x18]
            rax = m.r64(r12.wrapping_add(0x18)) as u64;
            // 1e70d mov rcx,QWORD PTR [r12+0x20]
            rcx = m.r64(r12.wrapping_add(0x20)) as u64;
            // 1e712 mov QWORD PTR [rbp-0x100],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rcx) as u64);
            // 1e719 mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 1e720 mov QWORD PTR [r12+0x28],0x0
            m.w64(r12.wrapping_add(0x28), (0x0u64) as u64);
            // 1e729 mov QWORD PTR [r12+0x20],0x0
            m.w64(r12.wrapping_add(0x20), (0x0u64) as u64);
            // 1e732 mov QWORD PTR [r12+0x18],0x0
            m.w64(r12.wrapping_add(0x18), (0x0u64) as u64);
            // 1e73b mov ax,WORD PTR [r12+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r12.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 1e741 mov WORD PTR [rbp-0xc4],ax
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff3c), ((rax as u16 as u64)) as u16);
            // 1e748 mov eax,DWORD PTR [r12+0x58]
            rax = (m.r32(r12.wrapping_add(0x58)) as u64) as u32 as u64;
            // 1e74d mov DWORD PTR [rbp-0xc8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), ((rax as u32 as u64)) as u32);
            // 1e753 mov rax,QWORD PTR [r12+0x50]
            rax = m.r64(r12.wrapping_add(0x50)) as u64;
            // 1e758 mov QWORD PTR [rbp-0xd0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (rax) as u64);
            // 1e75f mov rax,QWORD PTR [r12+0x48]
            rax = m.r64(r12.wrapping_add(0x48)) as u64;
            // 1e764 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 1e76b mov rax,QWORD PTR [r12+0x40]
            rax = m.r64(r12.wrapping_add(0x40)) as u64;
            // 1e770 mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 1e777 mov rax,QWORD PTR [r12+0x30]
            rax = m.r64(r12.wrapping_add(0x30)) as u64;
            // 1e77c mov rcx,QWORD PTR [r12+0x38]
            rcx = m.r64(r12.wrapping_add(0x38)) as u64;
            // 1e781 mov QWORD PTR [rbp-0xe8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rcx) as u64);
            // 1e788 mov QWORD PTR [rbp-0xf0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (rax) as u64);
            // 1e78f mov QWORD PTR [rbp-0xc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u64);
            // 1e79a mov rax,QWORD PTR [r12+0x60]
            rax = m.r64(r12.wrapping_add(0x60)) as u64;
            // 1e79f mov QWORD PTR [rbp-0xc0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rax) as u64);
            // 1e7a6 movups xmm0,XMMWORD PTR [r12+0x68]
            x0 = m.r128(r12.wrapping_add(0x68));
            // 1e7ac movups XMMWORD PTR [rbp-0xb8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff48), x0);
            // 1e7b3 mov QWORD PTR [r12+0x70],0x0
            m.w64(r12.wrapping_add(0x70), (0x0u64) as u64);
            // 1e7bc mov QWORD PTR [r12+0x68],0x0
            m.w64(r12.wrapping_add(0x68), (0x0u64) as u64);
            // 1e7c5 mov QWORD PTR [r12+0x60],0x0
            m.w64(r12.wrapping_add(0x60), (0x0u64) as u64);
            // 1e7ce mov QWORD PTR [rbp-0xa8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u64);
            // 1e7d9 movups xmm0,XMMWORD PTR [r12+0x78]
            x0 = m.r128(r12.wrapping_add(0x78));
            // 1e7df movups XMMWORD PTR [rbp-0xa8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff58), x0);
            // 1e7e6 mov rax,QWORD PTR [r12+0x88]
            rax = m.r64(r12.wrapping_add(0x88)) as u64;
            // 1e7ee mov QWORD PTR [rbp-0x98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rax) as u64);
            // 1e7f5 mov QWORD PTR [r12+0x88],0x0
            m.w64(r12.wrapping_add(0x88), (0x0u64) as u64);
            // 1e801 mov QWORD PTR [r12+0x80],0x0
            m.w64(r12.wrapping_add(0x80), (0x0u64) as u64);
            // 1e80d mov QWORD PTR [r12+0x78],0x0
            m.w64(r12.wrapping_add(0x78), (0x0u64) as u64);
            // 1e816 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 1e821 movups xmm0,XMMWORD PTR [r12+0x90]
            x0 = m.r128(r12.wrapping_add(0x90));
            // 1e82a movups XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // 1e831 mov rax,QWORD PTR [r12+0xa0]
            rax = m.r64(r12.wrapping_add(0xa0)) as u64;
            // 1e839 mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 1e83d mov QWORD PTR [r12+0xa0],0x0
            m.w64(r12.wrapping_add(0xa0), (0x0u64) as u64);
            // 1e849 mov QWORD PTR [r12+0x98],0x0
            m.w64(r12.wrapping_add(0x98), (0x0u64) as u64);
            // 1e855 mov QWORD PTR [r12+0x90],0x0
            m.w64(r12.wrapping_add(0x90), (0x0u64) as u64);
            // 1e861 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 1e869 movups xmm0,XMMWORD PTR [r12+0xa8]
            x0 = m.r128(r12.wrapping_add(0xa8));
            // 1e872 movups XMMWORD PTR [rbp-0x78],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff88), x0);
            // 1e876 mov rax,QWORD PTR [r12+0xb8]
            rax = m.r64(r12.wrapping_add(0xb8)) as u64;
            // 1e87e mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 1e882 mov QWORD PTR [r12+0xb8],0x0
            m.w64(r12.wrapping_add(0xb8), (0x0u64) as u64);
            // 1e88e mov QWORD PTR [r12+0xb0],0x0
            m.w64(r12.wrapping_add(0xb0), (0x0u64) as u64);
            // 1e89a mov QWORD PTR [r12+0xa8],0x0
            m.w64(r12.wrapping_add(0xa8), (0x0u64) as u64);
            // 1e8a6 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 1e8ae mov rax,QWORD PTR [r12+0xc0]
            rax = m.r64(r12.wrapping_add(0xc0)) as u64;
            // 1e8b6 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 1e8ba movups xmm0,XMMWORD PTR [r12+0xc8]
            x0 = m.r128(r12.wrapping_add(0xc8));
            // 1e8c3 movups XMMWORD PTR [rbp-0x58],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), x0);
            // 1e8c7 mov QWORD PTR [r12+0xd0],0x0
            m.w64(r12.wrapping_add(0xd0), (0x0u64) as u64);
            // 1e8d3 mov QWORD PTR [r12+0xc8],0x0
            m.w64(r12.wrapping_add(0xc8), (0x0u64) as u64);
            // 1e8df mov QWORD PTR [r12+0xc0],0x0
            m.w64(r12.wrapping_add(0xc0), (0x0u64) as u64);
            // 1e8eb mov eax,DWORD PTR [r12+0xf0]
            rax = (m.r32(r12.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 1e8f3 mov DWORD PTR [rbp-0x30],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rax as u32 as u64)) as u32);
            // 1e8f6 mov rax,QWORD PTR [r12+0xe8]
            rax = m.r64(r12.wrapping_add(0xe8)) as u64;
            // 1e8fe mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1e902 mov rax,QWORD PTR [r12+0xd8]
            rax = m.r64(r12.wrapping_add(0xd8)) as u64;
            // 1e90a mov rcx,QWORD PTR [r12+0xe0]
            rcx = m.r64(r12.wrapping_add(0xe0)) as u64;
            // 1e912 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 1e916 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 1e91a mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // 1e91e mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1e923 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1e926 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1e92a mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 1e92f mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 1e933 test BYTE PTR [r12+0x18],0x1
            fl = logic::<8>(m.r8(r12.wrapping_add(0x18)) as u64 & 0x1u64);
            // 1e939 jne 0x1ee0b
            if crate::cov::brk(0x1e939, fl.ne()) { pc = 0x1ee0b; continue; }
            pc = 0x1e93f;
        }
        0x1e93f => {
            // block 1e93f
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1e93f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1e93f mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // 1e944 jmp 0x1ee1c
            pc = 0x1ee1c; continue;
        }
        0x1e949 => {
            // block 1e949
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1e949 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1e949 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 1e94b test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 1e94d je 0x20b73
            if crate::cov::brk(0x1e94d, fl.e()) { pc = 0x20b73; continue; }
            pc = 0x1e953;
        }
        0x1e953 => {
            // block 1e953
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1e953 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1e953 mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 1e957 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 1e95e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 1e961 mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1e965 mov QWORD PTR [rbp-0x118],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rcx) as u64);
            // 1e96c mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 1e973 lea rbx,[r15+0x18]
            rbx = r15.wrapping_add(0x18);
            // 1e977 mov rax,QWORD PTR [r15+0x28]
            rax = m.r64(r15.wrapping_add(0x28)) as u64;
            // 1e97b mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // 1e982 mov rax,QWORD PTR [r15+0x18]
            rax = m.r64(r15.wrapping_add(0x18)) as u64;
            // 1e986 mov rcx,QWORD PTR [r15+0x20]
            rcx = m.r64(r15.wrapping_add(0x20)) as u64;
            // 1e98a mov QWORD PTR [rbp-0x100],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rcx) as u64);
            // 1e991 mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 1e998 mov QWORD PTR [r15+0x28],0x0
            m.w64(r15.wrapping_add(0x28), (0x0u64) as u64);
            // 1e9a0 mov QWORD PTR [r15+0x20],0x0
            m.w64(r15.wrapping_add(0x20), (0x0u64) as u64);
            // 1e9a8 mov QWORD PTR [r15+0x18],0x0
            m.w64(r15.wrapping_add(0x18), (0x0u64) as u64);
            // 1e9b0 mov ax,WORD PTR [r15+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r15.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 1e9b5 mov WORD PTR [rbp-0xc4],ax
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff3c), ((rax as u16 as u64)) as u16);
            // 1e9bc mov eax,DWORD PTR [r15+0x58]
            rax = (m.r32(r15.wrapping_add(0x58)) as u64) as u32 as u64;
            // 1e9c0 mov DWORD PTR [rbp-0xc8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), ((rax as u32 as u64)) as u32);
            // 1e9c6 mov rax,QWORD PTR [r15+0x50]
            rax = m.r64(r15.wrapping_add(0x50)) as u64;
            // 1e9ca mov QWORD PTR [rbp-0xd0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (rax) as u64);
            // 1e9d1 mov rax,QWORD PTR [r15+0x48]
            rax = m.r64(r15.wrapping_add(0x48)) as u64;
            // 1e9d5 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 1e9dc mov rax,QWORD PTR [r15+0x40]
            rax = m.r64(r15.wrapping_add(0x40)) as u64;
            // 1e9e0 mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 1e9e7 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 1e9eb mov rcx,QWORD PTR [r15+0x38]
            rcx = m.r64(r15.wrapping_add(0x38)) as u64;
            // 1e9ef mov QWORD PTR [rbp-0xe8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rcx) as u64);
            // 1e9f6 mov QWORD PTR [rbp-0xf0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (rax) as u64);
            // 1e9fd mov QWORD PTR [rbp-0xc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u64);
            // 1ea08 mov rax,QWORD PTR [r15+0x60]
            rax = m.r64(r15.wrapping_add(0x60)) as u64;
            // 1ea0c mov QWORD PTR [rbp-0xc0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rax) as u64);
            // 1ea13 movups xmm0,XMMWORD PTR [r15+0x68]
            x0 = m.r128(r15.wrapping_add(0x68));
            // 1ea18 movups XMMWORD PTR [rbp-0xb8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff48), x0);
            // 1ea1f mov QWORD PTR [r15+0x70],0x0
            m.w64(r15.wrapping_add(0x70), (0x0u64) as u64);
            // 1ea27 mov QWORD PTR [r15+0x68],0x0
            m.w64(r15.wrapping_add(0x68), (0x0u64) as u64);
            // 1ea2f mov QWORD PTR [r15+0x60],0x0
            m.w64(r15.wrapping_add(0x60), (0x0u64) as u64);
            // 1ea37 mov QWORD PTR [rbp-0xa8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u64);
            // 1ea42 movups xmm0,XMMWORD PTR [r15+0x78]
            x0 = m.r128(r15.wrapping_add(0x78));
            // 1ea47 movups XMMWORD PTR [rbp-0xa8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff58), x0);
            // 1ea4e mov rax,QWORD PTR [r15+0x88]
            rax = m.r64(r15.wrapping_add(0x88)) as u64;
            // 1ea55 mov QWORD PTR [rbp-0x98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rax) as u64);
            // 1ea5c mov QWORD PTR [r15+0x88],0x0
            m.w64(r15.wrapping_add(0x88), (0x0u64) as u64);
            // 1ea67 mov QWORD PTR [r15+0x80],0x0
            m.w64(r15.wrapping_add(0x80), (0x0u64) as u64);
            // 1ea72 mov QWORD PTR [r15+0x78],0x0
            m.w64(r15.wrapping_add(0x78), (0x0u64) as u64);
            // 1ea7a mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 1ea85 movups xmm0,XMMWORD PTR [r15+0x90]
            x0 = m.r128(r15.wrapping_add(0x90));
            // 1ea8d movups XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // 1ea94 mov rax,QWORD PTR [r15+0xa0]
            rax = m.r64(r15.wrapping_add(0xa0)) as u64;
            // 1ea9b mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 1ea9f mov QWORD PTR [r15+0xa0],0x0
            m.w64(r15.wrapping_add(0xa0), (0x0u64) as u64);
            // 1eaaa mov QWORD PTR [r15+0x98],0x0
            m.w64(r15.wrapping_add(0x98), (0x0u64) as u64);
            // 1eab5 mov QWORD PTR [r15+0x90],0x0
            m.w64(r15.wrapping_add(0x90), (0x0u64) as u64);
            // 1eac0 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 1eac8 movups xmm0,XMMWORD PTR [r15+0xa8]
            x0 = m.r128(r15.wrapping_add(0xa8));
            // 1ead0 movups XMMWORD PTR [rbp-0x78],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff88), x0);
            // 1ead4 mov rax,QWORD PTR [r15+0xb8]
            rax = m.r64(r15.wrapping_add(0xb8)) as u64;
            // 1eadb mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 1eadf mov QWORD PTR [r15+0xb8],0x0
            m.w64(r15.wrapping_add(0xb8), (0x0u64) as u64);
            // 1eaea mov QWORD PTR [r15+0xb0],0x0
            m.w64(r15.wrapping_add(0xb0), (0x0u64) as u64);
            // 1eaf5 mov QWORD PTR [r15+0xa8],0x0
            m.w64(r15.wrapping_add(0xa8), (0x0u64) as u64);
            // 1eb00 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 1eb08 mov rax,QWORD PTR [r15+0xc0]
            rax = m.r64(r15.wrapping_add(0xc0)) as u64;
            // 1eb0f mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 1eb13 movups xmm0,XMMWORD PTR [r15+0xc8]
            x0 = m.r128(r15.wrapping_add(0xc8));
            // 1eb1b movups XMMWORD PTR [rbp-0x58],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), x0);
            // 1eb1f mov QWORD PTR [r15+0xd0],0x0
            m.w64(r15.wrapping_add(0xd0), (0x0u64) as u64);
            // 1eb2a mov QWORD PTR [r15+0xc8],0x0
            m.w64(r15.wrapping_add(0xc8), (0x0u64) as u64);
            // 1eb35 mov QWORD PTR [r15+0xc0],0x0
            m.w64(r15.wrapping_add(0xc0), (0x0u64) as u64);
            // 1eb40 mov eax,DWORD PTR [r15+0xf0]
            rax = (m.r32(r15.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 1eb47 mov DWORD PTR [rbp-0x30],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rax as u32 as u64)) as u32);
            // 1eb4a mov rax,QWORD PTR [r15+0xe8]
            rax = m.r64(r15.wrapping_add(0xe8)) as u64;
            // 1eb51 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1eb55 mov rax,QWORD PTR [r15+0xd8]
            rax = m.r64(r15.wrapping_add(0xd8)) as u64;
            // 1eb5c mov rcx,QWORD PTR [r15+0xe0]
            rcx = m.r64(r15.wrapping_add(0xe0)) as u64;
            // 1eb63 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 1eb67 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 1eb6b mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // 1eb6f mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 1eb73 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1eb76 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1eb7a mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 1eb7e mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 1eb81 test BYTE PTR [r15+0x18],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x18)) as u64 & 0x1u64);
            // 1eb86 jne 0x1f097
            if crate::cov::brk(0x1eb86, fl.ne()) { pc = 0x1f097; continue; }
            pc = 0x1eb8c;
        }
        0x1eb8c => {
            // block 1eb8c
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1eb8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1eb8c mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // 1eb91 jmp 0x1f0a6
            pc = 0x1f0a6; continue;
        }
        0x1eb96 => {
            // block 1eb96
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1eb96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1eb96 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1eb9b mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 1eba2 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 1eba6 mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1ebab mov QWORD PTR [rbp-0x118],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rcx) as u64);
            // 1ebb2 mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 1ebb9 lea rdx,[r12+0x18]
            rdx = r12.wrapping_add(0x18);
            // 1ebbe mov rax,QWORD PTR [r12+0x28]
            rax = m.r64(r12.wrapping_add(0x28)) as u64;
            // 1ebc3 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // 1ebca mov rax,QWORD PTR [r12+0x18]
            rax = m.r64(r12.wrapping_add(0x18)) as u64;
            // 1ebcf mov rcx,QWORD PTR [r12+0x20]
            rcx = m.r64(r12.wrapping_add(0x20)) as u64;
            // 1ebd4 mov QWORD PTR [rbp-0x100],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rcx) as u64);
            // 1ebdb mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 1ebe2 mov QWORD PTR [r12+0x28],0x0
            m.w64(r12.wrapping_add(0x28), (0x0u64) as u64);
            // 1ebeb mov QWORD PTR [r12+0x20],0x0
            m.w64(r12.wrapping_add(0x20), (0x0u64) as u64);
            // 1ebf4 mov QWORD PTR [r12+0x18],0x0
            m.w64(r12.wrapping_add(0x18), (0x0u64) as u64);
            // 1ebfd mov ax,WORD PTR [r12+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r12.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 1ec03 mov WORD PTR [rbp-0xc4],ax
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff3c), ((rax as u16 as u64)) as u16);
            // 1ec0a mov eax,DWORD PTR [r12+0x58]
            rax = (m.r32(r12.wrapping_add(0x58)) as u64) as u32 as u64;
            // 1ec0f mov DWORD PTR [rbp-0xc8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), ((rax as u32 as u64)) as u32);
            // 1ec15 mov rax,QWORD PTR [r12+0x50]
            rax = m.r64(r12.wrapping_add(0x50)) as u64;
            // 1ec1a mov QWORD PTR [rbp-0xd0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (rax) as u64);
            // 1ec21 mov rax,QWORD PTR [r12+0x48]
            rax = m.r64(r12.wrapping_add(0x48)) as u64;
            // 1ec26 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 1ec2d mov rax,QWORD PTR [r12+0x40]
            rax = m.r64(r12.wrapping_add(0x40)) as u64;
            // 1ec32 mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 1ec39 mov rax,QWORD PTR [r12+0x30]
            rax = m.r64(r12.wrapping_add(0x30)) as u64;
            // 1ec3e mov rcx,QWORD PTR [r12+0x38]
            rcx = m.r64(r12.wrapping_add(0x38)) as u64;
            // 1ec43 mov QWORD PTR [rbp-0xe8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rcx) as u64);
            // 1ec4a mov QWORD PTR [rbp-0xf0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (rax) as u64);
            // 1ec51 mov QWORD PTR [rbp-0xc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u64);
            // 1ec5c mov rax,QWORD PTR [r12+0x60]
            rax = m.r64(r12.wrapping_add(0x60)) as u64;
            // 1ec61 mov QWORD PTR [rbp-0xc0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rax) as u64);
            // 1ec68 movups xmm0,XMMWORD PTR [r12+0x68]
            x0 = m.r128(r12.wrapping_add(0x68));
            // 1ec6e movups XMMWORD PTR [rbp-0xb8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff48), x0);
            // 1ec75 mov QWORD PTR [r12+0x70],0x0
            m.w64(r12.wrapping_add(0x70), (0x0u64) as u64);
            // 1ec7e mov QWORD PTR [r12+0x68],0x0
            m.w64(r12.wrapping_add(0x68), (0x0u64) as u64);
            // 1ec87 mov QWORD PTR [r12+0x60],0x0
            m.w64(r12.wrapping_add(0x60), (0x0u64) as u64);
            // 1ec90 mov QWORD PTR [rbp-0xa8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u64);
            // 1ec9b movups xmm0,XMMWORD PTR [r12+0x78]
            x0 = m.r128(r12.wrapping_add(0x78));
            // 1eca1 movups XMMWORD PTR [rbp-0xa8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff58), x0);
            // 1eca8 mov rax,QWORD PTR [r12+0x88]
            rax = m.r64(r12.wrapping_add(0x88)) as u64;
            // 1ecb0 mov QWORD PTR [rbp-0x98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rax) as u64);
            // 1ecb7 mov QWORD PTR [r12+0x88],0x0
            m.w64(r12.wrapping_add(0x88), (0x0u64) as u64);
            // 1ecc3 mov QWORD PTR [r12+0x80],0x0
            m.w64(r12.wrapping_add(0x80), (0x0u64) as u64);
            // 1eccf mov QWORD PTR [r12+0x78],0x0
            m.w64(r12.wrapping_add(0x78), (0x0u64) as u64);
            // 1ecd8 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 1ece3 movups xmm0,XMMWORD PTR [r12+0x90]
            x0 = m.r128(r12.wrapping_add(0x90));
            // 1ecec movups XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // 1ecf3 mov rax,QWORD PTR [r12+0xa0]
            rax = m.r64(r12.wrapping_add(0xa0)) as u64;
            // 1ecfb mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 1ecff mov QWORD PTR [r12+0xa0],0x0
            m.w64(r12.wrapping_add(0xa0), (0x0u64) as u64);
            // 1ed0b mov QWORD PTR [r12+0x98],0x0
            m.w64(r12.wrapping_add(0x98), (0x0u64) as u64);
            // 1ed17 mov QWORD PTR [r12+0x90],0x0
            m.w64(r12.wrapping_add(0x90), (0x0u64) as u64);
            // 1ed23 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 1ed2b movups xmm0,XMMWORD PTR [r12+0xa8]
            x0 = m.r128(r12.wrapping_add(0xa8));
            // 1ed34 movups XMMWORD PTR [rbp-0x78],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff88), x0);
            // 1ed38 mov rax,QWORD PTR [r12+0xb8]
            rax = m.r64(r12.wrapping_add(0xb8)) as u64;
            // 1ed40 mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 1ed44 mov QWORD PTR [r12+0xb8],0x0
            m.w64(r12.wrapping_add(0xb8), (0x0u64) as u64);
            // 1ed50 mov QWORD PTR [r12+0xb0],0x0
            m.w64(r12.wrapping_add(0xb0), (0x0u64) as u64);
            // 1ed5c mov QWORD PTR [r12+0xa8],0x0
            m.w64(r12.wrapping_add(0xa8), (0x0u64) as u64);
            // 1ed68 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 1ed70 mov rax,QWORD PTR [r12+0xc0]
            rax = m.r64(r12.wrapping_add(0xc0)) as u64;
            // 1ed78 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 1ed7c movups xmm0,XMMWORD PTR [r12+0xc8]
            x0 = m.r128(r12.wrapping_add(0xc8));
            // 1ed85 movups XMMWORD PTR [rbp-0x58],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), x0);
            // 1ed89 mov QWORD PTR [r12+0xd0],0x0
            m.w64(r12.wrapping_add(0xd0), (0x0u64) as u64);
            // 1ed95 mov QWORD PTR [r12+0xc8],0x0
            m.w64(r12.wrapping_add(0xc8), (0x0u64) as u64);
            // 1eda1 mov QWORD PTR [r12+0xc0],0x0
            m.w64(r12.wrapping_add(0xc0), (0x0u64) as u64);
            // 1edad mov eax,DWORD PTR [r12+0xf0]
            rax = (m.r32(r12.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 1edb5 mov DWORD PTR [rbp-0x30],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rax as u32 as u64)) as u32);
            // 1edb8 mov rax,QWORD PTR [r12+0xe8]
            rax = m.r64(r12.wrapping_add(0xe8)) as u64;
            // 1edc0 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1edc4 mov rax,QWORD PTR [r12+0xd8]
            rax = m.r64(r12.wrapping_add(0xd8)) as u64;
            // 1edcc mov rcx,QWORD PTR [r12+0xe0]
            rcx = m.r64(r12.wrapping_add(0xe0)) as u64;
            // 1edd4 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 1edd8 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 1eddc mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 1ede0 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1ede5 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 1ede8 mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1edec mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 1edf1 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 1edf5 test BYTE PTR [r12+0x18],0x1
            fl = logic::<8>(m.r8(r12.wrapping_add(0x18)) as u64 & 0x1u64);
            // 1edfb jne 0x1f334
            if crate::cov::brk(0x1edfb, fl.ne()) { pc = 0x1f334; continue; }
            pc = 0x1ee01;
        }
        0x1ee01 => {
            // block 1ee01
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1ee01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ee01 mov WORD PTR [rdx],0x0
            m.w16(rdx, (0x0u64) as u16);
            // 1ee06 jmp 0x1f345
            pc = 0x1f345; continue;
        }
        0x1ee0b => {
            // block 1ee0b
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1ee0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ee0b mov rax,QWORD PTR [r12+0x28]
            rax = m.r64(r12.wrapping_add(0x28)) as u64;
            // 1ee10 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 1ee13 mov QWORD PTR [r12+0x20],0x0
            m.w64(r12.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x1ee1c;
        }
        0x1ee1c => {
            // block 1ee1c
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1ee1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ee1c xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 1ee1e mov rdi,rbx
            rdi = rbx;
            // 1ee21 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1ee26 lea rax,[r12+0x30]
            rax = r12.wrapping_add(0x30);
            // 1ee2b lea rdi,[r12+0x60]
            rdi = r12.wrapping_add(0x60);
            // 1ee30 lea rcx,[r12+0x78]
            rcx = r12.wrapping_add(0x78);
            // 1ee35 mov QWORD PTR [rbp-0x140],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rcx) as u64);
            // 1ee3c lea rcx,[r12+0x90]
            rcx = r12.wrapping_add(0x90);
            // 1ee44 mov QWORD PTR [rbp-0x138],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rcx) as u64);
            // 1ee4b lea rcx,[r12+0xa8]
            rcx = r12.wrapping_add(0xa8);
            // 1ee53 mov QWORD PTR [rbp-0x130],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (rcx) as u64);
            // 1ee5a lea rcx,[r12+0xc0]
            rcx = r12.wrapping_add(0xc0);
            // 1ee62 mov QWORD PTR [rbp-0x128],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rcx) as u64);
            // 1ee69 lea r13,[r12+0xd8]
            r13 = r12.wrapping_add(0xd8);
            // 1ee71 lea r15,[r14+0x18]
            r15 = r14.wrapping_add(0x18);
            // 1ee75 mov rcx,QWORD PTR [r15+0x10]
            rcx = m.r64(r15.wrapping_add(0x10)) as u64;
            // 1ee79 mov QWORD PTR [rbx+0x10],rcx
            m.w64(rbx.wrapping_add(0x10), (rcx) as u64);
            // 1ee7d mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 1ee80 mov rdx,QWORD PTR [r15+0x8]
            rdx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1ee84 mov QWORD PTR [rbx+0x8],rdx
            m.w64(rbx.wrapping_add(0x8), (rdx) as u64);
            // 1ee88 mov QWORD PTR [rbx],rcx
            m.w64(rbx, (rcx) as u64);
            // 1ee8b mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // 1ee93 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // 1ee9b mov QWORD PTR [r15],0x0
            m.w64(r15, (0x0u64) as u64);
            // 1eea2 mov cx,WORD PTR [r14+0x5c]
            rcx = (rcx & !0xffff) | ((m.r16(r14.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 1eea7 mov WORD PTR [rax+0x2c],cx
            m.w16(rax.wrapping_add(0x2c), ((rcx as u16 as u64)) as u16);
            // 1eeab mov ecx,DWORD PTR [r14+0x58]
            rcx = (m.r32(r14.wrapping_add(0x58)) as u64) as u32 as u64;
            // 1eeaf mov DWORD PTR [rax+0x28],ecx
            m.w32(rax.wrapping_add(0x28), ((rcx as u32 as u64)) as u32);
            // 1eeb2 mov rcx,QWORD PTR [r14+0x50]
            rcx = m.r64(r14.wrapping_add(0x50)) as u64;
            // 1eeb6 mov QWORD PTR [rax+0x20],rcx
            m.w64(rax.wrapping_add(0x20), (rcx) as u64);
            // 1eeba mov rcx,QWORD PTR [r14+0x48]
            rcx = m.r64(r14.wrapping_add(0x48)) as u64;
            // 1eebe mov QWORD PTR [rax+0x18],rcx
            m.w64(rax.wrapping_add(0x18), (rcx) as u64);
            // 1eec2 mov rcx,QWORD PTR [r14+0x40]
            rcx = m.r64(r14.wrapping_add(0x40)) as u64;
            // 1eec6 mov QWORD PTR [rax+0x10],rcx
            m.w64(rax.wrapping_add(0x10), (rcx) as u64);
            // 1eeca mov rcx,QWORD PTR [r14+0x30]
            rcx = m.r64(r14.wrapping_add(0x30)) as u64;
            // 1eece mov rdx,QWORD PTR [r14+0x38]
            rdx = m.r64(r14.wrapping_add(0x38)) as u64;
            // 1eed2 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 1eed6 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 1eed9 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1eede movups xmm0,XMMWORD PTR [r14+0x60]
            x0 = m.r128(r14.wrapping_add(0x60));
            // 1eee3 movups XMMWORD PTR [r12+0x60],xmm0
            m.w128(r12.wrapping_add(0x60), x0);
            // 1eee9 mov rax,QWORD PTR [r14+0x70]
            rax = m.r64(r14.wrapping_add(0x70)) as u64;
            // 1eeed mov QWORD PTR [r12+0x70],rax
            m.w64(r12.wrapping_add(0x70), (rax) as u64);
            // 1eef2 mov QWORD PTR [r14+0x70],0x0
            m.w64(r14.wrapping_add(0x70), (0x0u64) as u64);
            // 1eefa mov QWORD PTR [r14+0x68],0x0
            m.w64(r14.wrapping_add(0x68), (0x0u64) as u64);
            // 1ef02 mov QWORD PTR [r14+0x60],0x0
            m.w64(r14.wrapping_add(0x60), (0x0u64) as u64);
            // 1ef0a mov rdi,QWORD PTR [rbp-0x140]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 1ef11 call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1ef16 movups xmm0,XMMWORD PTR [r14+0x78]
            x0 = m.r128(r14.wrapping_add(0x78));
            // 1ef1b movups XMMWORD PTR [r12+0x78],xmm0
            m.w128(r12.wrapping_add(0x78), x0);
            // 1ef21 mov rax,QWORD PTR [r14+0x88]
            rax = m.r64(r14.wrapping_add(0x88)) as u64;
            // 1ef28 mov QWORD PTR [r12+0x88],rax
            m.w64(r12.wrapping_add(0x88), (rax) as u64);
            // 1ef30 mov QWORD PTR [r14+0x88],0x0
            m.w64(r14.wrapping_add(0x88), (0x0u64) as u64);
            // 1ef3b mov QWORD PTR [r14+0x80],0x0
            m.w64(r14.wrapping_add(0x80), (0x0u64) as u64);
            // 1ef46 mov QWORD PTR [r14+0x78],0x0
            m.w64(r14.wrapping_add(0x78), (0x0u64) as u64);
            // 1ef4e mov rdi,QWORD PTR [rbp-0x138]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 1ef55 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1ef5a movups xmm0,XMMWORD PTR [r14+0x90]
            x0 = m.r128(r14.wrapping_add(0x90));
            // 1ef62 movups XMMWORD PTR [r12+0x90],xmm0
            m.w128(r12.wrapping_add(0x90), x0);
            // 1ef6b mov rax,QWORD PTR [r14+0xa0]
            rax = m.r64(r14.wrapping_add(0xa0)) as u64;
            // 1ef72 mov QWORD PTR [r12+0xa0],rax
            m.w64(r12.wrapping_add(0xa0), (rax) as u64);
            // 1ef7a mov QWORD PTR [r14+0xa0],0x0
            m.w64(r14.wrapping_add(0xa0), (0x0u64) as u64);
            // 1ef85 mov QWORD PTR [r14+0x98],0x0
            m.w64(r14.wrapping_add(0x98), (0x0u64) as u64);
            // 1ef90 mov QWORD PTR [r14+0x90],0x0
            m.w64(r14.wrapping_add(0x90), (0x0u64) as u64);
            // 1ef9b mov rdi,QWORD PTR [rbp-0x130]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 1efa2 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1efa7 movups xmm0,XMMWORD PTR [r14+0xa8]
            x0 = m.r128(r14.wrapping_add(0xa8));
            // 1efaf movups XMMWORD PTR [r12+0xa8],xmm0
            m.w128(r12.wrapping_add(0xa8), x0);
            // 1efb8 mov rax,QWORD PTR [r14+0xb8]
            rax = m.r64(r14.wrapping_add(0xb8)) as u64;
            // 1efbf mov QWORD PTR [r12+0xb8],rax
            m.w64(r12.wrapping_add(0xb8), (rax) as u64);
            // 1efc7 mov QWORD PTR [r14+0xb8],0x0
            m.w64(r14.wrapping_add(0xb8), (0x0u64) as u64);
            // 1efd2 mov QWORD PTR [r14+0xb0],0x0
            m.w64(r14.wrapping_add(0xb0), (0x0u64) as u64);
            // 1efdd mov QWORD PTR [r14+0xa8],0x0
            m.w64(r14.wrapping_add(0xa8), (0x0u64) as u64);
            // 1efe8 mov rdi,QWORD PTR [rbp-0x128]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 1efef call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1eff4 movups xmm0,XMMWORD PTR [r14+0xc0]
            x0 = m.r128(r14.wrapping_add(0xc0));
            // 1effc movups XMMWORD PTR [r12+0xc0],xmm0
            m.w128(r12.wrapping_add(0xc0), x0);
            // 1f005 mov rax,QWORD PTR [r14+0xd0]
            rax = m.r64(r14.wrapping_add(0xd0)) as u64;
            // 1f00c mov QWORD PTR [r12+0xd0],rax
            m.w64(r12.wrapping_add(0xd0), (rax) as u64);
            // 1f014 mov QWORD PTR [r14+0xd0],0x0
            m.w64(r14.wrapping_add(0xd0), (0x0u64) as u64);
            // 1f01f mov QWORD PTR [r14+0xc8],0x0
            m.w64(r14.wrapping_add(0xc8), (0x0u64) as u64);
            // 1f02a mov QWORD PTR [r14+0xc0],0x0
            m.w64(r14.wrapping_add(0xc0), (0x0u64) as u64);
            // 1f035 mov eax,DWORD PTR [r14+0xf0]
            rax = (m.r32(r14.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 1f03c mov DWORD PTR [r13+0x18],eax
            m.w32(r13.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 1f040 mov rax,QWORD PTR [r14+0xe8]
            rax = m.r64(r14.wrapping_add(0xe8)) as u64;
            // 1f047 mov QWORD PTR [r13+0x10],rax
            m.w64(r13.wrapping_add(0x10), (rax) as u64);
            // 1f04b mov rax,QWORD PTR [r14+0xd8]
            rax = m.r64(r14.wrapping_add(0xd8)) as u64;
            // 1f052 mov rcx,QWORD PTR [r14+0xe0]
            rcx = m.r64(r14.wrapping_add(0xe0)) as u64;
            // 1f059 mov QWORD PTR [r13+0x8],rcx
            m.w64(r13.wrapping_add(0x8), (rcx) as u64);
            // 1f05d mov QWORD PTR [r13+0x0],rax
            m.w64(r13, (rax) as u64);
            // 1f061 mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 1f068 mov QWORD PTR [r14+0x10],rax
            m.w64(r14.wrapping_add(0x10), (rax) as u64);
            // 1f06c mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 1f073 mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 1f07a mov QWORD PTR [r14+0x8],rcx
            m.w64(r14.wrapping_add(0x8), (rcx) as u64);
            // 1f07e mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 1f081 test BYTE PTR [r14+0x18],0x1
            fl = logic::<8>(m.r8(r14.wrapping_add(0x18)) as u64 & 0x1u64);
            // 1f086 jne 0x1f5d6
            if crate::cov::brk(0x1f086, fl.ne()) { pc = 0x1f5d6; continue; }
            pc = 0x1f08c;
        }
        0x1f08c => {
            // block 1f08c
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1f08c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f08c mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // 1f092 jmp 0x1f5e5
            pc = 0x1f5e5; continue;
        }
        0x1f097 => {
            // block 1f097
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1f097 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f097 mov rax,QWORD PTR [r15+0x28]
            rax = m.r64(r15.wrapping_add(0x28)) as u64;
            // 1f09b mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 1f09e mov QWORD PTR [r15+0x20],0x0
            m.w64(r15.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x1f0a6;
        }
        0x1f0a6 => {
            // block 1f0a6
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1f0a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f0a6 mov QWORD PTR [rbp-0x128],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (r13) as u64);
            // 1f0ad xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 1f0af mov rdi,rbx
            rdi = rbx;
            // 1f0b2 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f0b7 lea rdx,[r15+0x30]
            rdx = r15.wrapping_add(0x30);
            // 1f0bb mov QWORD PTR [rbp-0x180],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rdx) as u64);
            // 1f0c2 lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 1f0c6 mov QWORD PTR [rbp-0x170],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rdi) as u64);
            // 1f0cd lea rax,[r15+0x78]
            rax = r15.wrapping_add(0x78);
            // 1f0d1 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 1f0d8 lea rax,[r15+0x90]
            rax = r15.wrapping_add(0x90);
            // 1f0df mov QWORD PTR [rbp-0x140],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rax) as u64);
            // 1f0e6 lea rax,[r15+0xa8]
            rax = r15.wrapping_add(0xa8);
            // 1f0ed mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // 1f0f4 lea rax,[r15+0xc0]
            rax = r15.wrapping_add(0xc0);
            // 1f0fb mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 1f102 lea rax,[r15+0xd8]
            rax = r15.wrapping_add(0xd8);
            // 1f109 mov QWORD PTR [rbp-0x130],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (rax) as u64);
            // 1f110 lea r13,[r14+0x18]
            r13 = r14.wrapping_add(0x18);
            // 1f114 mov rax,QWORD PTR [r13+0x10]
            rax = m.r64(r13.wrapping_add(0x10)) as u64;
            // 1f118 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 1f11c mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 1f120 mov rcx,QWORD PTR [r13+0x8]
            rcx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 1f124 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 1f128 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 1f12b mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // 1f133 mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // 1f13b mov QWORD PTR [r13+0x0],0x0
            m.w64(r13, (0x0u64) as u64);
            // 1f143 mov ax,WORD PTR [r14+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r14.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 1f148 mov WORD PTR [rdx+0x2c],ax
            m.w16(rdx.wrapping_add(0x2c), ((rax as u16 as u64)) as u16);
            // 1f14c mov eax,DWORD PTR [r14+0x58]
            rax = (m.r32(r14.wrapping_add(0x58)) as u64) as u32 as u64;
            // 1f150 mov DWORD PTR [rdx+0x28],eax
            m.w32(rdx.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 1f153 mov rax,QWORD PTR [r14+0x50]
            rax = m.r64(r14.wrapping_add(0x50)) as u64;
            // 1f157 mov QWORD PTR [rdx+0x20],rax
            m.w64(rdx.wrapping_add(0x20), (rax) as u64);
            // 1f15b mov rax,QWORD PTR [r14+0x48]
            rax = m.r64(r14.wrapping_add(0x48)) as u64;
            // 1f15f mov QWORD PTR [rdx+0x18],rax
            m.w64(rdx.wrapping_add(0x18), (rax) as u64);
            // 1f163 mov rax,QWORD PTR [r14+0x40]
            rax = m.r64(r14.wrapping_add(0x40)) as u64;
            // 1f167 mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 1f16b mov rax,QWORD PTR [r14+0x30]
            rax = m.r64(r14.wrapping_add(0x30)) as u64;
            // 1f16f mov rcx,QWORD PTR [r14+0x38]
            rcx = m.r64(r14.wrapping_add(0x38)) as u64;
            // 1f173 mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 1f177 mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 1f17a call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f17f movups xmm0,XMMWORD PTR [r14+0x60]
            x0 = m.r128(r14.wrapping_add(0x60));
            // 1f184 movups XMMWORD PTR [r15+0x60],xmm0
            m.w128(r15.wrapping_add(0x60), x0);
            // 1f189 mov rax,QWORD PTR [r14+0x70]
            rax = m.r64(r14.wrapping_add(0x70)) as u64;
            // 1f18d mov QWORD PTR [r15+0x70],rax
            m.w64(r15.wrapping_add(0x70), (rax) as u64);
            // 1f191 mov QWORD PTR [r14+0x70],0x0
            m.w64(r14.wrapping_add(0x70), (0x0u64) as u64);
            // 1f199 mov QWORD PTR [r14+0x68],0x0
            m.w64(r14.wrapping_add(0x68), (0x0u64) as u64);
            // 1f1a1 mov QWORD PTR [r14+0x60],0x0
            m.w64(r14.wrapping_add(0x60), (0x0u64) as u64);
            // 1f1a9 mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 1f1b0 call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f1b5 movups xmm0,XMMWORD PTR [r14+0x78]
            x0 = m.r128(r14.wrapping_add(0x78));
            // 1f1ba movups XMMWORD PTR [r15+0x78],xmm0
            m.w128(r15.wrapping_add(0x78), x0);
            // 1f1bf mov rax,QWORD PTR [r14+0x88]
            rax = m.r64(r14.wrapping_add(0x88)) as u64;
            // 1f1c6 mov QWORD PTR [r15+0x88],rax
            m.w64(r15.wrapping_add(0x88), (rax) as u64);
            // 1f1cd mov QWORD PTR [r14+0x88],0x0
            m.w64(r14.wrapping_add(0x88), (0x0u64) as u64);
            // 1f1d8 mov QWORD PTR [r14+0x80],0x0
            m.w64(r14.wrapping_add(0x80), (0x0u64) as u64);
            // 1f1e3 mov QWORD PTR [r14+0x78],0x0
            m.w64(r14.wrapping_add(0x78), (0x0u64) as u64);
            // 1f1eb mov rdi,QWORD PTR [rbp-0x140]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 1f1f2 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f1f7 movups xmm0,XMMWORD PTR [r14+0x90]
            x0 = m.r128(r14.wrapping_add(0x90));
            // 1f1ff movups XMMWORD PTR [r15+0x90],xmm0
            m.w128(r15.wrapping_add(0x90), x0);
            // 1f207 mov rax,QWORD PTR [r14+0xa0]
            rax = m.r64(r14.wrapping_add(0xa0)) as u64;
            // 1f20e mov QWORD PTR [r15+0xa0],rax
            m.w64(r15.wrapping_add(0xa0), (rax) as u64);
            // 1f215 mov QWORD PTR [r14+0xa0],0x0
            m.w64(r14.wrapping_add(0xa0), (0x0u64) as u64);
            // 1f220 mov QWORD PTR [r14+0x98],0x0
            m.w64(r14.wrapping_add(0x98), (0x0u64) as u64);
            // 1f22b mov QWORD PTR [r14+0x90],0x0
            m.w64(r14.wrapping_add(0x90), (0x0u64) as u64);
            // 1f236 mov rdi,QWORD PTR [rbp-0x138]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 1f23d call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f242 movups xmm0,XMMWORD PTR [r14+0xa8]
            x0 = m.r128(r14.wrapping_add(0xa8));
            // 1f24a movups XMMWORD PTR [r15+0xa8],xmm0
            m.w128(r15.wrapping_add(0xa8), x0);
            // 1f252 mov rax,QWORD PTR [r14+0xb8]
            rax = m.r64(r14.wrapping_add(0xb8)) as u64;
            // 1f259 mov QWORD PTR [r15+0xb8],rax
            m.w64(r15.wrapping_add(0xb8), (rax) as u64);
            // 1f260 mov QWORD PTR [r14+0xb8],0x0
            m.w64(r14.wrapping_add(0xb8), (0x0u64) as u64);
            // 1f26b mov QWORD PTR [r14+0xb0],0x0
            m.w64(r14.wrapping_add(0xb0), (0x0u64) as u64);
            // 1f276 mov QWORD PTR [r14+0xa8],0x0
            m.w64(r14.wrapping_add(0xa8), (0x0u64) as u64);
            // 1f281 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 1f288 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f28d movups xmm0,XMMWORD PTR [r14+0xc0]
            x0 = m.r128(r14.wrapping_add(0xc0));
            // 1f295 movups XMMWORD PTR [r15+0xc0],xmm0
            m.w128(r15.wrapping_add(0xc0), x0);
            // 1f29d mov rax,QWORD PTR [r14+0xd0]
            rax = m.r64(r14.wrapping_add(0xd0)) as u64;
            // 1f2a4 mov QWORD PTR [r15+0xd0],rax
            m.w64(r15.wrapping_add(0xd0), (rax) as u64);
            // 1f2ab mov QWORD PTR [r14+0xd0],0x0
            m.w64(r14.wrapping_add(0xd0), (0x0u64) as u64);
            // 1f2b6 mov QWORD PTR [r14+0xc8],0x0
            m.w64(r14.wrapping_add(0xc8), (0x0u64) as u64);
            // 1f2c1 mov QWORD PTR [r14+0xc0],0x0
            m.w64(r14.wrapping_add(0xc0), (0x0u64) as u64);
            // 1f2cc mov eax,DWORD PTR [r14+0xf0]
            rax = (m.r32(r14.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 1f2d3 mov rdx,QWORD PTR [rbp-0x130]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 1f2da mov DWORD PTR [rdx+0x18],eax
            m.w32(rdx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 1f2dd mov rax,QWORD PTR [r14+0xe8]
            rax = m.r64(r14.wrapping_add(0xe8)) as u64;
            // 1f2e4 mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 1f2e8 mov rax,QWORD PTR [r14+0xd8]
            rax = m.r64(r14.wrapping_add(0xd8)) as u64;
            // 1f2ef mov rcx,QWORD PTR [r14+0xe0]
            rcx = m.r64(r14.wrapping_add(0xe0)) as u64;
            // 1f2f6 mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 1f2fa mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 1f2fd mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 1f304 mov QWORD PTR [r14+0x10],rax
            m.w64(r14.wrapping_add(0x10), (rax) as u64);
            // 1f308 mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 1f30f mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 1f316 mov QWORD PTR [r14+0x8],rcx
            m.w64(r14.wrapping_add(0x8), (rcx) as u64);
            // 1f31a mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 1f31d test BYTE PTR [r14+0x18],0x1
            fl = logic::<8>(m.r8(r14.wrapping_add(0x18)) as u64 & 0x1u64);
            // 1f322 jne 0x1f841
            if crate::cov::brk(0x1f322, fl.ne()) { pc = 0x1f841; continue; }
            pc = 0x1f328;
        }
        0x1f328 => {
            // block 1f328
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1f328 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f328 mov WORD PTR [r13+0x0],0x0
            m.w16(r13, (0x0u64) as u16);
            // 1f32f jmp 0x1f850
            pc = 0x1f850; continue;
        }
        0x1f334 => {
            // block 1f334
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1f334 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f334 mov rax,QWORD PTR [r12+0x28]
            rax = m.r64(r12.wrapping_add(0x28)) as u64;
            // 1f339 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 1f33c mov QWORD PTR [r12+0x20],0x0
            m.w64(r12.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x1f345;
        }
        0x1f345 => {
            // block 1f345
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1f345 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f345 mov QWORD PTR [rbp-0x128],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (r13) as u64);
            // 1f34c xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 1f34e mov r13,rdx
            r13 = rdx;
            // 1f351 mov rdi,r13
            rdi = r13;
            // 1f354 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f359 lea rax,[r12+0x30]
            rax = r12.wrapping_add(0x30);
            // 1f35e lea rdi,[r12+0x60]
            rdi = r12.wrapping_add(0x60);
            // 1f363 lea rcx,[r12+0x78]
            rcx = r12.wrapping_add(0x78);
            // 1f368 mov QWORD PTR [rbp-0x150],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rcx) as u64);
            // 1f36f lea rcx,[r12+0x90]
            rcx = r12.wrapping_add(0x90);
            // 1f377 mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // 1f37e lea rcx,[r12+0xa8]
            rcx = r12.wrapping_add(0xa8);
            // 1f386 mov QWORD PTR [rbp-0x140],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rcx) as u64);
            // 1f38d lea rcx,[r12+0xc0]
            rcx = r12.wrapping_add(0xc0);
            // 1f395 mov QWORD PTR [rbp-0x138],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rcx) as u64);
            // 1f39c lea rcx,[r12+0xd8]
            rcx = r12.wrapping_add(0xd8);
            // 1f3a4 mov QWORD PTR [rbp-0x130],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (rcx) as u64);
            // 1f3ab lea rbx,[r15+0x18]
            rbx = r15.wrapping_add(0x18);
            // 1f3af mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 1f3b3 mov QWORD PTR [r13+0x10],rcx
            m.w64(r13.wrapping_add(0x10), (rcx) as u64);
            // 1f3b7 mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 1f3ba mov rdx,QWORD PTR [rbx+0x8]
            rdx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1f3be mov QWORD PTR [r13+0x8],rdx
            m.w64(r13.wrapping_add(0x8), (rdx) as u64);
            // 1f3c2 mov QWORD PTR [r13+0x0],rcx
            m.w64(r13, (rcx) as u64);
            // 1f3c6 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 1f3ce mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 1f3d6 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 1f3dd mov cx,WORD PTR [r15+0x5c]
            rcx = (rcx & !0xffff) | ((m.r16(r15.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 1f3e2 mov WORD PTR [rax+0x2c],cx
            m.w16(rax.wrapping_add(0x2c), ((rcx as u16 as u64)) as u16);
            // 1f3e6 mov ecx,DWORD PTR [r15+0x58]
            rcx = (m.r32(r15.wrapping_add(0x58)) as u64) as u32 as u64;
            // 1f3ea mov DWORD PTR [rax+0x28],ecx
            m.w32(rax.wrapping_add(0x28), ((rcx as u32 as u64)) as u32);
            // 1f3ed mov rcx,QWORD PTR [r15+0x50]
            rcx = m.r64(r15.wrapping_add(0x50)) as u64;
            // 1f3f1 mov QWORD PTR [rax+0x20],rcx
            m.w64(rax.wrapping_add(0x20), (rcx) as u64);
            // 1f3f5 mov rcx,QWORD PTR [r15+0x48]
            rcx = m.r64(r15.wrapping_add(0x48)) as u64;
            // 1f3f9 mov QWORD PTR [rax+0x18],rcx
            m.w64(rax.wrapping_add(0x18), (rcx) as u64);
            // 1f3fd mov rcx,QWORD PTR [r15+0x40]
            rcx = m.r64(r15.wrapping_add(0x40)) as u64;
            // 1f401 mov QWORD PTR [rax+0x10],rcx
            m.w64(rax.wrapping_add(0x10), (rcx) as u64);
            // 1f405 mov rcx,QWORD PTR [r15+0x30]
            rcx = m.r64(r15.wrapping_add(0x30)) as u64;
            // 1f409 mov rdx,QWORD PTR [r15+0x38]
            rdx = m.r64(r15.wrapping_add(0x38)) as u64;
            // 1f40d mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 1f411 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 1f414 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f419 movups xmm0,XMMWORD PTR [r15+0x60]
            x0 = m.r128(r15.wrapping_add(0x60));
            // 1f41e movups XMMWORD PTR [r12+0x60],xmm0
            m.w128(r12.wrapping_add(0x60), x0);
            // 1f424 mov rax,QWORD PTR [r15+0x70]
            rax = m.r64(r15.wrapping_add(0x70)) as u64;
            // 1f428 mov QWORD PTR [r12+0x70],rax
            m.w64(r12.wrapping_add(0x70), (rax) as u64);
            // 1f42d mov QWORD PTR [r15+0x70],0x0
            m.w64(r15.wrapping_add(0x70), (0x0u64) as u64);
            // 1f435 mov QWORD PTR [r15+0x68],0x0
            m.w64(r15.wrapping_add(0x68), (0x0u64) as u64);
            // 1f43d mov QWORD PTR [r15+0x60],0x0
            m.w64(r15.wrapping_add(0x60), (0x0u64) as u64);
            // 1f445 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 1f44c call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f451 movups xmm0,XMMWORD PTR [r15+0x78]
            x0 = m.r128(r15.wrapping_add(0x78));
            // 1f456 movups XMMWORD PTR [r12+0x78],xmm0
            m.w128(r12.wrapping_add(0x78), x0);
            // 1f45c mov rax,QWORD PTR [r15+0x88]
            rax = m.r64(r15.wrapping_add(0x88)) as u64;
            // 1f463 mov QWORD PTR [r12+0x88],rax
            m.w64(r12.wrapping_add(0x88), (rax) as u64);
            // 1f46b mov QWORD PTR [r15+0x88],0x0
            m.w64(r15.wrapping_add(0x88), (0x0u64) as u64);
            // 1f476 mov QWORD PTR [r15+0x80],0x0
            m.w64(r15.wrapping_add(0x80), (0x0u64) as u64);
            // 1f481 mov QWORD PTR [r15+0x78],0x0
            m.w64(r15.wrapping_add(0x78), (0x0u64) as u64);
            // 1f489 mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 1f490 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f495 movups xmm0,XMMWORD PTR [r15+0x90]
            x0 = m.r128(r15.wrapping_add(0x90));
            // 1f49d movups XMMWORD PTR [r12+0x90],xmm0
            m.w128(r12.wrapping_add(0x90), x0);
            // 1f4a6 mov rax,QWORD PTR [r15+0xa0]
            rax = m.r64(r15.wrapping_add(0xa0)) as u64;
            // 1f4ad mov QWORD PTR [r12+0xa0],rax
            m.w64(r12.wrapping_add(0xa0), (rax) as u64);
            // 1f4b5 mov QWORD PTR [r15+0xa0],0x0
            m.w64(r15.wrapping_add(0xa0), (0x0u64) as u64);
            // 1f4c0 mov QWORD PTR [r15+0x98],0x0
            m.w64(r15.wrapping_add(0x98), (0x0u64) as u64);
            // 1f4cb mov QWORD PTR [r15+0x90],0x0
            m.w64(r15.wrapping_add(0x90), (0x0u64) as u64);
            // 1f4d6 mov rdi,QWORD PTR [rbp-0x140]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 1f4dd call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f4e2 movups xmm0,XMMWORD PTR [r15+0xa8]
            x0 = m.r128(r15.wrapping_add(0xa8));
            // 1f4ea movups XMMWORD PTR [r12+0xa8],xmm0
            m.w128(r12.wrapping_add(0xa8), x0);
            // 1f4f3 mov rax,QWORD PTR [r15+0xb8]
            rax = m.r64(r15.wrapping_add(0xb8)) as u64;
            // 1f4fa mov QWORD PTR [r12+0xb8],rax
            m.w64(r12.wrapping_add(0xb8), (rax) as u64);
            // 1f502 mov QWORD PTR [r15+0xb8],0x0
            m.w64(r15.wrapping_add(0xb8), (0x0u64) as u64);
            // 1f50d mov QWORD PTR [r15+0xb0],0x0
            m.w64(r15.wrapping_add(0xb0), (0x0u64) as u64);
            // 1f518 mov QWORD PTR [r15+0xa8],0x0
            m.w64(r15.wrapping_add(0xa8), (0x0u64) as u64);
            // 1f523 mov rdi,QWORD PTR [rbp-0x138]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 1f52a call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f52f movups xmm0,XMMWORD PTR [r15+0xc0]
            x0 = m.r128(r15.wrapping_add(0xc0));
            // 1f537 movups XMMWORD PTR [r12+0xc0],xmm0
            m.w128(r12.wrapping_add(0xc0), x0);
            // 1f540 mov rax,QWORD PTR [r15+0xd0]
            rax = m.r64(r15.wrapping_add(0xd0)) as u64;
            // 1f547 mov QWORD PTR [r12+0xd0],rax
            m.w64(r12.wrapping_add(0xd0), (rax) as u64);
            // 1f54f mov QWORD PTR [r15+0xd0],0x0
            m.w64(r15.wrapping_add(0xd0), (0x0u64) as u64);
            // 1f55a mov QWORD PTR [r15+0xc8],0x0
            m.w64(r15.wrapping_add(0xc8), (0x0u64) as u64);
            // 1f565 mov QWORD PTR [r15+0xc0],0x0
            m.w64(r15.wrapping_add(0xc0), (0x0u64) as u64);
            // 1f570 mov eax,DWORD PTR [r15+0xf0]
            rax = (m.r32(r15.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 1f577 mov rdx,QWORD PTR [rbp-0x130]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 1f57e mov DWORD PTR [rdx+0x18],eax
            m.w32(rdx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 1f581 mov rax,QWORD PTR [r15+0xe8]
            rax = m.r64(r15.wrapping_add(0xe8)) as u64;
            // 1f588 mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 1f58c mov rax,QWORD PTR [r15+0xd8]
            rax = m.r64(r15.wrapping_add(0xd8)) as u64;
            // 1f593 mov rcx,QWORD PTR [r15+0xe0]
            rcx = m.r64(r15.wrapping_add(0xe0)) as u64;
            // 1f59a mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 1f59e mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 1f5a1 mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 1f5a8 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 1f5ac mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 1f5b3 mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 1f5ba mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 1f5be mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 1f5c1 test BYTE PTR [r15+0x18],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x18)) as u64 & 0x1u64);
            // 1f5c6 jne 0x1fd41
            if crate::cov::brk(0x1f5c6, fl.ne()) { pc = 0x1fd41; continue; }
            pc = 0x1f5cc;
        }
        0x1f5cc => {
            // block 1f5cc
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1f5cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f5cc mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // 1f5d1 jmp 0x1fd50
            pc = 0x1fd50; continue;
        }
        0x1f5d6 => {
            // block 1f5d6
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1f5d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f5d6 mov rax,QWORD PTR [r14+0x28]
            rax = m.r64(r14.wrapping_add(0x28)) as u64;
            // 1f5da mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 1f5dd mov QWORD PTR [r14+0x20],0x0
            m.w64(r14.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x1f5e5;
        }
        0x1f5e5 => {
            // block 1f5e5
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1f5e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f5e5 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 1f5e7 mov rdi,r15
            rdi = r15;
            // 1f5ea call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f5ef lea rbx,[rbp-0x108]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 1f5f6 lea rax,[rbp-0xf0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 1f5fd lea rcx,[r14+0x30]
            rcx = r14.wrapping_add(0x30);
            // 1f601 lea rdi,[r14+0x60]
            rdi = r14.wrapping_add(0x60);
            // 1f605 mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 1f609 mov QWORD PTR [r15+0x10],rdx
            m.w64(r15.wrapping_add(0x10), (rdx) as u64);
            // 1f60d mov rdx,QWORD PTR [rbx]
            rdx = m.r64(rbx) as u64;
            // 1f610 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1f614 mov QWORD PTR [r15+0x8],rsi
            m.w64(r15.wrapping_add(0x8), (rsi) as u64);
            // 1f618 lea r13,[r14+0x78]
            r13 = r14.wrapping_add(0x78);
            // 1f61c mov QWORD PTR [r15],rdx
            m.w64(r15, (rdx) as u64);
            // 1f61f lea rdx,[r14+0x90]
            rdx = r14.wrapping_add(0x90);
            // 1f626 mov QWORD PTR [rbp-0x130],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (rdx) as u64);
            // 1f62d mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 1f635 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 1f63d mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 1f644 mov dx,WORD PTR [rax+0x2c]
            rdx = (rdx & !0xffff) | ((m.r16(rax.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 1f648 mov WORD PTR [rcx+0x2c],dx
            m.w16(rcx.wrapping_add(0x2c), ((rdx as u16 as u64)) as u16);
            // 1f64c mov edx,DWORD PTR [rax+0x28]
            rdx = (m.r32(rax.wrapping_add(0x28)) as u64) as u32 as u64;
            // 1f64f mov DWORD PTR [rcx+0x28],edx
            m.w32(rcx.wrapping_add(0x28), ((rdx as u32 as u64)) as u32);
            // 1f652 mov rdx,QWORD PTR [rax+0x20]
            rdx = m.r64(rax.wrapping_add(0x20)) as u64;
            // 1f656 mov QWORD PTR [rcx+0x20],rdx
            m.w64(rcx.wrapping_add(0x20), (rdx) as u64);
            // 1f65a mov rdx,QWORD PTR [rax+0x18]
            rdx = m.r64(rax.wrapping_add(0x18)) as u64;
            // 1f65e mov QWORD PTR [rcx+0x18],rdx
            m.w64(rcx.wrapping_add(0x18), (rdx) as u64);
            // 1f662 mov rdx,QWORD PTR [rax+0x10]
            rdx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 1f666 mov QWORD PTR [rcx+0x10],rdx
            m.w64(rcx.wrapping_add(0x10), (rdx) as u64);
            // 1f66a mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 1f66d mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 1f671 mov QWORD PTR [rcx+0x8],rax
            m.w64(rcx.wrapping_add(0x8), (rax) as u64);
            // 1f675 lea r15,[r14+0xa8]
            r15 = r14.wrapping_add(0xa8);
            // 1f67c mov QWORD PTR [rcx],rdx
            m.w64(rcx, (rdx) as u64);
            // 1f67f lea rax,[r14+0xc0]
            rax = r14.wrapping_add(0xc0);
            // 1f686 mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // 1f68d lea r12,[r14+0xd8]
            r12 = r14.wrapping_add(0xd8);
            // 1f694 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f699 movups xmm0,XMMWORD PTR [rbp-0xc0]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff40));
            // 1f6a0 movups XMMWORD PTR [r14+0x60],xmm0
            m.w128(r14.wrapping_add(0x60), x0);
            // 1f6a5 mov rax,QWORD PTR [rbp-0xb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 1f6ac mov QWORD PTR [r14+0x70],rax
            m.w64(r14.wrapping_add(0x70), (rax) as u64);
            // 1f6b0 lea rax,[rbp-0xc0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 1f6b7 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1f6bf mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1f6c7 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1f6ce mov rdi,r13
            rdi = r13;
            // 1f6d1 call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f6d6 movups xmm0,XMMWORD PTR [rbp-0xa8]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff58));
            // 1f6dd movups XMMWORD PTR [r14+0x78],xmm0
            m.w128(r14.wrapping_add(0x78), x0);
            // 1f6e2 mov rax,QWORD PTR [rbp-0x98]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 1f6e9 mov QWORD PTR [r14+0x88],rax
            m.w64(r14.wrapping_add(0x88), (rax) as u64);
            // 1f6f0 lea rax,[rbp-0xa8]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 1f6f7 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1f6ff mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1f707 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1f70e mov rdi,QWORD PTR [rbp-0x130]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 1f715 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f71a lea r13,[rbp-0x90]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 1f721 movups xmm0,XMMWORD PTR [rbp-0x90]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff70));
            // 1f728 movups XMMWORD PTR [r14+0x90],xmm0
            m.w128(r14.wrapping_add(0x90), x0);
            // 1f730 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 1f734 mov QWORD PTR [r14+0xa0],rax
            m.w64(r14.wrapping_add(0xa0), (rax) as u64);
            // 1f73b lea rax,[rbp-0x90]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 1f742 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1f74a mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1f752 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1f759 mov rdi,r15
            rdi = r15;
            // 1f75c call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f761 lea rbx,[rbp-0x78]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 1f765 movups xmm0,XMMWORD PTR [rbp-0x78]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff88));
            // 1f769 movups XMMWORD PTR [r14+0xa8],xmm0
            m.w128(r14.wrapping_add(0xa8), x0);
            // 1f771 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1f775 mov QWORD PTR [r14+0xb8],rax
            m.w64(r14.wrapping_add(0xb8), (rax) as u64);
            // 1f77c lea rax,[rbp-0x78]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 1f780 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1f788 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1f790 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1f797 mov rdi,QWORD PTR [rbp-0x128]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 1f79e call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f7a3 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1f7a7 movups xmm0,XMMWORD PTR [rbp-0x60]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0));
            // 1f7ab movups XMMWORD PTR [r14+0xc0],xmm0
            m.w128(r14.wrapping_add(0xc0), x0);
            // 1f7b3 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 1f7b7 mov QWORD PTR [r14+0xd0],rax
            m.w64(r14.wrapping_add(0xd0), (rax) as u64);
            // 1f7be lea rax,[rbp-0x60]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1f7c2 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1f7ca mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1f7d2 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1f7d9 lea rcx,[rbp-0x48]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 1f7dd mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 1f7e0 mov DWORD PTR [r12+0x18],eax
            m.w32(r12.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 1f7e5 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 1f7e9 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1f7ee mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 1f7f1 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 1f7f5 mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 1f7fa mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 1f7fe call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f803 mov rdi,rbx
            rdi = rbx;
            // 1f806 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f80b mov rdi,r13
            rdi = r13;
            // 1f80e call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f813 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 1f81a call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f81f lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 1f826 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f82b lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 1f832 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f837 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 1f83c jmp 0x20b73
            pc = 0x20b73; continue;
        }
        0x1f841 => {
            // block 1f841
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1f841 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f841 mov rax,QWORD PTR [r14+0x28]
            rax = m.r64(r14.wrapping_add(0x28)) as u64;
            // 1f845 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 1f848 mov QWORD PTR [r14+0x20],0x0
            m.w64(r14.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x1f850;
        }
        0x1f850 => {
            // block 1f850
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1f850 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1f850 mov QWORD PTR [rbp-0x178],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rbx) as u64);
            // 1f857 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 1f859 mov rdi,r13
            rdi = r13;
            // 1f85c call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f861 lea rbx,[rbp-0x108]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 1f868 lea rsi,[rbp-0xf0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 1f86f lea rax,[r14+0x30]
            rax = r14.wrapping_add(0x30);
            // 1f873 lea rdi,[r14+0x60]
            rdi = r14.wrapping_add(0x60);
            // 1f877 lea rcx,[r14+0x78]
            rcx = r14.wrapping_add(0x78);
            // 1f87b mov QWORD PTR [rbp-0x168],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rcx) as u64);
            // 1f882 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 1f886 mov QWORD PTR [r13+0x10],rcx
            m.w64(r13.wrapping_add(0x10), (rcx) as u64);
            // 1f88a mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 1f88d mov rdx,QWORD PTR [rbx+0x8]
            rdx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1f891 mov QWORD PTR [r13+0x8],rdx
            m.w64(r13.wrapping_add(0x8), (rdx) as u64);
            // 1f895 lea rdx,[r14+0x90]
            rdx = r14.wrapping_add(0x90);
            // 1f89c mov QWORD PTR [rbp-0x160],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rdx) as u64);
            // 1f8a3 mov QWORD PTR [r13+0x0],rcx
            m.w64(r13, (rcx) as u64);
            // 1f8a7 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 1f8af mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 1f8b7 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 1f8be mov cx,WORD PTR [rsi+0x2c]
            rcx = (rcx & !0xffff) | ((m.r16(rsi.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 1f8c2 mov WORD PTR [rax+0x2c],cx
            m.w16(rax.wrapping_add(0x2c), ((rcx as u16 as u64)) as u16);
            // 1f8c6 mov ecx,DWORD PTR [rsi+0x28]
            rcx = (m.r32(rsi.wrapping_add(0x28)) as u64) as u32 as u64;
            // 1f8c9 mov DWORD PTR [rax+0x28],ecx
            m.w32(rax.wrapping_add(0x28), ((rcx as u32 as u64)) as u32);
            // 1f8cc mov rcx,QWORD PTR [rsi+0x20]
            rcx = m.r64(rsi.wrapping_add(0x20)) as u64;
            // 1f8d0 mov QWORD PTR [rax+0x20],rcx
            m.w64(rax.wrapping_add(0x20), (rcx) as u64);
            // 1f8d4 mov rcx,QWORD PTR [rsi+0x18]
            rcx = m.r64(rsi.wrapping_add(0x18)) as u64;
            // 1f8d8 mov QWORD PTR [rax+0x18],rcx
            m.w64(rax.wrapping_add(0x18), (rcx) as u64);
            // 1f8dc mov rcx,QWORD PTR [rsi+0x10]
            rcx = m.r64(rsi.wrapping_add(0x10)) as u64;
            // 1f8e0 mov QWORD PTR [rax+0x10],rcx
            m.w64(rax.wrapping_add(0x10), (rcx) as u64);
            // 1f8e4 mov rcx,QWORD PTR [rsi]
            rcx = m.r64(rsi) as u64;
            // 1f8e7 mov rdx,QWORD PTR [rsi+0x8]
            rdx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // 1f8eb mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 1f8ef lea r13,[r14+0xa8]
            r13 = r14.wrapping_add(0xa8);
            // 1f8f6 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 1f8f9 lea rax,[r14+0xc0]
            rax = r14.wrapping_add(0xc0);
            // 1f900 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // 1f907 lea rbx,[r14+0xd8]
            rbx = r14.wrapping_add(0xd8);
            // 1f90e call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f913 movups xmm0,XMMWORD PTR [rbp-0xc0]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff40));
            // 1f91a movups XMMWORD PTR [r14+0x60],xmm0
            m.w128(r14.wrapping_add(0x60), x0);
            // 1f91f mov rax,QWORD PTR [rbp-0xb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 1f926 mov QWORD PTR [r14+0x70],rax
            m.w64(r14.wrapping_add(0x70), (rax) as u64);
            // 1f92a lea rax,[rbp-0xc0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 1f931 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1f939 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1f941 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1f948 mov rdi,QWORD PTR [rbp-0x168]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 1f94f call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f954 movups xmm0,XMMWORD PTR [rbp-0xa8]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff58));
            // 1f95b movups XMMWORD PTR [r14+0x78],xmm0
            m.w128(r14.wrapping_add(0x78), x0);
            // 1f960 mov rax,QWORD PTR [rbp-0x98]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 1f967 mov QWORD PTR [r14+0x88],rax
            m.w64(r14.wrapping_add(0x88), (rax) as u64);
            // 1f96e lea rax,[rbp-0xa8]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 1f975 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1f97d mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1f985 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1f98c mov rdi,QWORD PTR [rbp-0x160]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 1f993 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f998 movups xmm0,XMMWORD PTR [rbp-0x90]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff70));
            // 1f99f movups XMMWORD PTR [r14+0x90],xmm0
            m.w128(r14.wrapping_add(0x90), x0);
            // 1f9a7 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 1f9ab mov QWORD PTR [r14+0xa0],rax
            m.w64(r14.wrapping_add(0xa0), (rax) as u64);
            // 1f9b2 lea rax,[rbp-0x90]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 1f9b9 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1f9c1 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1f9c9 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1f9d0 mov rdi,r13
            rdi = r13;
            // 1f9d3 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1f9d8 lea r13,[rbp-0x78]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 1f9dc movups xmm0,XMMWORD PTR [rbp-0x78]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff88));
            // 1f9e0 movups XMMWORD PTR [r14+0xa8],xmm0
            m.w128(r14.wrapping_add(0xa8), x0);
            // 1f9e8 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1f9ec mov QWORD PTR [r14+0xb8],rax
            m.w64(r14.wrapping_add(0xb8), (rax) as u64);
            // 1f9f3 lea rax,[rbp-0x78]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 1f9f7 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1f9ff mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1fa07 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1fa0e mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 1fa15 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1fa1a lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1fa1e movups xmm0,XMMWORD PTR [rbp-0x60]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0));
            // 1fa22 movups XMMWORD PTR [r14+0xc0],xmm0
            m.w128(r14.wrapping_add(0xc0), x0);
            // 1fa2a mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 1fa2e mov QWORD PTR [r14+0xd0],rax
            m.w64(r14.wrapping_add(0xd0), (rax) as u64);
            // 1fa35 lea rax,[rbp-0x60]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1fa39 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1fa41 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1fa49 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1fa50 lea rcx,[rbp-0x48]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 1fa54 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 1fa57 mov DWORD PTR [rbx+0x18],eax
            m.w32(rbx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 1fa5a mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 1fa5e mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 1fa62 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 1fa65 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 1fa69 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 1fa6d mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 1fa70 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1fa75 mov rdi,r13
            rdi = r13;
            // 1fa78 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1fa7d lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 1fa84 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1fa89 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 1fa90 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1fa95 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 1fa9c call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1faa1 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 1faa8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1faad mov rdi,r15
            rdi = r15;
            // 1fab0 mov rsi,r12
            rsi = r12;
            // 1fab3 mov rax,QWORD PTR [rbp-0x128]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 1faba call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1fabc mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1fabe mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 1fac3 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 1fac5 je 0x20b73
            if crate::cov::brk(0x1fac5, fl.e()) { pc = 0x20b73; continue; }
            pc = 0x1facb;
        }
        0x1facb => {
            // block 1facb
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1facb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1facb mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1fad0 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 1fad7 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 1fadb mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1fae0 mov QWORD PTR [rbp-0x118],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rcx) as u64);
            // 1fae7 mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 1faee lea r14,[r12+0x18]
            r14 = r12.wrapping_add(0x18);
            // 1faf3 mov rax,QWORD PTR [r12+0x28]
            rax = m.r64(r12.wrapping_add(0x28)) as u64;
            // 1faf8 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // 1faff mov rax,QWORD PTR [r12+0x18]
            rax = m.r64(r12.wrapping_add(0x18)) as u64;
            // 1fb04 mov rcx,QWORD PTR [r12+0x20]
            rcx = m.r64(r12.wrapping_add(0x20)) as u64;
            // 1fb09 mov QWORD PTR [rbp-0x100],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rcx) as u64);
            // 1fb10 mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 1fb17 mov QWORD PTR [r12+0x28],0x0
            m.w64(r12.wrapping_add(0x28), (0x0u64) as u64);
            // 1fb20 mov QWORD PTR [r12+0x20],0x0
            m.w64(r12.wrapping_add(0x20), (0x0u64) as u64);
            // 1fb29 mov QWORD PTR [r12+0x18],0x0
            m.w64(r12.wrapping_add(0x18), (0x0u64) as u64);
            // 1fb32 mov ax,WORD PTR [r12+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r12.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 1fb38 mov WORD PTR [rbp-0xc4],ax
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff3c), ((rax as u16 as u64)) as u16);
            // 1fb3f mov eax,DWORD PTR [r12+0x58]
            rax = (m.r32(r12.wrapping_add(0x58)) as u64) as u32 as u64;
            // 1fb44 mov DWORD PTR [rbp-0xc8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), ((rax as u32 as u64)) as u32);
            // 1fb4a mov rax,QWORD PTR [r12+0x50]
            rax = m.r64(r12.wrapping_add(0x50)) as u64;
            // 1fb4f mov QWORD PTR [rbp-0xd0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (rax) as u64);
            // 1fb56 mov rax,QWORD PTR [r12+0x48]
            rax = m.r64(r12.wrapping_add(0x48)) as u64;
            // 1fb5b mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 1fb62 mov rax,QWORD PTR [r12+0x40]
            rax = m.r64(r12.wrapping_add(0x40)) as u64;
            // 1fb67 mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 1fb6e mov rax,QWORD PTR [r12+0x30]
            rax = m.r64(r12.wrapping_add(0x30)) as u64;
            // 1fb73 mov rcx,QWORD PTR [r12+0x38]
            rcx = m.r64(r12.wrapping_add(0x38)) as u64;
            // 1fb78 mov QWORD PTR [rbp-0xe8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rcx) as u64);
            // 1fb7f mov QWORD PTR [rbp-0xf0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (rax) as u64);
            // 1fb86 mov QWORD PTR [rbp-0xc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u64);
            // 1fb91 mov rax,QWORD PTR [r12+0x60]
            rax = m.r64(r12.wrapping_add(0x60)) as u64;
            // 1fb96 mov QWORD PTR [rbp-0xc0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rax) as u64);
            // 1fb9d movups xmm0,XMMWORD PTR [r12+0x68]
            x0 = m.r128(r12.wrapping_add(0x68));
            // 1fba3 movups XMMWORD PTR [rbp-0xb8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff48), x0);
            // 1fbaa mov QWORD PTR [r12+0x70],0x0
            m.w64(r12.wrapping_add(0x70), (0x0u64) as u64);
            // 1fbb3 mov QWORD PTR [r12+0x68],0x0
            m.w64(r12.wrapping_add(0x68), (0x0u64) as u64);
            // 1fbbc mov QWORD PTR [r12+0x60],0x0
            m.w64(r12.wrapping_add(0x60), (0x0u64) as u64);
            // 1fbc5 mov QWORD PTR [rbp-0xa8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u64);
            // 1fbd0 movups xmm0,XMMWORD PTR [r12+0x78]
            x0 = m.r128(r12.wrapping_add(0x78));
            // 1fbd6 movups XMMWORD PTR [rbp-0xa8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff58), x0);
            // 1fbdd mov rax,QWORD PTR [r12+0x88]
            rax = m.r64(r12.wrapping_add(0x88)) as u64;
            // 1fbe5 mov QWORD PTR [rbp-0x98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rax) as u64);
            // 1fbec mov QWORD PTR [r12+0x88],0x0
            m.w64(r12.wrapping_add(0x88), (0x0u64) as u64);
            // 1fbf8 mov QWORD PTR [r12+0x80],0x0
            m.w64(r12.wrapping_add(0x80), (0x0u64) as u64);
            // 1fc04 mov QWORD PTR [r12+0x78],0x0
            m.w64(r12.wrapping_add(0x78), (0x0u64) as u64);
            // 1fc0d mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 1fc18 movups xmm0,XMMWORD PTR [r12+0x90]
            x0 = m.r128(r12.wrapping_add(0x90));
            // 1fc21 movups XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // 1fc28 mov rax,QWORD PTR [r12+0xa0]
            rax = m.r64(r12.wrapping_add(0xa0)) as u64;
            // 1fc30 mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 1fc34 mov QWORD PTR [r12+0xa0],0x0
            m.w64(r12.wrapping_add(0xa0), (0x0u64) as u64);
            // 1fc40 mov QWORD PTR [r12+0x98],0x0
            m.w64(r12.wrapping_add(0x98), (0x0u64) as u64);
            // 1fc4c mov QWORD PTR [r12+0x90],0x0
            m.w64(r12.wrapping_add(0x90), (0x0u64) as u64);
            // 1fc58 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 1fc60 movups xmm0,XMMWORD PTR [r12+0xa8]
            x0 = m.r128(r12.wrapping_add(0xa8));
            // 1fc69 movups XMMWORD PTR [rbp-0x78],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff88), x0);
            // 1fc6d mov rax,QWORD PTR [r12+0xb8]
            rax = m.r64(r12.wrapping_add(0xb8)) as u64;
            // 1fc75 mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 1fc79 mov QWORD PTR [r12+0xb8],0x0
            m.w64(r12.wrapping_add(0xb8), (0x0u64) as u64);
            // 1fc85 mov QWORD PTR [r12+0xb0],0x0
            m.w64(r12.wrapping_add(0xb0), (0x0u64) as u64);
            // 1fc91 mov QWORD PTR [r12+0xa8],0x0
            m.w64(r12.wrapping_add(0xa8), (0x0u64) as u64);
            // 1fc9d mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 1fca5 mov rax,QWORD PTR [r12+0xc0]
            rax = m.r64(r12.wrapping_add(0xc0)) as u64;
            // 1fcad mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 1fcb1 movups xmm0,XMMWORD PTR [r12+0xc8]
            x0 = m.r128(r12.wrapping_add(0xc8));
            // 1fcba movups XMMWORD PTR [rbp-0x58],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), x0);
            // 1fcbe mov QWORD PTR [r12+0xd0],0x0
            m.w64(r12.wrapping_add(0xd0), (0x0u64) as u64);
            // 1fcca mov QWORD PTR [r12+0xc8],0x0
            m.w64(r12.wrapping_add(0xc8), (0x0u64) as u64);
            // 1fcd6 mov QWORD PTR [r12+0xc0],0x0
            m.w64(r12.wrapping_add(0xc0), (0x0u64) as u64);
            // 1fce2 mov eax,DWORD PTR [r12+0xf0]
            rax = (m.r32(r12.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 1fcea mov DWORD PTR [rbp-0x30],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rax as u32 as u64)) as u32);
            // 1fced mov rax,QWORD PTR [r12+0xe8]
            rax = m.r64(r12.wrapping_add(0xe8)) as u64;
            // 1fcf5 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1fcf9 mov rax,QWORD PTR [r12+0xd8]
            rax = m.r64(r12.wrapping_add(0xd8)) as u64;
            // 1fd01 mov rcx,QWORD PTR [r12+0xe0]
            rcx = m.r64(r12.wrapping_add(0xe0)) as u64;
            // 1fd09 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 1fd0d mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 1fd11 mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 1fd15 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1fd1a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 1fd1d mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1fd21 mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 1fd26 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 1fd2a test BYTE PTR [r12+0x18],0x1
            fl = logic::<8>(m.r8(r12.wrapping_add(0x18)) as u64 & 0x1u64);
            // 1fd30 jne 0x20455
            if crate::cov::brk(0x1fd30, fl.ne()) { pc = 0x20455; continue; }
            pc = 0x1fd36;
        }
        0x1fd36 => {
            // block 1fd36
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1fd36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1fd36 mov WORD PTR [r14],0x0
            m.w16(r14, (0x0u64) as u16);
            // 1fd3c jmp 0x20466
            pc = 0x20466; continue;
        }
        0x1fd41 => {
            // block 1fd41
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1fd41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1fd41 mov rax,QWORD PTR [r15+0x28]
            rax = m.r64(r15.wrapping_add(0x28)) as u64;
            // 1fd45 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 1fd48 mov QWORD PTR [r15+0x20],0x0
            m.w64(r15.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x1fd50;
        }
        0x1fd50 => {
            // block 1fd50
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1fd50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1fd50 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 1fd52 mov rdi,rbx
            rdi = rbx;
            // 1fd55 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1fd5a lea rsi,[rbp-0x108]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 1fd61 lea rdx,[rbp-0xf0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 1fd68 lea r12,[r15+0x30]
            r12 = r15.wrapping_add(0x30);
            // 1fd6c lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 1fd70 mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // 1fd77 lea rax,[r15+0x78]
            rax = r15.wrapping_add(0x78);
            // 1fd7b mov QWORD PTR [rbp-0x140],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rax) as u64);
            // 1fd82 lea rax,[r15+0x90]
            rax = r15.wrapping_add(0x90);
            // 1fd89 mov QWORD PTR [rbp-0x130],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (rax) as u64);
            // 1fd90 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // 1fd94 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 1fd98 mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // 1fd9b mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // 1fd9f mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 1fda3 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 1fda6 mov QWORD PTR [rbp-0x160],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rbx) as u64);
            // 1fdad mov QWORD PTR [rsi+0x10],0x0
            m.w64(rsi.wrapping_add(0x10), (0x0u64) as u64);
            // 1fdb5 mov QWORD PTR [rsi+0x8],0x0
            m.w64(rsi.wrapping_add(0x8), (0x0u64) as u64);
            // 1fdbd mov QWORD PTR [rsi],0x0
            m.w64(rsi, (0x0u64) as u64);
            // 1fdc4 mov ax,WORD PTR [rdx+0x2c]
            rax = (rax & !0xffff) | ((m.r16(rdx.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 1fdc8 mov WORD PTR [r12+0x2c],ax
            m.w16(r12.wrapping_add(0x2c), ((rax as u16 as u64)) as u16);
            // 1fdce mov eax,DWORD PTR [rdx+0x28]
            rax = (m.r32(rdx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 1fdd1 mov DWORD PTR [r12+0x28],eax
            m.w32(r12.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 1fdd6 mov rax,QWORD PTR [rdx+0x20]
            rax = m.r64(rdx.wrapping_add(0x20)) as u64;
            // 1fdda mov QWORD PTR [r12+0x20],rax
            m.w64(r12.wrapping_add(0x20), (rax) as u64);
            // 1fddf mov rax,QWORD PTR [rdx+0x18]
            rax = m.r64(rdx.wrapping_add(0x18)) as u64;
            // 1fde3 mov QWORD PTR [r12+0x18],rax
            m.w64(r12.wrapping_add(0x18), (rax) as u64);
            // 1fde8 mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // 1fdec mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1fdf1 mov rax,QWORD PTR [rdx]
            rax = m.r64(rdx) as u64;
            // 1fdf4 mov rcx,QWORD PTR [rdx+0x8]
            rcx = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 1fdf8 mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 1fdfd lea r13,[r15+0xa8]
            r13 = r15.wrapping_add(0xa8);
            // 1fe04 mov QWORD PTR [rbp-0x158],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (r13) as u64);
            // 1fe0b mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 1fe0f lea rax,[r15+0xc0]
            rax = r15.wrapping_add(0xc0);
            // 1fe16 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // 1fe1d lea rbx,[r15+0xd8]
            rbx = r15.wrapping_add(0xd8);
            // 1fe24 mov QWORD PTR [rbp-0x148],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rbx) as u64);
            // 1fe2b call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1fe30 movups xmm0,XMMWORD PTR [rbp-0xc0]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff40));
            // 1fe37 movups XMMWORD PTR [r15+0x60],xmm0
            m.w128(r15.wrapping_add(0x60), x0);
            // 1fe3c mov rax,QWORD PTR [rbp-0xb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 1fe43 mov QWORD PTR [r15+0x70],rax
            m.w64(r15.wrapping_add(0x70), (rax) as u64);
            // 1fe47 lea rax,[rbp-0xc0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 1fe4e mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1fe56 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1fe5e mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1fe65 mov rdi,QWORD PTR [rbp-0x140]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 1fe6c call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1fe71 movups xmm0,XMMWORD PTR [rbp-0xa8]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff58));
            // 1fe78 movups XMMWORD PTR [r15+0x78],xmm0
            m.w128(r15.wrapping_add(0x78), x0);
            // 1fe7d mov rax,QWORD PTR [rbp-0x98]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 1fe84 mov QWORD PTR [r15+0x88],rax
            m.w64(r15.wrapping_add(0x88), (rax) as u64);
            // 1fe8b lea rax,[rbp-0xa8]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 1fe92 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1fe9a mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1fea2 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1fea9 mov rdi,QWORD PTR [rbp-0x130]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 1feb0 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1feb5 movups xmm0,XMMWORD PTR [rbp-0x90]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff70));
            // 1febc movups XMMWORD PTR [r15+0x90],xmm0
            m.w128(r15.wrapping_add(0x90), x0);
            // 1fec4 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 1fec8 mov QWORD PTR [r15+0xa0],rax
            m.w64(r15.wrapping_add(0xa0), (rax) as u64);
            // 1fecf lea rax,[rbp-0x90]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 1fed6 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1fede mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1fee6 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1feed mov rdi,r13
            rdi = r13;
            // 1fef0 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1fef5 lea r13,[rbp-0x78]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 1fef9 movups xmm0,XMMWORD PTR [rbp-0x78]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff88));
            // 1fefd movups XMMWORD PTR [r15+0xa8],xmm0
            m.w128(r15.wrapping_add(0xa8), x0);
            // 1ff05 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1ff09 mov QWORD PTR [r15+0xb8],rax
            m.w64(r15.wrapping_add(0xb8), (rax) as u64);
            // 1ff10 lea rax,[rbp-0x78]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 1ff14 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1ff1c mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1ff24 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1ff2b mov rdi,QWORD PTR [rbp-0x138]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 1ff32 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1ff37 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1ff3b movups xmm0,XMMWORD PTR [rbp-0x60]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0));
            // 1ff3f movups XMMWORD PTR [r15+0xc0],xmm0
            m.w128(r15.wrapping_add(0xc0), x0);
            // 1ff47 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 1ff4b mov QWORD PTR [r15+0xd0],rax
            m.w64(r15.wrapping_add(0xd0), (rax) as u64);
            // 1ff52 lea rax,[rbp-0x60]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1ff56 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 1ff5e mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 1ff66 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 1ff6d lea rcx,[rbp-0x48]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 1ff71 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 1ff74 mov DWORD PTR [rbx+0x18],eax
            m.w32(rbx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 1ff77 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 1ff7b mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 1ff7f mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 1ff82 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 1ff86 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 1ff8a mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 1ff8d call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1ff92 mov rdi,r13
            rdi = r13;
            // 1ff95 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1ff9a lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 1ffa1 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1ffa6 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 1ffad call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1ffb2 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 1ffb9 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1ffbe lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 1ffc5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1ffca mov rdi,r14
            rdi = r14;
            // 1ffcd mov rsi,r15
            rsi = r15;
            // 1ffd0 mov rax,QWORD PTR [rbp-0x128]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 1ffd7 call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1ffd9 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1ffdb mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 1ffe0 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 1ffe2 je 0x20b73
            if crate::cov::brk(0x1ffe2, fl.e()) { pc = 0x20b73; continue; }
            pc = 0x1ffe8;
        }
        0x1ffe8 => {
            // block 1ffe8
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 1ffe8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ffe8 mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 1ffec mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 1fff3 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 1fff6 mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1fffa mov QWORD PTR [rbp-0x118],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rcx) as u64);
            // 20001 mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 20008 mov rbx,QWORD PTR [rbp-0x160]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 2000f mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 20013 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // 2001a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2001d mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 20021 mov QWORD PTR [rbp-0x100],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rcx) as u64);
            // 20028 mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 2002f mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 20037 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 2003f mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 20046 mov ax,WORD PTR [r12+0x2c]
            rax = (rax & !0xffff) | ((m.r16(r12.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 2004c mov WORD PTR [rbp-0xc4],ax
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff3c), ((rax as u16 as u64)) as u16);
            // 20053 mov eax,DWORD PTR [r12+0x28]
            rax = (m.r32(r12.wrapping_add(0x28)) as u64) as u32 as u64;
            // 20058 mov DWORD PTR [rbp-0xc8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), ((rax as u32 as u64)) as u32);
            // 2005e mov rax,QWORD PTR [r12+0x20]
            rax = m.r64(r12.wrapping_add(0x20)) as u64;
            // 20063 mov QWORD PTR [rbp-0xd0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (rax) as u64);
            // 2006a mov rax,QWORD PTR [r12+0x18]
            rax = m.r64(r12.wrapping_add(0x18)) as u64;
            // 2006f mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 20076 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // 2007b mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 20082 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 20086 mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2008b mov QWORD PTR [rbp-0xe8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rcx) as u64);
            // 20092 mov QWORD PTR [rbp-0xf0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (rax) as u64);
            // 20099 mov QWORD PTR [rbp-0xc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u64);
            // 200a4 mov rax,QWORD PTR [r15+0x60]
            rax = m.r64(r15.wrapping_add(0x60)) as u64;
            // 200a8 mov QWORD PTR [rbp-0xc0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rax) as u64);
            // 200af movups xmm0,XMMWORD PTR [r15+0x68]
            x0 = m.r128(r15.wrapping_add(0x68));
            // 200b4 movups XMMWORD PTR [rbp-0xb8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff48), x0);
            // 200bb mov rax,QWORD PTR [rbp-0x150]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 200c2 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 200ca mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 200d2 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 200d9 mov QWORD PTR [rbp-0xa8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u64);
            // 200e4 movups xmm0,XMMWORD PTR [r15+0x78]
            x0 = m.r128(r15.wrapping_add(0x78));
            // 200e9 movups XMMWORD PTR [rbp-0xa8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff58), x0);
            // 200f0 mov rax,QWORD PTR [r15+0x88]
            rax = m.r64(r15.wrapping_add(0x88)) as u64;
            // 200f7 mov QWORD PTR [rbp-0x98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rax) as u64);
            // 200fe mov r13,QWORD PTR [rbp-0x140]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 20105 mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // 2010d mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // 20115 mov QWORD PTR [r13+0x0],0x0
            m.w64(r13, (0x0u64) as u64);
            // 2011d mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 20128 movups xmm0,XMMWORD PTR [r15+0x90]
            x0 = m.r128(r15.wrapping_add(0x90));
            // 20130 movups XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // 20137 mov rax,QWORD PTR [r15+0xa0]
            rax = m.r64(r15.wrapping_add(0xa0)) as u64;
            // 2013e mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 20142 mov rax,QWORD PTR [rbp-0x130]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 20149 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 20151 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 20159 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 20160 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 20168 movups xmm0,XMMWORD PTR [r15+0xa8]
            x0 = m.r128(r15.wrapping_add(0xa8));
            // 20170 movups XMMWORD PTR [rbp-0x78],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff88), x0);
            // 20174 mov rax,QWORD PTR [r15+0xb8]
            rax = m.r64(r15.wrapping_add(0xb8)) as u64;
            // 2017b mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 2017f mov r13,QWORD PTR [rbp-0x158]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 20186 mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // 2018e mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // 20196 mov QWORD PTR [r13+0x0],0x0
            m.w64(r13, (0x0u64) as u64);
            // 2019e mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 201a6 mov rax,QWORD PTR [r15+0xc0]
            rax = m.r64(r15.wrapping_add(0xc0)) as u64;
            // 201ad mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 201b1 movups xmm0,XMMWORD PTR [r15+0xc8]
            x0 = m.r128(r15.wrapping_add(0xc8));
            // 201b9 movups XMMWORD PTR [rbp-0x58],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), x0);
            // 201bd mov rax,QWORD PTR [rbp-0x138]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 201c4 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 201cc mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 201d4 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 201db mov rcx,QWORD PTR [rbp-0x148]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 201e2 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 201e5 mov DWORD PTR [rbp-0x30],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rax as u32 as u64)) as u32);
            // 201e8 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 201ec mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 201f0 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 201f3 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 201f7 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 201fb mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 201ff mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // 20203 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 20207 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 2020a mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 2020e mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 20212 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 20215 mov WORD PTR [r15+0x18],0x0
            m.w16(r15.wrapping_add(0x18), (0x0u64) as u16);
            // 2021c xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2021e mov rdi,rbx
            rdi = rbx;
            // 20221 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20226 mov rdx,rbx
            rdx = rbx;
            // 20229 lea rbx,[r14+0x18]
            rbx = r14.wrapping_add(0x18);
            // 2022d mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 20231 mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 20235 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 20238 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2023c mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 20240 mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 20243 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 2024b mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 20253 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 2025a mov ax,WORD PTR [r14+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r14.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 2025f mov WORD PTR [r12+0x2c],ax
            m.w16(r12.wrapping_add(0x2c), ((rax as u16 as u64)) as u16);
            // 20265 mov eax,DWORD PTR [r14+0x58]
            rax = (m.r32(r14.wrapping_add(0x58)) as u64) as u32 as u64;
            // 20269 mov DWORD PTR [r12+0x28],eax
            m.w32(r12.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 2026e mov rax,QWORD PTR [r14+0x50]
            rax = m.r64(r14.wrapping_add(0x50)) as u64;
            // 20272 mov QWORD PTR [r12+0x20],rax
            m.w64(r12.wrapping_add(0x20), (rax) as u64);
            // 20277 mov rax,QWORD PTR [r14+0x48]
            rax = m.r64(r14.wrapping_add(0x48)) as u64;
            // 2027b mov QWORD PTR [r12+0x18],rax
            m.w64(r12.wrapping_add(0x18), (rax) as u64);
            // 20280 mov rax,QWORD PTR [r14+0x40]
            rax = m.r64(r14.wrapping_add(0x40)) as u64;
            // 20284 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 20289 mov rax,QWORD PTR [r14+0x30]
            rax = m.r64(r14.wrapping_add(0x30)) as u64;
            // 2028d mov rcx,QWORD PTR [r14+0x38]
            rcx = m.r64(r14.wrapping_add(0x38)) as u64;
            // 20291 mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 20296 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 2029a mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 202a1 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 202a6 movups xmm0,XMMWORD PTR [r14+0x60]
            x0 = m.r128(r14.wrapping_add(0x60));
            // 202ab movups XMMWORD PTR [r15+0x60],xmm0
            m.w128(r15.wrapping_add(0x60), x0);
            // 202b0 mov rax,QWORD PTR [r14+0x70]
            rax = m.r64(r14.wrapping_add(0x70)) as u64;
            // 202b4 mov QWORD PTR [r15+0x70],rax
            m.w64(r15.wrapping_add(0x70), (rax) as u64);
            // 202b8 mov QWORD PTR [r14+0x70],0x0
            m.w64(r14.wrapping_add(0x70), (0x0u64) as u64);
            // 202c0 mov QWORD PTR [r14+0x68],0x0
            m.w64(r14.wrapping_add(0x68), (0x0u64) as u64);
            // 202c8 mov QWORD PTR [r14+0x60],0x0
            m.w64(r14.wrapping_add(0x60), (0x0u64) as u64);
            // 202d0 mov rdi,QWORD PTR [rbp-0x140]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 202d7 call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 202dc movups xmm0,XMMWORD PTR [r14+0x78]
            x0 = m.r128(r14.wrapping_add(0x78));
            // 202e1 movups XMMWORD PTR [r15+0x78],xmm0
            m.w128(r15.wrapping_add(0x78), x0);
            // 202e6 mov rax,QWORD PTR [r14+0x88]
            rax = m.r64(r14.wrapping_add(0x88)) as u64;
            // 202ed mov QWORD PTR [r15+0x88],rax
            m.w64(r15.wrapping_add(0x88), (rax) as u64);
            // 202f4 mov QWORD PTR [r14+0x88],0x0
            m.w64(r14.wrapping_add(0x88), (0x0u64) as u64);
            // 202ff mov QWORD PTR [r14+0x80],0x0
            m.w64(r14.wrapping_add(0x80), (0x0u64) as u64);
            // 2030a mov QWORD PTR [r14+0x78],0x0
            m.w64(r14.wrapping_add(0x78), (0x0u64) as u64);
            // 20312 mov rdi,QWORD PTR [rbp-0x130]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 20319 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2031e movups xmm0,XMMWORD PTR [r14+0x90]
            x0 = m.r128(r14.wrapping_add(0x90));
            // 20326 movups XMMWORD PTR [r15+0x90],xmm0
            m.w128(r15.wrapping_add(0x90), x0);
            // 2032e mov rax,QWORD PTR [r14+0xa0]
            rax = m.r64(r14.wrapping_add(0xa0)) as u64;
            // 20335 mov QWORD PTR [r15+0xa0],rax
            m.w64(r15.wrapping_add(0xa0), (rax) as u64);
            // 2033c mov QWORD PTR [r14+0xa0],0x0
            m.w64(r14.wrapping_add(0xa0), (0x0u64) as u64);
            // 20347 mov QWORD PTR [r14+0x98],0x0
            m.w64(r14.wrapping_add(0x98), (0x0u64) as u64);
            // 20352 mov QWORD PTR [r14+0x90],0x0
            m.w64(r14.wrapping_add(0x90), (0x0u64) as u64);
            // 2035d mov rdi,r13
            rdi = r13;
            // 20360 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20365 movups xmm0,XMMWORD PTR [r14+0xa8]
            x0 = m.r128(r14.wrapping_add(0xa8));
            // 2036d movups XMMWORD PTR [r15+0xa8],xmm0
            m.w128(r15.wrapping_add(0xa8), x0);
            // 20375 mov rax,QWORD PTR [r14+0xb8]
            rax = m.r64(r14.wrapping_add(0xb8)) as u64;
            // 2037c mov QWORD PTR [r15+0xb8],rax
            m.w64(r15.wrapping_add(0xb8), (rax) as u64);
            // 20383 mov QWORD PTR [r14+0xb8],0x0
            m.w64(r14.wrapping_add(0xb8), (0x0u64) as u64);
            // 2038e mov QWORD PTR [r14+0xb0],0x0
            m.w64(r14.wrapping_add(0xb0), (0x0u64) as u64);
            // 20399 mov QWORD PTR [r14+0xa8],0x0
            m.w64(r14.wrapping_add(0xa8), (0x0u64) as u64);
            // 203a4 mov rdi,QWORD PTR [rbp-0x138]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 203ab call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 203b0 movups xmm0,XMMWORD PTR [r14+0xc0]
            x0 = m.r128(r14.wrapping_add(0xc0));
            // 203b8 movups XMMWORD PTR [r15+0xc0],xmm0
            m.w128(r15.wrapping_add(0xc0), x0);
            // 203c0 mov rax,QWORD PTR [r14+0xd0]
            rax = m.r64(r14.wrapping_add(0xd0)) as u64;
            // 203c7 mov QWORD PTR [r15+0xd0],rax
            m.w64(r15.wrapping_add(0xd0), (rax) as u64);
            // 203ce mov QWORD PTR [r14+0xd0],0x0
            m.w64(r14.wrapping_add(0xd0), (0x0u64) as u64);
            // 203d9 mov QWORD PTR [r14+0xc8],0x0
            m.w64(r14.wrapping_add(0xc8), (0x0u64) as u64);
            // 203e4 mov QWORD PTR [r14+0xc0],0x0
            m.w64(r14.wrapping_add(0xc0), (0x0u64) as u64);
            // 203ef mov eax,DWORD PTR [r14+0xf0]
            rax = (m.r32(r14.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 203f6 mov rdx,QWORD PTR [rbp-0x148]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 203fd mov DWORD PTR [rdx+0x18],eax
            m.w32(rdx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 20400 mov rax,QWORD PTR [r14+0xe8]
            rax = m.r64(r14.wrapping_add(0xe8)) as u64;
            // 20407 mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 2040b mov rax,QWORD PTR [r14+0xd8]
            rax = m.r64(r14.wrapping_add(0xd8)) as u64;
            // 20412 mov rcx,QWORD PTR [r14+0xe0]
            rcx = m.r64(r14.wrapping_add(0xe0)) as u64;
            // 20419 mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 2041d mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 20420 mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 20427 mov QWORD PTR [r14+0x10],rax
            m.w64(r14.wrapping_add(0x10), (rax) as u64);
            // 2042b mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 20432 mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 20439 mov QWORD PTR [r14+0x8],rcx
            m.w64(r14.wrapping_add(0x8), (rcx) as u64);
            // 2043d mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 20440 test BYTE PTR [r14+0x18],0x1
            fl = logic::<8>(m.r8(r14.wrapping_add(0x18)) as u64 & 0x1u64);
            // 20445 jne 0x206e2
            if crate::cov::brk(0x20445, fl.ne()) { pc = 0x206e2; continue; }
            pc = 0x2044b;
        }
        0x2044b => {
            // block 2044b
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 2044b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2044b mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // 20450 jmp 0x206f1
            pc = 0x206f1; continue;
        }
        0x20455 => {
            // block 20455
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 20455 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 20455 mov rax,QWORD PTR [r12+0x28]
            rax = m.r64(r12.wrapping_add(0x28)) as u64;
            // 2045a mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 2045d mov QWORD PTR [r12+0x20],0x0
            m.w64(r12.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x20466;
        }
        0x20466 => {
            // block 20466
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 20466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 20466 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 20468 mov rdi,r14
            rdi = r14;
            // 2046b call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20470 lea rax,[r12+0x30]
            rax = r12.wrapping_add(0x30);
            // 20475 lea rdi,[r12+0x60]
            rdi = r12.wrapping_add(0x60);
            // 2047a lea rcx,[r12+0x78]
            rcx = r12.wrapping_add(0x78);
            // 2047f mov QWORD PTR [rbp-0x168],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rcx) as u64);
            // 20486 lea rcx,[r12+0x90]
            rcx = r12.wrapping_add(0x90);
            // 2048e mov QWORD PTR [rbp-0x160],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rcx) as u64);
            // 20495 lea rcx,[r12+0xa8]
            rcx = r12.wrapping_add(0xa8);
            // 2049d mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // 204a4 lea rcx,[r12+0xc0]
            rcx = r12.wrapping_add(0xc0);
            // 204ac mov QWORD PTR [rbp-0x128],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rcx) as u64);
            // 204b3 lea r13,[r12+0xd8]
            r13 = r12.wrapping_add(0xd8);
            // 204bb mov rbx,QWORD PTR [rbp-0x178]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 204c2 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 204c6 mov QWORD PTR [r14+0x10],rcx
            m.w64(r14.wrapping_add(0x10), (rcx) as u64);
            // 204ca mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 204cd mov rdx,QWORD PTR [rbx+0x8]
            rdx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 204d1 mov QWORD PTR [r14+0x8],rdx
            m.w64(r14.wrapping_add(0x8), (rdx) as u64);
            // 204d5 mov QWORD PTR [r14],rcx
            m.w64(r14, (rcx) as u64);
            // 204d8 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 204e0 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 204e8 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 204ef mov r14,QWORD PTR [rbp-0x180]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 204f6 mov cx,WORD PTR [r14+0x2c]
            rcx = (rcx & !0xffff) | ((m.r16(r14.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 204fb mov WORD PTR [rax+0x2c],cx
            m.w16(rax.wrapping_add(0x2c), ((rcx as u16 as u64)) as u16);
            // 204ff mov ecx,DWORD PTR [r14+0x28]
            rcx = (m.r32(r14.wrapping_add(0x28)) as u64) as u32 as u64;
            // 20503 mov DWORD PTR [rax+0x28],ecx
            m.w32(rax.wrapping_add(0x28), ((rcx as u32 as u64)) as u32);
            // 20506 mov rcx,QWORD PTR [r14+0x20]
            rcx = m.r64(r14.wrapping_add(0x20)) as u64;
            // 2050a mov QWORD PTR [rax+0x20],rcx
            m.w64(rax.wrapping_add(0x20), (rcx) as u64);
            // 2050e mov rcx,QWORD PTR [r14+0x18]
            rcx = m.r64(r14.wrapping_add(0x18)) as u64;
            // 20512 mov QWORD PTR [rax+0x18],rcx
            m.w64(rax.wrapping_add(0x18), (rcx) as u64);
            // 20516 mov rcx,QWORD PTR [r14+0x10]
            rcx = m.r64(r14.wrapping_add(0x10)) as u64;
            // 2051a mov QWORD PTR [rax+0x10],rcx
            m.w64(rax.wrapping_add(0x10), (rcx) as u64);
            // 2051e mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // 20521 mov rdx,QWORD PTR [r14+0x8]
            rdx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 20525 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 20529 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 2052c call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20531 movups xmm0,XMMWORD PTR [r15+0x60]
            x0 = m.r128(r15.wrapping_add(0x60));
            // 20536 movups XMMWORD PTR [r12+0x60],xmm0
            m.w128(r12.wrapping_add(0x60), x0);
            // 2053c mov rax,QWORD PTR [r15+0x70]
            rax = m.r64(r15.wrapping_add(0x70)) as u64;
            // 20540 mov QWORD PTR [r12+0x70],rax
            m.w64(r12.wrapping_add(0x70), (rax) as u64);
            // 20545 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 2054c mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 20554 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2055c mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 20563 mov rdi,QWORD PTR [rbp-0x168]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 2056a call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2056f movups xmm0,XMMWORD PTR [r15+0x78]
            x0 = m.r128(r15.wrapping_add(0x78));
            // 20574 movups XMMWORD PTR [r12+0x78],xmm0
            m.w128(r12.wrapping_add(0x78), x0);
            // 2057a mov rax,QWORD PTR [r15+0x88]
            rax = m.r64(r15.wrapping_add(0x88)) as u64;
            // 20581 mov QWORD PTR [r12+0x88],rax
            m.w64(r12.wrapping_add(0x88), (rax) as u64);
            // 20589 mov rax,QWORD PTR [rbp-0x148]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 20590 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 20598 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 205a0 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 205a7 mov rdi,QWORD PTR [rbp-0x160]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 205ae call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 205b3 movups xmm0,XMMWORD PTR [r15+0x90]
            x0 = m.r128(r15.wrapping_add(0x90));
            // 205bb movups XMMWORD PTR [r12+0x90],xmm0
            m.w128(r12.wrapping_add(0x90), x0);
            // 205c4 mov rax,QWORD PTR [r15+0xa0]
            rax = m.r64(r15.wrapping_add(0xa0)) as u64;
            // 205cb mov QWORD PTR [r12+0xa0],rax
            m.w64(r12.wrapping_add(0xa0), (rax) as u64);
            // 205d3 mov rax,QWORD PTR [rbp-0x140]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 205da mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 205e2 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 205ea mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 205f1 mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 205f8 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 205fd movups xmm0,XMMWORD PTR [r15+0xa8]
            x0 = m.r128(r15.wrapping_add(0xa8));
            // 20605 movups XMMWORD PTR [r12+0xa8],xmm0
            m.w128(r12.wrapping_add(0xa8), x0);
            // 2060e mov rax,QWORD PTR [r15+0xb8]
            rax = m.r64(r15.wrapping_add(0xb8)) as u64;
            // 20615 mov QWORD PTR [r12+0xb8],rax
            m.w64(r12.wrapping_add(0xb8), (rax) as u64);
            // 2061d mov rax,QWORD PTR [rbp-0x138]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 20624 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 2062c mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 20634 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 2063b mov rdi,QWORD PTR [rbp-0x128]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 20642 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20647 movups xmm0,XMMWORD PTR [r15+0xc0]
            x0 = m.r128(r15.wrapping_add(0xc0));
            // 2064f movups XMMWORD PTR [r12+0xc0],xmm0
            m.w128(r12.wrapping_add(0xc0), x0);
            // 20658 mov rax,QWORD PTR [r15+0xd0]
            rax = m.r64(r15.wrapping_add(0xd0)) as u64;
            // 2065f mov QWORD PTR [r12+0xd0],rax
            m.w64(r12.wrapping_add(0xd0), (rax) as u64);
            // 20667 mov r12,QWORD PTR [rbp-0x150]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 2066e mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 20677 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 20680 mov QWORD PTR [r12],0x0
            m.w64(r12, (0x0u64) as u64);
            // 20688 mov rcx,QWORD PTR [rbp-0x130]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 2068f mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 20692 mov DWORD PTR [r13+0x18],eax
            m.w32(r13.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 20696 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 2069a mov QWORD PTR [r13+0x10],rax
            m.w64(r13.wrapping_add(0x10), (rax) as u64);
            // 2069e mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 206a1 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 206a5 mov QWORD PTR [r13+0x8],rcx
            m.w64(r13.wrapping_add(0x8), (rcx) as u64);
            // 206a9 mov QWORD PTR [r13+0x0],rax
            m.w64(r13, (rax) as u64);
            // 206ad mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 206b4 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 206b8 mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 206bf mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 206c6 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 206ca mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 206cd test BYTE PTR [r15+0x18],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x18)) as u64 & 0x1u64);
            // 206d2 jne 0x20932
            if crate::cov::brk(0x206d2, fl.ne()) { pc = 0x20932; continue; }
            pc = 0x206d8;
        }
        0x206d8 => {
            // block 206d8
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 206d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 206d8 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // 206dd jmp 0x20941
            pc = 0x20941; continue;
        }
        0x206e2 => {
            // block 206e2
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 206e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 206e2 mov rax,QWORD PTR [r14+0x28]
            rax = m.r64(r14.wrapping_add(0x28)) as u64;
            // 206e6 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 206e9 mov QWORD PTR [r14+0x20],0x0
            m.w64(r14.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x206f1;
        }
        0x206f1 => {
            // block 206f1
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 206f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 206f1 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 206f3 mov rdi,rbx
            rdi = rbx;
            // 206f6 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 206fb lea rax,[r14+0x30]
            rax = r14.wrapping_add(0x30);
            // 206ff lea rdi,[r14+0x60]
            rdi = r14.wrapping_add(0x60);
            // 20703 lea r15,[r14+0x78]
            r15 = r14.wrapping_add(0x78);
            // 20707 lea rcx,[r14+0x90]
            rcx = r14.wrapping_add(0x90);
            // 2070e mov QWORD PTR [rbp-0x138],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rcx) as u64);
            // 20715 lea rcx,[r14+0xa8]
            rcx = r14.wrapping_add(0xa8);
            // 2071c mov QWORD PTR [rbp-0x130],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (rcx) as u64);
            // 20723 lea rcx,[r14+0xc0]
            rcx = r14.wrapping_add(0xc0);
            // 2072a mov QWORD PTR [rbp-0x128],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rcx) as u64);
            // 20731 lea r12,[r14+0xd8]
            r12 = r14.wrapping_add(0xd8);
            // 20738 lea r13,[rbp-0x108]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 2073f mov rcx,QWORD PTR [r13+0x10]
            rcx = m.r64(r13.wrapping_add(0x10)) as u64;
            // 20743 mov QWORD PTR [rbx+0x10],rcx
            m.w64(rbx.wrapping_add(0x10), (rcx) as u64);
            // 20747 mov rcx,QWORD PTR [r13+0x0]
            rcx = m.r64(r13) as u64;
            // 2074b mov rdx,QWORD PTR [r13+0x8]
            rdx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 2074f mov QWORD PTR [rbx+0x8],rdx
            m.w64(rbx.wrapping_add(0x8), (rdx) as u64);
            // 20753 mov QWORD PTR [rbx],rcx
            m.w64(rbx, (rcx) as u64);
            // 20756 mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // 2075e mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // 20766 mov QWORD PTR [r13+0x0],0x0
            m.w64(r13, (0x0u64) as u64);
            // 2076e lea rcx,[rbp-0xf0]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 20775 mov rdx,rcx
            rdx = rcx;
            // 20778 mov cx,WORD PTR [rdx+0x2c]
            rcx = (rcx & !0xffff) | ((m.r16(rdx.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 2077c mov WORD PTR [rax+0x2c],cx
            m.w16(rax.wrapping_add(0x2c), ((rcx as u16 as u64)) as u16);
            // 20780 mov ecx,DWORD PTR [rdx+0x28]
            rcx = (m.r32(rdx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 20783 mov DWORD PTR [rax+0x28],ecx
            m.w32(rax.wrapping_add(0x28), ((rcx as u32 as u64)) as u32);
            // 20786 mov rcx,QWORD PTR [rdx+0x20]
            rcx = m.r64(rdx.wrapping_add(0x20)) as u64;
            // 2078a mov QWORD PTR [rax+0x20],rcx
            m.w64(rax.wrapping_add(0x20), (rcx) as u64);
            // 2078e mov rcx,QWORD PTR [rdx+0x18]
            rcx = m.r64(rdx.wrapping_add(0x18)) as u64;
            // 20792 mov QWORD PTR [rax+0x18],rcx
            m.w64(rax.wrapping_add(0x18), (rcx) as u64);
            // 20796 mov rcx,QWORD PTR [rdx+0x10]
            rcx = m.r64(rdx.wrapping_add(0x10)) as u64;
            // 2079a mov QWORD PTR [rax+0x10],rcx
            m.w64(rax.wrapping_add(0x10), (rcx) as u64);
            // 2079e mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            // 207a1 mov rdx,QWORD PTR [rdx+0x8]
            rdx = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 207a5 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 207a9 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 207ac call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 207b1 movups xmm0,XMMWORD PTR [rbp-0xc0]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff40));
            // 207b8 movups XMMWORD PTR [r14+0x60],xmm0
            m.w128(r14.wrapping_add(0x60), x0);
            // 207bd mov rax,QWORD PTR [rbp-0xb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 207c4 mov QWORD PTR [r14+0x70],rax
            m.w64(r14.wrapping_add(0x70), (rax) as u64);
            // 207c8 lea rax,[rbp-0xc0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 207cf mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 207d7 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 207df mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 207e6 mov rdi,r15
            rdi = r15;
            // 207e9 call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 207ee movups xmm0,XMMWORD PTR [rbp-0xa8]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff58));
            // 207f5 movups XMMWORD PTR [r14+0x78],xmm0
            m.w128(r14.wrapping_add(0x78), x0);
            // 207fa mov rax,QWORD PTR [rbp-0x98]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 20801 mov QWORD PTR [r14+0x88],rax
            m.w64(r14.wrapping_add(0x88), (rax) as u64);
            // 20808 lea rax,[rbp-0xa8]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 2080f mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 20817 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2081f mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 20826 mov rdi,QWORD PTR [rbp-0x138]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 2082d call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20832 lea r15,[rbp-0x90]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 20839 movups xmm0,XMMWORD PTR [rbp-0x90]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff70));
            // 20840 movups XMMWORD PTR [r14+0x90],xmm0
            m.w128(r14.wrapping_add(0x90), x0);
            // 20848 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 2084c mov QWORD PTR [r14+0xa0],rax
            m.w64(r14.wrapping_add(0xa0), (rax) as u64);
            // 20853 lea rax,[rbp-0x90]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 2085a mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 20862 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2086a mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 20871 mov rdi,QWORD PTR [rbp-0x130]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 20878 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2087d lea rbx,[rbp-0x78]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 20881 movups xmm0,XMMWORD PTR [rbp-0x78]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff88));
            // 20885 movups XMMWORD PTR [r14+0xa8],xmm0
            m.w128(r14.wrapping_add(0xa8), x0);
            // 2088d mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 20891 mov QWORD PTR [r14+0xb8],rax
            m.w64(r14.wrapping_add(0xb8), (rax) as u64);
            // 20898 lea rax,[rbp-0x78]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 2089c mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 208a4 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 208ac mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 208b3 mov rdi,QWORD PTR [rbp-0x128]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 208ba call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 208bf lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 208c3 movups xmm0,XMMWORD PTR [rbp-0x60]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0));
            // 208c7 movups XMMWORD PTR [r14+0xc0],xmm0
            m.w128(r14.wrapping_add(0xc0), x0);
            // 208cf mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 208d3 mov QWORD PTR [r14+0xd0],rax
            m.w64(r14.wrapping_add(0xd0), (rax) as u64);
            // 208da lea rax,[rbp-0x60]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 208de mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 208e6 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 208ee mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 208f5 lea rax,[rbp-0x48]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 208f9 mov rcx,rax
            rcx = rax;
            // 208fc mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 208ff mov DWORD PTR [r12+0x18],eax
            m.w32(r12.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 20904 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 20908 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 2090d mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 20910 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 20914 mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 20919 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 2091d call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20922 mov rdi,rbx
            rdi = rbx;
            // 20925 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2092a mov rdi,r15
            rdi = r15;
            // 2092d jmp 0x20b49
            pc = 0x20b49; continue;
        }
        0x20932 => {
            // block 20932
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 20932 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 20932 mov rax,QWORD PTR [r15+0x28]
            rax = m.r64(r15.wrapping_add(0x28)) as u64;
            // 20936 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 20939 mov QWORD PTR [r15+0x20],0x0
            m.w64(r15.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x20941;
        }
        0x20941 => {
            // block 20941
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 20941 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 20941 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 20943 mov rdi,rbx
            rdi = rbx;
            // 20946 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2094b lea r13,[rbp-0x108]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 20952 mov rax,QWORD PTR [r13+0x10]
            rax = m.r64(r13.wrapping_add(0x10)) as u64;
            // 20956 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 2095a mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 2095e mov rcx,QWORD PTR [r13+0x8]
            rcx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 20962 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 20966 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 20969 mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // 20971 mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // 20979 mov QWORD PTR [r13+0x0],0x0
            m.w64(r13, (0x0u64) as u64);
            // 20981 lea rax,[rbp-0xf0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 20988 mov rcx,rax
            rcx = rax;
            // 2098b mov ax,WORD PTR [rcx+0x2c]
            rax = (rax & !0xffff) | ((m.r16(rcx.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 2098f mov WORD PTR [r14+0x2c],ax
            m.w16(r14.wrapping_add(0x2c), ((rax as u16 as u64)) as u16);
            // 20994 mov eax,DWORD PTR [rcx+0x28]
            rax = (m.r32(rcx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 20997 mov DWORD PTR [r14+0x28],eax
            m.w32(r14.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 2099b mov rax,QWORD PTR [rcx+0x20]
            rax = m.r64(rcx.wrapping_add(0x20)) as u64;
            // 2099f mov QWORD PTR [r14+0x20],rax
            m.w64(r14.wrapping_add(0x20), (rax) as u64);
            // 209a3 mov rax,QWORD PTR [rcx+0x18]
            rax = m.r64(rcx.wrapping_add(0x18)) as u64;
            // 209a7 mov QWORD PTR [r14+0x18],rax
            m.w64(r14.wrapping_add(0x18), (rax) as u64);
            // 209ab mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 209af mov QWORD PTR [r14+0x10],rax
            m.w64(r14.wrapping_add(0x10), (rax) as u64);
            // 209b3 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 209b6 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 209ba mov QWORD PTR [r14+0x8],rcx
            m.w64(r14.wrapping_add(0x8), (rcx) as u64);
            // 209be mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 209c1 mov rdi,QWORD PTR [rbp-0x170]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 209c8 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 209cd movups xmm0,XMMWORD PTR [rbp-0xc0]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff40));
            // 209d4 movups XMMWORD PTR [r15+0x60],xmm0
            m.w128(r15.wrapping_add(0x60), x0);
            // 209d9 mov rax,QWORD PTR [rbp-0xb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 209e0 mov QWORD PTR [r15+0x70],rax
            m.w64(r15.wrapping_add(0x70), (rax) as u64);
            // 209e4 lea rax,[rbp-0xc0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 209eb mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 209f3 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 209fb mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 20a02 mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 20a09 call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20a0e movups xmm0,XMMWORD PTR [rbp-0xa8]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff58));
            // 20a15 movups XMMWORD PTR [r15+0x78],xmm0
            m.w128(r15.wrapping_add(0x78), x0);
            // 20a1a mov rax,QWORD PTR [rbp-0x98]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 20a21 mov QWORD PTR [r15+0x88],rax
            m.w64(r15.wrapping_add(0x88), (rax) as u64);
            // 20a28 lea rax,[rbp-0xa8]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 20a2f mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 20a37 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 20a3f mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 20a46 mov rdi,QWORD PTR [rbp-0x140]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 20a4d call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20a52 movups xmm0,XMMWORD PTR [rbp-0x90]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff70));
            // 20a59 movups XMMWORD PTR [r15+0x90],xmm0
            m.w128(r15.wrapping_add(0x90), x0);
            // 20a61 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 20a65 mov QWORD PTR [r15+0xa0],rax
            m.w64(r15.wrapping_add(0xa0), (rax) as u64);
            // 20a6c lea rax,[rbp-0x90]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 20a73 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 20a7b mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 20a83 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 20a8a mov rdi,QWORD PTR [rbp-0x138]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 20a91 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20a96 lea rbx,[rbp-0x78]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 20a9a movups xmm0,XMMWORD PTR [rbp-0x78]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff88));
            // 20a9e movups XMMWORD PTR [r15+0xa8],xmm0
            m.w128(r15.wrapping_add(0xa8), x0);
            // 20aa6 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 20aaa mov QWORD PTR [r15+0xb8],rax
            m.w64(r15.wrapping_add(0xb8), (rax) as u64);
            // 20ab1 lea rax,[rbp-0x78]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 20ab5 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 20abd mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 20ac5 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 20acc mov rdi,r12
            rdi = r12;
            // 20acf call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20ad4 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 20ad8 movups xmm0,XMMWORD PTR [rbp-0x60]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0));
            // 20adc movups XMMWORD PTR [r15+0xc0],xmm0
            m.w128(r15.wrapping_add(0xc0), x0);
            // 20ae4 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 20ae8 mov QWORD PTR [r15+0xd0],rax
            m.w64(r15.wrapping_add(0xd0), (rax) as u64);
            // 20aef lea rax,[rbp-0x60]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 20af3 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 20afb mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 20b03 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 20b0a lea rax,[rbp-0x48]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 20b0e mov rcx,rax
            rcx = rax;
            // 20b11 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 20b14 mov r14,QWORD PTR [rbp-0x130]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 20b1b mov DWORD PTR [r14+0x18],eax
            m.w32(r14.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 20b1f mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 20b23 mov QWORD PTR [r14+0x10],rax
            m.w64(r14.wrapping_add(0x10), (rax) as u64);
            // 20b27 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 20b2a mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 20b2e mov QWORD PTR [r14+0x8],rcx
            m.w64(r14.wrapping_add(0x8), (rcx) as u64);
            // 20b32 mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 20b35 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20b3a mov rdi,rbx
            rdi = rbx;
            // 20b3d call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20b42 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            pc = 0x20b49;
        }
        0x20b49 => {
            // block 20b49
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 20b49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 20b49 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20b4e lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 20b55 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20b5a lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 20b61 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20b66 mov rdi,r13
            rdi = r13;
            // 20b69 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20b6e mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            pc = 0x20b73;
        }
        0x20b73 => {
            // block 20b73
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 20b73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 20b73 add rsp,0x158
            { let (r, f) = add::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // 20b7a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 20b7b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 20b7d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 20b7f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 20b81 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 20b83 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 20b84 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x20b85 => {
            // block 20b85
            #[cfg(feature = "trace")] eprintln!("f_0001e68c pc 20b85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 20b85 mov rdi,rax
            rdi = rax;
            // 20b88 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20b8d mov rdi,rax
            rdi = rax;
            // 20b90 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20b95 mov rdi,rax
            rdi = rax;
            // 20b98 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20b9d mov rdi,rax
            rdi = rax;
            // 20ba0 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20ba5 mov rdi,rax
            rdi = rax;
            // 20ba8 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20bad mov rdi,rax
            rdi = rax;
            // 20bb0 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20bb5 mov rdi,rax
            rdi = rax;
            // 20bb8 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20bbd mov rdi,rax
            rdi = rax;
            // 20bc0 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20bc5 mov rdi,rax
            rdi = rax;
            // 20bc8 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 20bcd mov rdi,rax
            rdi = rax;
            // 20bd0 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
