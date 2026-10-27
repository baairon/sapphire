//! Mechanically lifted functions (scripts/lift_run.py), module h004. Each one is checked against the original
//! in the world harness (tests/world_h004.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xe99dc size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e99dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe99dc;
    loop {
        match pc {
        0xe99dc => {
            // block e99dc
            #[cfg(feature = "trace")] eprintln!("f_000e99dc pc e99dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e99dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e99dd mov rbp,rsp
            rbp = rsp;
            // e99e0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e99e2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e99e3 mov rbx,rdi
            rbx = rdi;
            // e99e6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e99e9 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // e99ed mov rdx,rdi
            rdx = rdi;
            // e99f0 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // e99f3 sar rdx,0x3
            { let (r, f) = sar::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // e99f7 movabs rcx,0xaaaaaaaaaaaaaaab
            rcx = 0xaaaa_aaaa_aaaa_aaabu64;
            // e9a01 imul rcx,rdx
            { let (r, f) = imul::<64>(rcx, rdx); rcx = r; fl = f; }
            // e9a05 cmp rcx,rsi
            fl = sub::<64>(rcx, rsi).1;
            // e9a08 jae 0xe9a19
            if fl.ae() { pc = 0xe9a19; continue; }
            pc = 0xe9a0a;
        }
        0xe9a0a => {
            // block e9a0a
            #[cfg(feature = "trace")] eprintln!("f_000e99dc pc e9a0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9a0a sub rsi,rcx
            { let (r, f) = sub::<64>(rsi, rcx);
              rsi = r;
              fl = f; }
            // e9a0d mov rdi,rbx
            rdi = rbx;
            // e9a10 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9a11 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9a13 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9a14 jmp 0xebf1e
            { let r = { let t = m.env.stub_va(0x6eb); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0xe9a19 => {
            // block e9a19
            #[cfg(feature = "trace")] eprintln!("f_000e99dc pc e9a19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9a19 jbe 0xe9a3b
            if fl.be() { pc = 0xe9a3b; continue; }
            pc = 0xe9a1b;
        }
        0xe9a1b => {
            // block e9a1b
            #[cfg(feature = "trace")] eprintln!("f_000e99dc pc e9a1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9a1b lea rcx,[rsi+rsi*2]
            rcx = rsi.wrapping_add(rsi.wrapping_mul(2));
            // e9a1f lea r14,[rax+rcx*8]
            r14 = rax.wrapping_add(rcx.wrapping_mul(8));
            // e9a23 jmp 0xe9a36
            pc = 0xe9a36; continue;
        }
        0xe9a25 => {
            // block e9a25
            #[cfg(feature = "trace")] eprintln!("f_000e99dc pc e9a25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9a25 add rdi,0xffffffffffffffe8
            { let (r, f) = add::<64>(rdi, 0xffff_ffff_ffff_ffe8u64);
              rdi = r;
              fl = f; }
            // e9a29 mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // e9a2d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9a32 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0xe9a36;
        }
        0xe9a36 => {
            // block e9a36
            #[cfg(feature = "trace")] eprintln!("f_000e99dc pc e9a36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9a36 cmp rdi,r14
            fl = sub::<64>(rdi, r14).1;
            // e9a39 jne 0xe9a25
            if fl.ne() { pc = 0xe9a25; continue; }
            pc = 0xe9a3b;
        }
        0xe9a3b => {
            // block e9a3b
            #[cfg(feature = "trace")] eprintln!("f_000e99dc pc e9a3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9a3b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9a3c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9a3e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9a3f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9548 size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9548<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9548;
    loop {
        match pc {
        0xb9548 => {
            // block b9548
            #[cfg(feature = "trace")] eprintln!("f_000b9548 pc b9548 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9548 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9549 mov rbp,rsp
            rbp = rsp;
            // b954c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b954e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b9550 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9551 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b9552 mov r15,rdi
            r15 = rdi;
            // b9555 lea rax,[r15+0x8]
            rax = r15.wrapping_add(0x8);
            // b9559 mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // b9561 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // b9569 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // b956c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // b956e lea r14,[rip+0x4946b] # 0x1029e0
            r14 = m.env.data_va(94637);
            pc = 0xb9575;
        }
        0xb9575 => {
            // block b9575
            #[cfg(feature = "trace")] eprintln!("f_000b9548 pc b9575 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9575 lea rsi,[r14+rbx*1]
            rsi = r14.wrapping_add(rbx);
            // b9579 mov rdi,r15
            rdi = r15;
            // b957c call 0xebce4
            { let r = { let t = m.env.stub_va(0x4b1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9581 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // b9584 cmp rbx,0x10
            fl = sub::<64>(rbx, 0x10u64).1;
            // b9588 jne 0xb9575
            if fl.ne() { pc = 0xb9575; continue; }
            pc = 0xb958a;
        }
        0xb958a => {
            // block b958a
            #[cfg(feature = "trace")] eprintln!("f_000b9548 pc b958a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b958a mov rax,r15
            rax = r15;
            // b958d add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b9591 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b9592 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9594 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9596 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9597 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb9598 => {
            // block b9598
            #[cfg(feature = "trace")] eprintln!("f_000b9548 pc b9598 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9598 mov r14,rax
            r14 = rax;
            // b959b mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // b959f mov rdi,r15
            rdi = r15;
            // b95a2 call 0xebcf0
            { let r = { let t = m.env.stub_va(0x4bd); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b95a7 mov rdi,r14
            rdi = r14;
            // b95aa call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9ab8 size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9ab8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe9ab8;
    loop {
        match pc {
        0xe9ab8 => {
            // block e9ab8
            #[cfg(feature = "trace")] eprintln!("f_000e9ab8 pc e9ab8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ab8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9ab9 mov rbp,rsp
            rbp = rsp;
            // e9abc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e9abe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9ac0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9ac1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e9ac2 mov rbx,rdi
            rbx = rdi;
            // e9ac5 mov r14,QWORD PTR [rbx+0x78]
            r14 = m.r64(rbx.wrapping_add(0x78)) as u64;
            // e9ac9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9acb test r14,r14
            fl = logic::<64>(r14 & r14);
            // e9ace je 0xe9b00
            if fl.e() { pc = 0xe9b00; continue; }
            pc = 0xe9ad0;
        }
        0xe9ad0 => {
            // block e9ad0
            #[cfg(feature = "trace")] eprintln!("f_000e9ab8 pc e9ad0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ad0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e9ad3 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // e9ad7 mov rdi,rbx
            rdi = rbx;
            // e9ada call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9adc mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // e9adf mov rdi,r14
            rdi = r14;
            // e9ae2 call 0xec1b8
            { let r = { let t = m.env.stub_va(0x985); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9ae7 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // e9ae9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9aeb test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // e9aed jne 0xe9b00
            if fl.ne() { pc = 0xe9b00; continue; }
            pc = 0xe9aef;
        }
        0xe9aef => {
            // block e9aef
            #[cfg(feature = "trace")] eprintln!("f_000e9ab8 pc e9aef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9aef xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9af1 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // e9af4 cmove rax,rbx
            if fl.e() { rax = rbx; }
            // e9af8 mov QWORD PTR [rbx+0x78],0x0
            m.w64(rbx.wrapping_add(0x78), (0x0u64) as u64);
            pc = 0xe9b00;
        }
        0xe9b00 => {
            // block e9b00
            #[cfg(feature = "trace")] eprintln!("f_000e9ab8 pc e9b00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9b00 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e9b04 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9b05 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9b07 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e9b09 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9b0a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe9b0b => {
            // block e9b0b
            #[cfg(feature = "trace")] eprintln!("f_000e9ab8 pc e9b0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9b0b mov rbx,rax
            rbx = rax;
            // e9b0e mov rdi,r14
            rdi = r14;
            // e9b11 call 0xec1b8
            { let r = { let t = m.env.stub_va(0x985); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9b16 mov rdi,rbx
            rdi = rbx;
            // e9b19 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc5762 size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c5762<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc5762;
    loop {
        match pc {
        0xc5762 => {
            // block c5762
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c5762 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5762 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c5763 mov rbp,rsp
            rbp = rsp;
            // c5766 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c5768 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c576a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c576b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c576c mov rbx,rsi
            rbx = rsi;
            // c576f mov r14,rdi
            r14 = rdi;
            // c5772 mov edi,0x28
            rdi = (0x28u64) as u32 as u64;
            // c5777 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c577c mov r15,rax
            r15 = rax;
            // c577f mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // c5781 mov DWORD PTR [r15+0x1c],eax
            m.w32(r15.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // c5785 mov ecx,DWORD PTR [rbx+0x4]
            rcx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // c5788 mov DWORD PTR [r15+0x20],ecx
            m.w32(r15.wrapping_add(0x20), ((rcx as u32 as u64)) as u32);
            // c578c mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // c5790 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // c5793 jne 0xc579e
            if fl.ne() { pc = 0xc579e; continue; }
            pc = 0xc5795;
        }
        0xc5795 => {
            // block c5795
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c5795 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5795 lea rsi,[r14+0x8]
            rsi = r14.wrapping_add(0x8);
            // c5799 jmp 0xc57bc
            pc = 0xc57bc; continue;
        }
        0xc579b => {
            // block c579b
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c579b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c579b mov rsi,rcx
            rsi = rcx;
            pc = 0xc579e;
        }
        0xc579e => {
            // block c579e
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c579e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c579e cmp eax,DWORD PTR [rsi+0x1c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rsi.wrapping_add(0x1c)) as u64).1;
            // c57a1 jge 0xc57ad
            if fl.ge() { pc = 0xc57ad; continue; }
            pc = 0xc57a3;
        }
        0xc57a3 => {
            // block c57a3
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c57a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c57a3 mov rcx,QWORD PTR [rsi]
            rcx = m.r64(rsi) as u64;
            // c57a6 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c57a9 jne 0xc579b
            if fl.ne() { pc = 0xc579b; continue; }
            pc = 0xc57ab;
        }
        0xc57ab => {
            // block c57ab
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c57ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c57ab jmp 0xc57bc
            pc = 0xc57bc; continue;
        }
        0xc57ad => {
            // block c57ad
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c57ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c57ad mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c57b1 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c57b4 jne 0xc579b
            if fl.ne() { pc = 0xc579b; continue; }
            pc = 0xc57b6;
        }
        0xc57b6 => {
            // block c57b6
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c57b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c57b6 lea rdx,[rsi+0x8]
            rdx = rsi.wrapping_add(0x8);
            // c57ba jmp 0xc57bf
            pc = 0xc57bf; continue;
        }
        0xc57bc => {
            // block c57bc
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c57bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c57bc mov rdx,rsi
            rdx = rsi;
            pc = 0xc57bf;
        }
        0xc57bf => {
            // block c57bf
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c57bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c57bf mov rdi,r14
            rdi = r14;
            // c57c2 mov rcx,r15
            rcx = r15;
            // c57c5 call 0xebdc2
            { let r = { let t = m.env.stub_va(0x58f); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c57ca mov rax,r15
            rax = r15;
            // c57cd add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c57d1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c57d2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c57d4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c57d6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c57d7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd9f72 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d9f72<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd9f72;
    loop {
        match pc {
        0xd9f72 => {
            // block d9f72
            #[cfg(feature = "trace")] eprintln!("f_000d9f72 pc d9f72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9f72 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d9f73 mov rbp,rsp
            rbp = rsp;
            // d9f76 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d9f78 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d9f7a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d9f7b sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d9f7f mov r14,rsi
            r14 = rsi;
            // d9f82 mov rbx,rdi
            rbx = rdi;
            // d9f85 lea rdi,[rip+0x5f0b4] # 0x139040
            rdi = m.env.data_va(317453);
            // d9f8c call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9f91 add rbx,0x380
            { let (r, f) = add::<64>(rbx, 0x380u64);
              rbx = r;
              fl = f; }
            // d9f98 mov rdi,r14
            rdi = r14;
            // d9f9b call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fa0 lea r15,[rbp-0x30]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d9fa4 mov rdi,r15
            rdi = r15;
            // d9fa7 mov rsi,r14
            rsi = r14;
            // d9faa mov rdx,rax
            rdx = rax;
            // d9fad call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fb2 mov rdi,rbx
            rdi = rbx;
            // d9fb5 mov rsi,r15
            rsi = r15;
            // d9fb8 call 0xe8100
            { let r = { let t = m.env.code_va(0xe8100); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fbd mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d9fbf lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d9fc3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fc8 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d9fca add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d9fce pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d9fcf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d9fd1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d9fd3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d9fd4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd9fd5 => {
            // block d9fd5
            #[cfg(feature = "trace")] eprintln!("f_000d9f72 pc d9fd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9fd5 mov rbx,rax
            rbx = rax;
            // d9fd8 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d9fdc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fe1 mov rdi,rbx
            rdi = rbx;
            // d9fe4 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0562 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0562<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut x1: u128 = fx[1] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe0562;
    loop {
        match pc {
        0xe0562 => {
            // block e0562
            #[cfg(feature = "trace")] eprintln!("f_000e0562 pc e0562 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0562 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0563 mov rbp,rsp
            rbp = rsp;
            // e0566 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e0568 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e056a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e056c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e056d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0571 movsd QWORD PTR [rbp-0x28],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), x1 as u64);
            // e0576 movsd QWORD PTR [rbp-0x30],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0 as u64);
            // e057b mov r14d,r9d
            r14 = ((r9 as u32 as u64)) as u32 as u64;
            // e057e mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // e0581 mov r15,rcx
            r15 = rcx;
            // e0584 mov rbx,rdi
            rbx = rdi;
            // e0587 lea rax,[rip+0x65c3a] # 0x1461c8
            rax = m.env.data_va(371093);
            // e058e mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e0591 mov QWORD PTR [rbx+0x8],rsi
            m.w64(rbx.wrapping_add(0x8), (rsi) as u64);
            // e0595 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // e0598 lea rdi,[rip+0x4b649] # 0x12bbe8
            rdi = m.env.data_va(263093);
            // e059f cmovne rdi,rdx
            if fl.ne() { rdi = rdx; }
            // e05a3 call 0xec2ba
            { let r = { let t = m.env.stub_va(0xa87); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // e05a8 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // e05ac mov DWORD PTR [rbx+0x18],r12d
            m.w32(rbx.wrapping_add(0x18), ((r12 as u32 as u64)) as u32);
            // e05b0 mov DWORD PTR [rbx+0x1c],r14d
            m.w32(rbx.wrapping_add(0x1c), ((r14 as u32 as u64)) as u32);
            // e05b4 movsd xmm0,QWORD PTR [rbp-0x30]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u128;
            // e05b9 movsd QWORD PTR [rbx+0x20],xmm0
            m.w64(rbx.wrapping_add(0x20), x0 as u64);
            // e05be movsd xmm0,QWORD PTR [rbp-0x28]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u128;
            // e05c3 movsd QWORD PTR [rbx+0x28],xmm0
            m.w64(rbx.wrapping_add(0x28), x0 as u64);
            // e05c8 mov QWORD PTR [rbx+0x30],r15
            m.w64(rbx.wrapping_add(0x30), (r15) as u64);
            // e05cc add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e05d0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e05d1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e05d3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e05d5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e05d7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e05d8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd65bc size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d65bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd65bc;
    loop {
        match pc {
        0xd65bc => {
            // block d65bc
            #[cfg(feature = "trace")] eprintln!("f_000d65bc pc d65bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d65bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d65bd mov rbp,rsp
            rbp = rsp;
            // d65c0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d65c2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d65c4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d65c5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d65c6 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d65c9 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // d65cd lea r15,[rdi+rax*1]
            r15 = rdi.wrapping_add(rax);
            // d65d1 mov rcx,QWORD PTR [rip+0x69de0] # 0x1403b8
            rcx = m.r64(m.env.data_va(347013)) as u64;
            // d65d8 lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // d65dc mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // d65e0 lea r14,[rdi+rax*1+0x78]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x78);
            // d65e5 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // d65e9 mov QWORD PTR [rdi+rax*1+0x78],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x78), (rcx) as u64);
            // d65ee lea rbx,[rdi+rax*1+0x10]
            rbx = rdi.wrapping_add(rax).wrapping_add(0x10);
            // d65f3 mov rcx,QWORD PTR [rip+0x69dae] # 0x1403a8
            rcx = m.r64(m.env.data_va(346997)) as u64;
            // d65fa add rcx,0x10
            { let (r, f) = add::<64>(rcx, 0x10u64);
              rcx = r;
              fl = f; }
            // d65fe mov QWORD PTR [rdi+rax*1+0x10],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x10), (rcx) as u64);
            // d6603 lea rdi,[rdi+rax*1+0x50]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x50);
            // d6608 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d660d mov rdi,rbx
            rdi = rbx;
            // d6610 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6615 mov rsi,QWORD PTR [rip+0x69be4] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // d661c add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // d6620 mov rdi,r15
            rdi = r15;
            // d6623 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6628 mov rdi,r14
            rdi = r14;
            // d662b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d662f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6630 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6632 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d6634 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6635 jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0a7c size=130
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0a7c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0a7c;
    loop {
        match pc {
        0xe0a7c => {
            // block e0a7c
            #[cfg(feature = "trace")] eprintln!("f_000e0a7c pc e0a7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a7c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0a7d mov rbp,rsp
            rbp = rsp;
            // e0a80 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0a81 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0a82 mov rbx,rdi
            rbx = rdi;
            // e0a85 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0a88 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0a8a call QWORD PTR [rax+0x1a48]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a48)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0a90 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0a93 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0a95 mov rdi,rbx
            rdi = rbx;
            // e0a98 call QWORD PTR [rax+0x1a50]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a50)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0a9e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0aa1 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // e0aa6 mov rdi,rbx
            rdi = rbx;
            // e0aa9 call QWORD PTR [rax+0x1a58]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a58)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0aaf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ab2 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // e0ab7 mov rdi,rbx
            rdi = rbx;
            // e0aba call QWORD PTR [rax+0x1a60]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a60)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ac0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ac3 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e0ac8 mov rdi,rbx
            rdi = rbx;
            // e0acb call QWORD PTR [rax+0x1a28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ad1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ad4 mov esi,0x8000
            rsi = (0x8000u64) as u32 as u64;
            // e0ad9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0adb mov rdi,rbx
            rdi = rbx;
            // e0ade call QWORD PTR [rax+0x1a98]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a98)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ae4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ae7 mov rax,QWORD PTR [rax+0x1aa8]
            rax = m.r64(rax.wrapping_add(0x1aa8)) as u64;
            // e0aee mov esi,0x10000
            rsi = (0x10000u64) as u32 as u64;
            // e0af3 mov rdi,rbx
            rdi = rbx;
            // e0af6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e0afa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0afb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0afc jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
