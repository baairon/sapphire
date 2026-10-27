//! Mechanically lifted functions (scripts/lift_run.py), module s001. Each one is checked against the original
//! in the world harness (tests/world_s001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x4b6de size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_0004b6de<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4b6de;
    loop {
        match pc {
        0x4b6de => {
            // block 4b6de
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b6de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b6de push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4b6df mov rbp,rsp
            rbp = rsp;
            // 4b6e2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4b6e4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4b6e6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4b6e7 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 4b6eb mov rbx,rdi
            rbx = rdi;
            // 4b6ee mov r15,QWORD PTR [rip+0xf4ce3] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 4b6f5 mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 4b6f8 mov QWORD PTR [rbp-0x20],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (r15) as u64);
            // 4b6fc mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 4b704 mov DWORD PTR [rbp-0x34],0x1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), (0x1u64) as u32);
            // 4b70b movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // 4b70e mov QWORD PTR [rbp-0x28],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rax) as u64);
            // 4b712 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4b715 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 4b718 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4b71a call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b720 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4b722 je 0x4b758
            if fl.e() { pc = 0x4b758; continue; }
            pc = 0x4b724;
        }
        0x4b724 => {
            // block 4b724
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b724 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b724 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4b727 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4b729 mov rdi,rbx
            rdi = rbx;
            // 4b72c call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b732 lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 4b736 lea r8,[rbp-0x30]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 4b73a lea r9,[rbp-0x34]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 4b73e mov esi,0x1a
            rsi = (0x1au64) as u32 as u64;
            // 4b743 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 4b748 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4b74a call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b74f cmp QWORD PTR [rbp-0x30],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, 0x0u64).1;
            // 4b754 setne r14b
            r14 = (r14 & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0x4b758;
        }
        0x4b758 => {
            // block 4b758
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b758 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b758 cmp r15,QWORD PTR [rbp-0x20]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // 4b75c jne 0x4b76c
            if fl.ne() { pc = 0x4b76c; continue; }
            pc = 0x4b75e;
        }
        0x4b75e => {
            // block 4b75e
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b75e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b75e mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 4b761 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 4b765 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4b766 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4b768 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4b76a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4b76b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4b76c => {
            // block 4b76c
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b76c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b76c call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4aaf4 size=1528
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_0004aaf4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4aaf4;
    loop {
        match pc {
        0x4aaf4 => {
            // block 4aaf4
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4aaf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aaf4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4aaf5 mov rbp,rsp
            rbp = rsp;
            // 4aaf8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4aafa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4aafc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4aafe push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4ab00 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4ab01 sub rsp,0x188
            { let (r, f) = sub::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 4ab08 mov r14,rdi
            r14 = rdi;
            // 4ab0b mov rbx,QWORD PTR [rip+0xf58c6] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4ab12 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4ab15 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 4ab19 mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 4ab20 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 4ab22 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 4ab26 movsxd rax,edx
            rax = sx::<32>((rdx as u32 as u64)) as u64;
            // 4ab29 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 4ab2d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4ab30 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4ab32 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ab38 mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 4ab3e test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4ab40 je 0x4ab7a
            if fl.e() { pc = 0x4ab7a; continue; }
            pc = 0x4ab42;
        }
        0x4ab42 => {
            // block 4ab42
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4ab42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ab42 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4ab45 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4ab47 mov rdi,r14
            rdi = r14;
            // 4ab4a call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ab50 lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4ab54 lea r9,[rbp-0x64]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 4ab58 mov esi,0x6
            rsi = (0x6u64) as u32 as u64;
            // 4ab5d mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 4ab62 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4ab65 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4ab67 call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ab6c mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 4ab6f mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4ab71 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 4ab74 je 0x4b02a
            if fl.e() { pc = 0x4b02a; continue; }
            pc = 0x4ab7a;
        }
        0x4ab7a => {
            // block 4ab7a
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4ab7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ab7a test BYTE PTR [rip+0x17d70f],0x40 # 0x1c8290
            fl = logic::<8>(m.r8(m.env.data_va(903773)) as u64 & 0x40u64);
            // 4ab81 je 0x4b028
            if fl.e() { pc = 0x4b028; continue; }
            pc = 0x4ab87;
        }
        0x4ab87 => {
            // block 4ab87
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4ab87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ab87 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4ab8e lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4ab95 mov rax,QWORD PTR [rip+0xf5634] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 4ab9c lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4aba0 mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 4aba7 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4abab mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 4abb2 mov rsi,rbx
            rsi = rbx;
            // 4abb5 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4abba mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 4abc2 mov DWORD PTR [rbp-0x70],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0xffffffffu64) as u32);
            // 4abc9 mov r12,QWORD PTR [rip+0xf57f0] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 4abd0 lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 4abd5 mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 4abdc mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4abe3 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 4abe7 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4abee mov rdi,rbx
            rdi = rbx;
            // 4abf1 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4abf6 mov rax,QWORD PTR [rip+0xf57ab] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4abfd add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 4ac01 mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            // 4ac08 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4ac0f mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 4ac1a mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 4ac25 mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 4ac30 mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 4ac3b mov DWORD PTR [rbp-0x108],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x10u64) as u32);
            // 4ac45 xorps xmm0,xmm0
            x0 = 0;
            // 4ac48 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 4ac4c mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 4ac54 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4ac58 mov rdi,rbx
            rdi = rbx;
            // 4ac5b call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ac60 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4ac64 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ac69 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ac6e mov r15,rax
            r15 = rax;
            // 4ac71 lea rsi,[rip+0xe0049] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4ac78 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4ac7f mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4ac84 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ac89 lea rsi,[rip+0xe0035] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 4ac90 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 4ac95 mov rdi,rax
            rdi = rax;
            // 4ac98 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ac9d lea rsi,[rip+0xdfcdc] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4aca4 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4aca9 mov rdi,rax
            rdi = rax;
            // 4acac call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4acb1 lea rsi,[rip+0xdf7bd] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4acb8 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4acbd mov rdi,rax
            rdi = rax;
            // 4acc0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4acc5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4acc8 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4accc mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4acd0 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4acd3 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4acd6 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4acda mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4acdd mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ace1 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4acea mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4aced mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4acf1 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4acfc mov rdi,rax
            rdi = rax;
            // 4acff mov rsi,r15
            rsi = r15;
            // 4ad02 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ad07 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ad0a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ad0e mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4ad12 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ad15 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ad18 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4ad1c lea rsi,[rip+0xdfc5d] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4ad23 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4ad28 mov rdi,rax
            rdi = rax;
            // 4ad2b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ad30 mov rbx,rax
            rbx = rax;
            // 4ad33 lea rsi,[rip+0xe013a] # 0x12ae74
            rsi = m.env.data_va(259649);
            // 4ad3a lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4ad41 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 4ad46 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ad4b movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 4ad52 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 4ad55 jne 0x4ad63
            if fl.ne() { pc = 0x4ad63; continue; }
            pc = 0x4ad57;
        }
        0x4ad57 => {
            // block 4ad57
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4ad57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ad57 lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // 4ad5e shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 4ad61 jmp 0x4ad71
            pc = 0x4ad71; continue;
        }
        0x4ad63 => {
            // block 4ad63
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4ad63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ad63 mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 4ad6a mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x4ad71;
        }
        0x4ad71 => {
            // block 4ad71
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4ad71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ad71 mov rdi,rbx
            rdi = rbx;
            // 4ad74 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ad79 lea rsi,[rip+0xdfc00] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4ad80 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4ad85 mov rdi,rax
            rdi = rax;
            // 4ad88 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ad8d lea rsi,[rip+0xdffdc] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 4ad94 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4ad99 mov rdi,rax
            rdi = rax;
            // 4ad9c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ada1 lea rsi,[rip+0xdf6cd] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4ada8 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4adad mov rdi,rax
            rdi = rax;
            // 4adb0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4adb5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4adb8 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4adbc mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4adc0 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4adc3 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4adc6 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4adca mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4adcd mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4add1 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4adda mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4addd mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ade1 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4adec mov rdi,rax
            rdi = rax;
            // 4adef mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4adf2 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4adf7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4adfa mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4adfe mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4ae02 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ae05 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ae08 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4ae0c lea rsi,[rip+0xdfb73] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4ae13 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4ae18 mov rdi,rax
            rdi = rax;
            // 4ae1b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ae20 mov r15,rax
            r15 = rax;
            // 4ae23 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 4ae26 call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ae2b mov rbx,rax
            rbx = rax;
            // 4ae2e mov rdi,rbx
            rdi = rbx;
            // 4ae31 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ae36 mov rdi,r15
            rdi = r15;
            // 4ae39 mov rsi,rbx
            rsi = rbx;
            // 4ae3c mov rdx,rax
            rdx = rax;
            // 4ae3f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ae44 lea rsi,[rip+0xdf837] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4ae4b mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4ae50 mov rdi,rax
            rdi = rax;
            // 4ae53 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ae58 lea rsi,[rip+0xdfe9b] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 4ae5f mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4ae64 mov rdi,rax
            rdi = rax;
            // 4ae67 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ae6c lea rsi,[rip+0xdf602] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4ae73 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4ae78 mov rdi,rax
            rdi = rax;
            // 4ae7b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ae80 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ae83 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ae87 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4ae8b and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ae8e or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ae91 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4ae95 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4ae98 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4ae9c mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4aea5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4aea8 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4aeac mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4aeb7 mov rdi,rax
            rdi = rax;
            // 4aeba mov rsi,r14
            rsi = r14;
            // 4aebd call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4aec2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4aec5 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4aec9 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4aecd and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4aed0 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4aed3 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4aed7 lea rsi,[rip+0xdfe2e] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 4aede mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4aee3 mov rdi,rax
            rdi = rax;
            // 4aee6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4aeeb lea rsi,[rip+0xdf583] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4aef2 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4aef7 mov rdi,rax
            rdi = rax;
            // 4aefa call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4aeff mov rbx,rax
            rbx = rax;
            // 4af02 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4af05 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4af09 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4af0d and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4af10 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4af13 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 4af17 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4af1a mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4af1e mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 4af27 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4af2a mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4af2e mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 4af39 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4af3c mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 4af43 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4af45 mov rdi,r14
            rdi = r14;
            // 4af48 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4af4a mov rdi,rbx
            rdi = rbx;
            // 4af4d mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 4af4f call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4af54 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4af57 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4af5b mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4af5f and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4af62 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4af65 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4af69 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4af70 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4af75 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4af7c lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4af83 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4af88 test BYTE PTR [rbp-0x1a0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64 & 0x1u64);
            // 4af8f jne 0x4af9a
            if fl.ne() { pc = 0x4af9a; continue; }
            pc = 0x4af91;
        }
        0x4af91 => {
            // block 4af91
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4af91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4af91 lea rdx,[rbp-0x19f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe61);
            // 4af98 jmp 0x4afa1
            pc = 0x4afa1; continue;
        }
        0x4af9a => {
            // block 4af9a
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4af9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4af9a mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            pc = 0x4afa1;
        }
        0x4afa1 => {
            // block 4afa1
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4afa1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4afa1 mov rbx,QWORD PTR [rip+0xf5430] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4afa8 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4afab lea rsi,[rip+0xdfd67] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 4afb2 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 4afb7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4afb9 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4afbe lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4afc5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4afca mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4afd1 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4afd8 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4afdf mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 4afe6 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4afed lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4aff4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4aff9 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4b000 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b005 mov rsi,QWORD PTR [rip+0xf51fc] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4b00c add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4b010 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4b017 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b01c lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4b023 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4b028;
        }
        0x4b028 => {
            // block 4b028
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b028 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b028 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4b02a;
        }
        0x4b02a => {
            // block 4b02a
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b02a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b02a cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 4b02e jne 0x4b0c0
            if fl.ne() { pc = 0x4b0c0; continue; }
            pc = 0x4b034;
        }
        0x4b034 => {
            // block 4b034
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b034 add rsp,0x188
            { let (r, f) = add::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 4b03b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4b03c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4b03e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4b040 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4b042 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4b044 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4b045 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4b046 => {
            // block 4b046
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b046 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b046 mov rbx,rax
            rbx = rax;
            // 4b049 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            pc = 0x4b050;
        }
        0x4b050 => {
            // block 4b050
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b050 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b050 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b055 jmp 0x4b05a
            pc = 0x4b05a; continue;
        }
        0x4b057 => {
            // block 4b057
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b057 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b057 mov rbx,rax
            rbx = rax;
            pc = 0x4b05a;
        }
        0x4b05a => {
            // block 4b05a
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b05a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b05a mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4b061 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4b068 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4b06f mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 4b076 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            pc = 0x4b07d;
        }
        0x4b07d => {
            // block 4b07d
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b07d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b07d lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4b084 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b089 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4b090 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4b095;
        }
        0x4b095 => {
            // block 4b095
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b095 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b095 mov rsi,QWORD PTR [rip+0xf516c] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4b09c add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4b0a0 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4b0a7 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4b0ac;
        }
        0x4b0ac => {
            // block 4b0ac
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b0ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b0ac lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4b0b3 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b0b8 mov rdi,rbx
            rdi = rbx;
            // 4b0bb call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4b0c0;
        }
        0x4b0c0 => {
            // block 4b0c0
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b0c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b0c0 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x4b0ca => {
            // block 4b0ca
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b0ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b0ca mov rbx,rax
            rbx = rax;
            // 4b0cd jmp 0x4b095
            pc = 0x4b095; continue;
        }
        0x4b0cf => {
            // block 4b0cf
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b0cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b0cf mov rbx,rax
            rbx = rax;
            // 4b0d2 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4b0d6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b0db jmp 0x4b07d
            pc = 0x4b07d; continue;
        }
        0x4b0dd => {
            // block 4b0dd
            #[cfg(feature = "trace")] eprintln!("f_0004aaf4 pc 4b0dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b0dd mov rbx,rax
            rbx = rax;
            // 4b0e0 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4b0e7 jmp 0x4b050
            pc = 0x4b050; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4d55a size=1546
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_0004d55a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4d55a;
    loop {
        match pc {
        0x4d55a => {
            // block 4d55a
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4d55a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d55a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4d55b mov rbp,rsp
            rbp = rsp;
            // 4d55e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4d560 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4d562 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4d564 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4d566 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4d567 sub rsp,0x188
            { let (r, f) = sub::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 4d56e mov rbx,rdx
            rbx = rdx;
            // 4d571 mov r14,rdi
            r14 = rdi;
            // 4d574 mov r15,QWORD PTR [rip+0xf2e5d] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 4d57b mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 4d57e mov QWORD PTR [rbp-0x30],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r15) as u64);
            // 4d582 mov QWORD PTR [rbp-0x58],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0x0u64) as u64);
            // 4d58a mov DWORD PTR [rbp-0x5c],0x1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), (0x1u64) as u32);
            // 4d591 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 4d593 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 4d597 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4d59a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4d59c call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d5a2 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4d5a4 je 0x4d5e5
            if fl.e() { pc = 0x4d5e5; continue; }
            pc = 0x4d5a6;
        }
        0x4d5a6 => {
            // block 4d5a6
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4d5a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d5a6 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4d5a9 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4d5ab mov rdi,r14
            rdi = r14;
            // 4d5ae call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d5b4 lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 4d5b8 lea r8,[rbp-0x58]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 4d5bc lea r9,[rbp-0x5c]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffa4);
            // 4d5c0 mov esi,0xc
            rsi = (0xcu64) as u32 as u64;
            // 4d5c5 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 4d5ca mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4d5cc call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d5d1 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 4d5d4 mov eax,DWORD PTR [rbp-0x58]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // 4d5d7 mov DWORD PTR [rbx],eax
            m.w32(rbx, ((rax as u32 as u64)) as u32);
            // 4d5d9 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4d5db test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 4d5de jne 0x4d5f1
            if fl.ne() { pc = 0x4d5f1; continue; }
            pc = 0x4d5e0;
        }
        0x4d5e0 => {
            // block 4d5e0
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4d5e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d5e0 jmp 0x4daa1
            pc = 0x4daa1; continue;
        }
        0x4d5e5 => {
            // block 4d5e5
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4d5e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d5e5 mov DWORD PTR [rbx],0x0
            m.w32(rbx, (0x0u64) as u32);
            // 4d5eb mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            pc = 0x4d5f1;
        }
        0x4d5f1 => {
            // block 4d5f1
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4d5f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d5f1 test BYTE PTR [rip+0x17ac99],0x10 # 0x1c8291
            fl = logic::<8>(m.r8(m.env.data_va(903774)) as u64 & 0x10u64);
            // 4d5f8 je 0x4da9f
            if fl.e() { pc = 0x4da9f; continue; }
            pc = 0x4d5fe;
        }
        0x4d5fe => {
            // block 4d5fe
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4d5fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d5fe lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 4d605 lea r15,[rbp-0x160]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 4d60c mov rax,QWORD PTR [rip+0xf2bbd] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 4d613 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4d617 mov QWORD PTR [rbp-0x168],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rcx) as u64);
            // 4d61e add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4d622 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // 4d629 mov rsi,r15
            rsi = r15;
            // 4d62c call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d631 mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // 4d639 mov DWORD PTR [rbp-0x68],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0xffffffffu64) as u32);
            // 4d640 mov r12,QWORD PTR [rip+0xf2d79] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 4d647 lea rbx,[r12+0x18]
            rbx = r12.wrapping_add(0x18);
            // 4d64c mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // 4d653 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 4d657 mov QWORD PTR [rbp-0xf8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r12) as u64);
            // 4d65e mov rdi,r15
            rdi = r15;
            // 4d661 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d666 mov QWORD PTR [rbp-0x1a0],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe60), (rbx) as u64);
            // 4d66d mov rax,QWORD PTR [rip+0xf2d34] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4d674 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 4d678 mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 4d67f mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // 4d686 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // 4d691 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 4d69c mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 4d6a7 mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 4d6b2 mov DWORD PTR [rbp-0x100],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x10u64) as u32);
            // 4d6bc xorps xmm0,xmm0
            x0 = 0;
            // 4d6bf movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 4d6c3 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 4d6cb lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 4d6cf mov rdi,r15
            rdi = r15;
            // 4d6d2 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d6d7 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 4d6db call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d6e0 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d6e5 mov r15,rax
            r15 = rax;
            // 4d6e8 lea rsi,[rip+0xdd5d2] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4d6ef lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4d6f6 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4d6fb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d700 lea rsi,[rip+0xdd5be] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 4d707 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 4d70c mov rdi,rax
            rdi = rax;
            // 4d70f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d714 lea rsi,[rip+0xdd265] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4d71b mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4d720 mov rdi,rax
            rdi = rax;
            // 4d723 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d728 lea rsi,[rip+0xdcd46] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4d72f mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4d734 mov rdi,rax
            rdi = rax;
            // 4d737 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d73c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4d73f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d743 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4d747 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d74a or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d74d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4d751 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4d754 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d758 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4d761 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4d764 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d768 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4d773 mov rdi,rax
            rdi = rax;
            // 4d776 mov rsi,r15
            rsi = r15;
            // 4d779 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d77e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4d781 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d785 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4d789 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d78c or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d78f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4d793 lea rsi,[rip+0xdd1e6] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4d79a mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4d79f mov rdi,rax
            rdi = rax;
            // 4d7a2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d7a7 mov rbx,rax
            rbx = rax;
            // 4d7aa lea rsi,[rip+0xdd745] # 0x12aef6
            rsi = m.env.data_va(259779);
            // 4d7b1 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 4d7b8 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 4d7bd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d7c2 movzx edx,BYTE PTR [rbp-0x180]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64) as u32 as u64;
            // 4d7c9 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 4d7cc jne 0x4d7da
            if fl.ne() { pc = 0x4d7da; continue; }
            pc = 0x4d7ce;
        }
        0x4d7ce => {
            // block 4d7ce
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4d7ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d7ce lea rsi,[rbp-0x17f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe81);
            // 4d7d5 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 4d7d8 jmp 0x4d7e8
            pc = 0x4d7e8; continue;
        }
        0x4d7da => {
            // block 4d7da
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4d7da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d7da mov rdx,QWORD PTR [rbp-0x178]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 4d7e1 mov rsi,QWORD PTR [rbp-0x170]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            pc = 0x4d7e8;
        }
        0x4d7e8 => {
            // block 4d7e8
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4d7e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d7e8 mov rdi,rbx
            rdi = rbx;
            // 4d7eb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d7f0 lea rsi,[rip+0xdd189] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4d7f7 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4d7fc mov rdi,rax
            rdi = rax;
            // 4d7ff call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d804 lea rsi,[rip+0xdd565] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 4d80b mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4d810 mov rdi,rax
            rdi = rax;
            // 4d813 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d818 lea rsi,[rip+0xdcc56] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4d81f mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4d824 mov rdi,rax
            rdi = rax;
            // 4d827 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d82c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4d82f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d833 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4d837 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d83a or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d83d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4d841 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4d844 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d848 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4d851 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4d854 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d858 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4d863 mov rdi,rax
            rdi = rax;
            // 4d866 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4d869 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d86e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4d871 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d875 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4d879 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d87c or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d87f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4d883 lea rsi,[rip+0xdd0fc] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4d88a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4d88f mov rdi,rax
            rdi = rax;
            // 4d892 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d897 mov r15,rax
            r15 = rax;
            // 4d89a mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 4d89d call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d8a2 mov rbx,rax
            rbx = rax;
            // 4d8a5 mov rdi,rbx
            rdi = rbx;
            // 4d8a8 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d8ad mov rdi,r15
            rdi = r15;
            // 4d8b0 mov rsi,rbx
            rsi = rbx;
            // 4d8b3 mov rdx,rax
            rdx = rax;
            // 4d8b6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d8bb lea rsi,[rip+0xdcdc0] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4d8c2 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4d8c7 mov rdi,rax
            rdi = rax;
            // 4d8ca call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d8cf lea rsi,[rip+0xdd424] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 4d8d6 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4d8db mov rdi,rax
            rdi = rax;
            // 4d8de call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d8e3 lea rsi,[rip+0xdcb8b] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4d8ea mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4d8ef mov rdi,rax
            rdi = rax;
            // 4d8f2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d8f7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4d8fa mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d8fe mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4d902 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d905 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d908 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4d90c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4d90f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d913 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4d91c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4d91f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d923 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4d92e mov rdi,rax
            rdi = rax;
            // 4d931 mov rsi,r14
            rsi = r14;
            // 4d934 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d939 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4d93c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d940 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4d944 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d947 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d94a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4d94e lea rsi,[rip+0xdd3b7] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 4d955 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4d95a mov rdi,rax
            rdi = rax;
            // 4d95d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d962 lea rsi,[rip+0xdcb0c] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4d969 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4d96e mov rdi,rax
            rdi = rax;
            // 4d971 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d976 mov rbx,rax
            rbx = rax;
            // 4d979 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4d97c mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d980 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4d984 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4d987 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4d98a mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 4d98e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4d991 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d995 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 4d99e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4d9a1 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d9a5 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 4d9b0 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4d9b3 mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 4d9ba xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4d9bc mov rdi,r14
            rdi = r14;
            // 4d9bf call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d9c1 mov rdi,rbx
            rdi = rbx;
            // 4d9c4 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 4d9c6 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d9cb mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4d9ce mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4d9d2 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4d9d6 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d9d9 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4d9dc mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4d9e0 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 4d9e7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d9ec lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // 4d9f3 lea rsi,[rbp-0x160]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 4d9fa call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d9ff test BYTE PTR [rbp-0x198],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64 & 0x1u64);
            // 4da06 jne 0x4da11
            if fl.ne() { pc = 0x4da11; continue; }
            pc = 0x4da08;
        }
        0x4da08 => {
            // block 4da08
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4da08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4da08 lea rdx,[rbp-0x197]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe69);
            // 4da0f jmp 0x4da18
            pc = 0x4da18; continue;
        }
        0x4da11 => {
            // block 4da11
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4da11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4da11 mov rdx,QWORD PTR [rbp-0x188]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            pc = 0x4da18;
        }
        0x4da18 => {
            // block 4da18
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4da18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4da18 mov r15,QWORD PTR [rip+0xf29b9] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 4da1f mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 4da22 lea rsi,[rip+0xdd2f0] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 4da29 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 4da2e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4da30 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4da35 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // 4da3c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4da41 mov rax,QWORD PTR [rbp-0x1a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // 4da48 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4da4f mov QWORD PTR [rbp-0xf8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r12) as u64);
            // 4da56 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4da5d mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // 4da64 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 4da6b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4da70 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 4da77 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4da7c mov rsi,QWORD PTR [rip+0xf2785] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4da83 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4da87 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4da8e call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4da93 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 4da9a call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4da9f;
        }
        0x4da9f => {
            // block 4da9f
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4da9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4da9f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4daa1;
        }
        0x4daa1 => {
            // block 4daa1
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4daa1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4daa1 cmp r15,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 4daa5 jne 0x4db37
            if fl.ne() { pc = 0x4db37; continue; }
            pc = 0x4daab;
        }
        0x4daab => {
            // block 4daab
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4daab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4daab add rsp,0x188
            { let (r, f) = add::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 4dab2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4dab3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4dab5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4dab7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4dab9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4dabb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4dabc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4dabd => {
            // block 4dabd
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4dabd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4dabd mov r14,rax
            r14 = rax;
            // 4dac0 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            pc = 0x4dac7;
        }
        0x4dac7 => {
            // block 4dac7
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4dac7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4dac7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dacc jmp 0x4dad1
            pc = 0x4dad1; continue;
        }
        0x4dace => {
            // block 4dace
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4dace rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4dace mov r14,rax
            r14 = rax;
            pc = 0x4dad1;
        }
        0x4dad1 => {
            // block 4dad1
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4dad1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4dad1 mov rax,QWORD PTR [rbp-0x1a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // 4dad8 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4dadf mov QWORD PTR [rbp-0xf8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r12) as u64);
            // 4dae6 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4daed mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            pc = 0x4daf4;
        }
        0x4daf4 => {
            // block 4daf4
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4daf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4daf4 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 4dafb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4db00 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 4db07 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4db0c;
        }
        0x4db0c => {
            // block 4db0c
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4db0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4db0c mov rsi,QWORD PTR [rip+0xf26f5] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4db13 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4db17 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4db1e call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4db23;
        }
        0x4db23 => {
            // block 4db23
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4db23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4db23 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 4db2a call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4db2f mov rdi,r14
            rdi = r14;
            // 4db32 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4db37;
        }
        0x4db37 => {
            // block 4db37
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4db37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4db37 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x4db41 => {
            // block 4db41
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4db41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4db41 mov r14,rax
            r14 = rax;
            // 4db44 jmp 0x4db0c
            pc = 0x4db0c; continue;
        }
        0x4db46 => {
            // block 4db46
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4db46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4db46 mov r14,rax
            r14 = rax;
            // 4db49 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 4db4d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4db52 jmp 0x4daf4
            pc = 0x4daf4; continue;
        }
        0x4db54 => {
            // block 4db54
            #[cfg(feature = "trace")] eprintln!("f_0004d55a pc 4db54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4db54 mov r14,rax
            r14 = rax;
            // 4db57 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // 4db5e jmp 0x4dac7
            pc = 0x4dac7; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4990e size=1528
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_0004990e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4990e;
    loop {
        match pc {
        0x4990e => {
            // block 4990e
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 4990e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4990e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4990f mov rbp,rsp
            rbp = rsp;
            // 49912 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 49914 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 49916 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 49918 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4991a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4991b sub rsp,0x188
            { let (r, f) = sub::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 49922 mov r14,rdi
            r14 = rdi;
            // 49925 mov rbx,QWORD PTR [rip+0xf6aac] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4992c mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4992f mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 49933 mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 4993a mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 4993c mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 49940 movsxd rax,edx
            rax = sx::<32>((rdx as u32 as u64)) as u64;
            // 49943 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 49947 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4994a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4994c call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49952 mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 49958 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4995a je 0x49994
            if fl.e() { pc = 0x49994; continue; }
            pc = 0x4995c;
        }
        0x4995c => {
            // block 4995c
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 4995c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4995c mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4995f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 49961 mov rdi,r14
            rdi = r14;
            // 49964 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4996a lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4996e lea r9,[rbp-0x64]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 49972 mov esi,0x1e
            rsi = (0x1eu64) as u32 as u64;
            // 49977 mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 4997c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4997f mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 49981 call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49986 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 49989 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4998b test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 4998e je 0x49e44
            if fl.e() { pc = 0x49e44; continue; }
            pc = 0x49994;
        }
        0x49994 => {
            // block 49994
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49994 test BYTE PTR [rip+0x17e8f8],0x40 # 0x1c8293
            fl = logic::<8>(m.r8(m.env.data_va(903776)) as u64 & 0x40u64);
            // 4999b je 0x49e42
            if fl.e() { pc = 0x49e42; continue; }
            pc = 0x499a1;
        }
        0x499a1 => {
            // block 499a1
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 499a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 499a1 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 499a8 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 499af mov rax,QWORD PTR [rip+0xf681a] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 499b6 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 499ba mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 499c1 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 499c5 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 499cc mov rsi,rbx
            rsi = rbx;
            // 499cf call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 499d4 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 499dc mov DWORD PTR [rbp-0x70],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0xffffffffu64) as u32);
            // 499e3 mov r12,QWORD PTR [rip+0xf69d6] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 499ea lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 499ef mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 499f6 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 499fd add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 49a01 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 49a08 mov rdi,rbx
            rdi = rbx;
            // 49a0b call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49a10 mov rax,QWORD PTR [rip+0xf6991] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 49a17 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 49a1b mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            // 49a22 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 49a29 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 49a34 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 49a3f mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 49a4a mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 49a55 mov DWORD PTR [rbp-0x108],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x10u64) as u32);
            // 49a5f xorps xmm0,xmm0
            x0 = 0;
            // 49a62 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 49a66 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 49a6e lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 49a72 mov rdi,rbx
            rdi = rbx;
            // 49a75 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49a7a lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 49a7e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49a83 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49a88 mov r15,rax
            r15 = rax;
            // 49a8b lea rsi,[rip+0xe122f] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 49a92 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 49a99 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 49a9e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49aa3 lea rsi,[rip+0xe121b] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 49aaa mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 49aaf mov rdi,rax
            rdi = rax;
            // 49ab2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49ab7 lea rsi,[rip+0xe0ec2] # 0x12a980
            rsi = m.env.data_va(258381);
            // 49abe mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 49ac3 mov rdi,rax
            rdi = rax;
            // 49ac6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49acb lea rsi,[rip+0xe09a3] # 0x12a475
            rsi = m.env.data_va(257090);
            // 49ad2 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 49ad7 mov rdi,rax
            rdi = rax;
            // 49ada call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49adf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49ae2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49ae6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 49aea and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49aed or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49af0 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 49af4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49af7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49afb mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 49b04 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49b07 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49b0b mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 49b16 mov rdi,rax
            rdi = rax;
            // 49b19 mov rsi,r15
            rsi = r15;
            // 49b1c call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49b21 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49b24 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49b28 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 49b2c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49b2f or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49b32 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 49b36 lea rsi,[rip+0xe0e43] # 0x12a980
            rsi = m.env.data_va(258381);
            // 49b3d mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 49b42 mov rdi,rax
            rdi = rax;
            // 49b45 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49b4a mov rbx,rax
            rbx = rax;
            // 49b4d lea rsi,[rip+0xe12c4] # 0x12ae18
            rsi = m.env.data_va(259557);
            // 49b54 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 49b5b mov edx,0x28
            rdx = (0x28u64) as u32 as u64;
            // 49b60 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49b65 movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 49b6c test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 49b6f jne 0x49b7d
            if fl.ne() { pc = 0x49b7d; continue; }
            pc = 0x49b71;
        }
        0x49b71 => {
            // block 49b71
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49b71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49b71 lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // 49b78 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 49b7b jmp 0x49b8b
            pc = 0x49b8b; continue;
        }
        0x49b7d => {
            // block 49b7d
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49b7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49b7d mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 49b84 mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x49b8b;
        }
        0x49b8b => {
            // block 49b8b
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49b8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49b8b mov rdi,rbx
            rdi = rbx;
            // 49b8e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49b93 lea rsi,[rip+0xe0de6] # 0x12a980
            rsi = m.env.data_va(258381);
            // 49b9a mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 49b9f mov rdi,rax
            rdi = rax;
            // 49ba2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49ba7 lea rsi,[rip+0xe11c2] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 49bae mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 49bb3 mov rdi,rax
            rdi = rax;
            // 49bb6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49bbb lea rsi,[rip+0xe08b3] # 0x12a475
            rsi = m.env.data_va(257090);
            // 49bc2 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 49bc7 mov rdi,rax
            rdi = rax;
            // 49bca call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49bcf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49bd2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49bd6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 49bda and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49bdd or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49be0 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 49be4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49be7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49beb mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 49bf4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49bf7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49bfb mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 49c06 mov rdi,rax
            rdi = rax;
            // 49c09 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 49c0c call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49c11 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49c14 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49c18 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 49c1c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49c1f or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49c22 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 49c26 lea rsi,[rip+0xe0d59] # 0x12a986
            rsi = m.env.data_va(258387);
            // 49c2d mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 49c32 mov rdi,rax
            rdi = rax;
            // 49c35 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49c3a mov r15,rax
            r15 = rax;
            // 49c3d mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 49c40 call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49c45 mov rbx,rax
            rbx = rax;
            // 49c48 mov rdi,rbx
            rdi = rbx;
            // 49c4b call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49c50 mov rdi,r15
            rdi = r15;
            // 49c53 mov rsi,rbx
            rsi = rbx;
            // 49c56 mov rdx,rax
            rdx = rax;
            // 49c59 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49c5e lea rsi,[rip+0xe0a1d] # 0x12a682
            rsi = m.env.data_va(257615);
            // 49c65 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 49c6a mov rdi,rax
            rdi = rax;
            // 49c6d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49c72 lea rsi,[rip+0xe1081] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 49c79 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 49c7e mov rdi,rax
            rdi = rax;
            // 49c81 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49c86 lea rsi,[rip+0xe07e8] # 0x12a475
            rsi = m.env.data_va(257090);
            // 49c8d mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 49c92 mov rdi,rax
            rdi = rax;
            // 49c95 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49c9a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49c9d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49ca1 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 49ca5 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49ca8 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49cab mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 49caf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49cb2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49cb6 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 49cbf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49cc2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49cc6 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 49cd1 mov rdi,rax
            rdi = rax;
            // 49cd4 mov rsi,r14
            rsi = r14;
            // 49cd7 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49cdc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49cdf mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49ce3 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 49ce7 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49cea or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49ced mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 49cf1 lea rsi,[rip+0xe1014] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 49cf8 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 49cfd mov rdi,rax
            rdi = rax;
            // 49d00 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49d05 lea rsi,[rip+0xe0769] # 0x12a475
            rsi = m.env.data_va(257090);
            // 49d0c mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 49d11 mov rdi,rax
            rdi = rax;
            // 49d14 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49d19 mov rbx,rax
            rbx = rax;
            // 49d1c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 49d1f mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49d23 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 49d27 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 49d2a or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 49d2d mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 49d31 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 49d34 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49d38 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 49d41 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 49d44 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49d48 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 49d53 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 49d56 mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 49d5d xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 49d5f mov rdi,r14
            rdi = r14;
            // 49d62 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49d64 mov rdi,rbx
            rdi = rbx;
            // 49d67 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 49d69 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49d6e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49d71 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49d75 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 49d79 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49d7c or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49d7f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 49d83 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 49d8a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49d8f lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 49d96 lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 49d9d call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49da2 test BYTE PTR [rbp-0x1a0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64 & 0x1u64);
            // 49da9 jne 0x49db4
            if fl.ne() { pc = 0x49db4; continue; }
            pc = 0x49dab;
        }
        0x49dab => {
            // block 49dab
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49dab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49dab lea rdx,[rbp-0x19f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe61);
            // 49db2 jmp 0x49dbb
            pc = 0x49dbb; continue;
        }
        0x49db4 => {
            // block 49db4
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49db4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49db4 mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            pc = 0x49dbb;
        }
        0x49dbb => {
            // block 49dbb
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49dbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49dbb mov rbx,QWORD PTR [rip+0xf6616] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 49dc2 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 49dc5 lea rsi,[rip+0xe0f4d] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 49dcc mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 49dd1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 49dd3 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49dd8 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 49ddf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49de4 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 49deb mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 49df2 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 49df9 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 49e00 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 49e07 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 49e0e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49e13 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 49e1a call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49e1f mov rsi,QWORD PTR [rip+0xf63e2] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 49e26 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 49e2a lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 49e31 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49e36 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 49e3d call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x49e42;
        }
        0x49e42 => {
            // block 49e42
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49e42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49e42 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x49e44;
        }
        0x49e44 => {
            // block 49e44
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49e44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49e44 cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 49e48 jne 0x49eda
            if fl.ne() { pc = 0x49eda; continue; }
            pc = 0x49e4e;
        }
        0x49e4e => {
            // block 49e4e
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49e4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49e4e add rsp,0x188
            { let (r, f) = add::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 49e55 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 49e56 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 49e58 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 49e5a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 49e5c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 49e5e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 49e5f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x49e60 => {
            // block 49e60
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49e60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49e60 mov rbx,rax
            rbx = rax;
            // 49e63 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            pc = 0x49e6a;
        }
        0x49e6a => {
            // block 49e6a
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49e6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49e6a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49e6f jmp 0x49e74
            pc = 0x49e74; continue;
        }
        0x49e71 => {
            // block 49e71
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49e71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49e71 mov rbx,rax
            rbx = rax;
            pc = 0x49e74;
        }
        0x49e74 => {
            // block 49e74
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49e74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49e74 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 49e7b mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 49e82 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 49e89 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 49e90 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            pc = 0x49e97;
        }
        0x49e97 => {
            // block 49e97
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49e97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49e97 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 49e9e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49ea3 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 49eaa call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x49eaf;
        }
        0x49eaf => {
            // block 49eaf
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49eaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49eaf mov rsi,QWORD PTR [rip+0xf6352] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 49eb6 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 49eba lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 49ec1 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x49ec6;
        }
        0x49ec6 => {
            // block 49ec6
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49ec6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49ec6 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 49ecd call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49ed2 mov rdi,rbx
            rdi = rbx;
            // 49ed5 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x49eda;
        }
        0x49eda => {
            // block 49eda
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49eda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49eda call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x49ee4 => {
            // block 49ee4
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49ee4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49ee4 mov rbx,rax
            rbx = rax;
            // 49ee7 jmp 0x49eaf
            pc = 0x49eaf; continue;
        }
        0x49ee9 => {
            // block 49ee9
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49ee9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49ee9 mov rbx,rax
            rbx = rax;
            // 49eec lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 49ef0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49ef5 jmp 0x49e97
            pc = 0x49e97; continue;
        }
        0x49ef7 => {
            // block 49ef7
            #[cfg(feature = "trace")] eprintln!("f_0004990e pc 49ef7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49ef7 mov rbx,rax
            rbx = rax;
            // 49efa lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 49f01 jmp 0x49e6a
            pc = 0x49e6a; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x47910 size=2026
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_00047910<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x47910;
    loop {
        match pc {
        0x47910 => {
            // block 47910
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 47910 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47910 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 47911 mov rbp,rsp
            rbp = rsp;
            // 47914 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 47916 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 47918 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4791a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4791c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4791d sub rsp,0x1c8
            { let (r, f) = sub::<64>(rsp, 0x1c8u64);
              rsp = r;
              fl = f; }
            // 47924 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // 47927 mov DWORD PTR [rbp-0x1bc],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe44), ((rcx as u32 as u64)) as u32);
            // 4792d mov DWORD PTR [rbp-0x1b8],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe48), ((rdx as u32 as u64)) as u32);
            // 47933 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 47936 mov r12,rdi
            r12 = rdi;
            // 47939 mov rbx,QWORD PTR [rip+0xf8a98] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 47940 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 47943 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 47947 mov DWORD PTR [rbp-0x74],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c), (0x0u64) as u32);
            // 4794e mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 47951 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 47955 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 47957 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 4795b mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 4795d mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 47961 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 47965 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 47967 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4796d mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 47973 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 47975 je 0x479b0
            if fl.e() { pc = 0x479b0; continue; }
            pc = 0x47977;
        }
        0x47977 => {
            // block 47977
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 47977 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47977 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4797b xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4797d mov rdi,r12
            rdi = r12;
            // 47980 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47986 lea rdx,[rbp-0x50]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 4798a lea r9,[rbp-0x74]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // 4798e mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 47993 mov ecx,0x3
            rcx = (0x3u64) as u32 as u64;
            // 47998 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4799b mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4799d call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 479a2 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 479a5 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 479a7 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 479aa je 0x4801c
            if fl.e() { pc = 0x4801c; continue; }
            pc = 0x479b0;
        }
        0x479b0 => {
            // block 479b0
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 479b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 479b0 test BYTE PTR [rip+0x1808d9],0x2 # 0x1c8290
            fl = logic::<8>(m.r8(m.env.data_va(903773)) as u64 & 0x2u64);
            // 479b7 je 0x4801a
            if fl.e() { pc = 0x4801a; continue; }
            pc = 0x479bd;
        }
        0x479bd => {
            // block 479bd
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 479bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 479bd lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 479c4 lea rbx,[rbp-0x178]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 479cb mov rax,QWORD PTR [rip+0xf87fe] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 479d2 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 479d6 mov QWORD PTR [rbp-0x180],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rcx) as u64);
            // 479dd add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 479e1 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 479e8 mov rsi,rbx
            rsi = rbx;
            // 479eb call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 479f0 mov QWORD PTR [rbp-0x88],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (0x0u64) as u64);
            // 479fb mov DWORD PTR [rbp-0x80],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0xffffffffu64) as u32);
            // 47a02 mov rax,QWORD PTR [rip+0xf89b7] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 47a09 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 47a0d mov QWORD PTR [rbp-0x1d0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe30), (rcx) as u64);
            // 47a14 mov QWORD PTR [rbp-0x180],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rcx) as u64);
            // 47a1b add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 47a1f mov QWORD PTR [rbp-0x1c8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe38), (rax) as u64);
            // 47a26 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 47a2d mov rdi,rbx
            rdi = rbx;
            // 47a30 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47a35 mov rax,QWORD PTR [rip+0xf896c] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 47a3c add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 47a40 mov QWORD PTR [rbp-0x1d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe28), (rax) as u64);
            // 47a47 mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            // 47a4e mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 47a59 mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 47a64 mov QWORD PTR [rbp-0x130],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (0x0u64) as u64);
            // 47a6f mov QWORD PTR [rbp-0x138],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (0x0u64) as u64);
            // 47a7a mov DWORD PTR [rbp-0x118],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x10u64) as u32);
            // 47a84 xorps xmm0,xmm0
            x0 = 0;
            // 47a87 movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // 47a8b mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 47a93 lea rsi,[rbp-0x70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 47a97 mov rdi,rbx
            rdi = rbx;
            // 47a9a call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47a9f lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 47aa3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47aa8 call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47aad mov rbx,rax
            rbx = rax;
            // 47ab0 lea rsi,[rip+0xe320a] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 47ab7 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 47abe mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 47ac3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47ac8 lea rsi,[rip+0xe31f6] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 47acf mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 47ad4 mov rdi,rax
            rdi = rax;
            // 47ad7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47adc lea rsi,[rip+0xe2e9d] # 0x12a980
            rsi = m.env.data_va(258381);
            // 47ae3 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 47ae8 mov rdi,rax
            rdi = rax;
            // 47aeb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47af0 lea rsi,[rip+0xe297e] # 0x12a475
            rsi = m.env.data_va(257090);
            // 47af7 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 47afc mov rdi,rax
            rdi = rax;
            // 47aff call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47b04 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47b07 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47b0b mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47b0f and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47b12 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47b15 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47b19 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47b1c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47b20 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 47b29 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47b2c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47b30 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 47b3b mov rdi,rax
            rdi = rax;
            // 47b3e mov rsi,rbx
            rsi = rbx;
            // 47b41 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47b46 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47b49 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47b4d mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47b51 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47b54 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47b57 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47b5b lea rsi,[rip+0xe2e1e] # 0x12a980
            rsi = m.env.data_va(258381);
            // 47b62 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 47b67 mov rdi,rax
            rdi = rax;
            // 47b6a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47b6f mov rbx,rax
            rbx = rax;
            // 47b72 lea rsi,[rip+0xe3235] # 0x12adae
            rsi = m.env.data_va(259451);
            // 47b79 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // 47b80 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 47b85 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47b8a mov DWORD PTR [rbp-0x1dc],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe24), ((r15 as u32 as u64)) as u32);
            // 47b91 movzx edx,BYTE PTR [rbp-0x198]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64) as u32 as u64;
            // 47b98 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 47b9b jne 0x47ba9
            if fl.ne() { pc = 0x47ba9; continue; }
            pc = 0x47b9d;
        }
        0x47b9d => {
            // block 47b9d
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 47b9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47b9d lea rsi,[rbp-0x197]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe69);
            // 47ba4 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 47ba7 jmp 0x47bb7
            pc = 0x47bb7; continue;
        }
        0x47ba9 => {
            // block 47ba9
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 47ba9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47ba9 mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // 47bb0 mov rsi,QWORD PTR [rbp-0x188]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            pc = 0x47bb7;
        }
        0x47bb7 => {
            // block 47bb7
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 47bb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47bb7 mov rdi,rbx
            rdi = rbx;
            // 47bba call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47bbf lea rsi,[rip+0xe2dba] # 0x12a980
            rsi = m.env.data_va(258381);
            // 47bc6 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 47bcb mov rdi,rax
            rdi = rax;
            // 47bce call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47bd3 lea rsi,[rip+0xe3196] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 47bda mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 47bdf mov rdi,rax
            rdi = rax;
            // 47be2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47be7 lea rsi,[rip+0xe2887] # 0x12a475
            rsi = m.env.data_va(257090);
            // 47bee mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 47bf3 mov rdi,rax
            rdi = rax;
            // 47bf6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47bfb mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47bfe mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47c02 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47c06 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47c09 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47c0c mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47c10 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47c13 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47c17 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 47c20 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47c23 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47c27 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 47c32 mov rdi,rax
            rdi = rax;
            // 47c35 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 47c38 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47c3d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47c40 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47c44 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47c48 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47c4b or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47c4e mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47c52 lea rsi,[rip+0xe2d2d] # 0x12a986
            rsi = m.env.data_va(258387);
            // 47c59 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 47c5e mov rdi,rax
            rdi = rax;
            // 47c61 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47c66 mov r15,rax
            r15 = rax;
            // 47c69 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 47c6c call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47c71 mov rbx,rax
            rbx = rax;
            // 47c74 mov rdi,rbx
            rdi = rbx;
            // 47c77 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47c7c mov rdi,r15
            rdi = r15;
            // 47c7f mov rsi,rbx
            rsi = rbx;
            // 47c82 mov rdx,rax
            rdx = rax;
            // 47c85 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47c8a lea rsi,[rip+0xe29f1] # 0x12a682
            rsi = m.env.data_va(257615);
            // 47c91 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 47c96 mov rdi,rax
            rdi = rax;
            // 47c99 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47c9e lea rsi,[rip+0xe3055] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 47ca5 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 47caa mov rdi,rax
            rdi = rax;
            // 47cad call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47cb2 lea rsi,[rip+0xe27bc] # 0x12a475
            rsi = m.env.data_va(257090);
            // 47cb9 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 47cbe mov rdi,rax
            rdi = rax;
            // 47cc1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47cc6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47cc9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47ccd mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47cd1 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47cd4 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47cd7 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47cdb mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47cde mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47ce2 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 47ceb mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47cee mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47cf2 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 47cfd mov rdi,rax
            rdi = rax;
            // 47d00 mov rsi,r12
            rsi = r12;
            // 47d03 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47d08 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47d0b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47d0f mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47d13 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47d16 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47d19 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47d1d lea rsi,[rip+0xe2fe8] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 47d24 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 47d29 mov rdi,rax
            rdi = rax;
            // 47d2c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47d31 lea rsi,[rip+0xe273d] # 0x12a475
            rsi = m.env.data_va(257090);
            // 47d38 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 47d3d mov rdi,rax
            rdi = rax;
            // 47d40 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47d45 mov rbx,rax
            rbx = rax;
            // 47d48 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 47d4b mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47d4f mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47d53 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 47d56 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 47d59 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 47d5d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 47d60 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47d64 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 47d6d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 47d70 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47d74 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 47d7f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 47d83 mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 47d8a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 47d8c mov rdi,r12
            rdi = r12;
            // 47d8f call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47d91 mov rdi,rbx
            rdi = rbx;
            // 47d94 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 47d96 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47d9b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47d9e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47da2 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47da6 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47da9 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47dac mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47db0 lea rsi,[rip+0xe2fc5] # 0x12ad7c
            rsi = m.env.data_va(259401);
            // 47db7 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 47dbc mov rdi,rax
            rdi = rax;
            // 47dbf call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47dc4 mov rdi,rax
            rdi = rax;
            // 47dc7 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 47dca call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47dcf lea rsi,[rip+0xe2fe6] # 0x12adbc
            rsi = m.env.data_va(259465);
            // 47dd6 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 47ddb mov rdi,rax
            rdi = rax;
            // 47dde call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47de3 lea rsi,[rip+0xe268b] # 0x12a475
            rsi = m.env.data_va(257090);
            // 47dea mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 47def mov rdi,rax
            rdi = rax;
            // 47df2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47df7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47dfa mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47dfe mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47e02 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47e05 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47e08 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47e0c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47e0f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47e13 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 47e1c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47e1f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47e23 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 47e2e mov rdi,rax
            rdi = rax;
            // 47e31 mov esi,DWORD PTR [rbp-0x1b8]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64) as u32 as u64;
            // 47e37 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47e3c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47e3f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47e43 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47e47 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47e4a or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47e4d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47e51 lea rsi,[rip+0xe2f2d] # 0x12ad85
            rsi = m.env.data_va(259410);
            // 47e58 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 47e5d mov rdi,rax
            rdi = rax;
            // 47e60 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47e65 lea rsi,[rip+0xe2609] # 0x12a475
            rsi = m.env.data_va(257090);
            // 47e6c mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 47e71 mov rdi,rax
            rdi = rax;
            // 47e74 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47e79 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47e7c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47e80 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47e84 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47e87 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47e8a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47e8e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47e91 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47e95 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 47e9e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47ea1 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47ea5 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 47eb0 mov rdi,rax
            rdi = rax;
            // 47eb3 mov esi,DWORD PTR [rbp-0x1bc]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64) as u32 as u64;
            // 47eb9 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47ebe mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47ec1 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47ec5 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47ec9 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47ecc or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47ecf mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47ed3 lea rsi,[rip+0xe2eb3] # 0x12ad8d
            rsi = m.env.data_va(259418);
            // 47eda mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 47edf mov rdi,rax
            rdi = rax;
            // 47ee2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47ee7 lea rsi,[rip+0xe2587] # 0x12a475
            rsi = m.env.data_va(257090);
            // 47eee mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 47ef3 mov rdi,rax
            rdi = rax;
            // 47ef6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47efb mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47efe mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47f02 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47f06 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47f09 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47f0c mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47f10 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47f13 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47f17 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 47f20 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47f23 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47f27 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 47f32 mov rdi,rax
            rdi = rax;
            // 47f35 mov esi,DWORD PTR [rbp-0x1dc]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe24)) as u64) as u32 as u64;
            // 47f3b call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47f40 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 47f43 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 47f47 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 47f4b and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47f4e or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 47f51 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 47f55 lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            // 47f5c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47f61 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // 47f68 lea r14,[rbp-0x178]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 47f6f mov rsi,r14
            rsi = r14;
            // 47f72 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47f77 mov rbx,QWORD PTR [rip+0xf845a] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 47f7e mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 47f81 test BYTE PTR [rbp-0x1b0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64 & 0x1u64);
            // 47f88 jne 0x47f93
            if fl.ne() { pc = 0x47f93; continue; }
            pc = 0x47f8a;
        }
        0x47f8a => {
            // block 47f8a
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 47f8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47f8a lea rdx,[rbp-0x1af]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe51);
            // 47f91 jmp 0x47f9a
            pc = 0x47f9a; continue;
        }
        0x47f93 => {
            // block 47f93
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 47f93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47f93 mov rdx,QWORD PTR [rbp-0x1a0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            pc = 0x47f9a;
        }
        0x47f9a => {
            // block 47f9a
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 47f9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 47f9a lea rsi,[rip+0xe2d78] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 47fa1 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 47fa6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 47fa8 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47fad lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // 47fb4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47fb9 mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // 47fc0 mov QWORD PTR [rbp-0x180],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rax) as u64);
            // 47fc7 mov rax,QWORD PTR [rbp-0x1c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64;
            // 47fce mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 47fd5 mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // 47fdc mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            // 47fe3 lea rdi,[rbp-0x138]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // 47fea call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47fef mov rdi,r14
            rdi = r14;
            // 47ff2 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 47ff7 mov rsi,QWORD PTR [rip+0xf820a] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 47ffe add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 48002 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 48009 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4800e lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 48015 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4801a;
        }
        0x4801a => {
            // block 4801a
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 4801a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4801a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4801c;
        }
        0x4801c => {
            // block 4801c
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 4801c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4801c cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 48020 jne 0x480b9
            if fl.ne() { pc = 0x480b9; continue; }
            pc = 0x48026;
        }
        0x48026 => {
            // block 48026
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 48026 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 48026 add rsp,0x1c8
            { let (r, f) = add::<64>(rsp, 0x1c8u64);
              rsp = r;
              fl = f; }
            // 4802d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4802e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 48030 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 48032 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 48034 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 48036 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 48037 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x48038 => {
            // block 48038
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 48038 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 48038 mov r14,rax
            r14 = rax;
            // 4803b lea rdi,[rbp-0x198]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe68);
            pc = 0x48042;
        }
        0x48042 => {
            // block 48042
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 48042 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 48042 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48047 jmp 0x4804c
            pc = 0x4804c; continue;
        }
        0x48049 => {
            // block 48049
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 48049 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 48049 mov r14,rax
            r14 = rax;
            pc = 0x4804c;
        }
        0x4804c => {
            // block 4804c
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 4804c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4804c mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // 48053 mov QWORD PTR [rbp-0x180],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rax) as u64);
            // 4805a mov rax,QWORD PTR [rbp-0x1c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64;
            // 48061 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 48068 mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // 4806f mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            // 48076 lea rdi,[rbp-0x138]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // 4807d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 48082 lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 48089 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4808e;
        }
        0x4808e => {
            // block 4808e
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 4808e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4808e mov rsi,QWORD PTR [rip+0xf8173] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 48095 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 48099 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 480a0 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x480a5;
        }
        0x480a5 => {
            // block 480a5
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 480a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 480a5 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 480ac call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 480b1 mov rdi,r14
            rdi = r14;
            // 480b4 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x480b9;
        }
        0x480b9 => {
            // block 480b9
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 480b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 480b9 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x480c3 => {
            // block 480c3
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 480c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 480c3 mov r14,rax
            r14 = rax;
            // 480c6 jmp 0x4808e
            pc = 0x4808e; continue;
        }
        0x480c8 => {
            // block 480c8
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 480c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 480c8 mov r14,rax
            r14 = rax;
            // 480cb lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 480cf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 480d4 lea rdi,[rbp-0x138]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // 480db call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 480e0 mov rdi,rbx
            rdi = rbx;
            // 480e3 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 480e8 jmp 0x4808e
            pc = 0x4808e; continue;
        }
        0x480ea => {
            // block 480ea
            #[cfg(feature = "trace")] eprintln!("f_00047910 pc 480ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 480ea mov r14,rax
            r14 = rax;
            // 480ed lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // 480f4 jmp 0x48042
            pc = 0x48042; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x516ec size=1710
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000516ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x516ec;
    loop {
        match pc {
        0x516ec => {
            // block 516ec
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 516ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 516ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 516ed mov rbp,rsp
            rbp = rsp;
            // 516f0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 516f2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 516f4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 516f6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 516f8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 516f9 sub rsp,0x1a8
            { let (r, f) = sub::<64>(rsp, 0x1a8u64);
              rsp = r;
              fl = f; }
            // 51700 mov r15,rsi
            r15 = rsi;
            // 51703 mov r14,rdi
            r14 = rdi;
            // 51706 mov r13,QWORD PTR [rip+0xeeccb] # 0x1403d8
            r13 = m.r64(m.env.data_va(347045)) as u64;
            // 5170d mov r13,QWORD PTR [r13+0x0]
            r13 = m.r64(r13) as u64;
            // 51711 mov QWORD PTR [rbp-0x30],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r13) as u64);
            // 51715 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 51718 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 5171a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5171c call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51722 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // 51725 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 51728 je 0x51cc1
            if fl.e() { pc = 0x51cc1; continue; }
            pc = 0x5172e;
        }
        0x5172e => {
            // block 5172e
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 5172e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5172e mov eax,DWORD PTR [r15+0x10]
            rax = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 51732 test rax,rax
            fl = logic::<64>(rax & rax);
            // 51735 je 0x51979
            if fl.e() { pc = 0x51979; continue; }
            pc = 0x5173b;
        }
        0x5173b => {
            // block 5173b
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 5173b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5173b mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 51742 mov QWORD PTR [rbp-0x40],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r15) as u64);
            // 51746 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 5174a mov eax,0x5
            rax = (0x5u64) as u32 as u64;
            // 5174f test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // 51752 je 0x51784
            if fl.e() { pc = 0x51784; continue; }
            pc = 0x51754;
        }
        0x51754 => {
            // block 51754
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51754 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51754 lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 51758 lea r9,[rbp-0x64]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 5175c mov esi,0x22
            rsi = (0x22u64) as u32 as u64;
            // 51761 mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 51766 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51769 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 5176c call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51771 cmp eax,0xe00002d7
            fl = sub::<32>((rax as u32 as u64), 0xe00002d7u64).1;
            // 51776 je 0x51cbc
            if fl.e() { pc = 0x51cbc; continue; }
            pc = 0x5177c;
        }
        0x5177c => {
            // block 5177c
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 5177c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5177c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 5177e je 0x51cbc
            if fl.e() { pc = 0x51cbc; continue; }
            pc = 0x51784;
        }
        0x51784 => {
            // block 51784
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51784 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51784 test BYTE PTR [rip+0x176b09],0x4 # 0x1c8294
            fl = logic::<8>(m.r8(m.env.data_va(903777)) as u64 & 0x4u64);
            // 5178b je 0x51cbc
            if fl.e() { pc = 0x51cbc; continue; }
            pc = 0x51791;
        }
        0x51791 => {
            // block 51791
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51791 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51791 mov DWORD PTR [rbp-0x1a4],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe5c), ((rax as u32 as u64)) as u32);
            // 51797 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 5179e lea r13,[rbp-0x168]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 517a5 mov rax,QWORD PTR [rip+0xeea24] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 517ac lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 517b0 mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 517b7 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 517bb mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 517c2 mov rsi,r13
            rsi = r13;
            // 517c5 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 517ca mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 517d2 mov DWORD PTR [rbp-0x70],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0xffffffffu64) as u32);
            // 517d9 mov rbx,QWORD PTR [rip+0xeebe0] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // 517e0 lea rax,[rbx+0x18]
            rax = rbx.wrapping_add(0x18);
            // 517e4 mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            // 517eb mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 517f2 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // 517f6 mov QWORD PTR [rbp-0x100],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rbx) as u64);
            // 517fd mov rdi,r13
            rdi = r13;
            // 51800 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51805 mov QWORD PTR [rbp-0x1b8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe48), (rbx) as u64);
            // 5180c mov rax,QWORD PTR [rip+0xeeb95] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 51813 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 51817 mov QWORD PTR [rbp-0x1c0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe40), (rax) as u64);
            // 5181e mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 51825 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 51830 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 5183b mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 51846 mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 51851 mov DWORD PTR [rbp-0x108],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x10u64) as u32);
            // 5185b xorps xmm0,xmm0
            x0 = 0;
            // 5185e movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 51862 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 5186a lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 5186e mov rdi,r13
            rdi = r13;
            // 51871 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51876 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 5187a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5187f call 0xec26c
            { let r = { let t = m.env.stub_va(0xa39); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51884 mov rbx,rax
            rbx = rax;
            // 51887 lea rsi,[rip+0xd9433] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 5188e lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 51895 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 5189a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5189f lea rsi,[rip+0xd941f] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 518a6 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 518ab mov rdi,rax
            rdi = rax;
            // 518ae call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 518b3 lea rsi,[rip+0xd90c6] # 0x12a980
            rsi = m.env.data_va(258381);
            // 518ba mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 518bf mov rdi,rax
            rdi = rax;
            // 518c2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 518c7 lea rsi,[rip+0xd8ba7] # 0x12a475
            rsi = m.env.data_va(257090);
            // 518ce mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 518d3 mov rdi,rax
            rdi = rax;
            // 518d6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 518db mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 518de mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 518e2 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 518e6 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 518e9 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 518ec mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 518f0 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 518f3 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 518f7 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 51900 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 51903 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51907 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 51912 mov rdi,rax
            rdi = rax;
            // 51915 mov rsi,rbx
            rsi = rbx;
            // 51918 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5191d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 51920 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51924 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 51928 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 5192b or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 5192e mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 51932 lea rsi,[rip+0xd9047] # 0x12a980
            rsi = m.env.data_va(258381);
            // 51939 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 5193e mov rdi,rax
            rdi = rax;
            // 51941 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51946 mov rbx,rax
            rbx = rax;
            // 51949 lea rsi,[rip+0xd967e] # 0x12afce
            rsi = m.env.data_va(259995);
            // 51950 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 51957 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 5195c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51961 movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 51968 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 5196b jne 0x51980
            if fl.ne() { pc = 0x51980; continue; }
            pc = 0x5196d;
        }
        0x5196d => {
            // block 5196d
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 5196d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5196d lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // 51974 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 51977 jmp 0x5198e
            pc = 0x5198e; continue;
        }
        0x51979 => {
            // block 51979
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51979 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51979 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 5197b jmp 0x51cc1
            pc = 0x51cc1; continue;
        }
        0x51980 => {
            // block 51980
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51980 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51980 mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 51987 mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x5198e;
        }
        0x5198e => {
            // block 5198e
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 5198e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5198e mov rdi,rbx
            rdi = rbx;
            // 51991 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51996 lea rsi,[rip+0xd8fe3] # 0x12a980
            rsi = m.env.data_va(258381);
            // 5199d mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 519a2 mov rdi,rax
            rdi = rax;
            // 519a5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 519aa lea rsi,[rip+0xd93bf] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 519b1 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 519b6 mov rdi,rax
            rdi = rax;
            // 519b9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 519be lea rsi,[rip+0xd8ab0] # 0x12a475
            rsi = m.env.data_va(257090);
            // 519c5 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 519ca mov rdi,rax
            rdi = rax;
            // 519cd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 519d2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 519d5 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 519d9 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 519dd and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 519e0 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 519e3 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 519e7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 519ea mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 519ee mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 519f7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 519fa mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 519fe mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 51a09 mov rdi,rax
            rdi = rax;
            // 51a0c mov esi,DWORD PTR [rbp-0x1a4]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe5c)) as u64) as u32 as u64;
            // 51a12 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51a17 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 51a1a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51a1e mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 51a22 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 51a25 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 51a28 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 51a2c lea rsi,[rip+0xd8f53] # 0x12a986
            rsi = m.env.data_va(258387);
            // 51a33 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 51a38 mov rdi,rax
            rdi = rax;
            // 51a3b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51a40 mov rbx,rax
            rbx = rax;
            // 51a43 mov edi,DWORD PTR [rbp-0x1a4]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe5c)) as u64) as u32 as u64;
            // 51a49 call 0x474d5
            { let r = { let t = m.env.code_va(0x474d5); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51a4e mov r13,rax
            r13 = rax;
            // 51a51 mov rdi,r13
            rdi = r13;
            // 51a54 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51a59 mov rdi,rbx
            rdi = rbx;
            // 51a5c mov rsi,r13
            rsi = r13;
            // 51a5f mov rdx,rax
            rdx = rax;
            // 51a62 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51a67 lea rsi,[rip+0xd8c14] # 0x12a682
            rsi = m.env.data_va(257615);
            // 51a6e mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 51a73 mov rdi,rax
            rdi = rax;
            // 51a76 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51a7b lea rsi,[rip+0xd9278] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 51a82 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 51a87 mov rdi,rax
            rdi = rax;
            // 51a8a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51a8f lea rsi,[rip+0xd89df] # 0x12a475
            rsi = m.env.data_va(257090);
            // 51a96 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 51a9b mov rdi,rax
            rdi = rax;
            // 51a9e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51aa3 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 51aa6 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51aaa mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 51aae and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 51ab1 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 51ab4 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 51ab8 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 51abb mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51abf mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 51ac8 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 51acb mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51acf mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 51ada mov rdi,rax
            rdi = rax;
            // 51add mov rsi,r14
            rsi = r14;
            // 51ae0 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51ae5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 51ae8 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51aec mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 51af0 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 51af3 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 51af6 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 51afa lea rsi,[rip+0xd920b] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 51b01 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 51b06 mov rdi,rax
            rdi = rax;
            // 51b09 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51b0e lea rsi,[rip+0xd8960] # 0x12a475
            rsi = m.env.data_va(257090);
            // 51b15 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 51b1a mov rdi,rax
            rdi = rax;
            // 51b1d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51b22 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 51b25 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51b29 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 51b2d and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 51b30 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 51b33 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 51b37 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 51b3a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51b3e mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 51b47 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 51b4a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51b4e mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 51b59 mov rdi,rax
            rdi = rax;
            // 51b5c mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 51b5f call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51b64 mov rbx,rax
            rbx = rax;
            // 51b67 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 51b6a mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51b6e mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 51b72 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 51b75 or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 51b78 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 51b7c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 51b7f mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51b83 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 51b86 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 51b8a call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51b8f mov rsi,QWORD PTR [rip+0xee5fa] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 51b96 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 51b9a call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51b9f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 51ba2 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 51ba6 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 51bab mov rdi,rax
            rdi = rax;
            // 51bae call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51bb0 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 51bb3 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 51bb7 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51bbc movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // 51bc0 mov rdi,rbx
            rdi = rbx;
            // 51bc3 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51bc8 mov rdi,rbx
            rdi = rbx;
            // 51bcb call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51bd0 lea rsi,[rip+0xd9403] # 0x12afda
            rsi = m.env.data_va(260007);
            // 51bd7 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 51bdc mov rdi,rbx
            rdi = rbx;
            // 51bdf call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51be4 mov rdi,rax
            rdi = rax;
            // 51be7 mov rsi,r15
            rsi = r15;
            // 51bea call 0x2615f
            { let r = { let t = m.env.code_va(0x2615f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51bef lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 51bf6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51bfb lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 51c02 lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 51c09 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51c0e mov r13,QWORD PTR [rip+0xee7c3] # 0x1403d8
            r13 = m.r64(m.env.data_va(347045)) as u64;
            // 51c15 mov r13,QWORD PTR [r13+0x0]
            r13 = m.r64(r13) as u64;
            // 51c19 test BYTE PTR [rbp-0x1a0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64 & 0x1u64);
            // 51c20 jne 0x51c2b
            if fl.ne() { pc = 0x51c2b; continue; }
            pc = 0x51c22;
        }
        0x51c22 => {
            // block 51c22
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51c22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51c22 lea rdx,[rbp-0x19f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe61);
            // 51c29 jmp 0x51c32
            pc = 0x51c32; continue;
        }
        0x51c2b => {
            // block 51c2b
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51c2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51c2b mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            pc = 0x51c32;
        }
        0x51c32 => {
            // block 51c32
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51c32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51c32 lea rsi,[rip+0xd90e0] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 51c39 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 51c3e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 51c40 call 0xec2e4
            { let r = { let t = m.env.stub_va(0xab1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51c45 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 51c4c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51c51 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 51c58 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 51c5f mov rax,QWORD PTR [rbp-0x1b8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            // 51c66 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 51c6d mov rax,QWORD PTR [rbp-0x1c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // 51c74 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 51c7b lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 51c82 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51c87 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 51c8e call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51c93 mov rsi,QWORD PTR [rip+0xee56e] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 51c9a add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 51c9e lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 51ca5 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51caa lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 51cb1 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51cb6 mov eax,DWORD PTR [rbp-0x1a4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe5c)) as u64) as u32 as u64;
            pc = 0x51cbc;
        }
        0x51cbc => {
            // block 51cbc
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51cbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51cbc test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 51cbe sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x51cc1;
        }
        0x51cc1 => {
            // block 51cc1
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51cc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51cc1 cmp r13,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r13, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 51cc5 jne 0x51cec
            if fl.ne() { pc = 0x51cec; continue; }
            pc = 0x51cc7;
        }
        0x51cc7 => {
            // block 51cc7
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51cc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51cc7 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 51cc9 add rsp,0x1a8
            { let (r, f) = add::<64>(rsp, 0x1a8u64);
              rsp = r;
              fl = f; }
            // 51cd0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 51cd1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 51cd3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 51cd5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 51cd7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 51cd9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51cda ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x51cdb => {
            // block 51cdb
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51cdb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51cdb mov rbx,rax
            rbx = rax;
            pc = 0x51cde;
        }
        0x51cde => {
            // block 51cde
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51cde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51cde lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            pc = 0x51ce5;
        }
        0x51ce5 => {
            // block 51ce5
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51ce5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51ce5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51cea jmp 0x51cf4
            pc = 0x51cf4; continue;
        }
        0x51cec => {
            // block 51cec
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51cec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51cec call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x51cf4 => {
            // block 51cf4
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51cf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51cf4 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 51cfb mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 51d02 mov rax,QWORD PTR [rbp-0x1b8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            // 51d09 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 51d10 mov rax,QWORD PTR [rbp-0x1c0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // 51d17 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            pc = 0x51d1e;
        }
        0x51d1e => {
            // block 51d1e
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51d1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51d1e lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 51d25 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51d2a lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 51d31 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x51d36;
        }
        0x51d36 => {
            // block 51d36
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51d36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51d36 mov rsi,QWORD PTR [rip+0xee4cb] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 51d3d add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 51d41 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 51d48 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x51d4d;
        }
        0x51d4d => {
            // block 51d4d
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51d4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51d4d lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 51d54 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51d59 mov rdi,rbx
            rdi = rbx;
            // 51d5c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51d61 mov rbx,rax
            rbx = rax;
            // 51d64 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 51d68 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51d6d jmp 0x51cde
            pc = 0x51cde; continue;
        }
        0x51d72 => {
            // block 51d72
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51d72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51d72 mov rbx,rax
            rbx = rax;
            // 51d75 jmp 0x51d4d
            pc = 0x51d4d; continue;
        }
        0x51d77 => {
            // block 51d77
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51d77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51d77 mov rbx,rax
            rbx = rax;
            // 51d7a jmp 0x51d36
            pc = 0x51d36; continue;
        }
        0x51d7c => {
            // block 51d7c
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51d7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51d7c mov rbx,rax
            rbx = rax;
            // 51d7f lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 51d83 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51d88 jmp 0x51d1e
            pc = 0x51d1e; continue;
        }
        0x51d8a => {
            // block 51d8a
            #[cfg(feature = "trace")] eprintln!("f_000516ec pc 51d8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51d8a mov rbx,rax
            rbx = rax;
            // 51d8d lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 51d94 jmp 0x51ce5
            pc = 0x51ce5; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
