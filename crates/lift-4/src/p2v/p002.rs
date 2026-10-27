//! Mechanically lifted functions (scripts/lift_run.py), module p002. Each one is checked against the original
//! in the world harness (tests/world_p002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xe8294 size=124
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e8294<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe8294;
    loop {
        match pc {
        0xe8294 => {
            // block e8294
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e8294 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8294 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e8295 mov rbp,rsp
            rbp = rsp;
            // e8298 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e829a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e829c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e829d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e829e mov rbx,rsi
            rbx = rsi;
            // e82a1 mov r14,rdi
            r14 = rdi;
            // e82a4 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e82a7 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e82aa lea r15,[r14+0x8]
            r15 = r14.wrapping_add(0x8);
            // e82ae test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // e82b1 jne 0xe82b8
            if fl.ne() { pc = 0xe82b8; continue; }
            pc = 0xe82b3;
        }
        0xe82b3 => {
            // block e82b3
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82b3 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e82b6 jmp 0xe82bc
            pc = 0xe82bc; continue;
        }
        0xe82b8 => {
            // block e82b8
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82b8 mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0xe82bc;
        }
        0xe82bc => {
            // block e82bc
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82bc mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // e82c1 mov rdi,r15
            rdi = r15;
            // e82c4 mov rsi,rbx
            rsi = rbx;
            // e82c7 call 0xeb918
            { let r = { let t = m.env.stub_va(0xe5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e82cc mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // e82cf mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e82d3 test BYTE PTR [rax+r15*1+0x20],0x5
            fl = logic::<8>(m.r8(rax.wrapping_add(r15).wrapping_add(0x20)) as u64 & 0x5u64);
            // e82d9 je 0xe82df
            if fl.e() { pc = 0xe82df; continue; }
            pc = 0xe82db;
        }
        0xe82db => {
            // block e82db
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82db xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e82dd jmp 0xe8304
            pc = 0xe8304; continue;
        }
        0xe82df => {
            // block e82df
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82df mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e82e2 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // e82e7 mov rdi,r14
            rdi = r14;
            // e82ea call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e82ed mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e82f0 mov rdi,r14
            rdi = r14;
            // e82f3 call QWORD PTR [rax+0xa0]
            { let r = { let t = m.r64(rax.wrapping_add(0xa0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e82f9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e82fc mov rdi,r14
            rdi = r14;
            // e82ff call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8302 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe8304;
        }
        0xe8304 => {
            // block e8304
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e8304 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8304 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e8308 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e8309 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e830b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e830d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e830e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2bb44 size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002bb44<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2bb44;
    loop {
        match pc {
        0x2bb44 => {
            // block 2bb44
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb44 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bb45 mov rbp,rsp
            rbp = rsp;
            // 2bb48 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2bb4a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bb4c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2bb4e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bb4f mov r12,rsi
            r12 = rsi;
            // 2bb52 mov r14,rdi
            r14 = rdi;
            // 2bb55 lea rbx,[r12+0x8]
            rbx = r12.wrapping_add(0x8);
            // 2bb5a mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2bb5f mov rdi,r12
            rdi = r12;
            // 2bb62 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bb67 mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 2bb70 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 2bb74 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 2bb7d mov r15,QWORD PTR [r14+0x50]
            r15 = m.r64(r14.wrapping_add(0x50)) as u64;
            // 2bb81 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 2bb84 je 0x2bb9b
            if fl.e() { pc = 0x2bb9b; continue; }
            pc = 0x2bb86;
        }
        0x2bb86 => {
            // block 2bb86
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb86 cmp DWORD PTR [r14+0x58],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x58)) as u64, 0x0u64).1;
            // 2bb8b je 0x2bb9b
            if fl.e() { pc = 0x2bb9b; continue; }
            pc = 0x2bb8d;
        }
        0x2bb8d => {
            // block 2bb8d
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb8d mov eax,DWORD PTR [r14+0x4c]
            rax = (m.r32(r14.wrapping_add(0x4c)) as u64) as u32 as u64;
            // 2bb91 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2bb93 je 0x2bb9b
            if fl.e() { pc = 0x2bb9b; continue; }
            pc = 0x2bb95;
        }
        0x2bb95 => {
            // block 2bb95
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb95 cmp eax,DWORD PTR [r14+0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x20)) as u64).1;
            // 2bb99 jbe 0x2bba6
            if fl.be() { pc = 0x2bba6; continue; }
            pc = 0x2bb9b;
        }
        0x2bb9b => {
            // block 2bb9b
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb9b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bb9d;
        }
        0x2bb9d => {
            // block 2bb9d
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb9d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2bb9e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2bba0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2bba2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2bba4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2bba5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2bba6 => {
            // block 2bba6
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bba6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bba6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bba8;
        }
        0x2bba8 => {
            // block 2bba8
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bba8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bba8 mov esi,DWORD PTR [r15+rbx*4]
            rsi = (m.r32(r15.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2bbac mov rdi,r12
            rdi = r12;
            // 2bbaf call 0x60955
            { let r = { let t = m.env.code_va(0x60955); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bbb4 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 2bbb7 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2bbb9 cmp ebx,DWORD PTR [r14+0x58]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x58)) as u64).1;
            // 2bbbd jb 0x2bba8
            if fl.b() { pc = 0x2bba8; continue; }
            pc = 0x2bbbf;
        }
        0x2bbbf => {
            // block 2bbbf
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bbbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bbbf jmp 0x2bb9d
            pc = 0x2bb9d; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd648e size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d648e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd648e;
    loop {
        match pc {
        0xd648e => {
            // block d648e
            #[cfg(feature = "trace")] eprintln!("f_000d648e pc d648e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d648e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d648f mov rbp,rsp
            rbp = rsp;
            // d6492 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d6494 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d6496 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d6498 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6499 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d649d mov r12,rsi
            r12 = rsi;
            // d64a0 mov r14,rdi
            r14 = rdi;
            // d64a3 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // d64a7 mov rdx,r12
            rdx = r12;
            // d64aa call 0xebeb8
            { let r = { let t = m.env.stub_va(0x685); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d64af mov r15,rax
            r15 = rax;
            // d64b2 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // d64b5 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // d64b8 jne 0xd64f8
            if fl.ne() { pc = 0xd64f8; continue; }
            pc = 0xd64ba;
        }
        0xd64ba => {
            // block d64ba
            #[cfg(feature = "trace")] eprintln!("f_000d648e pc d64ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d64ba mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // d64bf call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d64c4 mov rbx,rax
            rbx = rax;
            // d64c7 mov eax,DWORD PTR [r12]
            rax = (m.r32(r12) as u64) as u32 as u64;
            // d64cb mov DWORD PTR [rbx+0x20],eax
            m.w32(rbx.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // d64ce mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // d64d6 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // d64de mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // d64e6 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // d64ea mov rdi,r14
            rdi = r14;
            // d64ed mov rdx,r15
            rdx = r15;
            // d64f0 mov rcx,rbx
            rcx = rbx;
            // d64f3 call 0xebec4
            { let r = { let t = m.env.stub_va(0x691); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd64f8;
        }
        0xd64f8 => {
            // block d64f8
            #[cfg(feature = "trace")] eprintln!("f_000d648e pc d64f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d64f8 add rbx,0x28
            { let (r, f) = add::<64>(rbx, 0x28u64);
              rbx = r;
              fl = f; }
            // d64fc mov rax,rbx
            rax = rbx;
            // d64ff add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d6503 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6504 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d6506 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6508 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d650a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d650b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e76e size=128
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e76e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e76e;
    loop {
        match pc {
        0x2e76e => {
            // block 2e76e
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e76e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e76e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e76f mov rbp,rsp
            rbp = rsp;
            // 2e772 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e774 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e776 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e777 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2e778 mov r14,rsi
            r14 = rsi;
            // 2e77b mov r15,rdi
            r15 = rdi;
            // 2e77e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e781 call QWORD PTR [rax+0x70]
            { let r = { let t = m.r64(rax.wrapping_add(0x70)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e784 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e786 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2e789 je 0x2e7e3
            if fl.e() { pc = 0x2e7e3; continue; }
            pc = 0x2e78b;
        }
        0x2e78b => {
            // block 2e78b
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e78b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e78b mov rbx,QWORD PTR [r15+0x10]
            rbx = m.r64(r15.wrapping_add(0x10)) as u64;
            // 2e78f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e791 cmp rbx,QWORD PTR [r15+0x18]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x18)) as u64).1;
            // 2e795 je 0x2e7e3
            if fl.e() { pc = 0x2e7e3; continue; }
            pc = 0x2e797;
        }
        0x2e797 => {
            // block 2e797
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e797 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e797 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 2e79a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 2e79d call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e7a0 mov rdi,r14
            rdi = r14;
            // 2e7a3 mov rsi,rax
            rsi = rax;
            // 2e7a6 call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e7ab test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e7ad je 0x2e7e0
            if fl.e() { pc = 0x2e7e0; continue; }
            pc = 0x2e7af;
        }
        0x2e7af => {
            // block 2e7af
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7af add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 2e7b3 cmp rbx,QWORD PTR [r15+0x18]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x18)) as u64).1;
            // 2e7b7 jne 0x2e797
            if fl.ne() { pc = 0x2e797; continue; }
            pc = 0x2e7b9;
        }
        0x2e7b9 => {
            // block 2e7b9
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7b9 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 2e7bd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e7bf cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 2e7c2 je 0x2e7e3
            if fl.e() { pc = 0x2e7e3; continue; }
            pc = 0x2e7c4;
        }
        0x2e7c4 => {
            // block 2e7c4
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7c4 lea rsi,[rip+0xfc3b5] # 0x12ab80
            rsi = m.env.data_va(258893);
            // 2e7cb mov rdi,r14
            rdi = r14;
            // 2e7ce call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e7d3 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 2e7d5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e7d7 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 2e7d9 jne 0x2e7e3
            if fl.ne() { pc = 0x2e7e3; continue; }
            pc = 0x2e7db;
        }
        0x2e7db => {
            // block 2e7db
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7db mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e7de jmp 0x2e7e3
            pc = 0x2e7e3; continue;
        }
        0x2e7e0 => {
            // block 2e7e0
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7e0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            pc = 0x2e7e3;
        }
        0x2e7e3 => {
            // block 2e7e3
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7e3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2e7e7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e7e8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e7ea pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e7ec pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e7ed ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x64996 size=129
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00064996<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x64996;
    loop {
        match pc {
        0x64996 => {
            // block 64996
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 64996 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64996 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 64997 mov rbp,rsp
            rbp = rsp;
            // 6499a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6499c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6499e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 649a0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 649a2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 649a3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 649a4 mov r14,rsi
            r14 = rsi;
            // 649a7 mov r12,rdi
            r12 = rdi;
            // 649aa mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // 649ad add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 649b1 cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 649b4 je 0x64a05
            if fl.e() { pc = 0x64a05; continue; }
            pc = 0x649b6;
        }
        0x649b6 => {
            // block 649b6
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649b6 lea r15,[rip+0xc5ae0] # 0x12a49d
            r15 = m.env.data_va(257130);
            // 649bd jmp 0x649d2
            pc = 0x649d2; continue;
        }
        0x649bf => {
            // block 649bf
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649bf mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 649c4 mov rdi,r12
            rdi = r12;
            // 649c7 mov rsi,r15
            rsi = r15;
            // 649ca call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 649cf mov r13,rbx
            r13 = rbx;
            pc = 0x649d2;
        }
        0x649d2 => {
            // block 649d2
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649d2 lea rsi,[r13+0x20]
            rsi = r13.wrapping_add(0x20);
            // 649d6 mov rdi,r12
            rdi = r12;
            // 649d9 call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 649de mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // 649e2 test rax,rax
            fl = logic::<64>(rax & rax);
            // 649e5 je 0x649f4
            if fl.e() { pc = 0x649f4; continue; }
            pc = 0x649e7;
        }
        0x649e7 => {
            // block 649e7
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649e7 mov rbx,rax
            rbx = rax;
            // 649ea mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 649ed test rax,rax
            fl = logic::<64>(rax & rax);
            // 649f0 jne 0x649e7
            if fl.ne() { pc = 0x649e7; continue; }
            pc = 0x649f2;
        }
        0x649f2 => {
            // block 649f2
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649f2 jmp 0x64a00
            pc = 0x64a00; continue;
        }
        0x649f4 => {
            // block 649f4
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649f4 mov rbx,QWORD PTR [r13+0x10]
            rbx = m.r64(r13.wrapping_add(0x10)) as u64;
            // 649f8 cmp QWORD PTR [rbx],r13
            fl = sub::<64>(m.r64(rbx) as u64, r13).1;
            // 649fb mov r13,rbx
            r13 = rbx;
            // 649fe jne 0x649f4
            if fl.ne() { pc = 0x649f4; continue; }
            pc = 0x64a00;
        }
        0x64a00 => {
            // block 64a00
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 64a00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a00 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 64a03 jne 0x649bf
            if fl.ne() { pc = 0x649bf; continue; }
            pc = 0x64a05;
        }
        0x64a05 => {
            // block 64a05
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 64a05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a05 mov rax,r12
            rax = r12;
            // 64a08 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 64a0c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 64a0d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 64a0f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 64a11 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 64a13 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 64a15 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 64a16 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5a600 size=132
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005a600<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5a600;
    loop {
        match pc {
        0x5a600 => {
            // block 5a600
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a600 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a600 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5a601 mov rbp,rsp
            rbp = rsp;
            // 5a604 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5a606 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5a608 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5a609 movzx r9d,cl
            r9 = ((rcx as u8 as u64)) as u32 as u64;
            // 5a60d mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 5a613 xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // 5a616 lea r11,[rip+0xa6733] # 0x100d50
            r11 = m.env.data_va(87325);
            // 5a61d lea rbx,[rip+0xa691c] # 0x100f40
            rbx = m.env.data_va(87821);
            // 5a624 lea r14,[rip+0xa6b05] # 0x101130
            r14 = m.env.data_va(88317);
            // 5a62b lea r15,[rip+0xa6d5e] # 0x101390
            r15 = m.env.data_va(88925);
            pc = 0x5a632;
        }
        0x5a632 => {
            // block 5a632
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a632 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a632 cmp DWORD PTR [r10+r11*1],edi
            fl = sub::<32>(m.r32(r10.wrapping_add(r11)) as u64, (rdi as u32 as u64)).1;
            // 5a636 jne 0x5a669
            if fl.ne() { pc = 0x5a669; continue; }
            pc = 0x5a638;
        }
        0x5a638 => {
            // block 5a638
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a638 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a638 cmp esi,DWORD PTR [r10+rbx*1+0x4]
            fl = sub::<32>((rsi as u32 as u64), m.r32(r10.wrapping_add(rbx).wrapping_add(0x4)) as u64).1;
            // 5a63d jne 0x5a669
            if fl.ne() { pc = 0x5a669; continue; }
            pc = 0x5a63f;
        }
        0x5a63f => {
            // block 5a63f
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a63f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a63f cmp edx,DWORD PTR [r10+r14*1]
            fl = sub::<32>((rdx as u32 as u64), m.r32(r10.wrapping_add(r14)) as u64).1;
            // 5a643 jne 0x5a669
            if fl.ne() { pc = 0x5a669; continue; }
            pc = 0x5a645;
        }
        0x5a645 => {
            // block 5a645
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a645 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a645 mov ecx,DWORD PTR [r10+r15*1]
            rcx = (m.r32(r10.wrapping_add(r15)) as u64) as u32 as u64;
            // 5a649 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 5a64b cmp ecx,0xa
            fl = sub::<32>((rcx as u32 as u64), 0xau64).1;
            // 5a64e ja 0x5a65c
            if fl.a() { pc = 0x5a65c; continue; }
            pc = 0x5a650;
        }
        0x5a650 => {
            // block 5a650
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a650 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a650 mov eax,0x7a9
            rax = (0x7a9u64) as u32 as u64;
            // 5a655 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 5a657 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a65a jmp 0x5a65e
            pc = 0x5a65e; continue;
        }
        0x5a65c => {
            // block 5a65c
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a65c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a65c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5a65e;
        }
        0x5a65e => {
            // block 5a65e
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a65e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a65e movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 5a661 xor eax,0x1
            { let r = (rax as u32 as u64) ^ 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a664 cmp r9d,eax
            fl = sub::<32>((r9 as u32 as u64), (rax as u32 as u64)).1;
            // 5a667 je 0x5a67a
            if fl.e() { pc = 0x5a67a; continue; }
            pc = 0x5a669;
        }
        0x5a669 => {
            // block 5a669
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a669 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a669 inc r8
            { let (r, f) = inc::<64>(r8, fl); r8 = r; fl = f; }
            // 5a66c add r10,0x4
            { let (r, f) = add::<64>(r10, 0x4u64);
              r10 = r;
              fl = f; }
            // 5a670 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a672 cmp r8,0x7a
            fl = sub::<64>(r8, 0x7au64).1;
            // 5a676 jl 0x5a632
            if fl.l() { pc = 0x5a632; continue; }
            pc = 0x5a678;
        }
        0x5a678 => {
            // block 5a678
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a678 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a678 jmp 0x5a67d
            pc = 0x5a67d; continue;
        }
        0x5a67a => {
            // block 5a67a
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a67a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a67a mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            pc = 0x5a67d;
        }
        0x5a67d => {
            // block 5a67d
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a67d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a67d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5a67e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5a680 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5a682 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a683 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2f1f8 size=138
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002f1f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2f1f8;
    loop {
        match pc {
        0x2f1f8 => {
            // block 2f1f8
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f1f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f1f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2f1f9 mov rbp,rsp
            rbp = rsp;
            // 2f1fc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2f1fe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2f1ff sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f203 mov rbx,rdi
            rbx = rdi;
            // 2f206 lea r14,[rbx+0x18]
            r14 = rbx.wrapping_add(0x18);
            // 2f20a mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2f20e mov rdi,r14
            rdi = r14;
            // 2f211 call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f217 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2f219 je 0x2f279
            if fl.e() { pc = 0x2f279; continue; }
            pc = 0x2f21b;
        }
        0x2f21b => {
            // block 2f21b
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f21b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f21b mov rdi,r14
            rdi = r14;
            // 2f21e call 0xb6914
            { let r = { let t = m.env.code_va(0xb6914); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f223 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2f226 cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // 2f22b jne 0x2f279
            if fl.ne() { pc = 0x2f279; continue; }
            pc = 0x2f22d;
        }
        0x2f22d => {
            // block 2f22d
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f22d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f22d mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2f231 mov esi,DWORD PTR [rbx+0x2b4]
            rsi = (m.r32(rbx.wrapping_add(0x2b4)) as u64) as u32 as u64;
            // 2f237 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2f239 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2f23e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2f241 mov rdi,r14
            rdi = r14;
            // 2f244 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f247 mov rdi,rbx
            rdi = rbx;
            // 2f24a call 0x31164
            { let r = { let t = m.env.code_va(0x31164); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f24f mov BYTE PTR [rbp-0x11],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffef), (0x0u64) as u8);
            // 2f253 lea rsi,[rbp-0x11]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffef);
            // 2f257 mov rdi,rbx
            rdi = rbx;
            // 2f25a call 0x311bc
            { let r = { let t = m.env.code_va(0x311bc); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f25f movzx esi,BYTE PTR [rbp-0x11]
            rsi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64) as u32 as u64;
            // 2f263 or esi,0x80
            { let r = (rsi as u32 as u64) | 0x80u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2f269 mov rdi,rbx
            rdi = rbx;
            // 2f26c add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f270 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f271 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f273 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f274 jmp 0x31368
            { let r = { let t = m.env.code_va(0x31368); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x2f279 => {
            // block 2f279
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f279 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f279 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f27d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f27e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f280 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f281 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15f84 size=142
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x78d
pub fn f_00015f84<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x15f84;
    loop {
        match pc {
        0x15f84 => {
            // block 15f84
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15f84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f84 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15f85 mov rbp,rsp
            rbp = rsp;
            // 15f88 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15f8a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15f8c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15f8e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15f90 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15f91 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 15f92 mov r14,rdx
            r14 = rdx;
            // 15f95 mov rbx,rsi
            rbx = rsi;
            // 15f98 mov r15,rdi
            r15 = rdi;
            // 15f9b mov r13,r14
            r13 = r14;
            // 15f9e sub r13,rbx
            { let (r, f) = sub::<64>(r13, rbx);
              r13 = r;
              fl = f; }
            // 15fa1 cmp r13,0xffffffffffffffef
            fl = sub::<64>(r13, 0xffff_ffff_ffff_ffefu64).1;
            // 15fa5 jbe 0x15fb1
            if fl.be() { pc = 0x15fb1; continue; }
            pc = 0x15fa7;
        }
        0x15fa7 => {
            // block 15fa7
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fa7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fa7 mov rdi,r15
            rdi = r15;
            // 15faa call 0xebfc0
            { let _ = { let t = m.env.stub_va(0x78d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x15fb1 => {
            // block 15fb1
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fb1 cmp r13,0x16
            fl = sub::<64>(r13, 0x16u64).1;
            // 15fb5 ja 0x15fc7
            if fl.a() { pc = 0x15fc7; continue; }
            pc = 0x15fb7;
        }
        0x15fb7 => {
            // block 15fb7
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fb7 lea rax,[r13+r13*1+0x0]
            rax = r13.wrapping_add(r13);
            // 15fbc mov BYTE PTR [r15],al
            m.w8(r15, ((rax as u8 as u64)) as u8);
            // 15fbf inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 15fc2 mov rax,r15
            rax = r15;
            // 15fc5 jmp 0x15fe6
            pc = 0x15fe6; continue;
        }
        0x15fc7 => {
            // block 15fc7
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fc7 lea r12,[r13+0x10]
            r12 = r13.wrapping_add(0x10);
            // 15fcb and r12,0xfffffffffffffff0
            { let r = r12 & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              r12 = r;
              fl = f; }
            // 15fcf mov rdi,r12
            rdi = r12;
            // 15fd2 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15fd7 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 15fdb or r12,0x1
            { let r = r12 | 0x1u64; let f = logic::<64>(r);
              r12 = r;
              fl = f; }
            // 15fdf mov QWORD PTR [r15],r12
            m.w64(r15, (r12) as u64);
            // 15fe2 mov QWORD PTR [r15+0x8],r13
            m.w64(r15.wrapping_add(0x8), (r13) as u64);
            pc = 0x15fe6;
        }
        0x15fe6 => {
            // block 15fe6
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fe6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fe6 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 15fe9 je 0x16000
            if fl.e() { pc = 0x16000; continue; }
            pc = 0x15feb;
        }
        0x15feb => {
            // block 15feb
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15feb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15feb mov rcx,rax
            rcx = rax;
            pc = 0x15fee;
        }
        0x15fee => {
            // block 15fee
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fee mov dl,BYTE PTR [rbx]
            rdx = (rdx & !0xff) | ((m.r8(rbx) as u64) as u8 as u64);
            // 15ff0 mov BYTE PTR [rcx],dl
            m.w8(rcx, ((rdx as u8 as u64)) as u8);
            // 15ff2 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 15ff5 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 15ff8 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // 15ffb jne 0x15fee
            if fl.ne() { pc = 0x15fee; continue; }
            pc = 0x15ffd;
        }
        0x15ffd => {
            // block 15ffd
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15ffd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ffd add rax,r13
            { let (r, f) = add::<64>(rax, r13);
              rax = r;
              fl = f; }
            pc = 0x16000;
        }
        0x16000 => {
            // block 16000
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 16000 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16000 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 16003 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 16007 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 16008 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1600a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1600c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1600e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 16010 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 16011 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x595ae size=149
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000595ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x595ae;
    loop {
        match pc {
        0x595ae => {
            // block 595ae
            #[cfg(feature = "trace")] eprintln!("f_000595ae pc 595ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 595ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 595af mov rbp,rsp
            rbp = rsp;
            // 595b2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 595b4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 595b6 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 595b8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 595ba push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 595bb sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 595bf mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // 595c2 mov DWORD PTR [rbp-0x6c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff94), ((rdx as u32 as u64)) as u32);
            // 595c5 mov rbx,rdi
            rbx = rdi;
            // 595c8 movzx eax,BYTE PTR [rbp+0x10]
            rax = (m.r8(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // 595cc movzx ecx,r8b
            rcx = ((r8 as u8 as u64)) as u32 as u64;
            // 595d0 movzx r8d,r9b
            r8 = ((r9 as u8 as u64)) as u32 as u64;
            // 595d4 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 595d8 mov r9d,eax
            r9 = ((rax as u32 as u64)) as u32 as u64;
            // 595db call 0x32c5c
            { let r = { let t = m.env.code_va(0x32c5c); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 595e0 cmp DWORD PTR [rbp-0x68],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, 0x0u64).1;
            // 595e4 je 0x59634
            if fl.e() { pc = 0x59634; continue; }
            pc = 0x595e6;
        }
        0x595e6 => {
            // block 595e6
            #[cfg(feature = "trace")] eprintln!("f_000595ae pc 595e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 595e6 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 595e9 and eax,0xffffff
            { let r = (rax as u32 as u64) & 0xffffffu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 595ee mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 595f0 shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 595f3 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 595f6 mov r14w,0x1
            r14 = (r14 & !0xffff) | ((0x1u64) as u16 as u64);
            // 595fb movzx r12d,cl
            r12 = ((rcx as u8 as u64)) as u32 as u64;
            // 595ff movzx r15d,r15b
            r15 = ((r15 as u8 as u64)) as u32 as u64;
            // 59603 movzx r13d,al
            r13 = ((rax as u8 as u64)) as u32 as u64;
            pc = 0x59607;
        }
        0x59607 => {
            // block 59607
            #[cfg(feature = "trace")] eprintln!("f_000595ae pc 59607 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59607 mov r8d,DWORD PTR [rbp-0x64]
            r8 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // 5960b mov rdi,rbx
            rdi = rbx;
            // 5960e mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 59611 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 59614 mov ecx,r13d
            rcx = ((r13 as u32 as u64)) as u32 as u64;
            // 59617 mov r9d,DWORD PTR [rbp-0x6c]
            r9 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // 5961b call 0x59541
            { let r = { let t = m.env.code_va(0x59541); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 59620 mov eax,DWORD PTR [rbp-0x60]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // 59623 lea rbx,[rbx+rax*4]
            rbx = rbx.wrapping_add(rax.wrapping_mul(4));
            // 59627 movzx eax,r14w
            rax = ((r14 as u16 as u64)) as u32 as u64;
            // 5962b lea r14d,[r14+0x1]
            r14 = (r14.wrapping_add(0x1)) as u32 as u64;
            // 5962f cmp eax,DWORD PTR [rbp-0x68]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64).1;
            // 59632 jb 0x59607
            if fl.b() { pc = 0x59607; continue; }
            pc = 0x59634;
        }
        0x59634 => {
            // block 59634
            #[cfg(feature = "trace")] eprintln!("f_000595ae pc 59634 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59634 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 59638 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 59639 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5963b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 5963d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5963f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 59641 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 59642 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1729c size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001729c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1729c;
    loop {
        match pc {
        0x1729c => {
            // block 1729c
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 1729c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1729c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1729d mov rbp,rsp
            rbp = rsp;
            // 172a0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 172a2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 172a3 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 172a7 mov r14,rsi
            r14 = rsi;
            // 172aa mov ebx,edi
            rbx = ((rdi as u32 as u64)) as u32 as u64;
            // 172ac lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 172b0 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 172b5 call 0x16080
            { let r = { let t = m.env.code_va(0x16080); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 172ba mov rax,QWORD PTR [rbp-0x28]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 172be mov rcx,QWORD PTR [rbp-0x20]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // 172c2 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 172c5 je 0x172d6
            if fl.e() { pc = 0x172d6; continue; }
            pc = 0x172c7;
        }
        0x172c7 => {
            // block 172c7
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172c7 cmp DWORD PTR [rax],ebx
            fl = sub::<32>(m.r32(rax) as u64, (rbx as u32 as u64)).1;
            // 172c9 je 0x172da
            if fl.e() { pc = 0x172da; continue; }
            pc = 0x172cb;
        }
        0x172cb => {
            // block 172cb
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172cb add rax,0xf8
            { let (r, f) = add::<64>(rax, 0xf8u64);
              rax = r;
              fl = f; }
            // 172d1 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 172d4 jne 0x172c7
            if fl.ne() { pc = 0x172c7; continue; }
            pc = 0x172d6;
        }
        0x172d6 => {
            // block 172d6
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172d6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 172d8 jmp 0x172f3
            pc = 0x172f3; continue;
        }
        0x172da => {
            // block 172da
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172da mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // 172dd mov rbx,QWORD PTR [rcx+0x10]
            rbx = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 172e1 movzx esi,WORD PTR [rax+0x4]
            rsi = (m.r16(rax.wrapping_add(0x4)) as u64) as u32 as u64;
            // 172e5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 172e7 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 172e9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 172ec mov rdi,r14
            rdi = r14;
            // 172ef call rbx
            { let r = { let t = rbx; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 172f1 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x172f3;
        }
        0x172f3 => {
            // block 172f3
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172f3 lea rax,[rip+0x12a11e] # 0x141418
            rax = m.env.data_va(351205);
            // 172fa mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 172fe lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 17302 call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17307 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 17309 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 1730d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1730e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 17310 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 17311 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x17312 => {
            // block 17312
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 17312 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17312 mov rbx,rax
            rbx = rax;
            // 17315 lea rax,[rip+0x12a0fc] # 0x141418
            rax = m.env.data_va(351205);
            // 1731c mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 17320 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 17324 call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17329 mov rdi,rbx
            rdi = rbx;
            // 1732c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc623e size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c623e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc623e;
    loop {
        match pc {
        0xc623e => {
            // block c623e
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c623e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c623e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c623f mov rbp,rsp
            rbp = rsp;
            // c6242 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c6244 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c6246 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c6248 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c624a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c624b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c624c mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // c624f mov rbx,rdi
            rbx = rdi;
            // c6252 lea rax,[rip+0x28c1] # 0xc8b1a
            rax = m.env.code_va(0xc8b1a);
            // c6259 lea r12,[rip+0x293d] # 0xc8b9d
            r12 = m.env.code_va(0xc8b9d);
            // c6260 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // c6263 cmovne r12,rax
            if fl.ne() { r12 = rax; }
            // c6267 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c626a jle 0xc62c5
            if fl.le() { pc = 0xc62c5; continue; }
            pc = 0xc626c;
        }
        0xc626c => {
            // block c626c
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c626c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c626c add ecx,ecx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rcx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c626e and ecx,0xfffffffc
            { let r = (rcx as u32 as u64) & 0xfffffffcu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c6271 movsxd rax,ecx
            rax = sx::<32>((rcx as u32 as u64)) as u64;
            // c6274 lea r13,[rsi+rax*2]
            r13 = rsi.wrapping_add(rax.wrapping_mul(2));
            // c6278 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc627b;
        }
        0xc627b => {
            // block c627b
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c627b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c627b mov rdi,rbx
            rdi = rbx;
            // c627e lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // c6282 call r12
            { let r = { let t = r12; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c6285 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // c6289 jne 0xc62ac
            if fl.ne() { pc = 0xc62ac; continue; }
            pc = 0xc628b;
        }
        0xc628b => {
            // block c628b
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c628b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c628b mov ax,WORD PTR [rbp-0x30]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u16 as u64);
            // c628f mov WORD PTR [r13+0x0],ax
            m.w16(r13, ((rax as u16 as u64)) as u16);
            // c6294 mov ax,WORD PTR [rbp-0x2e]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffd2)) as u64) as u16 as u64);
            // c6298 mov WORD PTR [r13+0x2],ax
            m.w16(r13.wrapping_add(0x2), ((rax as u16 as u64)) as u16);
            // c629d mov ax,WORD PTR [rbp-0x2c]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u16 as u64);
            // c62a1 mov WORD PTR [r13+0x4],ax
            m.w16(r13.wrapping_add(0x4), ((rax as u16 as u64)) as u16);
            // c62a6 add r13,0x6
            { let (r, f) = add::<64>(r13, 0x6u64);
              r13 = r;
              fl = f; }
            // c62aa jmp 0xc62b9
            pc = 0xc62b9; continue;
        }
        0xc62ac => {
            // block c62ac
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c62ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62ac mov ax,WORD PTR [rbp-0x2e]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffd2)) as u64) as u16 as u64);
            // c62b0 mov WORD PTR [r13+0x0],ax
            m.w16(r13, ((rax as u16 as u64)) as u16);
            // c62b5 add r13,0x2
            { let (r, f) = add::<64>(r13, 0x2u64);
              r13 = r;
              fl = f; }
            pc = 0xc62b9;
        }
        0xc62b9 => {
            // block c62b9
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c62b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62b9 add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // c62bd inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // c62c0 cmp r14d,r15d
            fl = sub::<32>((r14 as u32 as u64), (r15 as u32 as u64)).1;
            // c62c3 jne 0xc627b
            if fl.ne() { pc = 0xc627b; continue; }
            pc = 0xc62c5;
        }
        0xc62c5 => {
            // block c62c5
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c62c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62c5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c62c9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c62ca pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c62cc pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c62ce pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c62d0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c62d2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c62d3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x17202 size=154
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00017202<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x17202;
    loop {
        match pc {
        0x17202 => {
            // block 17202
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 17202 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17202 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 17203 mov rbp,rsp
            rbp = rsp;
            // 17206 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 17208 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 17209 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 1720d mov r14,rsi
            r14 = rsi;
            // 17210 mov ebx,edi
            rbx = ((rdi as u32 as u64)) as u32 as u64;
            // 17212 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 17216 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 1721b call 0x16080
            { let r = { let t = m.env.code_va(0x16080); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17220 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 17222 mov rcx,QWORD PTR [rbp-0x20]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // 17226 sub rcx,QWORD PTR [rbp-0x28]
            { let (r, f) = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64);
              rcx = r;
              fl = f; }
            // 1722a sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 1722e movabs rdx,0xef7bdef7bdef7bdf
            rdx = 0xef7b_def7_bdef_7bdfu64;
            // 17238 imul rdx,rcx
            { let (r, f) = imul::<64>(rdx, rcx); rdx = r; fl = f; }
            // 1723c cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 1723f jae 0x1727a
            if fl.ae() { pc = 0x1727a; continue; }
            pc = 0x17241;
        }
        0x17241 => {
            // block 17241
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 17241 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17241 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 17244 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 17248 movzx esi,bx
            rsi = ((rbx as u16 as u64)) as u32 as u64;
            // 1724b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1724d xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 1724f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 17252 mov rdi,r14
            rdi = r14;
            // 17255 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17257 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 17259 jmp 0x1727c
            pc = 0x1727c; continue;
        }
        0x1725b => {
            // block 1725b
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 1725b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1725b mov rbx,rax
            rbx = rax;
            // 1725e lea rax,[rip+0x12a1b3] # 0x141418
            rax = m.env.data_va(351205);
            // 17265 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 17269 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 1726d call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17272 mov rdi,rbx
            rdi = rbx;
            // 17275 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1727a;
        }
        0x1727a => {
            // block 1727a
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 1727a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1727a xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x1727c;
        }
        0x1727c => {
            // block 1727c
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 1727c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1727c lea rax,[rip+0x12a195] # 0x141418
            rax = m.env.data_va(351205);
            // 17283 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 17287 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 1728b call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17290 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 17292 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 17296 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 17297 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 17299 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1729a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x89c8 size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000089c8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x89c8;
    loop {
        match pc {
        0x89c8 => {
            // block 89c8
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 89c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 89c8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 89c9 mov rbp,rsp
            rbp = rsp;
            // 89cc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 89ce push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 89cf sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 89d3 mov rbx,rdi
            rbx = rdi;
            // 89d6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 89d9 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 89dd call QWORD PTR [rax+0x12b0]
            { let r = { let t = m.r64(rax.wrapping_add(0x12b0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 89e3 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 89e6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 89e8 je 0x8a58
            if fl.e() { pc = 0x8a58; continue; }
            pc = 0x89ea;
        }
        0x89ea => {
            // block 89ea
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 89ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 89ea test BYTE PTR [rbp-0x14],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64 & 0x1u64);
            // 89ee je 0x8a58
            if fl.e() { pc = 0x8a58; continue; }
            pc = 0x89f0;
        }
        0x89f0 => {
            // block 89f0
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 89f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 89f0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 89f3 mov rdi,rbx
            rdi = rbx;
            // 89f6 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 89fc mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 89fe call 0xce64c
            { let r = { let t = m.env.code_va(0xce64c); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8a03 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 8a06 jne 0x8a27
            if fl.ne() { pc = 0x8a27; continue; }
            pc = 0x8a08;
        }
        0x8a08 => {
            // block 8a08
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a08 mov edi,DWORD PTR [rbp-0x14]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 8a0b mov esi,edi
            rsi = ((rdi as u32 as u64)) as u32 as u64;
            // 8a0d shr esi,0x18
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x18u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 8a10 and esi,0x7
            { let r = (rsi as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 8a13 mov r14d,0x19
            r14 = (0x19u64) as u32 as u64;
            // 8a19 cmp esi,0x5
            fl = sub::<32>((rsi as u32 as u64), 0x5u64).1;
            // 8a1c je 0x8a58
            if fl.e() { pc = 0x8a58; continue; }
            pc = 0x8a1e;
        }
        0x8a1e => {
            // block 8a1e
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a1e shr edi,0x1c
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x1cu64, fl); rdi = (r) as u32 as u64; fl = f; }
            // 8a21 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 8a23 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 8a25 jmp 0x8a4d
            pc = 0x8a4d; continue;
        }
        0x8a27 => {
            // block 8a27
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a27 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 8a2a jb 0x8a58
            if fl.b() { pc = 0x8a58; continue; }
            pc = 0x8a2c;
        }
        0x8a2c => {
            // block 8a2c
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a2c mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 8a2f mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 8a31 shr edi,0x1c
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x1cu64, fl); rdi = (r) as u32 as u64; fl = f; }
            // 8a34 shr eax,0x4
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8a37 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 8a39 and esi,0xf
            { let r = (rsi as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 8a3c and eax,0xe
            { let r = (rax as u32 as u64) & 0xeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8a3f cmp eax,0x6
            fl = sub::<32>((rax as u32 as u64), 0x6u64).1;
            // 8a42 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 8a45 movzx ecx,al
            rcx = ((rax as u8 as u64)) as u32 as u64;
            // 8a48 shl ecx,0x3
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x3u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 8a4b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x8a4d;
        }
        0x8a4d => {
            // block 8a4d
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a4d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 8a50 call 0x6d8
            { let r = { let t = m.env.code_va(0x6d8); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8a55 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x8a58;
        }
        0x8a58 => {
            // block 8a58
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a58 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 8a5b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8a5f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8a60 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8a62 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8a63 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11282 size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011282<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x11282;
    loop {
        match pc {
        0x11282 => {
            // block 11282
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 11282 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11282 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11283 mov rbp,rsp
            rbp = rsp;
            // 11286 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 11288 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1128a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1128b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1128c mov r15,rdi
            r15 = rdi;
            // 1128f lea rax,[rip+0x13000a] # 0x1412a0
            rax = m.env.data_va(350829);
            // 11296 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 11299 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1129d mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 112a1 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 112a4 je 0x112c1
            if fl.e() { pc = 0x112c1; continue; }
            pc = 0x112a6;
        }
        0x112a6 => {
            // block 112a6
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112a6 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            // 112aa test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 112ad je 0x112b8
            if fl.e() { pc = 0x112b8; continue; }
            pc = 0x112af;
        }
        0x112af => {
            // block 112af
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112af mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 112b2 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 112b6 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x112b8;
        }
        0x112b8 => {
            // block 112b8
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112b8 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 112bc cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // 112bf jne 0x112a6
            if fl.ne() { pc = 0x112a6; continue; }
            pc = 0x112c1;
        }
        0x112c1 => {
            // block 112c1
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112c1 mov rdi,QWORD PTR [r15+0x40]
            rdi = m.r64(r15.wrapping_add(0x40)) as u64;
            // 112c5 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 112c8 je 0x112e8
            if fl.e() { pc = 0x112e8; continue; }
            pc = 0x112ca;
        }
        0x112ca => {
            // block 112ca
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112ca mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 112cd mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // 112d1 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 112d3 mov rdi,QWORD PTR [r15+0x40]
            rdi = m.r64(r15.wrapping_add(0x40)) as u64;
            // 112d7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 112da mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 112de call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 112e0 mov QWORD PTR [r15+0x40],0x0
            m.w64(r15.wrapping_add(0x40), (0x0u64) as u64);
            pc = 0x112e8;
        }
        0x112e8 => {
            // block 112e8
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112e8 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 112eb mov rax,QWORD PTR [rax+0x88]
            rax = m.r64(rax.wrapping_add(0x88)) as u64;
            // 112f2 mov rdi,r15
            rdi = r15;
            // 112f5 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 112f7 mov rdi,r15
            rdi = r15;
            // 112fa add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 112fe pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 112ff pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11301 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 11303 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11304 jmp 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x11309 => {
            // block 11309
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 11309 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11309 jmp 0x1130b
            pc = 0x1130b; continue;
        }
        0x1130b => {
            // block 1130b
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 1130b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1130b mov r14,rax
            r14 = rax;
            // 1130e mov rdi,r15
            rdi = r15;
            // 11311 call 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11316 mov rdi,r14
            rdi = r14;
            // 11319 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd5c8e size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d5c8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd5c8e;
    loop {
        match pc {
        0xd5c8e => {
            // block d5c8e
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5c8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5c8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5c8f mov rbp,rsp
            rbp = rsp;
            // d5c92 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5c93 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d5c94 mov rbx,rdi
            rbx = rdi;
            // d5c97 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // d5c9e mov DWORD PTR [rbp-0x10],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u32);
            // d5ca5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5ca8 lea rdx,[rbp-0x10]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff0);
            // d5cac mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // d5cb1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5cb6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5cb9 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5cbc mov ecx,DWORD PTR [rbp-0x10]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64) as u32 as u64;
            // d5cbf cmp ecx,0x106466ff
            fl = sub::<32>((rcx as u32 as u64), 0x106466ffu64).1;
            // d5cc5 jg 0xd5cfe
            if fl.g() { pc = 0xd5cfe; continue; }
            pc = 0xd5cc7;
        }
        0xd5cc7 => {
            // block d5cc7
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5cc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5cc7 cmp ecx,0x10518400
            fl = sub::<32>((rcx as u32 as u64), 0x10518400u64).1;
            // d5ccd je 0xd5cd7
            if fl.e() { pc = 0xd5cd7; continue; }
            pc = 0xd5ccf;
        }
        0xd5ccf => {
            // block d5ccf
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5ccf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ccf cmp ecx,0x10518450
            fl = sub::<32>((rcx as u32 as u64), 0x10518450u64).1;
            // d5cd5 jne 0xd5d21
            if fl.ne() { pc = 0xd5d21; continue; }
            pc = 0xd5cd7;
        }
        0xd5cd7 => {
            // block d5cd7
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5cd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5cd7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5cda lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // d5cde mov esi,0x40020
            rsi = (0x40020u64) as u32 as u64;
            // d5ce3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5ce8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5ceb mov rdi,rbx
            rdi = rbx;
            // d5cee call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5cf1 mov eax,DWORD PTR [rbp-0xc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // d5cf4 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d5cf6 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d5cf9 seta al
            rax = (rax & !0xff) | (((fl.a() as u64)) as u8 as u64);
            // d5cfc jmp 0xd5d23
            pc = 0xd5d23; continue;
        }
        0xd5cfe => {
            // block d5cfe
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5cfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5cfe mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d5d00 lea edx,[rcx-0x10646700]
            rdx = (rcx.wrapping_add(0xffff_ffff_ef9b_9900)) as u32 as u64;
            // d5d06 cmp edx,0x4
            fl = sub::<32>((rdx as u32 as u64), 0x4u64).1;
            // d5d09 jb 0xd5d23
            if fl.b() { pc = 0xd5d23; continue; }
            pc = 0xd5d0b;
        }
        0xd5d0b => {
            // block d5d0b
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d0b lea edx,[rcx-0x10646705]
            rdx = (rcx.wrapping_add(0xffff_ffff_ef9b_98fb)) as u32 as u64;
            // d5d11 cmp edx,0x2
            fl = sub::<32>((rdx as u32 as u64), 0x2u64).1;
            // d5d14 jb 0xd5d23
            if fl.b() { pc = 0xd5d23; continue; }
            pc = 0xd5d16;
        }
        0xd5d16 => {
            // block d5d16
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d16 add ecx,0xef8ef7b0
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xef8ef7b0u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d5d1c cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // d5d1f jb 0xd5d23
            if fl.b() { pc = 0xd5d23; continue; }
            pc = 0xd5d21;
        }
        0xd5d21 => {
            // block d5d21
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d21 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd5d23;
        }
        0xd5d23 => {
            // block d5d23
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d23 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d5d27 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5d28 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5d29 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x12785 size=159
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00012785<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x12785;
    loop {
        match pc {
        0x12785 => {
            // block 12785
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 12785 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12785 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12786 mov rbp,rsp
            rbp = rsp;
            // 12789 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1278b push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1278d push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1278f push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12791 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 12792 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 12793 mov r15,r9
            r15 = r9;
            // 12796 mov r14,rcx
            r14 = rcx;
            // 12799 mov r12,rsi
            r12 = rsi;
            // 1279c cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 1279f je 0x12815
            if fl.e() { pc = 0x12815; continue; }
            pc = 0x127a1;
        }
        0x127a1 => {
            // block 127a1
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127a1 lea r13,[rbp+0x20]
            r13 = rbp.wrapping_add(0x20);
            // 127a5 lea rax,[rbp+0x10]
            rax = rbp.wrapping_add(0x10);
            // 127a9 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 127ad mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            pc = 0x127b1;
        }
        0x127b1 => {
            // block 127b1
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127b1 cmp r15,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 127b5 je 0x127f0
            if fl.e() { pc = 0x127f0; continue; }
            pc = 0x127b7;
        }
        0x127b7 => {
            // block 127b7
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127b7 mov eax,DWORD PTR [r15-0x20]
            rax = (m.r32(r15.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // 127bb mov rbx,QWORD PTR [r13+0x8]
            rbx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 127bf add rbx,0xffffffffffffffc8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffc8u64);
              rbx = r;
              fl = f; }
            // 127c3 cmp eax,DWORD PTR [r12-0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(r12.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // 127c8 jge 0x127d6
            if fl.ge() { pc = 0x127d6; continue; }
            pc = 0x127ca;
        }
        0x127ca => {
            // block 127ca
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127ca add r12,0xffffffffffffffc8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffc8u64);
              r12 = r;
              fl = f; }
            // 127ce mov rdi,rbx
            rdi = rbx;
            // 127d1 mov rsi,r12
            rsi = r12;
            // 127d4 jmp 0x127e0
            pc = 0x127e0; continue;
        }
        0x127d6 => {
            // block 127d6
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127d6 add r15,0xffffffffffffffc8
            { let (r, f) = add::<64>(r15, 0xffff_ffff_ffff_ffc8u64);
              r15 = r;
              fl = f; }
            // 127da mov rdi,rbx
            rdi = rbx;
            // 127dd mov rsi,r15
            rsi = r15;
            pc = 0x127e0;
        }
        0x127e0 => {
            // block 127e0
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127e0 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 127e5 mov QWORD PTR [r13+0x8],rbx
            m.w64(r13.wrapping_add(0x8), (rbx) as u64);
            // 127e9 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 127ec jne 0x127b1
            if fl.ne() { pc = 0x127b1; continue; }
            pc = 0x127ee;
        }
        0x127ee => {
            // block 127ee
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127ee jmp 0x12815
            pc = 0x12815; continue;
        }
        0x127f0 => {
            // block 127f0
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127f0 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 127f3 je 0x12815
            if fl.e() { pc = 0x12815; continue; }
            pc = 0x127f5;
        }
        0x127f5 => {
            // block 127f5
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127f5 mov rbx,QWORD PTR [r13+0x8]
            rbx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 127f9 add rbx,0xffffffffffffffc8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffc8u64);
              rbx = r;
              fl = f; }
            pc = 0x127fd;
        }
        0x127fd => {
            // block 127fd
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127fd add r12,0xffffffffffffffc8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffc8u64);
              r12 = r;
              fl = f; }
            // 12801 mov rdi,rbx
            rdi = rbx;
            // 12804 mov rsi,r12
            rsi = r12;
            // 12807 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1280c add rbx,0xffffffffffffffc8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffc8u64);
              rbx = r;
              fl = f; }
            // 12810 cmp r14,r12
            fl = sub::<64>(r14, r12).1;
            // 12813 jne 0x127fd
            if fl.ne() { pc = 0x127fd; continue; }
            pc = 0x12815;
        }
        0x12815 => {
            // block 12815
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 12815 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12815 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 12819 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1281a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1281c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1281e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 12820 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 12822 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 12823 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x29d86 size=164
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00029d86<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x29d86;
    loop {
        match pc {
        0x29d86 => {
            // block 29d86
            #[cfg(feature = "trace")] eprintln!("f_00029d86 pc 29d86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d86 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 29d87 mov rbp,rsp
            rbp = rsp;
            // 29d8a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 29d8b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 29d8c mov rbx,rdi
            rbx = rdi;
            // 29d8f movabs rax,0x0
            rax = 0x0u64;
            // 29d99 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 29d9c mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 29da4 movabs rax,0x800000050
            rax = 0x0_0008_0000_0050u64;
            // 29dae mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 29db2 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // 29db6 mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // 29dbe mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 29dc6 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // 29dce mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 29dd6 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 29dde mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 29de6 mov esi,0xe0
            rsi = (0xe0u64) as u32 as u64;
            // 29deb call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29df0 mov rax,QWORD PTR [rbx+0x20]
            rax = m.r64(rbx.wrapping_add(0x20)) as u64;
            // 29df4 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            // 29df6 test rax,rax
            fl = logic::<64>(rax & rax);
            // 29df9 je 0x29e0a
            if fl.e() { pc = 0x29e0a; continue; }
            pc = 0x29dfb;
        }
        0x29dfb => {
            // block 29dfb
            #[cfg(feature = "trace")] eprintln!("f_00029d86 pc 29dfb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29dfb mov esi,DWORD PTR [rbx+0x28]
            rsi = (m.r32(rbx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 29dfe mov rdi,rax
            rdi = rax;
            // 29e01 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29e06 mov rdi,QWORD PTR [rbx+0x20]
            rdi = m.r64(rbx.wrapping_add(0x20)) as u64;
            pc = 0x29e0a;
        }
        0x29e0a => {
            // block 29e0a
            #[cfg(feature = "trace")] eprintln!("f_00029d86 pc 29e0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29e0a movabs rax,0x6e74763200000000
            rax = 0x6e74_7632_0000_0000u64;
            // 29e14 mov QWORD PTR [rbx+0x48],rax
            m.w64(rbx.wrapping_add(0x48), (rax) as u64);
            // 29e18 mov esi,0xe0
            rsi = (0xe0u64) as u32 as u64;
            // 29e1d call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29e22 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 29e26 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 29e27 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29e28 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2b8dc size=166
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002b8dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2b8dc;
    loop {
        match pc {
        0x2b8dc => {
            // block 2b8dc
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b8dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b8dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2b8dd mov rbp,rsp
            rbp = rsp;
            // 2b8e0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2b8e2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2b8e3 mov rbx,rsi
            rbx = rsi;
            // 2b8e6 mov r14,rdi
            r14 = rdi;
            // 2b8e9 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 2b8ed mov DWORD PTR [r14+0x20],eax
            m.w32(r14.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // 2b8f1 mov DWORD PTR [r14+0x4c],0x0
            m.w32(r14.wrapping_add(0x4c), (0x0u64) as u32);
            // 2b8f9 lea rdi,[r14+0x24]
            rdi = r14.wrapping_add(0x24);
            // 2b8fd shl rax,0x2
            { let (r, f) = shl::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 2b901 movabs rsi,0x3fffffffc
            rsi = 0x0_0003_ffff_fffcu64;
            // 2b90b and rsi,rax
            { let r = rsi & rax; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // 2b90e call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b913 lea rdi,[r14+0x50]
            rdi = r14.wrapping_add(0x50);
            // 2b917 mov esi,DWORD PTR [r14+0x20]
            rsi = (m.r32(r14.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2b91b shl rsi,0x2
            { let (r, f) = shl::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 2b91f call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b924 lea rdi,[r14+0x78]
            rdi = r14.wrapping_add(0x78);
            // 2b928 mov esi,DWORD PTR [r14+0x20]
            rsi = (m.r32(r14.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2b92c shl rsi,0x2
            { let (r, f) = shl::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 2b930 call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b935 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2b938 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 2b93c cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // 2b93f je 0x2b97b
            if fl.e() { pc = 0x2b97b; continue; }
            pc = 0x2b941;
        }
        0x2b941 => {
            // block 2b941
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b941 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b941 mov rcx,QWORD PTR [r14+0x24]
            rcx = m.r64(r14.wrapping_add(0x24)) as u64;
            // 2b945 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x2b947;
        }
        0x2b947 => {
            // block 2b947
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b947 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b947 mov esi,DWORD PTR [rax+0x1c]
            rsi = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 2b94a mov edi,edx
            rdi = ((rdx as u32 as u64)) as u32 as u64;
            // 2b94c mov DWORD PTR [rcx+rdi*4],esi
            m.w32(rcx.wrapping_add(rdi.wrapping_mul(4)), ((rsi as u32 as u64)) as u32);
            // 2b94f mov rsi,QWORD PTR [rax+0x8]
            rsi = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2b953 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 2b956 je 0x2b965
            if fl.e() { pc = 0x2b965; continue; }
            pc = 0x2b958;
        }
        0x2b958 => {
            // block 2b958
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b958 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b958 mov rax,rsi
            rax = rsi;
            // 2b95b mov rsi,QWORD PTR [rax]
            rsi = m.r64(rax) as u64;
            // 2b95e test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 2b961 jne 0x2b958
            if fl.ne() { pc = 0x2b958; continue; }
            pc = 0x2b963;
        }
        0x2b963 => {
            // block 2b963
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b963 jmp 0x2b974
            pc = 0x2b974; continue;
        }
        0x2b965 => {
            // block 2b965
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b965 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b965 mov rsi,rax
            rsi = rax;
            pc = 0x2b968;
        }
        0x2b968 => {
            // block 2b968
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b968 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b968 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // 2b96c cmp QWORD PTR [rax],rsi
            fl = sub::<64>(m.r64(rax) as u64, rsi).1;
            // 2b96f mov rsi,rax
            rsi = rax;
            // 2b972 jne 0x2b968
            if fl.ne() { pc = 0x2b968; continue; }
            pc = 0x2b974;
        }
        0x2b974 => {
            // block 2b974
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b974 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b974 inc edx
            { let (r, f) = inc::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // 2b976 cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // 2b979 jne 0x2b947
            if fl.ne() { pc = 0x2b947; continue; }
            pc = 0x2b97b;
        }
        0x2b97b => {
            // block 2b97b
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b97b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b97b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2b97d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2b97e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2b980 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2b981 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x12041 size=167
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00012041<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x12041;
    loop {
        match pc {
        0x12041 => {
            // block 12041
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12041 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12041 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12042 mov rbp,rsp
            rbp = rsp;
            // 12045 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12047 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12049 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1204b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1204d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1204e push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1204f mov r15,r8
            r15 = r8;
            // 12052 mov r14,rcx
            r14 = rcx;
            // 12055 mov rbx,rdx
            rbx = rdx;
            // 12058 mov r12,rsi
            r12 = rsi;
            // 1205b mov r13,rdi
            r13 = rdi;
            // 1205e jmp 0x12064
            pc = 0x12064; continue;
        }
        0x12060 => {
            // block 12060
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12060 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12060 add r15,0x38
            { let (r, f) = add::<64>(r15, 0x38u64);
              r15 = r;
              fl = f; }
            pc = 0x12064;
        }
        0x12064 => {
            // block 12064
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12064 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12064 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 12067 je 0x12099
            if fl.e() { pc = 0x12099; continue; }
            pc = 0x12069;
        }
        0x12069 => {
            // block 12069
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12069 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12069 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1206c je 0x120b8
            if fl.e() { pc = 0x120b8; continue; }
            pc = 0x1206e;
        }
        0x1206e => {
            // block 1206e
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 1206e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1206e mov eax,DWORD PTR [rbx+0x18]
            rax = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 12071 cmp eax,DWORD PTR [r13+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(r13.wrapping_add(0x18)) as u64).1;
            // 12075 jge 0x12088
            if fl.ge() { pc = 0x12088; continue; }
            pc = 0x12077;
        }
        0x12077 => {
            // block 12077
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12077 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12077 mov rdi,r15
            rdi = r15;
            // 1207a mov rsi,rbx
            rsi = rbx;
            // 1207d call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12082 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12086 jmp 0x12060
            pc = 0x12060; continue;
        }
        0x12088 => {
            // block 12088
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12088 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12088 mov rdi,r15
            rdi = r15;
            // 1208b mov rsi,r13
            rsi = r13;
            // 1208e call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12093 add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 12097 jmp 0x12060
            pc = 0x12060; continue;
        }
        0x12099 => {
            // block 12099
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12099 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12099 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1209c je 0x120d9
            if fl.e() { pc = 0x120d9; continue; }
            pc = 0x1209e;
        }
        0x1209e => {
            // block 1209e
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 1209e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1209e mov rdi,r15
            rdi = r15;
            // 120a1 mov rsi,rbx
            rsi = rbx;
            // 120a4 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 120a9 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 120ad add r15,0x38
            { let (r, f) = add::<64>(r15, 0x38u64);
              r15 = r;
              fl = f; }
            // 120b1 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // 120b4 jne 0x1209e
            if fl.ne() { pc = 0x1209e; continue; }
            pc = 0x120b6;
        }
        0x120b6 => {
            // block 120b6
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120b6 jmp 0x120d9
            pc = 0x120d9; continue;
        }
        0x120b8 => {
            // block 120b8
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120b8 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 120bb je 0x120d9
            if fl.e() { pc = 0x120d9; continue; }
            pc = 0x120bd;
        }
        0x120bd => {
            // block 120bd
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120bd sub r12,r13
            { let (r, f) = sub::<64>(r12, r13);
              r12 = r;
              fl = f; }
            // 120c0 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x120c2;
        }
        0x120c2 => {
            // block 120c2
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120c2 lea rsi,[r13+rbx*1+0x0]
            rsi = r13.wrapping_add(rbx);
            // 120c7 lea rdi,[r15+rbx*1]
            rdi = r15.wrapping_add(rbx);
            // 120cb call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 120d0 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 120d4 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // 120d7 jne 0x120c2
            if fl.ne() { pc = 0x120c2; continue; }
            pc = 0x120d9;
        }
        0x120d9 => {
            // block 120d9
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120d9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 120dd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 120de pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 120e0 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 120e2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 120e4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 120e6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 120e7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd432 size=170
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d432<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd432;
    loop {
        match pc {
        0xd432 => {
            // block d432
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d432 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d432 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d433 mov rbp,rsp
            rbp = rsp;
            // d436 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d438 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d43a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d43c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d43d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d441 mov r14,rdx
            r14 = rdx;
            // d444 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d446 mov r12,rdi
            r12 = rdi;
            // d449 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d44e call 0xd0ea0
            { let r = { let t = m.env.code_va(0xd0ea0); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d453 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d456 cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d458 jbe 0xd4cd
            if fl.be() { pc = 0xd4cd; continue; }
            pc = 0xd45a;
        }
        0xd45a => {
            // block d45a
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d45a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d45a mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // d461 mov r15d,0x1
            r15 = (0x1u64) as u32 as u64;
            // d467 test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // d46a je 0xd47a
            if fl.e() { pc = 0xd47a; continue; }
            pc = 0xd46c;
        }
        0xd46c => {
            // block d46c
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d46c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d46c movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // d46f mov r15d,0x9
            r15 = (0x9u64) as u32 as u64;
            // d475 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d478 jne 0xd4c9
            if fl.ne() { pc = 0xd4c9; continue; }
            pc = 0xd47a;
        }
        0xd47a => {
            // block d47a
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d47a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d47a mov BYTE PTR [rbp-0x25],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb), (0x0u64) as u8);
            // d47e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d482 lea rsi,[rbp-0x25]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdb);
            // d486 mov rdi,r12
            rdi = r12;
            // d489 call QWORD PTR [rax+0x1820]
            { let r = { let t = m.r64(rax.wrapping_add(0x1820)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d48f mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d491 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d493 cmp BYTE PTR [rbp-0x25],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb)) as u64, 0x0u64).1;
            // d497 je 0xd4cf
            if fl.e() { pc = 0xd4cf; continue; }
            pc = 0xd499;
        }
        0xd499 => {
            // block d499
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d499 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d499 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d49c jne 0xd4cf
            if fl.ne() { pc = 0xd4cf; continue; }
            pc = 0xd49e;
        }
        0xd49e => {
            // block d49e
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d49e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d4a2 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // d4a6 mov esi,0xe9
            rsi = (0xe9u64) as u32 as u64;
            // d4ab mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // d4b0 mov rdi,r12
            rdi = r12;
            // d4b3 mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // d4b6 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4b9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d4bb je 0xd4cd
            if fl.e() { pc = 0xd4cd; continue; }
            pc = 0xd4bd;
        }
        0xd4bd => {
            // block d4bd
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d4bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4bd mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // d4c0 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d4c2 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // d4c5 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d4c7 jmp 0xd4cf
            pc = 0xd4cf; continue;
        }
        0xd4c9 => {
            // block d4c9
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d4c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c9 mov BYTE PTR [rbp-0x25],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb), (0x0u64) as u8);
            pc = 0xd4cd;
        }
        0xd4cd => {
            // block d4cd
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d4cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4cd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd4cf;
        }
        0xd4cf => {
            // block d4cf
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d4cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4cf add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d4d3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4d4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d4d6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d4d8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d4da pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4db ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd61a size=170
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d61a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd61a;
    loop {
        match pc {
        0xd61a => {
            // block d61a
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d61a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d61a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d61b mov rbp,rsp
            rbp = rsp;
            // d61e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d620 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d622 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d624 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d625 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d629 mov r14,rdx
            r14 = rdx;
            // d62c mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d62e mov r12,rdi
            r12 = rdi;
            // d631 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d636 call 0xd0fb9
            { let r = { let t = m.env.code_va(0xd0fb9); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d63b movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d63e cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d640 jbe 0xd6b5
            if fl.be() { pc = 0xd6b5; continue; }
            pc = 0xd642;
        }
        0xd642 => {
            // block d642
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d642 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d642 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // d649 mov r15d,0x10
            r15 = (0x10u64) as u32 as u64;
            // d64f test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // d652 je 0xd662
            if fl.e() { pc = 0xd662; continue; }
            pc = 0xd654;
        }
        0xd654 => {
            // block d654
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d654 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d654 movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // d657 mov r15d,0x14
            r15 = (0x14u64) as u32 as u64;
            // d65d cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d660 jne 0xd6b1
            if fl.ne() { pc = 0xd6b1; continue; }
            pc = 0xd662;
        }
        0xd662 => {
            // block d662
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d662 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d662 mov BYTE PTR [rbp-0x25],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb), (0x0u64) as u8);
            // d666 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d66a lea rsi,[rbp-0x25]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdb);
            // d66e mov rdi,r12
            rdi = r12;
            // d671 call QWORD PTR [rax+0x1820]
            { let r = { let t = m.r64(rax.wrapping_add(0x1820)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d677 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d679 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d67b cmp BYTE PTR [rbp-0x25],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb)) as u64, 0x0u64).1;
            // d67f je 0xd6b7
            if fl.e() { pc = 0xd6b7; continue; }
            pc = 0xd681;
        }
        0xd681 => {
            // block d681
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d681 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d681 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d684 jne 0xd6b7
            if fl.ne() { pc = 0xd6b7; continue; }
            pc = 0xd686;
        }
        0xd686 => {
            // block d686
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d686 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d686 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d68a lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // d68e mov esi,0xe9
            rsi = (0xe9u64) as u32 as u64;
            // d693 mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // d698 mov rdi,r12
            rdi = r12;
            // d69b mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // d69e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6a3 je 0xd6b5
            if fl.e() { pc = 0xd6b5; continue; }
            pc = 0xd6a5;
        }
        0xd6a5 => {
            // block d6a5
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d6a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a5 mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // d6a8 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d6aa mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // d6ad mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d6af jmp 0xd6b7
            pc = 0xd6b7; continue;
        }
        0xd6b1 => {
            // block d6b1
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d6b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b1 mov BYTE PTR [rbp-0x25],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb), (0x0u64) as u8);
            pc = 0xd6b5;
        }
        0xd6b5 => {
            // block d6b5
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d6b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd6b7;
        }
        0xd6b7 => {
            // block d6b7
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d6b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d6bb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6bc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d6be pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6c0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d6c2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6c3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc13c size=170
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc13c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbc13c;
    loop {
        match pc {
        0xbc13c => {
            // block bc13c
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc13c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc13c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc13d mov rbp,rsp
            rbp = rsp;
            // bc140 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc142 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc144 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc146 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc147 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc14b mov r15,rsi
            r15 = rsi;
            // bc14e mov r14d,edi
            r14 = ((rdi as u32 as u64)) as u32 as u64;
            // bc151 mov DWORD PTR [rbp-0x24],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((r14 as u32 as u64)) as u32);
            // bc155 lea rbx,[r15+0x8]
            rbx = r15.wrapping_add(0x8);
            // bc159 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // bc15d mov rdi,r15
            rdi = r15;
            // bc160 call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc165 mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // bc16d mov QWORD PTR [r15],rbx
            m.w64(r15, (rbx) as u64);
            // bc170 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // bc178 lea rdi,[rip+0x10c521] # 0x1c86a0
            rdi = m.env.data_va(904813);
            // bc17f lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // bc183 call 0xebdc8
            { let r = { let t = m.env.stub_va(0x595); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc188 mov rbx,rax
            rbx = rax;
            // bc18b lea r12,[rip+0x10c516] # 0x1c86a8
            r12 = m.env.data_va(904821);
            // bc192 jmp 0xbc1cb
            pc = 0xbc1cb; continue;
        }
        0xbc194 => {
            // block bc194
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc194 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc194 cmp DWORD PTR [rbx+0x1c],r14d
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x1c)) as u64, (r14 as u32 as u64)).1;
            // bc198 jne 0xbc1d0
            if fl.ne() { pc = 0xbc1d0; continue; }
            pc = 0xbc19a;
        }
        0xbc19a => {
            // block bc19a
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc19a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc19a lea rsi,[rbx+0x20]
            rsi = rbx.wrapping_add(0x20);
            // bc19e mov rdi,r15
            rdi = r15;
            // bc1a1 call 0xebd98
            { let r = { let t = m.env.stub_va(0x565); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc1a6 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc1aa test rax,rax
            fl = logic::<64>(rax & rax);
            // bc1ad je 0xbc1bc
            if fl.e() { pc = 0xbc1bc; continue; }
            pc = 0xbc1af;
        }
        0xbc1af => {
            // block bc1af
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1af mov rbx,rax
            rbx = rax;
            // bc1b2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bc1b5 test rax,rax
            fl = logic::<64>(rax & rax);
            // bc1b8 jne 0xbc1af
            if fl.ne() { pc = 0xbc1af; continue; }
            pc = 0xbc1ba;
        }
        0xbc1ba => {
            // block bc1ba
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1ba jmp 0xbc1cb
            pc = 0xbc1cb; continue;
        }
        0xbc1bc => {
            // block bc1bc
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1bc mov rax,rbx
            rax = rbx;
            pc = 0xbc1bf;
        }
        0xbc1bf => {
            // block bc1bf
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1bf mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // bc1c3 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // bc1c6 mov rax,rbx
            rax = rbx;
            // bc1c9 jne 0xbc1bf
            if fl.ne() { pc = 0xbc1bf; continue; }
            pc = 0xbc1cb;
        }
        0xbc1cb => {
            // block bc1cb
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1cb cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // bc1ce jne 0xbc194
            if fl.ne() { pc = 0xbc194; continue; }
            pc = 0xbc1d0;
        }
        0xbc1d0 => {
            // block bc1d0
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1d0 cmp QWORD PTR [r15+0x10],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x0u64).1;
            // bc1d5 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc1d8 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc1dc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc1dd pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc1df pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc1e1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc1e3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc1e4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc4d4 size=170
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc4d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbc4d4;
    loop {
        match pc {
        0xbc4d4 => {
            // block bc4d4
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc4d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc4d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc4d5 mov rbp,rsp
            rbp = rsp;
            // bc4d8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc4da push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc4dc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc4de push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc4df sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc4e3 mov r15,rsi
            r15 = rsi;
            // bc4e6 mov r14d,edi
            r14 = ((rdi as u32 as u64)) as u32 as u64;
            // bc4e9 mov DWORD PTR [rbp-0x24],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((r14 as u32 as u64)) as u32);
            // bc4ed lea rbx,[r15+0x8]
            rbx = r15.wrapping_add(0x8);
            // bc4f1 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // bc4f5 mov rdi,r15
            rdi = r15;
            // bc4f8 call 0xebdb6
            { let r = { let t = m.env.stub_va(0x583); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc4fd mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // bc505 mov QWORD PTR [r15],rbx
            m.w64(r15, (rbx) as u64);
            // bc508 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // bc510 lea rdi,[rip+0x10c171] # 0x1c8688
            rdi = m.env.data_va(904789);
            // bc517 lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // bc51b call 0xebde0
            { let r = { let t = m.env.stub_va(0x5ad); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc520 mov rbx,rax
            rbx = rax;
            // bc523 lea r12,[rip+0x10c166] # 0x1c8690
            r12 = m.env.data_va(904797);
            // bc52a jmp 0xbc563
            pc = 0xbc563; continue;
        }
        0xbc52c => {
            // block bc52c
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc52c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc52c cmp DWORD PTR [rbx+0x1c],r14d
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x1c)) as u64, (r14 as u32 as u64)).1;
            // bc530 jne 0xbc568
            if fl.ne() { pc = 0xbc568; continue; }
            pc = 0xbc532;
        }
        0xbc532 => {
            // block bc532
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc532 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc532 lea rsi,[rbx+0x20]
            rsi = rbx.wrapping_add(0x20);
            // bc536 mov rdi,r15
            rdi = r15;
            // bc539 call 0xebdaa
            { let r = { let t = m.env.stub_va(0x577); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc53e mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc542 test rax,rax
            fl = logic::<64>(rax & rax);
            // bc545 je 0xbc554
            if fl.e() { pc = 0xbc554; continue; }
            pc = 0xbc547;
        }
        0xbc547 => {
            // block bc547
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc547 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc547 mov rbx,rax
            rbx = rax;
            // bc54a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bc54d test rax,rax
            fl = logic::<64>(rax & rax);
            // bc550 jne 0xbc547
            if fl.ne() { pc = 0xbc547; continue; }
            pc = 0xbc552;
        }
        0xbc552 => {
            // block bc552
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc552 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc552 jmp 0xbc563
            pc = 0xbc563; continue;
        }
        0xbc554 => {
            // block bc554
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc554 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc554 mov rax,rbx
            rax = rbx;
            pc = 0xbc557;
        }
        0xbc557 => {
            // block bc557
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc557 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc557 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // bc55b cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // bc55e mov rax,rbx
            rax = rbx;
            // bc561 jne 0xbc557
            if fl.ne() { pc = 0xbc557; continue; }
            pc = 0xbc563;
        }
        0xbc563 => {
            // block bc563
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc563 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc563 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // bc566 jne 0xbc52c
            if fl.ne() { pc = 0xbc52c; continue; }
            pc = 0xbc568;
        }
        0xbc568 => {
            // block bc568
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc568 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc568 cmp QWORD PTR [r15+0x10],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x0u64).1;
            // bc56d setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc570 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc574 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc575 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc577 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc579 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc57b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc57c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3914 size=172
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00003914<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x3914;
    loop {
        match pc {
        0x3914 => {
            // block 3914
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 3914 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3914 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3915 mov rbp,rsp
            rbp = rsp;
            // 3918 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 391a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 391c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 391e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 3920 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3921 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 3922 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 3925 mov r12,rdi
            r12 = rdi;
            // 3928 shl r14d,0xe
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0xeu64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 392c and r14d,0x3fffc000
            { let r = (r14 as u32 as u64) & 0x3fffc000u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 3933 je 0x39ab
            if fl.e() { pc = 0x39ab; continue; }
            pc = 0x3935;
        }
        0x3935 => {
            // block 3935
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 3935 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3935 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 3938 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 393a xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x393d;
        }
        0x393d => {
            // block 393d
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 393d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 393d mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3941 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // 3946 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 394b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 394e mov rdi,r12
            rdi = r12;
            // 3951 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 3953 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3956 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 395a mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 395f mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 3964 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3969 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 396c mov rdi,r12
            rdi = r12;
            // 396f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3972 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3976 mov rdi,r12
            rdi = r12;
            // 3979 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 397f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3983 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // 3988 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 398d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3990 mov rdi,r12
            rdi = r12;
            // 3993 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 3997 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 399a cmp DWORD PTR [rbp-0x2c],0xffffffff
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64, 0xffffffffu64).1;
            // 399e jne 0x39ae
            if fl.ne() { pc = 0x39ae; continue; }
            pc = 0x39a0;
        }
        0x39a0 => {
            // block 39a0
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 39a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 39a0 inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 39a3 add ebx,0x4
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x4u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 39a6 cmp r15d,r14d
            fl = sub::<32>((r15 as u32 as u64), (r14 as u32 as u64)).1;
            // 39a9 jb 0x393d
            if fl.b() { pc = 0x393d; continue; }
            pc = 0x39ab;
        }
        0x39ab => {
            // block 39ab
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 39ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 39ab mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x39ae;
        }
        0x39ae => {
            // block 39ae
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 39ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 39ae mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 39b1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 39b5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 39b6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 39b8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 39ba pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 39bc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 39be pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 39bf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd386 size=172
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d386<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd386;
    loop {
        match pc {
        0xd386 => {
            // block d386
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d386 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d386 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d387 mov rbp,rsp
            rbp = rsp;
            // d38a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d38c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d38e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d390 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d391 mov r14,rdx
            r14 = rdx;
            // d394 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d396 mov r12,rdi
            r12 = rdi;
            // d399 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d39e call 0xd0ea0
            { let r = { let t = m.env.code_va(0xd0ea0); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3a3 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d3a6 cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d3a8 jbe 0xd3cb
            if fl.be() { pc = 0xd3cb; continue; }
            pc = 0xd3aa;
        }
        0xd3aa => {
            // block d3aa
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3aa test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // d3ad je 0xd3cf
            if fl.e() { pc = 0xd3cf; continue; }
            pc = 0xd3af;
        }
        0xd3af => {
            // block d3af
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3af movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // d3b2 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3b4 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d3b7 mov esi,0xfe
            rsi = (0xfeu64) as u32 as u64;
            // d3bc cmovne esi,ecx
            if fl.ne() { rsi = ((rcx as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // d3bf mov r15d,0xff
            r15 = (0xffu64) as u32 as u64;
            // d3c5 cmovne r15d,ecx
            if fl.ne() { r15 = ((rcx as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // d3c9 jmp 0xd3da
            pc = 0xd3da; continue;
        }
        0xd3cb => {
            // block d3cb
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cb xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3cd jmp 0xd426
            pc = 0xd426; continue;
        }
        0xd3cf => {
            // block d3cf
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cf mov r15d,0x71
            r15 = (0x71u64) as u32 as u64;
            // d3d5 mov esi,0x70
            rsi = (0x70u64) as u32 as u64;
            pc = 0xd3da;
        }
        0xd3da => {
            // block d3da
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3da mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            // d3e1 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3e3 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // d3e5 je 0xd426
            if fl.e() { pc = 0xd426; continue; }
            pc = 0xd3e7;
        }
        0xd3e7 => {
            // block d3e7
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3e7 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // d3ea je 0xd426
            if fl.e() { pc = 0xd426; continue; }
            pc = 0xd3ec;
        }
        0xd3ec => {
            // block d3ec
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3ec lea rdx,[r14+0x4]
            rdx = r14.wrapping_add(0x4);
            // d3f0 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d3f4 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3f6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d3fb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d3fe mov rdi,r12
            rdi = r12;
            // d401 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d404 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d406 je 0xd426
            if fl.e() { pc = 0xd426; continue; }
            pc = 0xd408;
        }
        0xd408 => {
            // block d408
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d408 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d408 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // d40c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d410 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d415 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d418 mov rdi,r12
            rdi = r12;
            // d41b mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // d41e mov rdx,r14
            rdx = r14;
            // d421 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d424 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0xd426;
        }
        0xd426 => {
            // block d426
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d426 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d426 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d428 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d429 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d42b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d42d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d42f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d430 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4dc size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d4dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd4dc;
    loop {
        match pc {
        0xd4dc => {
            // block d4dc
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d4dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4dd mov rbp,rsp
            rbp = rsp;
            // d4e0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d4e2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d4e4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d4e6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d4e7 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d4eb mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d4ee mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d4f0 mov r12,rdi
            r12 = rdi;
            // d4f3 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d4f8 call 0xd0ea0
            { let r = { let t = m.env.code_va(0xd0ea0); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4fd movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d500 cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d502 jbe 0xd57f
            if fl.be() { pc = 0xd57f; continue; }
            pc = 0xd504;
        }
        0xd504 => {
            // block d504
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d504 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d504 mov r15d,0x1
            r15 = (0x1u64) as u32 as u64;
            // d50a test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // d50d je 0xd51f
            if fl.e() { pc = 0xd51f; continue; }
            pc = 0xd50f;
        }
        0xd50f => {
            // block d50f
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d50f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d50f movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // d512 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d515 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d518 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // d51b lea r15d,[rax+rax*8]
            r15 = (rax.wrapping_add(rax.wrapping_mul(8))) as u32 as u64;
            pc = 0xd51f;
        }
        0xd51f => {
            // block d51f
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d51f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d51f mov BYTE PTR [rbp-0x21],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf), (0x0u64) as u8);
            // d523 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d527 mov rdi,r12
            rdi = r12;
            // d52a mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d52d call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d533 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d535 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d537 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // d53a je 0xd581
            if fl.e() { pc = 0xd581; continue; }
            pc = 0xd53c;
        }
        0xd53c => {
            // block d53c
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d53c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d53c test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d53e jne 0xd581
            if fl.ne() { pc = 0xd581; continue; }
            pc = 0xd540;
        }
        0xd540 => {
            // block d540
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d540 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d540 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d544 lea rsi,[rbp-0x21]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdf);
            // d548 mov rdi,r12
            rdi = r12;
            // d54b call QWORD PTR [rax+0x1820]
            { let r = { let t = m.r64(rax.wrapping_add(0x1820)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d551 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d553 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d555 cmp BYTE PTR [rbp-0x21],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64, 0x0u64).1;
            // d559 je 0xd581
            if fl.e() { pc = 0xd581; continue; }
            pc = 0xd55b;
        }
        0xd55b => {
            // block d55b
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d55b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d55b xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d55e jne 0xd581
            if fl.ne() { pc = 0xd581; continue; }
            pc = 0xd560;
        }
        0xd560 => {
            // block d560
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d560 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d564 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // d567 mov esi,0xe9
            rsi = (0xe9u64) as u32 as u64;
            // d56c mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // d571 mov rdi,r12
            rdi = r12;
            // d574 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d577 mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // d57a call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d57d jmp 0xd581
            pc = 0xd581; continue;
        }
        0xd57f => {
            // block d57f
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d57f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d57f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd581;
        }
        0xd581 => {
            // block d581
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d581 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d581 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d585 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d586 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d588 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d58a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d58c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d58d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43d98 size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043d98<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x43d98;
    loop {
        match pc {
        0x43d98 => {
            // block 43d98
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43d98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43d98 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43d99 mov rbp,rsp
            rbp = rsp;
            // 43d9c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 43d9e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43da0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 43da2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 43da4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43da5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 43da6 mov r14,rdx
            r14 = rdx;
            // 43da9 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 43dac mov r13,rdi
            r13 = rdi;
            // 43daf mov edi,DWORD PTR [r13+0x10]
            rdi = (m.r32(r13.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43db3 call 0xcc811
            { let r = { let t = m.env.code_va(0xcc811); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43db8 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 43dba xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43dbc cmp r15d,0x7
            fl = sub::<32>((r15 as u32 as u64), 0x7u64).1;
            // 43dc0 ja 0x43e3b
            if fl.a() { pc = 0x43e3b; continue; }
            pc = 0x43dc2;
        }
        0x43dc2 => {
            // block 43dc2
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43dc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43dc2 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 43dc4 je 0x43e3b
            if fl.e() { pc = 0x43e3b; continue; }
            pc = 0x43dc6;
        }
        0x43dc6 => {
            // block 43dc6
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43dc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43dc6 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 43dcd lea r12,[r14+0x8]
            r12 = r14.wrapping_add(0x8);
            // 43dd1 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 43dd4 lea rax,[rax+rax*2]
            rax = rax.wrapping_add(rax.wrapping_mul(2));
            // 43dd8 shl rax,0x5
            { let (r, f) = shl::<64>(rax, 0x5u64, fl); rax = r; fl = f; }
            // 43ddc lea rcx,[rip+0xbc2dd] # 0x1000c0
            rcx = m.env.data_va(84109);
            // 43de3 lea rbx,[rax+rcx*1+0x8]
            rbx = rax.wrapping_add(rcx).wrapping_add(0x8);
            // 43de8 mov r15d,0x1
            r15 = (0x1u64) as u32 as u64;
            pc = 0x43dee;
        }
        0x43dee => {
            // block 43dee
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43dee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43dee mov rdi,r14
            rdi = r14;
            // 43df1 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 43df5 call 0xebb40
            { let r = { let t = m.env.stub_va(0x30d); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43dfa cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 43dfd setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 43e00 mov r9,QWORD PTR [r13+0x0]
            r9 = m.r64(r13) as u64;
            // 43e04 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // 43e07 mov esi,DWORD PTR [rbx-0x8]
            rsi = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fff8)) as u64) as u32 as u64;
            // 43e0a mov ecx,DWORD PTR [rbx-0x4]
            rcx = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 43e0d mov r8d,DWORD PTR [rbx]
            r8 = (m.r32(rbx) as u64) as u32 as u64;
            // 43e10 mov rdi,r13
            rdi = r13;
            // 43e13 call QWORD PTR [r9+0x30]
            { let r = { let t = m.r64(r9.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43e17 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43e19 je 0x43e39
            if fl.e() { pc = 0x43e39; continue; }
            pc = 0x43e1b;
        }
        0x43e1b => {
            // block 43e1b
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e1b mov DWORD PTR [rbp-0x2c],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((r15 as u32 as u64)) as u32);
            // 43e1f cmp r15d,0x3f
            fl = sub::<32>((r15 as u32 as u64), 0x3fu64).1;
            // 43e23 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 43e25 ja 0x43e3b
            if fl.a() { pc = 0x43e3b; continue; }
            pc = 0x43e27;
        }
        0x43e27 => {
            // block 43e27
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e27 lea ecx,[r15-0x8]
            rcx = (r15.wrapping_add(0xffff_ffff_ffff_fff8)) as u32 as u64;
            // 43e2b inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 43e2e add rbx,0xc
            { let (r, f) = add::<64>(rbx, 0xcu64);
              rbx = r;
              fl = f; }
            // 43e32 cmp ecx,0x37
            fl = sub::<32>((rcx as u32 as u64), 0x37u64).1;
            // 43e35 ja 0x43dee
            if fl.a() { pc = 0x43dee; continue; }
            pc = 0x43e37;
        }
        0x43e37 => {
            // block 43e37
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e37 jmp 0x43e3b
            pc = 0x43e3b; continue;
        }
        0x43e39 => {
            // block 43e39
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e39 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x43e3b;
        }
        0x43e3b => {
            // block 43e3b
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e3b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43e3f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43e40 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 43e42 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 43e44 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43e46 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 43e48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43e49 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44320 size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044320<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44320;
    loop {
        match pc {
        0x44320 => {
            // block 44320
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 44320 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44320 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44321 mov rbp,rsp
            rbp = rsp;
            // 44324 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44326 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44328 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4432a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4432c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4432d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4432e mov r14,rcx
            r14 = rcx;
            // 44331 mov r12,rdx
            r12 = rdx;
            // 44334 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 44337 mov rbx,rdi
            rbx = rdi;
            // 4433a mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 4433d call 0xcf5fd
            { let r = { let t = m.env.code_va(0xcf5fd); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44342 mov DWORD PTR [r12],0x8
            m.w32(r12, (0x8u64) as u32);
            // 4434a mov DWORD PTR [r14],0x20
            m.w32(r14, (0x20u64) as u32);
            // 44351 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 44354 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 44357 jb 0x44368
            if fl.b() { pc = 0x44368; continue; }
            pc = 0x44359;
        }
        0x44359 => {
            // block 44359
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 44359 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44359 lea ecx,[r15*4+0x4]
            rcx = (r15.wrapping_mul(4).wrapping_add(0x4)) as u32 as u64;
            // 44361 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // 44364 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 44366 jbe 0x4437d
            if fl.be() { pc = 0x4437d; continue; }
            pc = 0x44368;
        }
        0x44368 => {
            // block 44368
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 44368 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44368 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x4436b;
        }
        0x4436b => {
            // block 4436b
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 4436b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4436b mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 4436e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 44372 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44373 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 44375 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 44377 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44379 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4437b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4437c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4437d => {
            // block 4437d
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 4437d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4437d mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 44384 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 44387 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 4438b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 4438e mov esi,0xbe
            rsi = (0xbeu64) as u32 as u64;
            // 44393 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 44398 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4439b mov rdi,rbx
            rdi = rbx;
            // 4439e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 443a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 443a3 je 0x4436b
            if fl.e() { pc = 0x4436b; continue; }
            pc = 0x443a5;
        }
        0x443a5 => {
            // block 443a5
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 443a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 443a5 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 443a8 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 443ab lea rdx,[rip+0xbc2de] # 0x100690
            rdx = m.env.data_va(85597);
            // 443b2 mov cl,BYTE PTR [rdx+rcx*4]
            rcx = (rcx & !0xff) | ((m.r8(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u8 as u64);
            // 443b5 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 443b7 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 443b9 and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 443bc mov DWORD PTR [rbp-0x2c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rcx as u32 as u64)) as u32);
            // 443bf shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 443c2 mov DWORD PTR [r12],ecx
            m.w32(r12, ((rcx as u32 as u64)) as u32);
            // 443c6 and eax,0x3
            { let r = (rax as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 443c9 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 443cc mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // 443cf jmp 0x4436b
            pc = 0x4436b; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe08f8 size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e08f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe08f8;
    loop {
        match pc {
        0xe08f8 => {
            // block e08f8
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e08f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e08f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e08f9 mov rbp,rsp
            rbp = rsp;
            // e08fc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e08fe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0900 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e0902 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e0904 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0905 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0906 mov r14,rsi
            r14 = rsi;
            // e0909 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e090b test r14,r14
            fl = logic::<64>(r14 & r14);
            // e090e je 0xe0997
            if fl.e() { pc = 0xe0997; continue; }
            pc = 0xe0914;
        }
        0xe0914 => {
            // block e0914
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0914 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0914 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0917 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e091a mov r15,QWORD PTR [rax]
            r15 = m.r64(rax) as u64;
            // e091d mov rcx,QWORD PTR [rax+0x8]
            rcx = m.r64(rax.wrapping_add(0x8)) as u64;
            // e0921 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0923 mov rax,rcx
            rax = rcx;
            // e0926 sub rax,r15
            { let (r, f) = sub::<64>(rax, r15);
              rax = r;
              fl = f; }
            // e0929 je 0xe0997
            if fl.e() { pc = 0xe0997; continue; }
            pc = 0xe092b;
        }
        0xe092b => {
            // block e092b
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e092b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e092b mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // e092f sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e0933 movabs r13,0x6db6db6db6db6db7
            r13 = 0x6db6_db6d_b6db_6db7u64;
            // e093d imul r13,rax
            { let (r, f) = imul::<64>(r13, rax); r13 = r; fl = f; }
            // e0941 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e0944 mov rbx,r15
            rbx = r15;
            pc = 0xe0947;
        }
        0xe0947 => {
            // block e0947
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0947 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0947 mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e094b mov rsi,r14
            rsi = r14;
            // e094e call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0953 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e0955 je 0xe096d
            if fl.e() { pc = 0xe096d; continue; }
            pc = 0xe0957;
        }
        0xe0957 => {
            // block e0957
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0957 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0957 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // e095a add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // e095e cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // e0961 jb 0xe0947
            if fl.b() { pc = 0xe0947; continue; }
            pc = 0xe0963;
        }
        0xe0963 => {
            // block e0963
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0963 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0965 cmp QWORD PTR [rbp-0x30],r15
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, r15).1;
            // e0969 jne 0xe097b
            if fl.ne() { pc = 0xe097b; continue; }
            pc = 0xe096b;
        }
        0xe096b => {
            // block e096b
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e096b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e096b jmp 0xe0997
            pc = 0xe0997; continue;
        }
        0xe096d => {
            // block e096d
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e096d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e096d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // e0970 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e0974 jne 0xe0997
            if fl.ne() { pc = 0xe0997; continue; }
            pc = 0xe0976;
        }
        0xe0976 => {
            // block e0976
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0976 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0976 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // e0979 je 0xe0997
            if fl.e() { pc = 0xe0997; continue; }
            pc = 0xe097b;
        }
        0xe097b => {
            // block e097b
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e097b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e097b lea rsi,[rip+0x4a1fe] # 0x12ab80
            rsi = m.env.data_va(258893);
            // e0982 mov rdi,r14
            rdi = r14;
            // e0985 call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e098a xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e098c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e098e cmove rcx,r15
            if fl.e() { rcx = r15; }
            // e0992 mov rax,rcx
            rax = rcx;
            // e0995 jmp 0xe099a
            pc = 0xe099a; continue;
        }
        0xe0997 => {
            // block e0997
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0997 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0997 mov rax,rbx
            rax = rbx;
            pc = 0xe099a;
        }
        0xe099a => {
            // block e099a
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e099a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e099a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e099e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e099f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e09a1 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e09a3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e09a5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e09a7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e09a8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6c4 size=180
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d6c4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd6c4;
    loop {
        match pc {
        0xd6c4 => {
            // block d6c4
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d6c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6c5 mov rbp,rsp
            rbp = rsp;
            // d6c8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d6ca push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d6cc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d6ce push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6cf sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d6d3 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d6d6 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d6d8 mov r12,rdi
            r12 = rdi;
            // d6db mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d6e0 call 0xd0fb9
            { let r = { let t = m.env.code_va(0xd0fb9); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6e5 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d6e8 cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d6ea jbe 0xd769
            if fl.be() { pc = 0xd769; continue; }
            pc = 0xd6ec;
        }
        0xd6ec => {
            // block d6ec
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d6ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ec mov r15d,0x10
            r15 = (0x10u64) as u32 as u64;
            // d6f2 test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // d6f5 je 0xd709
            if fl.e() { pc = 0xd709; continue; }
            pc = 0xd6f7;
        }
        0xd6f7 => {
            // block d6f7
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d6f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6f7 movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // d6fa xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d6fc cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d6ff mov r15d,0x14
            r15 = (0x14u64) as u32 as u64;
            // d705 cmovne r15d,ecx
            if fl.ne() { r15 = ((rcx as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            pc = 0xd709;
        }
        0xd709 => {
            // block d709
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d709 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d709 mov BYTE PTR [rbp-0x21],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf), (0x0u64) as u8);
            // d70d mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d711 mov rdi,r12
            rdi = r12;
            // d714 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d717 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d71d mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d71f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d721 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // d724 je 0xd76b
            if fl.e() { pc = 0xd76b; continue; }
            pc = 0xd726;
        }
        0xd726 => {
            // block d726
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d726 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d726 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d728 jne 0xd76b
            if fl.ne() { pc = 0xd76b; continue; }
            pc = 0xd72a;
        }
        0xd72a => {
            // block d72a
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d72a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d72e lea rsi,[rbp-0x21]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdf);
            // d732 mov rdi,r12
            rdi = r12;
            // d735 call QWORD PTR [rax+0x1820]
            { let r = { let t = m.r64(rax.wrapping_add(0x1820)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d73b mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d73d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d73f cmp BYTE PTR [rbp-0x21],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64, 0x0u64).1;
            // d743 je 0xd76b
            if fl.e() { pc = 0xd76b; continue; }
            pc = 0xd745;
        }
        0xd745 => {
            // block d745
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d745 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d745 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d748 jne 0xd76b
            if fl.ne() { pc = 0xd76b; continue; }
            pc = 0xd74a;
        }
        0xd74a => {
            // block d74a
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d74a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d74a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d74e dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // d751 mov esi,0xe9
            rsi = (0xe9u64) as u32 as u64;
            // d756 mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // d75b mov rdi,r12
            rdi = r12;
            // d75e mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d761 mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // d764 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d767 jmp 0xd76b
            pc = 0xd76b; continue;
        }
        0xd769 => {
            // block d769
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d769 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d769 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd76b;
        }
        0xd76b => {
            // block d76b
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d76b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d76b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d76f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d770 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d772 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d774 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d776 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d777 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
