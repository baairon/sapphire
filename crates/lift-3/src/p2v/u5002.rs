//! Mechanically lifted functions (scripts/lift_run.py), module u5002. Each one is checked against the original
//! in the world harness (tests/world_u5002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xd3dd4 size=334
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3dd4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut x2: u128 = fx[2] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd3dd4;
    loop {
        match pc {
        0xd3dd4 => {
            // block d3dd4
            #[cfg(feature = "trace")] eprintln!("f_000d3dd4 pc d3dd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3dd4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3dd5 mov rbp,rsp
            rbp = rsp;
            // d3dd8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d3dda push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d3ddc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d3dde push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d3de0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d3de1 sub rsp,0x78
            { let (r, f) = sub::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // d3de5 mov r13d,r9d
            r13 = ((r9 as u32 as u64)) as u32 as u64;
            // d3de8 mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // d3deb mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // d3dee mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d3df0 mov r14,rdi
            r14 = rdi;
            // d3df3 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // d3dfa mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // d3e01 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // d3e04 ja 0xd3e32
            if fl.a() { pc = 0xd3e32; continue; }
            pc = 0xd3e06;
        }
        0xd3e06 => {
            // block d3e06
            #[cfg(feature = "trace")] eprintln!("f_000d3dd4 pc d3e06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3e06 test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // d3e09 je 0xd3e32
            if fl.e() { pc = 0xd3e32; continue; }
            pc = 0xd3e0b;
        }
        0xd3e0b => {
            // block d3e0b
            #[cfg(feature = "trace")] eprintln!("f_000d3dd4 pc d3e0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3e0b movzx eax,BYTE PTR [rbp+0x10]
            rax = (m.r8(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // d3e0f movzx ecx,BYTE PTR [rbp+0x18]
            rcx = (m.r8(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // d3e13 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // d3e15 jne 0xd3e39
            if fl.ne() { pc = 0xd3e39; continue; }
            pc = 0xd3e17;
        }
        0xd3e17 => {
            // block d3e17
            #[cfg(feature = "trace")] eprintln!("f_000d3dd4 pc d3e17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3e17 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d3e1a movzx esi,dl
            rsi = ((rdx as u8 as u64)) as u32 as u64;
            // d3e1d lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // d3e21 lea rcx,[rbp-0x30]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d3e25 mov rdi,r14
            rdi = r14;
            // d3e28 call QWORD PTR [rax+0xc80]
            { let r = { let t = m.r64(rax.wrapping_add(0xc80)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // d3e2e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d3e30 jne 0xd3e3f
            if fl.ne() { pc = 0xd3e3f; continue; }
            pc = 0xd3e32;
        }
        0xd3e32 => {
            // block d3e32
            #[cfg(feature = "trace")] eprintln!("f_000d3dd4 pc d3e32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3e32 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d3e34 jmp 0xd3f12
            pc = 0xd3f12; continue;
        }
        0xd3e39 => {
            // block d3e39
            #[cfg(feature = "trace")] eprintln!("f_000d3dd4 pc d3e39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3e39 mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // d3e3c mov DWORD PTR [rbp-0x30],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rcx as u32 as u64)) as u32);
            pc = 0xd3e3f;
        }
        0xd3e3f => {
            // block d3e3f
            #[cfg(feature = "trace")] eprintln!("f_000d3dd4 pc d3e3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3e3f movzx eax,r13b
            rax = ((r13 as u8 as u64)) as u32 as u64;
            // d3e43 mov DWORD PTR [rbp-0x94],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c), ((rax as u32 as u64)) as u32);
            // d3e49 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d3e50 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d3e53 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d3e55 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // d3e5a call 0x26090
            { let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // d3e5f mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // d3e61 call 0x5c6f2
            { let r = { let t = m.env.code_va(0x5c6f2); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // d3e66 mov DWORD PTR [rbp-0x8c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff74), ((rax as u32 as u64)) as u32);
            // d3e6c mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // d3e6f mov DWORD PTR [rbp-0x88],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff78), ((rax as u32 as u64)) as u32);
            // d3e75 mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // d3e78 mov DWORD PTR [rbp-0x84],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c), ((rax as u32 as u64)) as u32);
            // d3e7e mov DWORD PTR [rbp-0x80],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff80), ((r15 as u32 as u64)) as u32);
            // d3e82 mov eax,DWORD PTR [rbp-0x94]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c)) as u64) as u32 as u64;
            // d3e88 mov DWORD PTR [rbp-0x7c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff84), ((rax as u32 as u64)) as u32);
            // d3e8b test r12b,0x80
            fl = logic::<8>((r12 as u8 as u64) & 0x80u64);
            // d3e8f jne 0xd3e9c
            if fl.ne() { pc = 0xd3e9c; continue; }
            pc = 0xd3e91;
        }
        0xd3e91 => {
            // block d3e91
            #[cfg(feature = "trace")] eprintln!("f_000d3dd4 pc d3e91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3e91 cmp r15d,0x8
            fl = sub::<32>((r15 as u32 as u64), 0x8u64).1;
            // d3e95 sbb r13d,r13d
            { let (r, f) = sbb::<32>((r13 as u32 as u64), (r13 as u32 as u64), fl.cf);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d3e98 and r13d,0x1
            { let r = (r13 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3e9c;
        }
        0xd3e9c => {
            // block d3e9c
            #[cfg(feature = "trace")] eprintln!("f_000d3dd4 pc d3e9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3e9c mov DWORD PTR [rbp-0x70],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), ((r13 as u32 as u64)) as u32);
            // d3ea0 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // d3ea3 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // d3ea6 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // d3ea9 shr ecx,0x3
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x3u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // d3eac mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // d3eaf shr edx,0x4
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x4u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // d3eb2 movd xmm0,edx
            x0 = ((rdx as u32 as u64)) as u32 as u128;
            // d3eb6 movd xmm1,eax
            x1 = ((rax as u32 as u64)) as u32 as u128;
            // d3eba punpckldq xmm1,xmm0
            { let a = x1; let b = x0; x1 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // d3ebe movd xmm0,r12d
            x0 = ((r12 as u32 as u64)) as u32 as u128;
            // d3ec3 movd xmm2,ecx
            x2 = ((rcx as u32 as u64)) as u32 as u128;
            // d3ec7 punpckldq xmm0,xmm2
            { let a = x0; let b = x2; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // d3ecb punpckldq xmm0,xmm1
            { let a = x0; let b = x1; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // d3ecf pand xmm0,XMMWORD PTR [rip+0x2f019] # 0x102ef0
            x0 &= m.r128(m.env.data_va(95933));
            // d3ed7 movdqu XMMWORD PTR [rbp-0x6c],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff94), x0);
            // d3edc mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // d3edf shr eax,0x5
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x5u64, fl); rax = (r) as u32 as u64; fl = f; }
            // d3ee2 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d3ee5 mov DWORD PTR [rbp-0x5c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rax as u32 as u64)) as u32);
            // d3ee8 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // d3eeb shr eax,0x6
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x6u64, fl); rax = (r) as u32 as u64; fl = f; }
            // d3eee and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d3ef1 mov DWORD PTR [rbp-0x58],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((rax as u32 as u64)) as u32);
            // d3ef4 shr r12d,1
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x1u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // d3ef7 and r12d,0x1
            { let r = (r12 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d3efb mov DWORD PTR [rbp-0x54],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((r12 as u32 as u64)) as u32);
            // d3eff mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d3f02 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d3f09 mov rdi,r14
            rdi = r14;
            // d3f0c call QWORD PTR [rax+0xc8]
            { let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            pc = 0xd3f12;
        }
        0xd3f12 => {
            // block d3f12
            #[cfg(feature = "trace")] eprintln!("f_000d3dd4 pc d3f12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3f12 add rsp,0x78
            { let (r, f) = add::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // d3f16 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3f17 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d3f19 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d3f1b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3f1d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d3f1f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3f20 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd3f22 size=334
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3f22<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut x2: u128 = fx[2] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd3f22;
    loop {
        match pc {
        0xd3f22 => {
            // block d3f22
            #[cfg(feature = "trace")] eprintln!("f_000d3f22 pc d3f22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3f22 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3f23 mov rbp,rsp
            rbp = rsp;
            // d3f26 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d3f28 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d3f2a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d3f2c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d3f2e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d3f2f sub rsp,0x78
            { let (r, f) = sub::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // d3f33 mov r13d,r9d
            r13 = ((r9 as u32 as u64)) as u32 as u64;
            // d3f36 mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // d3f39 mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // d3f3c mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d3f3e mov r14,rdi
            r14 = rdi;
            // d3f41 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // d3f48 mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // d3f4f cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // d3f52 ja 0xd3f80
            if fl.a() { pc = 0xd3f80; continue; }
            pc = 0xd3f54;
        }
        0xd3f54 => {
            // block d3f54
            #[cfg(feature = "trace")] eprintln!("f_000d3f22 pc d3f54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3f54 test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // d3f57 je 0xd3f80
            if fl.e() { pc = 0xd3f80; continue; }
            pc = 0xd3f59;
        }
        0xd3f59 => {
            // block d3f59
            #[cfg(feature = "trace")] eprintln!("f_000d3f22 pc d3f59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3f59 movzx eax,BYTE PTR [rbp+0x10]
            rax = (m.r8(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // d3f5d movzx ecx,BYTE PTR [rbp+0x18]
            rcx = (m.r8(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // d3f61 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // d3f63 jne 0xd3f87
            if fl.ne() { pc = 0xd3f87; continue; }
            pc = 0xd3f65;
        }
        0xd3f65 => {
            // block d3f65
            #[cfg(feature = "trace")] eprintln!("f_000d3f22 pc d3f65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3f65 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d3f68 movzx esi,dl
            rsi = ((rdx as u8 as u64)) as u32 as u64;
            // d3f6b lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // d3f6f lea rcx,[rbp-0x30]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d3f73 mov rdi,r14
            rdi = r14;
            // d3f76 call QWORD PTR [rax+0xc80]
            { let r = { let t = m.r64(rax.wrapping_add(0xc80)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // d3f7c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d3f7e jne 0xd3f8d
            if fl.ne() { pc = 0xd3f8d; continue; }
            pc = 0xd3f80;
        }
        0xd3f80 => {
            // block d3f80
            #[cfg(feature = "trace")] eprintln!("f_000d3f22 pc d3f80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3f80 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d3f82 jmp 0xd4060
            pc = 0xd4060; continue;
        }
        0xd3f87 => {
            // block d3f87
            #[cfg(feature = "trace")] eprintln!("f_000d3f22 pc d3f87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3f87 mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // d3f8a mov DWORD PTR [rbp-0x30],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rcx as u32 as u64)) as u32);
            pc = 0xd3f8d;
        }
        0xd3f8d => {
            // block d3f8d
            #[cfg(feature = "trace")] eprintln!("f_000d3f22 pc d3f8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3f8d movzx eax,r13b
            rax = ((r13 as u8 as u64)) as u32 as u64;
            // d3f91 mov DWORD PTR [rbp-0x94],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c), ((rax as u32 as u64)) as u32);
            // d3f97 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d3f9e xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d3fa1 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d3fa3 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // d3fa8 call 0x26090
            { let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // d3fad mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // d3faf call 0x5c70e
            { let r = { let t = m.env.code_va(0x5c70e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // d3fb4 mov DWORD PTR [rbp-0x8c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff74), ((rax as u32 as u64)) as u32);
            // d3fba mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // d3fbd mov DWORD PTR [rbp-0x88],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff78), ((rax as u32 as u64)) as u32);
            // d3fc3 mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // d3fc6 mov DWORD PTR [rbp-0x84],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c), ((rax as u32 as u64)) as u32);
            // d3fcc mov DWORD PTR [rbp-0x80],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff80), ((r15 as u32 as u64)) as u32);
            // d3fd0 mov eax,DWORD PTR [rbp-0x94]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c)) as u64) as u32 as u64;
            // d3fd6 mov DWORD PTR [rbp-0x7c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff84), ((rax as u32 as u64)) as u32);
            // d3fd9 test r12b,0x80
            fl = logic::<8>((r12 as u8 as u64) & 0x80u64);
            // d3fdd jne 0xd3fea
            if fl.ne() { pc = 0xd3fea; continue; }
            pc = 0xd3fdf;
        }
        0xd3fdf => {
            // block d3fdf
            #[cfg(feature = "trace")] eprintln!("f_000d3f22 pc d3fdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3fdf cmp r15d,0x8
            fl = sub::<32>((r15 as u32 as u64), 0x8u64).1;
            // d3fe3 sbb r13d,r13d
            { let (r, f) = sbb::<32>((r13 as u32 as u64), (r13 as u32 as u64), fl.cf);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d3fe6 and r13d,0x1
            { let r = (r13 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3fea;
        }
        0xd3fea => {
            // block d3fea
            #[cfg(feature = "trace")] eprintln!("f_000d3f22 pc d3fea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3fea mov DWORD PTR [rbp-0x70],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), ((r13 as u32 as u64)) as u32);
            // d3fee mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // d3ff1 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // d3ff4 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // d3ff7 shr ecx,0x3
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x3u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // d3ffa mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // d3ffd shr edx,0x4
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x4u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // d4000 movd xmm0,edx
            x0 = ((rdx as u32 as u64)) as u32 as u128;
            // d4004 movd xmm1,eax
            x1 = ((rax as u32 as u64)) as u32 as u128;
            // d4008 punpckldq xmm1,xmm0
            { let a = x1; let b = x0; x1 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // d400c movd xmm0,r12d
            x0 = ((r12 as u32 as u64)) as u32 as u128;
            // d4011 movd xmm2,ecx
            x2 = ((rcx as u32 as u64)) as u32 as u128;
            // d4015 punpckldq xmm0,xmm2
            { let a = x0; let b = x2; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // d4019 punpckldq xmm0,xmm1
            { let a = x0; let b = x1; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // d401d pand xmm0,XMMWORD PTR [rip+0x2eecb] # 0x102ef0
            x0 &= m.r128(m.env.data_va(95933));
            // d4025 movdqu XMMWORD PTR [rbp-0x6c],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff94), x0);
            // d402a mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // d402d shr eax,0x5
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x5u64, fl); rax = (r) as u32 as u64; fl = f; }
            // d4030 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d4033 mov DWORD PTR [rbp-0x5c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rax as u32 as u64)) as u32);
            // d4036 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // d4039 shr eax,0x6
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x6u64, fl); rax = (r) as u32 as u64; fl = f; }
            // d403c and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d403f mov DWORD PTR [rbp-0x58],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((rax as u32 as u64)) as u32);
            // d4042 shr r12d,1
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x1u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // d4045 and r12d,0x1
            { let r = (r12 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d4049 mov DWORD PTR [rbp-0x54],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((r12 as u32 as u64)) as u32);
            // d404d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d4050 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d4057 mov rdi,r14
            rdi = r14;
            // d405a call QWORD PTR [rax+0xc8]
            { let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            pc = 0xd4060;
        }
        0xd4060 => {
            // block d4060
            #[cfg(feature = "trace")] eprintln!("f_000d3f22 pc d4060 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4060 add rsp,0x78
            { let (r, f) = add::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // d4064 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4065 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d4067 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d4069 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d406b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d406d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d406e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x688be size=338
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000688be<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x688be;
    loop {
        match pc {
        0x688be => {
            // block 688be
            #[cfg(feature = "trace")] eprintln!("f_000688be pc 688be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 688be push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 688bf mov rbp,rsp
            rbp = rsp;
            // 688c2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 688c4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 688c6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 688c7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 688c8 lea r14,[rip+0x15f9f1] # 0x1c82c0
            r14 = m.env.data_va(903821);
            // 688cf lea rsi,[rip+0xcb06b] # 0x133941
            rsi = m.env.data_va(295182);
            // 688d6 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 688db mov rdi,r14
            rdi = r14;
            // 688de call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 688e3 lea rbx,[rip+0x15f9ee] # 0x1c82d8
            rbx = m.env.data_va(903845);
            // 688ea lea rsi,[rip+0xcb063] # 0x133954
            rsi = m.env.data_va(295201);
            // 688f1 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 688f6 mov rdi,rbx
            rdi = rbx;
            // 688f9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 688fe lea rbx,[rip+0x15f9eb] # 0x1c82f0
            rbx = m.env.data_va(903869);
            // 68905 lea rsi,[rip+0xcb05b] # 0x133967
            rsi = m.env.data_va(295220);
            // 6890c mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 68911 mov rdi,rbx
            rdi = rbx;
            // 68914 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68919 lea rbx,[rip+0x15f9e8] # 0x1c8308
            rbx = m.env.data_va(903893);
            // 68920 lea rsi,[rip+0xcb053] # 0x13397a
            rsi = m.env.data_va(295239);
            // 68927 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 6892c mov rdi,rbx
            rdi = rbx;
            // 6892f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68934 lea rbx,[rip+0x15f9e5] # 0x1c8320
            rbx = m.env.data_va(903917);
            // 6893b lea rsi,[rip+0xcb04b] # 0x13398d
            rsi = m.env.data_va(295258);
            // 68942 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 68947 mov rdi,rbx
            rdi = rbx;
            // 6894a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6894f lea rbx,[rip+0x15f9e2] # 0x1c8338
            rbx = m.env.data_va(903941);
            // 68956 lea rsi,[rip+0xcb043] # 0x1339a0
            rsi = m.env.data_va(295277);
            // 6895d mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 68962 mov rdi,rbx
            rdi = rbx;
            // 68965 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6896a lea rbx,[rip+0x15f9df] # 0x1c8350
            rbx = m.env.data_va(903965);
            // 68971 lea rsi,[rip+0xcb03b] # 0x1339b3
            rsi = m.env.data_va(295296);
            // 68978 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 6897d mov rdi,rbx
            rdi = rbx;
            // 68980 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68985 lea rbx,[rip+0x15f9dc] # 0x1c8368
            rbx = m.env.data_va(903989);
            // 6898c lea rsi,[rip+0xcb033] # 0x1339c6
            rsi = m.env.data_va(295315);
            // 68993 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 68998 mov rdi,rbx
            rdi = rbx;
            // 6899b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 689a0 lea rdi,[rip+0xfffffffffffffc3c] # 0x685e3
            rdi = m.env.code_va(0x685e3);
            // 689a7 lea r14,[rip+0xfffffffffff967c2] # 0xfffffffffffff170
            r14 = m.env.data_va(-968387);
            // 689ae xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 689b0 mov rdx,r14
            rdx = r14;
            // 689b3 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 689b8 lea rbx,[rip+0x15f9c1] # 0x1c8380
            rbx = m.env.data_va(904013);
            // 689bf mov rdi,rbx
            rdi = rbx;
            // 689c2 call 0x69a04
            { let r = { let t = m.env.code_va(0x69a04); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 689c7 lea rdi,[rip+0x42] # 0x68a10
            rdi = m.env.code_va(0x68a10);
            // 689ce mov rsi,rbx
            rsi = rbx;
            // 689d1 mov rdx,r14
            rdx = r14;
            // 689d4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 689d8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 689d9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 689db pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 689dd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 689de jmp 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0x689e3 => {
            // block 689e3
            #[cfg(feature = "trace")] eprintln!("f_000688be pc 689e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 689e3 mov r15,rax
            r15 = rax;
            pc = 0x689e6;
        }
        0x689e6 => {
            // block 689e6
            #[cfg(feature = "trace")] eprintln!("f_000688be pc 689e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 689e6 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 689ea mov rdi,rbx
            rdi = rbx;
            // 689ed call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 689f2 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // 689f5 jne 0x689e6
            if fl.ne() { pc = 0x689e6; continue; }
            pc = 0x689f7;
        }
        0x689f7 => {
            // block 689f7
            #[cfg(feature = "trace")] eprintln!("f_000688be pc 689f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 689f7 jmp 0x689fc
            pc = 0x689fc; continue;
        }
        0x689f9 => {
            // block 689f9
            #[cfg(feature = "trace")] eprintln!("f_000688be pc 689f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 689f9 mov r15,rax
            r15 = rax;
            pc = 0x689fc;
        }
        0x689fc => {
            // block 689fc
            #[cfg(feature = "trace")] eprintln!("f_000688be pc 689fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 689fc mov rdi,r15
            rdi = r15;
            // 689ff call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5504 size=340
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005504<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5504;
    loop {
        match pc {
        0x5504 => {
            // block 5504
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 5504 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5504 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5505 mov rbp,rsp
            rbp = rsp;
            // 5508 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 550a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 550c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 550e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 550f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 5513 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 5516 mov r14,rsi
            r14 = rsi;
            // 5519 mov rbx,rdi
            rbx = rdi;
            // 551c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 551f call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5525 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 5528 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 552a cmove r12d,r15d
            if fl.e() { r12 = ((r15 as u32 as u64)) as u32 as u64; } else { r12 = r12 as u32 as u64; }
            // 552e mov DWORD PTR [rbp-0x24],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0xcu64) as u32);
            // 5535 mov DWORD PTR [rbp-0x28],0x12
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x12u64) as u32);
            // 553c mov DWORD PTR [r14],0x3
            m.w32(r14, (0x3u64) as u32);
            // 5543 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5546 mov rdi,rbx
            rdi = rbx;
            // 5549 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 554c call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5552 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5554 jne 0x5618
            if fl.ne() { pc = 0x5618; continue; }
            pc = 0x555a;
        }
        0x555a => {
            // block 555a
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 555a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 555a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 555d lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 5561 mov rdi,rbx
            rdi = rbx;
            // 5564 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 5567 call QWORD PTR [rax+0x3e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x3e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 556d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5570 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 5574 mov rdi,rbx
            rdi = rbx;
            // 5577 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 557a call QWORD PTR [rax+0x380]
            { let r = { let t = m.r64(rax.wrapping_add(0x380)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5580 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 5583 mov ecx,DWORD PTR [rbp-0x24]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 5586 cmp rcx,0xb
            fl = sub::<64>(rcx, 0xbu64).1;
            // 558a ja 0x5618
            if fl.a() { pc = 0x5618; continue; }
            pc = 0x5590;
        }
        0x5590 => {
            // block 5590
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 5590 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5590 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 5595 lea rdx,[rip+0x8c] # 0x5628
            rdx = m.env.code_va(0x5628);
            // 559c movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // 55a0 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // 55a3 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x55a5 => {
            // block 55a5
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55a5 mov eax,DWORD PTR [rbp-0x28]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 55a8 lea ecx,[rax-0x4]
            rcx = (rax.wrapping_add(0xffff_ffff_ffff_fffc)) as u32 as u64;
            // 55ab cmp ecx,0x9
            fl = sub::<32>((rcx as u32 as u64), 0x9u64).1;
            // 55ae ja 0x55c0
            if fl.a() { pc = 0x55c0; continue; }
            pc = 0x55b0;
        }
        0x55b0 => {
            // block 55b0
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55b0 movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // 55b3 lea rdx,[rip+0xfa276] # 0xff830
            rdx = m.env.data_va(81917);
            // 55ba mov r15b,BYTE PTR [rcx+rdx*1]
            r15 = (r15 & !0xff) | ((m.r8(rcx.wrapping_add(rdx)) as u64) as u8 as u64);
            // 55be jmp 0x55c3
            pc = 0x55c3; continue;
        }
        0x55c0 => {
            // block 55c0
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55c0 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x55c3;
        }
        0x55c3 => {
            // block 55c3
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55c3 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 55c6 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 55c9 jmp 0x55f7
            pc = 0x55f7; continue;
        }
        0x55cb => {
            // block 55cb
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55cb cmp DWORD PTR [rbp-0x28],0x9
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, 0x9u64).1;
            // 55cf jmp 0x5605
            pc = 0x5605; continue;
        }
        0x55d1 => {
            // block 55d1
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55d1 mov eax,DWORD PTR [rbp-0x28]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 55d4 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 55d6 and ecx,0xfffffff7
            { let r = (rcx as u32 as u64) & 0xfffffff7u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 55d9 cmp ecx,0x6
            fl = sub::<32>((rcx as u32 as u64), 0x6u64).1;
            // 55dc sete r15b
            r15 = (r15 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 55e0 cmp eax,0xe
            fl = sub::<32>((rax as u32 as u64), 0xeu64).1;
            // 55e3 jmp 0x55f7
            pc = 0x55f7; continue;
        }
        0x55e5 => {
            // block 55e5
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55e5 mov eax,DWORD PTR [rbp-0x28]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 55e8 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 55ea and ecx,0xfffffff7
            { let r = (rcx as u32 as u64) & 0xfffffff7u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 55ed cmp ecx,0x7
            fl = sub::<32>((rcx as u32 as u64), 0x7u64).1;
            // 55f0 sete r15b
            r15 = (r15 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 55f4 cmp eax,0xf
            fl = sub::<32>((rax as u32 as u64), 0xfu64).1;
            pc = 0x55f7;
        }
        0x55f7 => {
            // block 55f7
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 55f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55f7 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 55fa movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 55fd inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 55ff jmp 0x5609
            pc = 0x5609; continue;
        }
        0x5601 => {
            // block 5601
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 5601 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5601 cmp DWORD PTR [rbp-0x28],0xc
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, 0xcu64).1;
            pc = 0x5605;
        }
        0x5605 => {
            // block 5605
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 5605 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5605 sete r15b
            r15 = (r15 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x5609;
        }
        0x5609 => {
            // block 5609
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 5609 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5609 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 560b test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // 560f cmovne ecx,eax
            if fl.ne() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 5612 mov DWORD PTR [r14],ecx
            m.w32(r14, ((rcx as u32 as u64)) as u32);
            // 5615 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x5618;
        }
        0x5618 => {
            // block 5618
            #[cfg(feature = "trace")] eprintln!("f_00005504 pc 5618 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5618 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 561b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 561f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5620 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5622 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5624 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5626 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5627 ret
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
            { let r = { let t = m.env.code_va(0xd0ac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x2a25e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4467 lea rbx,[r15+0x20]
            rbx = r15.wrapping_add(0x20);
            // d446b mov rdi,r12
            rdi = r12;
            // d446e mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d4471 mov rdx,rbx
            rdx = rbx;
            // d4474 call 0xd40e0
            { let r = { let t = m.env.code_va(0xd40e0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0xd0)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x5c6f2); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rcx.wrapping_add(0xc18)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x5c70e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rcx.wrapping_add(0xc18)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x2a1d8); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x5c70e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4568 lea rbx,[rbp-0x90]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d456f mov rdi,rbx
            rdi = rbx;
            // d4572 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d4574 call 0x2a0c4
            { let r = { let t = m.env.code_va(0x2a0c4); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x3a3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x499); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69475 mov rdi,rbx
            rdi = rbx;
            // 69478 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df089 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df08c mov edx,DWORD PTR [rbx+0x250]
            rdx = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // df092 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // df096 mov rdi,rbx
            rdi = rbx;
            // df099 call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x32d57); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x1a00)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x1a08)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x1a10)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0xb672c size=346
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b672c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb672c;
    loop {
        match pc {
        0xb672c => {
            // block b672c
            #[cfg(feature = "trace")] eprintln!("f_000b672c pc b672c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b672c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b672d mov rbp,rsp
            rbp = rsp;
            // b6730 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b6732 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b6734 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b6736 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b6738 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b6739 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b673d mov r15,r8
            r15 = r8;
            // b6740 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // b6743 mov r13d,edx
            r13 = ((rdx as u32 as u64)) as u32 as u64;
            // b6746 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // b6749 mov rbx,rdi
            rbx = rdi;
            // b674c call 0x4522c
            { let r = { let t = m.env.code_va(0x4522c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6751 lea rax,[rip+0x8bcd8] # 0x142430
            rax = m.env.data_va(355325);
            // b6758 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b675b lea rax,[rbx+0x258]
            rax = rbx.wrapping_add(0x258);
            // b6762 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // b6766 mov QWORD PTR [rbx+0x268],0x0
            m.w64(rbx.wrapping_add(0x268), (0x0u64) as u64);
            // b6771 mov QWORD PTR [rbx+0x260],0x0
            m.w64(rbx.wrapping_add(0x260), (0x0u64) as u64);
            // b677c mov QWORD PTR [rbx+0x258],0x0
            m.w64(rbx.wrapping_add(0x258), (0x0u64) as u64);
            // b6787 mov BYTE PTR [rbx+0xa],0x0
            m.w8(rbx.wrapping_add(0xa), (0x0u64) as u8);
            // b678b movzx edx,r13b
            rdx = ((r13 as u8 as u64)) as u32 as u64;
            // b678f mov rdi,rbx
            rdi = rbx;
            // b6792 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // b6795 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // b6798 mov r8,r15
            r8 = r15;
            // b679b call 0x452ea
            { let r = { let t = m.env.code_va(0x452ea); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b67a0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b67a2 je 0xb6852
            if fl.e() { pc = 0xb6852; continue; }
            pc = 0xb67a8;
        }
        0xb67a8 => {
            // block b67a8
            #[cfg(feature = "trace")] eprintln!("f_000b672c pc b67a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b67a8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b67ab mov rax,QWORD PTR [rax+0x418]
            rax = m.r64(rax.wrapping_add(0x418)) as u64;
            // b67b2 mov rdi,rbx
            rdi = rbx;
            // b67b5 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b67b7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b67b9 je 0xb67e4
            if fl.e() { pc = 0xb67e4; continue; }
            pc = 0xb67bb;
        }
        0xb67bb => {
            // block b67bb
            #[cfg(feature = "trace")] eprintln!("f_000b672c pc b67bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b67bb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b67be mov rax,QWORD PTR [rax+0xcd8]
            rax = m.r64(rax.wrapping_add(0xcd8)) as u64;
            // b67c5 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // b67c9 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b67cb mov rdi,rbx
            rdi = rbx;
            // b67ce call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b67d0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b67d3 mov rax,QWORD PTR [rax+0x3c8]
            rax = m.r64(rax.wrapping_add(0x3c8)) as u64;
            // b67da mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // b67dd mov rdi,rbx
            rdi = rbx;
            // b67e0 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b67e2 jmp 0xb6852
            pc = 0xb6852; continue;
        }
        0xb67e4 => {
            // block b67e4
            #[cfg(feature = "trace")] eprintln!("f_000b672c pc b67e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b67e4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b67e7 mov rax,QWORD PTR [rax+0x388]
            rax = m.r64(rax.wrapping_add(0x388)) as u64;
            // b67ee lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b67f2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b67f4 mov rdi,rbx
            rdi = rbx;
            // b67f7 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b67f9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b67fc mov rax,QWORD PTR [rax+0x398]
            rax = m.r64(rax.wrapping_add(0x398)) as u64;
            // b6803 lea rdx,[rbp-0x34]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // b6807 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b6809 mov rdi,rbx
            rdi = rbx;
            // b680c call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b680e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b6811 mov rax,QWORD PTR [rax+0x198]
            rax = m.r64(rax.wrapping_add(0x198)) as u64;
            // b6818 mov rdi,rbx
            rdi = rbx;
            // b681b call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b681d mov esi,DWORD PTR [rbp-0x30]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // b6820 mov edx,DWORD PTR [rbp-0x34]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // b6823 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b6825 call 0xd162b
            { let r = { let t = m.env.code_va(0xd162b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b682a mov DWORD PTR [rbx+0x24c],eax
            m.w32(rbx.wrapping_add(0x24c), ((rax as u32 as u64)) as u32);
            // b6830 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b6833 mov rax,QWORD PTR [rax+0x198]
            rax = m.r64(rax.wrapping_add(0x198)) as u64;
            // b683a mov rdi,rbx
            rdi = rbx;
            // b683d call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b683f mov esi,DWORD PTR [rbp-0x30]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // b6842 mov edx,DWORD PTR [rbp-0x34]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // b6845 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b6847 call 0xd1b6c
            { let r = { let t = m.env.code_va(0xd1b6c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b684c mov DWORD PTR [rbx+0x248],eax
            m.w32(rbx.wrapping_add(0x248), ((rax as u32 as u64)) as u32);
            pc = 0xb6852;
        }
        0xb6852 => {
            // block b6852
            #[cfg(feature = "trace")] eprintln!("f_000b672c pc b6852 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6852 mov rdi,rbx
            rdi = rbx;
            // b6855 call 0xdc1f4
            { let r = { let t = m.env.code_va(0xdc1f4); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b685a add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b685e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b685f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b6861 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b6863 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b6865 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b6867 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6868 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb6869 => {
            // block b6869
            #[cfg(feature = "trace")] eprintln!("f_000b672c pc b6869 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6869 mov r14,rax
            r14 = rax;
            // b686c mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b6870 call 0xebcd8
            { let r = { let t = m.env.stub_va(0x4a5); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6875 mov rdi,rbx
            rdi = rbx;
            // b6878 call 0x45280
            { let r = { let t = m.env.code_va(0x45280); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b687d mov rdi,r14
            rdi = r14;
            // b6880 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe5c size=348
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000e5c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe5c;
    loop {
        match pc {
        0xe5c => {
            // block e5c
            #[cfg(feature = "trace")] eprintln!("f_00000e5c pc e5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e5d mov rbp,rsp
            rbp = rsp;
            // e60 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e62 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e63 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e67 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e6a mov rbx,rdi
            rbx = rdi;
            // e6d mov DWORD PTR [rbp-0x14],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0xcu64) as u32);
            // e74 mov DWORD PTR [rbp-0x18],0x12
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (0x12u64) as u32);
            // e7b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e7e call QWORD PTR [rax+0x9b0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9b0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e84 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e86 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e88 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // e8a mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // e8f je 0xf94
            if fl.e() { pc = 0xf94; continue; }
            pc = 0xe95;
        }
        0xe95 => {
            // block e95
            #[cfg(feature = "trace")] eprintln!("f_00000e5c pc e95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e95 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e98 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e9c mov rdi,rbx
            rdi = rbx;
            // e9f mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // ea2 call QWORD PTR [rax+0x3e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x3e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea8 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // eaa xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // eac test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // eae mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // eb3 je 0xf94
            if fl.e() { pc = 0xf94; continue; }
            pc = 0xeb9;
        }
        0xeb9 => {
            // block eb9
            #[cfg(feature = "trace")] eprintln!("f_00000e5c pc eb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ebc lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // ec0 mov rdi,rbx
            rdi = rbx;
            // ec3 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // ec6 call QWORD PTR [rax+0x380]
            { let r = { let t = m.r64(rax.wrapping_add(0x380)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ecc mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // ece xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // ed0 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // ed2 mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // ed7 je 0xf94
            if fl.e() { pc = 0xf94; continue; }
            pc = 0xedd;
        }
        0xedd => {
            // block edd
            #[cfg(feature = "trace")] eprintln!("f_00000e5c pc edd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // edd mov edx,0x500
            rdx = (0x500u64) as u32 as u64;
            // ee2 mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // ee5 cmp rcx,0x5
            fl = sub::<64>(rcx, 0x5u64).1;
            // ee9 jbe 0xf07
            if fl.be() { pc = 0xf07; continue; }
            pc = 0xeeb;
        }
        0xeeb => {
            // block eeb
            #[cfg(feature = "trace")] eprintln!("f_00000e5c pc eeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eeb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // eee lea rsi,[rip+0x128f0b] # 0x129e00
            rsi = m.env.data_va(255437);
            // ef5 mov rdi,rbx
            rdi = rbx;
            // ef8 call QWORD PTR [rax+0x170]
            { let r = { let t = m.r64(rax.wrapping_add(0x170)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // efe xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // f00 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // f02 jmp 0xf94
            pc = 0xf94; continue;
        }
        0xf07 => {
            // block f07
            #[cfg(feature = "trace")] eprintln!("f_00000e5c pc f07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f07 movabs rax,0x2d000000000
            rax = 0x0_02d0_0000_0000u64;
            // f11 lea rsi,[rip+0x88] # 0xfa0
            rsi = m.env.code_va(0xfa0);
            // f18 movsxd rcx,DWORD PTR [rsi+rcx*4]
            rcx = sx::<32>(m.r32(rsi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // f1c add rcx,rsi
            { let (r, f) = add::<64>(rcx, rsi);
              rcx = r;
              fl = f; }
            // f1f jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xf21 => {
            // block f21
            #[cfg(feature = "trace")] eprintln!("f_00000e5c pc f21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f21 mov eax,DWORD PTR [rbp-0x18]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // f24 mov edx,0x800
            rdx = (0x800u64) as u32 as u64;
            // f29 cmp eax,0xa
            fl = sub::<32>((rax as u32 as u64), 0xau64).1;
            // f2c je 0xf3e
            if fl.e() { pc = 0xf3e; continue; }
            pc = 0xf2e;
        }
        0xf2e => {
            // block f2e
            #[cfg(feature = "trace")] eprintln!("f_00000e5c pc f2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f2e cmp eax,0x11
            fl = sub::<32>((rax as u32 as u64), 0x11u64).1;
            // f31 mov ecx,0x800
            rcx = (0x800u64) as u32 as u64;
            // f36 mov edx,0x780
            rdx = (0x780u64) as u32 as u64;
            // f3b cmove edx,ecx
            if fl.e() { rdx = ((rcx as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            pc = 0xf3e;
        }
        0xf3e => {
            // block f3e
            #[cfg(feature = "trace")] eprintln!("f_00000e5c pc f3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f3e and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // f41 cmp eax,0x10
            fl = sub::<32>((rax as u32 as u64), 0x10u64).1;
            // f44 sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // f47 movabs rsi,0x87000000000
            rsi = 0x0_0870_0000_0000u64;
            // f51 movabs rax,0x43800000000
            rax = 0x0_0438_0000_0000u64;
            // f5b cmove rax,rsi
            if fl.e() { rax = rsi; }
            // f5f shl edx,cl
            { let (r, f) = shl::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // f61 jmp 0xf94
            pc = 0xf94; continue;
        }
        0xf63 => {
            // block f63
            #[cfg(feature = "trace")] eprintln!("f_00000e5c pc f63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f63 movabs rax,0x1e600000000
            rax = 0x0_01e6_0000_0000u64;
            // f6d mov edx,0x2d0
            rdx = (0x2d0u64) as u32 as u64;
            // f72 jmp 0xf94
            pc = 0xf94; continue;
        }
        0xf74 => {
            // block f74
            #[cfg(feature = "trace")] eprintln!("f_00000e5c pc f74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f74 movabs rax,0x24000000000
            rax = 0x0_0240_0000_0000u64;
            // f7e mov edx,0x2d0
            rdx = (0x2d0u64) as u32 as u64;
            // f83 jmp 0xf94
            pc = 0xf94; continue;
        }
        0xf85 => {
            // block f85
            #[cfg(feature = "trace")] eprintln!("f_00000e5c pc f85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f85 movabs rax,0x63400000000
            rax = 0x0_0634_0000_0000u64;
            // f8f mov edx,0x800
            rdx = (0x800u64) as u32 as u64;
            pc = 0xf94;
        }
        0xf94 => {
            // block f94
            #[cfg(feature = "trace")] eprintln!("f_00000e5c pc f94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f94 or rax,rdx
            { let r = rax | rdx; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // f97 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f9b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f9c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f9e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f9f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
