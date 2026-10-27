//! Mechanically lifted functions (scripts/lift_run.py), module q002. Each one is checked against the original
//! in the world harness (tests/world_q002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xa32 size=370
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000a32<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xa32;
    loop {
        match pc {
        0xa32 => {
            // block a32
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc a32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a32 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a33 mov rbp,rsp
            rbp = rsp;
            // a36 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // a38 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // a3a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // a3b sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // a3f mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // a42 mov rbx,rdi
            rbx = rdi;
            // a45 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a48 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4e mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // a51 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a54 lea rcx,[rbp-0x30]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a58 mov QWORD PTR [rsp],rcx
            m.w64(rsp, (rcx) as u64);
            // a5c lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // a60 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // a64 lea rcx,[rbp-0x24]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // a68 lea r8,[rbp-0x28]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // a6c lea r9,[rbp-0x2c]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // a70 mov rdi,rbx
            rdi = rbx;
            // a73 call QWORD PTR [rax+0xd10]
            { let r = { let t = m.r64(rax.wrapping_add(0xd10)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a79 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // a7b je 0xb26
            if fl.e() { pc = 0xb26; continue; }
            pc = 0xa81;
        }
        0xa81 => {
            // block a81
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc a81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a81 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // a84 call 0xce033
            { let r = { let t = m.env.code_va(0xce033); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a89 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // a8c neg ecx
            { let (r, f) = neg::<32>((rcx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // a8e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // a90 cmovne ecx,r14d
            if fl.ne() { rcx = ((r14 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // a94 add ecx,DWORD PTR [rbp-0x1c]
            { let (r, f) = add::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // a97 mov DWORD PTR [rbp-0x1c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rcx as u32 as u64)) as u32);
            // a9a mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // a9d cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // a9f jg 0xaa8
            if fl.g() { pc = 0xaa8; continue; }
            pc = 0xaa1;
        }
        0xaa1 => {
            // block aa1
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc aa1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa1 mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // aa4 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // aa6 jge 0xaab
            if fl.ge() { pc = 0xaab; continue; }
            pc = 0xaa8;
        }
        0xaa8 => {
            // block aa8
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc aa8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa8 mov DWORD PTR [rbp-0x1c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rax as u32 as u64)) as u32);
            pc = 0xaab;
        }
        0xaab => {
            // block aab
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc aab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // aae lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // ab2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ab4 mov rdi,rbx
            rdi = rbx;
            // ab7 call QWORD PTR [rax+0xd60]
            { let r = { let t = m.r64(rax.wrapping_add(0xd60)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abd mov eax,DWORD PTR [rbp-0x34]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // ac0 movzx ecx,ax
            rcx = ((rax as u16 as u64)) as u32 as u64;
            // ac3 mov esi,DWORD PTR [rbp-0x1c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // ac6 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // ac9 cmp ecx,esi
            fl = sub::<32>((rcx as u32 as u64), (rsi as u32 as u64)).1;
            // acb je 0xb95
            if fl.e() { pc = 0xb95; continue; }
            pc = 0xad1;
        }
        0xad1 => {
            // block ad1
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc ad1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad1 lea edx,[rcx+0x1]
            rdx = (rcx.wrapping_add(0x1)) as u32 as u64;
            // ad4 cmp edx,esi
            fl = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64)).1;
            // ad6 jne 0xb2b
            if fl.ne() { pc = 0xb2b; continue; }
            pc = 0xad8;
        }
        0xad8 => {
            // block ad8
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc ad8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad8 and eax,0xffff0000
            { let r = (rax as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // add add esi,0x2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ae0 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ae2 mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // ae5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ae8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aea mov rdi,rbx
            rdi = rbx;
            // aed call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af6 lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // afa mov rdi,rbx
            rdi = rbx;
            // afd call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03 lea r15,[rbp-0x3c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            pc = 0xb07;
        }
        0xb07 => {
            // block b07
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b07 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0a mov rdi,rbx
            rdi = rbx;
            // b0d mov rsi,r15
            rsi = r15;
            // b10 call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b16 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // b19 cmp eax,DWORD PTR [rbp-0x3c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64).1;
            // b1c jne 0xb07
            if fl.ne() { pc = 0xb07; continue; }
            pc = 0xb1e;
        }
        0xb1e => {
            // block b1e
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b1e mov esi,DWORD PTR [rbp-0x34]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // b21 add esi,0xfffffffe
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffffeu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b24 jmp 0xb84
            pc = 0xb84; continue;
        }
        0xb26 => {
            // block b26
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b26 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b29 jmp 0xb95
            pc = 0xb95; continue;
        }
        0xb2b => {
            // block b2b
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2b dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // b2d and eax,0xffff0000
            { let r = (rax as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b32 mov DWORD PTR [rbp-0x34],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rax as u32 as u64)) as u32);
            // b35 cmp ecx,esi
            fl = sub::<32>((rcx as u32 as u64), (rsi as u32 as u64)).1;
            // b37 jne 0xb82
            if fl.ne() { pc = 0xb82; continue; }
            pc = 0xb39;
        }
        0xb39 => {
            // block b39
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b39 add esi,0xfffffffe
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffffeu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b3c or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b3e mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // b41 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b44 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b46 mov rdi,rbx
            rdi = rbx;
            // b49 call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b52 lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b56 mov rdi,rbx
            rdi = rbx;
            // b59 call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5f lea r15,[rbp-0x3c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            pc = 0xb63;
        }
        0xb63 => {
            // block b63
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b66 mov rdi,rbx
            rdi = rbx;
            // b69 mov rsi,r15
            rsi = r15;
            // b6c call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b72 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // b75 cmp eax,DWORD PTR [rbp-0x3c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64).1;
            // b78 jne 0xb63
            if fl.ne() { pc = 0xb63; continue; }
            pc = 0xb7a;
        }
        0xb7a => {
            // block b7a
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a mov esi,DWORD PTR [rbp-0x34]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // b7d add esi,0x2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b80 jmp 0xb84
            pc = 0xb84; continue;
        }
        0xb82 => {
            // block b82
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b82 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0xb84;
        }
        0xb84 => {
            // block b84
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b84 mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // b87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b8a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b8c mov rdi,rbx
            rdi = rbx;
            // b8f call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb95;
        }
        0xb95 => {
            // block b95
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b95 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b98 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // b9c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b9d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d18 size=358
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002d18<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2d18;
    loop {
        match pc {
        0x2d18 => {
            // block 2d18
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d18 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d19 mov rbp,rsp
            rbp = rsp;
            // 2d1c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2d1e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2d20 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d21 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2d22 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 2d24 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 2d27 mov r15,rdi
            r15 = rdi;
            // 2d2a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2d2d mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 2d2f call QWORD PTR [rax+0x1d30]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d35 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2d37 je 0x2d40
            if fl.e() { pc = 0x2d40; continue; }
            pc = 0x2d39;
        }
        0x2d39 => {
            // block 2d39
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d39 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2d3b jmp 0x2e72
            pc = 0x2e72; continue;
        }
        0x2d40 => {
            // block 2d40
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d40 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2d43 mov rdi,r15
            rdi = r15;
            // 2d46 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d4c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2d4e je 0x2d65
            if fl.e() { pc = 0x2d65; continue; }
            pc = 0x2d50;
        }
        0x2d50 => {
            // block 2d50
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d50 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2d53 movzx ecx,bx
            rcx = ((rbx as u16 as u64)) as u32 as u64;
            // 2d56 lea rdx,[rip+0xfc3e3] # 0xff140
            rdx = m.env.data_va(80141);
            // 2d5d mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2d60 jmp 0x2e61
            pc = 0x2e61; continue;
        }
        0x2d65 => {
            // block 2d65
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d65 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2d68 mov rdi,r15
            rdi = r15;
            // 2d6b call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d71 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 2d73 call 0xcc7a3
            { let r = { let t = m.env.code_va(0xcc7a3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d78 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2d7a je 0x2e59
            if fl.e() { pc = 0x2e59; continue; }
            pc = 0x2d80;
        }
        0x2d80 => {
            // block 2d80
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d80 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2d83 mov rdi,r15
            rdi = r15;
            // 2d86 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d8c mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 2d8e call 0xcf034
            { let r = { let t = m.env.code_va(0xcf034); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d93 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 2d96 je 0x2e40
            if fl.e() { pc = 0x2e40; continue; }
            pc = 0x2d9c;
        }
        0x2d9c => {
            // block 2d9c
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2d9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d9c cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 2d9f je 0x2e0e
            if fl.e() { pc = 0x2e0e; continue; }
            pc = 0x2da1;
        }
        0x2da1 => {
            // block 2da1
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2da1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2da1 cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // 2da4 jne 0x2e59
            if fl.ne() { pc = 0x2e59; continue; }
            pc = 0x2daa;
        }
        0x2daa => {
            // block 2daa
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2daa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2daa mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2dad mov esi,0x1e4
            rsi = (0x1e4u64) as u32 as u64;
            // 2db2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2db7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2dba mov rdi,r15
            rdi = r15;
            // 2dbd mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2dc0 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dc3 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2dc6 mov esi,0x1e3
            rsi = (0x1e3u64) as u32 as u64;
            // 2dcb mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2dd0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2dd3 mov rdi,r15
            rdi = r15;
            // 2dd6 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2dd9 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ddc mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2ddf mov esi,0x1e2
            rsi = (0x1e2u64) as u32 as u64;
            // 2de4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2de9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2dec mov rdi,r15
            rdi = r15;
            // 2def mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2df2 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2df5 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2df8 mov esi,0x1e1
            rsi = (0x1e1u64) as u32 as u64;
            // 2dfd mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2e02 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2e05 mov rdi,r15
            rdi = r15;
            // 2e08 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2e0b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e0e;
        }
        0x2e0e => {
            // block 2e0e
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2e0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e0e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e11 mov esi,0x1e0
            rsi = (0x1e0u64) as u32 as u64;
            // 2e16 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2e1b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2e1e mov rdi,r15
            rdi = r15;
            // 2e21 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2e24 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e27 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e2a mov esi,0x1df
            rsi = (0x1dfu64) as u32 as u64;
            // 2e2f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2e34 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2e37 mov rdi,r15
            rdi = r15;
            // 2e3a mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2e3d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e40;
        }
        0x2e40 => {
            // block 2e40
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2e40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e40 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e43 mov esi,0x1de
            rsi = (0x1deu64) as u32 as u64;
            // 2e48 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2e4d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2e50 mov rdi,r15
            rdi = r15;
            // 2e53 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2e56 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e59;
        }
        0x2e59 => {
            // block 2e59
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2e59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e59 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e5c mov esi,0xe
            rsi = (0xeu64) as u32 as u64;
            pc = 0x2e61;
        }
        0x2e61 => {
            // block 2e61
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2e61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e61 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2e66 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2e69 mov rdi,r15
            rdi = r15;
            // 2e6c mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2e6f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e72;
        }
        0x2e72 => {
            // block 2e72
            #[cfg(feature = "trace")] eprintln!("f_00002d18 pc 2e72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e72 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2e76 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e77 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e79 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e7b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e7c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7b00 size=304
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007b00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x7b00;
    loop {
        match pc {
        0x7b00 => {
            // block 7b00
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7b01 mov rbp,rsp
            rbp = rsp;
            // 7b04 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7b06 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7b08 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 7b0a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 7b0c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7b0d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7b11 mov r15,rsi
            r15 = rsi;
            // 7b14 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 7b18 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 7b1b je 0x7c1b
            if fl.e() { pc = 0x7c1b; continue; }
            pc = 0x7b21;
        }
        0x7b21 => {
            // block 7b21
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b21 add r15,0x8
            { let (r, f) = add::<64>(r15, 0x8u64);
              r15 = r;
              fl = f; }
            // 7b25 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 7b2a mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 7b2e mov r14,QWORD PTR [rbp-0x38]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x7b32;
        }
        0x7b32 => {
            // block 7b32
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b32 movsd xmm0,QWORD PTR [r15-0x8]
            x0 = m.r64(r15.wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7b38 addsd xmm0,QWORD PTR [rip+0xf6d98] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 7b40 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 7b45 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 7b47 mov ebx,0x0
            rbx = (0x0u64) as u32 as u64;
            // 7b4c js 0x7b5d
            if fl.s() { pc = 0x7b5d; continue; }
            pc = 0x7b4e;
        }
        0x7b4e => {
            // block 7b4e
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b4e cmp eax,0x3ff
            fl = sub::<32>((rax as u32 as u64), 0x3ffu64).1;
            // 7b53 mov ecx,0x3ff
            rcx = (0x3ffu64) as u32 as u64;
            // 7b58 cmovg eax,ecx
            if fl.g() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 7b5b mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x7b5d;
        }
        0x7b5d => {
            // block 7b5d
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b5d mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7b61 lea r12,[rax+0x1ff]
            r12 = rax.wrapping_add(0x1ff);
            // 7b68 movsd xmm0,QWORD PTR [r15]
            x0 = m.r64(r15) as u128;
            // 7b6d addsd xmm0,QWORD PTR [rip+0xf6d63] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 7b75 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7b7a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7b7c test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7b7e js 0x7b90
            if fl.s() { pc = 0x7b90; continue; }
            pc = 0x7b80;
        }
        0x7b80 => {
            // block 7b80
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b80 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7b86 mov eax,0x3ff
            rax = (0x3ffu64) as u32 as u64;
            // 7b8b cmovg ecx,eax
            if fl.g() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7b8e mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7b90;
        }
        0x7b90 => {
            // block 7b90
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b90 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7b93 shl ebx,0x6
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x6u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 7b96 add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // 7b98 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7b9b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 7b9e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7ba3 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7ba6 mov rdi,r14
            rdi = r14;
            // 7ba9 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bac mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bae call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bb1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7bb3 je 0x7c1e
            if fl.e() { pc = 0x7c1e; continue; }
            pc = 0x7bb5;
        }
        0x7bb5 => {
            // block 7bb5
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7bb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bb5 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7bb8 add r12,0x200
            { let (r, f) = add::<64>(r12, 0x200u64);
              r12 = r;
              fl = f; }
            // 7bbf xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 7bc2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7bc7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7bca mov rdi,r14
            rdi = r14;
            // 7bcd mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bd0 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bd2 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bd5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7bd7 je 0x7c1e
            if fl.e() { pc = 0x7c1e; continue; }
            pc = 0x7bd9;
        }
        0x7bd9 => {
            // block 7bd9
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7bd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bd9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7bdc add r12d,0x200
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x200u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 7be3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7be8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7beb mov rdi,r14
            rdi = r14;
            // 7bee mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bf1 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bf3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bf6 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 7bf9 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7bfd cmp rax,0x1ff
            fl = sub::<64>(rax, 0x1ffu64).1;
            // 7c03 ja 0x7c1e
            if fl.a() { pc = 0x7c1e; continue; }
            pc = 0x7c05;
        }
        0x7c05 => {
            // block 7c05
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c05 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // 7c09 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 7c0c mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 7c10 test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // 7c13 jne 0x7b32
            if fl.ne() { pc = 0x7b32; continue; }
            pc = 0x7c19;
        }
        0x7c19 => {
            // block 7c19
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c19 jmp 0x7c1e
            pc = 0x7c1e; continue;
        }
        0x7c1b => {
            // block 7c1b
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c1b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x7c1e;
        }
        0x7c1e => {
            // block 7c1e
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c1e mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 7c21 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7c25 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7c26 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 7c28 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 7c2a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 7c2c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 7c2e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7c2f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x95bc size=320
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000095bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x95bc;
    loop {
        match pc {
        0x95bc => {
            // block 95bc
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 95bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 95bd mov rbp,rsp
            rbp = rsp;
            // 95c0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 95c2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 95c3 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 95c7 mov r14,rsi
            r14 = rsi;
            // 95ca mov rbx,rdi
            rbx = rdi;
            // 95cd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 95d0 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 95d4 call QWORD PTR [rax+0xde0]
            { let r = { let t = m.r64(rax.wrapping_add(0xde0)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 95da mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 95dd lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 95e1 mov rdi,rbx
            rdi = rbx;
            // 95e4 call QWORD PTR [rax+0xdc0]
            { let r = { let t = m.r64(rax.wrapping_add(0xdc0)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 95ea mov DWORD PTR [r14],0x25
            m.w32(r14, (0x25u64) as u32);
            // 95f1 mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 95f4 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // 95f8 jbe 0x9605
            if fl.be() { pc = 0x9605; continue; }
            pc = 0x95fa;
        }
        0x95fa => {
            // block 95fa
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 95fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95fa xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x95fc;
        }
        0x95fc => {
            // block 95fc
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 95fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95fc add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 9600 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 9601 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 9603 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 9604 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x9605 => {
            // block 9605
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9605 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9605 lea rcx,[rip+0xdc] # 0x96e8
            rcx = m.env.code_va(0x96e8);
            // 960c movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 9610 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 9613 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x9615 => {
            // block 9615
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9615 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9615 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 9617 mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 961a cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 961d jne 0x967c
            if fl.ne() { pc = 0x967c; continue; }
            pc = 0x961f;
        }
        0x961f => {
            // block 961f
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 961f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 961f mov DWORD PTR [r14],0x1
            m.w32(r14, (0x1u64) as u32);
            // 9626 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x9628 => {
            // block 9628
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9628 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9628 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 962a mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 962d cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 9630 jne 0x9691
            if fl.ne() { pc = 0x9691; continue; }
            pc = 0x9632;
        }
        0x9632 => {
            // block 9632
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9632 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9632 mov DWORD PTR [r14],0x3
            m.w32(r14, (0x3u64) as u32);
            // 9639 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x963b => {
            // block 963b
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 963b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 963b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 963d mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 9640 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 9643 je 0x96a6
            if fl.e() { pc = 0x96a6; continue; }
            pc = 0x9645;
        }
        0x9645 => {
            // block 9645
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9645 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9645 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // 9648 jne 0x96be
            if fl.ne() { pc = 0x96be; continue; }
            pc = 0x964a;
        }
        0x964a => {
            // block 964a
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 964a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 964a mov DWORD PTR [r14],0x5
            m.w32(r14, (0x5u64) as u32);
            // 9651 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x9653 => {
            // block 9653
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9653 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9653 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 9655 mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 9658 cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 965b je 0x96b2
            if fl.e() { pc = 0x96b2; continue; }
            pc = 0x965d;
        }
        0x965d => {
            // block 965d
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 965d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 965d cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // 9660 jne 0x96d2
            if fl.ne() { pc = 0x96d2; continue; }
            pc = 0x9662;
        }
        0x9662 => {
            // block 9662
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9662 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9662 mov DWORD PTR [r14],0x7
            m.w32(r14, (0x7u64) as u32);
            // 9669 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x966b => {
            // block 966b
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 966b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 966b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 966d cmp DWORD PTR [rbp-0x18],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64, 0x1u64).1;
            // 9671 jne 0x95fc
            if fl.ne() { pc = 0x95fc; continue; }
            pc = 0x9673;
        }
        0x9673 => {
            // block 9673
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9673 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9673 mov DWORD PTR [r14],0x1e
            m.w32(r14, (0x1eu64) as u32);
            // 967a jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x967c => {
            // block 967c
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 967c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 967c cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 967f jne 0x95fc
            if fl.ne() { pc = 0x95fc; continue; }
            pc = 0x9685;
        }
        0x9685 => {
            // block 9685
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9685 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9685 mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            // 968c jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x9691 => {
            // block 9691
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9691 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9691 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 9694 jne 0x95fc
            if fl.ne() { pc = 0x95fc; continue; }
            pc = 0x969a;
        }
        0x969a => {
            // block 969a
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 969a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 969a mov DWORD PTR [r14],0x2
            m.w32(r14, (0x2u64) as u32);
            // 96a1 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x96a6 => {
            // block 96a6
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96a6 mov DWORD PTR [r14],0x17
            m.w32(r14, (0x17u64) as u32);
            // 96ad jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x96b2 => {
            // block 96b2
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96b2 mov DWORD PTR [r14],0x18
            m.w32(r14, (0x18u64) as u32);
            // 96b9 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x96be => {
            // block 96be
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96be test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 96c0 jne 0x95fc
            if fl.ne() { pc = 0x95fc; continue; }
            pc = 0x96c6;
        }
        0x96c6 => {
            // block 96c6
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96c6 mov DWORD PTR [r14],0x4
            m.w32(r14, (0x4u64) as u32);
            // 96cd jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x96d2 => {
            // block 96d2
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96d2 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 96d4 jne 0x95fc
            if fl.ne() { pc = 0x95fc; continue; }
            pc = 0x96da;
        }
        0x96da => {
            // block 96da
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96da mov DWORD PTR [r14],0x6
            m.w32(r14, (0x6u64) as u32);
            // 96e1 jmp 0x95fc
            pc = 0x95fc; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10c76 size=366
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010c76<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x10c76;
    loop {
        match pc {
        0x10c76 => {
            // block 10c76
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10c76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c76 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10c77 mov rbp,rsp
            rbp = rsp;
            // 10c7a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10c7c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10c7e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 10c80 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 10c82 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10c83 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10c87 mov r14,rsi
            r14 = rsi;
            // 10c8a mov r15,rdi
            r15 = rdi;
            // 10c8d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 10c90 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10c94 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 10c97 jne 0x10dc9
            if fl.ne() { pc = 0x10dc9; continue; }
            pc = 0x10c9d;
        }
        0x10c9d => {
            // block 10c9d
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10c9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c9d mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10ca1 mov rcx,QWORD PTR [r15+0x18]
            rcx = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10ca5 cmp r12,rcx
            fl = sub::<64>(r12, rcx).1;
            // 10ca8 jae 0x10cf7
            if fl.ae() { pc = 0x10cf7; continue; }
            pc = 0x10caa;
        }
        0x10caa => {
            // block 10caa
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10caa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10caa sub rcx,r12
            { let (r, f) = sub::<64>(rcx, r12);
              rcx = r;
              fl = f; }
            // 10cad sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 10cb1 lea rax,[rcx+0x1]
            rax = rcx.wrapping_add(0x1);
            // 10cb5 shr rax,0x3f
            { let (r, f) = shr::<64>(rax, 0x3fu64, fl); rax = r; fl = f; }
            // 10cb9 lea rbx,[rcx+rax*1+0x1]
            rbx = rcx.wrapping_add(rax).wrapping_add(0x1);
            // 10cbe sar rbx,1
            { let (r, f) = sar::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 10cc1 mov rdx,r12
            rdx = r12;
            // 10cc4 sub rdx,r13
            { let (r, f) = sub::<64>(rdx, r13);
              rdx = r;
              fl = f; }
            // 10cc7 mov rax,rdx
            rax = rdx;
            // 10cca shr rax,0x3
            { let (r, f) = shr::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 10cce mov rcx,rbx
            rcx = rbx;
            // 10cd1 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 10cd4 lea r12,[r12+rcx*8]
            r12 = r12.wrapping_add(rcx.wrapping_mul(8));
            // 10cd8 mov rdi,r12
            rdi = r12;
            // 10cdb mov rsi,r13
            rsi = r13;
            // 10cde call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10ce3 mov QWORD PTR [r15+0x8],r12
            m.w64(r15.wrapping_add(0x8), (r12) as u64);
            // 10ce7 shl rbx,0x3
            { let (r, f) = shl::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // 10ceb add QWORD PTR [r15+0x10],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, rbx);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 10cef mov r13,r12
            r13 = r12;
            // 10cf2 jmp 0x10dc9
            pc = 0x10dc9; continue;
        }
        0x10cf7 => {
            // block 10cf7
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10cf7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10cf7 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10cfb mov QWORD PTR [rbp-0x58],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r14) as u64);
            // 10cff mov rax,rcx
            rax = rcx;
            // 10d02 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 10d05 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10d09 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 10d0f cmovne r14,rax
            if fl.ne() { r14 = rax; }
            // 10d13 lea rbx,[r14+r14*1+0x6]
            rbx = r14.wrapping_add(r14).wrapping_add(0x6);
            // 10d18 and rbx,0xfffffffffffffff8
            { let r = rbx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 10d1c lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 10d20 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 10d24 lea rdi,[r14*8+0x0]
            rdi = r14.wrapping_mul(8);
            // 10d2c mov QWORD PTR [rbp-0x68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rcx) as u64);
            // 10d30 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10d35 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 10d39 add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 10d3c mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // 10d40 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 10d44 lea rcx,[rax+r14*8]
            rcx = rax.wrapping_add(r14.wrapping_mul(8));
            // 10d48 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 10d4c cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 10d4f je 0x10d8e
            if fl.e() { pc = 0x10d8e; continue; }
            pc = 0x10d51;
        }
        0x10d51 => {
            // block 10d51
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10d51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10d51 mov r14,QWORD PTR [rbp-0x58]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x10d55;
        }
        0x10d55 => {
            // block 10d55
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10d55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10d55 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 10d59 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 10d5c mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 10d60 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 10d64 mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // 10d68 add r13,0x8
            { let (r, f) = add::<64>(r13, 0x8u64);
              r13 = r;
              fl = f; }
            // 10d6c cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 10d6f jne 0x10d55
            if fl.ne() { pc = 0x10d55; continue; }
            pc = 0x10d71;
        }
        0x10d71 => {
            // block 10d71
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10d71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10d71 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // 10d74 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10d78 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 10d7c mov rdx,QWORD PTR [rbp-0x48]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 10d80 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10d84 mov rsi,QWORD PTR [r15+0x18]
            rsi = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10d88 mov rcx,QWORD PTR [rbp-0x38]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 10d8c jmp 0x10d9d
            pc = 0x10d9d; continue;
        }
        0x10d8e => {
            // block 10d8e
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10d8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10d8e mov rdx,rbx
            rdx = rbx;
            // 10d91 mov r14,QWORD PTR [rbp-0x58]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 10d95 mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 10d99 mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            pc = 0x10d9d;
        }
        0x10d9d => {
            // block 10d9d
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10d9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10d9d mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 10da0 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 10da4 mov QWORD PTR [r15+0x8],rdx
            m.w64(r15.wrapping_add(0x8), (rdx) as u64);
            // 10da8 mov QWORD PTR [rbp-0x48],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r13) as u64);
            // 10dac mov QWORD PTR [r15+0x10],rbx
            m.w64(r15.wrapping_add(0x10), (rbx) as u64);
            // 10db0 mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 10db4 mov QWORD PTR [r15+0x18],rcx
            m.w64(r15.wrapping_add(0x18), (rcx) as u64);
            // 10db8 mov QWORD PTR [rbp-0x38],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rsi) as u64);
            // 10dbc lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10dc0 call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10dc5 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x10dc9;
        }
        0x10dc9 => {
            // block 10dc9
            #[cfg(feature = "trace")] eprintln!("f_00010c76 pc 10dc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10dc9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 10dcc mov QWORD PTR [r13-0x8],rax
            m.w64(r13.wrapping_add(0xffff_ffff_ffff_fff8), (rax) as u64);
            // 10dd0 add QWORD PTR [r15+0x8],0xfffffffffffffff8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, 0xffff_ffff_ffff_fff8u64);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 10dd5 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10dd9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 10dda pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 10ddc pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 10dde pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10de0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 10de2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10de3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10f58 size=366
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010f58<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x10f58;
    loop {
        match pc {
        0x10f58 => {
            // block 10f58
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 10f58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f58 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10f59 mov rbp,rsp
            rbp = rsp;
            // 10f5c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10f5e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10f60 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 10f62 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 10f64 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10f65 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10f69 mov r14,rsi
            r14 = rsi;
            // 10f6c mov r15,rdi
            r15 = rdi;
            // 10f6f mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 10f72 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10f76 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 10f79 jne 0x110ab
            if fl.ne() { pc = 0x110ab; continue; }
            pc = 0x10f7f;
        }
        0x10f7f => {
            // block 10f7f
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 10f7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f7f mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10f83 mov rcx,QWORD PTR [r15+0x18]
            rcx = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10f87 cmp r12,rcx
            fl = sub::<64>(r12, rcx).1;
            // 10f8a jae 0x10fd9
            if fl.ae() { pc = 0x10fd9; continue; }
            pc = 0x10f8c;
        }
        0x10f8c => {
            // block 10f8c
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 10f8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f8c sub rcx,r12
            { let (r, f) = sub::<64>(rcx, r12);
              rcx = r;
              fl = f; }
            // 10f8f sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 10f93 lea rax,[rcx+0x1]
            rax = rcx.wrapping_add(0x1);
            // 10f97 shr rax,0x3f
            { let (r, f) = shr::<64>(rax, 0x3fu64, fl); rax = r; fl = f; }
            // 10f9b lea rbx,[rcx+rax*1+0x1]
            rbx = rcx.wrapping_add(rax).wrapping_add(0x1);
            // 10fa0 sar rbx,1
            { let (r, f) = sar::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 10fa3 mov rdx,r12
            rdx = r12;
            // 10fa6 sub rdx,r13
            { let (r, f) = sub::<64>(rdx, r13);
              rdx = r;
              fl = f; }
            // 10fa9 mov rax,rdx
            rax = rdx;
            // 10fac shr rax,0x3
            { let (r, f) = shr::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 10fb0 mov rcx,rbx
            rcx = rbx;
            // 10fb3 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 10fb6 lea r12,[r12+rcx*8]
            r12 = r12.wrapping_add(rcx.wrapping_mul(8));
            // 10fba mov rdi,r12
            rdi = r12;
            // 10fbd mov rsi,r13
            rsi = r13;
            // 10fc0 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10fc5 mov QWORD PTR [r15+0x8],r12
            m.w64(r15.wrapping_add(0x8), (r12) as u64);
            // 10fc9 shl rbx,0x3
            { let (r, f) = shl::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // 10fcd add QWORD PTR [r15+0x10],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, rbx);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 10fd1 mov r13,r12
            r13 = r12;
            // 10fd4 jmp 0x110ab
            pc = 0x110ab; continue;
        }
        0x10fd9 => {
            // block 10fd9
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 10fd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10fd9 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10fdd mov QWORD PTR [rbp-0x58],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r14) as u64);
            // 10fe1 mov rax,rcx
            rax = rcx;
            // 10fe4 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 10fe7 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10feb mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 10ff1 cmovne r14,rax
            if fl.ne() { r14 = rax; }
            // 10ff5 lea rbx,[r14+r14*1+0x6]
            rbx = r14.wrapping_add(r14).wrapping_add(0x6);
            // 10ffa and rbx,0xfffffffffffffff8
            { let r = rbx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 10ffe mov rax,QWORD PTR [r15+0x20]
            rax = m.r64(r15.wrapping_add(0x20)) as u64;
            // 11002 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 11006 lea rdi,[r14*8+0x0]
            rdi = r14.wrapping_mul(8);
            // 1100e mov QWORD PTR [rbp-0x68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rcx) as u64);
            // 11012 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11017 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 1101b add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 1101e mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // 11022 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 11026 lea rcx,[rax+r14*8]
            rcx = rax.wrapping_add(r14.wrapping_mul(8));
            // 1102a mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 1102e cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 11031 je 0x11070
            if fl.e() { pc = 0x11070; continue; }
            pc = 0x11033;
        }
        0x11033 => {
            // block 11033
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 11033 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11033 mov r14,QWORD PTR [rbp-0x58]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x11037;
        }
        0x11037 => {
            // block 11037
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 11037 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11037 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 1103b mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 1103e mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 11042 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 11046 mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // 1104a add r13,0x8
            { let (r, f) = add::<64>(r13, 0x8u64);
              r13 = r;
              fl = f; }
            // 1104e cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 11051 jne 0x11037
            if fl.ne() { pc = 0x11037; continue; }
            pc = 0x11053;
        }
        0x11053 => {
            // block 11053
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 11053 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11053 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // 11056 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1105a mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 1105e mov rdx,QWORD PTR [rbp-0x48]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 11062 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 11066 mov rsi,QWORD PTR [r15+0x18]
            rsi = m.r64(r15.wrapping_add(0x18)) as u64;
            // 1106a mov rcx,QWORD PTR [rbp-0x38]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 1106e jmp 0x1107f
            pc = 0x1107f; continue;
        }
        0x11070 => {
            // block 11070
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 11070 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11070 mov rdx,rbx
            rdx = rbx;
            // 11073 mov r14,QWORD PTR [rbp-0x58]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 11077 mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1107b mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            pc = 0x1107f;
        }
        0x1107f => {
            // block 1107f
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 1107f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1107f mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 11082 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 11086 mov QWORD PTR [r15+0x8],rdx
            m.w64(r15.wrapping_add(0x8), (rdx) as u64);
            // 1108a mov QWORD PTR [rbp-0x48],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r13) as u64);
            // 1108e mov QWORD PTR [r15+0x10],rbx
            m.w64(r15.wrapping_add(0x10), (rbx) as u64);
            // 11092 mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 11096 mov QWORD PTR [r15+0x18],rcx
            m.w64(r15.wrapping_add(0x18), (rcx) as u64);
            // 1109a mov QWORD PTR [rbp-0x38],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rsi) as u64);
            // 1109e lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 110a2 call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 110a7 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x110ab;
        }
        0x110ab => {
            // block 110ab
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 110ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 110ab mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 110ae mov QWORD PTR [r13-0x8],rax
            m.w64(r13.wrapping_add(0xffff_ffff_ffff_fff8), (rax) as u64);
            // 110b2 add QWORD PTR [r15+0x8],0xfffffffffffffff8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, 0xffff_ffff_ffff_fff8u64);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 110b7 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 110bb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 110bc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 110be pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 110c0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 110c2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 110c4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 110c5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1261c size=361
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001261c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1261c;
    loop {
        match pc {
        0x1261c => {
            // block 1261c
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 1261c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1261c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1261d mov rbp,rsp
            rbp = rsp;
            // 12620 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12622 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12624 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 12626 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12628 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 12629 sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 1262d mov r15,rdx
            r15 = rdx;
            // 12630 mov rax,rsi
            rax = rsi;
            // 12633 mov rbx,QWORD PTR [rbp+0x10]
            rbx = m.r64(rbp.wrapping_add(0x10)) as u64;
            // 12637 cmp r8,r9
            fl = sub::<64>(r8, r9).1;
            // 1263a jle 0x12673
            if fl.le() { pc = 0x12673; continue; }
            pc = 0x1263c;
        }
        0x1263c => {
            // block 1263c
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 1263c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1263c mov QWORD PTR [rbp-0x60],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rcx) as u64);
            // 12640 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 12644 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 12647 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 1264a je 0x126b0
            if fl.e() { pc = 0x126b0; continue; }
            pc = 0x1264c;
        }
        0x1264c => {
            // block 1264c
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 1264c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1264c mov r13,rbx
            r13 = rbx;
            // 1264f mov r12,rax
            r12 = rax;
            // 12652 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            pc = 0x12656;
        }
        0x12656 => {
            // block 12656
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12656 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12656 mov rdi,r13
            rdi = r13;
            // 12659 mov rsi,r12
            rsi = r12;
            // 1265c call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12661 inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // 12664 add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 12668 add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 1266c cmp r15,r12
            fl = sub::<64>(r15, r12).1;
            // 1266f jne 0x12656
            if fl.ne() { pc = 0x12656; continue; }
            pc = 0x12671;
        }
        0x12671 => {
            // block 12671
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12671 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12671 jmp 0x126b7
            pc = 0x126b7; continue;
        }
        0x12673 => {
            // block 12673
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12673 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12673 mov QWORD PTR [rbp-0x58],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r15) as u64);
            // 12677 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 1267a cmp rdi,rax
            fl = sub::<64>(rdi, rax).1;
            // 1267d je 0x12711
            if fl.e() { pc = 0x12711; continue; }
            pc = 0x12683;
        }
        0x12683 => {
            // block 12683
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12683 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12683 mov r13,rbx
            r13 = rbx;
            // 12686 mov r12,rdi
            r12 = rdi;
            // 12689 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            pc = 0x1268d;
        }
        0x1268d => {
            // block 1268d
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 1268d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1268d mov r15,rax
            r15 = rax;
            // 12690 mov rdi,r13
            rdi = r13;
            // 12693 mov rsi,r12
            rsi = r12;
            // 12696 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1269b inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // 1269e add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 126a2 add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 126a6 mov rax,r15
            rax = r15;
            // 126a9 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 126ac jne 0x1268d
            if fl.ne() { pc = 0x1268d; continue; }
            pc = 0x126ae;
        }
        0x126ae => {
            // block 126ae
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 126ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 126ae jmp 0x12718
            pc = 0x12718; continue;
        }
        0x126b0 => {
            // block 126b0
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 126b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 126b0 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 126b4 mov r13,rbx
            r13 = rbx;
            pc = 0x126b7;
        }
        0x126b7 => {
            // block 126b7
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 126b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 126b7 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 126bb mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 126bf mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 126c3 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 126c7 mov QWORD PTR [rbp-0x40],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r15) as u64);
            // 126cb mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 126cf mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 126d3 mov QWORD PTR [rsp+0x18],rcx
            m.w64(rsp.wrapping_add(0x18), (rcx) as u64);
            // 126d8 mov QWORD PTR [rsp+0x10],rax
            m.w64(rsp.wrapping_add(0x10), (rax) as u64);
            // 126dd mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 126e1 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 126e5 mov QWORD PTR [rsp+0x8],rcx
            m.w64(rsp.wrapping_add(0x8), (rcx) as u64);
            // 126ea mov QWORD PTR [rsp],rax
            m.w64(rsp, (rax) as u64);
            // 126ee mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 126f2 mov QWORD PTR [rsp+0x20],rax
            m.w64(rsp.wrapping_add(0x20), (rax) as u64);
            // 126f7 mov rdi,r13
            rdi = r13;
            // 126fa mov rsi,r13
            rsi = r13;
            // 126fd mov rdx,rbx
            rdx = rbx;
            // 12700 mov rcx,rbx
            rcx = rbx;
            // 12703 mov r8,QWORD PTR [rbp-0x58]
            r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 12707 mov r9,r8
            r9 = r8;
            // 1270a call 0xeb8d0
            { let r = { let t = m.env.stub_va(0x9d); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1270f jmp 0x1272e
            pc = 0x1272e; continue;
        }
        0x12711 => {
            // block 12711
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12711 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12711 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 12715 mov r13,rbx
            r13 = rbx;
            pc = 0x12718;
        }
        0x12718 => {
            // block 12718
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12718 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12718 mov rdi,rbx
            rdi = rbx;
            // 1271b mov rsi,r13
            rsi = r13;
            // 1271e mov rdx,rax
            rdx = rax;
            // 12721 mov rcx,QWORD PTR [rbp-0x58]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 12725 mov r8,QWORD PTR [rbp-0x50]
            r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 12729 call 0xebf24
            { let r = { let t = m.env.stub_va(0x6f1); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1272e;
        }
        0x1272e => {
            // block 1272e
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 1272e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1272e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 12731 je 0x12749
            if fl.e() { pc = 0x12749; continue; }
            pc = 0x12733;
        }
        0x12733 => {
            // block 12733
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12733 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12733 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 12736 je 0x12749
            if fl.e() { pc = 0x12749; continue; }
            pc = 0x12738;
        }
        0x12738 => {
            // block 12738
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12738 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12738 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1273b mov rdi,rbx
            rdi = rbx;
            // 1273e call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12740 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12744 dec r14
            { let (r, f) = dec::<64>(r14, fl); r14 = r; fl = f; }
            // 12747 jne 0x12738
            if fl.ne() { pc = 0x12738; continue; }
            pc = 0x12749;
        }
        0x12749 => {
            // block 12749
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12749 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12749 add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 1274d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1274e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 12750 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 12752 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 12754 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 12756 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 12757 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x12758 => {
            // block 12758
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12758 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12758 jmp 0x1275a
            pc = 0x1275a; continue;
        }
        0x1275a => {
            // block 1275a
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 1275a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1275a mov r15,rax
            r15 = rax;
            // 1275d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 12760 je 0x12778
            if fl.e() { pc = 0x12778; continue; }
            pc = 0x12762;
        }
        0x12762 => {
            // block 12762
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12762 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12762 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 12765 je 0x12778
            if fl.e() { pc = 0x12778; continue; }
            pc = 0x12767;
        }
        0x12767 => {
            // block 12767
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12767 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12767 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1276a mov rdi,rbx
            rdi = rbx;
            // 1276d call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1276f add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12773 dec r14
            { let (r, f) = dec::<64>(r14, fl); r14 = r; fl = f; }
            // 12776 jne 0x12767
            if fl.ne() { pc = 0x12767; continue; }
            pc = 0x12778;
        }
        0x12778 => {
            // block 12778
            #[cfg(feature = "trace")] eprintln!("f_0001261c pc 12778 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12778 mov rdi,r15
            rdi = r15;
            // 1277b call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12780 jmp 0x1275a
            pc = 0x1275a; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x12e88 size=300
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00012e88<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x12e88;
    loop {
        match pc {
        0x12e88 => {
            // block 12e88
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12e88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e88 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12e89 mov rbp,rsp
            rbp = rsp;
            // 12e8c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12e8e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12e90 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 12e92 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12e94 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 12e95 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 12e99 mov r15,rdx
            r15 = rdx;
            // 12e9c mov r13,rsi
            r13 = rsi;
            // 12e9f mov r14,rdi
            r14 = rdi;
            // 12ea2 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 12ea6 mov rdi,r13
            rdi = r13;
            // 12ea9 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12eac mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 12eb3 lea r12,[r13+0x248]
            r12 = r13.wrapping_add(0x248);
            // 12eba xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x12ebc;
        }
        0x12ebc => {
            // block 12ebc
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12ebc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12ebc lea rsi,[r15+rbx*1]
            rsi = r15.wrapping_add(rbx);
            // 12ec0 mov rax,QWORD PTR [r13+0x250]
            rax = m.r64(r13.wrapping_add(0x250)) as u64;
            // 12ec7 cmp rax,QWORD PTR [r13+0x258]
            fl = sub::<64>(rax, m.r64(r13.wrapping_add(0x258)) as u64).1;
            // 12ece je 0x12edd
            if fl.e() { pc = 0x12edd; continue; }
            pc = 0x12ed0;
        }
        0x12ed0 => {
            // block 12ed0
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12ed0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12ed0 mov cl,BYTE PTR [rsi]
            rcx = (rcx & !0xff) | ((m.r8(rsi) as u64) as u8 as u64);
            // 12ed2 mov BYTE PTR [rax],cl
            m.w8(rax, ((rcx as u8 as u64)) as u8);
            // 12ed4 inc QWORD PTR [r13+0x250]
            { let (r, f) = inc::<64>(m.r64(r13.wrapping_add(0x250)) as u64, fl); m.w64(r13.wrapping_add(0x250), (r) as u64); fl = f; }
            // 12edb jmp 0x12ee5
            pc = 0x12ee5; continue;
        }
        0x12edd => {
            // block 12edd
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12edd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12edd mov rdi,r12
            rdi = r12;
            // 12ee0 call 0xebb0a
            { let r = { let t = m.env.stub_va(0x2d7); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x12ee5;
        }
        0x12ee5 => {
            // block 12ee5
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12ee5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12ee5 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 12ee8 cmp rbx,0xb7
            fl = sub::<64>(rbx, 0xb7u64).1;
            // 12eef jne 0x12ebc
            if fl.ne() { pc = 0x12ebc; continue; }
            pc = 0x12ef1;
        }
        0x12ef1 => {
            // block 12ef1
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12ef1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12ef1 lea r15,[r13+0x2c8]
            r15 = r13.wrapping_add(0x2c8);
            // 12ef8 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 12efc lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 12f00 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 12f04 mov rsi,r13
            rsi = r13;
            // 12f07 call QWORD PTR [rax+0x90]
            { let r = { let t = m.r64(rax.wrapping_add(0x90)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12f0d test BYTE PTR [r13+0x2c8],0x1
            fl = logic::<8>(m.r8(r13.wrapping_add(0x2c8)) as u64 & 0x1u64);
            // 12f15 jne 0x12f1f
            if fl.ne() { pc = 0x12f1f; continue; }
            pc = 0x12f17;
        }
        0x12f17 => {
            // block 12f17
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f17 mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // 12f1d jmp 0x12f34
            pc = 0x12f34; continue;
        }
        0x12f1f => {
            // block 12f1f
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f1f mov rax,QWORD PTR [r13+0x2d8]
            rax = m.r64(r13.wrapping_add(0x2d8)) as u64;
            // 12f26 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 12f29 mov QWORD PTR [r13+0x2d0],0x0
            m.w64(r13.wrapping_add(0x2d0), (0x0u64) as u64);
            pc = 0x12f34;
        }
        0x12f34 => {
            // block 12f34
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f34 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 12f36 mov rdi,r15
            rdi = r15;
            // 12f39 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12f3e mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 12f42 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 12f46 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 12f4a mov rcx,QWORD PTR [rbp-0x48]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 12f4e mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 12f52 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 12f55 xorps xmm0,xmm0
            x0 = 0;
            // 12f58 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 12f5c mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 12f64 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 12f68 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12f6d movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 12f71 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 12f73 jne 0x12f7a
            if fl.ne() { pc = 0x12f7a; continue; }
            pc = 0x12f75;
        }
        0x12f75 => {
            // block 12f75
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f75 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 12f78 jmp 0x12f81
            pc = 0x12f81; continue;
        }
        0x12f7a => {
            // block 12f7a
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f7a mov rax,QWORD PTR [r13+0x2d0]
            rax = m.r64(r13.wrapping_add(0x2d0)) as u64;
            pc = 0x12f81;
        }
        0x12f81 => {
            // block 12f81
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f81 test rax,rax
            fl = logic::<64>(rax & rax);
            // 12f84 jne 0x12f8e
            if fl.ne() { pc = 0x12f8e; continue; }
            pc = 0x12f86;
        }
        0x12f86 => {
            // block 12f86
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f86 mov BYTE PTR [r13+0x2e8],0x0
            m.w8(r13.wrapping_add(0x2e8), (0x0u64) as u8);
            pc = 0x12f8e;
        }
        0x12f8e => {
            // block 12f8e
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f8e mov rdi,r14
            rdi = r14;
            // 12f91 mov rsi,r15
            rsi = r15;
            // 12f94 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12f99 mov rax,r14
            rax = r14;
            // 12f9c add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 12fa0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 12fa1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 12fa3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 12fa5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 12fa7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 12fa9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 12faa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x12fab => {
            // block 12fab
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12fab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12fab mov rdi,rax
            rdi = rax;
            // 12fae call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15028 size=304
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00015028<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x15028;
    loop {
        match pc {
        0x15028 => {
            // block 15028
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 15028 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15028 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15029 mov rbp,rsp
            rbp = rsp;
            // 1502c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1502e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15030 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15032 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15033 mov r15,rdx
            r15 = rdx;
            // 15036 mov r14,rsi
            r14 = rsi;
            // 15039 mov rbx,rdi
            rbx = rdi;
            // 1503c mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 15044 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // 1504c mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 15054 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 1505c mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 15064 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 1506c cmp BYTE PTR [rbx+0x190],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x190)) as u64, 0x0u64).1;
            // 15073 je 0x15083
            if fl.e() { pc = 0x15083; continue; }
            pc = 0x15075;
        }
        0x15075 => {
            // block 15075
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 15075 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15075 mov rdi,QWORD PTR [rbx+0x40]
            rdi = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 15079 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 1507c je 0x15083
            if fl.e() { pc = 0x15083; continue; }
            pc = 0x1507e;
        }
        0x1507e => {
            // block 1507e
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 1507e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1507e call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x15083;
        }
        0x15083 => {
            // block 15083
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 15083 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15083 cmp BYTE PTR [rbx+0x191],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x191)) as u64, 0x0u64).1;
            // 1508a je 0x1509a
            if fl.e() { pc = 0x1509a; continue; }
            pc = 0x1508c;
        }
        0x1508c => {
            // block 1508c
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 1508c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1508c mov rdi,QWORD PTR [rbx+0x68]
            rdi = m.r64(rbx.wrapping_add(0x68)) as u64;
            // 15090 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 15093 je 0x1509a
            if fl.e() { pc = 0x1509a; continue; }
            pc = 0x15095;
        }
        0x15095 => {
            // block 15095
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 15095 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15095 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1509a;
        }
        0x1509a => {
            // block 1509a
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 1509a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1509a mov QWORD PTR [rbx+0x60],r15
            m.w64(rbx.wrapping_add(0x60), (r15) as u64);
            // 1509e cmp r15,0x9
            fl = sub::<64>(r15, 0x9u64).1;
            // 150a2 jb 0x150c2
            if fl.b() { pc = 0x150c2; continue; }
            pc = 0x150a4;
        }
        0x150a4 => {
            // block 150a4
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150a4 mov r12b,BYTE PTR [rbx+0x192]
            r12 = (r12 & !0xff) | ((m.r8(rbx.wrapping_add(0x192)) as u64) as u8 as u64);
            // 150ab test r14,r14
            fl = logic::<64>(r14 & r14);
            // 150ae je 0x150e2
            if fl.e() { pc = 0x150e2; continue; }
            pc = 0x150b0;
        }
        0x150b0 => {
            // block 150b0
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150b0 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 150b3 je 0x150e2
            if fl.e() { pc = 0x150e2; continue; }
            pc = 0x150b5;
        }
        0x150b5 => {
            // block 150b5
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150b5 mov QWORD PTR [rbx+0x40],r14
            m.w64(rbx.wrapping_add(0x40), (r14) as u64);
            // 150b9 mov BYTE PTR [rbx+0x190],0x0
            m.w8(rbx.wrapping_add(0x190), (0x0u64) as u8);
            // 150c0 jmp 0x150f5
            pc = 0x150f5; continue;
        }
        0x150c2 => {
            // block 150c2
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150c2 lea rax,[rbx+0x58]
            rax = rbx.wrapping_add(0x58);
            // 150c6 mov QWORD PTR [rbx+0x40],rax
            m.w64(rbx.wrapping_add(0x40), (rax) as u64);
            // 150ca mov QWORD PTR [rbx+0x60],0x8
            m.w64(rbx.wrapping_add(0x60), (0x8u64) as u64);
            // 150d2 mov BYTE PTR [rbx+0x190],0x0
            m.w8(rbx.wrapping_add(0x190), (0x0u64) as u8);
            // 150d9 mov r12b,BYTE PTR [rbx+0x192]
            r12 = (r12 & !0xff) | ((m.r8(rbx.wrapping_add(0x192)) as u64) as u8 as u64);
            // 150e0 jmp 0x150f5
            pc = 0x150f5; continue;
        }
        0x150e2 => {
            // block 150e2
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150e2 mov rdi,r15
            rdi = r15;
            // 150e5 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 150ea mov QWORD PTR [rbx+0x40],rax
            m.w64(rbx.wrapping_add(0x40), (rax) as u64);
            // 150ee mov BYTE PTR [rbx+0x190],0x1
            m.w8(rbx.wrapping_add(0x190), (0x1u64) as u8);
            pc = 0x150f5;
        }
        0x150f5 => {
            // block 150f5
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150f5 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 150f8 je 0x15113
            if fl.e() { pc = 0x15113; continue; }
            pc = 0x150fa;
        }
        0x150fa => {
            // block 150fa
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150fa mov BYTE PTR [rbx+0x191],0x0
            m.w8(rbx.wrapping_add(0x191), (0x0u64) as u8);
            // 15101 mov QWORD PTR [rbx+0x70],0x0
            m.w64(rbx.wrapping_add(0x70), (0x0u64) as u64);
            // 15109 mov QWORD PTR [rbx+0x68],0x0
            m.w64(rbx.wrapping_add(0x68), (0x0u64) as u64);
            // 15111 jmp 0x1514c
            pc = 0x1514c; continue;
        }
        0x15113 => {
            // block 15113
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 15113 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15113 cmp r15,0x8
            fl = sub::<64>(r15, 0x8u64).1;
            // 15117 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // 1511c cmovge rdi,r15
            if fl.ge() { rdi = r15; }
            // 15120 mov QWORD PTR [rbx+0x70],rdi
            m.w64(rbx.wrapping_add(0x70), (rdi) as u64);
            // 15124 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 15127 je 0x1513c
            if fl.e() { pc = 0x1513c; continue; }
            pc = 0x15129;
        }
        0x15129 => {
            // block 15129
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 15129 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15129 cmp rdi,0x8
            fl = sub::<64>(rdi, 0x8u64).1;
            // 1512d jb 0x1513c
            if fl.b() { pc = 0x1513c; continue; }
            pc = 0x1512f;
        }
        0x1512f => {
            // block 1512f
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 1512f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1512f mov QWORD PTR [rbx+0x68],r14
            m.w64(rbx.wrapping_add(0x68), (r14) as u64);
            // 15133 mov BYTE PTR [rbx+0x191],0x0
            m.w8(rbx.wrapping_add(0x191), (0x0u64) as u8);
            // 1513a jmp 0x1514c
            pc = 0x1514c; continue;
        }
        0x1513c => {
            // block 1513c
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 1513c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1513c call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15141 mov QWORD PTR [rbx+0x68],rax
            m.w64(rbx.wrapping_add(0x68), (rax) as u64);
            // 15145 mov BYTE PTR [rbx+0x191],0x1
            m.w8(rbx.wrapping_add(0x191), (0x1u64) as u8);
            pc = 0x1514c;
        }
        0x1514c => {
            // block 1514c
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 1514c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1514c mov rax,rbx
            rax = rbx;
            // 1514f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15150 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15152 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15154 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 15156 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15157 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15158 size=360
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949
pub fn f_00015158<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x15158;
    loop {
        match pc {
        0x15158 => {
            // block 15158
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15158 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15158 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15159 mov rbp,rsp
            rbp = rsp;
            // 1515c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1515e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15160 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15162 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15164 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15165 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 15166 mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // 15169 mov r12,rdx
            r12 = rdx;
            // 1516c mov r14,rsi
            r14 = rsi;
            // 1516f mov rbx,rdi
            rbx = rdi;
            // 15172 mov rdi,QWORD PTR [r14+0x80]
            rdi = m.r64(r14.wrapping_add(0x80)) as u64;
            // 15179 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 1517c je 0x15294
            if fl.e() { pc = 0x15294; continue; }
            pc = 0x15182;
        }
        0x15182 => {
            // block 15182
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15182 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15182 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 15185 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15188 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 1518b cmp QWORD PTR [r14+0x78],0x0
            fl = sub::<64>(m.r64(r14.wrapping_add(0x78)) as u64, 0x0u64).1;
            // 15190 je 0x151d1
            if fl.e() { pc = 0x151d1; continue; }
            pc = 0x15192;
        }
        0x15192 => {
            // block 15192
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15192 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15192 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 15195 je 0x1519c
            if fl.e() { pc = 0x1519c; continue; }
            pc = 0x15197;
        }
        0x15197 => {
            // block 15197
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15197 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15197 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 1519a jle 0x151d1
            if fl.le() { pc = 0x151d1; continue; }
            pc = 0x1519c;
        }
        0x1519c => {
            // block 1519c
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 1519c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1519c mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1519f mov rdi,r14
            rdi = r14;
            // 151a2 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 151a5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 151a7 jne 0x151d1
            if fl.ne() { pc = 0x151d1; continue; }
            pc = 0x151a9;
        }
        0x151a9 => {
            // block 151a9
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151a9 cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 151ad jae 0x151d1
            if fl.ae() { pc = 0x151d1; continue; }
            pc = 0x151af;
        }
        0x151af => {
            // block 151af
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151af mov rdi,QWORD PTR [r14+0x78]
            rdi = m.r64(r14.wrapping_add(0x78)) as u64;
            // 151b3 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 151b5 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 151b8 jle 0x151c1
            if fl.le() { pc = 0x151c1; continue; }
            pc = 0x151ba;
        }
        0x151ba => {
            // block 151ba
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151ba movsxd rsi,r13d
            rsi = sx::<32>((r13 as u32 as u64)) as u64;
            // 151bd imul rsi,r12
            { let (r, f) = imul::<64>(rsi, r12); rsi = r; fl = f; }
            pc = 0x151c1;
        }
        0x151c1 => {
            // block 151c1
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151c1 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 151c4 call 0xec1dc
            { let r = { let t = m.env.stub_va(0x9a9); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 151c9 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 151cb je 0x1526d
            if fl.e() { pc = 0x1526d; continue; }
            pc = 0x151d1;
        }
        0x151d1 => {
            // block 151d1
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151d1 mov QWORD PTR [rbx+0x78],0x0
            m.w64(rbx.wrapping_add(0x78), (0x0u64) as u64);
            // 151d9 mov QWORD PTR [rbx+0x70],0x0
            m.w64(rbx.wrapping_add(0x70), (0x0u64) as u64);
            // 151e1 mov QWORD PTR [rbx+0x68],0x0
            m.w64(rbx.wrapping_add(0x68), (0x0u64) as u64);
            // 151e9 mov QWORD PTR [rbx+0x60],0x0
            m.w64(rbx.wrapping_add(0x60), (0x0u64) as u64);
            // 151f1 mov QWORD PTR [rbx+0x58],0x0
            m.w64(rbx.wrapping_add(0x58), (0x0u64) as u64);
            // 151f9 mov QWORD PTR [rbx+0x50],0x0
            m.w64(rbx.wrapping_add(0x50), (0x0u64) as u64);
            // 15201 mov QWORD PTR [rbx+0x48],0x0
            m.w64(rbx.wrapping_add(0x48), (0x0u64) as u64);
            // 15209 mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // 15211 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 15219 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // 15221 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 15229 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 15231 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 15239 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 15241 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 15249 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 15250 mov QWORD PTR [rbx+0x80],0xffffffffffffffff
            m.w64(rbx.wrapping_add(0x80), (0xffff_ffff_ffff_ffffu64) as u64);
            pc = 0x1525b;
        }
        0x1525b => {
            // block 1525b
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 1525b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1525b mov rax,rbx
            rax = rbx;
            // 1525e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 15262 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15263 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15265 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 15267 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15269 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1526b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1526c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1526d => {
            // block 1526d
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 1526d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1526d mov rdi,QWORD PTR [r14+0x78]
            rdi = m.r64(r14.wrapping_add(0x78)) as u64;
            // 15271 call 0xec1e2
            { let r = { let t = m.env.stub_va(0x9af); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15276 add r14,0x88
            { let (r, f) = add::<64>(r14, 0x88u64);
              r14 = r;
              fl = f; }
            // 1527d mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // 15282 mov rdi,rbx
            rdi = rbx;
            // 15285 mov rsi,r14
            rsi = r14;
            // 15288 rep movs QWORD PTR [rdi],QWORD PTR [rsi]
            while rcx != 0 { let v = m.r64(rsi); m.w64(rdi, v); rsi = rsi.wrapping_add(8); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); }
            // 1528b mov QWORD PTR [rbx+0x80],rax
            m.w64(rbx.wrapping_add(0x80), (rax) as u64);
            // 15292 jmp 0x1525b
            pc = 0x1525b; continue;
        }
        0x15294 => {
            // block 15294
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15294 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15294 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // 15299 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1529e mov rbx,rax
            rbx = rax;
            // 152a1 mov rdi,rbx
            rdi = rbx;
            // 152a4 call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 152a9 mov rsi,QWORD PTR [rip+0x12af30] # 0x1401e0
            rsi = m.r64(m.env.data_va(346541)) as u64;
            // 152b0 mov rdx,QWORD PTR [rip+0x12aef1] # 0x1401a8
            rdx = m.r64(m.env.data_va(346485)) as u64;
            // 152b7 mov rdi,rbx
            rdi = rbx;
            // 152ba call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28551 size=356
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028551<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x28551;
    loop {
        match pc {
        0x28551 => {
            // block 28551
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 28551 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28551 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28552 mov rbp,rsp
            rbp = rsp;
            // 28555 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 28557 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28559 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2855b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2855d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2855e sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 28562 mov r14,rsi
            r14 = rsi;
            // 28565 mov r15,rdi
            r15 = rdi;
            // 28568 mov r12,QWORD PTR [r14]
            r12 = m.r64(r14) as u64;
            // 2856b lea rsi,[rip+0x1021e2] # 0x12a754
            rsi = m.env.data_va(257825);
            // 28572 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 28577 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2857c mov rsi,QWORD PTR [r14+0x10]
            rsi = m.r64(r14.wrapping_add(0x10)) as u64;
            // 28580 mov rdi,rax
            rdi = rax;
            // 28583 call 0xec0a4
            { let r = { let t = m.env.stub_va(0x871); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28588 lea rsi,[rip+0x101ec7] # 0x12a456
            rsi = m.env.data_va(257059);
            // 2858f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 28594 mov rdi,rax
            rdi = rax;
            // 28597 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2859c add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 285a0 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 285a3 je 0x2867e
            if fl.e() { pc = 0x2867e; continue; }
            pc = 0x285a9;
        }
        0x285a9 => {
            // block 285a9
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 285a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 285a9 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 285ad jmp 0x285c9
            pc = 0x285c9; continue;
        }
        0x285af => {
            // block 285af
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 285af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 285af mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 285b4 mov rdi,r15
            rdi = r15;
            // 285b7 lea rsi,[rip+0x1021a5] # 0x12a763
            rsi = m.env.data_va(257840);
            // 285be call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 285c3 mov r12,rbx
            r12 = rbx;
            // 285c6 mov rbx,r13
            rbx = r13;
            pc = 0x285c9;
        }
        0x285c9 => {
            // block 285c9
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 285c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 285c9 mov esi,DWORD PTR [r12+0x1c]
            rsi = (m.r32(r12.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 285ce mov r13d,DWORD PTR [r12+0x20]
            r13 = (m.r32(r12.wrapping_add(0x20)) as u64) as u32 as u64;
            // 285d3 mov rdi,rbx
            rdi = rbx;
            // 285d6 call 0x60623
            { let r = { let t = m.env.code_va(0x60623); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 285db movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 285df test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 285e2 jne 0x285ed
            if fl.ne() { pc = 0x285ed; continue; }
            pc = 0x285e4;
        }
        0x285e4 => {
            // block 285e4
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 285e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 285e4 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 285e7 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 285eb jmp 0x285f5
            pc = 0x285f5; continue;
        }
        0x285ed => {
            // block 285ed
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 285ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 285ed mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 285f1 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x285f5;
        }
        0x285f5 => {
            // block 285f5
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 285f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 285f5 mov rdi,r15
            rdi = r15;
            // 285f8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 285fd mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 28602 mov rdi,rax
            rdi = rax;
            // 28605 lea rsi,[rip+0x102153] # 0x12a75f
            rsi = m.env.data_va(257836);
            // 2860c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28611 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 28614 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 28618 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2861c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2861f or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 28622 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 28626 mov rdi,rax
            rdi = rax;
            // 28629 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 2862c call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28631 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 28634 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 28638 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2863c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2863f or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 28642 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 28646 mov rdi,rbx
            rdi = rbx;
            // 28649 mov r13,rbx
            r13 = rbx;
            // 2864c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28651 mov rax,QWORD PTR [r12+0x8]
            rax = m.r64(r12.wrapping_add(0x8)) as u64;
            // 28656 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28659 je 0x28668
            if fl.e() { pc = 0x28668; continue; }
            pc = 0x2865b;
        }
        0x2865b => {
            // block 2865b
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 2865b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2865b mov rbx,rax
            rbx = rax;
            // 2865e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28661 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28664 jne 0x2865b
            if fl.ne() { pc = 0x2865b; continue; }
            pc = 0x28666;
        }
        0x28666 => {
            // block 28666
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 28666 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28666 jmp 0x28675
            pc = 0x28675; continue;
        }
        0x28668 => {
            // block 28668
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 28668 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28668 mov rbx,QWORD PTR [r12+0x10]
            rbx = m.r64(r12.wrapping_add(0x10)) as u64;
            // 2866d cmp QWORD PTR [rbx],r12
            fl = sub::<64>(m.r64(rbx) as u64, r12).1;
            // 28670 mov r12,rbx
            r12 = rbx;
            // 28673 jne 0x28668
            if fl.ne() { pc = 0x28668; continue; }
            pc = 0x28675;
        }
        0x28675 => {
            // block 28675
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 28675 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28675 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 28678 jne 0x285af
            if fl.ne() { pc = 0x285af; continue; }
            pc = 0x2867e;
        }
        0x2867e => {
            // block 2867e
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 2867e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2867e lea rsi,[rip+0x101dcf] # 0x12a454
            rsi = m.env.data_va(257057);
            // 28685 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2868a mov rdi,r15
            rdi = r15;
            // 2868d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28692 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 28696 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28697 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 28699 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2869b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2869d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2869f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 286a0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x286a1 => {
            // block 286a1
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 286a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 286a1 mov rbx,rax
            rbx = rax;
            // 286a4 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 286a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 286ad mov rdi,rbx
            rdi = rbx;
            // 286b0 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2ca6e size=318
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002ca6e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2ca6e;
    loop {
        match pc {
        0x2ca6e => {
            // block 2ca6e
            #[cfg(feature = "trace")] eprintln!("f_0002ca6e pc 2ca6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ca6e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2ca6f mov rbp,rsp
            rbp = rsp;
            // 2ca72 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2ca74 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2ca76 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2ca78 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2ca7a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2ca7b sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2ca7f mov r15,rsi
            r15 = rsi;
            // 2ca82 mov QWORD PTR [rbp-0x40],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdi) as u64);
            // 2ca86 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 2ca8a sub rsi,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rsi, m.r64(r15) as u64);
              rsi = r;
              fl = f; }
            // 2ca8d sar rsi,0x4
            { let (r, f) = sar::<64>(rsi, 0x4u64, fl); rsi = r; fl = f; }
            // 2ca91 call 0xec0a4
            { let r = { let t = m.env.stub_va(0x871); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ca96 lea rsi,[rip+0xfdf07] # 0x12a9a4
            rsi = m.env.data_va(258417);
            // 2ca9d mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 2caa2 mov rdi,rax
            rdi = rax;
            // 2caa5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2caaa mov rbx,rax
            rbx = rax;
            // 2caad mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2cab0 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2cab4 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 2cab7 lea r14,[rbp-0x30]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2cabb mov rdi,r14
            rdi = r14;
            // 2cabe call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cac3 mov rsi,QWORD PTR [rip+0x1136c6] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 2caca mov rdi,r14
            rdi = r14;
            // 2cacd call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cad2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2cad5 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 2cad9 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 2cade mov rdi,rax
            rdi = rax;
            // 2cae1 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cae3 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2cae6 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2caea call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2caef movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 2caf3 mov rdi,rbx
            rdi = rbx;
            // 2caf6 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cafb mov rdi,rbx
            rdi = rbx;
            // 2cafe call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cb03 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 2cb06 cmp rbx,QWORD PTR [r15+0x8]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 2cb0a je 0x2cb7b
            if fl.e() { pc = 0x2cb7b; continue; }
            pc = 0x2cb0c;
        }
        0x2cb0c => {
            // block 2cb0c
            #[cfg(feature = "trace")] eprintln!("f_0002ca6e pc 2cb0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cb0c lea r12,[rbp-0x38]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            pc = 0x2cb10;
        }
        0x2cb10 => {
            // block 2cb10
            #[cfg(feature = "trace")] eprintln!("f_0002ca6e pc 2cb10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cb10 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2cb14 mov rsi,rbx
            rsi = rbx;
            // 2cb17 call 0x2c238
            { let r = { let t = m.env.code_va(0x2c238); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cb1c mov r13,rax
            r13 = rax;
            // 2cb1f mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 2cb23 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2cb27 add rsi,r13
            { let (r, f) = add::<64>(rsi, r13);
              rsi = r;
              fl = f; }
            // 2cb2a mov rdi,r12
            rdi = r12;
            // 2cb2d call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cb32 mov rdi,r12
            rdi = r12;
            // 2cb35 mov rsi,QWORD PTR [rip+0x113654] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 2cb3c call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cb41 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2cb44 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 2cb48 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 2cb4d mov rdi,rax
            rdi = rax;
            // 2cb50 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cb52 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2cb55 mov rdi,r12
            rdi = r12;
            // 2cb58 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cb5d movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // 2cb61 mov rdi,r13
            rdi = r13;
            // 2cb64 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cb69 mov rdi,r13
            rdi = r13;
            // 2cb6c call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cb71 add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // 2cb75 cmp rbx,QWORD PTR [r15+0x8]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 2cb79 jne 0x2cb10
            if fl.ne() { pc = 0x2cb10; continue; }
            pc = 0x2cb7b;
        }
        0x2cb7b => {
            // block 2cb7b
            #[cfg(feature = "trace")] eprintln!("f_0002ca6e pc 2cb7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cb7b mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2cb7f add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2cb83 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2cb84 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2cb86 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2cb88 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2cb8a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2cb8c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2cb8d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2cb8e => {
            // block 2cb8e
            #[cfg(feature = "trace")] eprintln!("f_0002ca6e pc 2cb8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cb8e mov rbx,rax
            rbx = rax;
            // 2cb91 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            pc = 0x2cb95;
        }
        0x2cb95 => {
            // block 2cb95
            #[cfg(feature = "trace")] eprintln!("f_0002ca6e pc 2cb95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cb95 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cb9a mov rdi,rbx
            rdi = rbx;
            // 2cb9d call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cba2 mov rbx,rax
            rbx = rax;
            // 2cba5 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2cba9 jmp 0x2cb95
            pc = 0x2cb95; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x331bc size=308
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000331bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x331bc;
    loop {
        match pc {
        0x331bc => {
            // block 331bc
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 331bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 331bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 331bd mov rbp,rsp
            rbp = rsp;
            // 331c0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 331c2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 331c4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 331c6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 331c8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 331c9 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 331cd mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 331d1 mov QWORD PTR [rbp-0x30],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rdx) as u64);
            // 331d5 mov QWORD PTR [rbp-0x40],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rsi) as u64);
            // 331d9 mov r13,rdi
            r13 = rdi;
            // 331dc mov DWORD PTR [rcx],0xffffffff
            m.w32(rcx, (0xffffffffu64) as u32);
            // 331e2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 331e4 cmp rsi,rdx
            fl = sub::<64>(rsi, rdx).1;
            // 331e7 je 0x332e0
            if fl.e() { pc = 0x332e0; continue; }
            pc = 0x331ed;
        }
        0x331ed => {
            // block 331ed
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 331ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 331ed test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 331f0 je 0x332e0
            if fl.e() { pc = 0x332e0; continue; }
            pc = 0x331f6;
        }
        0x331f6 => {
            // block 331f6
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 331f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 331f6 cmp QWORD PTR [rbp-0x30],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, 0x0u64).1;
            // 331fb je 0x332e0
            if fl.e() { pc = 0x332e0; continue; }
            pc = 0x33201;
        }
        0x33201 => {
            // block 33201
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33201 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33201 cmp DWORD PTR [r13+0x0],0x0
            fl = sub::<32>(m.r32(r13) as u64, 0x0u64).1;
            // 33206 je 0x332de
            if fl.e() { pc = 0x332de; continue; }
            pc = 0x3320c;
        }
        0x3320c => {
            // block 3320c
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 3320c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3320c cmp DWORD PTR [r13+0x4],0x0
            fl = sub::<32>(m.r32(r13.wrapping_add(0x4)) as u64, 0x0u64).1;
            // 33211 je 0x332de
            if fl.e() { pc = 0x332de; continue; }
            pc = 0x33217;
        }
        0x33217 => {
            // block 33217
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33217 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33217 mov cx,WORD PTR [r13+0x34]
            rcx = (rcx & !0xffff) | ((m.r16(r13.wrapping_add(0x34)) as u64) as u16 as u64);
            // 3321c test cx,cx
            fl = logic::<16>((rcx as u16 as u64) & (rcx as u16 as u64));
            // 3321f je 0x332de
            if fl.e() { pc = 0x332de; continue; }
            pc = 0x33225;
        }
        0x33225 => {
            // block 33225
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33225 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33225 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33227 cmp DWORD PTR [r13+0x24],0x0
            fl = sub::<32>(m.r32(r13.wrapping_add(0x24)) as u64, 0x0u64).1;
            // 3322c je 0x332e0
            if fl.e() { pc = 0x332e0; continue; }
            pc = 0x33232;
        }
        0x33232 => {
            // block 33232
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33232 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33232 movzx eax,cx
            rax = ((rcx as u16 as u64)) as u32 as u64;
            // 33235 mov DWORD PTR [rbp-0x44],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), ((rax as u32 as u64)) as u32);
            // 33238 mov DWORD PTR [rbp-0x48],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0x0u64) as u32);
            pc = 0x3323f;
        }
        0x3323f => {
            // block 3323f
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 3323f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3323f mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 33243 mov DWORD PTR [rax],0x0
            m.w32(rax, (0x0u64) as u32);
            // 33249 cmp DWORD PTR [r13+0x0],0x0
            fl = sub::<32>(m.r32(r13) as u64, 0x0u64).1;
            // 3324e je 0x332bc
            if fl.e() { pc = 0x332bc; continue; }
            pc = 0x33250;
        }
        0x33250 => {
            // block 33250
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33250 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33250 mov eax,DWORD PTR [rbp-0x48]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 33253 movzx ebx,ax
            rbx = ((rax as u16 as u64)) as u32 as u64;
            // 33256 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            pc = 0x3325c;
        }
        0x3325c => {
            // block 3325c
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 3325c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3325c lea r15d,[r12-0x1]
            r15 = (r12.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 33261 mov rdi,r13
            rdi = r13;
            // 33264 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 33267 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 33269 call 0x330d0
            { let r = { let t = m.env.code_va(0x330d0); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3326e mov r14,rax
            r14 = rax;
            // 33271 mov rdi,r13
            rdi = r13;
            // 33274 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 33278 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 3327b mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 3327d call 0x330d0
            { let r = { let t = m.env.code_va(0x330d0); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33282 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 33284 cmp DWORD PTR [rbp-0x44],ebx
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64, (rbx as u32 as u64)).1;
            // 33287 jbe 0x3328e
            if fl.be() { pc = 0x3328e; continue; }
            pc = 0x33289;
        }
        0x33289 => {
            // block 33289
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33289 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33289 mov ecx,DWORD PTR [r13+rbx*4+0x24]
            rcx = (m.r32(r13.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0x24)) as u64) as u32 as u64;
            pc = 0x3328e;
        }
        0x3328e => {
            // block 3328e
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 3328e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3328e mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 33290 mov rdi,r14
            rdi = r14;
            // 33293 mov rsi,rax
            rsi = rax;
            // 33296 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3329b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 3329d mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 3329f test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 332a1 mov rsi,QWORD PTR [rbp-0x40]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 332a5 jne 0x332e0
            if fl.ne() { pc = 0x332e0; continue; }
            pc = 0x332a7;
        }
        0x332a7 => {
            // block 332a7
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332a7 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 332ab mov DWORD PTR [rax],r12d
            m.w32(rax, ((r12 as u32 as u64)) as u32);
            // 332ae cmp r12d,DWORD PTR [r13+0x0]
            fl = sub::<32>((r12 as u32 as u64), m.r32(r13) as u64).1;
            // 332b2 lea eax,[r12+0x1]
            rax = (r12.wrapping_add(0x1)) as u32 as u64;
            // 332b7 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // 332ba jb 0x3325c
            if fl.b() { pc = 0x3325c; continue; }
            pc = 0x332bc;
        }
        0x332bc => {
            // block 332bc
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332bc mov eax,DWORD PTR [rbp-0x48]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 332bf inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 332c1 mov DWORD PTR [rbp-0x48],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), ((rax as u32 as u64)) as u32);
            // 332c4 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 332c7 cmp eax,DWORD PTR [rbp-0x44]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64).1;
            // 332ca jb 0x3323f
            if fl.b() { pc = 0x3323f; continue; }
            pc = 0x332d0;
        }
        0x332d0 => {
            // block 332d0
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332d0 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 332d4 mov DWORD PTR [rax],0xffffffff
            m.w32(rax, (0xffffffffu64) as u32);
            // 332da mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 332dc jmp 0x332e0
            pc = 0x332e0; continue;
        }
        0x332de => {
            // block 332de
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332de xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x332e0;
        }
        0x332e0 => {
            // block 332e0
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332e0 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 332e4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 332e5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 332e7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 332e9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 332eb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 332ed pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 332ee ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x421e0 size=340
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000421e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x421e0;
    loop {
        match pc {
        0x421e0 => {
            // block 421e0
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 421e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 421e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 421e1 mov rbp,rsp
            rbp = rsp;
            // 421e4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 421e6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 421e8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 421ea push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 421eb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 421ef mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 421f2 mov r14,rsi
            r14 = rsi;
            // 421f5 mov rbx,rdi
            rbx = rdi;
            // 421f8 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 421fb lea rcx,[rip+0xbe26e] # 0x100470
            rcx = m.env.data_va(85053);
            // 42202 mov r12d,DWORD PTR [rcx+rax*4]
            r12 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 42206 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 42209 call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4220e mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 42215 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 42218 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 4221b ja 0x4223d
            if fl.a() { pc = 0x4223d; continue; }
            pc = 0x4221d;
        }
        0x4221d => {
            // block 4221d
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 4221d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4221d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42220 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 42224 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 42229 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 4222f mov rdi,rbx
            rdi = rbx;
            // 42232 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 42235 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42238 jmp 0x4230a
            pc = 0x4230a; continue;
        }
        0x4223d => {
            // block 4223d
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 4223d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4223d mov DWORD PTR [rbp-0x28],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u32);
            // 42244 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 4224b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4224e lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 42252 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 42257 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 4225d mov rdi,rbx
            rdi = rbx;
            // 42260 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 42263 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42266 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 42268 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4226a mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // 4226f je 0x4228f
            if fl.e() { pc = 0x4228f; continue; }
            pc = 0x42271;
        }
        0x42271 => {
            // block 42271
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 42271 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42271 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42274 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 42278 mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 4227d mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            // 42283 mov rdi,rbx
            rdi = rbx;
            // 42286 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 42289 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4228c mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            pc = 0x4228f;
        }
        0x4228f => {
            // block 4228f
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 4228f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4228f mov edx,DWORD PTR [rbp-0x28]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 42292 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 42294 or esi,edx
            { let r = (rsi as u32 as u64) | (rdx as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 42296 je 0x422a9
            if fl.e() { pc = 0x422a9; continue; }
            pc = 0x42298;
        }
        0x42298 => {
            // block 42298
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 42298 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42298 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 4229a je 0x422b2
            if fl.e() { pc = 0x422b2; continue; }
            pc = 0x4229c;
        }
        0x4229c => {
            // block 4229c
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 4229c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4229c test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 4229e jne 0x422b2
            if fl.ne() { pc = 0x422b2; continue; }
            pc = 0x422a0;
        }
        0x422a0 => {
            // block 422a0
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422a0 mov DWORD PTR [rbp-0x24],0x1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x1u64) as u32);
            // 422a7 jmp 0x422d2
            pc = 0x422d2; continue;
        }
        0x422a9 => {
            // block 422a9
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422a9 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 422b0 jmp 0x422d2
            pc = 0x422d2; continue;
        }
        0x422b2 => {
            // block 422b2
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422b2 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 422b4 jne 0x422c3
            if fl.ne() { pc = 0x422c3; continue; }
            pc = 0x422b6;
        }
        0x422b6 => {
            // block 422b6
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422b6 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 422b8 je 0x422c3
            if fl.e() { pc = 0x422c3; continue; }
            pc = 0x422ba;
        }
        0x422ba => {
            // block 422ba
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422ba mov DWORD PTR [rbp-0x24],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x2u64) as u32);
            // 422c1 jmp 0x422d2
            pc = 0x422d2; continue;
        }
        0x422c3 => {
            // block 422c3
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422c3 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 422c5 je 0x422d2
            if fl.e() { pc = 0x422d2; continue; }
            pc = 0x422c7;
        }
        0x422c7 => {
            // block 422c7
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422c7 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 422c9 je 0x422d2
            if fl.e() { pc = 0x422d2; continue; }
            pc = 0x422cb;
        }
        0x422cb => {
            // block 422cb
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422cb mov DWORD PTR [rbp-0x24],0x3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x3u64) as u32);
            pc = 0x422d2;
        }
        0x422d2 => {
            // block 422d2
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422d2 cmp r15d,0x4
            fl = sub::<32>((r15 as u32 as u64), 0x4u64).1;
            // 422d6 jl 0x4230a
            if fl.l() { pc = 0x4230a; continue; }
            pc = 0x422d8;
        }
        0x422d8 => {
            // block 422d8
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422d8 mov ecx,DWORD PTR [rbp-0x24]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 422db cmp rcx,0x3
            fl = sub::<64>(rcx, 0x3u64).1;
            // 422df ja 0x4230a
            if fl.a() { pc = 0x4230a; continue; }
            pc = 0x422e1;
        }
        0x422e1 => {
            // block 422e1
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422e1 lea rdx,[rip+0x3c] # 0x42324
            rdx = m.env.code_va(0x42324);
            // 422e8 movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // 422ec add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // 422ef jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x422f1 => {
            // block 422f1
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422f1 mov DWORD PTR [rbp-0x24],0x4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x4u64) as u32);
            // 422f8 jmp 0x4230a
            pc = 0x4230a; continue;
        }
        0x422fa => {
            // block 422fa
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422fa mov DWORD PTR [rbp-0x24],0x5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x5u64) as u32);
            // 42301 jmp 0x4230a
            pc = 0x4230a; continue;
        }
        0x42303 => {
            // block 42303
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 42303 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42303 mov DWORD PTR [rbp-0x24],0x6
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x6u64) as u32);
            pc = 0x4230a;
        }
        0x4230a => {
            // block 4230a
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 4230a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4230a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4230c je 0x42314
            if fl.e() { pc = 0x42314; continue; }
            pc = 0x4230e;
        }
        0x4230e => {
            // block 4230e
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 4230e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4230e mov ecx,DWORD PTR [rbp-0x24]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 42311 mov DWORD PTR [r14],ecx
            m.w32(r14, ((rcx as u32 as u64)) as u32);
            pc = 0x42314;
        }
        0x42314 => {
            // block 42314
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 42314 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42314 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 42318 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42319 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4231b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4231d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4231f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42320 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43edc size=354
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043edc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x43edc;
    loop {
        match pc {
        0x43edc => {
            // block 43edc
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43edc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43edc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43edd mov rbp,rsp
            rbp = rsp;
            // 43ee0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 43ee2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43ee4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 43ee6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 43ee8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43ee9 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 43eed mov r12,rdx
            r12 = rdx;
            // 43ef0 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 43ef3 mov r15,rdi
            r15 = rdi;
            // 43ef6 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 43efd mov BYTE PTR [rbp-0x2d],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3), (0x0u64) as u8);
            // 43f01 lea rbx,[r12+0x8]
            rbx = r12.wrapping_add(0x8);
            // 43f06 mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 43f0b mov rdi,r12
            rdi = r12;
            // 43f0e call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f13 mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 43f1c mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 43f20 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 43f29 cmp r14d,0x7
            fl = sub::<32>((r14 as u32 as u64), 0x7u64).1;
            // 43f2d ja 0x43fae
            if fl.a() { pc = 0x43fae; continue; }
            pc = 0x43f2f;
        }
        0x43f2f => {
            // block 43f2f
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f2f movzx ebx,r14w
            rbx = ((r14 as u16 as u64)) as u32 as u64;
            // 43f33 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43f37 call 0xcf820
            { let r = { let t = m.env.code_va(0xcf820); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f3c movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 43f3f cmp ebx,eax
            fl = sub::<32>((rbx as u32 as u64), (rax as u32 as u64)).1;
            // 43f41 jae 0x43fae
            if fl.ae() { pc = 0x43fae; continue; }
            pc = 0x43f43;
        }
        0x43f43 => {
            // block 43f43
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f43 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43f46 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 43f4a mov rdi,r15
            rdi = r15;
            // 43f4d mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 43f50 call QWORD PTR [rax+0x688]
            { let r = { let t = m.r64(rax.wrapping_add(0x688)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f56 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43f58 je 0x43fae
            if fl.e() { pc = 0x43fae; continue; }
            pc = 0x43f5a;
        }
        0x43f5a => {
            // block 43f5a
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f5a mov ebx,DWORD PTR [rbp-0x2c]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 43f5d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43f60 lea rdx,[rbp-0x2d]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd3);
            // 43f64 mov rdi,r15
            rdi = r15;
            // 43f67 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 43f6a call QWORD PTR [rax+0x7e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x7e0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f70 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 43f72 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43f74 mov r13d,0x0
            r13 = (0x0u64) as u32 as u64;
            // 43f7a je 0x43fb1
            if fl.e() { pc = 0x43fb1; continue; }
            pc = 0x43f7c;
        }
        0x43f7c => {
            // block 43f7c
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f7c cmp BYTE PTR [rbp-0x2d],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3)) as u64, 0x0u64).1;
            // 43f80 je 0x43fc3
            if fl.e() { pc = 0x43fc3; continue; }
            pc = 0x43f82;
        }
        0x43f82 => {
            // block 43f82
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f82 mov r15d,ebx
            r15 = ((rbx as u32 as u64)) as u32 as u64;
            // 43f85 shr r15d,1
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x1u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // 43f88 mov bx,0x1
            rbx = (rbx & !0xffff) | ((0x1u64) as u16 as u64);
            // 43f8c lea r14,[rbp-0x34]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            pc = 0x43f90;
        }
        0x43f90 => {
            // block 43f90
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f90 mov DWORD PTR [rbp-0x34],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rcx as u32 as u64)) as u32);
            // 43f93 mov rdi,r12
            rdi = r12;
            // 43f96 mov rsi,r14
            rsi = r14;
            // 43f99 call 0xebb58
            { let r = { let t = m.env.stub_va(0x325); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f9e movzx ecx,bx
            rcx = ((rbx as u16 as u64)) as u32 as u64;
            // 43fa1 lea ebx,[rbx+0x1]
            rbx = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 43fa4 cmp ecx,r15d
            fl = sub::<32>((rcx as u32 as u64), (r15 as u32 as u64)).1;
            // 43fa7 mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // 43faa jbe 0x43f90
            if fl.be() { pc = 0x43f90; continue; }
            pc = 0x43fac;
        }
        0x43fac => {
            // block 43fac
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fac jmp 0x43fb1
            pc = 0x43fb1; continue;
        }
        0x43fae => {
            // block 43fae
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fae xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x43fb1;
        }
        0x43fb1 => {
            // block 43fb1
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fb1 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 43fb4 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 43fb8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43fb9 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 43fbb pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 43fbd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43fbf pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 43fc1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43fc2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x43fc3 => {
            // block 43fc3
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fc3 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43fc7 call 0xcc811
            { let r = { let t = m.env.code_va(0xcc811); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43fcc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43fce je 0x44036
            if fl.e() { pc = 0x44036; continue; }
            pc = 0x43fd0;
        }
        0x43fd0 => {
            // block 43fd0
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fd0 mov DWORD PTR [rbp-0x38],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u32);
            // 43fd7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43fda cmp r14d,0x3
            fl = sub::<32>((r14 as u32 as u64), 0x3u64).1;
            // 43fde setg cl
            rcx = (rcx & !0xff) | (((fl.g() as u64)) as u8 as u64);
            // 43fe1 movzx esi,cl
            rsi = ((rcx as u8 as u64)) as u32 as u64;
            // 43fe4 or esi,0x1f0
            { let r = (rsi as u32 as u64) | 0x1f0u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 43fea lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 43fee xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 43ff1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 43ff6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 43ff9 mov rdi,r15
            rdi = r15;
            // 43ffc call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43fff test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44001 je 0x43fb1
            if fl.e() { pc = 0x43fb1; continue; }
            pc = 0x44003;
        }
        0x44003 => {
            // block 44003
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 44003 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44003 mov DWORD PTR [rbp-0x3c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), (0x0u64) as u32);
            // 4400a shl r14d,0x3
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x3u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 4400e lea r15,[rbp-0x3c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            pc = 0x44012;
        }
        0x44012 => {
            // block 44012
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 44012 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44012 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // 44015 lea ecx,[r14+r13*1]
            rcx = (r14.wrapping_add(r13)) as u32 as u64;
            // 44019 bt eax,ecx
            fl.cf = bit::<32>((rax as u32 as u64), (rcx as u32 as u64));
            // 4401c jae 0x44029
            if fl.ae() { pc = 0x44029; continue; }
            pc = 0x4401e;
        }
        0x4401e => {
            // block 4401e
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 4401e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4401e mov rdi,r12
            rdi = r12;
            // 44021 mov rsi,r15
            rsi = r15;
            // 44024 call 0xebb52
            { let r = { let t = m.env.stub_va(0x31f); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x44029;
        }
        0x44029 => {
            // block 44029
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 44029 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44029 inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // 4402c mov DWORD PTR [rbp-0x3c],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((r13 as u32 as u64)) as u32);
            // 44030 cmp r13d,0x8
            fl = sub::<32>((r13 as u32 as u64), 0x8u64).1;
            // 44034 jb 0x44012
            if fl.b() { pc = 0x44012; continue; }
            pc = 0x44036;
        }
        0x44036 => {
            // block 44036
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 44036 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44036 mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // 44039 jmp 0x43fb1
            pc = 0x43fb1; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5855b size=335
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005855b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5855b;
    loop {
        match pc {
        0x5855b => {
            // block 5855b
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5855b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5855b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5855c mov rbp,rsp
            rbp = rsp;
            // 5855f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 58561 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 58563 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 58565 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 58567 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 58568 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 58569 mov rbx,rsi
            rbx = rsi;
            // 5856c mov r14,rdi
            r14 = rdi;
            // 5856f mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 58572 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 58576 cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // 58579 je 0x58593
            if fl.e() { pc = 0x58593; continue; }
            pc = 0x5857b;
        }
        0x5857b => {
            // block 5857b
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5857b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5857b lea rsi,[rcx-0x2]
            rsi = rcx.wrapping_add(0xffff_ffff_ffff_fffe);
            // 5857f sub rsi,rdi
            { let (r, f) = sub::<64>(rsi, rdi);
              rsi = r;
              fl = f; }
            // 58582 not rsi
            { let v = !rsi; rsi = v; }
            // 58585 and rsi,0xfffffffffffffffe
            { let r = rsi & 0xffff_ffff_ffff_fffeu64; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // 58589 add rsi,rcx
            { let (r, f) = add::<64>(rsi, rcx);
              rsi = r;
              fl = f; }
            // 5858c mov QWORD PTR [rbx+0x8],rsi
            m.w64(rbx.wrapping_add(0x8), (rsi) as u64);
            // 58590 mov rdi,rsi
            rdi = rsi;
            pc = 0x58593;
        }
        0x58593 => {
            // block 58593
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58593 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58593 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58595 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 58598 je 0x5869b
            if fl.e() { pc = 0x5869b; continue; }
            pc = 0x5859e;
        }
        0x5859e => {
            // block 5859e
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5859e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5859e cmp edx,0x6
            fl = sub::<32>((rdx as u32 as u64), 0x6u64).1;
            // 585a1 jb 0x5869b
            if fl.b() { pc = 0x5869b; continue; }
            pc = 0x585a7;
        }
        0x585a7 => {
            // block 585a7
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 585a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 585a7 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 585a9 mov edx,0xaaaaaaab
            rdx = (0xaaaaaaabu64) as u32 as u64;
            // 585ae imul rdx,rcx
            { let (r, f) = imul::<64>(rdx, rcx); rdx = r; fl = f; }
            // 585b2 shr rdx,0x22
            { let (r, f) = shr::<64>(rdx, 0x22u64, fl); rdx = r; fl = f; }
            // 585b6 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 585b8 lea ecx,[rdx+rdx*2]
            rcx = (rdx.wrapping_add(rdx.wrapping_mul(2))) as u32 as u64;
            // 585bb shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 585be mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 585c0 cmp ecx,0x6
            fl = sub::<32>((rcx as u32 as u64), 0x6u64).1;
            // 585c3 jb 0x5869b
            if fl.b() { pc = 0x5869b; continue; }
            pc = 0x585c9;
        }
        0x585c9 => {
            // block 585c9
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 585c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 585c9 mov ecx,ecx
            rcx = ((rcx as u32 as u64)) as u32 as u64;
            // 585cb mov r12d,0xaaaaaaab
            r12 = (0xaaaaaaabu64) as u32 as u64;
            // 585d1 imul r12,rcx
            { let (r, f) = imul::<64>(r12, rcx); r12 = r; fl = f; }
            // 585d5 shr r12,0x22
            { let (r, f) = shr::<64>(r12, 0x22u64, fl); r12 = r; fl = f; }
            // 585d9 mov r13d,0x1
            r13 = (0x1u64) as u32 as u64;
            // 585df lea r15,[rbp-0x2a]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd6);
            // 585e3 jmp 0x585ec
            pc = 0x585ec; continue;
        }
        0x585e5 => {
            // block 585e5
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 585e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 585e5 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 585e9 inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            pc = 0x585ec;
        }
        0x585ec => {
            // block 585ec
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 585ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 585ec mov ecx,DWORD PTR [r14+r13*4-0x4]
            rcx = (m.r32(r14.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 585f1 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 585f3 and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 585f9 mov WORD PTR [rbp-0x2a],si
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd6), ((rsi as u16 as u64)) as u16);
            // 585fd mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 58601 cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // 58604 jae 0x58613
            if fl.ae() { pc = 0x58613; continue; }
            pc = 0x58606;
        }
        0x58606 => {
            // block 58606
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58606 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58606 mov WORD PTR [rdi],si
            m.w16(rdi, ((rsi as u16 as u64)) as u16);
            // 58609 add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // 5860d mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // 58611 jmp 0x5862b
            pc = 0x5862b; continue;
        }
        0x58613 => {
            // block 58613
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58613 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58613 mov rdi,rbx
            rdi = rbx;
            // 58616 mov rsi,r15
            rsi = r15;
            // 58619 call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5861e mov ecx,DWORD PTR [r14+r13*4-0x4]
            rcx = (m.r32(r14.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 58623 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 58627 mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0x5862b;
        }
        0x5862b => {
            // block 5862b
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5862b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5862b mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 5862d shr esi,0xa
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xau64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 58630 and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 58636 mov WORD PTR [rbp-0x2c],si
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rsi as u16 as u64)) as u16);
            // 5863a cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // 5863d jae 0x5864c
            if fl.ae() { pc = 0x5864c; continue; }
            pc = 0x5863f;
        }
        0x5863f => {
            // block 5863f
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5863f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5863f mov WORD PTR [rdi],si
            m.w16(rdi, ((rsi as u16 as u64)) as u16);
            // 58642 add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // 58646 mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // 5864a jmp 0x58665
            pc = 0x58665; continue;
        }
        0x5864c => {
            // block 5864c
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5864c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5864c mov rdi,rbx
            rdi = rbx;
            // 5864f lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 58653 call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58658 mov ecx,DWORD PTR [r14+r13*4-0x4]
            rcx = (m.r32(r14.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 5865d mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 58661 mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0x58665;
        }
        0x58665 => {
            // block 58665
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58665 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58665 shr ecx,0x14
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x14u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 58668 and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5866e mov WORD PTR [rbp-0x2e],cx
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd2), ((rcx as u16 as u64)) as u16);
            // 58672 cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // 58675 jae 0x58684
            if fl.ae() { pc = 0x58684; continue; }
            pc = 0x58677;
        }
        0x58677 => {
            // block 58677
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58677 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58677 mov WORD PTR [rdi],cx
            m.w16(rdi, ((rcx as u16 as u64)) as u16);
            // 5867a add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // 5867e mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // 58682 jmp 0x58690
            pc = 0x58690; continue;
        }
        0x58684 => {
            // block 58684
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58684 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58684 mov rdi,rbx
            rdi = rbx;
            // 58687 lea rsi,[rbp-0x2e]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd2);
            // 5868b call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x58690;
        }
        0x58690 => {
            // block 58690
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58690 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58690 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 58693 jb 0x585e5
            if fl.b() { pc = 0x585e5; continue; }
            pc = 0x58699;
        }
        0x58699 => {
            // block 58699
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58699 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58699 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x5869b;
        }
        0x5869b => {
            // block 5869b
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5869b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5869b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5869f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 586a0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 586a2 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 586a4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 586a6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 586a8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 586a9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x586aa size=332
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000586aa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x586aa;
    loop {
        match pc {
        0x586aa => {
            // block 586aa
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586aa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 586ab mov rbp,rsp
            rbp = rsp;
            // 586ae push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 586b0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 586b2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 586b4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 586b6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 586b7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 586b8 mov rbx,rdx
            rbx = rdx;
            // 586bb mov r14,rsi
            r14 = rsi;
            // 586be mov r15,rdi
            r15 = rdi;
            // 586c1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 586c4 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 586c8 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 586cb je 0x586e5
            if fl.e() { pc = 0x586e5; continue; }
            pc = 0x586cd;
        }
        0x586cd => {
            // block 586cd
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586cd lea rdx,[rcx-0x2]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fffe);
            // 586d1 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 586d4 not rdx
            { let v = !rdx; rdx = v; }
            // 586d7 and rdx,0xfffffffffffffffe
            { let r = rdx & 0xffff_ffff_ffff_fffeu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 586db add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 586de mov QWORD PTR [rbx+0x8],rdx
            m.w64(rbx.wrapping_add(0x8), (rdx) as u64);
            // 586e2 mov rax,rdx
            rax = rdx;
            pc = 0x586e5;
        }
        0x586e5 => {
            // block 586e5
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586e5 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 586e8 je 0x58716
            if fl.e() { pc = 0x58716; continue; }
            pc = 0x586ea;
        }
        0x586ea => {
            // block 586ea
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586ea cmp DWORD PTR [r14],0x0
            fl = sub::<32>(m.r32(r14) as u64, 0x0u64).1;
            // 586ee je 0x58716
            if fl.e() { pc = 0x58716; continue; }
            pc = 0x586f0;
        }
        0x586f0 => {
            // block 586f0
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586f0 mov ecx,DWORD PTR [r14+0x4]
            rcx = (m.r32(r14.wrapping_add(0x4)) as u64) as u32 as u64;
            // 586f4 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 586f6 je 0x58716
            if fl.e() { pc = 0x58716; continue; }
            pc = 0x586f8;
        }
        0x586f8 => {
            // block 586f8
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586f8 cmp WORD PTR [r14+0x34],0x0
            fl = sub::<16>(m.r16(r14.wrapping_add(0x34)) as u64, 0x0u64).1;
            // 586fe je 0x58716
            if fl.e() { pc = 0x58716; continue; }
            pc = 0x58700;
        }
        0x58700 => {
            // block 58700
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58700 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58700 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 58703 cmp DWORD PTR [r14+0x24],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x24)) as u64, 0x0u64).1;
            // 58708 je 0x58719
            if fl.e() { pc = 0x58719; continue; }
            pc = 0x5870a;
        }
        0x5870a => {
            // block 5870a
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5870a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5870a cmp ecx,0x6
            fl = sub::<32>((rcx as u32 as u64), 0x6u64).1;
            // 5870d jb 0x58719
            if fl.b() { pc = 0x58719; continue; }
            pc = 0x5870f;
        }
        0x5870f => {
            // block 5870f
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5870f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5870f cmp DWORD PTR [r14+0x18],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x18)) as u64, 0x0u64).1;
            // 58714 je 0x5872b
            if fl.e() { pc = 0x5872b; continue; }
            pc = 0x58716;
        }
        0x58716 => {
            // block 58716
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58716 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58716 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x58719;
        }
        0x58719 => {
            // block 58719
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58719 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58719 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 5871c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 58720 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 58721 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 58723 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 58725 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 58727 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 58729 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5872a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x5872b => {
            // block 5872b
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5872b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5872b mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // 5872e cmp DWORD PTR [r14+0x8],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x8)) as u64, 0x0u64).1;
            // 58733 je 0x58719
            if fl.e() { pc = 0x58719; continue; }
            pc = 0x58735;
        }
        0x58735 => {
            // block 58735
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58735 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58735 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 58738 jmp 0x58741
            pc = 0x58741; continue;
        }
        0x5873a => {
            // block 5873a
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5873a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5873a mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 5873e inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            pc = 0x58741;
        }
        0x58741 => {
            // block 58741
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58741 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58741 mov ecx,DWORD PTR [r15+r13*4]
            rcx = (m.r32(r15.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 58745 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 58747 and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5874d mov WORD PTR [rbp-0x2a],si
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd6), ((rsi as u16 as u64)) as u16);
            // 58751 mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 58755 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 58758 jae 0x58767
            if fl.ae() { pc = 0x58767; continue; }
            pc = 0x5875a;
        }
        0x5875a => {
            // block 5875a
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5875a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5875a mov WORD PTR [rax],si
            m.w16(rax, ((rsi as u16 as u64)) as u16);
            // 5875d add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // 58761 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 58765 jmp 0x5877f
            pc = 0x5877f; continue;
        }
        0x58767 => {
            // block 58767
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58767 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58767 mov rdi,rbx
            rdi = rbx;
            // 5876a lea rsi,[rbp-0x2a]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd6);
            // 5876e call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58773 mov ecx,DWORD PTR [r15+r13*4]
            rcx = (m.r32(r15.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 58777 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 5877b mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0x5877f;
        }
        0x5877f => {
            // block 5877f
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5877f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5877f mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 58781 shr esi,0xa
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xau64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 58784 and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5878a mov WORD PTR [rbp-0x2c],si
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rsi as u16 as u64)) as u16);
            // 5878e cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 58791 jae 0x587a0
            if fl.ae() { pc = 0x587a0; continue; }
            pc = 0x58793;
        }
        0x58793 => {
            // block 58793
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58793 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58793 mov WORD PTR [rax],si
            m.w16(rax, ((rsi as u16 as u64)) as u16);
            // 58796 add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // 5879a mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 5879e jmp 0x587b8
            pc = 0x587b8; continue;
        }
        0x587a0 => {
            // block 587a0
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587a0 mov rdi,rbx
            rdi = rbx;
            // 587a3 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 587a7 call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 587ac mov ecx,DWORD PTR [r15+r13*4]
            rcx = (m.r32(r15.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 587b0 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 587b4 mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0x587b8;
        }
        0x587b8 => {
            // block 587b8
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587b8 shr ecx,0x14
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x14u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 587bb and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 587c1 mov WORD PTR [rbp-0x2e],cx
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd2), ((rcx as u16 as u64)) as u16);
            // 587c5 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 587c8 jae 0x587d7
            if fl.ae() { pc = 0x587d7; continue; }
            pc = 0x587ca;
        }
        0x587ca => {
            // block 587ca
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587ca mov WORD PTR [rax],cx
            m.w16(rax, ((rcx as u16 as u64)) as u16);
            // 587cd add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // 587d1 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 587d5 jmp 0x587e3
            pc = 0x587e3; continue;
        }
        0x587d7 => {
            // block 587d7
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587d7 mov rdi,rbx
            rdi = rbx;
            // 587da lea rsi,[rbp-0x2e]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd2);
            // 587de call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x587e3;
        }
        0x587e3 => {
            // block 587e3
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587e3 lea eax,[r13+0x1]
            rax = (r13.wrapping_add(0x1)) as u32 as u64;
            // 587e7 cmp eax,DWORD PTR [r14+0x8]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x8)) as u64).1;
            // 587eb jae 0x58719
            if fl.ae() { pc = 0x58719; continue; }
            pc = 0x587f1;
        }
        0x587f1 => {
            // block 587f1
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587f1 jmp 0x5873a
            pc = 0x5873a; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x645eb size=351
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000645eb<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x645eb;
    loop {
        match pc {
        0x645eb => {
            // block 645eb
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 645eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 645eb push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 645ec mov rbp,rsp
            rbp = rsp;
            // 645ef push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 645f0 movzx ebx,WORD PTR [rdi]
            rbx = (m.r16(rdi) as u64) as u32 as u64;
            // 645f3 cmp ebx,0xc350
            fl = sub::<32>((rbx as u32 as u64), 0xc350u64).1;
            // 645f9 ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x645fb;
        }
        0x645fb => {
            // block 645fb
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 645fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 645fb movzx eax,WORD PTR [rdi+0x2]
            rax = (m.r16(rdi.wrapping_add(0x2)) as u64) as u32 as u64;
            // 645ff cmp eax,0xc350
            fl = sub::<32>((rax as u32 as u64), 0xc350u64).1;
            // 64604 ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x64606;
        }
        0x64606 => {
            // block 64606
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64606 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64606 movzx edx,WORD PTR [rdi+0x4]
            rdx = (m.r16(rdi.wrapping_add(0x4)) as u64) as u32 as u64;
            // 6460a cmp edx,0xc350
            fl = sub::<32>((rdx as u32 as u64), 0xc350u64).1;
            // 64610 ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x64612;
        }
        0x64612 => {
            // block 64612
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64612 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64612 movzx ecx,WORD PTR [rdi+0x6]
            rcx = (m.r16(rdi.wrapping_add(0x6)) as u64) as u32 as u64;
            // 64616 cmp ecx,0xc350
            fl = sub::<32>((rcx as u32 as u64), 0xc350u64).1;
            // 6461c ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x6461e;
        }
        0x6461e => {
            // block 6461e
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 6461e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6461e movzx r11d,WORD PTR [rdi+0x8]
            r11 = (m.r16(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 64623 cmp r11d,0xc350
            fl = sub::<32>((r11 as u32 as u64), 0xc350u64).1;
            // 6462a ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x6462c;
        }
        0x6462c => {
            // block 6462c
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 6462c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6462c movzx r10d,WORD PTR [rdi+0xa]
            r10 = (m.r16(rdi.wrapping_add(0xa)) as u64) as u32 as u64;
            // 64631 cmp r10d,0xc350
            fl = sub::<32>((r10 as u32 as u64), 0xc350u64).1;
            // 64638 ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x6463a;
        }
        0x6463a => {
            // block 6463a
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 6463a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6463a movzx r9d,WORD PTR [rdi+0xc]
            r9 = (m.r16(rdi.wrapping_add(0xc)) as u64) as u32 as u64;
            // 6463f cmp r9d,0xc350
            fl = sub::<32>((r9 as u32 as u64), 0xc350u64).1;
            // 64646 ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x64648;
        }
        0x64648 => {
            // block 64648
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64648 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64648 movzx r8d,WORD PTR [rdi+0xe]
            r8 = (m.r16(rdi.wrapping_add(0xe)) as u64) as u32 as u64;
            // 6464d cmp r8d,0xc350
            fl = sub::<32>((r8 as u32 as u64), 0xc350u64).1;
            // 64654 jbe 0x6465b
            if fl.be() { pc = 0x6465b; continue; }
            pc = 0x64656;
        }
        0x64656 => {
            // block 64656
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64656 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64656 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x64658;
        }
        0x64658 => {
            // block 64658
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64658 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64658 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 64659 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6465a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x6465b => {
            // block 6465b
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 6465b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6465b cvtsi2sd xmm1,ebx
            x1 = set_lo64(x1, (sx::<32>((rbx as u32 as u64)) as f64).to_bits());
            // 6465f movsd xmm0,QWORD PTR [rip+0x9c139] # 0x1007a0
            x0 = m.r64(m.env.data_va(85869)) as u128;
            // 64667 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 6466b cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 6466f movss DWORD PTR [rsi],xmm1
            m.w32(rsi, x1 as u32);
            // 64673 xorps xmm1,xmm1
            x1 = 0;
            // 64676 cvtsi2sd xmm1,eax
            x1 = set_lo64(x1, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 6467a mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 6467e cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 64682 movss DWORD PTR [rsi+0x4],xmm1
            m.w32(rsi.wrapping_add(0x4), x1 as u32);
            // 64687 xorps xmm1,xmm1
            x1 = 0;
            // 6468a cvtsi2sd xmm1,edx
            x1 = set_lo64(x1, (sx::<32>((rdx as u32 as u64)) as f64).to_bits());
            // 6468e mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 64692 cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 64696 movss DWORD PTR [rsi+0x8],xmm1
            m.w32(rsi.wrapping_add(0x8), x1 as u32);
            // 6469b xorps xmm1,xmm1
            x1 = 0;
            // 6469e cvtsi2sd xmm1,ecx
            x1 = set_lo64(x1, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // 646a2 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646a6 cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 646aa movss DWORD PTR [rsi+0xc],xmm1
            m.w32(rsi.wrapping_add(0xc), x1 as u32);
            // 646af xorps xmm1,xmm1
            x1 = 0;
            // 646b2 cvtsi2sd xmm1,r11d
            x1 = set_lo64(x1, (sx::<32>((r11 as u32 as u64)) as f64).to_bits());
            // 646b7 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646bb cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 646bf movss DWORD PTR [rsi+0x10],xmm1
            m.w32(rsi.wrapping_add(0x10), x1 as u32);
            // 646c4 xorps xmm1,xmm1
            x1 = 0;
            // 646c7 cvtsi2sd xmm1,r10d
            x1 = set_lo64(x1, (sx::<32>((r10 as u32 as u64)) as f64).to_bits());
            // 646cc mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646d0 cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 646d4 movss DWORD PTR [rsi+0x14],xmm1
            m.w32(rsi.wrapping_add(0x14), x1 as u32);
            // 646d9 xorps xmm1,xmm1
            x1 = 0;
            // 646dc cvtsi2sd xmm1,r9d
            x1 = set_lo64(x1, (sx::<32>((r9 as u32 as u64)) as f64).to_bits());
            // 646e1 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646e5 cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 646e9 movss DWORD PTR [rsi+0x18],xmm1
            m.w32(rsi.wrapping_add(0x18), x1 as u32);
            // 646ee xorps xmm1,xmm1
            x1 = 0;
            // 646f1 cvtsi2sd xmm1,r8d
            x1 = set_lo64(x1, (sx::<32>((r8 as u32 as u64)) as f64).to_bits());
            // 646f6 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646fa cvtsd2ss xmm0,xmm1
            x0 = set_lo32(x0, (f64_of(x1) as f32).to_bits());
            // 646fe movss DWORD PTR [rsi+0x1c],xmm0
            m.w32(rsi.wrapping_add(0x1c), x0 as u32);
            // 64703 movzx eax,WORD PTR [rdi+0x12]
            rax = (m.r16(rdi.wrapping_add(0x12)) as u64) as u32 as u64;
            // 64707 xorps xmm0,xmm0
            x0 = 0;
            // 6470a cvtsi2sd xmm0,eax
            x0 = set_lo64(x0, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 6470e mulsd xmm0,QWORD PTR [rip+0x9c092] # 0x1007a8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(85877)) as u128))).to_bits());
            // 64716 cvtsd2ss xmm0,xmm0
            x0 = set_lo32(x0, (f64_of(x0) as f32).to_bits());
            // 6471a movss DWORD PTR [rsi+0x24],xmm0
            m.w32(rsi.wrapping_add(0x24), x0 as u32);
            // 6471f mov ax,WORD PTR [rdi+0x10]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x10)) as u64) as u16 as u64);
            // 64723 mov WORD PTR [rsi+0x20],ax
            m.w16(rsi.wrapping_add(0x20), ((rax as u16 as u64)) as u16);
            // 64727 mov ax,WORD PTR [rdi+0x14]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x14)) as u64) as u16 as u64);
            // 6472b mov WORD PTR [rsi+0x28],ax
            m.w16(rsi.wrapping_add(0x28), ((rax as u16 as u64)) as u16);
            // 6472f mov ax,WORD PTR [rdi+0x16]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x16)) as u64) as u16 as u64);
            // 64733 mov WORD PTR [rsi+0x2a],ax
            m.w16(rsi.wrapping_add(0x2a), ((rax as u16 as u64)) as u16);
            // 64737 mov al,BYTE PTR [rdi+0x18]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x18)) as u64) as u8 as u64);
            // 6473a mov BYTE PTR [rsi+0x2c],al
            m.w8(rsi.wrapping_add(0x2c), ((rax as u8 as u64)) as u8);
            // 6473d mov al,BYTE PTR [rdi+0x19]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x19)) as u64) as u8 as u64);
            // 64740 mov BYTE PTR [rsi+0x2d],al
            m.w8(rsi.wrapping_add(0x2d), ((rax as u8 as u64)) as u8);
            // 64743 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 64745 jmp 0x64658
            pc = 0x64658; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x65094 size=302
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00065094<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x65094;
    loop {
        match pc {
        0x65094 => {
            // block 65094
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 65094 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65094 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 65095 mov rbp,rsp
            rbp = rsp;
            // 65098 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6509a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6509c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 6509e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 650a0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 650a1 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 650a5 mov r14,rcx
            r14 = rcx;
            // 650a8 mov rbx,rdx
            rbx = rdx;
            // 650ab mov r13,rdi
            r13 = rdi;
            // 650ae xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 650b0 test r13,r13
            fl = logic::<64>(r13 & r13);
            // 650b3 je 0x6519f
            if fl.e() { pc = 0x6519f; continue; }
            pc = 0x650b9;
        }
        0x650b9 => {
            // block 650b9
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 650b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 650b9 mov rax,r14
            rax = r14;
            // 650bc sub rax,rsi
            { let (r, f) = sub::<64>(rax, rsi);
              rax = r;
              fl = f; }
            // 650bf mov rcx,QWORD PTR [r8+0x18]
            rcx = m.r64(r8.wrapping_add(0x18)) as u64;
            // 650c3 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 650c6 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 650c9 cmovg r15,rcx
            if fl.g() { r15 = rcx; }
            // 650cd mov r12,rbx
            r12 = rbx;
            // 650d0 sub r12,rsi
            { let (r, f) = sub::<64>(r12, rsi);
              r12 = r;
              fl = f; }
            // 650d3 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 650d6 jle 0x6510f
            if fl.le() { pc = 0x6510f; continue; }
            pc = 0x650d8;
        }
        0x650d8 => {
            // block 650d8
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 650d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 650d8 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 650dc mov rdi,r13
            rdi = r13;
            // 650df mov rdx,r12
            rdx = r12;
            // 650e2 mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // 650e6 mov QWORD PTR [rbp-0x48],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r14) as u64);
            // 650ea mov rbx,r8
            rbx = r8;
            // 650ed mov r14d,r9d
            r14 = ((r9 as u32 as u64)) as u32 as u64;
            // 650f0 call QWORD PTR [rax+0x60]
            { let r = { let t = m.r64(rax.wrapping_add(0x60)) as u64; m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 650f3 mov r9d,r14d
            r9 = ((r14 as u32 as u64)) as u32 as u64;
            // 650f6 mov r8,rbx
            r8 = rbx;
            // 650f9 mov r14,QWORD PTR [rbp-0x48]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 650fd mov rbx,QWORD PTR [rbp-0x50]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 65101 mov rcx,rax
            rcx = rax;
            // 65104 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 65106 cmp rcx,r12
            fl = sub::<64>(rcx, r12).1;
            // 65109 jne 0x6519f
            if fl.ne() { pc = 0x6519f; continue; }
            pc = 0x6510f;
        }
        0x6510f => {
            // block 6510f
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 6510f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6510f test r15,r15
            fl = logic::<64>(r15 & r15);
            // 65112 jle 0x6516c
            if fl.le() { pc = 0x6516c; continue; }
            pc = 0x65114;
        }
        0x65114 => {
            // block 65114
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 65114 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65114 mov QWORD PTR [rbp-0x48],r8
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r8) as u64);
            // 65118 movsx edx,r9b
            rdx = (sx::<8>((r9 as u8 as u64)) as u64) as u32 as u64;
            // 6511c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 65120 mov rsi,r15
            rsi = r15;
            // 65123 call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65128 test BYTE PTR [rbp-0x40],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64 & 0x1u64);
            // 6512c jne 0x65137
            if fl.ne() { pc = 0x65137; continue; }
            pc = 0x6512e;
        }
        0x6512e => {
            // block 6512e
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 6512e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6512e mov r12,r14
            r12 = r14;
            // 65131 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 65135 jmp 0x6513e
            pc = 0x6513e; continue;
        }
        0x65137 => {
            // block 65137
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 65137 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65137 mov r12,r14
            r12 = r14;
            // 6513a mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x6513e;
        }
        0x6513e => {
            // block 6513e
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 6513e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6513e mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 65142 mov rax,QWORD PTR [rax+0x60]
            rax = m.r64(rax.wrapping_add(0x60)) as u64;
            // 65146 mov rdi,r13
            rdi = r13;
            // 65149 mov rdx,r15
            rdx = r15;
            // 6514c call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6514e mov r14,rax
            r14 = rax;
            // 65151 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 65155 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6515a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6515c cmp r14,r15
            fl = sub::<64>(r14, r15).1;
            // 6515f cmovne r13,rax
            if fl.ne() { r13 = rax; }
            // 65163 mov r8,QWORD PTR [rbp-0x48]
            r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 65167 mov r14,r12
            r14 = r12;
            // 6516a jne 0x6519f
            if fl.ne() { pc = 0x6519f; continue; }
            pc = 0x6516c;
        }
        0x6516c => {
            // block 6516c
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 6516c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6516c sub r14,rbx
            { let (r, f) = sub::<64>(r14, rbx);
              r14 = r;
              fl = f; }
            // 6516f test r14,r14
            fl = logic::<64>(r14 & r14);
            // 65172 jle 0x65194
            if fl.le() { pc = 0x65194; continue; }
            pc = 0x65174;
        }
        0x65174 => {
            // block 65174
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 65174 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65174 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 65178 mov rdi,r13
            rdi = r13;
            // 6517b mov rsi,rbx
            rsi = rbx;
            // 6517e mov rdx,r14
            rdx = r14;
            // 65181 mov rbx,r8
            rbx = r8;
            // 65184 call QWORD PTR [rax+0x60]
            { let r = { let t = m.r64(rax.wrapping_add(0x60)) as u64; m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65187 mov r8,rbx
            r8 = rbx;
            // 6518a mov rcx,rax
            rcx = rax;
            // 6518d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6518f cmp rcx,r14
            fl = sub::<64>(rcx, r14).1;
            // 65192 jne 0x6519f
            if fl.ne() { pc = 0x6519f; continue; }
            pc = 0x65194;
        }
        0x65194 => {
            // block 65194
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 65194 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65194 mov QWORD PTR [r8+0x18],0x0
            m.w64(r8.wrapping_add(0x18), (0x0u64) as u64);
            // 6519c mov rax,r13
            rax = r13;
            pc = 0x6519f;
        }
        0x6519f => {
            // block 6519f
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 6519f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6519f add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 651a3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 651a4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 651a6 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 651a8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 651aa pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 651ac pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 651ad ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x651ae => {
            // block 651ae
            #[cfg(feature = "trace")] eprintln!("f_00065094 pc 651ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 651ae mov rbx,rax
            rbx = rax;
            // 651b1 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 651b5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 651ba mov rdi,rbx
            rdi = rbx;
            // 651bd call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68fea size=308
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068fea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68fea;
    loop {
        match pc {
        0x68fea => {
            // block 68fea
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 68fea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68fea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68feb mov rbp,rsp
            rbp = rsp;
            // 68fee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 68ff0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 68ff2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 68ff4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 68ff6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68ff7 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 68ffb mov r12,rdx
            r12 = rdx;
            // 68ffe mov DWORD PTR [rbp-0x2c],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rsi as u32 as u64)) as u32);
            // 69001 mov r15,rdi
            r15 = rdi;
            // 69004 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 6900b mov rsi,r12
            rsi = r12;
            // 6900e call 0xebbd6
            { let r = { let t = m.env.stub_va(0x3a3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69013 mov rbx,rax
            rbx = rax;
            // 69016 add r15,0x1a0
            { let (r, f) = add::<64>(r15, 0x1a0u64);
              r15 = r;
              fl = f; }
            // 6901d cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 69020 je 0x6910d
            if fl.e() { pc = 0x6910d; continue; }
            pc = 0x69026;
        }
        0x69026 => {
            // block 69026
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69026 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69026 movzx r10d,BYTE PTR [r12]
            r10 = (m.r8(r12) as u64) as u32 as u64;
            // 6902b mov r11b,r10b
            r11 = (r11 & !0xff) | (((r10 as u8 as u64)) as u8 as u64);
            // 6902e and r11b,0x1
            { let r = (r11 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r11 = (r11 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 69032 shr r10,1
            { let (r, f) = shr::<64>(r10, 0x1u64, fl); r10 = r; fl = f; }
            // 69035 mov r8,QWORD PTR [r12+0x8]
            r8 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 6903a mov r13,QWORD PTR [r12+0x10]
            r13 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 6903f lea r14,[r12+0x1]
            r14 = r12.wrapping_add(0x1);
            // 69044 mov QWORD PTR [rbp-0x40],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r14) as u64);
            pc = 0x69048;
        }
        0x69048 => {
            // block 69048
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69048 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69048 movzx edx,BYTE PTR [rbx+0x20]
            rdx = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 6904c mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 6904e and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 69050 jne 0x69057
            if fl.ne() { pc = 0x69057; continue; }
            pc = 0x69052;
        }
        0x69052 => {
            // block 69052
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69052 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69052 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 69055 jmp 0x6905b
            pc = 0x6905b; continue;
        }
        0x69057 => {
            // block 69057
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69057 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69057 mov rdx,QWORD PTR [rbx+0x28]
            rdx = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x6905b;
        }
        0x6905b => {
            // block 6905b
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6905b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6905b test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // 6905e mov rcx,r8
            rcx = r8;
            // 69061 cmove rcx,r10
            if fl.e() { rcx = r10; }
            // 69065 cmp rdx,rcx
            fl = sub::<64>(rdx, rcx).1;
            // 69068 jne 0x6910d
            if fl.ne() { pc = 0x6910d; continue; }
            pc = 0x6906e;
        }
        0x6906e => {
            // block 6906e
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6906e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6906e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 69070 jne 0x6907b
            if fl.ne() { pc = 0x6907b; continue; }
            pc = 0x69072;
        }
        0x69072 => {
            // block 69072
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69072 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69072 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // 69076 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 69079 jmp 0x6907f
            pc = 0x6907f; continue;
        }
        0x6907b => {
            // block 6907b
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6907b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6907b mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x6907f;
        }
        0x6907f => {
            // block 6907f
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6907f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6907f test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // 69082 mov rsi,r13
            rsi = r13;
            // 69085 cmove rsi,r14
            if fl.e() { rsi = r14; }
            // 69089 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6908b jne 0x690a9
            if fl.ne() { pc = 0x690a9; continue; }
            pc = 0x6908d;
        }
        0x6908d => {
            // block 6908d
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6908d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6908d test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 69090 je 0x690d5
            if fl.e() { pc = 0x690d5; continue; }
            pc = 0x69092;
        }
        0x69092 => {
            // block 69092
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69092 movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // 69095 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 69098 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 6909a jne 0x6910d
            if fl.ne() { pc = 0x6910d; continue; }
            pc = 0x6909c;
        }
        0x6909c => {
            // block 6909c
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6909c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6909c inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 6909f inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 690a2 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 690a5 jne 0x69092
            if fl.ne() { pc = 0x69092; continue; }
            pc = 0x690a7;
        }
        0x690a7 => {
            // block 690a7
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690a7 jmp 0x690d5
            pc = 0x690d5; continue;
        }
        0x690a9 => {
            // block 690a9
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690a9 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 690ac je 0x690d5
            if fl.e() { pc = 0x690d5; continue; }
            pc = 0x690ae;
        }
        0x690ae => {
            // block 690ae
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690ae mov r12,r8
            r12 = r8;
            // 690b1 mov QWORD PTR [rbp-0x38],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r15) as u64);
            // 690b5 mov r15,r10
            r15 = r10;
            // 690b8 mov r14b,r11b
            r14 = (r14 & !0xff) | (((r11 as u8 as u64)) as u8 as u64);
            // 690bb call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 690c0 mov r11b,r14b
            r11 = (r11 & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 690c3 mov r14,QWORD PTR [rbp-0x40]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 690c7 mov r10,r15
            r10 = r15;
            // 690ca mov r15,QWORD PTR [rbp-0x38]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 690ce mov r8,r12
            r8 = r12;
            // 690d1 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 690d3 jne 0x6910d
            if fl.ne() { pc = 0x6910d; continue; }
            pc = 0x690d5;
        }
        0x690d5 => {
            // block 690d5
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690d5 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 690d7 mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 690da cmp DWORD PTR [rbx+0x38],ecx
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x38)) as u64, (rcx as u32 as u64)).1;
            // 690dd je 0x6910f
            if fl.e() { pc = 0x6910f; continue; }
            pc = 0x690df;
        }
        0x690df => {
            // block 690df
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690df mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 690e3 test rax,rax
            fl = logic::<64>(rax & rax);
            // 690e6 je 0x690f5
            if fl.e() { pc = 0x690f5; continue; }
            pc = 0x690e8;
        }
        0x690e8 => {
            // block 690e8
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690e8 mov rbx,rax
            rbx = rax;
            // 690eb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 690ee test rax,rax
            fl = logic::<64>(rax & rax);
            // 690f1 jne 0x690e8
            if fl.ne() { pc = 0x690e8; continue; }
            pc = 0x690f3;
        }
        0x690f3 => {
            // block 690f3
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690f3 jmp 0x69104
            pc = 0x69104; continue;
        }
        0x690f5 => {
            // block 690f5
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690f5 mov rax,rbx
            rax = rbx;
            pc = 0x690f8;
        }
        0x690f8 => {
            // block 690f8
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690f8 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 690fc cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 690ff mov rax,rbx
            rax = rbx;
            // 69102 jne 0x690f8
            if fl.ne() { pc = 0x690f8; continue; }
            pc = 0x69104;
        }
        0x69104 => {
            // block 69104
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69104 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69104 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 69107 jne 0x69048
            if fl.ne() { pc = 0x69048; continue; }
            pc = 0x6910d;
        }
        0x6910d => {
            // block 6910d
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6910d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6910d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x6910f;
        }
        0x6910f => {
            // block 6910f
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6910f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6910f add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 69113 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 69114 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 69116 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 69118 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6911a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6911c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6911d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x69328 size=342
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00069328<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x69328;
    loop {
        match pc {
        0x69328 => {
            // block 69328
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69328 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69328 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 69329 mov rbp,rsp
            rbp = rsp;
            // 6932c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6932e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 69330 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 69332 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 69334 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 69335 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 69339 mov r12,rdx
            r12 = rdx;
            // 6933c mov r15,rsi
            r15 = rsi;
            // 6933f mov r14,rdi
            r14 = rdi;
            // 69342 lea rax,[r14+0x8]
            rax = r14.wrapping_add(0x8);
            // 69346 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 6934a mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // 69352 mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // 6935a mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 6935d lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 69364 mov rsi,r12
            rsi = r12;
            // 69367 call 0xebbd6
            { let r = { let t = m.env.stub_va(0x3a3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6936c mov rbx,rax
            rbx = rax;
            // 6936f add r15,0x1a0
            { let (r, f) = add::<64>(r15, 0x1a0u64);
              r15 = r;
              fl = f; }
            // 69376 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 69379 je 0x69454
            if fl.e() { pc = 0x69454; continue; }
            pc = 0x6937f;
        }
        0x6937f => {
            // block 6937f
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6937f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6937f lea rax,[r12+0x1]
            rax = r12.wrapping_add(0x1);
            // 69384 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            pc = 0x69388;
        }
        0x69388 => {
            // block 69388
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69388 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69388 movzx edx,BYTE PTR [rbx+0x20]
            rdx = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 6938c mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 6938e and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 69390 jne 0x69397
            if fl.ne() { pc = 0x69397; continue; }
            pc = 0x69392;
        }
        0x69392 => {
            // block 69392
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69392 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69392 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 69395 jmp 0x6939b
            pc = 0x6939b; continue;
        }
        0x69397 => {
            // block 69397
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69397 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69397 mov rdx,QWORD PTR [rbx+0x28]
            rdx = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x6939b;
        }
        0x6939b => {
            // block 6939b
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6939b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6939b movzx esi,BYTE PTR [r12]
            rsi = (m.r8(r12) as u64) as u32 as u64;
            // 693a0 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 693a3 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 693a6 jne 0x693ad
            if fl.ne() { pc = 0x693ad; continue; }
            pc = 0x693a8;
        }
        0x693a8 => {
            // block 693a8
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693a8 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 693ab jmp 0x693b2
            pc = 0x693b2; continue;
        }
        0x693ad => {
            // block 693ad
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693ad mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            pc = 0x693b2;
        }
        0x693b2 => {
            // block 693b2
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693b2 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 693b5 jne 0x69454
            if fl.ne() { pc = 0x69454; continue; }
            pc = 0x693bb;
        }
        0x693bb => {
            // block 693bb
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693bb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 693bd jne 0x693c8
            if fl.ne() { pc = 0x693c8; continue; }
            pc = 0x693bf;
        }
        0x693bf => {
            // block 693bf
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693bf lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // 693c3 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 693c6 jmp 0x693cc
            pc = 0x693cc; continue;
        }
        0x693c8 => {
            // block 693c8
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693c8 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x693cc;
        }
        0x693cc => {
            // block 693cc
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693cc test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 693ce mov rsi,QWORD PTR [r12+0x10]
            rsi = m.r64(r12.wrapping_add(0x10)) as u64;
            // 693d3 cmove rsi,QWORD PTR [rbp-0x30]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 693d8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 693da jne 0x693f8
            if fl.ne() { pc = 0x693f8; continue; }
            pc = 0x693dc;
        }
        0x693dc => {
            // block 693dc
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693dc test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 693df je 0x69406
            if fl.e() { pc = 0x69406; continue; }
            pc = 0x693e1;
        }
        0x693e1 => {
            // block 693e1
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693e1 movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // 693e4 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 693e7 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 693e9 jne 0x69454
            if fl.ne() { pc = 0x69454; continue; }
            pc = 0x693eb;
        }
        0x693eb => {
            // block 693eb
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693eb inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 693ee inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 693f1 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 693f4 jne 0x693e1
            if fl.ne() { pc = 0x693e1; continue; }
            pc = 0x693f6;
        }
        0x693f6 => {
            // block 693f6
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693f6 jmp 0x69406
            pc = 0x69406; continue;
        }
        0x693f8 => {
            // block 693f8
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693f8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 693fb je 0x69406
            if fl.e() { pc = 0x69406; continue; }
            pc = 0x693fd;
        }
        0x693fd => {
            // block 693fd
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 693fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 693fd call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69402 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 69404 jne 0x69454
            if fl.ne() { pc = 0x69454; continue; }
            pc = 0x69406;
        }
        0x69406 => {
            // block 69406
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69406 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69406 lea r13,[rbx+0x38]
            r13 = rbx.wrapping_add(0x38);
            // 6940a mov rdi,r14
            rdi = r14;
            // 6940d mov rsi,r13
            rsi = r13;
            // 69410 call 0xebccc
            { let r = { let t = m.env.stub_va(0x499); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69415 cmp rax,QWORD PTR [rbp-0x38]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64).1;
            // 69419 jne 0x69426
            if fl.ne() { pc = 0x69426; continue; }
            pc = 0x6941b;
        }
        0x6941b => {
            // block 6941b
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6941b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6941b mov rdi,r14
            rdi = r14;
            // 6941e mov rsi,r13
            rsi = r13;
            // 69421 call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x69426;
        }
        0x69426 => {
            // block 69426
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69426 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69426 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 6942a test rax,rax
            fl = logic::<64>(rax & rax);
            // 6942d je 0x6943c
            if fl.e() { pc = 0x6943c; continue; }
            pc = 0x6942f;
        }
        0x6942f => {
            // block 6942f
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6942f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6942f mov rbx,rax
            rbx = rax;
            // 69432 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 69435 test rax,rax
            fl = logic::<64>(rax & rax);
            // 69438 jne 0x6942f
            if fl.ne() { pc = 0x6942f; continue; }
            pc = 0x6943a;
        }
        0x6943a => {
            // block 6943a
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6943a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6943a jmp 0x6944b
            pc = 0x6944b; continue;
        }
        0x6943c => {
            // block 6943c
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6943c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6943c mov rax,rbx
            rax = rbx;
            pc = 0x6943f;
        }
        0x6943f => {
            // block 6943f
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6943f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6943f mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 69443 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 69446 mov rax,rbx
            rax = rbx;
            // 69449 jne 0x6943f
            if fl.ne() { pc = 0x6943f; continue; }
            pc = 0x6944b;
        }
        0x6944b => {
            // block 6944b
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 6944b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6944b cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 6944e jne 0x69388
            if fl.ne() { pc = 0x69388; continue; }
            pc = 0x69454;
        }
        0x69454 => {
            // block 69454
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69454 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69454 mov rax,r14
            rax = r14;
            // 69457 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 6945b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6945c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 6945e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 69460 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 69462 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 69464 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 69465 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x69466 => {
            // block 69466
            #[cfg(feature = "trace")] eprintln!("f_00069328 pc 69466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69466 mov rbx,rax
            rbx = rax;
            // 69469 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 6946d mov rdi,r14
            rdi = r14;
            // 69470 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69475 mov rdi,rbx
            rdi = rbx;
            // 69478 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc362 size=370
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc362<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbc362;
    loop {
        match pc {
        0xbc362 => {
            // block bc362
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc362 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc362 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc363 mov rbp,rsp
            rbp = rsp;
            // bc366 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc368 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc36a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // bc36c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc36e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc36f sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // bc373 mov rbx,rsi
            rbx = rsi;
            // bc376 mov r14,rdi
            r14 = rdi;
            // bc379 mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // bc37c mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc380 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bc383 je 0xbc39a
            if fl.e() { pc = 0xbc39a; continue; }
            pc = 0xbc385;
        }
        0xbc385 => {
            // block bc385
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc385 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc385 lea rdx,[rax-0x10]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff0);
            // bc389 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // bc38c not rdx
            { let v = !rdx; rdx = v; }
            // bc38f and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // bc393 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // bc396 mov QWORD PTR [rbx+0x8],rdx
            m.w64(rbx.wrapping_add(0x8), (rdx) as u64);
            pc = 0xbc39a;
        }
        0xbc39a => {
            // block bc39a
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc39a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc39a lea r15,[rbp-0x38]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bc39e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // bc3a6 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // bc3ae mov QWORD PTR [rbp-0x40],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r15) as u64);
            // bc3b2 mov DWORD PTR [rbp-0x44],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), (0x0u64) as u32);
            // bc3b9 mov DWORD PTR [rbp-0x48],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0x0u64) as u32);
            // bc3c0 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // bc3c3 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // bc3c7 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // bc3c9 cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // bc3cc je 0xbc4a2
            if fl.e() { pc = 0xbc4a2; continue; }
            pc = 0xbc3d2;
        }
        0xbc3d2 => {
            // block bc3d2
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc3d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc3d2 lea r12,[rbp-0x48]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            pc = 0xbc3d6;
        }
        0xbc3d6 => {
            // block bc3d6
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc3d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc3d6 mov edi,DWORD PTR [r13+0x1c]
            rdi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // bc3da lea rsi,[rbp-0x44]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // bc3de mov rdx,r12
            rdx = r12;
            // bc3e1 call 0x6858c
            { let r = { let t = m.env.code_va(0x6858c); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc3e6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // bc3e8 je 0xbc46e
            if fl.e() { pc = 0xbc46e; continue; }
            pc = 0xbc3ee;
        }
        0xbc3ee => {
            // block bc3ee
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc3ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc3ee mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // bc3f2 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // bc3f5 mov eax,DWORD PTR [rbp-0x44]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64) as u32 as u64;
            // bc3f8 mov rdx,r15
            rdx = r15;
            // bc3fb je 0xbc42a
            if fl.e() { pc = 0xbc42a; continue; }
            pc = 0xbc3fd;
        }
        0xbc3fd => {
            // block bc3fd
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc3fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc3fd mov rcx,rsi
            rcx = rsi;
            pc = 0xbc400;
        }
        0xbc400 => {
            // block bc400
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc400 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc400 cmp DWORD PTR [rcx+0x1c],eax
            fl = sub::<32>(m.r32(rcx.wrapping_add(0x1c)) as u64, (rax as u32 as u64)).1;
            // bc403 jae 0xbc410
            if fl.ae() { pc = 0xbc410; continue; }
            pc = 0xbc405;
        }
        0xbc405 => {
            // block bc405
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc405 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc405 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // bc409 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // bc40c jne 0xbc400
            if fl.ne() { pc = 0xbc400; continue; }
            pc = 0xbc40e;
        }
        0xbc40e => {
            // block bc40e
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc40e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc40e jmp 0xbc41d
            pc = 0xbc41d; continue;
        }
        0xbc410 => {
            // block bc410
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc410 mov rsi,QWORD PTR [rcx]
            rsi = m.r64(rcx) as u64;
            // bc413 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // bc416 mov rdx,rcx
            rdx = rcx;
            // bc419 jne 0xbc3fd
            if fl.ne() { pc = 0xbc3fd; continue; }
            pc = 0xbc41b;
        }
        0xbc41b => {
            // block bc41b
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc41b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc41b jmp 0xbc420
            pc = 0xbc420; continue;
        }
        0xbc41d => {
            // block bc41d
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc41d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc41d mov rcx,rdx
            rcx = rdx;
            pc = 0xbc420;
        }
        0xbc420 => {
            // block bc420
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc420 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc420 cmp rcx,r15
            fl = sub::<64>(rcx, r15).1;
            // bc423 je 0xbc42a
            if fl.e() { pc = 0xbc42a; continue; }
            pc = 0xbc425;
        }
        0xbc425 => {
            // block bc425
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc425 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc425 cmp eax,DWORD PTR [rcx+0x1c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rcx.wrapping_add(0x1c)) as u64).1;
            // bc428 jae 0xbc46e
            if fl.ae() { pc = 0xbc46e; continue; }
            pc = 0xbc42a;
        }
        0xbc42a => {
            // block bc42a
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc42a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc42a mov DWORD PTR [rbp-0x58],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((rax as u32 as u64)) as u32);
            // bc42d mov DWORD PTR [rbp-0x54],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), (0x0u64) as u32);
            // bc434 mov DWORD PTR [rbp-0x50],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0xffffffffu64) as u32);
            // bc43b mov DWORD PTR [rbp-0x4c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), (0x0u64) as u32);
            // bc442 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc446 cmp rax,QWORD PTR [rbx+0x10]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x10)) as u64).1;
            // bc44a jae 0xbc462
            if fl.ae() { pc = 0xbc462; continue; }
            pc = 0xbc44c;
        }
        0xbc44c => {
            // block bc44c
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc44c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc44c mov rcx,QWORD PTR [rbp-0x58]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // bc450 mov rdx,QWORD PTR [rbp-0x50]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // bc454 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // bc458 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // bc45b add QWORD PTR [rbx+0x8],0x10
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, 0x10u64);
              m.w64(rbx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // bc460 jmp 0xbc46e
            pc = 0xbc46e; continue;
        }
        0xbc462 => {
            // block bc462
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc462 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc462 mov rdi,rbx
            rdi = rbx;
            // bc465 lea rsi,[rbp-0x58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // bc469 call 0xebbbe
            { let r = { let t = m.env.stub_va(0x38b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbc46e;
        }
        0xbc46e => {
            // block bc46e
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc46e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc46e mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bc472 test rax,rax
            fl = logic::<64>(rax & rax);
            // bc475 je 0xbc485
            if fl.e() { pc = 0xbc485; continue; }
            pc = 0xbc477;
        }
        0xbc477 => {
            // block bc477
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc477 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc477 mov r13,rax
            r13 = rax;
            // bc47a mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // bc47e test rax,rax
            fl = logic::<64>(rax & rax);
            // bc481 jne 0xbc477
            if fl.ne() { pc = 0xbc477; continue; }
            pc = 0xbc483;
        }
        0xbc483 => {
            // block bc483
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc483 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc483 jmp 0xbc495
            pc = 0xbc495; continue;
        }
        0xbc485 => {
            // block bc485
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc485 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc485 mov rax,r13
            rax = r13;
            pc = 0xbc488;
        }
        0xbc488 => {
            // block bc488
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc488 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc488 mov r13,QWORD PTR [rax+0x10]
            r13 = m.r64(rax.wrapping_add(0x10)) as u64;
            // bc48c cmp QWORD PTR [r13+0x0],rax
            fl = sub::<64>(m.r64(r13) as u64, rax).1;
            // bc490 mov rax,r13
            rax = r13;
            // bc493 jne 0xbc488
            if fl.ne() { pc = 0xbc488; continue; }
            pc = 0xbc495;
        }
        0xbc495 => {
            // block bc495
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc495 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc495 cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // bc498 jne 0xbc3d6
            if fl.ne() { pc = 0xbc3d6; continue; }
            pc = 0xbc49e;
        }
        0xbc49e => {
            // block bc49e
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc49e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc49e mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0xbc4a2;
        }
        0xbc4a2 => {
            // block bc4a2
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc4a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc4a2 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bc4a6 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc4ab mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // bc4ad add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // bc4b1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc4b2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc4b4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // bc4b6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc4b8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc4ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc4bb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xbc4bc => {
            // block bc4bc
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc4bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc4bc mov rbx,rax
            rbx = rax;
            // bc4bf mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // bc4c3 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bc4c7 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc4cc mov rdi,rbx
            rdi = rbx;
            // bc4cf call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc75d9 size=339
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c75d9<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc75d9;
    loop {
        match pc {
        0xc75d9 => {
            // block c75d9
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c75d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c75d9 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c75da mov rbp,rsp
            rbp = rsp;
            // c75dd push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c75df push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c75e1 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c75e3 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c75e5 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c75e6 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c75e8 je 0xc7707
            if fl.e() { pc = 0xc7707; continue; }
            pc = 0xc75ee;
        }
        0xc75ee => {
            // block c75ee
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c75ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c75ee and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c75f1 lea r9,[rdi+0x2]
            r9 = rdi.wrapping_add(0x2);
            // c75f5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // c75f8 mov r10d,0xaaaaaaab
            r10 = (0xaaaaaaabu64) as u32 as u64;
            // c75fe lea r11,[rip+0x10f] # 0xc7714
            r11 = m.env.code_va(0xc7714);
            pc = 0xc7605;
        }
        0xc7605 => {
            // block c7605
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7605 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7605 cmp r8d,0x2
            fl = sub::<32>((r8 as u32 as u64), 0x2u64).1;
            // c7609 je 0xc7617
            if fl.e() { pc = 0xc7617; continue; }
            pc = 0xc760b;
        }
        0xc760b => {
            // block c760b
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c760b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c760b cmp r8d,0x3
            fl = sub::<32>((r8 as u32 as u64), 0x3u64).1;
            // c760f jne 0xc7627
            if fl.ne() { pc = 0xc7627; continue; }
            pc = 0xc7611;
        }
        0xc7611 => {
            // block c7611
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7611 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7611 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // c7613 jne 0xc761b
            if fl.ne() { pc = 0xc761b; continue; }
            pc = 0xc7615;
        }
        0xc7615 => {
            // block c7615
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7615 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7615 jmp 0xc763c
            pc = 0xc763c; continue;
        }
        0xc7617 => {
            // block c7617
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7617 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7617 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // c7619 jne 0xc763c
            if fl.ne() { pc = 0xc763c; continue; }
            pc = 0xc761b;
        }
        0xc761b => {
            // block c761b
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c761b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c761b lea r14,[rdi+r12*8+0x2]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x2);
            // c7620 lea r15,[rdi+r12*8+0x4]
            r15 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4);
            // c7625 jmp 0xc7663
            pc = 0xc7663; continue;
        }
        0xc7627 => {
            // block c7627
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7627 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7627 cmp r8d,0x4
            fl = sub::<32>((r8 as u32 as u64), 0x4u64).1;
            // c762b jne 0xc7642
            if fl.ne() { pc = 0xc7642; continue; }
            pc = 0xc762d;
        }
        0xc762d => {
            // block c762d
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c762d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c762d test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // c762f jne 0xc7650
            if fl.ne() { pc = 0xc7650; continue; }
            pc = 0xc7631;
        }
        0xc7631 => {
            // block c7631
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7631 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7631 lea r14,[rdi+r12*8+0x2]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x2);
            // c7636 lea r15,[rdi+r12*8]
            r15 = rdi.wrapping_add(r12.wrapping_mul(8));
            // c763a jmp 0xc7663
            pc = 0xc7663; continue;
        }
        0xc763c => {
            // block c763c
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c763c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c763c lea r14,[rdi+r12*8]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8));
            // c7640 jmp 0xc7655
            pc = 0xc7655; continue;
        }
        0xc7642 => {
            // block c7642
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7642 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7642 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // c7644 jne 0xc765c
            if fl.ne() { pc = 0xc765c; continue; }
            pc = 0xc7646;
        }
        0xc7646 => {
            // block c7646
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7646 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7646 lea r14,[rdi+r12*8+0x4]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4);
            // c764b mov r15,r9
            r15 = r9;
            // c764e jmp 0xc7663
            pc = 0xc7663; continue;
        }
        0xc7650 => {
            // block c7650
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7650 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7650 lea r14,[rdi+r12*8+0x4]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4);
            pc = 0xc7655;
        }
        0xc7655 => {
            // block c7655
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7655 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7655 lea r15,[rdi+r12*8+0x2]
            r15 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x2);
            // c765a jmp 0xc7663
            pc = 0xc7663; continue;
        }
        0xc765c => {
            // block c765c
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c765c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c765c lea r15,[rdi+r12*8]
            r15 = rdi.wrapping_add(r12.wrapping_mul(8));
            // c7660 mov r14,r9
            r14 = r9;
            pc = 0xc7663;
        }
        0xc7663 => {
            // block c7663
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7663 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7663 mov r13d,r12d
            r13 = ((r12 as u32 as u64)) as u32 as u64;
            // c7666 imul r13,r10
            { let (r, f) = imul::<64>(r13, r10); r13 = r; fl = f; }
            // c766a mov rax,r13
            rax = r13;
            // c766d shr rax,0x22
            { let (r, f) = shr::<64>(rax, 0x22u64, fl); rax = r; fl = f; }
            // c7671 add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c7673 lea ebx,[rax+rax*2]
            rbx = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // c7676 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // c7679 sub eax,ebx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c767b cmp eax,0x5
            fl = sub::<32>((rax as u32 as u64), 0x5u64).1;
            // c767e ja 0xc76f7
            if fl.a() { pc = 0xc76f7; continue; }
            pc = 0xc7680;
        }
        0xc7680 => {
            // block c7680
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7680 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7680 movzx r14d,WORD PTR [r14]
            r14 = (m.r16(r14) as u64) as u32 as u64;
            // c7684 movzx r15d,WORD PTR [r15]
            r15 = (m.r16(r15) as u64) as u32 as u64;
            // c7688 shr r13,0x21
            { let (r, f) = shr::<64>(r13, 0x21u64, fl); r13 = r; fl = f; }
            // c768c movsxd rax,DWORD PTR [r11+rax*4]
            rax = sx::<32>(m.r32(r11.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // c7690 add rax,r11
            { let (r, f) = add::<64>(rax, r11);
              rax = r;
              fl = f; }
            // c7693 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc7695 => {
            // block c7695
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7695 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7695 shr r14d,0x4
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x4u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c7699 and r14d,0xffc
            { let r = (r14 as u32 as u64) & 0xffcu64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // c76a0 mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c76a3 mov DWORD PTR [rsi+rax*4],r14d
            m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), ((r14 as u32 as u64)) as u32);
            // c76a7 jmp 0xc76f7
            pc = 0xc76f7; continue;
        }
        0xc76a9 => {
            // block c76a9
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76a9 shl r15d,0x6
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x6u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // c76ad and r15d,0x3ff000
            { let r = (r15 as u32 as u64) & 0x3ff000u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c76b4 jmp 0xc76f0
            pc = 0xc76f0; continue;
        }
        0xc76b6 => {
            // block c76b6
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76b6 and r14d,0xffffffc0
            { let r = (r14 as u32 as u64) & 0xffffffc0u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // c76ba shl r14d,0x10
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x10u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c76be jmp 0xc76df
            pc = 0xc76df; continue;
        }
        0xc76c0 => {
            // block c76c0
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76c0 shr r15d,0x4
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x4u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // c76c4 and r15d,0xffc
            { let r = (r15 as u32 as u64) & 0xffcu64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c76cb mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c76ce mov DWORD PTR [rsi+rax*4],r15d
            m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), ((r15 as u32 as u64)) as u32);
            // c76d2 jmp 0xc76f7
            pc = 0xc76f7; continue;
        }
        0xc76d4 => {
            // block c76d4
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76d4 shl r14d,0x6
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x6u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c76d8 and r14d,0x3ff000
            { let r = (r14 as u32 as u64) & 0x3ff000u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc76df;
        }
        0xc76df => {
            // block c76df
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76df mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c76e2 or DWORD PTR [rsi+rax*4],r14d
            { let r = m.r32(rsi.wrapping_add(rax.wrapping_mul(4))) as u64 | (r14 as u32 as u64); let f = logic::<32>(r);
              m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), (r) as u32);
              fl = f; }
            // c76e6 jmp 0xc76f7
            pc = 0xc76f7; continue;
        }
        0xc76e8 => {
            // block c76e8
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76e8 and r15d,0xffffffc0
            { let r = (r15 as u32 as u64) & 0xffffffc0u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c76ec shl r15d,0x10
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x10u64, fl); r15 = (r) as u32 as u64; fl = f; }
            pc = 0xc76f0;
        }
        0xc76f0 => {
            // block c76f0
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76f0 mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c76f3 or DWORD PTR [rsi+rax*4],r15d
            { let r = m.r32(rsi.wrapping_add(rax.wrapping_mul(4))) as u64 | (r15 as u32 as u64); let f = logic::<32>(r);
              m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), (r) as u32);
              fl = f; }
            pc = 0xc76f7;
        }
        0xc76f7 => {
            // block c76f7
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c76f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c76f7 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // c76fa add r9,0x8
            { let (r, f) = add::<64>(r9, 0x8u64);
              r9 = r;
              fl = f; }
            // c76fe cmp edx,r12d
            fl = sub::<32>((rdx as u32 as u64), (r12 as u32 as u64)).1;
            // c7701 jne 0xc7605
            if fl.ne() { pc = 0xc7605; continue; }
            pc = 0xc7707;
        }
        0xc7707 => {
            // block c7707
            #[cfg(feature = "trace")] eprintln!("f_000c75d9 pc c7707 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7707 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7708 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c770a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c770c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c770e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c7710 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7711 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8168 size=304
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8168<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc8168;
    loop {
        match pc {
        0xc8168 => {
            // block c8168
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8168 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8168 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8169 mov rbp,rsp
            rbp = rsp;
            // c816c mov r11,r8
            r11 = r8;
            // c816f mov r10d,esi
            r10 = ((rsi as u32 as u64)) as u32 as u64;
            // c8172 add edi,0xffffffe6
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xffffffe6u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c8175 cmp edi,0x3
            fl = sub::<32>((rdi as u32 as u64), 0x3u64).1;
            // c8178 jbe 0xc8190
            if fl.be() { pc = 0xc8190; continue; }
            pc = 0xc817a;
        }
        0xc817a => {
            // block c817a
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c817a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c817a imul r10d,ecx
            { let (r, f) = imul::<32>((r10 as u32 as u64), (rcx as u32 as u64)); r10 = (r) as u32 as u64; fl = f; }
            // c817e add r11,r10
            { let (r, f) = add::<64>(r11, r10);
              r11 = r;
              fl = f; }
            // c8181 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // c8183 mov rdi,r11
            rdi = r11;
            // c8186 mov rsi,r9
            rsi = r9;
            // c8189 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc818e;
        }
        0xc818e => {
            // block c818e
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c818e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c818e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c818f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc8190 => {
            // block c8190
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8190 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8190 lea rax,[rip+0xf1] # 0xc8288
            rax = m.env.code_va(0xc8288);
            // c8197 movsxd rsi,DWORD PTR [rax+rdi*4]
            rsi = sx::<32>(m.r32(rax.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // c819b add rsi,rax
            { let (r, f) = add::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // c819e jmp rsi
            pc = (rsi.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc81a0 => {
            // block c81a0
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81a0 lea eax,[rdx+0x2]
            rax = (rdx.wrapping_add(0x2)) as u32 as u64;
            // c81a3 mov ecx,0xaaaaaaab
            rcx = (0xaaaaaaabu64) as u32 as u64;
            // c81a8 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // c81ac shr rcx,0x21
            { let (r, f) = shr::<64>(rcx, 0x21u64, fl); rcx = r; fl = f; }
            // c81b0 imul ecx,r10d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r10 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c81b4 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c81b7 add r11,rcx
            { let (r, f) = add::<64>(r11, rcx);
              r11 = r;
              fl = f; }
            // c81ba mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // c81c0 mov rdi,r9
            rdi = r9;
            // c81c3 mov rsi,r11
            rsi = r11;
            // c81c6 mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // c81c9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c81ca jmp 0xc75d9
            { let r = { let t = m.env.code_va(0xc75d9); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc81cf => {
            // block c81cf
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81cf test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c81d1 je 0xc818e
            if fl.e() { pc = 0xc818e; continue; }
            pc = 0xc81d3;
        }
        0xc81d3 => {
            // block c81d3
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81d3 mov r8d,r10d
            r8 = ((r10 as u32 as u64)) as u32 as u64;
            // c81d6 and r8d,0x1
            { let r = (r8 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c81da lea rax,[r9+0x2]
            rax = r9.wrapping_add(0x2);
            // c81de mov ecx,ecx
            rcx = ((rcx as u32 as u64)) as u32 as u64;
            // c81e0 mov esi,0xaaaaaaab
            rsi = (0xaaaaaaabu64) as u32 as u64;
            // c81e5 imul rsi,rcx
            { let (r, f) = imul::<64>(rsi, rcx); rsi = r; fl = f; }
            // c81e9 shr rsi,0x21
            { let (r, f) = shr::<64>(rsi, 0x21u64, fl); rsi = r; fl = f; }
            // c81ed imul esi,r10d
            { let (r, f) = imul::<32>((rsi as u32 as u64), (r10 as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            // c81f1 add r11,rsi
            { let (r, f) = add::<64>(r11, rsi);
              r11 = r;
              fl = f; }
            // c81f4 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // c81f6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xc81f8;
        }
        0xc81f8 => {
            // block c81f8
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81f8 test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // c81fb jne 0xc8206
            if fl.ne() { pc = 0xc8206; continue; }
            pc = 0xc81fd;
        }
        0xc81fd => {
            // block c81fd
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81fd lea rsi,[rax+0x2]
            rsi = rax.wrapping_add(0x2);
            // c8201 mov rdi,rax
            rdi = rax;
            // c8204 jmp 0xc820f
            pc = 0xc820f; continue;
        }
        0xc8206 => {
            // block c8206
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8206 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8206 lea rsi,[r9+rdx*8+0x2]
            rsi = r9.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x2);
            // c820b lea rdi,[r9+rdx*8]
            rdi = r9.wrapping_add(rdx.wrapping_mul(8));
            pc = 0xc820f;
        }
        0xc820f => {
            // block c820f
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c820f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c820f test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // c8212 cmove rdi,rsi
            if fl.e() { rdi = rsi; }
            // c8216 mov si,WORD PTR [rdi]
            rsi = (rsi & !0xffff) | ((m.r16(rdi) as u64) as u16 as u64);
            // c8219 mov WORD PTR [r11+rdx*2],si
            m.w16(r11.wrapping_add(rdx.wrapping_mul(2)), ((rsi as u16 as u64)) as u16);
            // c821e inc rdx
            { let (r, f) = inc::<64>(rdx, fl); rdx = r; fl = f; }
            // c8221 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c8225 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // c8227 jne 0xc81f8
            if fl.ne() { pc = 0xc81f8; continue; }
            pc = 0xc8229;
        }
        0xc8229 => {
            // block c8229
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8229 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8229 jmp 0xc818e
            pc = 0xc818e; continue;
        }
        0xc822e => {
            // block c822e
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c822e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c822e lea eax,[rdx+0x2]
            rax = (rdx.wrapping_add(0x2)) as u32 as u64;
            // c8231 mov ecx,0xaaaaaaab
            rcx = (0xaaaaaaabu64) as u32 as u64;
            // c8236 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // c823a shr rcx,0x21
            { let (r, f) = shr::<64>(rcx, 0x21u64, fl); rcx = r; fl = f; }
            // c823e imul ecx,r10d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r10 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c8242 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c8245 add r11,rcx
            { let (r, f) = add::<64>(r11, rcx);
              r11 = r;
              fl = f; }
            // c8248 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // c824e mov rdi,r9
            rdi = r9;
            // c8251 mov rsi,r11
            rsi = r11;
            // c8254 mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // c8257 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8258 jmp 0xc77be
            { let r = { let t = m.env.code_va(0xc77be); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc825d => {
            // block c825d
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c825d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c825d mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c825f mov ecx,0xaaaaaaab
            rcx = (0xaaaaaaabu64) as u32 as u64;
            // c8264 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // c8268 shr rcx,0x21
            { let (r, f) = shr::<64>(rcx, 0x21u64, fl); rcx = r; fl = f; }
            // c826c imul ecx,r10d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r10 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c8270 add r11,rcx
            { let (r, f) = add::<64>(r11, rcx);
              r11 = r;
              fl = f; }
            // c8273 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // c8279 mov rdi,r9
            rdi = r9;
            // c827c mov rsi,r11
            rsi = r11;
            // c827f mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // c8282 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8283 jmp 0xc7978
            { let r = { let t = m.env.code_va(0xc7978); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4444 size=340
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4444<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd4444;
    loop {
        match pc {
        0xd4444 => {
            // block d4444
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d4444 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4444 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4445 mov rbp,rsp
            rbp = rsp;
            // d4448 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d444a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d444c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d444e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d444f sub rsp,0xd0
            { let (r, f) = sub::<64>(rsp, 0xd0u64);
              rsp = r;
              fl = f; }
            // d4456 mov r15,rdx
            r15 = rdx;
            // d4459 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d445c mov r12,rdi
            r12 = rdi;
            // d445f mov rdi,r15
            rdi = r15;
            // d4462 call 0x2a25e
            { let r = { let t = m.env.code_va(0x2a25e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4467 lea rbx,[r15+0x20]
            rbx = r15.wrapping_add(0x20);
            // d446b mov rdi,r12
            rdi = r12;
            // d446e mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d4471 mov rdx,rbx
            rdx = rbx;
            // d4474 call 0xd40e0
            { let r = { let t = m.env.code_va(0xd40e0); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4479 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d447b je 0xd455c
            if fl.e() { pc = 0xd455c; continue; }
            pc = 0xd4481;
        }
        0xd4481 => {
            // block d4481
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d4481 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4481 mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // d4483 cmp eax,0x11
            fl = sub::<32>((rax as u32 as u64), 0x11u64).1;
            // d4486 ja 0xd4560
            if fl.a() { pc = 0xd4560; continue; }
            pc = 0xd448c;
        }
        0xd448c => {
            // block d448c
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d448c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d448c mov ecx,0x3ffcf
            rcx = (0x3ffcfu64) as u32 as u64;
            // d4491 bt ecx,eax
            fl.cf = bit::<32>((rcx as u32 as u64), (rax as u32 as u64));
            // d4494 jae 0xd4560
            if fl.ae() { pc = 0xd4560; continue; }
            pc = 0xd449a;
        }
        0xd449a => {
            // block d449a
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d449a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d449a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d449e mov rdi,r12
            rdi = r12;
            // d44a1 mov rsi,r15
            rsi = r15;
            // d44a4 call QWORD PTR [rax+0xd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xd0)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d44aa mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d44ac mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d44ae test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d44b0 jne 0xd4588
            if fl.ne() { pc = 0xd4588; continue; }
            pc = 0xd44b6;
        }
        0xd44b6 => {
            // block d44b6
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d44b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d44b6 xorps xmm0,xmm0
            x0 = 0;
            // d44b9 movaps XMMWORD PTR [rbp-0xb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff50), x0);
            // d44c0 movaps XMMWORD PTR [rbp-0xc0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff40), x0);
            // d44c7 movaps XMMWORD PTR [rbp-0xd0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff30), x0);
            // d44ce movaps XMMWORD PTR [rbp-0xe0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff20), x0);
            // d44d5 movaps XMMWORD PTR [rbp-0xf0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff10), x0);
            // d44dc mov DWORD PTR [rbp-0x98],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0x0u64) as u32);
            // d44e6 mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // d44f1 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d44f4 call 0x5c6f2
            { let r = { let t = m.env.code_va(0x5c6f2); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d44f9 mov DWORD PTR [rbp-0xf0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), ((rax as u32 as u64)) as u32);
            // d44ff mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // d4503 lea rdx,[rbp-0xf0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // d450a mov rdi,r12
            rdi = r12;
            // d450d mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d450f call QWORD PTR [rcx+0xc18]
            { let r = { let t = m.r64(rcx.wrapping_add(0xc18)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4515 cmp DWORD PTR [rbp-0xec],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff14)) as u64, 0x0u64).1;
            // d451c jne 0xd454b
            if fl.ne() { pc = 0xd454b; continue; }
            pc = 0xd451e;
        }
        0xd451e => {
            // block d451e
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d451e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d451e mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d4521 call 0x5c70e
            { let r = { let t = m.env.code_va(0x5c70e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4526 mov DWORD PTR [rbp-0xf0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), ((rax as u32 as u64)) as u32);
            // d452c mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // d4530 lea rdx,[rbp-0xf0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // d4537 mov rdi,r12
            rdi = r12;
            // d453a mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d453c call QWORD PTR [rcx+0xc18]
            { let r = { let t = m.r64(rcx.wrapping_add(0xc18)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4542 cmp DWORD PTR [rbp-0xec],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff14)) as u64, 0x0u64).1;
            // d4549 je 0xd4560
            if fl.e() { pc = 0xd4560; continue; }
            pc = 0xd454b;
        }
        0xd454b => {
            // block d454b
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d454b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d454b lea rsi,[rbp-0xf0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // d4552 mov rdi,r15
            rdi = r15;
            // d4555 call 0x2a1d8
            { let r = { let t = m.env.code_va(0x2a1d8); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d455a jmp 0xd4588
            pc = 0xd4588; continue;
        }
        0xd455c => {
            // block d455c
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d455c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d455c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d455e jmp 0xd4588
            pc = 0xd4588; continue;
        }
        0xd4560 => {
            // block d4560
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d4560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4560 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d4563 call 0x5c70e
            { let r = { let t = m.env.code_va(0x5c70e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4568 lea rbx,[rbp-0x90]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d456f mov rdi,rbx
            rdi = rbx;
            // d4572 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d4574 call 0x2a0c4
            { let r = { let t = m.env.code_va(0x2a0c4); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4579 mov ecx,0x1c
            rcx = (0x1cu64) as u32 as u64;
            // d457e mov rdi,r15
            rdi = r15;
            // d4581 mov rsi,rbx
            rsi = rbx;
            // d4584 rep movs DWORD PTR [rdi],DWORD PTR [rsi]
            while rcx != 0 { let v = m.r32(rsi); m.w32(rdi, v); rsi = rsi.wrapping_add(4); rdi = rdi.wrapping_add(4); rcx = rcx.wrapping_sub(1); }
            // d4586 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xd4588;
        }
        0xd4588 => {
            // block d4588
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d4588 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4588 add rsp,0xd0
            { let (r, f) = add::<64>(rsp, 0xd0u64);
              rsp = r;
              fl = f; }
            // d458f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4590 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d4592 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d4594 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d4596 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4597 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6d9a size=302
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6d9a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd6d9a;
    loop {
        match pc {
        0xd6d9a => {
            // block d6d9a
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6d9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6d9a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6d9b mov rbp,rsp
            rbp = rsp;
            // d6d9e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d6da0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d6da2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d6da4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d6da6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6da7 sub rsp,0x298
            { let (r, f) = sub::<64>(rsp, 0x298u64);
              rsp = r;
              fl = f; }
            // d6dae mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // d6db1 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // d6db3 mov r13d,esi
            r13 = ((rsi as u32 as u64)) as u32 as u64;
            // d6db6 mov r15,rdi
            r15 = rdi;
            // d6db9 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // d6dbc xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d6dbf xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d6dc1 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6dc4 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d6dc6 call QWORD PTR [rbx+0x10]
            { let r = { let t = m.r64(rbx.wrapping_add(0x10)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6dc9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6dcb je 0xd6e9b
            if fl.e() { pc = 0xd6e9b; continue; }
            pc = 0xd6dd1;
        }
        0xd6dd1 => {
            // block d6dd1
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6dd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6dd1 lea r12,[rbp-0x2b0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6dd8 mov rdi,r12
            rdi = r12;
            // d6ddb call 0xb6722
            { let r = { let t = m.env.code_va(0xb6722); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6de0 movzx edi,r13w
            rdi = ((r13 as u16 as u64)) as u32 as u64;
            // d6de4 mov rsi,r12
            rsi = r12;
            // d6de7 call 0x17202
            { let r = { let t = m.env.code_va(0x17202); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6dec mov rax,QWORD PTR [rbp-0x2b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            // d6df3 mov rax,QWORD PTR [rax+0x100]
            rax = m.r64(rax.wrapping_add(0x100)) as u64;
            // d6dfa lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6e01 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6e03 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6e05 je 0xd6e68
            if fl.e() { pc = 0xd6e68; continue; }
            pc = 0xd6e07;
        }
        0xd6e07 => {
            // block d6e07
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6e07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6e07 mov DWORD PTR [rbp-0x2b4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c), (0x0u64) as u32);
            // d6e11 mov rax,QWORD PTR [rbp-0x2b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            // d6e18 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d6e1c lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6e23 lea rdx,[rbp-0x2b4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fd4c);
            // d6e2a mov esi,0x40020
            rsi = (0x40020u64) as u32 as u64;
            // d6e2f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d6e34 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6e37 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6e39 test BYTE PTR [rbp-0x2b4],0x4
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd4c)) as u64 & 0x4u64);
            // d6e40 je 0xd6e68
            if fl.e() { pc = 0xd6e68; continue; }
            pc = 0xd6e42;
        }
        0xd6e42 => {
            // block d6e42
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6e42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6e42 mov rax,QWORD PTR [rbp-0x2b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            // d6e49 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // d6e4d lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6e54 mov esi,0x40002
            rsi = (0x40002u64) as u32 as u64;
            // d6e59 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d6e5e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d6e63 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6e66 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6e68;
        }
        0xd6e68 => {
            // block d6e68
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6e68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6e68 mov rdi,r15
            rdi = r15;
            // d6e6b call 0xd69d6
            { let r = { let t = m.env.code_va(0xd69d6); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6e70 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d6e73 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // d6e77 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d6e79 mov ecx,0x30000
            rcx = (0x30000u64) as u32 as u64;
            // d6e7e mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // d6e84 mov rdi,r15
            rdi = r15;
            // d6e87 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d6e8a call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6e8c mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d6e8f lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6e96 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6e9b;
        }
        0xd6e9b => {
            // block d6e9b
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6e9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6e9b mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // d6e9e add rsp,0x298
            { let (r, f) = add::<64>(rsp, 0x298u64);
              rsp = r;
              fl = f; }
            // d6ea5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6ea6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d6ea8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d6eaa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6eac pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d6eae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6eaf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd6eb0 => {
            // block d6eb0
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6eb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6eb0 mov rbx,rax
            rbx = rax;
            // d6eb3 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6eba call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6ebf mov rdi,rbx
            rdi = rbx;
            // d6ec2 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdf04e size=342
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000df04e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdf04e;
    loop {
        match pc {
        0xdf04e => {
            // block df04e
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df04e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df04e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // df04f mov rbp,rsp
            rbp = rsp;
            // df052 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // df054 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // df056 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // df058 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // df05a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // df05b sub rsp,0x78
            { let (r, f) = sub::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // df05f mov QWORD PTR [rbp-0x88],r8
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (r8) as u64);
            // df066 mov QWORD PTR [rbp-0x90],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (rcx) as u64);
            // df06d mov r12,rdx
            r12 = rdx;
            // df070 mov r13,rsi
            r13 = rsi;
            // df073 mov rbx,rdi
            rbx = rdi;
            // df076 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df079 mov edx,DWORD PTR [rbx+0x250]
            rdx = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // df07f lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // df083 call QWORD PTR [rax+0x3f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df089 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df08c mov edx,DWORD PTR [rbx+0x250]
            rdx = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // df092 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // df096 mov rdi,rbx
            rdi = rbx;
            // df099 call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df09f mov r14d,DWORD PTR [rbp-0x30]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // df0a3 lea ecx,[r14-0x5]
            rcx = (r14.wrapping_add(0xffff_ffff_ffff_fffb)) as u32 as u64;
            // df0a7 cmp ecx,0x8
            fl = sub::<32>((rcx as u32 as u64), 0x8u64).1;
            // df0aa ja 0xdf0bb
            if fl.a() { pc = 0xdf0bb; continue; }
            pc = 0xdf0ac;
        }
        0xdf0ac => {
            // block df0ac
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df0ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df0ac mov r15d,0x121
            r15 = (0x121u64) as u32 as u64;
            // df0b2 shr r15d,cl
            { let (r, f) = shr::<32>((r15 as u32 as u64), (rcx as u8 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // df0b5 and r15d,0x1
            { let r = (r15 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // df0b9 jmp 0xdf0be
            pc = 0xdf0be; continue;
        }
        0xdf0bb => {
            // block df0bb
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df0bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df0bb xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xdf0be;
        }
        0xdf0be => {
            // block df0be
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df0be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df0be mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // df0c1 mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // df0c7 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // df0cb xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // df0cd xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df0d0 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // df0d3 call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df0d8 mov eax,DWORD PTR [rbp-0x6c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // df0db mov DWORD PTR [rbp-0x74],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c), ((rax as u32 as u64)) as u32);
            // df0de mov eax,DWORD PTR [rbp-0x68]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // df0e1 mov DWORD PTR [rbp-0x78],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff88), ((rax as u32 as u64)) as u32);
            // df0e4 mov eax,DWORD PTR [rbp-0x70]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // df0e7 mov DWORD PTR [rbp-0x7c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff84), ((rax as u32 as u64)) as u32);
            // df0ea mov eax,DWORD PTR [rbp-0x64]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // df0ed mov DWORD PTR [rbp-0x94],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c), ((rax as u32 as u64)) as u32);
            // df0f3 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // df0f6 je 0xdf10c
            if fl.e() { pc = 0xdf10c; continue; }
            pc = 0xdf0f8;
        }
        0xdf0f8 => {
            // block df0f8
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df0f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df0f8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df0fb lea rsi,[rbp-0x74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // df0ff lea rdx,[rbp-0x78]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // df103 mov rdi,rbx
            rdi = rbx;
            // df106 call QWORD PTR [rax+0x1a00]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a00)) as u64; m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdf10c;
        }
        0xdf10c => {
            // block df10c
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df10c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df10c cmp r14d,0x11
            fl = sub::<32>((r14 as u32 as u64), 0x11u64).1;
            // df110 jne 0xdf12c
            if fl.ne() { pc = 0xdf12c; continue; }
            pc = 0xdf112;
        }
        0xdf112 => {
            // block df112
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df112 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df112 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df115 lea rsi,[rbp-0x74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // df119 lea rdx,[rbp-0x78]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // df11d lea rcx,[rbp-0x7c]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff84);
            // df121 mov rdi,rbx
            rdi = rbx;
            // df124 call QWORD PTR [rax+0x1a08]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a08)) as u64; m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df12a jmp 0xdf14a
            pc = 0xdf14a; continue;
        }
        0xdf12c => {
            // block df12c
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df12c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df12c cmp r14d,0x10
            fl = sub::<32>((r14 as u32 as u64), 0x10u64).1;
            // df130 jne 0xdf14a
            if fl.ne() { pc = 0xdf14a; continue; }
            pc = 0xdf132;
        }
        0xdf132 => {
            // block df132
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df132 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df132 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df135 lea rsi,[rbp-0x74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // df139 lea rdx,[rbp-0x78]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // df13d lea rcx,[rbp-0x7c]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff84);
            // df141 mov rdi,rbx
            rdi = rbx;
            // df144 call QWORD PTR [rax+0x1a10]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a10)) as u64; m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdf14a;
        }
        0xdf14a => {
            // block df14a
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df14a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df14a test r13,r13
            fl = logic::<64>(r13 & r13);
            // df14d mov rcx,QWORD PTR [rbp-0x88]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // df154 mov rdx,QWORD PTR [rbp-0x90]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // df15b je 0xdf164
            if fl.e() { pc = 0xdf164; continue; }
            pc = 0xdf15d;
        }
        0xdf15d => {
            // block df15d
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df15d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df15d mov eax,DWORD PTR [rbp-0x74]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c)) as u64) as u32 as u64;
            // df160 mov DWORD PTR [r13+0x0],eax
            m.w32(r13, ((rax as u32 as u64)) as u32);
            pc = 0xdf164;
        }
        0xdf164 => {
            // block df164
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df164 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df164 test r12,r12
            fl = logic::<64>(r12 & r12);
            // df167 je 0xdf170
            if fl.e() { pc = 0xdf170; continue; }
            pc = 0xdf169;
        }
        0xdf169 => {
            // block df169
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df169 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df169 mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            // df16c mov DWORD PTR [r12],eax
            m.w32(r12, ((rax as u32 as u64)) as u32);
            pc = 0xdf170;
        }
        0xdf170 => {
            // block df170
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df170 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df170 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // df173 je 0xdf17d
            if fl.e() { pc = 0xdf17d; continue; }
            pc = 0xdf175;
        }
        0xdf175 => {
            // block df175
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df175 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df175 mov eax,DWORD PTR [rbp-0x78]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // df178 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // df17b mov DWORD PTR [rdx],eax
            m.w32(rdx, ((rax as u32 as u64)) as u32);
            pc = 0xdf17d;
        }
        0xdf17d => {
            // block df17d
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df17d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df17d test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // df180 je 0xdf18a
            if fl.e() { pc = 0xdf18a; continue; }
            pc = 0xdf182;
        }
        0xdf182 => {
            // block df182
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df182 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df182 mov eax,DWORD PTR [rbp-0x94]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c)) as u64) as u32 as u64;
            // df188 mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            pc = 0xdf18a;
        }
        0xdf18a => {
            // block df18a
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df18a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df18a mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            // df18d imul eax,DWORD PTR [rbp-0x78]
            { let (r, f) = imul::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64); rax = (r) as u32 as u64; fl = f; }
            // df191 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // df194 add rsp,0x78
            { let (r, f) = add::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // df198 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // df199 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // df19b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // df19d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // df19f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // df1a1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // df1a2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe5874 size=358
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e5874<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe5874;
    loop {
        match pc {
        0xe5874 => {
            // block e5874
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5874 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5874 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e5875 mov rbp,rsp
            rbp = rsp;
            // e5878 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e587a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e587c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e587e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e5880 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e5881 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e5883 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e5885 jne 0xe588a
            if fl.ne() { pc = 0xe588a; continue; }
            pc = 0xe5887;
        }
        0xe5887 => {
            // block e5887
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5887 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5887 mov eax,DWORD PTR [rdi+0x8]
            rax = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            pc = 0xe588a;
        }
        0xe588a => {
            // block e588a
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e588a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e588a dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e588c mov r11d,0x1e
            r11 = (0x1eu64) as u32 as u64;
            // e5892 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // e5895 ja 0xe58a4
            if fl.a() { pc = 0xe58a4; continue; }
            pc = 0xe5897;
        }
        0xe5897 => {
            // block e5897
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5897 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5897 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e5899 lea rbx,[rip+0x44440] # 0x129ce0
            rbx = m.env.data_va(255149);
            // e58a0 mov r11d,DWORD PTR [rbx+rax*4]
            r11 = (m.r32(rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe58a4;
        }
        0xe58a4 => {
            // block e58a4
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e58a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e58a4 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e58a6 jne 0xe58ab
            if fl.ne() { pc = 0xe58ab; continue; }
            pc = 0xe58a8;
        }
        0xe58a8 => {
            // block e58a8
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e58a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e58a8 mov edx,DWORD PTR [rdi+0x8]
            rdx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            pc = 0xe58ab;
        }
        0xe58ab => {
            // block e58ab
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e58ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e58ab mov r15,QWORD PTR [rbp+0x10]
            r15 = m.r64(rbp.wrapping_add(0x10)) as u64;
            // e58af imul r14d,r11d,0x3c
            { let (r, f) = imul::<32>((r11 as u32 as u64), 0x3cu64); r14 = (r) as u32 as u64; fl = f; }
            // e58b3 cmp edx,0x4
            fl = sub::<32>((rdx as u32 as u64), 0x4u64).1;
            // e58b6 je 0xe58c1
            if fl.e() { pc = 0xe58c1; continue; }
            pc = 0xe58b8;
        }
        0xe58b8 => {
            // block e58b8
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e58b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e58b8 cmp edx,0x8
            fl = sub::<32>((rdx as u32 as u64), 0x8u64).1;
            // e58bb jne 0xe5997
            if fl.ne() { pc = 0xe5997; continue; }
            pc = 0xe58c1;
        }
        0xe58c1 => {
            // block e58c1
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e58c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e58c1 cmp DWORD PTR [rdi+0x8],0x8
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x8)) as u64, 0x8u64).1;
            // e58c5 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // e58c8 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // e58cb lea eax,[rax+rax*1+0x2]
            rax = (rax.wrapping_add(rax).wrapping_add(0x2)) as u32 as u64;
            // e58cf mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // e58d2 mov r12d,r11d
            r12 = ((r11 as u32 as u64)) as u32 as u64;
            // e58d5 sub r12d,eax
            { let (r, f) = sub::<32>((r12 as u32 as u64), (rax as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // e58d8 imul edi,r11d,0x3b
            { let (r, f) = imul::<32>((r11 as u32 as u64), 0x3bu64); rdi = (r) as u32 as u64; fl = f; }
            // e58dc add edi,r12d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r12 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // e58df lea ebx,[rdi+rdi*8]
            rbx = (rdi.wrapping_add(rdi.wrapping_mul(8))) as u32 as u64;
            // e58e2 add ebx,r14d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r14 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // e58e5 lea eax,[rbx+rbx*1]
            rax = (rbx.wrapping_add(rbx)) as u32 as u64;
            // e58e8 lea r13d,[rax+rax*2]
            r13 = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // e58ec mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e58ee shl eax,0x4
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e58f1 lea r10d,[rax+rax*8]
            r10 = (rax.wrapping_add(rax.wrapping_mul(8))) as u32 as u64;
            // e58f5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e58f7 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e58f9 div r10d
            { let (q, r) = div::<32>(rdx, rax, (r10 as u32 as u64)); rax = q; rdx = r; }
            // e58fc mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e58fe xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5900 div r13d
            { let (q, r) = div::<32>(rdx, rax, (r13 as u32 as u64)); rax = q; rdx = r; }
            // e5903 mov esi,edx
            rsi = ((rdx as u32 as u64)) as u32 as u64;
            // e5905 mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            // e5907 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5909 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e590b div ebx
            { let (q, r) = div::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // e590d mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // e590f add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e5911 lea esi,[rax+rax*4]
            rsi = (rax.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // e5914 mov DWORD PTR [r8],esi
            m.w32(r8, ((rsi as u32 as u64)) as u32);
            // e5917 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e5919 sub eax,r14d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r14 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e591c jb 0xe592c
            if fl.b() { pc = 0xe592c; continue; }
            pc = 0xe591e;
        }
        0xe591e => {
            // block e591e
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e591e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e591e or esi,0x1
            { let r = (rsi as u32 as u64) | 0x1u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e5921 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5923 div edi
            { let (q, r) = div::<32>(rdx, rax, (rdi as u32 as u64)); rax = q; rdx = r; }
            // e5925 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // e5927 add eax,esi
            { let (r, f) = add::<32>((rax as u32 as u64), (rsi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e5929 mov DWORD PTR [r8],eax
            m.w32(r8, ((rax as u32 as u64)) as u32);
            pc = 0xe592c;
        }
        0xe592c => {
            // block e592c
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e592c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e592c mov DWORD PTR [r9],0x0
            m.w32(r9, (0x0u64) as u32);
            // e5933 mov eax,DWORD PTR [r8]
            rax = (m.r32(r8) as u64) as u32 as u64;
            // e5936 mov edx,0xcccccccd
            rdx = (0xcccccccdu64) as u32 as u64;
            // e593b imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // e593f shr rdx,0x23
            { let (r, f) = shr::<64>(rdx, 0x23u64, fl); rdx = r; fl = f; }
            // e5943 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5945 lea edx,[rdx+rdx*4]
            rdx = (rdx.wrapping_add(rdx.wrapping_mul(4))) as u32 as u64;
            // e5948 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // e594a je 0xe5961
            if fl.e() { pc = 0xe5961; continue; }
            pc = 0xe594c;
        }
        0xe594c => {
            // block e594c
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e594c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e594c mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e594e sub eax,r12d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r12 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e5951 jb 0xe596d
            if fl.b() { pc = 0xe596d; continue; }
            pc = 0xe5953;
        }
        0xe5953 => {
            // block e5953
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5953 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5953 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5955 div r11d
            { let (q, r) = div::<32>(rdx, rax, (r11 as u32 as u64)); rax = q; rdx = r; }
            // e5958 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // e595a inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e595c mov DWORD PTR [r9],eax
            m.w32(r9, ((rax as u32 as u64)) as u32);
            // e595f jmp 0xe596d
            pc = 0xe596d; continue;
        }
        0xe5961 => {
            // block e5961
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5961 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5961 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5963 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e5965 div r11d
            { let (q, r) = div::<32>(rdx, rax, (r11 as u32 as u64)); rax = q; rdx = r; }
            // e5968 mov DWORD PTR [r9],eax
            m.w32(r9, ((rax as u32 as u64)) as u32);
            // e596b mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0xe596d;
        }
        0xe596d => {
            // block e596d
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e596d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e596d mov DWORD PTR [r15],ecx
            m.w32(r15, ((rcx as u32 as u64)) as u32);
            // e5970 cmp DWORD PTR [r9],0x0
            fl = sub::<32>(m.r32(r9) as u64, 0x0u64).1;
            // e5974 jne 0xe59ce
            if fl.ne() { pc = 0xe59ce; continue; }
            pc = 0xe5976;
        }
        0xe5976 => {
            // block e5976
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5976 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5976 mov eax,DWORD PTR [r8]
            rax = (m.r32(r8) as u64) as u32 as u64;
            // e5979 mov edx,0xcccccccd
            rdx = (0xcccccccdu64) as u32 as u64;
            // e597e imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // e5982 shr rdx,0x23
            { let (r, f) = shr::<64>(rdx, 0x23u64, fl); rdx = r; fl = f; }
            // e5986 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5988 lea edx,[rdx+rdx*4]
            rdx = (rdx.wrapping_add(rdx.wrapping_mul(4))) as u32 as u64;
            // e598b cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // e598d je 0xe59ce
            if fl.e() { pc = 0xe59ce; continue; }
            pc = 0xe598f;
        }
        0xe598f => {
            // block e598f
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e598f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e598f add ecx,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e5992 mov DWORD PTR [r15],ecx
            m.w32(r15, ((rcx as u32 as u64)) as u32);
            // e5995 jmp 0xe59ce
            pc = 0xe59ce; continue;
        }
        0xe5997 => {
            // block e5997
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e5997 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5997 imul edi,r11d,0xe10
            { let (r, f) = imul::<32>((r11 as u32 as u64), 0xe10u64); rdi = (r) as u32 as u64; fl = f; }
            // e599e imul ebx,r11d,0x15180
            { let (r, f) = imul::<32>((r11 as u32 as u64), 0x15180u64); rbx = (r) as u32 as u64; fl = f; }
            // e59a5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e59a7 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e59a9 div ebx
            { let (q, r) = div::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // e59ab mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e59ad xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e59af div edi
            { let (q, r) = div::<32>(rdx, rax, (rdi as u32 as u64)); rax = q; rdx = r; }
            // e59b1 mov esi,edx
            rsi = ((rdx as u32 as u64)) as u32 as u64;
            // e59b3 mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            // e59b5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e59b7 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e59b9 div r14d
            { let (q, r) = div::<32>(rdx, rax, (r14 as u32 as u64)); rax = q; rdx = r; }
            // e59bc mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // e59be mov DWORD PTR [r8],eax
            m.w32(r8, ((rax as u32 as u64)) as u32);
            // e59c1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e59c3 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e59c5 div r11d
            { let (q, r) = div::<32>(rdx, rax, (r11 as u32 as u64)); rax = q; rdx = r; }
            // e59c8 mov DWORD PTR [r9],eax
            m.w32(r9, ((rax as u32 as u64)) as u32);
            // e59cb mov DWORD PTR [r15],edx
            m.w32(r15, ((rdx as u32 as u64)) as u32);
            pc = 0xe59ce;
        }
        0xe59ce => {
            // block e59ce
            #[cfg(feature = "trace")] eprintln!("f_000e5874 pc e59ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e59ce pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e59cf pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e59d1 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e59d3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e59d5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e59d7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e59d8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe86f8 size=364
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e86f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe86f8;
    loop {
        match pc {
        0xe86f8 => {
            // block e86f8
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e86f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e86f9 mov rbp,rsp
            rbp = rsp;
            // e86fc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e86fe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e8700 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e8702 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e8704 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e8705 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e8706 mov r15,rsi
            r15 = rsi;
            // e8709 mov r14,rdi
            r14 = rdi;
            // e870c lea rsi,[r15+0x2a8]
            rsi = r15.wrapping_add(0x2a8);
            // e8713 movzx r13d,BYTE PTR [r15+0x2a8]
            r13 = (m.r8(r15.wrapping_add(0x2a8)) as u64) as u32 as u64;
            // e871b mov r11b,r13b
            r11 = (r11 & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // e871e and r11b,0x1
            { let r = (r11 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r11 = (r11 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e8722 jne 0xe8730
            if fl.ne() { pc = 0xe8730; continue; }
            pc = 0xe8724;
        }
        0xe8724 => {
            // block e8724
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8724 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8724 lea r8,[rsi+0x1]
            r8 = rsi.wrapping_add(0x1);
            // e8728 mov rdi,r13
            rdi = r13;
            // e872b shr rdi,1
            { let (r, f) = shr::<64>(rdi, 0x1u64, fl); rdi = r; fl = f; }
            // e872e jmp 0xe873e
            pc = 0xe873e; continue;
        }
        0xe8730 => {
            // block e8730
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8730 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8730 mov rdi,QWORD PTR [r15+0x2b0]
            rdi = m.r64(r15.wrapping_add(0x2b0)) as u64;
            // e8737 mov r8,QWORD PTR [r15+0x2b8]
            r8 = m.r64(r15.wrapping_add(0x2b8)) as u64;
            pc = 0xe873e;
        }
        0xe873e => {
            // block e873e
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e873e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e873e mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e8743 cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e8747 jb 0xe87bc
            if fl.b() { pc = 0xe87bc; continue; }
            pc = 0xe8749;
        }
        0xe8749 => {
            // block e8749
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8749 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8749 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e874e cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e8752 jl 0xe87bc
            if fl.l() { pc = 0xe87bc; continue; }
            pc = 0xe8754;
        }
        0xe8754 => {
            // block e8754
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8754 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8754 mov rax,rdi
            rax = rdi;
            // e8757 add rax,0xfffffffffffffffd
            { let (r, f) = add::<64>(rax, 0xffff_ffff_ffff_fffdu64);
              rax = r;
              fl = f; }
            // e875b mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e8760 je 0xe87bc
            if fl.e() { pc = 0xe87bc; continue; }
            pc = 0xe8762;
        }
        0xe8762 => {
            // block e8762
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8762 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8762 lea r9,[r8+rdi*1]
            r9 = r8.wrapping_add(rdi);
            // e8766 lea r10,[r8+rdi*1-0x3]
            r10 = r8.wrapping_add(rdi).wrapping_add(0xffff_ffff_ffff_fffd);
            // e876b lea r12,[rip+0x54ad7] # 0x13d249
            r12 = m.env.data_va(334358);
            // e8772 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e8777 mov rdi,r8
            rdi = r8;
            pc = 0xe877a;
        }
        0xe877a => {
            // block e877a
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e877a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e877a movzx eax,BYTE PTR [rdi]
            rax = (m.r8(rdi) as u64) as u32 as u64;
            // e877d cmp eax,0x44
            fl = sub::<32>((rax as u32 as u64), 0x44u64).1;
            // e8780 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // e8785 jne 0xe879d
            if fl.ne() { pc = 0xe879d; continue; }
            pc = 0xe8787;
        }
        0xe8787 => {
            // block e8787
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8787 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8787 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // e878b je 0xe87a7
            if fl.e() { pc = 0xe87a7; continue; }
            pc = 0xe878d;
        }
        0xe878d => {
            // block e878d
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e878d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e878d movzx ebx,BYTE PTR [rax+r12*1]
            rbx = (m.r8(rax.wrapping_add(r12)) as u64) as u32 as u64;
            // e8792 movzx ecx,BYTE PTR [rdi+rax*1]
            rcx = (m.r8(rdi.wrapping_add(rax)) as u64) as u32 as u64;
            // e8796 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // e8799 cmp ecx,ebx
            fl = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64)).1;
            // e879b je 0xe8787
            if fl.e() { pc = 0xe8787; continue; }
            pc = 0xe879d;
        }
        0xe879d => {
            // block e879d
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e879d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e879d inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e87a0 cmp rdi,r10
            fl = sub::<64>(rdi, r10).1;
            // e87a3 jne 0xe877a
            if fl.ne() { pc = 0xe877a; continue; }
            pc = 0xe87a5;
        }
        0xe87a5 => {
            // block e87a5
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87a5 jmp 0xe87bc
            pc = 0xe87bc; continue;
        }
        0xe87a7 => {
            // block e87a7
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87a7 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e87ac cmp rdi,r9
            fl = sub::<64>(rdi, r9).1;
            // e87af je 0xe87bc
            if fl.e() { pc = 0xe87bc; continue; }
            pc = 0xe87b1;
        }
        0xe87b1 => {
            // block e87b1
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87b1 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // e87b6 sub rdx,r8
            { let (r, f) = sub::<64>(rdx, r8);
              rdx = r;
              fl = f; }
            // e87b9 add rdx,rdi
            { let (r, f) = add::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            pc = 0xe87bc;
        }
        0xe87bc => {
            // block e87bc
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87bc test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // e87bf jne 0xe87ca
            if fl.ne() { pc = 0xe87ca; continue; }
            pc = 0xe87c1;
        }
        0xe87c1 => {
            // block e87c1
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87c1 lea r10,[rsi+0x1]
            r10 = rsi.wrapping_add(0x1);
            // e87c5 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // e87c8 jmp 0xe87d8
            pc = 0xe87d8; continue;
        }
        0xe87ca => {
            // block e87ca
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87ca mov r13,QWORD PTR [r15+0x2b0]
            r13 = m.r64(r15.wrapping_add(0x2b0)) as u64;
            // e87d1 mov r10,QWORD PTR [r15+0x2b8]
            r10 = m.r64(r15.wrapping_add(0x2b8)) as u64;
            pc = 0xe87d8;
        }
        0xe87d8 => {
            // block e87d8
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87d8 mov r11,0xffffffffffffffff
            r11 = 0xffff_ffff_ffff_ffffu64;
            // e87df cmp r13,0x4
            fl = sub::<64>(r13, 0x4u64).1;
            // e87e3 jb 0xe883f
            if fl.b() { pc = 0xe883f; continue; }
            pc = 0xe87e5;
        }
        0xe87e5 => {
            // block e87e5
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87e5 cmp r13,0x4
            fl = sub::<64>(r13, 0x4u64).1;
            // e87e9 jl 0xe883f
            if fl.l() { pc = 0xe883f; continue; }
            pc = 0xe87eb;
        }
        0xe87eb => {
            // block e87eb
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87eb mov rcx,r13
            rcx = r13;
            // e87ee add rcx,0xfffffffffffffffd
            { let (r, f) = add::<64>(rcx, 0xffff_ffff_ffff_fffdu64);
              rcx = r;
              fl = f; }
            // e87f2 je 0xe883f
            if fl.e() { pc = 0xe883f; continue; }
            pc = 0xe87f4;
        }
        0xe87f4 => {
            // block e87f4
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87f4 lea r8,[r10+r13*1]
            r8 = r10.wrapping_add(r13);
            // e87f8 lea r9,[r10+r13*1-0x3]
            r9 = r10.wrapping_add(r13).wrapping_add(0xffff_ffff_ffff_fffd);
            // e87fd lea r15,[rip+0x54a45] # 0x13d249
            r15 = m.env.data_va(334358);
            // e8804 mov rcx,r10
            rcx = r10;
            pc = 0xe8807;
        }
        0xe8807 => {
            // block e8807
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8807 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8807 movzx edi,BYTE PTR [rcx]
            rdi = (m.r8(rcx) as u64) as u32 as u64;
            // e880a cmp edi,0x44
            fl = sub::<32>((rdi as u32 as u64), 0x44u64).1;
            // e880d mov edi,0x1
            rdi = (0x1u64) as u32 as u64;
            // e8812 jne 0xe882a
            if fl.ne() { pc = 0xe882a; continue; }
            pc = 0xe8814;
        }
        0xe8814 => {
            // block e8814
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8814 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8814 cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e8818 je 0xe8834
            if fl.e() { pc = 0xe8834; continue; }
            pc = 0xe881a;
        }
        0xe881a => {
            // block e881a
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e881a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e881a movzx eax,BYTE PTR [rdi+r15*1]
            rax = (m.r8(rdi.wrapping_add(r15)) as u64) as u32 as u64;
            // e881f movzx ebx,BYTE PTR [rcx+rdi*1]
            rbx = (m.r8(rcx.wrapping_add(rdi)) as u64) as u32 as u64;
            // e8823 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e8826 cmp ebx,eax
            fl = sub::<32>((rbx as u32 as u64), (rax as u32 as u64)).1;
            // e8828 je 0xe8814
            if fl.e() { pc = 0xe8814; continue; }
            pc = 0xe882a;
        }
        0xe882a => {
            // block e882a
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e882a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e882a inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // e882d cmp rcx,r9
            fl = sub::<64>(rcx, r9).1;
            // e8830 jne 0xe8807
            if fl.ne() { pc = 0xe8807; continue; }
            pc = 0xe8832;
        }
        0xe8832 => {
            // block e8832
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8832 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8832 jmp 0xe883f
            pc = 0xe883f; continue;
        }
        0xe8834 => {
            // block e8834
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8834 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8834 cmp rcx,r8
            fl = sub::<64>(rcx, r8).1;
            // e8837 je 0xe883f
            if fl.e() { pc = 0xe883f; continue; }
            pc = 0xe8839;
        }
        0xe8839 => {
            // block e8839
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8839 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8839 sub rcx,r10
            { let (r, f) = sub::<64>(rcx, r10);
              rcx = r;
              fl = f; }
            // e883c mov r11,rcx
            r11 = rcx;
            pc = 0xe883f;
        }
        0xe883f => {
            // block e883f
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e883f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e883f mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // e8844 sub rcx,r11
            { let (r, f) = sub::<64>(rcx, r11);
              rcx = r;
              fl = f; }
            // e8847 mov rdi,r14
            rdi = r14;
            // e884a mov r8,rsi
            r8 = rsi;
            // e884d call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8852 mov rax,r14
            rax = r14;
            // e8855 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e8859 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e885a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e885c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e885e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e8860 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e8862 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e8863 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9608 size=312
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9608<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe9608;
    loop {
        match pc {
        0xe9608 => {
            // block e9608
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9608 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9608 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9609 mov rbp,rsp
            rbp = rsp;
            // e960c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e960e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9610 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9611 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e9612 mov r14,rsi
            r14 = rsi;
            // e9615 mov rbx,rdi
            rbx = rdi;
            // e9618 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // e961f movzx eax,BYTE PTR [rdi]
            rax = (m.r8(rdi) as u64) as u32 as u64;
            // e9622 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // e9624 jne 0xe962b
            if fl.ne() { pc = 0xe962b; continue; }
            pc = 0xe9626;
        }
        0xe9626 => {
            // block e9626
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9626 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9626 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // e9629 jmp 0xe962f
            pc = 0xe962f; continue;
        }
        0xe962b => {
            // block e962b
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e962b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e962b mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            pc = 0xe962f;
        }
        0xe962f => {
            // block e962f
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e962f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e962f test rax,rax
            fl = logic::<64>(rax & rax);
            // e9632 je 0xe9662
            if fl.e() { pc = 0xe9662; continue; }
            pc = 0xe9634;
        }
        0xe9634 => {
            // block e9634
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9634 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9634 lea rsi,[rip+0x407ed] # 0x129e28
            rsi = m.env.data_va(255477);
            // e963b call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9640 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e9642 je 0xe9662
            if fl.e() { pc = 0xe9662; continue; }
            pc = 0xe9644;
        }
        0xe9644 => {
            // block e9644
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9644 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9644 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // e964b mov WORD PTR [rbp-0x1e],0x0
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffe2), (0x0u64) as u16);
            // e9651 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // e9658 test BYTE PTR [rdi],0x1
            fl = logic::<8>(m.r8(rdi) as u64 & 0x1u64);
            // e965b jne 0xe966e
            if fl.ne() { pc = 0xe966e; continue; }
            pc = 0xe965d;
        }
        0xe965d => {
            // block e965d
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e965d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e965d inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e9660 jmp 0xe9672
            pc = 0xe9672; continue;
        }
        0xe9662 => {
            // block e9662
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9662 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9662 mov DWORD PTR [r14+0x8],0x0
            m.w32(r14.wrapping_add(0x8), (0x0u64) as u32);
            // e966a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e966c jmp 0xe96ca
            pc = 0xe96ca; continue;
        }
        0xe966e => {
            // block e966e
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e966e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e966e mov rdi,QWORD PTR [rdi+0x10]
            rdi = m.r64(rdi.wrapping_add(0x10)) as u64;
            pc = 0xe9672;
        }
        0xe9672 => {
            // block e9672
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9672 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9672 lea r15,[r14+0x2]
            r15 = r14.wrapping_add(0x2);
            // e9676 lea rsi,[rip+0x53caa] # 0x13d327
            rsi = m.env.data_va(334580);
            // e967d lea rdx,[rbp-0x1e]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe2);
            // e9681 lea r8,[rbp-0x1c]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e9685 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9687 mov rcx,r15
            rcx = r15;
            // e968a call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e968f mov al,BYTE PTR [rbp-0x1e]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe2)) as u64) as u8 as u64);
            // e9692 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // e9695 mov DWORD PTR [r14+0x4],0x0
            m.w32(r14.wrapping_add(0x4), (0x0u64) as u32);
            // e969d mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // e96a0 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // e96a4 ja 0xe96c0
            if fl.a() { pc = 0xe96c0; continue; }
            pc = 0xe96a6;
        }
        0xe96a6 => {
            // block e96a6
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96a6 lea rcx,[rip+0x7f] # 0xe972c
            rcx = m.env.code_va(0xe972c);
            // e96ad movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // e96b1 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // e96b4 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xe96b6 => {
            // block e96b6
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96b6 mov DWORD PTR [r14+0x8],0x1
            m.w32(r14.wrapping_add(0x8), (0x1u64) as u32);
            // e96be jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
        0xe96c0 => {
            // block e96c0
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96c0 mov DWORD PTR [r14+0x8],0x0
            m.w32(r14.wrapping_add(0x8), (0x0u64) as u32);
            pc = 0xe96c8;
        }
        0xe96c8 => {
            // block e96c8
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96c8 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe96ca;
        }
        0xe96ca => {
            // block e96ca
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96ca add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e96ce pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e96cf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e96d1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e96d3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e96d4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe96d5 => {
            // block e96d5
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96d5 mov DWORD PTR [r14+0x8],0x2
            m.w32(r14.wrapping_add(0x8), (0x2u64) as u32);
            // e96dd jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
        0xe96df => {
            // block e96df
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96df mov DWORD PTR [r14+0x8],0x3
            m.w32(r14.wrapping_add(0x8), (0x3u64) as u32);
            // e96e7 jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
        0xe96e9 => {
            // block e96e9
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96e9 mov DWORD PTR [r14+0x8],0x4
            m.w32(r14.wrapping_add(0x8), (0x4u64) as u32);
            // e96f1 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // e96f8 test BYTE PTR [rdi],0x1
            fl = logic::<8>(m.r8(rdi) as u64 & 0x1u64);
            // e96fb jne 0xe9702
            if fl.ne() { pc = 0xe9702; continue; }
            pc = 0xe96fd;
        }
        0xe96fd => {
            // block e96fd
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96fd inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e9700 jmp 0xe9706
            pc = 0xe9706; continue;
        }
        0xe9702 => {
            // block e9702
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9702 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9702 mov rdi,QWORD PTR [rdi+0x10]
            rdi = m.r64(rdi.wrapping_add(0x10)) as u64;
            pc = 0xe9706;
        }
        0xe9706 => {
            // block e9706
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9706 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9706 lea rsi,[rip+0x53c2a] # 0x13d337
            rsi = m.env.data_va(334596);
            // e970d lea rdx,[rbp-0x1e]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe2);
            // e9711 lea r8,[rbp-0x1c]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e9715 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9717 mov r9,r15
            r9 = r15;
            // e971a mov rcx,r15
            rcx = r15;
            // e971d call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9722 mov al,BYTE PTR [rbp-0x1e]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe2)) as u64) as u8 as u64);
            // e9725 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // e9728 jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
